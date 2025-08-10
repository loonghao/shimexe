// More cases for extract_exe_name_from_filename

use shimexe_core::downloader::Downloader;

#[test]
fn test_extract_exe_name_more() {
    // Current implementation strips only exe/zip/tar.gz/tgz
    assert_eq!(
        Downloader::extract_exe_name_from_filename("tool.bin").as_deref(),
        Some("tool.bin")
    );
    assert_eq!(
        Downloader::extract_exe_name_from_filename("run.app").as_deref(),
        Some("run.app")
    );
    assert_eq!(
        Downloader::extract_exe_name_from_filename("noext").as_deref(),
        Some("noext")
    );
}
