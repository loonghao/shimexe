use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use tracing::info;

use shimexe_core::ShimUpdater;
use crate::shim_manager::ShimManager;

#[derive(Args)]
pub struct UpdateCheckCommand {
    /// Name of the shim to check for updates
    pub name: Option<String>,
    
    /// Check all shims for updates
    #[arg(short, long)]
    pub all: bool,
    
    /// Force update check even if interval hasn't passed
    #[arg(short, long)]
    pub force: bool,
    
    /// Actually perform the update if available
    #[arg(short, long)]
    pub install: bool,
}

impl UpdateCheckCommand {
    pub fn execute(&self, shim_dir: Option<PathBuf>) -> Result<()> {
        let manager = ShimManager::new(shim_dir)?;
        
        if self.all {
            self.check_all_shims(&manager)?;
        } else if let Some(ref name) = self.name {
            self.check_single_shim(&manager, name)?;
        } else {
            return Err(anyhow::anyhow!("Must specify either --all or a shim name"));
        }
        
        Ok(())
    }
    
    fn check_all_shims(&self, manager: &ShimManager) -> Result<()> {
        let shims = manager.list_shims()?;
        
        if shims.is_empty() {
            println!("No shims found.");
            return Ok(());
        }
        
        println!("Checking {} shims for updates...", shims.len());
        
        for (name, config) in shims {
            if config.auto_update.is_some() {
                println!("\n📦 Checking {}", name);
                if let Err(e) = self.check_shim_update(&name, &config, manager) {
                    println!("  ❌ Error: {}", e);
                } else {
                    println!("  ✅ Check completed");
                }
            } else {
                println!("📦 {} - Auto-update not configured", name);
            }
        }
        
        Ok(())
    }
    
    fn check_single_shim(&self, manager: &ShimManager, name: &str) -> Result<()> {
        if !manager.shim_exists(name) {
            return Err(anyhow::anyhow!("Shim '{}' does not exist", name));
        }
        
        let config = manager.get_shim_config(name)?;
        
        if config.auto_update.is_none() {
            println!("Shim '{}' does not have auto-update configured", name);
            return Ok(());
        }
        
        println!("Checking '{}' for updates...", name);
        self.check_shim_update(name, &config, manager)?;
        
        Ok(())
    }
    
    fn check_shim_update(&self, name: &str, config: &shimexe_core::ShimConfig, manager: &ShimManager) -> Result<()> {
        let auto_update = config.auto_update.as_ref().unwrap();
        
        let shim_file = manager.shim_dir().join(format!("{}.shim.toml", name));
        let executable_path = config.get_executable_path()?;
        
        let updater = ShimUpdater::new(
            auto_update.clone(),
            shim_file,
            executable_path,
        );
        
        // Use async runtime
        let rt = tokio::runtime::Runtime::new()?;
        
        rt.block_on(async {
            match updater.check_update_needed().await {
                Ok(Some(version)) => {
                    println!("  🔄 Update available: {}", version);
                    
                    if self.install {
                        println!("  📥 Installing update...");
                        match updater.update_to_version(&version).await {
                            Ok(()) => {
                                println!("  ✅ Update installed successfully");
                                info!("Updated '{}' to version {}", name, version);
                            }
                            Err(e) => {
                                println!("  ❌ Update failed: {}", e);
                            }
                        }
                    } else {
                        println!("  💡 Use --install to apply the update");
                    }
                }
                Ok(None) => {
                    println!("  ✅ No update needed");
                }
                Err(e) => {
                    println!("  ❌ Update check failed: {}", e);
                }
            }
        });
        
        Ok(())
    }
}
