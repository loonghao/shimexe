//! High-level shim management API for tool managers like vx, rye, etc.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::config::{ShimConfig, ShimCore, ShimMetadata, SourceType};
use crate::downloader::Downloader;
use crate::error::{Result, ShimError};
use crate::runner::ShimRunner;

/// High-level shim manager for tool managers
#[derive(Debug, Clone)]
pub struct ShimManager {
    /// Directory where shims are stored
    pub shim_dir: PathBuf,
    /// Optional metadata directory for tool manager specific data
    pub metadata_dir: Option<PathBuf>,
}

/// Shim information for listing and management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShimInfo {
    pub name: String,
    pub path: String,
    pub source_type: SourceType,
    pub download_url: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub is_valid: bool,
}

/// Builder for creating shims with a fluent API
#[derive(Debug, Clone)]
pub struct ShimBuilder {
    name: String,
    path: Option<String>,
    args: Vec<String>,
    env: HashMap<String, String>,
    cwd: Option<String>,
    download_url: Option<String>,
    source_type: SourceType,
    metadata: ShimMetadata,
}

impl ShimBuilder {
    /// Create a new shim builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            path: None,
            args: Vec::new(),
            env: HashMap::new(),
            cwd: None,
            download_url: None,
            source_type: SourceType::File,
            metadata: ShimMetadata::default(),
        }
    }

    /// Set the executable path
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Add arguments
    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    /// Add a single argument
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    /// Set working directory
    pub fn cwd(mut self, cwd: impl Into<String>) -> Self {
        self.cwd = Some(cwd.into());
        self
    }

    /// Add environment variable
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env.insert(key.into(), value.into());
        self
    }

    /// Set download URL for HTTP-based shims
    pub fn download_url(mut self, url: impl Into<String>) -> Self {
        let url = url.into();
        self.download_url = Some(url.clone());
        
        // Auto-detect source type based on URL
        if url.ends_with(".zip") || url.ends_with(".tar.gz") || url.ends_with(".tgz") {
            self.source_type = SourceType::Archive;
        } else if Downloader::is_url(&url) {
            self.source_type = SourceType::Url;
        }
        
        self
    }

    /// Set source type explicitly
    pub fn source_type(mut self, source_type: SourceType) -> Self {
        self.source_type = source_type;
        self
    }

    /// Set description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.metadata.description = Some(description.into());
        self
    }

    /// Set version
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.metadata.version = Some(version.into());
        self
    }

    /// Set author
    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.metadata.author = Some(author.into());
        self
    }

    /// Add tags
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.metadata.tags = tags;
        self
    }

    /// Add a single tag
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.metadata.tags.push(tag.into());
        self
    }

    /// Build the shim configuration
    pub fn build(self) -> Result<ShimConfig> {
        let path = self.path.ok_or_else(|| {
            ShimError::Config("Shim path is required".to_string())
        })?;

        let config = ShimConfig {
            shim: ShimCore {
                name: self.name,
                path,
                args: self.args,
                cwd: self.cwd,
                download_url: self.download_url,
                source_type: self.source_type,
                extracted_executables: Vec::new(),
            },
            args: Default::default(),
            env: self.env,
            metadata: self.metadata,
            auto_update: None,
        };

        config.validate()?;
        Ok(config)
    }
}

impl ShimManager {
    /// Create a new shim manager
    pub fn new(shim_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&shim_dir)?;
        
