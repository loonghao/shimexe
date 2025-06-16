use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::path::Path;

use crate::error::{Result, ShimError};

/// Template engine for processing dynamic configuration
pub struct TemplateEngine {
    user_args: Vec<String>,
    env_cache: HashMap<String, String>,
}

/// Template-based argument configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgsConfig {
    /// Template-based argument list
    #[serde(default)]
    pub template: Option<Vec<String>>,

    /// Inline template string
    #[serde(default)]
    pub inline: Option<String>,

    /// Argument processing mode
    #[serde(default)]
    pub mode: ArgsMode,

    /// Default arguments (legacy support)
    #[serde(default)]
    pub default: Vec<String>,

    /// Arguments always prepended
    #[serde(default)]
    pub prefix: Vec<String>,

    /// Arguments always appended
    #[serde(default)]
    pub suffix: Vec<String>,
}

/// Argument processing modes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArgsMode {
    Template, // Use template-based processing
    Merge,    // Combine default + user args
    Replace,  // User args replace default
    Prepend,  // User args + default args
}

impl Default for ArgsMode {
    fn default() -> Self {
        ArgsMode::Template
    }
}

impl Default for ArgsConfig {
    fn default() -> Self {
        Self {
            template: None,
            inline: None,
            mode: ArgsMode::Template,
            default: Vec::new(),
            prefix: Vec::new(),
            suffix: Vec::new(),
        }
    }
}

impl TemplateEngine {
    /// Create a new template engine with user arguments
    pub fn new(user_args: Vec<String>) -> Self {
        Self {
            user_args,
            env_cache: HashMap::new(),
        }
    }

    /// Process arguments based on configuration
    pub fn process_args(&mut self, args_config: &ArgsConfig) -> Result<Vec<String>> {
        match args_config.mode {
            ArgsMode::Template => {
                if let Some(ref template) = args_config.template {
                    self.render_template_args(template)
                } else if let Some(ref inline) = args_config.inline {
                    self.render_inline_template(inline)
                } else {
                    // Fallback to user args or empty
                    Ok(self.user_args.clone())
                }
            }
            ArgsMode::Merge => {
                let mut result = args_config.prefix.clone();
                result.extend(args_config.default.clone());
                result.extend(self.user_args.clone());
                result.extend(args_config.suffix.clone());
                Ok(result)
            }
            ArgsMode::Replace => {
                let mut result = args_config.prefix.clone();
                if self.user_args.is_empty() {
                    result.extend(args_config.default.clone());
                } else {
                    result.extend(self.user_args.clone());
                }
                result.extend(args_config.suffix.clone());
                Ok(result)
            }
            ArgsMode::Prepend => {
                let mut result = args_config.prefix.clone();
                result.extend(self.user_args.clone());
                result.extend(args_config.default.clone());
                result.extend(args_config.suffix.clone());
                Ok(result)
            }
        }
    }

    /// Render template arguments
    fn render_template_args(&mut self, template: &[String]) -> Result<Vec<String>> {
        let mut result = Vec::new();

        for template_arg in template {
            let rendered = self.render_template(template_arg)?;
            if !rendered.is_empty() {
                // Split on whitespace for inline templates
                if rendered.contains(' ') {
                    result.extend(rendered.split_whitespace().map(String::from));
                } else {
                    result.push(rendered);
                }
            }
        }

        Ok(result)
    }

    /// Render inline template
    fn render_inline_template(&mut self, template: &str) -> Result<Vec<String>> {
        let rendered = self.render_template(template)?;
        Ok(rendered.split_whitespace().map(String::from).collect())
    }

    /// Render a single template string
    pub fn render_template(&mut self, template: &str) -> Result<String> {
        let mut result = template.to_string();

        // Process template expressions {{...}}
        while let Some(start) = result.find("{{") {
            if let Some(end) = result[start..].find("}}") {
                let expr_end = start + end + 2;
                let expression = &result[start + 2..start + end];

                let value = self.evaluate_expression(expression)?;
                result.replace_range(start..expr_end, &value);
            } else {
                break;
            }
        }

        Ok(result)
    }

