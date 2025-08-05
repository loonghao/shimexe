use shimexe_core::updater::ShimUpdater;
use shimexe_core::config::{AutoUpdate, UpdateProvider, VersionCheck};
use shimexe_core::error::ShimError;
use std::fs;
use std::path::PathBuf;
use tempfile::{NamedTempFile, TempDir};

fn create_test_auto_update_config() -> AutoUpdate {
    AutoUpdate {
        enabled: true,
        check_interval: 86400, // 24 hours
        provider: UpdateProvider::Github {
            repo: "test/repo".to_string(),
            asset_pattern: "test-{version}-{arch}.zip".to_string(),
            pre_release: false,
        },
        version_check: VersionCheck::Semantic,
        pre_update_command: None,
        post_update_command: None,
        backup_count: 3,
    }
}

#[test]
fn test_shim_updater_creation() {
    let temp_dir = TempDir::new().unwrap();
    let shim_path = temp_dir.path().join("test.shim.toml");
    let executable_path = temp_dir.path().join("test.exe");
    let config = create_test_auto_update_config();
    
    let updater = ShimUpdater::new(config.clone(), shim_path.clone(), executable_path.clone());
    
    assert_eq!(updater.config.enabled, config.enabled);
    assert_eq!(updater.config.check_interval, config.check_interval);
    assert_eq!(updater.shim_path, shim_path);
    assert_eq!(updater.executable_path, executable_path);
}

#[test]
fn test_should_check_for_updates_enabled() {
    let temp_dir = TempDir::new().unwrap();
    let shim_path = temp_dir.path().join("test.shim.toml");
    let executable_path = temp_dir.path().join("test.exe");
    let config = create_test_auto_update_config();
    
    let updater = ShimUpdater::new(config, shim_path, executable_path);
    
    // Should check when enabled and no last check file exists
    assert!(updater.should_check_for_updates());
}

#[test]
fn test_should_check_for_updates_disabled() {
    let temp_dir = TempDir::new().unwrap();
    let shim_path = temp_dir.path().join("test.shim.toml");
    let executable_path = temp_dir.path().join("test.exe");
    let mut config = create_test_auto_update_config();
    config.enabled = false;
    
    let updater = ShimUpdater::new(config, shim_path, executable_path);
    
    // Should not check when disabled
    assert!(!updater.should_check_for_updates());
}

#[test]
fn test_should_check_for_updates_with_recent_check() {
    let temp_dir = TempDir::new().unwrap();
    let shim_path = temp_dir.path().join("test.shim.toml");
    let executable_path = temp_dir.path().join("test.exe");
    let config = create_test_auto_update_config();
    
    let updater = ShimUpdater::new(config, shim_path, executable_path);
    
    // Create a recent last check file
    let last_check_path = updater.get_last_check_path();
    fs::create_dir_all(last_check_path.parent().unwrap()).unwrap();
    
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    fs::write(&last_check_path, current_time.to_string()).unwrap();
    
    // Should not check when recently checked
    assert!(!updater.should_check_for_updates());
}

#[test]
fn test_should_check_for_updates_with_old_check() {
    let temp_dir = TempDir::new().unwrap();
    let shim_path = temp_dir.path().join("test.shim.toml");
    let executable_path = temp_dir.path().join("test.exe");
    let config = create_test_auto_update_config();
    
    let updater = ShimUpdater::new(config, shim_path, executable_path);
    
    // Create an old last check file
    let last_check_path = updater.get_last_check_path();
    fs::create_dir_all(last_check_path.parent().unwrap()).unwrap();
    
    let old_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() - 90000; // More than 24 hours ago
    
    fs::write(&last_check_path, old_time.to_string()).unwrap();
    
    // Should check when last check was long ago
    assert!(updater.should_check_for_updates());
}

#[test]
fn test_get_last_check_path() {
    let temp_dir = TempDir::new().unwrap();
    let shim_path = temp_dir.path().join("test.shim.toml");
    let executable_path = temp_dir.path().join("test.exe");
    let config = create_test_auto_update_config();
    
    let updater = ShimUpdater::new(config, shim_path, executable_path);
    let last_check_path = updater.get_last_check_path();
    
    assert!(last_check_path.to_string_lossy().contains("test.shim.toml.last_check"));
}

