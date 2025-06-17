use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

use crate::error::{Result, ShimError};
use crate::template::ArgsConfig;
use crate::utils::expand_env_vars;

/// Configuration cache entry
#[derive(Debug, Clone)]
struct CacheEntry {
    config: ShimConfig,
    last_modified: SystemTime,
    cached_at: SystemTime,
}

/// Configuration cache for improved performance
#[derive(Debug, Clone)]
pub struct ConfigCache {
    cache: Arc<Mutex<HashMap<PathBuf, CacheEntry>>>,
    ttl: Duration,
}

impl ConfigCache {
    /// Create a new configuration cache with specified TTL
    pub fn new(ttl: Duration) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            ttl,
        }
    }

    /// Get configuration from cache or load from file
    pub fn get_or_load<P: AsRef<Path>>(&self, path: P) -> Result<ShimConfig> {
        let path = path.as_ref().to_path_buf();
        let now = SystemTime::now();

        // Check cache first
        if let Ok(cache) = self.cache.lock() {
            if let Some(entry) = cache.get(&path) {
                // Check if cache entry is still valid
                if now.duration_since(entry.cached_at).unwrap_or(Duration::MAX) < self.ttl {
                    // Check if file hasn't been modified
                    if let Ok(metadata) = std::fs::metadata(&path) {
                        if let Ok(modified) = metadata.modified() {
                            if modified <= entry.last_modified {
                                return Ok(entry.config.clone());
                            }
                        }
                    }
                }
            }
        }

        // Load from file and update cache
        let config = ShimConfig::from_file(&path)?;
        let last_modified = std::fs::metadata(&path)
            .and_then(|m| m.modified())
            .unwrap_or(now);

        if let Ok(mut cache) = self.cache.lock() {
            cache.insert(
                path,
                CacheEntry {
                    config: config.clone(),
                    last_modified,
                    cached_at: now,
                },
            );
        }

        Ok(config)
    }

    /// Invalidate cache entry for a specific path
    pub fn invalidate<P: AsRef<Path>>(&self, path: P) {
        let path = path.as_ref().to_path_buf();
        if let Ok(mut cache) = self.cache.lock() {
            cache.remove(&path);
        }
    }

    /// Clear all cache entries
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }

    /// Get cache statistics
    pub fn stats(&self) -> (usize, usize) {
        if let Ok(cache) = self.cache.lock() {
            let total = cache.len();
            let now = SystemTime::now();
            let valid = cache
                .values()
                .filter(|entry| {
                    now.duration_since(entry.cached_at).unwrap_or(Duration::MAX) < self.ttl
                })
                .count();
            (total, valid)
        } else {
            (0, 0)
        }
    }
}

impl Default for ConfigCache {
    /// Create a new configuration cache with default TTL (5 minutes)
    fn default() -> Self {
        Self::new(Duration::from_secs(300))
    }
}

/// Main shim configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShimConfig {
    /// Core shim configuration
    pub shim: ShimCore,
    /// Advanced argument configuration
    #[serde(default)]
    pub args: ArgsConfig,
    /// Environment variables to set
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Optional metadata
    #[serde(default)]
    pub metadata: ShimMetadata,
    /// Auto-update configuration
    #[serde(default)]
    pub auto_update: Option<AutoUpdate>,
}

/// Core shim configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShimCore {
    /// Name of the shim
    pub name: String,
    /// Path to the target executable
    pub path: String,
    /// Default arguments to pass to the executable
    #[serde(default)]
    pub args: Vec<String>,
    /// Working directory for the executable
    #[serde(default)]
    pub cwd: Option<String>,
    /// Original download URL (for HTTP-based shims)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// Type of source (file, archive, url)
    #[serde(default)]
    pub source_type: SourceType,
    /// For archives: list of extracted executables
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extracted_executables: Vec<ExtractedExecutable>,
}

/// Source type for the shim
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    /// Regular file or executable
    #[default]
    File,
    /// Archive file (zip, tar.gz, etc.)
    Archive,
    /// HTTP URL
    Url,
}

/// Information about an extracted executable from an archive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedExecutable {
    /// Name of the executable (without extension)
    pub name: String,
    /// Relative path within the extracted archive
    pub path: String,
    /// Full path to the executable
    pub full_path: String,
    /// Whether this executable is the primary one for this shim
    #[serde(default)]
    pub is_primary: bool,
}

