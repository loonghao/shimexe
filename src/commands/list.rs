use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use crate::shim_manager::ShimManager;

#[derive(Args)]
pub struct ListCommand {
    /// Show detailed information
    #[arg(short, long)]
    pub detailed: bool,

    /// Positional "detailed" keyword for backwards compatibility
    /// (e.g. `shimexe list detailed`)
    #[arg(value_name = "DETAILED", hide = true)]
    pub keyword: Option<String>,
}

impl ListCommand {
    pub fn execute(&self, shim_dir: Option<PathBuf>) -> Result<()> {
        let manager = ShimManager::new(shim_dir)?;
        let shims = manager.list_shims()?;

        let detailed = self.detailed || self.keyword.as_deref() == Some("detailed");

        if shims.is_empty() {
            println!("No shims found.");
            return Ok(());
        }

        if detailed {
            for (name, config) in shims {
                println!("📦 {}", name);
                println!("   Path: {}", config.shim.path);
                if !config.shim.args.is_empty() {
                    println!("   Args: {}", config.shim.args.join(" "));
                }
                if let Some(ref cwd) = config.shim.cwd {
                    println!("   CWD:  {}", cwd);
                }
                if !config.env.is_empty() {
                    println!("   Env:  {} variables", config.env.len());
                }
                if let Some(ref desc) = config.metadata.description {
                    println!("   Desc: {}", desc);
                }
                println!();
            }
        } else {
            println!("Available shims:");
            for (name, config) in shims {
                println!("  {} -> {}", name, config.shim.path);
            }
        }

        Ok(())
    }
}
