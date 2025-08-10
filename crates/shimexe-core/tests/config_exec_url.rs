// Tests for get_executable_path URL branches without network

use shimexe_core::config::{ShimConfig, ShimCore, SourceType};

#[test]
fn test_get_executable_path_url_no_file_yet() {
    let cfg = ShimConfig {
        shim: ShimCore {
            name: "urlapp".into(),
            path: "https://host/app.exe".into(), // legacy path as URL
            args: vec![],
            cwd: None,
            download_url: None,
            source_type: SourceType::Url,
            extracted_executables: vec![],
        },
        args: Default::default(),
        env: Default::default(),
        metadata: Default::default(),
        auto_update: None,
    };

    let err = cfg.get_executable_path().unwrap_err();
    let msg = format!("{}", err);
    assert!(msg.contains("Executable not found") || msg.contains("URL"));
}

#[test]
fn test_get_executable_path_legacy_url_path() {
    let cfg = ShimConfig {
        shim: ShimCore {
            name: "urlapp2".into(),
            path: "https://host/tool.exe".into(),
            args: vec![],
            cwd: None,
            download_url: None,
            source_type: SourceType::Url,
            extracted_executables: vec![],
        },
        args: Default::default(),
        env: Default::default(),
        metadata: Default::default(),
        auto_update: None,
    };

    let err = cfg.get_executable_path().unwrap_err();
    let msg = format!("{}", err);
    assert!(msg.contains("Executable not found") || msg.contains("URL"));
}

