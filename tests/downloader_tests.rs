use shimexe_core::Downloader;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

#[test]
fn test_is_url() {
    assert!(Downloader::is_url("https://example.com/file.zip"));
    assert!(Downloader::is_url("http://example.com/file.zip"));
    assert!(!Downloader::is_url("/local/path/file.zip"));
    assert!(!Downloader::is_url("file.zip"));
}

#[test]
fn test_extract_filename_from_url() {
    assert_eq!(
        Downloader::extract_filename_from_url("https://example.com/file.zip"),
        Some("file.zip".to_string())
    );
    assert_eq!(
        Downloader::extract_filename_from_url("https://example.com/path/to/file.exe"),
        Some("file.exe".to_string())
    );
    assert_eq!(
        Downloader::extract_filename_from_url("https://example.com/file.zip?param=value"),
        Some("file.zip".to_string())
    );
    assert_eq!(
        Downloader::extract_filename_from_url("https://example.com/file.zip#fragment"),
        Some("file.zip".to_string())
    );
}

#[test]
fn test_extract_exe_name_from_filename() {
    assert_eq!(
        Downloader::extract_exe_name_from_filename("program.exe"),
        Some("program".to_string())
    );
    assert_eq!(
        Downloader::extract_exe_name_from_filename("archive.zip"),
        Some("archive".to_string())
    );
    assert_eq!(
        Downloader::extract_exe_name_from_filename("package.tar.gz"),
        Some("package".to_string())
    );
    assert_eq!(
        Downloader::extract_exe_name_from_filename("file.tgz"),
        Some("file".to_string())
    );
    assert_eq!(
        Downloader::extract_exe_name_from_filename("binary"),
        Some("binary".to_string())
    );
}

#[test]
fn test_generate_download_path() {
    let base_dir = Path::new("/tmp");
    let app_name = "myapp";
    let filename = "myapp.exe";

    let expected = PathBuf::from("/tmp/myapp/bin/myapp.exe");
    let actual = Downloader::generate_download_path(base_dir, app_name, filename);

    assert_eq!(actual, expected);
}

#[test]
fn test_file_exists() {
    let temp_dir = tempdir().unwrap();
    let file_path = temp_dir.path().join("test_file.txt");

    // File doesn't exist yet
    assert!(!Downloader::file_exists(&file_path));

    // Create the file
    std::fs::write(&file_path, "test content").unwrap();

    // File should exist now
    assert!(Downloader::file_exists(&file_path));
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
    assert_eq!(
        Downloader::infer_app_name_from_url("https://example.com/my-tool.tar.gz"),
        Some("my-tool".to_string())
    );
    assert_eq!(
        Downloader::infer_app_name_from_url("https://example.com/mytool"),
        Some("mytool".to_string())
    );
    assert_eq!(
        Downloader::infer_app_name_from_url("https://example.com/"),
        None
    );
}
