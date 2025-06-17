use anyhow::{Context, Result};
use clap::Args;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{debug, info};

use crate::shim_manager::ShimManager;
use shimexe_core::{Downloader, ShimConfig, ShimCore, ShimMetadata};

#[derive(Args)]
pub struct AddCommand {
    /// Name of the shim (optional, will be inferred from path/URL if not provided)
    pub name: Option<String>,

    /// Path to the target executable or HTTP URL to download
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
    pub async fn execute(&self, global_shim_dir: Option<PathBuf>) -> Result<()> {
        // Use command-specific shim_dir if provided, otherwise use global setting
        let shim_dir = self.shim_dir.clone().or(global_shim_dir);
        let manager = ShimManager::new(shim_dir.clone())?;

        // Determine the actual name and path
        let (shim_name, actual_path) = self.resolve_name_and_path(&shim_dir).await?;

        // Check if shim already exists
        if manager.shim_exists(&shim_name) && !self.force {
            return Err(anyhow::anyhow!(
                "Shim '{}' already exists. Use --force to overwrite.",
                shim_name
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
        let download_url = if Downloader::is_url(&self.path) {
            Some(self.path.clone())
        } else {
            None
        };

        let config = ShimConfig {
            shim: ShimCore {
                name: shim_name.clone(),
                path: actual_path.clone(),
                args: self.args.clone(),
                cwd: self.cwd.clone(),
                download_url,
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
        manager.add_shim(&shim_name, &config)?;

        info!("Successfully added shim '{}'", shim_name);
        println!("✓ Added shim '{}' -> {}", shim_name, actual_path);

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

    /// Resolve the shim name and actual path, handling HTTP URLs and name inference
    async fn resolve_name_and_path(&self, shim_dir: &Option<PathBuf>) -> Result<(String, String)> {
        let is_url = Downloader::is_url(&self.path);

        if is_url {
            // Handle HTTP URL
            let shim_name = if let Some(ref name) = self.name {
                name.clone()
            } else {
                // Infer name from URL
                Downloader::infer_app_name_from_url(&self.path).ok_or_else(|| {
                    anyhow::anyhow!("Could not infer application name from URL: {}", self.path)
                })?
            };

            // Extract filename from URL
            let filename = Downloader::extract_filename_from_url(&self.path).ok_or_else(|| {
                anyhow::anyhow!("Could not extract filename from URL: {}", self.path)
            })?;

            // Determine base directory for downloads
            let base_dir = if let Some(ref dir) = shim_dir {
                dir.clone()
            } else {
                dirs::home_dir()
                    .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
                    .join(".shimexe")
            };

            // Generate download path
            let download_path =
                Downloader::generate_download_path(&base_dir, &shim_name, &filename);

            // Download the file if it doesn't exist
            let downloader = Downloader::new();
            let downloaded = downloader
                .download_if_missing(&self.path, &download_path)
                .await
                .with_context(|| format!("Failed to download file from {}", self.path))?;

            if downloaded {
                info!("Downloaded {} to {}", self.path, download_path.display());
            } else {
                debug!("File already exists at {}", download_path.display());
            }

            Ok((shim_name, download_path.to_string_lossy().to_string()))
        } else {
            // Handle local path
            let shim_name = if let Some(ref name) = self.name {
                name.clone()
            } else {
                // Infer name from local path
                PathBuf::from(&self.path)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
                    .ok_or_else(|| {
                        anyhow::anyhow!("Could not infer application name from path: {}", self.path)
                    })?
            };

            Ok((shim_name, self.path.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_resolve_name_and_path_local() {
        let cmd = AddCommand {
            name: Some("test-app".to_string()),
            path: "/usr/bin/test".to_string(),
            args: vec![],
            cwd: None,
            env: vec![],
            description: None,
            force: false,
            shim_dir: None,
            add_system_path: false,
        };

        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(cmd.resolve_name_and_path(&None)).unwrap();

        assert_eq!(result.0, "test-app");
        assert_eq!(result.1, "/usr/bin/test");
    }

    #[test]
    fn test_resolve_name_and_path_local_infer_name() {
        let cmd = AddCommand {
            name: None,
            path: "/usr/bin/my-tool".to_string(),
            args: vec![],
            cwd: None,
            env: vec![],
            description: None,
            force: false,
            shim_dir: None,
            add_system_path: false,
        };

        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(cmd.resolve_name_and_path(&None)).unwrap();

        assert_eq!(result.0, "my-tool");
        assert_eq!(result.1, "/usr/bin/my-tool");
    }

    #[test]
    fn test_url_detection_and_parsing() {
        let cmd = AddCommand {
            name: Some("custom-name".to_string()),
            path: "https://example.com/tool.exe".to_string(),
            args: vec![],
            cwd: None,
            env: vec![],
            description: None,
            force: false,
            shim_dir: None,
            add_system_path: false,
        };

        // Test URL detection and parsing logic
        assert!(shimexe_core::Downloader::is_url(&cmd.path));
        assert_eq!(
            shimexe_core::Downloader::extract_filename_from_url(&cmd.path),
            Some("tool.exe".to_string())
        );
        assert_eq!(
            shimexe_core::Downloader::infer_app_name_from_url(&cmd.path),
            Some("tool".to_string())
        );
    }

    #[test]
    fn test_env_parsing() {
        let cmd = AddCommand {
            name: Some("test".to_string()),
            path: "/usr/bin/test".to_string(),
            args: vec![],
            cwd: None,
            env: vec![
                "KEY1=value1".to_string(),
                "KEY2=value2".to_string(),
                "PATH=/usr/bin:/bin".to_string(),
            ],
            description: None,
            force: false,
            shim_dir: None,
            add_system_path: false,
        };

        // Test environment variable parsing logic
        let mut env_vars = HashMap::new();
        for env_var in &cmd.env {
            if let Some((key, value)) = env_var.split_once('=') {
                env_vars.insert(key.to_string(), value.to_string());
            }
        }

        assert_eq!(env_vars.get("KEY1"), Some(&"value1".to_string()));
        assert_eq!(env_vars.get("KEY2"), Some(&"value2".to_string()));
        assert_eq!(env_vars.get("PATH"), Some(&"/usr/bin:/bin".to_string()));
    }
}
