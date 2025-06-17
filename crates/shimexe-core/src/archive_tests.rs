#[cfg(test)]
mod archive_integration_tests {
    use crate::archive::ArchiveExtractor;
    use crate::downloader::Downloader;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;
    use zip::write::{SimpleFileOptions, ZipWriter};

    fn create_test_zip_with_executables(
        temp_dir: &std::path::Path,
    ) -> anyhow::Result<std::path::PathBuf> {
        let zip_path = temp_dir.join("test-tools.zip");
        let file = fs::File::create(&zip_path)?;
        let mut zip = ZipWriter::new(file);

        // Add multiple executables
        zip.start_file("tool1.exe", SimpleFileOptions::default())?;
        zip.write_all(b"fake executable 1")?;

        zip.start_file("tool2.exe", SimpleFileOptions::default())?;
        zip.write_all(b"fake executable 2")?;

        // Add a subdirectory with an executable
        zip.add_directory("subdir/", SimpleFileOptions::default())?;
        zip.start_file("subdir/tool3.exe", SimpleFileOptions::default())?;
        zip.write_all(b"fake executable 3")?;

        // Add a non-executable file
        zip.start_file("readme.txt", SimpleFileOptions::default())?;
        zip.write_all(b"This is a readme file")?;

        zip.finish()?;
        Ok(zip_path)
    }

    #[tokio::test]
    async fn test_download_and_extract_archive() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let zip_path = create_test_zip_with_executables(temp_dir.path())?;

        // Create a mock HTTP server URL (we'll use file:// for testing)
        let file_url = format!("file://{}", zip_path.to_string_lossy());

        let downloader = Downloader::new();
        let base_dir = temp_dir.path().join("downloads");

        // This would normally download from HTTP, but we're using a local file
        // In a real test, you'd need to mock the HTTP client or use a test server
        let _result = downloader
            .download_and_extract_archive(&file_url, &base_dir, "test-app")
            .await;

        // For now, just test the archive extraction directly
        let extract_dir = base_dir.join("test-app").join("bin");
        fs::create_dir_all(&extract_dir)?;

        let executables = ArchiveExtractor::extract_archive(&zip_path, &extract_dir)?;

        assert_eq!(executables.len(), 3); // Should find 3 .exe files
        assert!(executables
            .iter()
            .any(|p| p.file_name().unwrap() == "tool1.exe"));
        assert!(executables
            .iter()
            .any(|p| p.file_name().unwrap() == "tool2.exe"));
        assert!(executables
            .iter()
            .any(|p| p.file_name().unwrap() == "tool3.exe"));

        Ok(())
    }

    #[test]
    fn test_archive_extractor_is_archive_url() {
        assert!(ArchiveExtractor::is_archive_url(
            "https://example.com/file.zip"
        ));
        assert!(!ArchiveExtractor::is_archive_url(
            "https://example.com/file.exe"
        ));
        assert!(!ArchiveExtractor::is_archive_url(
            "https://example.com/file.tar.gz"
        )); // Not supported yet
    }

    #[test]
    fn test_archive_extractor_is_archive() {
        assert!(ArchiveExtractor::is_archive(std::path::Path::new(
            "test.zip"
        )));
        assert!(!ArchiveExtractor::is_archive(std::path::Path::new(
            "test.exe"
        )));
        assert!(!ArchiveExtractor::is_archive(std::path::Path::new("test")));
    }

    #[test]
    fn test_generate_shim_name() {
        let path = std::path::Path::new("tool.exe");
        let existing = vec![];
        assert_eq!(
            ArchiveExtractor::generate_shim_name(path, &existing),
            "tool"
        );

        let existing = vec!["tool".to_string()];
        assert_eq!(
            ArchiveExtractor::generate_shim_name(path, &existing),
            "tool-1"
        );

        let existing = vec!["tool".to_string(), "tool-1".to_string()];
        assert_eq!(
            ArchiveExtractor::generate_shim_name(path, &existing),
            "tool-2"
        );
    }

    #[test]
    fn test_find_executables_in_dir() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;

        // Create some test files
        fs::File::create(temp_dir.path().join("tool1.exe"))?;
        fs::File::create(temp_dir.path().join("tool2.exe"))?;
        fs::File::create(temp_dir.path().join("readme.txt"))?;
        fs::File::create(temp_dir.path().join("config.json"))?;

        let executables = ArchiveExtractor::find_executables_in_dir(temp_dir.path())?;

        assert_eq!(executables.len(), 2);
        assert!(executables
            .iter()
            .any(|p| p.file_name().unwrap() == "tool1.exe"));
        assert!(executables
            .iter()
            .any(|p| p.file_name().unwrap() == "tool2.exe"));

        Ok(())
    }
}