        Ok(Self {
            shim_dir,
            metadata_dir: None,
        })
    }

    /// Create a new shim manager with custom metadata directory
    pub fn with_metadata_dir(shim_dir: PathBuf, metadata_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&shim_dir)?;
        fs::create_dir_all(&metadata_dir)?;
        
        Ok(Self {
            shim_dir,
            metadata_dir: Some(metadata_dir),
        })
    }

    /// Create a new shim builder
    pub fn builder(&self, name: impl Into<String>) -> ShimBuilder {
        ShimBuilder::new(name)
    }

    /// Create a shim from configuration
    pub fn create_shim(&self, config: ShimConfig) -> Result<PathBuf> {
        let config_path = self.shim_dir.join(format!("{}.shim.toml", config.shim.name));
        let shim_path = self.get_shim_executable_path(&config.shim.name);

        // Save configuration
        config.to_file(&config_path)?;

        // Copy shimexe binary as the shim executable
        self.copy_shimexe_binary(&shim_path)?;

        info!("Created shim '{}' at {}", config.shim.name, shim_path.display());
        Ok(shim_path)
    }

    /// Create a shim using the builder pattern
    pub fn create_shim_with_builder<F>(&self, name: impl Into<String>, builder_fn: F) -> Result<PathBuf>
    where
        F: FnOnce(ShimBuilder) -> ShimBuilder,
    {
        let builder = self.builder(name);
        let config = builder_fn(builder).build()?;
        self.create_shim(config)
    }

    /// Remove a shim
    pub fn remove_shim(&self, name: &str) -> Result<()> {
        let config_path = self.shim_dir.join(format!("{}.shim.toml", name));
        let shim_path = self.get_shim_executable_path(name);

        // Remove files
        if config_path.exists() {
            fs::remove_file(&config_path)?;
        }
        if shim_path.exists() {
            fs::remove_file(&shim_path)?;
        }

        // Remove metadata if exists
        if let Some(ref metadata_dir) = self.metadata_dir {
            let metadata_path = metadata_dir.join(format!("{}.json", name));
            if metadata_path.exists() {
                fs::remove_file(&metadata_path)?;
            }
        }

        info!("Removed shim '{}'", name);
        Ok(())
    }

    /// List all shims
    pub fn list_shims(&self) -> Result<Vec<ShimInfo>> {
        let mut shims = Vec::new();

        if !self.shim_dir.exists() {
            return Ok(shims);
        }

        for entry in fs::read_dir(&self.shim_dir)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(extension) = path.extension() {
                if extension == "toml" && path.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.ends_with(".shim"))
                    .unwrap_or(false)
                {
                    if let Ok(config) = ShimConfig::from_file(&path) {
                        let shim_path = self.get_shim_executable_path(&config.shim.name);
                        let is_valid = shim_path.exists();

                        shims.push(ShimInfo {
                            name: config.shim.name.clone(),
                            path: config.shim.path.clone(),
                            source_type: config.shim.source_type.clone(),
                            download_url: config.shim.download_url.clone(),
                            version: config.metadata.version.clone(),
                            description: config.metadata.description.clone(),
                            tags: config.metadata.tags.clone(),
                            is_valid,
                        });
                    }
                }
            }
        }

        shims.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(shims)
    }

    /// Get shim information by name
    pub fn get_shim(&self, name: &str) -> Result<Option<ShimInfo>> {
        let config_path = self.shim_dir.join(format!("{}.shim.toml", name));
        
        if !config_path.exists() {
            return Ok(None);
        }

        let config = ShimConfig::from_file(&config_path)?;
        let shim_path = self.get_shim_executable_path(name);
        let is_valid = shim_path.exists();

        Ok(Some(ShimInfo {
            name: config.shim.name.clone(),
            path: config.shim.path.clone(),
            source_type: config.shim.source_type.clone(),
            download_url: config.shim.download_url.clone(),
            version: config.metadata.version.clone(),
            description: config.metadata.description.clone(),
            tags: config.metadata.tags.clone(),
            is_valid,
        }))
    }

    /// Update an existing shim
    pub fn update_shim(&self, name: &str, config: ShimConfig) -> Result<PathBuf> {
        if self.get_shim(name)?.is_none() {
            return Err(ShimError::Config(format!("Shim '{}' does not exist", name)));
        }

        // Remove old shim and create new one
        self.remove_shim(name)?;
        self.create_shim(config)
    }

    /// Execute a shim
    pub fn execute_shim(&self, name: &str, args: &[String]) -> Result<i32> {
        let config_path = self.shim_dir.join(format!("{}.shim.toml", name));
        
        if !config_path.exists() {
            return Err(ShimError::Config(format!("Shim '{}' not found", name)));
        }

        let runner = ShimRunner::from_file(&config_path)?;
        runner.execute(args)
    }

    /// Validate a shim
    pub fn validate_shim(&self, name: &str) -> Result<bool> {
        let config_path = self.shim_dir.join(format!("{}.shim.toml", name));
        
        if !config_path.exists() {
            return Ok(false);
        }

        match ShimRunner::from_file(&config_path) {
            Ok(runner) => Ok(runner.validate().is_ok()),
            Err(_) => Ok(false),
        }
    }

    /// Get the path to the shim executable
    fn get_shim_executable_path(&self, name: &str) -> PathBuf {
        if cfg!(windows) {
            self.shim_dir.join(format!("{}.exe", name))
        } else {
            self.shim_dir.join(name)
        }
    }

    /// Copy shimexe binary to create the shim executable
    fn copy_shimexe_binary(&self, dest_path: &Path) -> Result<()> {
        // Try to find shimexe binary
        let shimexe_path = which::which("shimexe")
            .or_else(|_| std::env::current_exe())
            .map_err(|_| ShimError::Config("Could not find shimexe binary".to_string()))?;

        fs::copy(&shimexe_path, dest_path)?;

        // Make executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(dest_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(dest_path, perms)?;
        }

        Ok(())
    }
}
