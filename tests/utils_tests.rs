use shimexe_core::utils::*;
use shimexe_core::error::ShimError;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use tempfile::{NamedTempFile, TempDir};

#[test]
fn test_expand_env_vars_basic() {
    // Set up test environment variable
    env::set_var("SHIMEXE_TEST_VAR", "test_value");

    // Test basic expansion
    let result = expand_env_vars("${SHIMEXE_TEST_VAR}").unwrap();
    assert_eq!(result, "test_value");

    // Test expansion in middle of string
    let result = expand_env_vars("prefix_${SHIMEXE_TEST_VAR}_suffix").unwrap();
    assert_eq!(result, "prefix_test_value_suffix");

    // Clean up
    env::remove_var("SHIMEXE_TEST_VAR");
}

#[test]
fn test_expand_env_vars_with_default() {
    // Test with default value when variable doesn't exist
    let result = expand_env_vars("${NONEXISTENT_VAR:default_value}").unwrap();
    assert_eq!(result, "default_value");

    // Test with default value when variable exists
    env::set_var("SHIMEXE_TEST_VAR2", "actual_value");
    let result = expand_env_vars("${SHIMEXE_TEST_VAR2:default_value}").unwrap();
    assert_eq!(result, "actual_value");

    // Test empty default value
    let result = expand_env_vars("${NONEXISTENT_VAR:}").unwrap();
    assert_eq!(result, "");

    // Clean up
    env::remove_var("SHIMEXE_TEST_VAR2");
}

#[test]
fn test_expand_env_vars_multiple() {
    env::set_var("VAR1", "value1");
    env::set_var("VAR2", "value2");

    let result = expand_env_vars("${VAR1}_${VAR2}").unwrap();
    assert_eq!(result, "value1_value2");

    let result = expand_env_vars("${VAR1}_${NONEXISTENT:default}_${VAR2}").unwrap();
    assert_eq!(result, "value1_default_value2");

    env::remove_var("VAR1");
    env::remove_var("VAR2");
}

#[test]
fn test_expand_env_vars_errors() {
    // Test unclosed brace
    let result = expand_env_vars("${UNCLOSED");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unclosed"));

    // Test nonexistent variable without default
    let result = expand_env_vars("${DEFINITELY_NONEXISTENT_VAR}");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));
}

#[test]
fn test_expand_env_vars_complex_cases() {
    env::set_var("COMPLEX_VAR", "complex/path");

    // Test with special characters in default
    let result = expand_env_vars("${NONEXISTENT:/default/path/with:colons}").unwrap();
    assert_eq!(result, "/default/path/with:colons");

    // Test nested-like syntax (should not be nested)
    let result = expand_env_vars("${COMPLEX_VAR}/subdir").unwrap();
    assert_eq!(result, "complex/path/subdir");

    env::remove_var("COMPLEX_VAR");
}

#[test]
fn test_get_exe_extension() {
    let ext = get_exe_extension();
    if cfg!(windows) {
        assert_eq!(ext, ".exe");
    } else {
        assert_eq!(ext, "");
    }
}

#[test]
fn test_get_path_separator() {
    let sep = get_path_separator();
    if cfg!(windows) {
        assert_eq!(sep, "\\");
    } else {
        assert_eq!(sep, "/");
    }
}

#[test]
fn test_get_builtin_env_vars() {
    let vars = get_builtin_env_vars();

    // Test required built-in variables
    assert!(vars.contains_key("EXE_EXT"));
    assert!(vars.contains_key("PATH_SEP"));

    // Test platform-specific values
    if cfg!(windows) {
        assert_eq!(vars.get("EXE_EXT").unwrap(), ".exe");
        assert_eq!(vars.get("PATH_SEP").unwrap(), "\\");
    } else {
        assert_eq!(vars.get("EXE_EXT").unwrap(), "");
        assert_eq!(vars.get("PATH_SEP").unwrap(), "/");
    }

    // Test directory variables (these should exist on most systems)
    // Note: We don't assert their presence since they might not exist in all environments
    if vars.contains_key("HOME") {
        assert!(!vars.get("HOME").unwrap().is_empty());
    }
}

