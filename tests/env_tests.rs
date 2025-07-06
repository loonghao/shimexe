use shimexe_core::config::{ShimConfig, ShimCore, SourceType};
use shimexe_core::runner::ShimRunner;
use shimexe_core::utils::{expand_env_vars, get_builtin_env_vars, merge_env_vars};
use std::collections::HashMap;
use std::env;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_env_vars_expansion() {
    // Set test environment variable
    env::set_var("TEST_EXPAND_VAR", "expanded_value");

    let result = expand_env_vars("${TEST_EXPAND_VAR}").unwrap();
    assert_eq!(result, "expanded_value");

    // Test with default value
    let result = expand_env_vars("${NONEXISTENT_VAR:default_value}").unwrap();
    assert_eq!(result, "default_value");

    // Test simple $VAR format
    let result = expand_env_vars("$TEST_EXPAND_VAR").unwrap();
    assert_eq!(result, "expanded_value");

    // Test mixed content
    let result = expand_env_vars("prefix_${TEST_EXPAND_VAR}_suffix").unwrap();
    assert_eq!(result, "prefix_expanded_value_suffix");

    // Clean up
    env::remove_var("TEST_EXPAND_VAR");
}

#[test]
fn test_env_vars_expansion_error() {
    // Test error when variable doesn't exist and no default
    let result = expand_env_vars("${NONEXISTENT_VAR_NO_DEFAULT}");
    assert!(result.is_err());

    // Test unclosed brace
    let result = expand_env_vars("${UNCLOSED");
    assert!(result.is_err());
}

#[test]
fn test_builtin_env_vars() {
    let builtin_vars = get_builtin_env_vars();

    // Check that built-in variables exist
    assert!(builtin_vars.contains_key("EXE_EXT"));
    assert!(builtin_vars.contains_key("PATH_SEP"));

    // Check platform-specific values
    #[cfg(windows)]
    {
        assert_eq!(builtin_vars.get("EXE_EXT"), Some(&".exe".to_string()));
        assert_eq!(builtin_vars.get("PATH_SEP"), Some(&"\\".to_string()));
    }

    #[cfg(not(windows))]
    {
        assert_eq!(builtin_vars.get("EXE_EXT"), Some(&"".to_string()));
        assert_eq!(builtin_vars.get("PATH_SEP"), Some(&"/".to_string()));
    }

    // Check that common directories are included
    assert!(builtin_vars.contains_key("HOME") || builtin_vars.contains_key("CONFIG_DIR"));
}

#[test]
fn test_merge_env_vars() {
    let mut custom_env = HashMap::new();
    custom_env.insert("CUSTOM_VAR".to_string(), "custom_value".to_string());
    custom_env.insert("PATH_SEP".to_string(), "overridden".to_string()); // Override built-in

    let merged = merge_env_vars(&custom_env);

    // Check that custom variables are included
    assert_eq!(merged.get("CUSTOM_VAR"), Some(&"custom_value".to_string()));

    // Check that built-in variables are overridden
    assert_eq!(merged.get("PATH_SEP"), Some(&"overridden".to_string()));

    // Check that other built-in variables still exist
    assert!(merged.contains_key("EXE_EXT"));
}

#[test]
fn test_shim_config_env_expansion() {
    // Set test environment variables
    env::set_var("TEST_CONFIG_VAR", "config_value");
    env::set_var("TEST_PATH_VAR", "/test/path");

    // Set EXE_EXT environment variable for the test
    #[cfg(windows)]
    env::set_var("EXE_EXT", ".exe");
    #[cfg(not(windows))]
    env::set_var("EXE_EXT", "");

    let mut config = ShimConfig {
        shim: ShimCore {
            name: "test".to_string(),
            path: "${TEST_PATH_VAR}/bin/test${EXE_EXT}".to_string(),
            args: vec!["${TEST_CONFIG_VAR}".to_string()],
            cwd: Some("${TEST_PATH_VAR}".to_string()),
            download_url: None,
            source_type: SourceType::File,
            extracted_executables: vec![],
        },
        args: Default::default(),
        env: {
            let mut env = HashMap::new();
            env.insert(
                "EXPANDED_VAR".to_string(),
                "${TEST_CONFIG_VAR}_expanded".to_string(),
            );
            env.insert("LITERAL_VAR".to_string(), "literal_value".to_string());
            env
        },
        metadata: Default::default(),
        auto_update: None,
    };

    // Expand environment variables
    config.expand_env_vars().unwrap();

    // Check that path was expanded
    #[cfg(windows)]
    assert_eq!(config.shim.path, "/test/path/bin/test.exe");
    #[cfg(not(windows))]
    assert_eq!(config.shim.path, "/test/path/bin/test");

    // Check that args were expanded
    assert_eq!(config.shim.args[0], "config_value");

    // Check that cwd was expanded
    assert_eq!(config.shim.cwd, Some("/test/path".to_string()));

    // Check that env vars were expanded
    assert_eq!(
        config.env.get("EXPANDED_VAR"),
        Some(&"config_value_expanded".to_string())
    );
    assert_eq!(
        config.env.get("LITERAL_VAR"),
        Some(&"literal_value".to_string())
    );

    // Clean up
    env::remove_var("TEST_CONFIG_VAR");
    env::remove_var("TEST_PATH_VAR");
    env::remove_var("EXE_EXT");
}