#[cfg(test)]
mod config_tests {
    use crate::config::{ExtractedExecutable, ShimConfig, ShimCore, SourceType};
    use std::collections::HashMap;

    #[test]
    fn test_archive_config_serialization() {
        let config = ShimConfig {
            shim: ShimCore {
                name: "test-archive".to_string(),
                path: "/path/to/extracted/tool.exe".to_string(),
                args: vec![],
                cwd: None,
                download_url: Some("https://example.com/tools.zip".to_string()),
                source_type: SourceType::Archive,
                extracted_executables: vec![
                    ExtractedExecutable {
                        name: "tool1".to_string(),
                        path: "tool1.exe".to_string(),
                        full_path: "/path/to/extracted/tool1.exe".to_string(),
                        is_primary: true,
                    },
                    ExtractedExecutable {
                        name: "tool2".to_string(),
                        path: "tool2.exe".to_string(),
                        full_path: "/path/to/extracted/tool2.exe".to_string(),
                        is_primary: false,
                    },
                ],
            },
            args: Default::default(),
            env: HashMap::new(),
            metadata: Default::default(),
            auto_update: None,
        };

        // Test serialization
        let toml_str = toml::to_string(&config).expect("Failed to serialize config");
        assert!(toml_str.contains("source_type = \"archive\""));
        assert!(toml_str.contains("[[shim.extracted_executables]]"));
        assert!(toml_str.contains("is_primary = true"));

        // Test deserialization
        let deserialized: ShimConfig =
            toml::from_str(&toml_str).expect("Failed to deserialize config");
        assert_eq!(deserialized.shim.source_type, SourceType::Archive);
        assert_eq!(deserialized.shim.extracted_executables.len(), 2);
        assert!(deserialized.shim.extracted_executables[0].is_primary);
        assert!(!deserialized.shim.extracted_executables[1].is_primary);
    }

    #[test]
    fn test_url_config_serialization() {
        let config = ShimConfig {
            shim: ShimCore {
                name: "test-url".to_string(),
                path: "/path/to/downloaded/tool.exe".to_string(),
                args: vec![],
                cwd: None,
                download_url: Some("https://example.com/tool.exe".to_string()),
                source_type: SourceType::Url,
                extracted_executables: vec![],
            },
            args: Default::default(),
            env: HashMap::new(),
            metadata: Default::default(),
            auto_update: None,
        };

        let toml_str = toml::to_string(&config).expect("Failed to serialize config");
        assert!(toml_str.contains("source_type = \"url\""));
        assert!(!toml_str.contains("extracted_executables")); // Should be omitted when empty

        let deserialized: ShimConfig =
            toml::from_str(&toml_str).expect("Failed to deserialize config");
        assert_eq!(deserialized.shim.source_type, SourceType::Url);
        assert!(deserialized.shim.extracted_executables.is_empty());
    }

    #[test]
    fn test_file_config_backward_compatibility() {
        // Test that old configs without source_type still work
        let toml_str = r#"
[shim]
name = "test"
path = "/usr/bin/test"
args = []

[env]

[metadata]
"#;

        let config: ShimConfig =
            toml::from_str(toml_str).expect("Failed to deserialize old config");
        assert_eq!(config.shim.source_type, SourceType::File); // Should default to File
        assert!(config.shim.extracted_executables.is_empty());
        assert!(config.shim.download_url.is_none());
    }
}
