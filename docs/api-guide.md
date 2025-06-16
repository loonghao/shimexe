# shimexe API Guide

This guide covers how to use shimexe as a library in your Rust projects, including custom implementations and extensions.

## Table of Contents

- [Basic Usage](#basic-usage)
- [Custom Configuration Loaders](#custom-configuration-loaders)
- [Custom Update Providers](#custom-update-providers)
- [Custom Version Checkers](#custom-version-checkers)
- [Extensible Runner](#extensible-runner)
- [Integration Examples](#integration-examples)

## Basic Usage

Add shimexe to your `Cargo.toml`:

```toml
[dependencies]
shimexe-core = "0.1"
```

### Simple Shim Execution

```rust
use shimexe_core::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Load and run a shim
    let runner = ShimRunner::from_file("my-tool.shim.toml")?;
    let exit_code = runner.execute(&["--help"])?;
    
    println!("Process exited with code: {}", exit_code);
    Ok(())
}
```

### Creating Shims Programmatically

```rust
use shimexe_core::*;
use std::collections::HashMap;

fn create_python_shim() -> Result<()> {
    let mut env_vars = HashMap::new();
    env_vars.insert("PYTHONPATH".to_string(), "${PROJECT_ROOT}/src".to_string());
    
    let config = ShimConfig {
        shim: ShimCore {
            name: "python".to_string(),
            path: "${PYTHON_HOME}/bin/python${EXE_EXT}".to_string(),
            args: vec![],
            cwd: Some("${PROJECT_ROOT}".to_string()),
        },
        env: env_vars,
        metadata: ShimMetadata {
            description: Some("Python interpreter shim".to_string()),
            version: Some("1.0.0".to_string()),
            author: Some("My Project".to_string()),
            tags: vec!["python".to_string(), "interpreter".to_string()],
        },
        auto_update: None,
    };
    
    config.to_file("python.shim.toml")?;
    Ok(())
}
```

## Custom Configuration Loaders

Implement the `ShimConfigLoader` trait to support custom configuration formats:

```rust
use shimexe_core::traits::ShimConfigLoader;
use shimexe_core::{ShimConfig, Result};
use std::path::Path;

pub struct JsonConfigLoader;

impl ShimConfigLoader for JsonConfigLoader {
    fn load_config<P: AsRef<Path>>(&self, path: P) -> Result<ShimConfig> {
        let content = std::fs::read_to_string(path)?;
        let config: ShimConfig = serde_json::from_str(&content)
            .map_err(|e| shimexe_core::ShimError::Config(e.to_string()))?;
        Ok(config)
    }
    
    fn save_config<P: AsRef<Path>>(&self, config: &ShimConfig, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(config)
            .map_err(|e| shimexe_core::ShimError::Config(e.to_string()))?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    fn file_extension(&self) -> &str {
        "shim.json"
    }
}

// Usage
fn use_json_loader() -> Result<()> {
    let runner = ShimRunnerBuilder::new()
        .with_config_loader(Box::new(JsonConfigLoader))
        .with_config_file_pattern("{name}.shim.json")
        .build("my-tool", Path::new("./shims"))?;
    
    Ok(())
}
```

## Custom Update Providers

Implement custom update logic by implementing the `UpdateProvider` trait:

```rust
use shimexe_core::traits::UpdateProvider;
use async_trait::async_trait;
use std::path::Path;

pub struct CustomUpdateProvider {
    base_url: String,
    api_key: String,
}

#[async_trait]
impl UpdateProvider for CustomUpdateProvider {
    async fn check_update_available(&self, current_version: &str) -> Result<Option<String>> {
        // Custom logic to check for updates
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/api/latest", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| shimexe_core::ShimError::ProcessExecution(e.to_string()))?;
        
        let latest_version: String = response.json().await
            .map_err(|e| shimexe_core::ShimError::ProcessExecution(e.to_string()))?;
        
        if latest_version != current_version {
            Ok(Some(latest_version))
        } else {
            Ok(None)
        }
    }
    
    async fn install_update(&self, version: &str, target_path: &Path) -> Result<()> {
        // Custom installation logic
        let download_url = self.get_download_url(version)?;
        
        // Download and install
        let client = reqwest::Client::new();
        let response = client.get(&download_url).send().await
            .map_err(|e| shimexe_core::ShimError::ProcessExecution(e.to_string()))?;
        
        let bytes = response.bytes().await
            .map_err(|e| shimexe_core::ShimError::ProcessExecution(e.to_string()))?;
        
        std::fs::write(target_path, bytes)?;
        
        // Set executable permissions on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(target_path)?.permissions();
            perms.set_mode(perms.mode() | 0o755);
            std::fs::set_permissions(target_path, perms)?;
        }
        
        Ok(())
    }
    
    fn get_download_url(&self, version: &str) -> Result<String> {
        Ok(format!("{}/releases/{}/download", self.base_url, version))
    }
}
```

## Custom Version Checkers

Implement custom version comparison logic:

```rust
use shimexe_core::traits::VersionChecker;
use async_trait::async_trait;

pub struct SemverChecker;

#[async_trait]
impl VersionChecker for SemverChecker {
    async fn get_latest_version(&self) -> Result<String> {
        // Implementation to get latest version
        Ok("1.2.3".to_string())
    }
    
    fn is_newer_version(&self, version1: &str, version2: &str) -> Result<bool> {
        // Use semver crate for proper semantic version comparison
        let v1 = semver::Version::parse(version1)
            .map_err(|e| shimexe_core::ShimError::Config(e.to_string()))?;
        let v2 = semver::Version::parse(version2)
            .map_err(|e| shimexe_core::ShimError::Config(e.to_string()))?;
        
        Ok(v1 > v2)
    }
    
    fn parse_version(&self, version_str: &str) -> Result<String> {
        // Extract version from string using regex
        let re = regex::Regex::new(r"v?(\d+\.\d+\.\d+)")
            .map_err(|e| shimexe_core::ShimError::Config(e.to_string()))?;
        
        if let Some(captures) = re.captures(version_str) {
            Ok(captures[1].to_string())
        } else {
            Err(shimexe_core::ShimError::Config("Invalid version format".to_string()))
        }
    }
}
```

## Extensible Runner

Use the `ShimRunnerBuilder` to create highly customizable runners:

```rust
use shimexe_core::traits::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let runner = ShimRunnerBuilder::new()
        .with_config_loader(Box::new(JsonConfigLoader))
        .with_update_provider(Box::new(CustomUpdateProvider {
            base_url: "https://api.example.com".to_string(),
            api_key: "your-api-key".to_string(),
        }))
        .with_version_checker(Box::new(SemverChecker))
        .with_config_file_pattern("{name}.custom.json")
        .with_pre_execute_hook(|args| {
            println!("About to execute with args: {:?}", args);
            Ok(())
        })
        .with_post_execute_hook(|exit_code| {
            println!("Process finished with exit code: {}", exit_code);
            Ok(())
        })
        .build("my-tool", Path::new("./shims"))?;
    
    let exit_code = runner.execute(&["--version"]).await?;
    println!("Final exit code: {}", exit_code);
    
    Ok(())
}
```

## Integration Examples

### Integration with vx

```rust
// In your vx project
use shimexe_core::*;

pub struct VxShimManager {
    shim_dir: PathBuf,
}

impl VxShimManager {
    pub fn new(shim_dir: PathBuf) -> Self {
        Self { shim_dir }
    }
    
    pub fn create_tool_shim(&self, tool_name: &str, version: &str) -> Result<()> {
        let config = ShimConfig {
            shim: ShimCore {
                name: tool_name.to_string(),
                path: format!("${{VX_HOME}}/tools/{}/{}/bin/{}", tool_name, version, tool_name),
                args: vec![],
                cwd: None,
            },
            env: self.get_tool_env(tool_name, version),
            metadata: ShimMetadata {
                description: Some(format!("vx managed {} v{}", tool_name, version)),
                version: Some(version.to_string()),
                author: Some("vx".to_string()),
                tags: vec!["vx".to_string(), tool_name.to_string()],
            },
            auto_update: Some(self.create_auto_update_config(tool_name)),
        };
        
        let shim_file = self.shim_dir.join(format!("{}.shim.toml", tool_name));
        config.to_file(shim_file)?;
        
        Ok(())
    }
    
    fn get_tool_env(&self, tool_name: &str, version: &str) -> HashMap<String, String> {
        let mut env = HashMap::new();
        env.insert("VX_TOOL".to_string(), tool_name.to_string());
        env.insert("VX_VERSION".to_string(), version.to_string());
        env
    }
    
    fn create_auto_update_config(&self, tool_name: &str) -> AutoUpdate {
        AutoUpdate {
            enabled: true,
            provider: UpdateProvider::Custom {
                update_command: format!("vx install {} --latest", tool_name),
                version_command: format!("vx list {} --latest", tool_name),
            },
            download_url: "".to_string(), // Not used for custom provider
            version_check: VersionCheck::Command {
                command: "vx".to_string(),
                args: vec!["list".to_string(), tool_name.to_string(), "--latest".to_string()],
            },
            check_interval_hours: 24,
            pre_update_command: Some("echo 'Updating via vx...'".to_string()),
            post_update_command: None,
        }
    }
}
```

### Integration with rye

```rust
// In your rye project
use shimexe_core::*;

pub struct RyeShimManager {
    rye_home: PathBuf,
}

impl RyeShimManager {
    pub fn create_python_shims(&self) -> Result<()> {
        // Create Python shim
        let python_config = ShimConfig {
            shim: ShimCore {
                name: "python".to_string(),
                path: "${RYE_HOME}/shims/python${EXE_EXT}".to_string(),
                args: vec![],
                cwd: None,
            },
            env: self.get_python_env(),
            metadata: ShimMetadata {
                description: Some("Rye-managed Python interpreter".to_string()),
                version: Some("1.0.0".to_string()),
                author: Some("rye".to_string()),
                tags: vec!["python".to_string(), "rye".to_string()],
            },
            auto_update: None, // Rye handles updates
        };
        
        python_config.to_file("python.shim.toml")?;
        
        // Create pip shim
        let pip_config = ShimConfig {
            shim: ShimCore {
                name: "pip".to_string(),
                path: "${RYE_HOME}/shims/pip${EXE_EXT}".to_string(),
                args: vec![],
                cwd: None,
            },
            env: self.get_python_env(),
            metadata: ShimMetadata {
                description: Some("Rye-managed pip".to_string()),
                version: Some("1.0.0".to_string()),
                author: Some("rye".to_string()),
                tags: vec!["python".to_string(), "pip".to_string(), "rye".to_string()],
            },
            auto_update: None,
        };
        
        pip_config.to_file("pip.shim.toml")?;
        
        Ok(())
    }
    
    fn get_python_env(&self) -> HashMap<String, String> {
        let mut env = HashMap::new();
        env.insert("RYE_HOME".to_string(), self.rye_home.to_string_lossy().to_string());
        env.insert("PYTHONPATH".to_string(), "${RYE_HOME}/lib".to_string());
        env
    }
}
```

## Best Practices

1. **Use async/await** for I/O operations in custom providers
2. **Implement proper error handling** using the `Result` type
3. **Cache expensive operations** like version checks
4. **Validate configurations** before using them
5. **Use environment variable expansion** for portable configurations
6. **Test custom implementations** thoroughly
7. **Document your custom traits** for other developers
8. **Follow semantic versioning** for version comparisons
9. **Handle network failures gracefully** in update providers
10. **Use structured logging** for debugging and monitoring
