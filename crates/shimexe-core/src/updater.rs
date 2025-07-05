use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info, warn};

use crate::config::{AutoUpdate, UpdateProvider, VersionCheck};
use crate::downloader::Downloader;
use crate::error::{Result, ShimError};
use crate::utils::expand_env_vars;

/// Shim updater that handles automatic updates
pub struct ShimUpdater {
    config: AutoUpdate,
    shim_path: PathBuf,
    executable_path: PathBuf,
}

impl ShimUpdater {
    /// Create a new shim updater
    pub fn new(config: AutoUpdate, shim_path: PathBuf, executable_path: PathBuf) -> Self {
        Self {
            config,
            shim_path,
            executable_path,
        }
    }

    /// Check if an update is needed and available
    pub async fn check_update_needed(&self) -> Result<Option<String>> {
        if !self.config.enabled {
            return Ok(None);
        }

        // Check if enough time has passed since last check
        if !self.should_check_for_updates()? {
            debug!("Skipping update check due to interval");
            return Ok(None);
        }

        // Get the latest version
        let latest_version = self.get_latest_version().await?;
        let current_version = self.get_current_version()?;

        if self.is_newer_version(&current_version, &latest_version)? {
            info!(
                "Update available: {} -> {}",
                current_version, latest_version
            );
            Ok(Some(latest_version))
        } else {
            debug!(
                "No update needed. Current: {}, Latest: {}",
                current_version, latest_version
            );
            Ok(None)
        }
    }

    /// Perform the update to the specified version
    pub async fn update_to_version(&self, version: &str) -> Result<()> {
        info!("Starting update to version {}", version);

        // Run pre-update command if specified
        if let Some(ref cmd) = self.config.pre_update_command {
            self.run_command(cmd, "pre-update")?;
        }

        // Download and install the new version
        match &self.config.provider {
            UpdateProvider::Github {
                repo,
                asset_pattern,
                ..
            } => {
                self.update_from_github(repo, asset_pattern, version)
                    .await?;
            }
            UpdateProvider::Https { base_url, .. } => {
                self.update_from_https(base_url, version).await?;
            }
            UpdateProvider::Custom { update_command, .. } => {
                self.update_from_custom(update_command, version)?;
            }
        }

        // Run post-update command if specified
        if let Some(ref cmd) = self.config.post_update_command {
            self.run_command(cmd, "post-update")?;
        }

        // Update the last check timestamp
        self.update_last_check_time()?;

        info!("Update completed successfully");
        Ok(())
    }

    /// Check if we should check for updates based on the interval
    fn should_check_for_updates(&self) -> Result<bool> {
        if self.config.check_interval_hours == 0 {
            return Ok(true); // Always check if interval is 0
        }

        let last_check_file = self.get_last_check_file();
        if !last_check_file.exists() {
            return Ok(true); // First time checking
        }

        let last_check_time = std::fs::read_to_string(&last_check_file)
            .map_err(ShimError::Io)?
            .trim()
            .parse::<u64>()
            .map_err(|_| ShimError::Config("Invalid last check timestamp".to_string()))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| ShimError::Config("System time error".to_string()))?
            .as_secs();

