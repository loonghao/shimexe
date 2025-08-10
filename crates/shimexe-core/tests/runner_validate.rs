// Test ShimRunner::validate using a temp file as executable

use shimexe_core::config::{ShimConfig, ShimCore, SourceType};
use shimexe_core::runner::ShimRunner;

#[test]
fn test_runner_validate_ok_and_cache() {
    let tmpdir = tempfile::tempdir().unwrap();
    let exe = tmpdir.path().join("tool.exe");
    std::fs::write(&exe, b"ok").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&exe).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&exe, perms).unwrap();
    }

    let cfg = ShimConfig {
        shim: ShimCore {
            name: "runnertool".into(),
            path: exe.to_string_lossy().to_string(),
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

    let runner = ShimRunner::from_config(cfg).unwrap();
    runner.validate().unwrap(); // initial validation
    runner.validate().unwrap(); // cached path (should be quick and ok)
}
