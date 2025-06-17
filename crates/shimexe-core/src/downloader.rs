use anyhow::{Context, Result};
use futures_util::StreamExt;
use reqwest::Client;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;
use tracing::{debug, info};

/// HTTP downloader for remote executables
pub struct Downloader {
    client: Client,
}

impl Downloader {
    /// Create a new downloader instance
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("shimexe/0.1.3")
            .timeout(std::time::Duration::from_secs(300)) // 5 minutes timeout
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    /// Check if a string is a valid HTTP/HTTPS URL
    pub fn is_url(path: &str) -> bool {
        path.starts_with("http://") || path.starts_with("https://")
    }

    /// Extract filename from URL
    pub fn extract_filename_from_url(url: &str) -> Option<String> {
        let url_path = url.split('?').next()?; // Remove query parameters
        let filename = url_path.split('/').next_back()?;

        if filename.is_empty() {
            None
        } else {
            Some(filename.to_string())
        }
    }

    /// Infer application name from URL
    /// Examples:
    /// - https://github.com/user/repo/releases/download/v1.0/app.exe -> "app"
    /// - https://example.com/tools/my-tool.exe -> "my-tool"
    pub fn infer_app_name_from_url(url: &str) -> Option<String> {
        let filename = Self::extract_filename_from_url(url)?;

        // Remove common executable extensions
        let name = filename
            .strip_suffix(".exe")
            .or_else(|| filename.strip_suffix(".bin"))
            .or_else(|| filename.strip_suffix(".app"))
            .unwrap_or(&filename);

        if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        }
    }

    /// Generate download path for an application
    /// Format: <base_dir>/<app_name>/bin/<filename>
    pub fn generate_download_path(base_dir: &Path, app_name: &str, filename: &str) -> PathBuf {
        base_dir.join(app_name).join("bin").join(filename)
    }

    /// Download a file from URL to the specified path
    pub async fn download_file(&self, url: &str, target_path: &Path) -> Result<()> {
        info!("Downloading {} to {}", url, target_path.display());

        // Create parent directories if they don't exist
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        // Send HTTP request
        let response = self
            .client
            .get(url)
            .send()
            .await
            .with_context(|| format!("Failed to send request to {}", url))?;

        // Check if request was successful
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "HTTP request failed with status: {}",
                response.status()
            ));
        }

        // Get content length for progress tracking
        let total_size = response.content_length();
        if let Some(size) = total_size {
            info!("Download size: {} bytes", size);
        }

        // Create the target file
        let mut file = tokio::fs::File::create(target_path)
            .await
            .with_context(|| format!("Failed to create file: {}", target_path.display()))?;

        // Download and write file content
        let mut downloaded = 0u64;
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.with_context(|| "Failed to read chunk from response")?;

            file.write_all(&chunk)
                .await
                .with_context(|| "Failed to write chunk to file")?;

            downloaded += chunk.len() as u64;

            // Log progress for large files
            if let Some(total) = total_size {
                if total > 1024 * 1024 {
                    // Only show progress for files > 1MB
                    let progress = (downloaded as f64 / total as f64) * 100.0;
                    if downloaded % (total / 10).max(1) == 0 {
                        debug!("Download progress: {:.1}%", progress);
                    }
                }
            }
        }

        file.flush().await.with_context(|| "Failed to flush file")?;

        // Make file executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(target_path)?.permissions();
            perms.set_mode(0o755); // rwxr-xr-x
            fs::set_permissions(target_path, perms)?;
        }

        info!(
            "Successfully downloaded {} bytes to {}",
            downloaded,
            target_path.display()
        );
        Ok(())
    }

    /// Check if a file exists at the given path
    pub fn file_exists(path: &Path) -> bool {
        path.exists() && path.is_file()
    }

    /// Download file if it doesn't exist
    pub async fn download_if_missing(&self, url: &str, target_path: &Path) -> Result<bool> {
        if Self::file_exists(target_path) {
            debug!("File already exists: {}", target_path.display());
            return Ok(false);
        }

        self.download_file(url, target_path).await?;
        Ok(true)
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_url() {
        assert!(Downloader::is_url("https://example.com/file.exe"));
        assert!(Downloader::is_url("http://example.com/file.exe"));
        assert!(!Downloader::is_url("/local/path/file.exe"));
        assert!(!Downloader::is_url("C:\\Windows\\file.exe"));
        assert!(!Downloader::is_url("file.exe"));
    }

    #[test]
    fn test_extract_filename_from_url() {
        assert_eq!(
            Downloader::extract_filename_from_url("https://example.com/file.exe"),
            Some("file.exe".to_string())
        );
        assert_eq!(
            Downloader::extract_filename_from_url(
                "https://github.com/user/repo/releases/download/v1.0/app.exe?download=1"
            ),
            Some("app.exe".to_string())
        );
        assert_eq!(
            Downloader::extract_filename_from_url("https://example.com/"),
            None
        );
    }

    #[test]
    fn test_infer_app_name_from_url() {
        assert_eq!(
            Downloader::infer_app_name_from_url("https://example.com/my-app.exe"),
            Some("my-app".to_string())
        );
        assert_eq!(
            Downloader::infer_app_name_from_url("https://github.com/user/installer-analyzer/releases/download/v0.7.0/installer-analyzer.exe"),
            Some("installer-analyzer".to_string())
        );
        assert_eq!(
            Downloader::infer_app_name_from_url("https://example.com/tool.bin"),
            Some("tool".to_string())
        );
    }

    #[test]
    fn test_generate_download_path() {
        let base = Path::new("/home/user/.shimexe");
        let path = Downloader::generate_download_path(base, "my-app", "my-app.exe");
        assert_eq!(path, Path::new("/home/user/.shimexe/my-app/bin/my-app.exe"));
    }
}