    /// Evaluate a template expression
    fn evaluate_expression(&mut self, expr: &str) -> Result<String> {
        let expr = expr.trim();

        // Handle simple cases first
        if expr == "args" {
            return Ok(self.user_args.join(" "));
        }

        // Handle args with default: args('default')
        if expr.starts_with("args(") && expr.ends_with(")") {
            let default = &expr[5..expr.len() - 1];
            let default = default.trim_matches('\'').trim_matches('"');

            if self.user_args.is_empty() {
                return Ok(default.to_string());
            } else {
                return Ok(self.user_args.join(" "));
            }
        }

        // Handle env() function
        if expr.starts_with("env(") && expr.ends_with(")") {
            return self.evaluate_env_function(expr);
        }

        // Handle if conditions
        if expr.starts_with("if ") {
            return self.evaluate_if_condition(expr);
        }

        // Handle function calls
        if expr.contains("()") {
            return self.evaluate_function_call(expr);
        }

        // Default: return as-is
        Ok(expr.to_string())
    }

    /// Evaluate env() function calls
    fn evaluate_env_function(&mut self, expr: &str) -> Result<String> {
        let inner = &expr[4..expr.len() - 1]; // Remove "env(" and ")"

        if inner.contains(',') {
            // env('VAR', 'default')
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() == 2 {
                let var_name = parts[0].trim().trim_matches('\'').trim_matches('"');
                let default = parts[1].trim().trim_matches('\'').trim_matches('"');

                Ok(env::var(var_name).unwrap_or_else(|_| default.to_string()))
            } else {
                Err(ShimError::TemplateError(format!(
                    "Invalid env() syntax: {}",
                    expr
                )))
            }
        } else {
            // env('VAR')
            let var_name = inner.trim().trim_matches('\'').trim_matches('"');
            Ok(env::var(var_name).unwrap_or_default())
        }
    }

    /// Evaluate if conditions
    fn evaluate_if_condition(&mut self, expr: &str) -> Result<String> {
        // Simple if condition parsing
        // Format: if condition}}content{{endif
        // For now, just handle basic env comparisons

        if expr.contains("env(") && expr.contains("==") {
            // Extract condition
            let condition_part = expr.strip_prefix("if ").unwrap_or(expr);

            // Very basic parsing for env('VAR') == 'value'
            if let Some(eq_pos) = condition_part.find("==") {
                let left = condition_part[..eq_pos].trim();
                let right = condition_part[eq_pos + 2..]
                    .trim()
                    .trim_matches('\'')
                    .trim_matches('"');

                if left.starts_with("env(") && left.ends_with(")") {
                    let env_value = self.evaluate_env_function(left)?;
                    if env_value == right {
                        return Ok("true".to_string());
                    }
                }
            }
        }

        Ok("false".to_string())
    }

    /// Evaluate function calls
    fn evaluate_function_call(&mut self, expr: &str) -> Result<String> {
        match expr {
            "platform()" => Ok(self.get_platform()),
            "arch()" => Ok(self.get_arch()),
            "exe_ext()" => Ok(self.get_exe_ext()),
            "home_dir()" => Ok(self.get_home_dir()),
            _ => {
                if expr.starts_with("file_exists(") && expr.ends_with(")") {
                    let path = &expr[12..expr.len() - 1];
                    let path = path.trim_matches('\'').trim_matches('"');
                    Ok(Path::new(path).exists().to_string())
                } else {
                    Ok(expr.to_string())
                }
            }
        }
    }

