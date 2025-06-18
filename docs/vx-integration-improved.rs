//! Improved shimexe integration for vx tool manager
//!
//! This module provides a better integration between vx-core and shimexe,
//! addressing the design issues in the original implementation.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Improved VX-Shimexe integration manager
pub struct VxShimexeManager {
    /// Path to the shim directory
    shim_dir: PathBuf,
    /// Whether to use shimexe CLI or library API
    use_cli: bool,
}

/// Shim metadata for VX integration
#[derive(Debug, Serialize, Deserialize)]
pub struct VxShimMetadata {
    pub tool_name: String,
    pub version: String,
    pub tool_path: PathBuf,
    pub created_at: String,
    pub vx_managed: bool,
}

impl VxShimexeManager {
    /// Create a new improved VxShimexeManager
    pub fn new(shim_dir: PathBuf) -> Result<Self> {
        // Ensure shim directory exists
        fs::create_dir_all(&shim_dir)
            .with_context(|| format!("Failed to create shim directory: {}", shim_dir.display()))?;

        // Check if shimexe CLI is available
        let use_cli = Command::new("shimexe")
            .arg("--version")
            .output()
            .is_ok();

        Ok(Self { shim_dir, use_cli })
    }

    /// Create a tool shim using the best available method
    pub fn create_tool_shim(
        &self,
        tool_name: &str,
        tool_path: &Path,
        version: &str,
        args: Option<Vec<String>>,
        env_vars: Option<HashMap<String, String>>,
    ) -> Result<PathBuf> {
        if self.use_cli {
            self.create_shim_via_cli(tool_name, tool_path, args, env_vars)
        } else {
            self.create_shim_via_library(tool_name, tool_path, args, env_vars)
        }
        .and_then(|shim_path| {
            // Store VX-specific metadata
            self.store_vx_metadata(tool_name, version, tool_path, &shim_path)?;
            Ok(shim_path)
        })
    }

    /// Create shim using shimexe CLI (preferred method)
    fn create_shim_via_cli(
        &self,
        tool_name: &str,
        tool_path: &Path,
        args: Option<Vec<String>>,
        env_vars: Option<HashMap<String, String>>,
    ) -> Result<PathBuf> {
        let mut cmd = Command::new("shimexe");
        cmd.arg("add")
            .arg(tool_name)
            .arg("--path")
            .arg(tool_path)
            .arg("--shim-dir")
            .arg(&self.shim_dir);

        // Add arguments if provided
        if let Some(args) = args {
            for arg in args {
                cmd.arg("--args").arg(arg);
            }
        }

        // Add environment variables if provided
        if let Some(env_vars) = env_vars {
            for (key, value) in env_vars {
                cmd.arg("--env").arg(format!("{}={}", key, value));
            }
        }

        // Execute the command
        let output = cmd.output()
            .with_context(|| "Failed to execute shimexe command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("shimexe command failed: {}", stderr));
        }

        // Return the expected shim path
        let shim_path = if cfg!(windows) {
            self.shim_dir.join(format!("{}.exe", tool_name))
        } else {
            self.shim_dir.join(tool_name)
        };

        Ok(shim_path)
    }

    /// Create shim using shimexe-core library (fallback method)
    fn create_shim_via_library(
        &self,
        tool_name: &str,
        tool_path: &Path,
        args: Option<Vec<String>>,
        env_vars: Option<HashMap<String, String>>,
    ) -> Result<PathBuf> {
        // Use shimexe-core API correctly
        use shimexe_core::{ShimConfig, ShimCore, ShimMetadata};

        let config = ShimConfig {
            shim: ShimCore {
                name: tool_name.to_string(),
                path: tool_path.to_string_lossy().to_string(),
                args: args.unwrap_or_default(),
                cwd: None,
                download_url: None,
                source_type: None,
                extracted_executables: None,
            },
            env: env_vars.unwrap_or_default(),
            metadata: ShimMetadata {
                description: Some(format!("VX-managed shim for {}", tool_name)),
                version: Some("vx-managed".to_string()),
                author: Some("vx".to_string()),
                tags: Some(vec!["vx".to_string(), "tool".to_string()]),
            },
        };

        // Write configuration file
        let config_path = self.shim_dir.join(format!("{}.shim.toml", tool_name));
        config.to_file(&config_path)
            .with_context(|| format!("Failed to write shim config for {}", tool_name))?;

        // Copy shimexe binary as the shim executable
        let shim_path = if cfg!(windows) {
            self.shim_dir.join(format!("{}.exe", tool_name))
        } else {
            self.shim_dir.join(tool_name)
        };

        self.copy_shimexe_binary(&shim_path)
            .with_context(|| "Failed to create shim executable")?;

        Ok(shim_path)
    }

