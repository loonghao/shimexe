// Platform function assertions (platform, arch, exe_ext)

use shimexe_core::template::TemplateEngine;

#[test]
fn test_platform_arch_exe_ext() {
    let mut engine = TemplateEngine::new(vec![]);

    let platform = engine.render_template("{{platform()}} ").unwrap().trim().to_string();
    assert!(matches!(platform.as_str(), "windows" | "linux" | "macos" | "unknown"));

    let arch = engine.render_template("{{arch()}} ").unwrap().trim().to_string();
    assert!(matches!(arch.as_str(), "x86_64" | "aarch64" | "unknown"));

    let exe_ext = engine.render_template("{{exe_ext()}} ").unwrap().trim().to_string();
    if cfg!(windows) { assert_eq!(exe_ext, ".exe"); } else { assert_eq!(exe_ext, ""); }
}

