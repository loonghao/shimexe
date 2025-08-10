// 2025 tech: Rust stable. Use tempfile for isolated file I/O. No network calls.
// Static type checking via rustc; clippy runs in CI. AI code review: Target hot paths.

use shimexe_core::config::{ShimConfig, ShimCore, SourceType};
use tempfile::NamedTempFile;

#[test]
fn test_config_from_and_to_file_roundtrip() {
    let config = ShimConfig {
        shim: ShimCore {
            name: "roundtrip".to_string(),
            path: "/usr/bin/echo".to_string(),
            args: vec!["hello".to_string()],
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

    let tmp = NamedTempFile::new().unwrap();
    config.to_file(tmp.path()).unwrap();

    let loaded = ShimConfig::from_file(tmp.path()).unwrap();
    assert_eq!(loaded.shim.name, "roundtrip");
    assert_eq!(loaded.shim.path, "/usr/bin/echo");
    assert_eq!(loaded.shim.args, vec!["hello"]);
}

#[test]
fn test_get_executable_path_archive_primary() {
    // Ensure get_executable_path returns error when no extracted executables exist for archives
    let cfg = ShimConfig {
        shim: ShimCore {
            name: "arch".to_string(),
            path: "/opt/tool".to_string(),
            args: vec![],
            cwd: None,
            download_url: None,
            source_type: SourceType::Archive,
            extracted_executables: vec![],
        },
        args: Default::default(),
        env: Default::default(),
        metadata: Default::default(),
        auto_update: None,
    };

    assert!(cfg.get_executable_path().is_err());
}

