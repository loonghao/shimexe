// Test ShimManager list/get shim behavior using temp dirs (no network)

use shimexe_core::config::{ShimConfig, ShimCore, SourceType};
use shimexe_core::manager::ShimManager;

#[test]
fn test_manager_list_and_get() {
    let tmp = tempfile::tempdir().unwrap();
    let mgr = ShimManager::new(tmp.path().to_path_buf()).unwrap();

    // Create a minimal shim config file manually
    let cfg = ShimConfig {
        shim: ShimCore {
            name: "echoer".into(),
            path: "/usr/bin/echo".into(),
            args: vec!["hello".into()],
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

    // Save config the same way create_shim would
    let cfg_path = tmp.path().join("echoer.shim.toml");
    cfg.to_file(&cfg_path).unwrap();

    // list_shims should include it (shim binary may not exist; is_valid will be false)
    let list = mgr.list_shims().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].name, "echoer");

    // get_shim should return metadata and validity state
    let got = mgr.get_shim("echoer").unwrap().unwrap();
    assert_eq!(got.name, "echoer");
    assert_eq!(got.is_valid, false); // shim exe not created in test
}
