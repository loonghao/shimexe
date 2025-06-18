//! Simple example showing how vx can integrate with shimexe-core using the new ShimManager API
//!
//! This example demonstrates the improved API that makes tool manager integration much simpler.

use anyhow::Result;
use shimexe_core::prelude::*;
use shimexe_core::{ShimCore, ShimMetadata, SourceType};
use std::path::PathBuf;

/// Simplified VX integration using ShimManager
pub struct VxShimIntegration {
    manager: ShimManager,
}

impl VxShimIntegration {
    /// Create a new VX integration
    pub fn new(vx_home: PathBuf) -> Result<Self> {
        let shim_dir = vx_home.join("shims");
        let metadata_dir = vx_home.join("metadata");

        let manager = ShimManager::with_metadata_dir(shim_dir, metadata_dir)?;

        Ok(Self { manager })
    }

    /// Install a tool version and create a shim
    pub fn install_tool(&self, name: &str, version: &str, tool_path: &str) -> Result<PathBuf> {
        let shim_path = self.manager.create_shim_with_builder(name, |builder| {
            builder
                .path(tool_path)
                .version(version)
                .description(format!("{} managed by vx", name))
                .tag("vx-managed")
                .tag(name)
        })?;

        println!(
            "✅ Installed {} v{} -> {}",
            name,
            version,
            shim_path.display()
        );
        Ok(shim_path)
    }

    /// Switch tool to a different version
    pub fn switch_version(&self, name: &str, version: &str, tool_path: &str) -> Result<()> {
        let config = self
            .manager
            .builder(name)
            .path(tool_path)
            .version(version)
            .description(format!("{} managed by vx", name))
            .tag("vx-managed")
            .tag(name)
            .build()?;

        self.manager.update_shim(name, config)?;
        println!("🔄 Switched {} to version {}", name, version);
        Ok(())
    }

    /// List all managed tools
    pub fn list_tools(&self) -> Result<()> {
        let shims = self.manager.list_shims()?;

        if shims.is_empty() {
            println!("No tools installed");
            return Ok(());
        }

        println!("📦 Installed tools:");
        for shim in shims {
            let version = shim.version.as_deref().unwrap_or("unknown");
            let status = if shim.is_valid { "✅" } else { "❌" };
            println!("  {} {} v{}", status, shim.name, version);
        }

        Ok(())
    }

    /// Remove a tool
    pub fn remove_tool(&self, name: &str) -> Result<()> {
        self.manager.remove_shim(name)?;
        println!("🗑️  Removed {}", name);
        Ok(())
    }

    /// Execute a tool
    pub fn execute_tool(&self, name: &str, args: &[String]) -> Result<i32> {
        Ok(self.manager.execute_shim(name, args)?)
    }

    /// Check if a tool is installed and valid
    pub fn is_tool_valid(&self, name: &str) -> Result<bool> {
        Ok(self.manager.validate_shim(name)?)
    }

    /// Get tool information
    pub fn get_tool_info(&self, name: &str) -> Result<Option<ShimInfo>> {
        Ok(self.manager.get_shim(name)?)
    }
}

fn main() -> Result<()> {
    // Example usage
    let vx_home = PathBuf::from("/home/user/.vx");
    let vx = VxShimIntegration::new(vx_home)?;

    // Install Node.js
    vx.install_tool(
        "node",
        "18.17.0",
        "/home/user/.vx/versions/node/18.17.0/bin/node",
    )?;

    // Install Python
    vx.install_tool(
        "python",
        "3.11.4",
        "/home/user/.vx/versions/python/3.11.4/bin/python",
    )?;

    // List tools
    vx.list_tools()?;

    // Switch Node.js version
    vx.switch_version(
        "node",
        "20.5.0",
        "/home/user/.vx/versions/node/20.5.0/bin/node",
    )?;

    // Execute a tool
    let exit_code = vx.execute_tool("node", &["--version".to_string()])?;
    println!("Node.js exited with code: {}", exit_code);

    // Check tool status
    if vx.is_tool_valid("python")? {
        println!("✅ Python is ready to use");
    }

    // Get detailed tool info
    if let Some(info) = vx.get_tool_info("node")? {
        println!(
            "📋 Node.js info: {} v{}",
            info.path,
            info.version.unwrap_or_default()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_vx_integration() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let vx = VxShimIntegration::new(temp_dir.path().to_path_buf())?;

        // Test tool installation
        let shim_path = vx.install_tool("test-tool", "1.0.0", "/usr/bin/echo")?;
        assert!(shim_path.exists());

        // Test listing
        let shims = vx.manager.list_shims()?;
        assert_eq!(shims.len(), 1);
        assert_eq!(shims[0].name, "test-tool");
        assert_eq!(shims[0].version, Some("1.0.0".to_string()));

        // Test tool info
        let info = vx.get_tool_info("test-tool")?;
        assert!(info.is_some());
        assert_eq!(info.unwrap().name, "test-tool");

        // Test validation
        assert!(vx.is_tool_valid("test-tool")?);

        // Test removal
        vx.remove_tool("test-tool")?;
        let shims_after = vx.manager.list_shims()?;
        assert_eq!(shims_after.len(), 0);

        Ok(())
    }

    #[test]
    fn test_version_switching() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let vx = VxShimIntegration::new(temp_dir.path().to_path_buf())?;

        // Install initial version
        vx.install_tool("node", "18.17.0", "/usr/bin/echo")?;

        // Switch version
        vx.switch_version("node", "20.5.0", "/usr/bin/echo")?;

        // Verify version was updated
        let info = vx.get_tool_info("node")?.unwrap();
        assert_eq!(info.version, Some("20.5.0".to_string()));

        Ok(())
    }
}

/// Compare with the old approach
#[allow(dead_code)]
mod old_approach_comparison {
    use super::*;

    // Old approach (complex, error-prone)
    fn old_create_shim(name: &str, path: &str, version: &str) -> Result<()> {
        use std::collections::HashMap;

        // Manual config creation
        let config = ShimConfig {
            shim: ShimCore {
                name: name.to_string(),
                path: path.to_string(),
                args: Vec::new(),
                cwd: None,
                download_url: None,
                source_type: SourceType::File,
                extracted_executables: Vec::new(),
            },
            args: Default::default(),
            env: HashMap::new(),
            metadata: ShimMetadata {
                description: Some(format!("{} managed by vx", name)),
                version: Some(version.to_string()),
                author: Some("vx".to_string()),
                tags: vec!["vx-managed".to_string(), name.to_string()],
            },
            auto_update: None,
        };

        // Manual file operations
        let config_path = format!("{}.shim.toml", name);
        config.to_file(&config_path)?;

        // Manual binary copying
        // ... complex shimexe binary copying logic ...

        Ok(())
    }

    // New approach (simple, clean)
    fn new_create_shim(
        manager: &ShimManager,
        name: &str,
        path: &str,
        version: &str,
    ) -> Result<PathBuf> {
        Ok(manager.create_shim_with_builder(name, |builder| {
            builder
                .path(path)
                .version(version)
                .description(format!("{} managed by vx", name))
                .tag("vx-managed")
                .tag(name)
        })?)
    }
}
