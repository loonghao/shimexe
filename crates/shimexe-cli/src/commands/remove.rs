use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use tracing::info;

use crate::shim_manager::ShimManager;

#[derive(Args)]
pub struct RemoveCommand {
    /// Name of the shim to remove
    pub name: String,

    /// Force removal without confirmation
    #[arg(short, long)]
    pub force: bool,
}

impl RemoveCommand {
    pub fn execute(&self, shim_dir: Option<PathBuf>) -> Result<()> {
        let manager = ShimManager::new(shim_dir)?;

        if !manager.shim_exists(&self.name) {
            return Err(anyhow::anyhow!("Shim '{}' does not exist", self.name));
        }

        if !self.force {
            print!(
                "Are you sure you want to remove shim '{}'? (y/N): ",
                self.name
            );
            use std::io::{self, Write};
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if !input.trim().to_lowercase().starts_with('y') {
                println!("Cancelled.");
                return Ok(());
            }
        }

        manager.remove_shim(&self.name)?;

        info!("Successfully removed shim '{}'", self.name);
        println!("✓ Removed shim '{}'", self.name);

        Ok(())
    }
}
