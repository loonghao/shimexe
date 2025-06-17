use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, SystemTime};
use tracing::{debug, info, warn};

use crate::config::ShimConfig;
use crate::downloader::Downloader;
use crate::error::{Result, ShimError};
use crate::updater::ShimUpdater;
use crate::utils::merge_env_vars;

/// Cache entry for executable validation results
#[derive(Debug, Clone)]
struct ValidationCacheEntry {
    is_valid: bool,
    last_checked: SystemTime,
    file_modified: SystemTime,
}

/// Performance cache for executable validation
#[derive(Debug, Clone)]
struct ExecutableCache {
    cache: Arc<Mutex<std::collections::HashMap<PathBuf, ValidationCacheEntry>>>,
    ttl: Duration,
}

impl ExecutableCache {
    fn new(ttl: Duration) -> Self {
        Self {
            cache: Arc::new(Mutex::new(std::collections::HashMap::new())),
            ttl,
        }
    }

    fn is_valid(&self, path: &Path) -> Option<bool> {
        let now = SystemTime::now();
        if let Ok(cache) = self.cache.lock() {
            if let Some(entry) = cache.get(path) {
                // Check if cache entry is still valid
                if now
                    .duration_since(entry.last_checked)
                    .unwrap_or(Duration::MAX)
                    < self.ttl
                {
                    // Check if file hasn't been modified
                    if let Ok(metadata) = std::fs::metadata(path) {
                        if let Ok(modified) = metadata.modified() {
                            if modified <= entry.file_modified {
                                return Some(entry.is_valid);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn set_valid(&self, path: &Path, is_valid: bool) {
        let now = SystemTime::now();
        let file_modified = std::fs::metadata(path)
            .and_then(|m| m.modified())
            .unwrap_or(now);

        if let Ok(mut cache) = self.cache.lock() {
            cache.insert(
                path.to_path_buf(),
                ValidationCacheEntry {
                    is_valid,
                    last_checked: now,
                    file_modified,
                },
            );
        }
    }
}

// Global cache instance with 30-second TTL
static EXECUTABLE_CACHE: OnceLock<ExecutableCache> = OnceLock::new();

fn get_executable_cache() -> &'static ExecutableCache {
    EXECUTABLE_CACHE.get_or_init(|| ExecutableCache::new(Duration::from_secs(30)))
}

/// Shim runner that executes the target executable
pub struct ShimRunner {
    config: ShimConfig,
    shim_file_path: Option<PathBuf>,
}

impl ShimRunner {
    /// Create a new shim runner from a configuration file
    pub fn from_file<P: AsRef<Path>>(shim_file: P) -> Result<Self> {
        let mut config = ShimConfig::from_file(&shim_file)?;
        config.expand_env_vars()?;

        Ok(Self {
            config,
            shim_file_path: Some(shim_file.as_ref().to_path_buf()),
        })
    }

    /// Create a new shim runner from a configuration
    pub fn from_config(mut config: ShimConfig) -> Result<Self> {
        config.expand_env_vars()?;
        Ok(Self {
            config,
            shim_file_path: None,
        })
    }

    /// Execute the shim with additional arguments
    pub fn execute(&self, additional_args: &[String]) -> Result<i32> {
        let start_time = SystemTime::now();

        // Check for updates if auto-update is enabled
        if let Some(ref auto_update) = self.config.auto_update {
            if let Some(ref shim_file_path) = self.shim_file_path {
                self.check_and_update(auto_update, shim_file_path)?;
            }
        }

        // Check if we need to download the executable
        self.ensure_executable_available()?;

        let executable_path = self.config.get_executable_path()?;

        // Use cached validation if available
        let cache = get_executable_cache();
        if let Some(is_valid) = cache.is_valid(&executable_path) {
            if !is_valid {
                return Err(ShimError::ExecutableNotFound(
                    executable_path.to_string_lossy().to_string(),
                ));
            }
        } else {
            // Validate and cache the result
            let is_valid = self.validate_executable_fast(&executable_path);
            cache.set_valid(&executable_path, is_valid);
            if !is_valid {
                return Err(ShimError::ExecutableNotFound(
                    executable_path.to_string_lossy().to_string(),
                ));
            }
        }

        debug!("Executing: {:?}", executable_path);
        debug!("Default args: {:?}", self.config.shim.args);
        debug!("Additional args: {:?}", additional_args);

        // Prepare command with optimized environment variable handling
        let mut cmd = Command::new(&executable_path);

        // Add default arguments
        cmd.args(&self.config.shim.args);

        // Add additional arguments
        cmd.args(additional_args);

        // Set working directory if specified
        if let Some(ref cwd) = self.config.shim.cwd {
            cmd.current_dir(cwd);
        }

        // Set environment variables (optimized to avoid unnecessary allocations)
        if !self.config.env.is_empty() {
            let env_vars = merge_env_vars(&self.config.env);
            for (key, value) in env_vars {
                cmd.env(key, value);
            }
        }

        // Configure stdio to inherit from parent
        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        info!(
            "Executing shim '{}' -> {:?}",
            self.config.shim.name, executable_path
        );

        // Execute the command
        let result = match cmd.status() {
            Ok(status) => {
                let exit_code = status.code().unwrap_or(-1);
                debug!("Process exited with code: {}", exit_code);
                Ok(exit_code)
            }
            Err(e) => {
                warn!("Failed to execute process: {}", e);
                Err(ShimError::ProcessExecution(e.to_string()))
            }
        };

        // Log execution time for performance monitoring
        if let Ok(elapsed) = start_time.elapsed() {
            debug!("Shim execution took: {:?}", elapsed);
        }

        result
    }

    /// Fast executable validation without full metadata checks
    fn validate_executable_fast(&self, path: &Path) -> bool {
        path.exists() && path.is_file()
    }

    /// Get the shim configuration
    pub fn config(&self) -> &ShimConfig {
        &self.config
    }

    /// Validate that the target executable exists and is executable
    pub fn validate(&self) -> Result<()> {
        let executable_path = self.config.get_executable_path()?;

        // Use cached validation if available
        let cache = get_executable_cache();
        if let Some(is_valid) = cache.is_valid(&executable_path) {
            if is_valid {
                return Ok(());
            } else {
                return Err(ShimError::ExecutableNotFound(
                    executable_path.to_string_lossy().to_string(),
                ));
            }
        }

        // Perform full validation and cache the result
        let validation_result = self.validate_executable_full(&executable_path);
        let is_valid = validation_result.is_ok();
        cache.set_valid(&executable_path, is_valid);
        validation_result
    }

    /// Perform full executable validation with all checks
    fn validate_executable_full(&self, executable_path: &Path) -> Result<()> {
        if !executable_path.exists() {
            return Err(ShimError::ExecutableNotFound(
                executable_path.to_string_lossy().to_string(),
            ));
        }

        // Check if it's a file (not a directory)
        if !executable_path.is_file() {
            return Err(ShimError::Config(format!(
                "Path is not a file: {}",
                executable_path.display()
            )));
        }

        // On Unix-like systems, check if the file is executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = executable_path.metadata().map_err(ShimError::Io)?;
            let permissions = metadata.permissions();

            if permissions.mode() & 0o111 == 0 {
                return Err(ShimError::PermissionDenied(format!(
                    "File is not executable: {}",
                    executable_path.display()
                )));
            }
        }

        Ok(())
    }

    /// Check for updates and perform update if needed
    fn check_and_update(
        &self,
        auto_update: &crate::config::AutoUpdate,
        shim_file_path: &Path,
    ) -> Result<()> {
        let executable_path = self.config.get_executable_path()?;
        let updater = ShimUpdater::new(
            auto_update.clone(),
            shim_file_path.to_path_buf(),
            executable_path,
        );

        // Use a simple blocking approach for now
        // In a real implementation, you might want to use async/await
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            ShimError::ProcessExecution(format!("Failed to create async runtime: {}", e))
        })?;

        rt.block_on(async {
            match updater.check_update_needed().await {
                Ok(Some(version)) => {
                    info!("Auto-update available: {}", version);
                    if let Err(e) = updater.update_to_version(&version).await {
                        warn!("Auto-update failed: {}", e);
                    }
                }
                Ok(None) => {
                    debug!("No update needed");
                }
                Err(e) => {
                    warn!("Update check failed: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Ensure the executable is available, downloading if necessary
    fn ensure_executable_available(&self) -> Result<()> {
        // Check if this shim has a download URL (was created from HTTP)
        if let Some(download_url) = self.config.get_download_url() {
            // This shim was created from an HTTP URL
            let executable_path = match self.config.get_executable_path() {
                Ok(path) => path,
                Err(_) => {
                    // If get_executable_path fails, it means we need to download
                    return self.download_executable_from_url(download_url);
                }
            };

            // Check if the file exists
            if !executable_path.exists() {
                return self.download_executable_from_url(download_url);
            }
        } else if Downloader::is_url(&self.config.shim.path) {
            // Legacy: path is still a URL (for backward compatibility)
            let executable_path = match self.config.get_executable_path() {
                Ok(path) => path,
                Err(_) => {
                    // If get_executable_path fails for a URL, it means we need to download
                    return self.download_executable_from_url(&self.config.shim.path);
                }
            };

            // Check if the file exists
            if !executable_path.exists() {
                return self.download_executable_from_url(&self.config.shim.path);
            }
        }
        Ok(())
    }

    /// Download the executable from HTTP URL
    fn download_executable_from_url(&self, url: &str) -> Result<()> {
        // Extract filename from URL
        let filename = Downloader::extract_filename_from_url(url).ok_or_else(|| {
            ShimError::Config(format!("Could not extract filename from URL: {}", url))
        })?;

        // Determine download directory
        let download_dir = if let Some(ref shim_file_path) = self.shim_file_path {
            // Use the same directory as the shim file
            shim_file_path
                .parent()
                .ok_or_else(|| {
                    ShimError::Config("Could not determine shim file directory".to_string())
                })?
                .join(&self.config.shim.name)
                .join("bin")
        } else {
            // Fallback to home directory
            dirs::home_dir()
                .ok_or_else(|| ShimError::Config("Could not determine home directory".to_string()))?
                .join(".shimexe")
                .join(&self.config.shim.name)
                .join("bin")
        };

        let download_path = download_dir.join(&filename);

        // Create a runtime for async operations
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            ShimError::ProcessExecution(format!("Failed to create async runtime: {}", e))
        })?;

        rt.block_on(async {
            let downloader = Downloader::new();
            downloader
                .download_if_missing(url, &download_path)
                .await
                .map_err(|e| {
                    ShimError::ProcessExecution(format!("Failed to download executable: {}", e))
                })
        })?;

        info!("Downloaded executable to: {}", download_path.display());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_runner_from_config() {
        let config = ShimConfig {
            shim: crate::config::ShimCore {
                name: "test".to_string(),
                path: "echo".to_string(),
                args: vec!["hello".to_string()],
                cwd: None,
                download_url: None,
            },
            args: Default::default(),
            env: std::collections::HashMap::new(),
            metadata: Default::default(),
            auto_update: None,
        };

        let runner = ShimRunner::from_config(config).unwrap();
        assert_eq!(runner.config().shim.name, "test");
    }

    #[test]
    fn test_runner_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
[shim]
name = "test"
path = "echo"
args = ["hello"]

[env]
TEST_VAR = "test_value"
        "#
        )
        .unwrap();

        let runner = ShimRunner::from_file(temp_file.path()).unwrap();
        assert_eq!(runner.config().shim.name, "test");
        assert_eq!(
            runner.config().env.get("TEST_VAR"),
            Some(&"test_value".to_string())
        );
    }
}
