// URL source: get_executable_path positive when file exists under ~/.shimexe/<name>/bin/<filename>

use shimexe_core::config::{ShimConfig, ShimCore, SourceType};

#[cfg(unix)]
#[test]
fn test_get_executable_path_url_positive_with_home() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path();

    // Make dirs::home_dir point to tmp
    std::env::set_var("HOME", home);
    std::env::set_var("USERPROFILE", home);

    let name = "urlok";
    let filename = "app.exe";
    let exe_path = home.join(".shimexe").join(name).join("bin").join(filename);
    std::fs::create_dir_all(exe_path.parent().unwrap()).unwrap();
    std::fs::write(&exe_path, b"ok").unwrap();

    let cfg = ShimConfig {
        shim: ShimCore {
            name: name.into(),
            path: "ignored".into(),
            args: vec![],
            cwd: None,
            download_url: Some(format!("https://host/{filename}")),
            source_type: SourceType::Url,
            extracted_executables: vec![],
        },
        args: Default::default(),
        env: Default::default(),
        metadata: Default::default(),
        auto_update: None,
    };

    let p = cfg.get_executable_path().unwrap();
    assert_eq!(p, exe_path);
}