#[test]
fn test_shim_config_from_file_with_env() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(
        temp_file,
        r#"
[shim]
name = "test-env"
path = "echo"
args = ["hello", "world"]

[env]
TEST_VAR = "test_value"
PATH_OVERRIDE = "/custom/path"
DEBUG = "true"
CONFIG_FILE = "/etc/myapp.conf"
        "#
    )
    .unwrap();

    let config = ShimConfig::from_file(temp_file.path()).unwrap();

    // Check that environment variables were loaded correctly
    assert_eq!(config.env.len(), 4);
    assert_eq!(config.env.get("TEST_VAR"), Some(&"test_value".to_string()));
    assert_eq!(
        config.env.get("PATH_OVERRIDE"),
        Some(&"/custom/path".to_string())
    );
    assert_eq!(config.env.get("DEBUG"), Some(&"true".to_string()));
    assert_eq!(
        config.env.get("CONFIG_FILE"),
        Some(&"/etc/myapp.conf".to_string())
    );
}

#[test]
fn test_shim_runner_env_vars() {
    // Create a test config with environment variables
    let config = ShimConfig {
        shim: ShimCore {
            name: "test-runner-env".to_string(),
            path: "echo".to_string(), // Use echo command which should be available
            args: vec!["test".to_string()],
            cwd: None,
            download_url: None,
            source_type: SourceType::File,
            extracted_executables: vec![],
        },
        args: Default::default(),
        env: {
            let mut env = HashMap::new();
            env.insert("RUNNER_TEST_VAR".to_string(), "runner_value".to_string());
            env.insert("RUNNER_DEBUG".to_string(), "1".to_string());
            env
        },
        metadata: Default::default(),
        auto_update: None,
    };

    // Create runner from config
    let runner = ShimRunner::from_config(config).unwrap();

    // Verify that the config has the expected environment variables
    assert_eq!(runner.config().env.len(), 2);
    assert_eq!(
        runner.config().env.get("RUNNER_TEST_VAR"),
        Some(&"runner_value".to_string())
    );
    assert_eq!(
        runner.config().env.get("RUNNER_DEBUG"),
        Some(&"1".to_string())
    );
}

#[test]
fn test_empty_env_config() {
    let config = ShimConfig {
        shim: ShimCore {
            name: "test-empty-env".to_string(),
            path: "echo".to_string(),
            args: vec!["test".to_string()],
            cwd: None,
            download_url: None,
            source_type: SourceType::File,
            extracted_executables: vec![],
        },
        args: Default::default(),
        env: HashMap::new(), // Empty environment
        metadata: Default::default(),
        auto_update: None,
    };

    let runner = ShimRunner::from_config(config).unwrap();

    // Even with empty env config, runner should work
    assert_eq!(runner.config().env.len(), 0);
}

#[test]
fn test_env_vars_with_special_characters() {
    let mut config = ShimConfig {
        shim: ShimCore {
            name: "test-special".to_string(),
            path: "echo".to_string(),
            args: vec![],
            cwd: None,
            download_url: None,
            source_type: SourceType::File,
            extracted_executables: vec![],
        },
        args: Default::default(),
        env: {
            let mut env = HashMap::new();
            env.insert(
                "SPECIAL_CHARS".to_string(),
                "value with spaces and symbols!@#$%".to_string(),
            );
            env.insert("UNICODE_VAR".to_string(), "测试中文字符".to_string());
            env.insert(
                "QUOTES_VAR".to_string(),
                r#"value with "quotes" and 'apostrophes'"#.to_string(),
            );
            env
        },
        metadata: Default::default(),
        auto_update: None,
    };

    // Expand environment variables
    config.expand_env_vars().unwrap();

    // Check that special characters are preserved
    assert_eq!(
        config.env.get("SPECIAL_CHARS"),
        Some(&"value with spaces and symbols!@#$%".to_string())
    );
    assert_eq!(
        config.env.get("UNICODE_VAR"),
        Some(&"测试中文字符".to_string())
    );
    assert_eq!(
        config.env.get("QUOTES_VAR"),
        Some(&r#"value with "quotes" and 'apostrophes'"#.to_string())
    );
}

#[test]
fn test_env_vars_precedence() {
    // Set a system environment variable
    env::set_var("PRECEDENCE_TEST", "system_value");

    let custom_env = {
        let mut env = HashMap::new();
        env.insert("PRECEDENCE_TEST".to_string(), "custom_value".to_string());
        env.insert("CUSTOM_ONLY".to_string(), "custom_only_value".to_string());
        env
    };

    let merged = merge_env_vars(&custom_env);

    // Custom environment variables should override system ones
    assert_eq!(
        merged.get("PRECEDENCE_TEST"),
        Some(&"custom_value".to_string())
    );
    assert_eq!(
        merged.get("CUSTOM_ONLY"),
        Some(&"custom_only_value".to_string())
    );

    // Clean up
    env::remove_var("PRECEDENCE_TEST");
}