/// Optional metadata for the shim
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShimMetadata {
    /// Description of the shim
    pub description: Option<String>,
    /// Version of the shim configuration
    pub version: Option<String>,
    /// Author of the shim
    pub author: Option<String>,
    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Auto-update configuration for the shim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoUpdate {
    /// Enable auto-update
    #[serde(default)]
    pub enabled: bool,
    /// Update provider type
    pub provider: UpdateProvider,
    /// Download URL template with version placeholder
    pub download_url: String,
    /// Version check URL or pattern
    pub version_check: VersionCheck,
    /// Update frequency in hours (0 = check every run)
    #[serde(default)]
    pub check_interval_hours: u64,
    /// Pre-update command to run
    pub pre_update_command: Option<String>,
    /// Post-update command to run
    pub post_update_command: Option<String>,
}

/// Update provider types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UpdateProvider {
    /// GitHub releases
    Github {
        /// Repository owner/name
        repo: String,
        /// Asset name pattern (supports {version}, {os}, {arch} placeholders)
        asset_pattern: String,
        /// Include pre-releases
        #[serde(default)]
        include_prerelease: bool,
    },
    /// Direct HTTPS download
    Https {
        /// Base URL for downloads
        base_url: String,
        /// URL pattern for version checking
        version_url: Option<String>,
    },
    /// Custom provider
    Custom {
        /// Custom update script or command
        update_command: String,
        /// Custom version check command
        version_command: String,
    },
}

/// Version check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionCheck {
    /// GitHub API for latest release
    GithubLatest {
        /// Repository owner/name
        repo: String,
        /// Include pre-releases
        #[serde(default)]
        include_prerelease: bool,
    },
    /// HTTP endpoint returning version
    Http {
        /// URL to check for version
        url: String,
        /// JSON path to extract version (e.g., "$.version")
        json_path: Option<String>,
        /// Regex pattern to extract version
        regex_pattern: Option<String>,
    },
    /// Semantic version comparison
    Semver {
        /// Current version
        current: String,
        /// Version check URL
        check_url: String,
    },
    /// Custom version check command
    Command {
        /// Command to run for version check
        command: String,
        /// Arguments for the command
        args: Vec<String>,
    },
}

impl ShimConfig {
    /// Load shim configuration from a TOML file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(&path).map_err(ShimError::Io)?;

        let config: ShimConfig = toml::from_str(&content).map_err(ShimError::TomlParse)?;

