use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use shimexe_core::{ShimConfig, ShimRunner};

#[derive(Args)]
pub struct ValidateCommand {
    /// Path to the shim file to validate
    pub shim_file: PathBuf,
}

impl ValidateCommand {
    pub fn execute(&self) -> Result<()> {
        println!("Validating shim file: {}", self.shim_file.display());

        // Load and validate configuration
        let config = ShimConfig::from_file(&self.shim_file)?;
        println!("✓ Configuration syntax is valid");

        // Create runner and validate executable
        let runner = ShimRunner::from_config(config)?;
        runner.validate()?;
        println!("✓ Target executable is valid and accessible");

        println!("✓ Shim configuration is valid");
        Ok(())
    }
}
