use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::error::{Result, ShimError};
use crate::template::ArgsConfig;
use crate::utils::expand_env_vars;

/// Main shim configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShimConfig {
    /// Core shim configuration
    pub shim: ShimCore,
    /// Advanced argument configuration
    #[serde(default)]
    pub args: ArgsConfig,
    /// Environment variables to set
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Optional metadata
    #[serde(default)]
    pub metadata: ShimMetadata,
    /// Auto-update configuration
    #[serde(default)]
    pub auto_update: Option<AutoUpdate>,
}

/// Core shim configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShimCore {
    /// Name of the shim
    pub name: String,
    /// Path to the target executable
    pub path: String,
    /// Default arguments to pass to the executable
    #[serde(default)]
    pub args: Vec<String>,
    /// Working directory for the executable
    #[serde(default)]
    pub cwd: Option<String>,
}

/// Optional metadata for the shim
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShimMetadata {
    /// Description of the shim
    pub description: Option<String>,
    /// Version of the shim configuration
    pub version: Option<String>,
    /// Author of the shim
    pub author: Option<String>,
    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Auto-update configuration for the shim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoUpdate {
    /// Enable auto-update
    #[serde(default)]
    pub enabled: bool,
    /// Update provider type
    pub provider: UpdateProvider,
    /// Download URL template with version placeholder
    pub download_url: String,
    /// Version check URL or pattern
    pub version_check: VersionCheck,
    /// Update frequency in hours (0 = check every run)
    #[serde(default)]
    pub check_interval_hours: u64,
    /// Pre-update command to run
    pub pre_update_command: Option<String>,
    /// Post-update command to run
    pub post_update_command: Option<String>,
}

/// Update provider types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UpdateProvider {
    /// GitHub releases
    Github {
        /// Repository owner/name
        repo: String,
        /// Asset name pattern (supports {version}, {os}, {arch} placeholders)
        asset_pattern: String,
        /// Include pre-releases
        #[serde(default)]
        include_prerelease: bool,
    },
    /// Direct HTTPS download
    Https {
        /// Base URL for downloads
        base_url: String,
        /// URL pattern for version checking
        version_url: Option<String>,
    },
    /// Custom provider
    Custom {
        /// Custom update script or command
        update_command: String,
        /// Custom version check command
        version_command: String,
    },
}

/// Version check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionCheck {
    /// GitHub API for latest release
    GithubLatest {
        /// Repository owner/name
        repo: String,
        /// Include pre-releases
        #[serde(default)]
        include_prerelease: bool,
    },
    /// HTTP endpoint returning version
    Http {
        /// URL to check for version
        url: String,
        /// JSON path to extract version (e.g., "$.version")
        json_path: Option<String>,
        /// Regex pattern to extract version
        regex_pattern: Option<String>,
    },
    /// Semantic version comparison
    Semver {
        /// Current version
        current: String,
        /// Version check URL
        check_url: String,
    },
    /// Custom version check command
    Command {
        /// Command to run for version check
        command: String,
        /// Arguments for the command
        args: Vec<String>,
    },
}

impl ShimConfig {
    /// Load shim configuration from a TOML file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(&path).map_err(ShimError::Io)?;

        let config: ShimConfig = toml::from_str(&content).map_err(ShimError::TomlParse)?;

