use crate::archive::ArchiveExtractor;
use anyhow::{Context, Result};
use futures_util::StreamExt;
use reqwest::Client;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;
use tracing::{debug, info, warn};

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
        let url_path = url_path.split('#').next()?; // Remove fragment
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
    /// - https://github.com/user/repo/releases/download/v1.0/app.zip -> "app"
    pub fn infer_app_name_from_url(url: &str) -> Option<String> {
        let filename = Self::extract_filename_from_url(url)?;

        // Remove common executable and archive extensions
        let name = filename
            .strip_suffix(".exe")
            .or_else(|| filename.strip_suffix(".bin"))
            .or_else(|| filename.strip_suffix(".app"))
            .or_else(|| filename.strip_suffix(".zip"))
            .or_else(|| filename.strip_suffix(".tar.gz"))
            .or_else(|| filename.strip_suffix(".tgz"))
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

    /// Download a file from URL to the specified path with resume support
    pub async fn download_file(&self, url: &str, target_path: &Path) -> Result<()> {
        self.download_file_with_resume(url, target_path, true).await
    }

    /// Download a file from URL to the specified path with optional resume support
    pub async fn download_file_with_resume(
        &self,
        url: &str,
        target_path: &Path,
        allow_resume: bool,
    ) -> Result<()> {
        info!("Downloading {} to {}", url, target_path.display());

        // Create parent directories if they don't exist
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        // Check for existing partial download
        let mut start_pos = 0u64;
        if allow_resume && target_path.exists() {
            start_pos = fs::metadata(target_path)?.len();
            if start_pos > 0 {
                info!("Resuming download from byte {}", start_pos);
            }
        }

        // Build HTTP request with range header for resume
        let mut request = self.client.get(url);
        if start_pos > 0 {
            request = request.header("Range", format!("bytes={}-", start_pos));
        }

        let response = request
            .send()
            .await
            .with_context(|| format!("Failed to send request to {}", url))?;

        // Check if request was successful
        let status = response.status();
        if !status.is_success() && status != reqwest::StatusCode::PARTIAL_CONTENT {
            return Err(anyhow::anyhow!(
                "HTTP request failed with status: {}",
                status
            ));
        }

        // Get content length for progress tracking
        let content_length = response.content_length().unwrap_or(0);
        let total_size = if start_pos > 0 && status == reqwest::StatusCode::PARTIAL_CONTENT {
            start_pos + content_length
        } else {
            content_length
        };

        if total_size > 0 {
            info!("Download size: {} bytes", total_size);
        }

        // Open file for writing (append if resuming)
        let mut file = if start_pos > 0 {
            tokio::fs::OpenOptions::new()
                .append(true)
                .open(target_path)
                .await
        } else {
            tokio::fs::File::create(target_path).await
        }
        .with_context(|| format!("Failed to open file: {}", target_path.display()))?;

        // Download and write file content with optimized buffering
        let mut downloaded = start_pos;
        let mut stream = response.bytes_stream();
        let mut last_progress_report = std::time::Instant::now();
        let progress_interval = std::time::Duration::from_millis(500); // Report every 500ms

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.with_context(|| "Failed to read chunk from response")?;

            file.write_all(&chunk)
                .await
                .with_context(|| "Failed to write chunk to file")?;

            downloaded += chunk.len() as u64;

            // Optimized progress reporting - only report every 500ms for large files
            if total_size > 1024 * 1024 && last_progress_report.elapsed() >= progress_interval {
                let progress = if total_size > 0 {
                    (downloaded as f64 / total_size as f64) * 100.0
                } else {
                    0.0
                };
                debug!("Download progress: {:.1}% ({} bytes)", progress, downloaded);
                last_progress_report = std::time::Instant::now();
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

    /// Download file if it doesn't exist or is incomplete
    pub async fn download_if_missing(&self, url: &str, target_path: &Path) -> Result<bool> {
        if Self::file_exists(target_path) {
            // Check if file is complete by attempting a HEAD request
            if let Ok(expected_size) = self.get_remote_file_size(url).await {
                if let Ok(local_size) = fs::metadata(target_path).map(|m| m.len()) {
                    if local_size == expected_size {
                        debug!(
                            "File already exists and is complete: {}",
                            target_path.display()
                        );
                        return Ok(false);
                    } else {
                        debug!(
                            "File exists but incomplete: {} bytes, expected {} bytes",
                            local_size, expected_size
                        );
                    }
                }
            }
        }

        self.download_file(url, target_path).await?;
        Ok(true)
    }

    /// Get the size of a remote file using HEAD request
    pub async fn get_remote_file_size(&self, url: &str) -> Result<u64> {
        let response = self
            .client
            .head(url)
            .send()
            .await
            .with_context(|| format!("Failed to send HEAD request to {}", url))?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "HEAD request failed with status: {}",
                response.status()
            ));
        }

        response
            .content_length()
            .ok_or_else(|| anyhow::anyhow!("Server did not provide content length"))
    }

    /// Download multiple files concurrently with limited parallelism
    pub async fn download_files_concurrent(
        &self,
        downloads: Vec<(String, PathBuf)>, // (url, target_path) pairs
        max_concurrent: usize,
    ) -> Result<Vec<Result<()>>> {
        use futures_util::stream::{self, StreamExt};

        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(max_concurrent));
        let results = stream::iter(downloads)
            .map(|(url, target_path)| {
                let client = &self;
                let semaphore = semaphore.clone();
                async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    client.download_file(&url, &target_path).await
                }
            })
            .buffer_unordered(max_concurrent)
            .collect::<Vec<_>>()
            .await;

        Ok(results)
    }

    /// Download and extract archive, returning paths to all extracted executables
    pub async fn download_and_extract_archive(
        &self,
        url: &str,
        base_dir: &Path,
        app_name: &str,
    ) -> Result<Vec<PathBuf>> {
        let filename = Self::extract_filename_from_url(url)
            .ok_or_else(|| anyhow::anyhow!("Could not extract filename from URL: {}", url))?;

        // Download the archive
        let download_path = Self::generate_download_path(base_dir, app_name, &filename);
        let downloaded = self.download_if_missing(url, &download_path).await?;

        if downloaded {
            info!("Downloaded archive {} to {}", url, download_path.display());
        }

        // Check if it's an archive that needs extraction
        if ArchiveExtractor::is_archive(&download_path) {
            info!("Extracting archive: {}", download_path.display());

            // Extract to the same directory as the archive
            let extract_dir = download_path
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Could not determine extraction directory"))?;

            let executables = ArchiveExtractor::extract_archive(&download_path, extract_dir)
                .with_context(|| {
                    format!("Failed to extract archive: {}", download_path.display())
                })?;

            if executables.is_empty() {
                warn!(
                    "No executables found in archive: {}",
                    download_path.display()
                );
            } else {
                info!("Found {} executables in archive", executables.len());
            }

            Ok(executables)
        } else {
            // Not an archive, return the downloaded file if it's executable
            if Self::is_executable_file(&download_path) {
                Ok(vec![download_path])
            } else {
                Ok(vec![])
            }
        }
    }

    /// Check if a file is an executable based on its extension
    fn is_executable_file(path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            match extension.to_str() {
                Some("exe") | Some("bin") | Some("app") => true,
                Some(ext) if cfg!(unix) => {
                    // On Unix, also check for files without extension that might be executables
                    ext.is_empty() || matches!(ext, "sh" | "bash" | "zsh" | "fish")
                }
                _ => false,
            }
        } else {
            // Files without extension might be executables on Unix
            cfg!(unix)
        }
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

    #[test]
    fn test_infer_app_name_edge_cases() {
        // Test with query parameters
        assert_eq!(
            Downloader::infer_app_name_from_url(
                "https://example.com/tool.exe?download=1&version=latest"
            ),
            Some("tool".to_string())
        );

        // Test with multiple extensions (removes known archive extensions too)
        assert_eq!(
            Downloader::infer_app_name_from_url("https://example.com/my-tool.tar.gz"),
            Some("my-tool".to_string())
        );

        // Test with no extension
        assert_eq!(
            Downloader::infer_app_name_from_url("https://example.com/mytool"),
            Some("mytool".to_string())
        );

        // Test with empty filename
        assert_eq!(
            Downloader::infer_app_name_from_url("https://example.com/"),
            None
        );

        // Test with complex GitHub release URL
        assert_eq!(
            Downloader::infer_app_name_from_url("https://github.com/user/complex-tool-name/releases/download/v1.2.3/complex-tool-name-v1.2.3-windows-x64.exe"),
            Some("complex-tool-name-v1.2.3-windows-x64".to_string())
        );
    }

    #[test]
    fn test_extract_filename_edge_cases() {
        // Test with fragment
        assert_eq!(
            Downloader::extract_filename_from_url("https://example.com/file.exe#section"),
            Some("file.exe".to_string())
        );

        // Test with query and fragment
        assert_eq!(
            Downloader::extract_filename_from_url("https://example.com/file.exe?v=1#section"),
            Some("file.exe".to_string())
        );

        // Test with encoded characters
        assert_eq!(
            Downloader::extract_filename_from_url("https://example.com/my%20tool.exe"),
            Some("my%20tool.exe".to_string())
        );

        // Test with trailing slash
        assert_eq!(
            Downloader::extract_filename_from_url("https://example.com/path/"),
            None
        );
    }

    #[test]
    fn test_file_exists() {
        // Test with non-existent file
        assert!(!Downloader::file_exists(Path::new(
            "/non/existent/file.exe"
        )));

        // Test with a file that should exist (Cargo.toml in the crate root)
        let cargo_toml = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        assert!(Downloader::file_exists(&cargo_toml));
    }

    #[test]
    fn test_generate_download_path_cross_platform() {
        // Test Windows-style path
        let base = Path::new("C:\\Users\\user\\.shimexe");
        let path = Downloader::generate_download_path(base, "my-app", "my-app.exe");
        assert_eq!(
            path,
            Path::new("C:\\Users\\user\\.shimexe\\my-app\\bin\\my-app.exe")
        );

        // Test with special characters in app name
        let path = Downloader::generate_download_path(base, "my-app-v1.0", "tool.exe");
        assert_eq!(
            path,
            Path::new("C:\\Users\\user\\.shimexe\\my-app-v1.0\\bin\\tool.exe")
        );
    }
}