#[test]
fn test_merge_env_vars() {
    let mut custom_env = HashMap::new();
    custom_env.insert("CUSTOM_VAR".to_string(), "custom_value".to_string());
    custom_env.insert("EXE_EXT".to_string(), "overridden".to_string());

    let merged = merge_env_vars(&custom_env);

    // Test that custom variables are included
    assert_eq!(merged.get("CUSTOM_VAR").unwrap(), "custom_value");

    // Test that custom variables override built-in ones
    assert_eq!(merged.get("EXE_EXT").unwrap(), "overridden");

    // Test that built-in variables are still present (unless overridden)
    assert!(merged.contains_key("PATH_SEP"));

    // Test that system environment variables are included
    // Set a test environment variable to ensure it's included
    env::set_var("SHIMEXE_TEST_MERGE", "test_value");
    let merged_with_env = merge_env_vars(&custom_env);
    assert!(merged_with_env.contains_key("SHIMEXE_TEST_MERGE"));
    env::remove_var("SHIMEXE_TEST_MERGE");
}

#[test]
fn test_merge_env_vars_empty_custom() {
    let custom_env = HashMap::new();
    let merged = merge_env_vars(&custom_env);

    // Should still contain built-in variables
    assert!(merged.contains_key("EXE_EXT"));
    assert!(merged.contains_key("PATH_SEP"));
}

#[test]
fn test_merge_env_vars_precedence() {
    // Set a system environment variable
    env::set_var("SHIMEXE_PRECEDENCE_TEST", "system_value");

    let mut custom_env = HashMap::new();
    custom_env.insert(
        "SHIMEXE_PRECEDENCE_TEST".to_string(),
        "custom_value".to_string(),
    );

    let merged = merge_env_vars(&custom_env);

    // Custom should override system
    assert_eq!(
        merged.get("SHIMEXE_PRECEDENCE_TEST").unwrap(),
        "custom_value"
    );

    // Clean up
    env::remove_var("SHIMEXE_PRECEDENCE_TEST");
}

#[test]
fn test_expand_env_vars_with_builtin_vars() {
    // Test expansion using built-in variables with default values
    let result = expand_env_vars("test${EXE_EXT:.exe}").unwrap();
    if cfg!(windows) {
        assert_eq!(result, "test.exe");
    } else {
        assert_eq!(result, "test.exe"); // Uses default since EXE_EXT is not in env
    }

    let result = expand_env_vars("path${PATH_SEP:/}to${PATH_SEP:/}file").unwrap();
    assert_eq!(result, "path/to/file"); // Uses default values
}

#[test]
fn test_expand_env_vars_edge_cases() {
    // Test empty string
    let result = expand_env_vars("").unwrap();
    assert_eq!(result, "");

    // Test string without variables
    let result = expand_env_vars("no variables here").unwrap();
    assert_eq!(result, "no variables here");

    // Test only variable
    env::set_var("ONLY_VAR", "only_value");
    let result = expand_env_vars("${ONLY_VAR}").unwrap();
    assert_eq!(result, "only_value");
    env::remove_var("ONLY_VAR");

    // Test variable at start and end
    env::set_var("START_VAR", "start");
    env::set_var("END_VAR", "end");
    let result = expand_env_vars("${START_VAR}_middle_${END_VAR}").unwrap();
    assert_eq!(result, "start_middle_end");
    env::remove_var("START_VAR");
    env::remove_var("END_VAR");
}

#[test]
fn test_normalize_path_basic() {
    let path = "test/path/to/file";
    let normalized = normalize_path(path);

    #[cfg(windows)]
    assert_eq!(normalized, "test\\path\\to\\file");
    #[cfg(not(windows))]
    assert_eq!(normalized, "test/path/to/file");
}

#[test]
fn test_normalize_path_with_backslashes() {
    let path = "test\\path\\to\\file";
    let normalized = normalize_path(path);

    #[cfg(windows)]
    assert_eq!(normalized, "test\\path\\to\\file");
    #[cfg(not(windows))]
    assert_eq!(normalized, "test/path/to/file");
}

