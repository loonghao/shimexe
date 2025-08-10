// Archive mode: no primary, fall back to first

use shimexe_core::config::{ExtractedExecutable, ShimConfig, ShimCore, SourceType};

#[test]
fn test_archive_first_executable_when_no_primary() {
    let tmp = tempfile::tempdir().unwrap();
    let exe1 = tmp.path().join("bin/a.exe");
    let exe2 = tmp.path().join("bin/b.exe");
    std::fs::create_dir_all(exe1.parent().unwrap()).unwrap();
    std::fs::write(&exe1, b"a").unwrap();
    std::fs::write(&exe2, b"b").unwrap();

    let cfg = ShimConfig {
        shim: ShimCore {
            name: "arch-first".into(),
            path: "ignored".into(),
            args: vec![],
            cwd: None,
            download_url: None,
            source_type: SourceType::Archive,
            extracted_executables: vec![
                ExtractedExecutable { name: "a".into(), path: "bin".into(), full_path: exe1.to_string_lossy().to_string(), is_primary: false },
                ExtractedExecutable { name: "b".into(), path: "bin".into(), full_path: exe2.to_string_lossy().to_string(), is_primary: false },
            ],
        },
        args: Default::default(),
        env: Default::default(),
        metadata: Default::default(),
        auto_update: None,
    };

    let p = cfg.get_executable_path().unwrap();
    assert_eq!(p, exe1); // first element chosen
}

