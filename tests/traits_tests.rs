use shimexe_core::traits::{
    CustomizableShimRunner, DefaultConfigLoader, ShimConfigLoader, ShimRunnerBuilder,
    ShimRunnerTrait,
};
use shimexe_core::config::{ShimConfig, ShimCore, SourceType};
use shimexe_core::error::{Result, ShimError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tempfile::{NamedTempFile, TempDir};
use async_trait::async_trait;

fn create_test_config() -> ShimConfig {
    ShimConfig {
        shim: ShimCore {
            name: "test-shim".to_string(),
            path: "echo".to_string(),
            args: vec!["hello".to_string()],
            cwd: None,
            download_url: None,
            source_type: SourceType::File,
            extracted_executables: vec![],
        },
        args: Default::default(),
        env: HashMap::new(),
        metadata: Default::default(),
        auto_update: None,
    }
}

#[test]
fn test_default_config_loader() {
    let loader = DefaultConfigLoader;
    
    // Test file extension
    assert_eq!(loader.file_extension(), "shim.toml");
    
    // Test save and load config
    let temp_file = NamedTempFile::new().unwrap();
    let config = create_test_config();
    
    // Save config
    loader.save_config(&config, temp_file.path()).unwrap();
    
    // Load config
    let loaded_config = loader.load_config(temp_file.path()).unwrap();
    assert_eq!(loaded_config.shim.name, config.shim.name);
    assert_eq!(loaded_config.shim.path, config.shim.path);
    assert_eq!(loaded_config.shim.args, config.shim.args);
}

#[test]
fn test_default_config_loader_invalid_file() {
    let loader = DefaultConfigLoader;
    let non_existent_path = Path::new("/non/existent/file.toml");
    
    let result = loader.load_config(non_existent_path);
    assert!(result.is_err());
}

#[test]
fn test_shim_runner_builder() {
    let builder = ShimRunnerBuilder::new();
    
    // Test default values
    assert!(builder.config_loader.is_none());
    assert!(builder.update_provider.is_none());
    assert!(builder.version_checker.is_none());
    assert!(builder.pre_execute_hooks.is_empty());
    assert!(builder.post_execute_hooks.is_empty());
}

#[test]
fn test_shim_runner_builder_default() {
    let builder1 = ShimRunnerBuilder::new();
    let builder2 = ShimRunnerBuilder::default();
    
    // Both should be equivalent
    assert_eq!(builder1.config_loader.is_none(), builder2.config_loader.is_none());
    assert_eq!(builder1.update_provider.is_none(), builder2.update_provider.is_none());
    assert_eq!(builder1.version_checker.is_none(), builder2.version_checker.is_none());
}

// Mock implementations for testing
struct MockConfigLoader {
    should_fail: bool,
}

impl ShimConfigLoader for MockConfigLoader {
    fn load_config(&self, _path: &Path) -> Result<ShimConfig> {
        if self.should_fail {
            Err(ShimError::Config("Mock load failure".to_string()))
        } else {
            Ok(create_test_config())
        }
    }
    
    fn save_config(&self, _config: &ShimConfig, _path: &Path) -> Result<()> {
        if self.should_fail {
            Err(ShimError::Config("Mock save failure".to_string()))
        } else {
            Ok(())
        }
    }
    
    fn file_extension(&self) -> &str {
        "mock.toml"
    }
}

// Mock implementations for testing basic functionality
struct MockRunner {
    config: ShimConfig,
}

impl MockRunner {
    fn new(config: ShimConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl ShimRunnerTrait for MockRunner {
    async fn execute(&self, _additional_args: &[String]) -> Result<i32> {
        Ok(0) // Always succeed for testing
    }

    fn config(&self) -> &ShimConfig {
        &self.config
    }

    fn validate(&self) -> Result<()> {
        Ok(()) // Always valid for testing
    }
}

#[tokio::test]
async fn test_mock_runner_basic_execution() {
    let config = create_test_config();
    let runner = MockRunner::new(config);

    // Test basic execution
    let result = runner.execute(&["world".to_string()]).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);

    // Test config access
    assert_eq!(runner.config().shim.name, "test-shim");

    // Test validation
    assert!(runner.validate().is_ok());
}

#[test]
fn test_shim_runner_builder_basic() {
    let builder = ShimRunnerBuilder::new();

    // Test that builder can be created
    assert!(builder.config_loader.is_none());
    assert!(builder.pre_execute_hooks.is_empty());
    assert!(builder.post_execute_hooks.is_empty());
}

#[test]
fn test_shim_runner_builder_default() {
    let builder1 = ShimRunnerBuilder::new();
    let builder2 = ShimRunnerBuilder::default();

    // Both should be equivalent
    assert_eq!(builder1.config_loader.is_none(), builder2.config_loader.is_none());
    assert_eq!(builder1.pre_execute_hooks.len(), builder2.pre_execute_hooks.len());
    assert_eq!(builder1.post_execute_hooks.len(), builder2.post_execute_hooks.len());
}

#[test]
fn test_default_config_loader_file_extension() {
    let loader = DefaultConfigLoader;
    assert_eq!(loader.file_extension(), "shim.toml");
}

#[test]
fn test_default_config_loader_operations() {
    let loader = DefaultConfigLoader;
    let temp_file = NamedTempFile::new().unwrap();
    let config = create_test_config();

    // Test save config
    let save_result = loader.save_config(&config, temp_file.path());
    assert!(save_result.is_ok());

    // Test load config
    let load_result = loader.load_config(temp_file.path());
    assert!(load_result.is_ok());

    let loaded_config = load_result.unwrap();
    assert_eq!(loaded_config.shim.name, config.shim.name);
    assert_eq!(loaded_config.shim.path, config.shim.path);
}

#[test]
fn test_default_config_loader_invalid_file() {
    let loader = DefaultConfigLoader;
    let non_existent_path = Path::new("/non/existent/file.toml");

    let result = loader.load_config(non_existent_path);
    assert!(result.is_err());
}

#[test]
fn test_mock_config_loader() {
    let loader = MockConfigLoader { should_fail: false };

    // Test file extension
    assert_eq!(loader.file_extension(), "mock.toml");

    // Test successful operations
    let temp_file = NamedTempFile::new().unwrap();
    let config = create_test_config();

    let save_result = loader.save_config(&config, temp_file.path());
    assert!(save_result.is_ok());

    let load_result = loader.load_config(temp_file.path());
    assert!(load_result.is_ok());

    // Test failing operations
    let failing_loader = MockConfigLoader { should_fail: true };

    let save_result = failing_loader.save_config(&config, temp_file.path());
    assert!(save_result.is_err());

    let load_result = failing_loader.load_config(temp_file.path());
    assert!(load_result.is_err());
}