#[test]
fn test_update_last_check_time() {
    let temp_dir = TempDir::new().unwrap();
    let shim_path = temp_dir.path().join("test.shim.toml");
    let executable_path = temp_dir.path().join("test.exe");
    let config = create_test_auto_update_config();
    
    let updater = ShimUpdater::new(config, shim_path, executable_path);
    
    // Update last check time
    let result = updater.update_last_check_time();
    assert!(result.is_ok());
    
    // Verify the file was created
    let last_check_path = updater.get_last_check_path();
    assert!(last_check_path.exists());
    
    // Verify the content is a valid timestamp
    let content = fs::read_to_string(&last_check_path).unwrap();
    let timestamp: u64 = content.trim().parse().unwrap();
    assert!(timestamp > 0);
}

#[test]
fn test_backup_executable() {
    let temp_dir = TempDir::new().unwrap();
    let shim_path = temp_dir.path().join("test.shim.toml");
    let executable_path = temp_dir.path().join("test.exe");
    let config = create_test_auto_update_config();
    
    // Create a test executable file
    fs::write(&executable_path, "test executable content").unwrap();
    
    let updater = ShimUpdater::new(config, shim_path, executable_path.clone());
    
    // Create backup
    let result = updater.backup_executable();
    assert!(result.is_ok());
    
    // Verify backup was created
    let backup_path = executable_path.with_extension("backup.1");
    assert!(backup_path.exists());
    
    let backup_content = fs::read_to_string(&backup_path).unwrap();
    assert_eq!(backup_content, "test executable content");
}

#[test]
fn test_backup_executable_multiple_backups() {
    let temp_dir = TempDir::new().unwrap();
    let shim_path = temp_dir.path().join("test.shim.toml");
    let executable_path = temp_dir.path().join("test.exe");
    let config = create_test_auto_update_config();
    
    // Create a test executable file
    fs::write(&executable_path, "version 1").unwrap();
    
    let updater = ShimUpdater::new(config, shim_path, executable_path.clone());
    
    // Create first backup
    updater.backup_executable().unwrap();
    
    // Update executable and create second backup
    fs::write(&executable_path, "version 2").unwrap();
    updater.backup_executable().unwrap();
    
    // Update executable and create third backup
    fs::write(&executable_path, "version 3").unwrap();
    updater.backup_executable().unwrap();
    
    // Verify all backups exist
    let backup1 = executable_path.with_extension("backup.1");
    let backup2 = executable_path.with_extension("backup.2");
    let backup3 = executable_path.with_extension("backup.3");
    
    assert!(backup1.exists());
    assert!(backup2.exists());
    assert!(backup3.exists());
    
    // Verify backup contents
    assert_eq!(fs::read_to_string(&backup1).unwrap(), "version 3");
    assert_eq!(fs::read_to_string(&backup2).unwrap(), "version 2");
    assert_eq!(fs::read_to_string(&backup3).unwrap(), "version 1");
}

#[test]
fn test_backup_executable_exceeds_limit() {
    let temp_dir = TempDir::new().unwrap();
    let shim_path = temp_dir.path().join("test.shim.toml");
    let executable_path = temp_dir.path().join("test.exe");
    let mut config = create_test_auto_update_config();
    config.backup_count = 2; // Limit to 2 backups
    
    // Create a test executable file
    fs::write(&executable_path, "version 1").unwrap();
    
    let updater = ShimUpdater::new(config, shim_path, executable_path.clone());
    
    // Create multiple backups
    for i in 1..=4 {
        fs::write(&executable_path, format!("version {}", i)).unwrap();
        updater.backup_executable().unwrap();
    }
    
    // Should only have 2 backups (the limit)
    let backup1 = executable_path.with_extension("backup.1");
    let backup2 = executable_path.with_extension("backup.2");
    let backup3 = executable_path.with_extension("backup.3");
    
    assert!(backup1.exists());
    assert!(backup2.exists());
    assert!(!backup3.exists()); // Should not exist due to limit
    
    // Verify backup contents (most recent should be in backup.1)
    assert_eq!(fs::read_to_string(&backup1).unwrap(), "version 4");
    assert_eq!(fs::read_to_string(&backup2).unwrap(), "version 3");
}

