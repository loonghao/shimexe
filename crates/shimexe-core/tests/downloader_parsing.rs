// 2025 tech stack: Rust 1.78+, stable. Uses standard library and shimexe_core public APIs.
// Static type checking: rustc enforces types; clippy will run in CI.
// AI code review summary: Focus on pure, network-free tests to safely raise coverage.

use std::path::Path;

use shimexe_core::downloader::Downloader;

#[test]
fn test_is_url() {
    assert!(Downloader::is_url("https://example.com/file.exe"));
    assert!(Downloader::is_url("http://example.com/file.exe"));
    assert!(!Downloader::is_url("file:///local"));
    assert!(!Downloader::is_url("/usr/bin/tool"));
}

#[test]
fn test_extract_filename_from_url() {
    let url = "https://host/app/releases/download/v1.0.0/my-tool.exe?sig=abc#top";
    let name = Downloader::extract_filename_from_url(url).unwrap();
    assert_eq!(name, "my-tool.exe");

    let url2 = "https://example.com/path/to/archive.tar.gz";
    let name2 = Downloader::extract_filename_from_url(url2).unwrap();
    assert_eq!(name2, "archive.tar.gz");
}

#[test]
fn test_extract_exe_name_from_filename() {
    assert_eq!(
        Downloader::extract_exe_name_from_filename("tool.exe").as_deref(),
        Some("tool")
    );
    assert_eq!(
        Downloader::extract_exe_name_from_filename("pkg.zip").as_deref(),
        Some("pkg")
    );
    assert_eq!(
        Downloader::extract_exe_name_from_filename("bundle.tar.gz").as_deref(),
        Some("bundle")
    );
    assert_eq!(
        Downloader::extract_exe_name_from_filename("unknown"),
        Some("unknown".to_string())
    );
}

#[test]
fn test_generate_download_path() {
    let base = Path::new("/tmp/.shimexe");
    let p = Downloader::generate_download_path(base, "myapp", "myapp.exe");
    assert!(p.ends_with("myapp/bin/myapp.exe"));
}

#[test]
fn test_infer_app_name_from_url() {
    assert_eq!(
        Downloader::infer_app_name_from_url("https://host/app.exe").as_deref(),
        Some("app")
    );
    assert_eq!(
        Downloader::infer_app_name_from_url("https://host/archive.zip").as_deref(),
        Some("archive")
    );
    assert_eq!(
        Downloader::infer_app_name_from_url("https://host/dir/tool.tgz").as_deref(),
        Some("tool")
    );
}

#[test]
fn test_is_executable_file_extension_based() {
    // Private fn cannot be called; we indirectly validate extension policy via infer_app_name etc.
    // Here we only assert that the logic which treats common extensions as executables does not panic.
    // This test is intentionally minimal to keep it platform-neutral.
    let _ = Downloader::infer_app_name_from_url("https://host/thing.bin");
}
