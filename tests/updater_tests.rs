use shimexe_core::updater::ShimUpdater;
use shimexe_core::config::{AutoUpdate, UpdateProvider, VersionCheck};
use shimexe_core::error::ShimError;
use std::fs;
use std::path::PathBuf;
use tempfile::{NamedTempFile, TempDir};

fn create_test_auto_update_config() -> AutoUpdate {
    AutoUpdate {
        enabled: true,
        provider: UpdateProvider::Github {
            repo: "test/repo".to_string(),
            asset_pattern: "test-{version}-{os}-{arch}".to_string(),
            include_prerelease: false,
        },
        download_url: "https://github.com/test/repo/releases/download/v{version}/test-{version}-{os}-{arch}".to_string(),
        version_check: VersionCheck::GithubLatest {
            repo: "test/repo".to_string(),
            include_prerelease: false,
        },
        check_interval_hours: 24,
        pre_update_command: None,
        post_update_command: None,
    }
}

#[test]
fn test_shim_updater_creation() {
    let temp_dir = TempDir::new().unwrap();
    let shim_path = temp_dir.path().join("test.shim.toml");
    let executable_path = temp_dir.path().join("test.exe");
    let config = create_test_auto_update_config();

    let updater = ShimUpdater::new(config.clone(), shim_path.clone(), executable_path.clone());

    // Test that updater was created successfully
    // Note: The fields are private, so we can't directly access them
    // This test mainly ensures the constructor works
    assert!(true); // Placeholder assertion
}

#[test]
fn test_auto_update_config_creation() {
    let config = create_test_auto_update_config();

    assert!(config.enabled);
    assert_eq!(config.check_interval_hours, 24);
    assert!(matches!(config.provider, UpdateProvider::Github { .. }));
    assert!(matches!(config.version_check, VersionCheck::GithubLatest { .. }));
}

#[test]
fn test_auto_update_config_disabled() {
    let mut config = create_test_auto_update_config();
    config.enabled = false;

    assert!(!config.enabled);
    assert_eq!(config.check_interval_hours, 24);
}

#[test]
fn test_version_check_github_latest() {
    let config = VersionCheck::GithubLatest {
        repo: "owner/repo".to_string(),
        include_prerelease: false,
    };

    if let VersionCheck::GithubLatest { repo, include_prerelease } = config {
        assert_eq!(repo, "owner/repo");
        assert!(!include_prerelease);
    } else {
        panic!("Expected GithubLatest version check");
    }
}

#[test]
fn test_version_check_http() {
    let config = VersionCheck::Http {
        url: "https://api.example.com/version".to_string(),
        json_path: Some("version".to_string()),
        regex_pattern: None,
    };

    if let VersionCheck::Http { url, json_path, regex_pattern } = config {
        assert_eq!(url, "https://api.example.com/version");
        assert_eq!(json_path, Some("version".to_string()));
        assert!(regex_pattern.is_none());
    } else {
        panic!("Expected Http version check");
    }
}

#[test]
fn test_update_provider_github() {
    let provider = UpdateProvider::Github {
        repo: "owner/repository".to_string(),
        asset_pattern: "app-{version}-{os}-{arch}.tar.gz".to_string(),
        include_prerelease: true,
    };

    if let UpdateProvider::Github { repo, asset_pattern, include_prerelease } = provider {
        assert_eq!(repo, "owner/repository");
        assert_eq!(asset_pattern, "app-{version}-{os}-{arch}.tar.gz");
        assert!(include_prerelease);
    } else {
        panic!("Expected Github provider");
    }
}

#[test]
fn test_version_check_command() {
    let config = VersionCheck::Command {
        command: "myapp".to_string(),
        args: vec!["--version".to_string()],
    };

    if let VersionCheck::Command { command, args } = config {
        assert_eq!(command, "myapp");
        assert_eq!(args, vec!["--version"]);
    } else {
        panic!("Expected Command version check");
    }
}

#[test]
fn test_auto_update_serialization() {
    let config = create_test_auto_update_config();

    // Test that the config can be serialized and deserialized
    let serialized = toml::to_string(&config).unwrap();
    let deserialized: AutoUpdate = toml::from_str(&serialized).unwrap();

    assert_eq!(config.enabled, deserialized.enabled);
    assert_eq!(config.check_interval_hours, deserialized.check_interval_hours);
    assert!(matches!(deserialized.provider, UpdateProvider::Github { .. }));
    assert!(matches!(deserialized.version_check, VersionCheck::GithubLatest { .. }));
}




