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

        // Create configuration based on source type
        let config = self
            .create_shim_config(&shim_name, &actual_path, env_vars, &shim_dir)
            .await?;

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

    /// Resolve the shim name and actual path, handling HTTP URLs, archives, and name inference
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

            // Determine base directory for downloads
            let base_dir = if let Some(ref dir) = shim_dir {
                dir.clone()
            } else {
                dirs::home_dir()
                    .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
                    .join(".shimexe")
            };

            // Check if this is an archive URL
            if shimexe_core::ArchiveExtractor::is_archive_url(&self.path) {
                // Handle archive download and extraction
                let mut downloader = Downloader::new().await?;
                let executables = downloader
                    .download_and_extract_archive(&self.path, &base_dir, &shim_name)
                    .await
                    .with_context(|| {
                        format!("Failed to download and extract archive from {}", self.path)
                    })?;

                if executables.is_empty() {
                    return Err(anyhow::anyhow!(
                        "No executables found in archive: {}",
                        self.path
                    ));
                }

                // Use the first executable as the primary one
                let primary_executable = &executables[0];
                println!(
                    "Downloaded and extracted archive from {}, found {} executables",
                    self.path,
                    executables.len()
                );
                debug!(
                    "Extracted {} executables from archive, using {} as primary",
                    executables.len(),
                    primary_executable.display()
                );

                Ok((shim_name, primary_executable.to_string_lossy().to_string()))
            } else {
                // Handle regular file download
                let filename =
                    Downloader::extract_filename_from_url(&self.path).ok_or_else(|| {
                        anyhow::anyhow!("Could not extract filename from URL: {}", self.path)
                    })?;

                let download_path =
                    Downloader::generate_download_path(&base_dir, &shim_name, &filename);

                let mut downloader = Downloader::new().await?;
                let downloaded = downloader
                    .download_if_missing(&self.path, &download_path)
                    .await
                    .with_context(|| format!("Failed to download file from {}", self.path))?;

                if downloaded {
                    println!("Downloaded {} to {}", self.path, download_path.display());
                    debug!("Downloaded {} to {}", self.path, download_path.display());
                } else {
                    debug!("File already exists at {}", download_path.display());
                }

                Ok((shim_name, download_path.to_string_lossy().to_string()))
            }
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

    /// Create shim configuration based on source type
    async fn create_shim_config(
        &self,
        shim_name: &str,
        actual_path: &str,
        env_vars: HashMap<String, String>,
        shim_dir: &Option<PathBuf>,
    ) -> Result<ShimConfig> {
        let is_url = Downloader::is_url(&self.path);
        let is_archive_url = is_url && shimexe_core::ArchiveExtractor::is_archive_url(&self.path);

        let (source_type, download_url, extracted_executables) = if is_archive_url {
            // Handle archive
            let base_dir = if let Some(ref dir) = shim_dir {
                dir.clone()
            } else {
                dirs::home_dir()
                    .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
                    .join(".shimexe")
            };

            let mut downloader = Downloader::new().await?;
            let executables = downloader
                .download_and_extract_archive(&self.path, &base_dir, shim_name)
                .await
                .with_context(|| {
                    format!("Failed to download and extract archive from {}", self.path)
                })?;

            let extracted_executables = executables
                .into_iter()
                .enumerate()
                .map(|(i, path)| {
                    let name = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    shimexe_core::ExtractedExecutable {
                        name,
                        path: path
                            .strip_prefix(base_dir.join(shim_name).join("bin"))
                            .unwrap_or(&path)
                            .to_string_lossy()
                            .to_string(),
                        full_path: path.to_string_lossy().to_string(),
                        is_primary: i == 0, // First executable is primary
                    }
                })
                .collect();

            (
                shimexe_core::SourceType::Archive,
                Some(self.path.clone()),
                extracted_executables,
            )
        } else if is_url {
            // Handle regular URL
            (
                shimexe_core::SourceType::Url,
                Some(self.path.clone()),
                vec![],
            )
        } else {
            // Handle local file
            (shimexe_core::SourceType::File, None, vec![])
        };

        Ok(ShimConfig {
            shim: ShimCore {
                name: shim_name.to_string(),
                path: actual_path.to_string(),
                args: self.args.clone(),
                cwd: self.cwd.clone(),
                download_url,
                source_type,
                extracted_executables,
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
        })
    }
}
