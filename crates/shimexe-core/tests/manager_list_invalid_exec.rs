// Manager list and validate for an invalid (non-executable) file path

use shimexe_core::config::{ShimConfig, ShimCore, SourceType};
use shimexe_core::manager::ShimManager;

#[test]
fn test_list_and_validate_invalid_exec() {
    let tmp = tempfile::tempdir().unwrap();
    let mgr = ShimManager::new(tmp.path().to_path_buf()).unwrap();

    // Write a minimal config file manually
    let cfg = ShimConfig {
        shim: ShimCore {
            name: "badexec".into(),
            path: tmp
                .path()
                .join("not_executable")
                .to_string_lossy()
                .to_string(),
            args: vec![],
            cwd: None,
            download_url: None,
            source_type: SourceType::File,
            extracted_executables: vec![],
        },
        args: Default::default(),
        env: Default::default(),
        metadata: Default::default(),
        auto_update: None,
    };

    cfg.to_file(&tmp.path().join("badexec.shim.toml")).unwrap();

    // list_shims shows it
    let items = mgr.list_shims().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "badexec");

    // validate_shim should be false (runner.validate fails)
    assert!(!mgr.validate_shim("badexec").unwrap());
}
