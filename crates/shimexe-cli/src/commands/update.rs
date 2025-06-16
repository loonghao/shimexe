use anyhow::Result;
use clap::Args;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::info;

use crate::shim_manager::ShimManager;

#[derive(Args)]
pub struct UpdateCommand {
    /// Name of the shim to update
    pub name: String,
    
    /// New path to the target executable
    #[arg(short, long)]
    pub path: Option<String>,
    
    /// New default arguments
    #[arg(short, long, value_delimiter = ' ')]
    pub args: Option<Vec<String>>,
    
    /// New working directory
    #[arg(short, long)]
    pub cwd: Option<String>,
    
    /// New environment variables (format: KEY=VALUE)
    #[arg(short, long, value_delimiter = ',')]
    pub env: Vec<String>,
    
    /// New description
    #[arg(short, long)]
    pub description: Option<String>,
}

impl UpdateCommand {
    pub fn execute(&self, shim_dir: Option<PathBuf>) -> Result<()> {
        let manager = ShimManager::new(shim_dir)?;
        
        if !manager.shim_exists(&self.name) {
            return Err(anyhow::anyhow!("Shim '{}' does not exist", self.name));
        }
        
        let mut config = manager.get_shim_config(&self.name)?;
        
        // Update fields if provided
        if let Some(ref path) = self.path {
            config.shim.path = path.clone();
        }
        
        if let Some(ref args) = self.args {
            config.shim.args = args.clone();
        }
        
        if let Some(ref cwd) = self.cwd {
            config.shim.cwd = Some(cwd.clone());
        }
        
        if let Some(ref description) = self.description {
            config.metadata.description = Some(description.clone());
        }
        
        // Parse and update environment variables
        if !self.env.is_empty() {
            let mut env_vars = HashMap::new();
            for env_var in &self.env {
                if let Some((key, value)) = env_var.split_once('=') {
                    env_vars.insert(key.to_string(), value.to_string());
                } else {
                    return Err(anyhow::anyhow!("Invalid environment variable format: {}", env_var));
                }
            }
            config.env = env_vars;
        }
        
        // Validate and update
        config.validate()?;
        manager.update_shim(&self.name, &config)?;
        
        info!("Successfully updated shim '{}'", self.name);
        println!("✓ Updated shim '{}'", self.name);
        
        Ok(())
    }
}
