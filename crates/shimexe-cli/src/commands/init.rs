use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use tracing::info;

use crate::shim_manager::ShimManager;

#[derive(Args)]
pub struct InitCommand {
    /// Initialize with example shims
    #[arg(short, long)]
    pub examples: bool,
}

impl InitCommand {
    pub fn execute(&self, shim_dir: Option<PathBuf>) -> Result<()> {
        let manager = ShimManager::new(shim_dir)?;

        println!("Initialized shimexe configuration");
        println!("Shim directory: {}", manager.shim_dir().display());

        if self.examples {
            self.create_example_shims(&manager)?;
        }

        println!("\nTo add your first shim, run:");
        println!("  shimexe add <name> --path <executable-path>");

        Ok(())
    }

    fn create_example_shims(&self, manager: &ShimManager) -> Result<()> {
        use shimexe_core::{ShimConfig, ShimCore, ShimMetadata};
        use std::collections::HashMap;

        // Example 1: Simple echo shim
        let echo_config = ShimConfig {
            shim: ShimCore {
                name: "hello".to_string(),
                path: "echo".to_string(),
                args: vec!["Hello from shimexe!".to_string()],
                cwd: None,
            },
            args: Default::default(),
            env: HashMap::new(),
            metadata: ShimMetadata {
                description: Some("Example echo shim".to_string()),
                version: Some("1.0.0".to_string()),
                author: Some("shimexe".to_string()),
                tags: vec!["example".to_string()],
            },
            auto_update: None,
        };

        manager.add_shim("hello", &echo_config)?;
        info!("Created example shim: hello");

        // Example 2: Environment variable expansion
        let mut env_vars = HashMap::new();
        env_vars.insert("GREETING".to_string(), "Hello".to_string());

        let env_config = ShimConfig {
            shim: ShimCore {
                name: "greet".to_string(),
                path: "echo".to_string(),
                args: vec!["${GREETING:Hi} from shimexe!".to_string()],
                cwd: None,
            },
            args: Default::default(),
            env: env_vars,
            metadata: ShimMetadata {
                description: Some("Example shim with environment variables".to_string()),
                version: Some("1.0.0".to_string()),
                author: Some("shimexe".to_string()),
                tags: vec!["example", "env"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            },
            auto_update: None,
        };

        manager.add_shim("greet", &env_config)?;
        info!("Created example shim: greet");

        println!("✓ Created example shims: hello, greet");
        println!("  Try running: hello");
        println!("  Try running: greet");

        Ok(())
    }
}
