// 2025 tech: Rust stable. Focus on pure functions for reliable coverage gains.

use shimexe_core::utils::{
    expand_env_vars, get_builtin_env_vars, get_exe_extension, get_path_separator,
};
use std::env;

#[test]
fn test_expand_env_vars_variants() {
    env::set_var("COVERAGE_TEST_VAR", "ok");

    // ${VAR}
    assert_eq!(expand_env_vars("${COVERAGE_TEST_VAR}").unwrap(), "ok");

    // ${VAR:default}
    assert_eq!(expand_env_vars("${NOT_SET:fallback}").unwrap(), "fallback");

    // $VAR
    assert_eq!(expand_env_vars("$COVERAGE_TEST_VAR").unwrap(), "ok");

    env::remove_var("COVERAGE_TEST_VAR");
}

#[test]
fn test_platform_helpers() {
    let ext = get_exe_extension();
    let sep = get_path_separator();
    if cfg!(windows) {
        assert_eq!(ext, ".exe");
        assert_eq!(sep, "\\");
    } else {
        assert_eq!(ext, "");
        assert_eq!(sep, "/");
    }

    let builtins = get_builtin_env_vars();
    assert!(builtins.contains_key("EXE_EXT"));
    assert!(builtins.contains_key("PATH_SEP"));
}
