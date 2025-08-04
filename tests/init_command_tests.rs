use shimexe::commands::init::InitCommand;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_init_command_basic() {
    let temp_dir = TempDir::new().unwrap();
    let shim_dir = temp_dir.path().join("shims");
    
    let init_cmd = InitCommand { examples: false };
    let result = init_cmd.execute(Some(shim_dir.clone()));
    
    assert!(result.is_ok());
    assert!(shim_dir.exists());
}

#[test]
fn test_init_command_with_examples() {
    let temp_dir = TempDir::new().unwrap();
    let shim_dir = temp_dir.path().join("shims");
    
    let init_cmd = InitCommand { examples: true };
    let result = init_cmd.execute(Some(shim_dir.clone()));
    
    assert!(result.is_ok());
    assert!(shim_dir.exists());
    
    // Check that example shims were created
    let hello_config = shim_dir.join("hello.toml");
    let ls_config = shim_dir.join("ls-example.toml");
    
    assert!(hello_config.exists());
    assert!(ls_config.exists());
    
    // Verify the content of example configs
    let hello_content = fs::read_to_string(hello_config).unwrap();
    assert!(hello_content.contains("Hello from shimexe!"));
    assert!(hello_content.contains("Example echo shim"));
    
    let ls_content = fs::read_to_string(ls_config).unwrap();
    assert!(ls_content.contains("ls"));
    assert!(ls_content.contains("--color=auto"));
}

#[test]
fn test_init_command_default_directory() {
    let init_cmd = InitCommand { examples: false };
    let result = init_cmd.execute(None);
    
    // Should succeed even with default directory
    assert!(result.is_ok());
}

#[test]
fn test_init_command_examples_flag() {
    // Test that the examples flag is properly set
    let init_cmd_no_examples = InitCommand { examples: false };
    assert!(!init_cmd_no_examples.examples);
    
    let init_cmd_with_examples = InitCommand { examples: true };
    assert!(init_cmd_with_examples.examples);
}

#[test]
fn test_init_command_creates_directory_structure() {
    let temp_dir = TempDir::new().unwrap();
    let shim_dir = temp_dir.path().join("nested").join("shims");
    
    let init_cmd = InitCommand { examples: false };
    let result = init_cmd.execute(Some(shim_dir.clone()));
    
    assert!(result.is_ok());
    assert!(shim_dir.exists());
    assert!(shim_dir.is_dir());
}

#[test]
fn test_init_command_with_existing_directory() {
    let temp_dir = TempDir::new().unwrap();
    let shim_dir = temp_dir.path().join("existing_shims");
    
    // Create the directory first
    fs::create_dir_all(&shim_dir).unwrap();
    
    let init_cmd = InitCommand { examples: false };
    let result = init_cmd.execute(Some(shim_dir.clone()));
    
    // Should still succeed with existing directory
    assert!(result.is_ok());
    assert!(shim_dir.exists());
}

#[test]
fn test_init_command_examples_content_validation() {
    let temp_dir = TempDir::new().unwrap();
    let shim_dir = temp_dir.path().join("shims");
    
    let init_cmd = InitCommand { examples: true };
    let result = init_cmd.execute(Some(shim_dir.clone()));
    
    assert!(result.is_ok());
    
    // Read and validate hello.toml
    let hello_config = shim_dir.join("hello.toml");
    let hello_content = fs::read_to_string(hello_config).unwrap();
    
    // Parse as TOML to ensure it's valid
    let parsed: toml::Value = toml::from_str(&hello_content).unwrap();
    
    // Verify structure
    assert!(parsed.get("shim").is_some());
    assert!(parsed.get("metadata").is_some());
    
    let shim_section = parsed.get("shim").unwrap();
    assert_eq!(shim_section.get("name").unwrap().as_str().unwrap(), "hello");
    assert_eq!(shim_section.get("path").unwrap().as_str().unwrap(), "echo");
    
    // Read and validate ls-example.toml
    let ls_config = shim_dir.join("ls-example.toml");
    let ls_content = fs::read_to_string(ls_config).unwrap();
    
    // Parse as TOML to ensure it's valid
    let parsed: toml::Value = toml::from_str(&ls_content).unwrap();
    
    // Verify structure
    assert!(parsed.get("shim").is_some());
    assert!(parsed.get("args").is_some());
    
    let shim_section = parsed.get("shim").unwrap();
    assert_eq!(shim_section.get("name").unwrap().as_str().unwrap(), "ls-example");
}

#[test]
fn test_init_command_multiple_runs() {
    let temp_dir = TempDir::new().unwrap();
    let shim_dir = temp_dir.path().join("shims");
    
    let init_cmd = InitCommand { examples: true };
    
    // Run init command twice
    let result1 = init_cmd.execute(Some(shim_dir.clone()));
    let result2 = init_cmd.execute(Some(shim_dir.clone()));
    
    // Both should succeed
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    
    // Files should still exist
    assert!(shim_dir.join("hello.toml").exists());
    assert!(shim_dir.join("ls-example.toml").exists());
}

#[cfg(unix)]
#[test]
fn test_init_command_unix_specific_examples() {
    let temp_dir = TempDir::new().unwrap();
    let shim_dir = temp_dir.path().join("shims");
    
    let init_cmd = InitCommand { examples: true };
    let result = init_cmd.execute(Some(shim_dir.clone()));
    
    assert!(result.is_ok());
    
    // On Unix, ls command should be available
    let ls_config = shim_dir.join("ls-example.toml");
    let ls_content = fs::read_to_string(ls_config).unwrap();
    
    // Should contain Unix-specific ls command
    assert!(ls_content.contains("ls"));
}

#[cfg(windows)]
#[test]
fn test_init_command_windows_specific_examples() {
    let temp_dir = TempDir::new().unwrap();
    let shim_dir = temp_dir.path().join("shims");
    
    let init_cmd = InitCommand { examples: true };
    let result = init_cmd.execute(Some(shim_dir.clone()));
    
    assert!(result.is_ok());
    
    // On Windows, the examples should still work
    let hello_config = shim_dir.join("hello.toml");
    assert!(hello_config.exists());
    
    let ls_config = shim_dir.join("ls-example.toml");
    assert!(ls_config.exists());
}
