use crate::error::{Result, ShimError};
use std::collections::HashMap;
use std::env;

/// Expand environment variables in a string with default value support
///
/// Supports formats:
/// - `${VAR}` - expand VAR, error if not found
/// - `${VAR:default}` - expand VAR, use default if not found
/// - `$VAR` - simple expansion
pub fn expand_env_vars(input: &str) -> Result<String> {
    let mut result = input.to_string();

    // Handle ${VAR:default} format
    while let Some(start) = result.find("${") {
        let end = result[start..]
            .find('}')
            .ok_or_else(|| ShimError::EnvExpansion("Unclosed ${".to_string()))?;
        let end = start + end;

        let var_expr = &result[start + 2..end];
        let (var_name, default_value) = if let Some(colon_pos) = var_expr.find(':') {
            (&var_expr[..colon_pos], Some(&var_expr[colon_pos + 1..]))
        } else {
            (var_expr, None)
        };

        let value = match env::var(var_name) {
            Ok(val) => val,
            Err(_) => {
                if let Some(default) = default_value {
                    default.to_string()
                } else {
                    return Err(ShimError::EnvExpansion(format!(
                        "Environment variable '{}' not found",
                        var_name
                    )));
                }
            }
        };

        result.replace_range(start..=end, &value);
    }

    // Handle simple $VAR format using shellexpand
    match shellexpand::env(&result) {
        Ok(expanded) => Ok(expanded.to_string()),
        Err(e) => Err(ShimError::EnvExpansion(e.to_string())),
    }
}

/// Get platform-specific executable extension
pub fn get_exe_extension() -> &'static str {
    if cfg!(windows) {
        ".exe"
    } else {
        ""
    }
}

/// Get platform-specific path separator
pub fn get_path_separator() -> &'static str {
    if cfg!(windows) {
        "\\"
    } else {
        "/"
    }
}

/// Get built-in environment variables
pub fn get_builtin_env_vars() -> HashMap<String, String> {
    let mut vars = HashMap::new();

    vars.insert("EXE_EXT".to_string(), get_exe_extension().to_string());
    vars.insert("PATH_SEP".to_string(), get_path_separator().to_string());

    // Add common directories
    if let Some(home) = dirs::home_dir() {
        vars.insert("HOME".to_string(), home.to_string_lossy().to_string());
    }

    if let Some(config) = dirs::config_dir() {
        vars.insert(
            "CONFIG_DIR".to_string(),
            config.to_string_lossy().to_string(),
        );
    }

    if let Some(data) = dirs::data_dir() {
        vars.insert("DATA_DIR".to_string(), data.to_string_lossy().to_string());
    }

    vars
}

/// Merge environment variables with built-in ones
pub fn merge_env_vars(custom_env: &HashMap<String, String>) -> HashMap<String, String> {
    let mut env_vars = get_builtin_env_vars();

    // Add current environment variables
    for (key, value) in env::vars() {
        env_vars.insert(key, value);
    }

    // Override with custom environment variables
    for (key, value) in custom_env {
        env_vars.insert(key.clone(), value.clone());
    }

    env_vars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_env_vars_with_default() {
        env::set_var("TEST_VAR", "test_value");

        let result = expand_env_vars("${TEST_VAR}").unwrap();
        assert_eq!(result, "test_value");

        let result = expand_env_vars("${NONEXISTENT:default_value}").unwrap();
        assert_eq!(result, "default_value");

        env::remove_var("TEST_VAR");
    }

    #[test]
    fn test_builtin_env_vars() {
        let vars = get_builtin_env_vars();
        assert!(vars.contains_key("EXE_EXT"));
        assert!(vars.contains_key("PATH_SEP"));
    }
}
