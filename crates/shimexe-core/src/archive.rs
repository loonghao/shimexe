use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};
use zip::ZipArchive;

/// Archive extractor for handling compressed files
pub struct ArchiveExtractor;

impl ArchiveExtractor {
    /// Check if a file is a supported archive format
    pub fn is_archive(path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            matches!(extension.to_str(), Some("zip"))
        } else {
            false
        }
    }

    /// Check if a URL points to an archive file
    pub fn is_archive_url(url: &str) -> bool {
        url.ends_with(".zip")
    }

    /// Extract archive to destination directory and return list of extracted executables
    pub fn extract_archive(archive_path: &Path, dest_dir: &Path) -> Result<Vec<PathBuf>> {
        info!(
            "Extracting archive: {} to {}",
            archive_path.display(),
            dest_dir.display()
        );

        // Create destination directory if it doesn't exist
        fs::create_dir_all(dest_dir)
            .with_context(|| format!("Failed to create directory: {}", dest_dir.display()))?;

        let extension = archive_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        match extension {
            "zip" => Self::extract_zip(archive_path, dest_dir),
            _ => Err(anyhow::anyhow!("Unsupported archive format: {}", extension)),
        }
    }

    /// Extract ZIP archive
    fn extract_zip(archive_path: &Path, dest_dir: &Path) -> Result<Vec<PathBuf>> {
        let file = fs::File::open(archive_path)
            .with_context(|| format!("Failed to open archive: {}", archive_path.display()))?;

        let mut archive = ZipArchive::new(file).with_context(|| "Failed to read ZIP archive")?;

        let mut executables = Vec::new();

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .with_context(|| format!("Failed to read file at index {}", i))?;

            let outpath = match file.enclosed_name() {
                Some(path) => dest_dir.join(path),
                None => {
                    warn!("Skipping file with invalid name at index {}", i);
                    continue;
                }
            };

            debug!("Extracting: {}", outpath.display());

            if file.is_dir() {
                // Create directory
                fs::create_dir_all(&outpath).with_context(|| {
                    format!("Failed to create directory: {}", outpath.display())
                })?;
            } else {
                // Create parent directories if needed
                if let Some(parent) = outpath.parent() {
                    fs::create_dir_all(parent).with_context(|| {
                        format!("Failed to create parent directory: {}", parent.display())
                    })?;
                }

                // Extract file
                let mut outfile = fs::File::create(&outpath)
                    .with_context(|| format!("Failed to create file: {}", outpath.display()))?;

                std::io::copy(&mut file, &mut outfile)
                    .with_context(|| format!("Failed to extract file: {}", outpath.display()))?;

                // Make executable on Unix systems
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if Self::is_executable_file(&outpath) {
                        let mut perms = fs::metadata(&outpath)?.permissions();
                        perms.set_mode(0o755); // rwxr-xr-x
                        fs::set_permissions(&outpath, perms)?;
                    }
                }

                // Check if this is an executable file
                if Self::is_executable_file(&outpath) {
                    executables.push(outpath);
                }
            }
        }

        info!(
            "Extracted {} files, found {} executables",
            archive.len(),
            executables.len()
        );
        Ok(executables)
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

    /// Find all executable files in a directory (non-recursive)
    pub fn find_executables_in_dir(dir: &Path) -> Result<Vec<PathBuf>> {
        let mut executables = Vec::new();

        if !dir.exists() || !dir.is_dir() {
            return Ok(executables);
        }

        let entries = fs::read_dir(dir)
            .with_context(|| format!("Failed to read directory: {}", dir.display()))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && Self::is_executable_file(&path) {
                executables.push(path);
            }
        }

        Ok(executables)
    }

    /// Generate a unique name for an executable based on its filename
    pub fn generate_shim_name(executable_path: &Path, existing_names: &[String]) -> String {
        let base_name = executable_path
            .file_stem()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();

        // If the base name is unique, use it
        if !existing_names.contains(&base_name) {
            return base_name;
        }

        // Otherwise, append a number to make it unique
        for i in 1..=999 {
            let candidate = format!("{}-{}", base_name, i);
            if !existing_names.contains(&candidate) {
                return candidate;
            }
        }

        // Fallback: use timestamp
        format!("{}-{}", base_name, chrono::Utc::now().timestamp())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;
    use zip::write::{SimpleFileOptions, ZipWriter};

    fn create_test_zip(temp_dir: &Path) -> Result<PathBuf> {
        let zip_path = temp_dir.join("test.zip");
        let file = File::create(&zip_path)?;
        let mut zip = ZipWriter::new(file);

        // Add a test executable
        zip.start_file("test.exe", SimpleFileOptions::default())?;
        zip.write_all(b"fake executable content")?;

        // Add a regular file
        zip.start_file("readme.txt", SimpleFileOptions::default())?;
        zip.write_all(b"This is a readme file")?;

        // Add a directory
        zip.add_directory("subdir/", SimpleFileOptions::default())?;

        // Add another executable in subdirectory
        zip.start_file("subdir/tool.exe", SimpleFileOptions::default())?;
        zip.write_all(b"another fake executable")?;

        zip.finish()?;
        Ok(zip_path)
    }

    #[test]
    fn test_is_archive() {
        assert!(ArchiveExtractor::is_archive(Path::new("test.zip")));
        assert!(!ArchiveExtractor::is_archive(Path::new("test.exe")));
        assert!(!ArchiveExtractor::is_archive(Path::new("test")));
    }

    #[test]
    fn test_is_archive_url() {
        assert!(ArchiveExtractor::is_archive_url(
            "https://example.com/file.zip"
        ));
        assert!(!ArchiveExtractor::is_archive_url(
            "https://example.com/file.exe"
        ));
    }

    #[test]
    fn test_extract_zip() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let zip_path = create_test_zip(temp_dir.path())?;

        let extract_dir = temp_dir.path().join("extracted");
        let executables = ArchiveExtractor::extract_archive(&zip_path, &extract_dir)?;

        // Should find 2 executables
        assert_eq!(executables.len(), 2);

        // Check that files were extracted
        assert!(extract_dir.join("test.exe").exists());
        assert!(extract_dir.join("readme.txt").exists());
        assert!(extract_dir.join("subdir").is_dir());
        assert!(extract_dir.join("subdir/tool.exe").exists());

        Ok(())
    }

    #[test]
    fn test_generate_shim_name() {
        let path = Path::new("test.exe");
        let existing = vec![];
        assert_eq!(
            ArchiveExtractor::generate_shim_name(path, &existing),
            "test"
        );

        let existing = vec!["test".to_string()];
        assert_eq!(
            ArchiveExtractor::generate_shim_name(path, &existing),
            "test-1"
        );

        let existing = vec!["test".to_string(), "test-1".to_string()];
        assert_eq!(
            ArchiveExtractor::generate_shim_name(path, &existing),
            "test-2"
        );
    }
}
