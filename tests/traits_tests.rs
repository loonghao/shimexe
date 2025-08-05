use shimexe_core::traits::{
    DefaultConfigLoader, ShimConfigLoader, ShimRunnerBuilder,
};
use shimexe_core::config::{ShimConfig, ShimCore, SourceType};
use shimexe_core::error::{Result, ShimError};
use std::collections::HashMap;
use std::path::Path;
use tempfile::NamedTempFile;

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
fn test_shim_runner_builder_creation() {
    let _builder = ShimRunnerBuilder::new();

    // Test that builder can be created - we can't test private fields
    // but we can test that it doesn't panic
    assert!(true);
}

#[test]
fn test_shim_runner_builder_default() {
    let _builder1 = ShimRunnerBuilder::new();
    let _builder2 = ShimRunnerBuilder::default();

    // Both should be created successfully
    assert!(true);
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
