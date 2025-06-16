use anyhow::Result;
use clap::Args;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::info;

use crate::shim_manager::ShimManager;
use shimexe_core::{ShimConfig, ShimCore, ShimMetadata};

#[derive(Args)]
pub struct AddCommand {
    /// Name of the shim
    pub name: String,

    /// Path to the target executable
    #[arg(short, long)]
    pub path: String,

    /// Default arguments to pass to the executable
    #[arg(short, long, value_delimiter = ' ')]
    pub args: Vec<String>,

    /// Working directory for the executable
    #[arg(short, long)]
    pub cwd: Option<String>,

    /// Environment variables (format: KEY=VALUE)
    #[arg(short, long, value_delimiter = ',')]
    pub env: Vec<String>,

    /// Description of the shim
    #[arg(short, long)]
    pub description: Option<String>,

    /// Force overwrite if shim already exists
    #[arg(short, long)]
    pub force: bool,

    /// Custom shim directory (overrides default ~/.shimexe)
    #[arg(long)]
    pub shim_dir: Option<PathBuf>,

    /// Add the shim directory to system PATH
    #[arg(long)]
    pub add_system_path: bool,
}

impl AddCommand {
    pub fn execute(&self, global_shim_dir: Option<PathBuf>) -> Result<()> {
        // Use command-specific shim_dir if provided, otherwise use global setting
        let shim_dir = self.shim_dir.clone().or(global_shim_dir);
        let manager = ShimManager::new(shim_dir)?;

        // Check if shim already exists
        if manager.shim_exists(&self.name) && !self.force {
            return Err(anyhow::anyhow!(
                "Shim '{}' already exists. Use --force to overwrite.",
                self.name
            ));
        }

        // Parse environment variables
        let mut env_vars = HashMap::new();
        for env_var in &self.env {
            if let Some((key, value)) = env_var.split_once('=') {
                env_vars.insert(key.to_string(), value.to_string());
            } else {
                return Err(anyhow::anyhow!(
                    "Invalid environment variable format: {}",
                    env_var
                ));
            }
        }

        // Create shim configuration
        let config = ShimConfig {
            shim: ShimCore {
                name: self.name.clone(),
                path: self.path.clone(),
                args: self.args.clone(),
                cwd: self.cwd.clone(),
            },
            args: Default::default(),
            env: env_vars,
            metadata: ShimMetadata {
                description: self.description.clone(),
                version: Some("1.0.0".to_string()),
                author: None,
                tags: vec![],
            },
            auto_update: None,
        };

        // Validate configuration
        config.validate()?;

        // Add the shim
        manager.add_shim(&self.name, &config)?;

        info!("Successfully added shim '{}'", self.name);
        println!("✓ Added shim '{}' -> {}", self.name, self.path);

        // Add to system PATH if requested
        if self.add_system_path {
            if let Some(ref custom_dir) = self.shim_dir {
                manager.add_directory_to_system_path(custom_dir)?;
            } else {
                manager.add_to_system_path()?;
            }
        }

        Ok(())
    }
}
