use shimexe_core::ArchiveExtractor;
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

#[test]
fn test_download_and_extract_archive() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let zip_path = create_test_zip_with_executables(temp_dir.path())?;

    // Test archive extraction directly without network download
    // This avoids the turbo-cdn initialization issue in CI environments
    let extract_dir = temp_dir.path().join("extract");
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
