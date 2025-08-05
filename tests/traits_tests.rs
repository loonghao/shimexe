use shimexe_core::traits::{
    CustomizableShimRunner, DefaultConfigLoader, ShimConfigLoader, ShimRunnerBuilder,
    ShimRunnerTrait, UpdateProvider, VersionChecker,
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

struct MockUpdateProvider {
    available_version: Option<String>,
    should_fail: bool,
}

#[async_trait]
impl UpdateProvider for MockUpdateProvider {
    async fn check_update_available(&self, _current_version: &str) -> Result<Option<String>> {
        if self.should_fail {
            Err(ShimError::Config("Mock update check failure".to_string()))
        } else {
            Ok(self.available_version.clone())
        }
    }
    
    async fn install_update(&self, _version: &str, _target_path: &Path) -> Result<()> {
        if self.should_fail {
            Err(ShimError::Config("Mock install failure".to_string()))
        } else {
            Ok(())
        }
    }
}

struct MockVersionChecker {
    should_fail: bool,
}

impl VersionChecker for MockVersionChecker {
    fn is_newer_version(&self, _new_version: &str, _current_version: &str) -> Result<bool> {
        if self.should_fail {
            Err(ShimError::Config("Mock version check failure".to_string()))
        } else {
            Ok(true) // Always return true for testing
        }
    }
}

#[tokio::test]
async fn test_customizable_shim_runner_basic_execution() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test.shim.toml");
    let config = create_test_config();
    
    let loader = Box::new(MockConfigLoader { should_fail: false });
    let runner = CustomizableShimRunner::new(
        config,
        config_path,
        loader,
        None,
        None,
    );
    
    // Test basic execution (should work with echo command)
    let result = runner.execute(&["world".to_string()]).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[tokio::test]
async fn test_customizable_shim_runner_with_update_provider() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test.shim.toml");
    let config = create_test_config();
    
    let loader = Box::new(MockConfigLoader { should_fail: false });
    let update_provider = Box::new(MockUpdateProvider {
        available_version: Some("2.0.0".to_string()),
        should_fail: false,
    });
    let version_checker = Box::new(MockVersionChecker { should_fail: false });
    
    let runner = CustomizableShimRunner::new(
        config,
        config_path,
        loader,
        Some(update_provider),
        Some(version_checker),
    );
    
    // Test execution with update checking
    let result = runner.execute(&["world".to_string()]).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_customizable_shim_runner_update_failure() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test.shim.toml");
    let config = create_test_config();
    
    let loader = Box::new(MockConfigLoader { should_fail: false });
    let update_provider = Box::new(MockUpdateProvider {
        available_version: Some("2.0.0".to_string()),
        should_fail: true, // This will cause update to fail
    });
    let version_checker = Box::new(MockVersionChecker { should_fail: false });
    
    let runner = CustomizableShimRunner::new(
        config,
        config_path,
        loader,
        Some(update_provider),
        Some(version_checker),
    );
    
    // Test execution with failing update
    let result = runner.execute(&["world".to_string()]).await;
    assert!(result.is_err());
}

#[test]
fn test_customizable_shim_runner_config_access() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test.shim.toml");
    let config = create_test_config();
    let expected_name = config.shim.name.clone();
    
    let loader = Box::new(MockConfigLoader { should_fail: false });
    let runner = CustomizableShimRunner::new(
        config,
        config_path,
        loader,
        None,
        None,
    );
    
    // Test config access
    let retrieved_config = runner.config();
    assert_eq!(retrieved_config.shim.name, expected_name);
}

#[test]
fn test_customizable_shim_runner_validation() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test.shim.toml");
    let config = create_test_config();
    
    // Test with successful validation
    let loader = Box::new(MockConfigLoader { should_fail: false });
    let runner = CustomizableShimRunner::new(
        config.clone(),
        config_path.clone(),
        loader,
        None,
        None,
    );
    
    let result = runner.validate();
    assert!(result.is_ok());
    
    // Test with failing validation
    let failing_loader = Box::new(MockConfigLoader { should_fail: true });
    let failing_runner = CustomizableShimRunner::new(
        config,
        config_path,
        failing_loader,
        None,
        None,
    );
    
    let result = failing_runner.validate();
    assert!(result.is_err());
}

#[test]
fn test_pre_and_post_execute_hooks() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("test.shim.toml");
    let config = create_test_config();
    
    let loader = Box::new(MockConfigLoader { should_fail: false });
    let mut runner = CustomizableShimRunner::new(
        config,
        config_path,
        loader,
        None,
        None,
    );
    
    // Add pre-execute hook
    runner.add_pre_execute_hook(Box::new(|args| {
        assert!(!args.is_empty());
        Ok(())
    }));
    
    // Add post-execute hook
    runner.add_post_execute_hook(Box::new(|exit_code| {
        assert_eq!(exit_code, 0);
        Ok(())
    }));
    
    // The hooks should be called during execution
    // Note: This is a basic test - in a real scenario we'd need more sophisticated
    // hook testing with shared state or mock objects
}

#[test]
fn test_version_checker_trait() {
    let checker = MockVersionChecker { should_fail: false };
    
    let result = checker.is_newer_version("2.0.0", "1.0.0");
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    let failing_checker = MockVersionChecker { should_fail: true };
    let result = failing_checker.is_newer_version("2.0.0", "1.0.0");
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_provider_trait() {
    let provider = MockUpdateProvider {
        available_version: Some("2.0.0".to_string()),
        should_fail: false,
    };
    
    // Test check_update_available
    let result = provider.check_update_available("1.0.0").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("2.0.0".to_string()));
    
    // Test install_update
    let temp_file = NamedTempFile::new().unwrap();
    let result = provider.install_update("2.0.0", temp_file.path()).await;
    assert!(result.is_ok());
    
    // Test failing provider
    let failing_provider = MockUpdateProvider {
        available_version: None,
        should_fail: true,
    };
    
    let result = failing_provider.check_update_available("1.0.0").await;
    assert!(result.is_err());
}

#[test]
fn test_shim_config_loader_trait() {
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