        config.validate()?;
        Ok(config)
    }

    /// Load shim configuration from a TOML file asynchronously
    pub async fn from_file_async<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(ShimError::Io)?;

        let config: ShimConfig = toml::from_str(&content).map_err(ShimError::TomlParse)?;

        config.validate()?;
        Ok(config)
    }

    /// Save shim configuration to a TOML file
    pub fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        // Check if file already exists and has the same content to avoid unnecessary writes
        if let Ok(existing_content) = std::fs::read_to_string(&path) {
            let new_content = toml::to_string_pretty(self).map_err(ShimError::TomlSerialize)?;
            if existing_content.trim() == new_content.trim() {
                return Ok(()); // No changes needed
            }
        }

        let content = toml::to_string_pretty(self).map_err(ShimError::TomlSerialize)?;
        std::fs::write(path, content).map_err(ShimError::Io)?;

        Ok(())
    }

    /// Save shim configuration to a TOML file asynchronously
    pub async fn to_file_async<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        // Check if file already exists and has the same content to avoid unnecessary writes
        if let Ok(existing_content) = tokio::fs::read_to_string(&path).await {
            let new_content = toml::to_string_pretty(self).map_err(ShimError::TomlSerialize)?;
            if existing_content.trim() == new_content.trim() {
                return Ok(()); // No changes needed
            }
        }

        let content = toml::to_string_pretty(self).map_err(ShimError::TomlSerialize)?;
        tokio::fs::write(path, content)
            .await
            .map_err(ShimError::Io)?;

        Ok(())
    }

    /// Load multiple configuration files concurrently
    pub async fn from_files_concurrent<P: AsRef<std::path::Path>>(
        paths: Vec<P>,
    ) -> Vec<Result<Self>> {
        use futures_util::future::join_all;

        let futures = paths
            .into_iter()
            .map(|path| Self::from_file_async(path))
            .collect::<Vec<_>>();

        join_all(futures).await
    }

    /// Save multiple configurations concurrently
    pub async fn to_files_concurrent<P: AsRef<std::path::Path>>(
        configs_and_paths: Vec<(&Self, P)>,
    ) -> Vec<Result<()>> {
        use futures_util::future::join_all;

        let futures = configs_and_paths
            .into_iter()
            .map(|(config, path)| config.to_file_async(path))
            .collect::<Vec<_>>();

        join_all(futures).await
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.shim.name.is_empty() {
            return Err(ShimError::Config("Shim name cannot be empty".to_string()));
        }

        if self.shim.path.is_empty() {
            return Err(ShimError::Config("Shim path cannot be empty".to_string()));
        }

        Ok(())
    }

    /// Expand environment variables in the configuration
    pub fn expand_env_vars(&mut self) -> Result<()> {
        // Expand path
        self.shim.path = expand_env_vars(&self.shim.path)?;

        // Expand args
        for arg in &mut self.shim.args {
            *arg = expand_env_vars(arg)?;
        }

        // Expand cwd if present
        if let Some(ref mut cwd) = self.shim.cwd {
            *cwd = expand_env_vars(cwd)?;
        }

        // Expand environment variables
        for value in self.env.values_mut() {
            *value = expand_env_vars(value)?;
        }

        Ok(())
    }

    /// Get the resolved executable path
    pub fn get_executable_path(&self) -> Result<PathBuf> {
        let expanded_path = expand_env_vars(&self.shim.path)?;

        match self.shim.source_type {
            SourceType::Archive => {
                // For archives, use the primary executable or the first one
                if let Some(primary_exe) = self
                    .shim
                    .extracted_executables
                    .iter()
                    .find(|exe| exe.is_primary)
                    .or_else(|| self.shim.extracted_executables.first())
                {
                    let path = PathBuf::from(&primary_exe.full_path);
                    if path.exists() {
                        Ok(path)
                    } else {
                        Err(ShimError::ExecutableNotFound(format!(
                            "Extracted executable not found: {}. Re-extraction may be required.",
                            primary_exe.full_path
                        )))
                    }
                } else {
                    Err(ShimError::ExecutableNotFound(
                        "No extracted executables found in archive configuration".to_string(),
                    ))
                }
            }
            SourceType::Url => {
                // Check if we have a download_url (indicating this was originally from HTTP)
                if let Some(ref download_url) = self.shim.download_url {
                    // This shim was created from an HTTP URL
                    let filename =
                        crate::downloader::Downloader::extract_filename_from_url(download_url)
                            .ok_or_else(|| {
                                ShimError::Config(format!(
                                    "Could not extract filename from download URL: {}",
                                    download_url
                                ))
                            })?;

                    // Try to find the downloaded file in the expected location
                    // First try relative to home directory
                    if let Some(home_dir) = dirs::home_dir() {
                        let download_path = home_dir
                            .join(".shimexe")
                            .join(&self.shim.name)
                            .join("bin")
                            .join(&filename);

                        if download_path.exists() {
                            return Ok(download_path);
                        }
                    }

                    // If not found, return an error indicating download is needed
                    Err(ShimError::ExecutableNotFound(format!(
                        "Executable not found for download URL: {}. Download may be required.",
                        download_url
                    )))
                } else if crate::downloader::Downloader::is_url(&expanded_path) {
                    // Legacy: path is still a URL (for backward compatibility)
                    let filename =
                        crate::downloader::Downloader::extract_filename_from_url(&expanded_path)
                            .ok_or_else(|| {
                                ShimError::Config(format!(
                                    "Could not extract filename from URL: {}",
                                    expanded_path
                                ))
                            })?;

                    // Try to find the downloaded file in the expected location
                    if let Some(home_dir) = dirs::home_dir() {
                        let download_path = home_dir
                            .join(".shimexe")
                            .join(&self.shim.name)
                            .join("bin")
                            .join(&filename);

                        if download_path.exists() {
                            return Ok(download_path);
                        }
                    }

                    // If not found, return an error indicating download is needed
                    Err(ShimError::ExecutableNotFound(format!(
                        "Executable not found for URL: {}. Download may be required.",
                        expanded_path
                    )))
                } else {
                    Err(ShimError::Config(
                        "URL source type specified but no download URL found".to_string(),
                    ))
                }
            }
            SourceType::File => {
                let path = PathBuf::from(expanded_path);

                if path.is_absolute() {
                    Ok(path)
                } else {
                    // Try to find in PATH
                    which::which(&path)
                        .map_err(|_| ShimError::ExecutableNotFound(self.shim.path.clone()))
                }
            }
        }
    }

    /// Get the download URL for this shim (if it was created from HTTP)
    pub fn get_download_url(&self) -> Option<&String> {
        self.shim.download_url.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_shim_config_from_file() {
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

[metadata]
description = "Test shim"
version = "1.0.0"
        "#
        )
        .unwrap();

        let config = ShimConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(config.shim.name, "test");
        assert_eq!(config.shim.path, "echo");
        assert_eq!(config.shim.args, vec!["hello"]);
        assert_eq!(config.env.get("TEST_VAR"), Some(&"test_value".to_string()));
        assert_eq!(config.metadata.description, Some("Test shim".to_string()));
    }

    #[test]
    fn test_shim_config_basic_structure() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
