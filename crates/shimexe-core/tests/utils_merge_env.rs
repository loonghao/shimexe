// Tests for merge_env_vars precedence and content

use std::collections::HashMap;
use std::env;

use shimexe_core::utils::{get_builtin_env_vars, merge_env_vars};

#[test]
fn test_merge_env_vars_overrides() {
    // Prepare OS env
    env::set_var("MERGE_TEST_OS", "os_value");

    // Custom env that overrides OS and builtins
    let mut custom = HashMap::new();
    custom.insert("MERGE_TEST_OS".to_string(), "custom_value".to_string());
    custom.insert("CUSTOM_ONLY".to_string(), "only_value".to_string());
    custom.insert("EXE_EXT".to_string(), "override_ext".to_string());

    let merged = merge_env_vars(&custom);

    // Custom overrides OS value
    assert_eq!(
        merged.get("MERGE_TEST_OS").map(String::as_str),
        Some("custom_value")
    );
    // Custom-only key present
    assert_eq!(
        merged.get("CUSTOM_ONLY").map(String::as_str),
        Some("only_value")
    );
    // Builtin exists and can be overridden by custom
    assert_eq!(
        merged.get("EXE_EXT").map(String::as_str),
        Some("override_ext")
    );

    // Builtins should generally be present (PATH_SEP at least)
    let builtins = get_builtin_env_vars();
    assert!(builtins.contains_key("PATH_SEP"));
}
