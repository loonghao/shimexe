// ShimManager::validate_shim negative path using temp dir

use shimexe_core::manager::ShimManager;

#[test]
fn test_validate_shim_negative() {
    let tmp = tempfile::tempdir().unwrap();
    let mgr = ShimManager::new(tmp.path().to_path_buf()).unwrap();

    // No shim created -> validate_shim should return false without panic
    assert!(!mgr.validate_shim("nope").unwrap());
}
