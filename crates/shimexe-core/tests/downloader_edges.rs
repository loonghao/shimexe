// Edge cases for Downloader parsing helpers

use shimexe_core::downloader::Downloader;

#[test]
fn test_extract_filename_from_url_edges() {
    // With our current implementation, "https://example.com" yields filename "example.com"
    assert_eq!(
        Downloader::extract_filename_from_url("https://example.com"),
        Some("example.com".to_string())
    );
    assert_eq!(
        Downloader::extract_filename_from_url("https://example.com/"),
        None
    );
    assert_eq!(
        Downloader::extract_filename_from_url("https://example.com/path/"),
        None
    );
}

#[test]
fn test_infer_app_name_from_url_edges() {
    // Given the filename rule above, infer_app_name_from_url will also return Some for bare host
    assert_eq!(
        Downloader::infer_app_name_from_url("https://example.com"),
        Some("example.com".to_string())
    );
    assert_eq!(
        Downloader::infer_app_name_from_url("https://example.com/"),
        None
    );
    assert_eq!(
        Downloader::infer_app_name_from_url("https://example.com/path/"),
        None
    );
}
