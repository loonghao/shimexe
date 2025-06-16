use anyhow::Result;
use clap::Args;
use std::path::PathBuf;

use crate::shim_manager::ShimManager;
use shimexe_core::ShimRunner;

#[derive(Args)]
pub struct RunCommand {
    /// Name of the shim to run (or path to shim file)
    pub shim_name_or_path: String,

    /// Arguments to pass to the target executable
    #[arg(trailing_var_arg = true)]
    pub args: Vec<String>,
}

impl RunCommand {
    pub fn execute(&self, shim_dir: Option<PathBuf>) -> Result<()> {
        let runner = if self.shim_name_or_path.contains('/')
            || self.shim_name_or_path.contains('\\')
            || self.shim_name_or_path.ends_with(".toml")
        {
            // Treat as file path
            let path = PathBuf::from(&self.shim_name_or_path);
            ShimRunner::from_file(&path)?
        } else {
            // Treat as shim name
            let manager = ShimManager::new(shim_dir)?;
            let config = manager
                .get_shim_config(&self.shim_name_or_path)
                .map_err(|_| {
                    anyhow::anyhow!(
                        "Shim '{}' not found. Use 'shimexe list' to see available shims.",
                        self.shim_name_or_path
                    )
                })?;
            ShimRunner::from_config(config)?
        };

        let exit_code = runner.execute(&self.args)?;
        std::process::exit(exit_code);
    }
}