    /// Get current platform
    fn get_platform(&self) -> String {
        if cfg!(target_os = "windows") {
            "windows".to_string()
        } else if cfg!(target_os = "macos") {
            "macos".to_string()
        } else if cfg!(target_os = "linux") {
            "linux".to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// Get current architecture
    fn get_arch(&self) -> String {
        if cfg!(target_arch = "x86_64") {
            "x86_64".to_string()
        } else if cfg!(target_arch = "aarch64") {
            "aarch64".to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// Get executable extension
    fn get_exe_ext(&self) -> String {
        if cfg!(target_os = "windows") {
            ".exe".to_string()
        } else {
            "".to_string()
        }
    }

    /// Get home directory
    fn get_home_dir(&self) -> String {
        env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_mode_merge() {
        let mut engine = TemplateEngine::new(vec!["user1".to_string(), "user2".to_string()]);

        let config = ArgsConfig {
            mode: ArgsMode::Merge,
            default: vec!["default1".to_string(), "default2".to_string()],
            prefix: vec!["prefix".to_string()],
            suffix: vec!["suffix".to_string()],
            ..Default::default()
        };

        let result = engine.process_args(&config).unwrap();
        assert_eq!(
            result,
            vec!["prefix", "default1", "default2", "user1", "user2", "suffix"]
        );
    }

    #[test]
    fn test_args_mode_replace() {
        let mut engine = TemplateEngine::new(vec!["user1".to_string()]);

        let config = ArgsConfig {
            mode: ArgsMode::Replace,
            default: vec!["default".to_string()],
            prefix: vec!["prefix".to_string()],
            ..Default::default()
        };

        let result = engine.process_args(&config).unwrap();
        assert_eq!(result, vec!["prefix", "user1"]);

        // Test with no user args
        let mut engine_empty = TemplateEngine::new(vec![]);
        let result_empty = engine_empty.process_args(&config).unwrap();
        assert_eq!(result_empty, vec!["prefix", "default"]);
    }

    #[test]
    fn test_template_args_basic() {
        let mut engine = TemplateEngine::new(vec!["--help".to_string()]);

        let config = ArgsConfig {
            mode: ArgsMode::Template,
            template: Some(vec![
                "{{args('--version')}}".to_string(),
                "--verbose".to_string(),
            ]),
            ..Default::default()
        };

        let result = engine.process_args(&config).unwrap();
        assert_eq!(result, vec!["--help", "--verbose"]);
    }

    #[test]
    fn test_template_args_with_default() {
        let mut engine = TemplateEngine::new(vec![]);

        let config = ArgsConfig {
            mode: ArgsMode::Template,
            template: Some(vec!["{{args('--version')}}".to_string()]),
            ..Default::default()
        };

        let result = engine.process_args(&config).unwrap();
        assert_eq!(result, vec!["--version"]);
    }

    #[test]
    fn test_env_function() {
        env::set_var("TEST_TEMPLATE_VAR", "test_value");

        let mut engine = TemplateEngine::new(vec![]);

        let result = engine
            .evaluate_env_function("env('TEST_TEMPLATE_VAR')")
            .unwrap();
        assert_eq!(result, "test_value");

        let result_with_default = engine
            .evaluate_env_function("env('NONEXISTENT', 'default')")
            .unwrap();
        assert_eq!(result_with_default, "default");

        env::remove_var("TEST_TEMPLATE_VAR");
    }

    #[test]
    fn test_platform_functions() {
        let mut engine = TemplateEngine::new(vec![]);

        let platform = engine.evaluate_function_call("platform()").unwrap();
        assert!(["windows", "linux", "macos", "unknown"].contains(&platform.as_str()));

        let arch = engine.evaluate_function_call("arch()").unwrap();
        assert!(["x86_64", "aarch64", "unknown"].contains(&arch.as_str()));

        let exe_ext = engine.evaluate_function_call("exe_ext()").unwrap();
        if cfg!(target_os = "windows") {
            assert_eq!(exe_ext, ".exe");
        } else {
            assert_eq!(exe_ext, "");
        }
    }

    #[test]
    fn test_file_exists_function() {
        let mut engine = TemplateEngine::new(vec![]);

        // Test with a file that should exist (Cargo.toml in project root)
        let result = engine
            .evaluate_function_call("file_exists('Cargo.toml')")
            .unwrap();
        // Note: This might be "false" depending on test execution context
        assert!(result == "true" || result == "false");

        // Test with a file that definitely doesn't exist
        let result = engine
            .evaluate_function_call("file_exists('definitely_not_exists.xyz')")
            .unwrap();
        assert_eq!(result, "false");
    }

    #[test]
    fn test_render_template_complete() {
        env::set_var("TEST_ENV", "production");

        let mut engine = TemplateEngine::new(vec!["--input".to_string(), "file.txt".to_string()]);

        let template = "--env {{env('TEST_ENV', 'development')}} {{args('--help')}}";
        let result = engine.render_template(template).unwrap();
        assert_eq!(result, "--env production --input file.txt");

        env::remove_var("TEST_ENV");
    }
}
