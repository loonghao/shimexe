// Test get_executable_path positive path for Archive with primary executable

use shimexe_core::config::{ExtractedExecutable, ShimConfig, ShimCore, SourceType};

#[test]
fn test_get_executable_path_archive_primary_ok() {
    // Use a dummy path that likely doesn't exist, but mark it as primary and ensure code
    // checks file existence. To keep test deterministic, we create a temp file to satisfy the check.
    let tmpdir = tempfile::tempdir().unwrap();
    let exe_file = tmpdir.path().join("app/bin/tool.exe");
    std::fs::create_dir_all(exe_file.parent().unwrap()).unwrap();
    std::fs::write(&exe_file, b"dummy").unwrap();

    let cfg = ShimConfig {
        shim: ShimCore {
            name: "arch-ok".to_string(),
            path: "ignored-when-archive".to_string(),
            args: vec![],
            cwd: None,
            download_url: None,
            source_type: SourceType::Archive,
            extracted_executables: vec![ExtractedExecutable {
                name: "tool".into(),
                path: "app/bin".into(),
                full_path: exe_file.to_string_lossy().to_string(),
                is_primary: true,
            }],
        },
        args: Default::default(),
        env: Default::default(),
        metadata: Default::default(),
        auto_update: None,
    };

    let got = cfg.get_executable_path().unwrap();
    assert_eq!(got, exe_file);
}