[shim]
name = "test"
path = "echo"

[args]
mode = "template"

[metadata]
description = "Test shim"
        "#
        )
        .unwrap();

        let config = ShimConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(config.shim.name, "test");
        assert_eq!(config.args.mode, crate::template::ArgsMode::Template);
        assert_eq!(config.metadata.description, Some("Test shim".to_string()));
    }

    #[test]
    fn test_shim_config_validation() {
        // Valid config
        let config = ShimConfig {
            shim: ShimCore {
                name: "test".to_string(),
                path: "echo".to_string(),
                args: vec![],
                cwd: None,
                download_url: None,
                source_type: SourceType::File,
                extracted_executables: vec![],
            },
            args: Default::default(),
            env: HashMap::new(),
            metadata: Default::default(),
            auto_update: None,
        };
        assert!(config.validate().is_ok());

        // Invalid config - empty name
        let invalid_config = ShimConfig {
            shim: ShimCore {
                name: "".to_string(),
                path: "echo".to_string(),
                args: vec![],
                cwd: None,
                download_url: None,
                source_type: SourceType::File,
                extracted_executables: vec![],
            },
            args: Default::default(),
            env: HashMap::new(),
            metadata: Default::default(),
            auto_update: None,
        };
        assert!(invalid_config.validate().is_err());

        // Invalid config - empty path
        let invalid_config = ShimConfig {
            shim: ShimCore {
                name: "test".to_string(),
                path: "".to_string(),
                args: vec![],
                cwd: None,
                download_url: None,
                source_type: SourceType::File,
                extracted_executables: vec![],
            },
            args: Default::default(),
            env: HashMap::new(),
            metadata: Default::default(),
            auto_update: None,
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_shim_config_to_file() {
        let config = ShimConfig {
            shim: ShimCore {
                name: "test".to_string(),
                path: "echo".to_string(),
                args: vec!["hello".to_string()],
                cwd: None,
                download_url: None,
                source_type: SourceType::File,
                extracted_executables: vec![],
            },
            args: Default::default(),
            env: {
                let mut env = HashMap::new();
                env.insert("TEST_VAR".to_string(), "test_value".to_string());
                env
            },
            metadata: ShimMetadata {
                description: Some("Test shim".to_string()),
                version: Some("1.0.0".to_string()),
                author: None,
                tags: vec![],
            },
            auto_update: None,
        };

        let temp_file = NamedTempFile::new().unwrap();
        config.to_file(temp_file.path()).unwrap();

        // Read back and verify
        let loaded_config = ShimConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(loaded_config.shim.name, config.shim.name);
        assert_eq!(loaded_config.shim.path, config.shim.path);
        assert_eq!(loaded_config.shim.args, config.shim.args);
        assert_eq!(loaded_config.env, config.env);
    }

    #[test]
    fn test_expand_env_vars() {
        std::env::set_var("TEST_VAR", "test_value");

        let mut config = ShimConfig {
            shim: ShimCore {
                name: "test".to_string(),
                path: "${TEST_VAR}/bin/test".to_string(),
                args: vec!["${TEST_VAR}".to_string()],
                cwd: Some("${TEST_VAR}/work".to_string()),
                download_url: None,
                source_type: SourceType::File,
                extracted_executables: vec![],
            },
            args: Default::default(),
            env: {
                let mut env = HashMap::new();
                env.insert("EXPANDED".to_string(), "${TEST_VAR}_expanded".to_string());
                env
            },
            metadata: Default::default(),
            auto_update: None,
        };

        config.expand_env_vars().unwrap();

        assert_eq!(config.shim.path, "test_value/bin/test");
        assert_eq!(config.shim.args[0], "test_value");
        assert_eq!(config.shim.cwd, Some("test_value/work".to_string()));
        assert_eq!(
            config.env.get("EXPANDED"),
            Some(&"test_value_expanded".to_string())
        );

        std::env::remove_var("TEST_VAR");
    }

    #[test]
    fn test_shim_config_with_args_template() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(
            temp_file,
            r#"
[shim]
name = "test"
path = "echo"

[args]
mode = "template"
template = [
    "{{{{if env('DEBUG') == 'true'}}}}--verbose{{{{endif}}}}",
    "{{{{args('--version')}}}}"
]

[metadata]
description = "Test shim with template args"
        "#
        )
        .unwrap();

        let config = ShimConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(config.shim.name, "test");
        assert_eq!(config.args.mode, crate::template::ArgsMode::Template);
        assert!(config.args.template.is_some());

        let template = config.args.template.unwrap();
        assert_eq!(template.len(), 2);
        assert_eq!(
            template[0],
            "{{if env('DEBUG') == 'true'}}--verbose{{endif}}"
        );
        assert_eq!(template[1], "{{args('--version')}}");
    }

    #[test]
    fn test_shim_config_with_args_modes() {
        // Test merge mode
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
[shim]
name = "test"
path = "echo"

[args]
mode = "merge"
default = ["--default"]
prefix = ["--prefix"]
suffix = ["--suffix"]
        "#
        )
        .unwrap();

        let config = ShimConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(config.args.mode, crate::template::ArgsMode::Merge);
        assert_eq!(config.args.default, vec!["--default"]);
        assert_eq!(config.args.prefix, vec!["--prefix"]);
        assert_eq!(config.args.suffix, vec!["--suffix"]);
    }

    #[test]
    fn test_shim_config_with_inline_template() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(
            temp_file,
            r#"
[shim]
name = "test"
path = "echo"

[args]
inline = "{{{{env('CONFIG_PATH', '/default/config')}}}} {{{{args('--help')}}}}"
        "#
        )
        .unwrap();

        let config = ShimConfig::from_file(temp_file.path()).unwrap();
        assert!(config.args.inline.is_some());
        assert_eq!(
            config.args.inline.unwrap(),
            "{{env('CONFIG_PATH', '/default/config')}} {{args('--help')}}"
        );
    }

    #[test]
    fn test_shim_config_with_download_url() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
[shim]
name = "test-tool"
path = "/home/user/.shimexe/test-tool/bin/test-tool.exe"
download_url = "https://example.com/test-tool.exe"

[metadata]
description = "Test shim with download URL"
        "#
        )
        .unwrap();

        let config = ShimConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(config.shim.name, "test-tool");
        assert_eq!(
            config.shim.path,
            "/home/user/.shimexe/test-tool/bin/test-tool.exe"
        );
        assert_eq!(
            config.shim.download_url,
            Some("https://example.com/test-tool.exe".to_string())
        );
        assert_eq!(
            config.get_download_url(),
            Some(&"https://example.com/test-tool.exe".to_string())
        );
    }

    #[test]
    fn test_shim_config_without_download_url() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
[shim]
name = "local-tool"
path = "/usr/bin/local-tool"

[metadata]
description = "Test shim without download URL"
        "#
        )
        .unwrap();

        let config = ShimConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(config.shim.name, "local-tool");
        assert_eq!(config.shim.path, "/usr/bin/local-tool");
        assert_eq!(config.shim.download_url, None);
        assert_eq!(config.get_download_url(), None);
    }
}