    /// Copy shimexe binary to create the shim executable
    fn copy_shimexe_binary(&self, dest_path: &Path) -> Result<()> {
        // Try to find shimexe binary
        let shimexe_path = which::which("shimexe")
            .or_else(|_| std::env::current_exe())
            .with_context(|| "Could not find shimexe binary")?;

        fs::copy(&shimexe_path, dest_path)
            .with_context(|| format!("Failed to copy shimexe binary to {}", dest_path.display()))?;

        // Make executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(dest_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(dest_path, perms)?;
        }

        Ok(())
    }

    /// Store VX-specific metadata
    fn store_vx_metadata(
        &self,
        tool_name: &str,
        version: &str,
        tool_path: &Path,
        shim_path: &Path,
    ) -> Result<()> {
        let metadata_dir = self.shim_dir.join(".vx-metadata");
        fs::create_dir_all(&metadata_dir)?;

        let metadata = VxShimMetadata {
            tool_name: tool_name.to_string(),
            version: version.to_string(),
            tool_path: tool_path.to_path_buf(),
            created_at: chrono::Utc::now().to_rfc3339(),
            vx_managed: true,
        };

        let metadata_file = metadata_dir.join(format!("{}.json", tool_name));
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        fs::write(&metadata_file, metadata_json)
            .with_context(|| format!("Failed to write VX metadata for {}", tool_name))?;

        Ok(())
    }

    /// List all VX-managed shims
    pub fn list_vx_shims(&self) -> Result<Vec<VxShimMetadata>> {
        let metadata_dir = self.shim_dir.join(".vx-metadata");
        let mut shims = Vec::new();

        if !metadata_dir.exists() {
            return Ok(shims);
        }

        for entry in fs::read_dir(&metadata_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)?;
                if let Ok(metadata) = serde_json::from_str::<VxShimMetadata>(&content) {
                    shims.push(metadata);
                }
            }
        }

        shims.sort_by(|a, b| a.tool_name.cmp(&b.tool_name));
        Ok(shims)
    }

    /// Remove a VX-managed shim
    pub fn remove_shim(&self, tool_name: &str) -> Result<()> {
        if self.use_cli {
            // Use shimexe CLI to remove
            let output = Command::new("shimexe")
                .arg("remove")
                .arg(tool_name)
                .arg("--shim-dir")
                .arg(&self.shim_dir)
                .output()
                .with_context(|| "Failed to execute shimexe remove command")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow::anyhow!("shimexe remove failed: {}", stderr));
            }
        } else {
            // Manual removal
            let config_path = self.shim_dir.join(format!("{}.shim.toml", tool_name));
            let shim_path = if cfg!(windows) {
                self.shim_dir.join(format!("{}.exe", tool_name))
            } else {
                self.shim_dir.join(tool_name)
            };

            if config_path.exists() {
                fs::remove_file(&config_path)?;
            }
            if shim_path.exists() {
                fs::remove_file(&shim_path)?;
            }
        }

        // Remove VX metadata
        let metadata_file = self.shim_dir.join(".vx-metadata").join(format!("{}.json", tool_name));
        if metadata_file.exists() {
            fs::remove_file(&metadata_file)?;
        }

        Ok(())
    }

    /// Switch tool version by updating the shim
    pub fn switch_tool_version(
        &self,
        tool_name: &str,
        version: &str,
        new_tool_path: &Path,
    ) -> Result<PathBuf> {
        // Remove existing shim
        let _ = self.remove_shim(tool_name);

        // Create new shim with updated path
        self.create_tool_shim(tool_name, new_tool_path, version, None, None)
    }

    /// Validate that a shim is working correctly
    pub fn validate_shim(&self, tool_name: &str) -> Result<bool> {
        let config_path = self.shim_dir.join(format!("{}.shim.toml", tool_name));
        
        if self.use_cli {
            let output = Command::new("shimexe")
                .arg("validate")
                .arg(&config_path)
                .output()
                .with_context(|| "Failed to validate shim")?;

            Ok(output.status.success())
        } else {
            // Basic validation: check if files exist
            let shim_path = if cfg!(windows) {
                self.shim_dir.join(format!("{}.exe", tool_name))
            } else {
                self.shim_dir.join(tool_name)
            };

            Ok(config_path.exists() && shim_path.exists())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_vx_shimexe_manager_creation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let manager = VxShimexeManager::new(temp_dir.path().to_path_buf())?;
        
        assert!(temp_dir.path().exists());
        Ok(())
    }

    #[test]
    fn test_metadata_storage() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let manager = VxShimexeManager::new(temp_dir.path().to_path_buf())?;
        
        let tool_path = PathBuf::from("/usr/bin/echo");
        manager.store_vx_metadata("test-tool", "1.0.0", &tool_path, &tool_path)?;
        
        let shims = manager.list_vx_shims()?;
        assert_eq!(shims.len(), 1);
        assert_eq!(shims[0].tool_name, "test-tool");
        assert_eq!(shims[0].version, "1.0.0");
        
        Ok(())
    }
}
