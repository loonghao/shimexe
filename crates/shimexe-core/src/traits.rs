use std::path::{Path, PathBuf};
use async_trait::async_trait;

use crate::config::ShimConfig;
use crate::error::Result;

/// Trait for custom shim configuration loaders
pub trait ShimConfigLoader: Send + Sync {
    /// Load shim configuration from a file
    fn load_config(&self, path: &Path) -> Result<ShimConfig>;

    /// Save shim configuration to a file
    fn save_config(&self, config: &ShimConfig, path: &Path) -> Result<()>;

    /// Get the default file extension for this loader
    fn file_extension(&self) -> &str;

    /// Validate the configuration
    fn validate_config(&self, config: &ShimConfig) -> Result<()> {
        config.validate()
    }
}

/// Trait for custom shim runners
#[async_trait]
pub trait ShimRunnerTrait {
    /// Execute the shim with additional arguments
    async fn execute(&self, additional_args: &[String]) -> Result<i32>;
    
    /// Get the shim configuration
    fn config(&self) -> &ShimConfig;
    
    /// Validate that the target executable exists and is executable
    fn validate(&self) -> Result<()>;
    
    /// Pre-execution hook (called before running the target executable)
    async fn pre_execute(&self, _args: &[String]) -> Result<()> {
        Ok(())
    }
    
    /// Post-execution hook (called after running the target executable)
    async fn post_execute(&self, _exit_code: i32) -> Result<()> {
        Ok(())
    }
}

/// Trait for custom update providers
#[async_trait]
pub trait UpdateProvider: Send + Sync {
    /// Check if an update is available
    async fn check_update_available(&self, current_version: &str) -> Result<Option<String>>;
    
    /// Download and install the update
    async fn install_update(&self, version: &str, target_path: &Path) -> Result<()>;
    
    /// Get the download URL for a specific version
    fn get_download_url(&self, version: &str) -> Result<String>;
    
    /// Verify the downloaded file (checksum, signature, etc.)
    async fn verify_download(&self, _file_path: &Path, _version: &str) -> Result<bool> {
        // Default implementation: always return true
        Ok(true)
    }
}

/// Trait for custom version checkers
#[async_trait]
pub trait VersionChecker: Send + Sync {
    /// Get the latest available version
    async fn get_latest_version(&self) -> Result<String>;
    
    /// Compare two versions (returns true if first is newer than second)
    fn is_newer_version(&self, version1: &str, version2: &str) -> Result<bool>;
    
    /// Parse version from string
    fn parse_version(&self, version_str: &str) -> Result<String>;
}

/// Builder for creating customizable shim runners
pub struct ShimRunnerBuilder {
    config_loader: Option<Box<dyn ShimConfigLoader>>,
    update_provider: Option<Box<dyn UpdateProvider>>,
    version_checker: Option<Box<dyn VersionChecker>>,
    config_file_pattern: Option<String>,
    pre_execute_hooks: Vec<Box<dyn Fn(&[String]) -> Result<()> + Send + Sync>>,
    post_execute_hooks: Vec<Box<dyn Fn(i32) -> Result<()> + Send + Sync>>,
}

impl ShimRunnerBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config_loader: None,
            update_provider: None,
            version_checker: None,
            config_file_pattern: None,
            pre_execute_hooks: Vec::new(),
            post_execute_hooks: Vec::new(),
        }
    }
    
    /// Set a custom configuration loader
    pub fn with_config_loader(mut self, loader: Box<dyn ShimConfigLoader>) -> Self {
        self.config_loader = Some(loader);
        self
    }
    
    /// Set a custom update provider
    pub fn with_update_provider(mut self, provider: Box<dyn UpdateProvider>) -> Self {
        self.update_provider = Some(provider);
        self
    }
    
    /// Set a custom version checker
    pub fn with_version_checker(mut self, checker: Box<dyn VersionChecker>) -> Self {
        self.version_checker = Some(checker);
        self
    }
    
    /// Set a custom config file pattern (e.g., "{name}.custom.toml")
    pub fn with_config_file_pattern<S: Into<String>>(mut self, pattern: S) -> Self {
        self.config_file_pattern = Some(pattern.into());
        self
    }
    
    /// Add a pre-execute hook
    pub fn with_pre_execute_hook<F>(mut self, hook: F) -> Self 
    where
        F: Fn(&[String]) -> Result<()> + Send + Sync + 'static,
    {
        self.pre_execute_hooks.push(Box::new(hook));
        self
    }
    
    /// Add a post-execute hook
    pub fn with_post_execute_hook<F>(mut self, hook: F) -> Self 
    where
        F: Fn(i32) -> Result<()> + Send + Sync + 'static,
    {
        self.post_execute_hooks.push(Box::new(hook));
        self
    }
    
    /// Build a customizable shim runner
    pub fn build(self, shim_name: &str, shim_dir: &Path) -> Result<CustomizableShimRunner> {
        let config_file_pattern = self.config_file_pattern
            .unwrap_or_else(|| "{name}.shim.toml".to_string());
        
        let config_file = shim_dir.join(
            config_file_pattern.replace("{name}", shim_name)
        );
        
        let config_loader = self.config_loader
            .unwrap_or_else(|| Box::new(DefaultConfigLoader));
        
        let config = config_loader.load_config(&config_file)?;
        
        Ok(CustomizableShimRunner {
            config,
            config_file_path: config_file,
            config_loader,
            update_provider: self.update_provider,
            version_checker: self.version_checker,
            pre_execute_hooks: self.pre_execute_hooks,
            post_execute_hooks: self.post_execute_hooks,
        })
    }
}

