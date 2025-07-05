use crate::archive::ArchiveExtractor;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tracing::{debug, warn};
use turbo_cdn::TurboCdn;

/// Turbo CDN-based downloader for remote executables
pub struct Downloader {
    client: TurboCdn,
}

impl Downloader {
    /// Create a new turbo CDN downloader instance
    pub async fn new() -> Result<Self> {
        let client = TurboCdn::new()
            .await
            .with_context(|| "Failed to create TurboCdn client")?;

        Ok(Self { client })
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

    /// Extract executable name from filename
    pub fn extract_exe_name_from_filename(filename: &str) -> Option<String> {
        let name = if filename.ends_with(".exe") {
            filename.strip_suffix(".exe")?
        } else if filename.ends_with(".zip") {
            filename.strip_suffix(".zip")?
        } else if filename.ends_with(".tar.gz") {
            filename.strip_suffix(".tar.gz")?
        } else if filename.ends_with(".tgz") {
            filename.strip_suffix(".tgz")?
        } else {
            filename
        };

        if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        }
    }

    /// Generate download path for an application
    /// Format: `<base_dir>/<app_name>/bin/<filename>`
    pub fn generate_download_path(base_dir: &Path, app_name: &str, filename: &str) -> PathBuf {
        base_dir.join(app_name).join("bin").join(filename)
    }

    /// Download a file from URL to the specified path using turbo-cdn
    pub async fn download_file(&mut self, url: &str, target_path: &Path) -> Result<()> {
        debug!(
            "Downloading {} to {} using turbo-cdn",
            url,
            target_path.display()
        );

        // Create parent directories if they don't exist
        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        // Use turbo-cdn to download from URL with automatic optimization
        let result = self
            .client
            .download_from_url(url)
            .await
            .with_context(|| format!("Failed to download from URL: {}", url))?;

        // Move the downloaded file to the target path if needed
        if result.path != target_path {
            std::fs::rename(&result.path, target_path).with_context(|| {
                format!(
                    "Failed to move downloaded file from {} to {}",
                    result.path.display(),
                    target_path.display()
                )
            })?;
        }

        debug!(
            "Download completed: {} bytes in {:.2}s at {:.2} MB/s",
            result.size,
            result.duration.as_secs_f64(),
            result.speed / 1_000_000.0
        );

        Ok(())
    }

    /// Check if file exists
    pub fn file_exists(path: &Path) -> bool {
        path.exists() && path.is_file()
    }

    /// Download file if it doesn't exist
    pub async fn download_if_missing(&mut self, url: &str, target_path: &Path) -> Result<bool> {
        if Self::file_exists(target_path) {
            debug!("File already exists: {}", target_path.display());
            return Ok(false);
        }

        self.download_file(url, target_path).await?;
        Ok(true)
    }

    /// Infer application name from URL
    /// Examples:
    /// - <https://github.com/user/repo/releases/download/v1.0/app.exe> -> "app"
    /// - <https://example.com/tools/my-tool.exe> -> "my-tool"
    /// - <https://github.com/user/repo/releases/download/v1.0/app.zip> -> "app"
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

    /// Download and extract archive, returning paths to all extracted executables
    pub async fn download_and_extract_archive(
        &mut self,
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
            debug!("Downloaded archive {} to {}", url, download_path.display());
        }

        // Check if it's an archive that needs extraction
        if ArchiveExtractor::is_archive(&download_path) {
            debug!("Extracting archive: {}", download_path.display());

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
                debug!("Found {} executables in archive", executables.len());
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
        // Note: This will panic if called, but it's required for the Default trait
        // In practice, users should use Downloader::new().await
        panic!("Downloader::default() cannot be used. Use Downloader::new().await instead")
    }
}