        config.validate()?;
        Ok(config)
    }

    /// Save shim configuration to a TOML file
    pub fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self).map_err(ShimError::TomlSerialize)?;

        std::fs::write(path, content).map_err(ShimError::Io)?;

        Ok(())
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.shim.name.is_empty() {
            return Err(ShimError::Config("Shim name cannot be empty".to_string()));
        }

        if self.shim.path.is_empty() {
            return Err(ShimError::Config("Shim path cannot be empty".to_string()));
        }

        Ok(())
    }

    /// Expand environment variables in the configuration
    pub fn expand_env_vars(&mut self) -> Result<()> {
        // Expand path
        self.shim.path = expand_env_vars(&self.shim.path)?;

        // Expand args
        for arg in &mut self.shim.args {
            *arg = expand_env_vars(arg)?;
        }

        // Expand cwd if present
        if let Some(ref mut cwd) = self.shim.cwd {
            *cwd = expand_env_vars(cwd)?;
        }

        // Expand environment variables
        for value in self.env.values_mut() {
            *value = expand_env_vars(value)?;
        }

        Ok(())
    }

    /// Get the resolved executable path
    pub fn get_executable_path(&self) -> Result<PathBuf> {
        let expanded_path = expand_env_vars(&self.shim.path)?;
        let path = PathBuf::from(expanded_path);

        if path.is_absolute() {
            Ok(path)
        } else {
            // Try to find in PATH
            which::which(&path).map_err(|_| ShimError::ExecutableNotFound(self.shim.path.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_shim_config_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
[shim]
name = "test"
path = "echo"
args = ["hello"]

[env]
TEST_VAR = "test_value"

[metadata]
description = "Test shim"
version = "1.0.0"
        "#
        )
        .unwrap();

        let config = ShimConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(config.shim.name, "test");
        assert_eq!(config.shim.path, "echo");
        assert_eq!(config.shim.args, vec!["hello"]);
        assert_eq!(config.env.get("TEST_VAR"), Some(&"test_value".to_string()));
        assert_eq!(config.metadata.description, Some("Test shim".to_string()));
    }

    #[test]
    fn test_shim_config_basic_structure() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
[shim]
name = "test"
path = "echo"

[args]
mode = "template"

[metadata]
description = "Test shim"
        "#
        )
        .unwrap();

        let config = ShimConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(config.shim.name, "test");
        assert_eq!(config.args.mode, crate::template::ArgsMode::Template);
        assert_eq!(config.metadata.description, Some("Test shim".to_string()));
    }

    #[test]
    fn test_shim_config_validation() {
        // Valid config
        let config = ShimConfig {
            shim: ShimCore {
                name: "test".to_string(),
                path: "echo".to_string(),
                args: vec![],
                cwd: None,
            },
            args: Default::default(),
            env: HashMap::new(),
            metadata: Default::default(),
            auto_update: None,
        };
        assert!(config.validate().is_ok());

        // Invalid config - empty name
        let invalid_config = ShimConfig {
            shim: ShimCore {
                name: "".to_string(),
                path: "echo".to_string(),
                args: vec![],
                cwd: None,
            },
            args: Default::default(),
            env: HashMap::new(),
            metadata: Default::default(),
            auto_update: None,
        };
        assert!(invalid_config.validate().is_err());

        // Invalid config - empty path
        let invalid_config = ShimConfig {
            shim: ShimCore {
                name: "test".to_string(),
                path: "".to_string(),
                args: vec![],
                cwd: None,
            },
            args: Default::default(),
            env: HashMap::new(),
            metadata: Default::default(),
            auto_update: None,
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_shim_config_to_file() {
        let config = ShimConfig {
            shim: ShimCore {
                name: "test".to_string(),
                path: "echo".to_string(),
                args: vec!["hello".to_string()],
                cwd: None,
            },
            args: Default::default(),
            env: {
                let mut env = HashMap::new();
                env.insert("TEST_VAR".to_string(), "test_value".to_string());
                env
            },
            metadata: ShimMetadata {
                description: Some("Test shim".to_string()),
                version: Some("1.0.0".to_string()),
                author: None,
                tags: vec![],
            },
            auto_update: None,
        };

        let temp_file = NamedTempFile::new().unwrap();
        config.to_file(temp_file.path()).unwrap();

        // Read back and verify
        let loaded_config = ShimConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(loaded_config.shim.name, config.shim.name);
        assert_eq!(loaded_config.shim.path, config.shim.path);
        assert_eq!(loaded_config.shim.args, config.shim.args);
        assert_eq!(loaded_config.env, config.env);
    }

    #[test]
    fn test_expand_env_vars() {
        std::env::set_var("TEST_VAR", "test_value");

        let mut config = ShimConfig {
            shim: ShimCore {
                name: "test".to_string(),
                path: "${TEST_VAR}/bin/test".to_string(),
                args: vec!["${TEST_VAR}".to_string()],
                cwd: Some("${TEST_VAR}/work".to_string()),
            },
            args: Default::default(),
            env: {
                let mut env = HashMap::new();
                env.insert("EXPANDED".to_string(), "${TEST_VAR}_expanded".to_string());
                env
            },
            metadata: Default::default(),
            auto_update: None,
        };

        config.expand_env_vars().unwrap();

        assert_eq!(config.shim.path, "test_value/bin/test");
        assert_eq!(config.shim.args[0], "test_value");
        assert_eq!(config.shim.cwd, Some("test_value/work".to_string()));
        assert_eq!(
            config.env.get("EXPANDED"),
            Some(&"test_value_expanded".to_string())
        );

        std::env::remove_var("TEST_VAR");
    }

    #[test]
    fn test_shim_config_with_args_template() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(
            temp_file,
            r#"
[shim]
name = "test"
path = "echo"

[args]
mode = "template"
template = [
    "{{{{if env('DEBUG') == 'true'}}}}--verbose{{{{endif}}}}",
    "{{{{args('--version')}}}}"
]

[metadata]
description = "Test shim with template args"
        "#
        )
        .unwrap();

        let config = ShimConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(config.shim.name, "test");
        assert_eq!(config.args.mode, crate::template::ArgsMode::Template);
        assert!(config.args.template.is_some());

        let template = config.args.template.unwrap();
        assert_eq!(template.len(), 2);
        assert_eq!(
            template[0],
            "{{if env('DEBUG') == 'true'}}--verbose{{endif}}"
        );
        assert_eq!(template[1], "{{args('--version')}}");
    }

    #[test]
    fn test_shim_config_with_args_modes() {
        // Test merge mode
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(
            temp_file,
            r#"
[shim]
name = "test"
path = "echo"

[args]
mode = "merge"
default = ["--default"]
prefix = ["--prefix"]
suffix = ["--suffix"]
        "#
        )
        .unwrap();

        let config = ShimConfig::from_file(temp_file.path()).unwrap();
        assert_eq!(config.args.mode, crate::template::ArgsMode::Merge);
        assert_eq!(config.args.default, vec!["--default"]);
        assert_eq!(config.args.prefix, vec!["--prefix"]);
        assert_eq!(config.args.suffix, vec!["--suffix"]);
    }

    #[test]
    fn test_shim_config_with_inline_template() {
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(
            temp_file,
            r#"
[shim]
name = "test"
path = "echo"

[args]
inline = "{{{{env('CONFIG_PATH', '/default/config')}}}} {{{{args('--help')}}}}"
        "#
        )
        .unwrap();

        let config = ShimConfig::from_file(temp_file.path()).unwrap();
        assert!(config.args.inline.is_some());
        assert_eq!(
            config.args.inline.unwrap(),
            "{{env('CONFIG_PATH', '/default/config')}} {{args('--help')}}"
        );
    }
}