        let interval_seconds = self.config.check_interval_hours * 3600;
        Ok(now - last_check_time >= interval_seconds)
    }

    /// Get the latest version from the configured source
    async fn get_latest_version(&self) -> Result<String> {
        match &self.config.version_check {
            VersionCheck::GithubLatest {
                repo,
                include_prerelease,
            } => {
                self.get_github_latest_version(repo, *include_prerelease)
                    .await
            }
            VersionCheck::Http {
                url,
                json_path,
                regex_pattern,
            } => {
                self.get_http_version(url, json_path.as_deref(), regex_pattern.as_deref())
                    .await
            }
            VersionCheck::Semver { check_url, .. } => self.get_semver_version(check_url).await,
            VersionCheck::Command { command, args } => self.get_command_version(command, args),
        }
    }

    /// Get current version of the executable
    fn get_current_version(&self) -> Result<String> {
        // Try to get version from metadata first
        if let VersionCheck::Semver { current, .. } = &self.config.version_check {
            return Ok(current.clone());
        }

        // Try to run the executable with --version
        let output = std::process::Command::new(&self.executable_path)
            .arg("--version")
            .output()
            .map_err(|e| ShimError::ProcessExecution(e.to_string()))?;

        if output.status.success() {
            let version_output = String::from_utf8_lossy(&output.stdout);
            // Extract version using regex or simple parsing
            self.extract_version_from_output(&version_output)
        } else {
            Err(ShimError::Config(
                "Could not determine current version".to_string(),
            ))
        }
    }

    /// Check if the new version is newer than the current version
    fn is_newer_version(&self, current: &str, latest: &str) -> Result<bool> {
        // Simple string comparison for now
        // TODO: Implement proper semantic version comparison
        Ok(current != latest)
    }

    /// Update from GitHub releases
    async fn update_from_github(
        &self,
        repo: &str,
        asset_pattern: &str,
        version: &str,
    ) -> Result<()> {
        let download_url = self.build_github_download_url(repo, asset_pattern, version)?;
        self.download_and_install(&download_url).await
    }

    /// Update from HTTPS source
    async fn update_from_https(&self, _base_url: &str, version: &str) -> Result<()> {
        let download_url = expand_env_vars(&self.config.download_url)?
            .replace("{version}", version)
            .replace("{os}", std::env::consts::OS)
            .replace("{arch}", std::env::consts::ARCH);

        self.download_and_install(&download_url).await
    }

    /// Update using custom command
    fn update_from_custom(&self, update_command: &str, version: &str) -> Result<()> {
        let command = update_command.replace("{version}", version);
        self.run_command(&command, "custom-update")
    }

    /// Download and install from URL
    async fn download_and_install(&self, url: &str) -> Result<()> {
        info!("Downloading from: {}", url);

        // Create a temporary directory for download
        let temp_dir = tempfile::tempdir().map_err(ShimError::Io)?;
        let temp_file = temp_dir.path().join("download");

        // Download using turbo-cdn
        let mut downloader = Downloader::new().await?;
        downloader.download_file(url, &temp_file).await?;

        // Backup current executable
        let backup_path = self.executable_path.with_extension("backup");
        if self.executable_path.exists() {
            std::fs::copy(&self.executable_path, &backup_path).map_err(ShimError::Io)?;
        }

        // Replace the current executable
        std::fs::copy(&temp_file, &self.executable_path).map_err(ShimError::Io)?;

        // Set executable permissions on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&self.executable_path)
                .map_err(ShimError::Io)?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&self.executable_path, perms).map_err(ShimError::Io)?;
        }

        // Remove backup if everything succeeded
        if backup_path.exists() {
            std::fs::remove_file(&backup_path).map_err(ShimError::Io)?;
        }

        info!("Successfully installed update");
        Ok(())
    }

    /// Build GitHub download URL
    fn build_github_download_url(
        &self,
        repo: &str,
        asset_pattern: &str,
        version: &str,
    ) -> Result<String> {
        let asset_name = asset_pattern
            .replace("{version}", version)
            .replace("{os}", std::env::consts::OS)
            .replace("{arch}", std::env::consts::ARCH);

        Ok(format!(
            "https://github.com/{}/releases/download/v{}/{}",
            repo, version, asset_name
        ))
    }

    /// Get latest version from GitHub API
    async fn get_github_latest_version(
        &self,
        repo: &str,
        include_prerelease: bool,
    ) -> Result<String> {
        let api_url = if include_prerelease {
            format!("https://api.github.com/repos/{}/releases", repo)
        } else {
            format!("https://api.github.com/repos/{}/releases/latest", repo)
        };

        // Create a temporary file for the API response
        let temp_dir = tempfile::tempdir().map_err(ShimError::Io)?;
        let temp_file = temp_dir.path().join("github_api_response.json");

        // Download API response using turbo-cdn
        let mut downloader = Downloader::new().await?;
        downloader.download_file(&api_url, &temp_file).await?;

        // Read and parse the JSON response
        let response_content = std::fs::read_to_string(&temp_file).map_err(ShimError::Io)?;

        if include_prerelease {
            // Parse array of releases and find the latest
            let releases: serde_json::Value =
                serde_json::from_str(&response_content).map_err(|e| {
                    ShimError::Config(format!("Failed to parse GitHub API response: {}", e))
                })?;

            if let Some(releases_array) = releases.as_array() {
                if let Some(latest_release) = releases_array.first() {
                    if let Some(tag_name) = latest_release["tag_name"].as_str() {
                        return Ok(tag_name.trim_start_matches('v').to_string());
                    }
                }
            }
            Err(ShimError::Config("No releases found".to_string()))
        } else {
            // Parse single latest release
            let release: serde_json::Value =
                serde_json::from_str(&response_content).map_err(|e| {
                    ShimError::Config(format!("Failed to parse GitHub API response: {}", e))
                })?;

            if let Some(tag_name) = release["tag_name"].as_str() {
                Ok(tag_name.trim_start_matches('v').to_string())
            } else {
                Err(ShimError::Config(
                    "No tag_name found in release".to_string(),
                ))
            }
        }
    }

    /// Get version from HTTP endpoint
    async fn get_http_version(
        &self,
        url: &str,
        json_path: Option<&str>,
        regex_pattern: Option<&str>,
    ) -> Result<String> {
        // Create a temporary file for the HTTP response
        let temp_dir = tempfile::tempdir().map_err(ShimError::Io)?;
        let temp_file = temp_dir.path().join("http_response");

        // Download response using turbo-cdn
        let mut downloader = Downloader::new().await?;
        downloader.download_file(url, &temp_file).await?;

        // Read the response content
        let response_content = std::fs::read_to_string(&temp_file).map_err(ShimError::Io)?;

        // Extract version based on the specified method
        if let Some(json_path) = json_path {
            // Parse as JSON and extract using JSON path
            let json_value: serde_json::Value = serde_json::from_str(&response_content)
                .map_err(|e| ShimError::Config(format!("Failed to parse JSON response: {}", e)))?;

            // Simple JSON path extraction (supports basic dot notation)
            let version = self.extract_json_value(&json_value, json_path)?;
            Ok(version)
        } else if let Some(regex_pattern) = regex_pattern {
            // Extract using regex pattern
            let re = regex::Regex::new(regex_pattern)
                .map_err(|e| ShimError::Config(format!("Invalid regex pattern: {}", e)))?;

            if let Some(captures) = re.captures(&response_content) {
                if captures.len() > 1 {
                    Ok(captures[1].to_string())
                } else {
                    Ok(captures[0].to_string())
                }
            } else {
                Err(ShimError::Config("Regex pattern did not match".to_string()))
            }
        } else {
            // Try to extract version using default regex
            self.extract_version_from_output(&response_content)
        }
    }

    /// Get version using semver check
    async fn get_semver_version(&self, _check_url: &str) -> Result<String> {
        // TODO: Implement semver version check
        warn!("Semver version check not yet implemented");
        Ok("1.0.0".to_string())
    }

    /// Get version using command
    fn get_command_version(&self, command: &str, args: &[String]) -> Result<String> {
        let output = std::process::Command::new(command)
            .args(args)
            .output()
            .map_err(|e| ShimError::ProcessExecution(e.to_string()))?;

        if output.status.success() {
            let version_output = String::from_utf8_lossy(&output.stdout);
            self.extract_version_from_output(&version_output)
        } else {
            Err(ShimError::ProcessExecution(
                "Version command failed".to_string(),
            ))
        }
    }

    /// Extract version from command output
    fn extract_version_from_output(&self, output: &str) -> Result<String> {
        // Simple regex to extract version numbers
        let re = regex::Regex::new(r"(\d+\.\d+\.\d+)")
            .map_err(|e| ShimError::Config(format!("Regex error: {}", e)))?;

        if let Some(captures) = re.captures(output) {
            Ok(captures[1].to_string())
        } else {
            Err(ShimError::Config(
                "Could not extract version from output".to_string(),
            ))
        }
    }

    /// Extract value from JSON using simple dot notation path
    fn extract_json_value(&self, json: &serde_json::Value, path: &str) -> Result<String> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = json;

        for part in parts {
            if let Some(value) = current.get(part) {
                current = value;
            } else {
                return Err(ShimError::Config(format!("JSON path '{}' not found", path)));
            }
        }

        match current {
            serde_json::Value::String(s) => Ok(s.clone()),
            serde_json::Value::Number(n) => Ok(n.to_string()),
            _ => Err(ShimError::Config(format!(
                "JSON value at path '{}' is not a string or number",
                path
            ))),
        }
    }

    /// Run a command
    fn run_command(&self, command: &str, context: &str) -> Result<()> {
        debug!("Running {} command: {}", context, command);

        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .status()
            .map_err(|e| ShimError::ProcessExecution(e.to_string()))?;

        if status.success() {
            Ok(())
        } else {
            Err(ShimError::ProcessExecution(format!(
                "{context} command failed"
            )))
        }
    }

    /// Get the path to the last check timestamp file
    fn get_last_check_file(&self) -> PathBuf {
        self.shim_path.with_extension("last_check")
    }

    /// Update the last check timestamp
    fn update_last_check_time(&self) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| ShimError::Config("System time error".to_string()))?
            .as_secs();

        std::fs::write(self.get_last_check_file(), now.to_string()).map_err(ShimError::Io)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AutoUpdate, UpdateProvider, VersionCheck};
    use tempfile::NamedTempFile;

    fn create_test_auto_update() -> AutoUpdate {
        AutoUpdate {
            enabled: true,
            provider: UpdateProvider::Github {
                repo: "test/repo".to_string(),
                asset_pattern: "test-{version}-{os}-{arch}".to_string(),
                include_prerelease: false,
            },
            download_url: "https://github.com/test/repo/releases/download/v{version}/test-{version}-{os}-{arch}".to_string(),
            version_check: VersionCheck::GithubLatest {
                repo: "test/repo".to_string(),
                include_prerelease: false,
            },
            check_interval_hours: 24,
            pre_update_command: None,
            post_update_command: None,
        }
    }

    #[test]
    fn test_should_check_for_updates_first_time() {
        let auto_update = create_test_auto_update();
        let temp_shim = NamedTempFile::new().unwrap();
        let temp_exe = NamedTempFile::new().unwrap();

        let updater = ShimUpdater::new(
            auto_update,
            temp_shim.path().to_path_buf(),
            temp_exe.path().to_path_buf(),
        );

        // First time should always check
        assert!(updater.should_check_for_updates().unwrap());
    }

    #[test]
    fn test_should_check_for_updates_zero_interval() {
        let mut auto_update = create_test_auto_update();
        auto_update.check_interval_hours = 0;

        let temp_shim = NamedTempFile::new().unwrap();
        let temp_exe = NamedTempFile::new().unwrap();

        let updater = ShimUpdater::new(
            auto_update,
            temp_shim.path().to_path_buf(),
            temp_exe.path().to_path_buf(),
        );

        // Zero interval should always check
        assert!(updater.should_check_for_updates().unwrap());
    }

    #[test]
    fn test_build_github_download_url() {
        let auto_update = create_test_auto_update();
        let temp_shim = NamedTempFile::new().unwrap();
        let temp_exe = NamedTempFile::new().unwrap();

        let updater = ShimUpdater::new(
            auto_update,
            temp_shim.path().to_path_buf(),
            temp_exe.path().to_path_buf(),
        );

        let url = updater
            .build_github_download_url("test/repo", "app-{version}-{os}-{arch}", "1.0.0")
            .unwrap();
        let expected = format!(
            "https://github.com/test/repo/releases/download/v1.0.0/app-1.0.0-{}-{}",
            std::env::consts::OS,
            std::env::consts::ARCH
        );
        assert_eq!(url, expected);
    }

    #[test]
    fn test_extract_version_from_output() {
        let auto_update = create_test_auto_update();
        let temp_shim = NamedTempFile::new().unwrap();
        let temp_exe = NamedTempFile::new().unwrap();

        let updater = ShimUpdater::new(
            auto_update,
            temp_shim.path().to_path_buf(),
            temp_exe.path().to_path_buf(),
        );

        // Test various version output formats
        assert_eq!(
            updater
                .extract_version_from_output("version 1.2.3")
                .unwrap(),
            "1.2.3"
        );
        assert_eq!(
            updater.extract_version_from_output("v1.2.3").unwrap(),
            "1.2.3"
        );
        assert_eq!(
            updater
                .extract_version_from_output("app 1.2.3 (build 123)")
                .unwrap(),
            "1.2.3"
        );

        // Test invalid format
        assert!(updater
            .extract_version_from_output("no version here")
            .is_err());
    }

    #[test]
    fn test_get_last_check_file() {
        let auto_update = create_test_auto_update();
        let temp_shim = NamedTempFile::new().unwrap();
        let temp_exe = NamedTempFile::new().unwrap();

        let updater = ShimUpdater::new(
            auto_update,
            temp_shim.path().to_path_buf(),
            temp_exe.path().to_path_buf(),
        );

        let last_check_file = updater.get_last_check_file();
        assert_eq!(
            last_check_file,
            temp_shim.path().with_extension("last_check")
        );
    }

    #[test]
    fn test_update_last_check_time() {
        let auto_update = create_test_auto_update();
        let temp_shim = NamedTempFile::new().unwrap();
        let temp_exe = NamedTempFile::new().unwrap();

        let updater = ShimUpdater::new(
            auto_update,
            temp_shim.path().to_path_buf(),
            temp_exe.path().to_path_buf(),
        );

        // Update timestamp
        updater.update_last_check_time().unwrap();

        // Check that file was created and contains a timestamp
        let last_check_file = updater.get_last_check_file();
        assert!(last_check_file.exists());

        let content = std::fs::read_to_string(&last_check_file).unwrap();
        let timestamp: u64 = content.trim().parse().unwrap();
        assert!(timestamp > 0);
    }
}