#[test]
fn test_backup_nonexistent_executable() {
    let temp_dir = TempDir::new().unwrap();
    let shim_path = temp_dir.path().join("test.shim.toml");
    let executable_path = temp_dir.path().join("nonexistent.exe");
    let config = create_test_auto_update_config();
    
    let updater = ShimUpdater::new(config, shim_path, executable_path);
    
    // Should succeed even if executable doesn't exist
    let result = updater.backup_executable();
    assert!(result.is_ok());
}

#[test]
fn test_run_command_success() {
    let temp_dir = TempDir::new().unwrap();
    let shim_path = temp_dir.path().join("test.shim.toml");
    let executable_path = temp_dir.path().join("test.exe");
    let config = create_test_auto_update_config();
    
    let updater = ShimUpdater::new(config, shim_path, executable_path);
    
    // Test with a simple command that should succeed
    #[cfg(windows)]
    let result = updater.run_command("echo test", "test");
    #[cfg(not(windows))]
    let result = updater.run_command("echo test", "test");
    
    assert!(result.is_ok());
}

#[test]
fn test_run_command_failure() {
    let temp_dir = TempDir::new().unwrap();
    let shim_path = temp_dir.path().join("test.shim.toml");
    let executable_path = temp_dir.path().join("test.exe");
    let config = create_test_auto_update_config();
    
    let updater = ShimUpdater::new(config, shim_path, executable_path);
    
    // Test with a command that should fail
    let result = updater.run_command("nonexistent_command_12345", "test");
    assert!(result.is_err());
}

#[test]
fn test_version_check_types() {
    // Test different version check configurations
    let mut config = create_test_auto_update_config();
    
    // Test Semantic versioning
    config.version_check = VersionCheck::Semantic;
    assert!(matches!(config.version_check, VersionCheck::Semantic));
    
    // Test Timestamp versioning
    config.version_check = VersionCheck::Timestamp;
    assert!(matches!(config.version_check, VersionCheck::Timestamp));
    
    // Test Custom versioning
    config.version_check = VersionCheck::Custom("custom-pattern".to_string());
    assert!(matches!(config.version_check, VersionCheck::Custom(_)));
}

#[test]
fn test_update_provider_types() {
    let mut config = create_test_auto_update_config();
    
    // Test Github provider
    config.provider = UpdateProvider::Github {
        repo: "owner/repo".to_string(),
        asset_pattern: "asset-{version}.zip".to_string(),
        pre_release: true,
    };
    assert!(matches!(config.provider, UpdateProvider::Github { .. }));
    
    // Test HTTPS provider
    config.provider = UpdateProvider::Https {
        base_url: "https://example.com/releases".to_string(),
        url_pattern: "{base_url}/{version}/app.zip".to_string(),
    };
    assert!(matches!(config.provider, UpdateProvider::Https { .. }));
    
    // Test Custom provider
    config.provider = UpdateProvider::Custom {
        update_command: "custom-update {version}".to_string(),
        check_command: "custom-check".to_string(),
    };
    assert!(matches!(config.provider, UpdateProvider::Custom { .. }));
}

#[test]
fn test_auto_update_serialization() {
    let config = create_test_auto_update_config();
    
    // Test that the config can be serialized and deserialized
    let serialized = toml::to_string(&config).unwrap();
    let deserialized: AutoUpdate = toml::from_str(&serialized).unwrap();
    
    assert_eq!(config.enabled, deserialized.enabled);
    assert_eq!(config.check_interval, deserialized.check_interval);
    assert_eq!(config.backup_count, deserialized.backup_count);
    assert!(matches!(deserialized.provider, UpdateProvider::Github { .. }));
    assert!(matches!(deserialized.version_check, VersionCheck::Semantic));
}