#[test]
fn test_normalize_path_mixed_separators() {
    let path = "test/path\\to/file";
    let normalized = normalize_path(path);

    #[cfg(windows)]
    assert_eq!(normalized, "test\\path\\to\\file");
    #[cfg(not(windows))]
    assert_eq!(normalized, "test/path/to/file");
}

#[test]
fn test_normalize_path_empty() {
    let normalized = normalize_path("");
    assert_eq!(normalized, "");
}

#[test]
fn test_validate_shim_name_valid() {
    let valid_names = vec![
        "test",
        "test-shim",
        "test_shim",
        "test123",
        "Test",
        "TEST",
        "a",
        "my-awesome-tool",
        "tool_v2",
    ];

    for name in valid_names {
        assert!(validate_shim_name(name).is_ok(), "Name '{}' should be valid", name);
    }
}

#[test]
fn test_validate_shim_name_invalid() {
    let invalid_names = vec![
        "",           // Empty
        " ",          // Whitespace only
        "test shim",  // Contains space
        "test/shim",  // Contains slash
        "test\\shim", // Contains backslash
        "test:shim",  // Contains colon
        "test*shim",  // Contains asterisk
        "test?shim",  // Contains question mark
        "test\"shim", // Contains quote
        "test<shim",  // Contains less than
        "test>shim",  // Contains greater than
        "test|shim",  // Contains pipe
        ".test",      // Starts with dot
        "test.",      // Ends with dot
        "CON",        // Reserved Windows name
        "PRN",        // Reserved Windows name
        "AUX",        // Reserved Windows name
        "NUL",        // Reserved Windows name
        "COM1",       // Reserved Windows name
        "LPT1",       // Reserved Windows name
    ];

    for name in invalid_names {
        assert!(validate_shim_name(name).is_err(), "Name '{}' should be invalid", name);
    }
}

#[test]
fn test_get_platform() {
    let platform = get_platform();

    #[cfg(target_os = "windows")]
    assert_eq!(platform, "windows");

    #[cfg(target_os = "macos")]
    assert_eq!(platform, "macos");

    #[cfg(target_os = "linux")]
    assert_eq!(platform, "linux");

    // Should always return a non-empty string
    assert!(!platform.is_empty());
}

#[test]
fn test_get_architecture() {
    let arch = get_architecture();

    #[cfg(target_arch = "x86_64")]
    assert_eq!(arch, "x86_64");

    #[cfg(target_arch = "x86")]
    assert_eq!(arch, "x86");

    #[cfg(target_arch = "aarch64")]
    assert_eq!(arch, "aarch64");

    // Should always return a non-empty string
    assert!(!arch.is_empty());
}

#[test]
fn test_resolve_executable_path_absolute() {
    let temp_dir = TempDir::new().unwrap();
    let exe_path = temp_dir.path().join("test.exe");
    fs::write(&exe_path, "test").unwrap();

    let resolved = resolve_executable_path(&exe_path.to_string_lossy()).unwrap();
    assert_eq!(resolved, exe_path);
}

#[test]
fn test_resolve_executable_path_in_path() {
    // Test with a command that should be in PATH
    #[cfg(windows)]
    let result = resolve_executable_path("cmd");
    #[cfg(not(windows))]
    let result = resolve_executable_path("sh");

    assert!(result.is_ok());
    let resolved = result.unwrap();
    assert!(resolved.exists());
}

#[test]
fn test_resolve_executable_path_not_found() {
    let result = resolve_executable_path("nonexistent_executable_12345");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ShimError::ExecutableNotFound(_)));
}

#[test]
fn test_extract_archive_unsupported() {
    let temp_dir = TempDir::new().unwrap();
    let archive_path = temp_dir.path().join("test.unsupported");
    let extract_dir = temp_dir.path().join("extract");

    fs::write(&archive_path, "fake archive").unwrap();

    let result = extract_archive(&archive_path, &extract_dir);
    assert!(result.is_err());
    // Should return an error for unsupported archive types
}

#[test]
fn test_extract_archive_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let archive_path = temp_dir.path().join("nonexistent.zip");
    let extract_dir = temp_dir.path().join("extract");

    let result = extract_archive(&archive_path, &extract_dir);
    assert!(result.is_err());
}