impl Default for ShimRunnerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Customizable shim runner that supports custom loaders and providers
pub struct CustomizableShimRunner {
    config: ShimConfig,
    config_file_path: PathBuf,
    config_loader: Box<dyn ShimConfigLoader>,
    update_provider: Option<Box<dyn UpdateProvider>>,
    version_checker: Option<Box<dyn VersionChecker>>,
    pre_execute_hooks: Vec<Box<dyn Fn(&[String]) -> Result<()> + Send + Sync>>,
    post_execute_hooks: Vec<Box<dyn Fn(i32) -> Result<()> + Send + Sync>>,
}

#[async_trait]
impl ShimRunnerTrait for CustomizableShimRunner {
    async fn execute(&self, additional_args: &[String]) -> Result<i32> {
        // Run pre-execute hooks
        for hook in &self.pre_execute_hooks {
            hook(additional_args)?;
        }
        
        // Check for updates if configured
        if let (Some(ref update_provider), Some(ref version_checker)) = 
            (&self.update_provider, &self.version_checker) {
            self.check_and_update(update_provider.as_ref(), version_checker.as_ref()).await?;
        }
        
        // Execute the actual command (simplified version)
        let executable_path = self.config.get_executable_path()?;
        let exit_code = self.run_executable(&executable_path, additional_args)?;
        
        // Run post-execute hooks
        for hook in &self.post_execute_hooks {
            hook(exit_code)?;
        }
        
        Ok(exit_code)
    }
    
    fn config(&self) -> &ShimConfig {
        &self.config
    }
    
    fn validate(&self) -> Result<()> {
        self.config_loader.validate_config(&self.config)
    }
}

impl CustomizableShimRunner {
    /// Check for updates and install if available
    async fn check_and_update(&self, update_provider: &dyn UpdateProvider, version_checker: &dyn VersionChecker) -> Result<()> {
        let current_version = "1.0.0"; // TODO: Get from config or executable
        
        if let Some(latest_version) = update_provider.check_update_available(current_version).await? {
            if version_checker.is_newer_version(&latest_version, current_version)? {
                let executable_path = self.config.get_executable_path()?;
                update_provider.install_update(&latest_version, &executable_path).await?;
            }
        }
        
        Ok(())
    }
    
    /// Run the actual executable
    fn run_executable(&self, executable_path: &Path, args: &[String]) -> Result<i32> {
        // Simplified implementation - in reality this would be more complex
        use std::process::Command;
        
        let status = Command::new(executable_path)
            .args(args)
            .status()
            .map_err(|e| crate::error::ShimError::ProcessExecution(e.to_string()))?;
        
        Ok(status.code().unwrap_or(-1))
    }
}

/// Default TOML configuration loader
pub struct DefaultConfigLoader;

impl ShimConfigLoader for DefaultConfigLoader {
    fn load_config(&self, path: &Path) -> Result<ShimConfig> {
        ShimConfig::from_file(path)
    }

    fn save_config(&self, config: &ShimConfig, path: &Path) -> Result<()> {
        config.to_file(path)
    }

    fn file_extension(&self) -> &str {
        "shim.toml"
    }
}
