use shimexe_core::{
    ArgsConfig, AutoUpdate, ExtractedExecutable, ShimConfig, ShimCore, ShimMetadata, SourceType,
    UpdateProvider, VersionCheck,
};
use std::collections::HashMap;
use tempfile::NamedTempFile;
use std::fs;

#[test]
fn test_archive_config_serialization() {
    let config = ShimConfig {
        shim: ShimCore {
            name: "test-archive".to_string(),
            path: "/path/to/extracted/tool.exe".to_string(),
            args: vec![],
            cwd: None,
            download_url: Some("https://example.com/tools.zip".to_string()),
            source_type: SourceType::Archive,
            extracted_executables: vec![
                ExtractedExecutable {
                    name: "tool1".to_string(),
                    path: "tool1.exe".to_string(),
                    full_path: "/path/to/extracted/tool1.exe".to_string(),
                    is_primary: true,
                },
                ExtractedExecutable {
                    name: "tool2".to_string(),
                    path: "tool2.exe".to_string(),
                    full_path: "/path/to/extracted/tool2.exe".to_string(),
                    is_primary: false,
                },
            ],
        },
        args: Default::default(),
        env: HashMap::new(),
        metadata: Default::default(),
        auto_update: None,
    };

    // Test serialization
    let toml_str = toml::to_string(&config).expect("Failed to serialize config");
    assert!(toml_str.contains("source_type = \"archive\""));
    assert!(toml_str.contains("[[shim.extracted_executables]]"));
    assert!(toml_str.contains("is_primary = true"));

    // Test deserialization
    let deserialized: ShimConfig = toml::from_str(&toml_str).expect("Failed to deserialize config");
    assert_eq!(deserialized.shim.source_type, SourceType::Archive);
    assert_eq!(deserialized.shim.extracted_executables.len(), 2);
    assert!(deserialized.shim.extracted_executables[0].is_primary);
    assert!(!deserialized.shim.extracted_executables[1].is_primary);
}

#[test]
fn test_url_config_serialization() {
    let config = ShimConfig {
        shim: ShimCore {
            name: "test-url".to_string(),
            path: "/path/to/downloaded/tool.exe".to_string(),
            args: vec![],
            cwd: None,
            download_url: Some("https://example.com/tool.exe".to_string()),
            source_type: SourceType::Url,
            extracted_executables: vec![],
        },
        args: Default::default(),
        env: HashMap::new(),
        metadata: Default::default(),
        auto_update: None,
    };

    let toml_str = toml::to_string(&config).expect("Failed to serialize config");
    assert!(toml_str.contains("source_type = \"url\""));
    assert!(!toml_str.contains("extracted_executables")); // Should be omitted when empty

    let deserialized: ShimConfig = toml::from_str(&toml_str).expect("Failed to deserialize config");
    assert_eq!(deserialized.shim.source_type, SourceType::Url);
    assert!(deserialized.shim.extracted_executables.is_empty());
}

#[test]
fn test_file_config_backward_compatibility() {
    // Test that old configs without source_type still work
    let toml_str = r#"
[shim]
name = "test"
path = "/usr/bin/test"
args = []

[env]

[metadata]
"#;

    let config: ShimConfig = toml::from_str(toml_str).expect("Failed to deserialize old config");
    assert_eq!(config.shim.source_type, SourceType::File); // Should default to File
    assert!(config.shim.extracted_executables.is_empty());
    assert!(config.shim.download_url.is_none());
}

#[test]
fn test_args_config_serialization() {
    let args_config = ArgsConfig {
        pass_through: true,
        prepend: vec!["--debug".to_string(), "--verbose".to_string()],
        append: vec!["--output".to_string(), "result.txt".to_string()],
    };

    let toml_str = toml::to_string(&args_config).unwrap();
    let deserialized: ArgsConfig = toml::from_str(&toml_str).unwrap();

    assert_eq!(deserialized.pass_through, args_config.pass_through);
    assert_eq!(deserialized.prepend, args_config.prepend);
    assert_eq!(deserialized.append, args_config.append);
}

#[test]
fn test_shim_metadata_full() {
    let metadata = ShimMetadata {
        description: Some("A comprehensive test tool".to_string()),
        version: Some("2.1.0".to_string()),
        author: Some("Test Team <test@example.com>".to_string()),
        tags: vec!["testing".to_string(), "cli".to_string(), "utility".to_string()],
    };

    let toml_str = toml::to_string(&metadata).unwrap();
    let deserialized: ShimMetadata = toml::from_str(&toml_str).unwrap();

    assert_eq!(deserialized.description, metadata.description);
    assert_eq!(deserialized.version, metadata.version);
    assert_eq!(deserialized.author, metadata.author);
    assert_eq!(deserialized.tags, metadata.tags);
}

#[test]
fn test_auto_update_github_provider() {
    let auto_update = AutoUpdate {
        enabled: true,
        check_interval: 86400, // 24 hours
        provider: UpdateProvider::Github {
            repo: "owner/repository".to_string(),
            asset_pattern: "app-{version}-{platform}-{arch}.tar.gz".to_string(),
            pre_release: false,
        },
        version_check: VersionCheck::Semantic,
        pre_update_command: Some("echo 'Starting update'".to_string()),
        post_update_command: Some("echo 'Update completed'".to_string()),
        backup_count: 5,
    };

    let toml_str = toml::to_string(&auto_update).unwrap();
    let deserialized: AutoUpdate = toml::from_str(&toml_str).unwrap();

    assert_eq!(deserialized.enabled, auto_update.enabled);
    assert_eq!(deserialized.check_interval, auto_update.check_interval);
    assert_eq!(deserialized.backup_count, auto_update.backup_count);

    if let UpdateProvider::Github { repo, asset_pattern, pre_release } = deserialized.provider {
        assert_eq!(repo, "owner/repository");
        assert_eq!(asset_pattern, "app-{version}-{platform}-{arch}.tar.gz");
        assert!(!pre_release);
    } else {
        panic!("Expected Github provider");
    }
}

#[test]
fn test_auto_update_https_provider() {
    let auto_update = AutoUpdate {
        enabled: true,
        check_interval: 3600, // 1 hour
        provider: UpdateProvider::Https {
            base_url: "https://releases.example.com".to_string(),
            url_pattern: "{base_url}/v{version}/app-{platform}.zip".to_string(),
        },
        version_check: VersionCheck::Timestamp,
        pre_update_command: None,
        post_update_command: None,
        backup_count: 3,
    };

    let toml_str = toml::to_string(&auto_update).unwrap();
    let deserialized: AutoUpdate = toml::from_str(&toml_str).unwrap();

    assert!(matches!(deserialized.version_check, VersionCheck::Timestamp));
    assert!(deserialized.pre_update_command.is_none());
    assert!(deserialized.post_update_command.is_none());

    if let UpdateProvider::Https { base_url, url_pattern } = deserialized.provider {
        assert_eq!(base_url, "https://releases.example.com");
        assert_eq!(url_pattern, "{base_url}/v{version}/app-{platform}.zip");
    } else {
        panic!("Expected Https provider");
    }
}

#[test]
fn test_auto_update_custom_provider() {
    let auto_update = AutoUpdate {
        enabled: false,
        check_interval: 7200, // 2 hours
        provider: UpdateProvider::Custom {
            update_command: "custom-updater --install {version}".to_string(),
            check_command: "custom-checker --latest".to_string(),
        },
        version_check: VersionCheck::Custom("build-{timestamp}".to_string()),
        pre_update_command: Some("systemctl stop myapp".to_string()),
        post_update_command: Some("systemctl start myapp".to_string()),
        backup_count: 10,
    };

    let toml_str = toml::to_string(&auto_update).unwrap();
    let deserialized: AutoUpdate = toml::from_str(&toml_str).unwrap();

    assert!(!deserialized.enabled);
    assert_eq!(deserialized.backup_count, 10);

    if let UpdateProvider::Custom { update_command, check_command } = deserialized.provider {
        assert_eq!(update_command, "custom-updater --install {version}");
        assert_eq!(check_command, "custom-checker --latest");
    } else {
        panic!("Expected Custom provider");
    }

    if let VersionCheck::Custom(pattern) = deserialized.version_check {
        assert_eq!(pattern, "build-{timestamp}");
    } else {
        panic!("Expected Custom version check");
    }
}

#[test]
fn test_complete_config_serialization() {
    let mut env_vars = HashMap::new();
    env_vars.insert("APP_ENV".to_string(), "production".to_string());
    env_vars.insert("LOG_LEVEL".to_string(), "info".to_string());
    env_vars.insert("CONFIG_PATH".to_string(), "/etc/myapp/config.json".to_string());

    let config = ShimConfig {
        shim: ShimCore {
            name: "complete-test".to_string(),
            path: "/opt/myapp/bin/myapp".to_string(),
            args: vec!["--config".to_string(), "${CONFIG_PATH}".to_string()],
            cwd: Some("/opt/myapp".to_string()),
            download_url: Some("https://releases.myapp.com/latest.tar.gz".to_string()),
            source_type: SourceType::Archive,
            extracted_executables: vec![
                ExtractedExecutable {
                    name: "myapp".to_string(),
                    path: "bin/myapp".to_string(),
                    full_path: "/opt/myapp/bin/myapp".to_string(),
                    is_primary: true,
                },
                ExtractedExecutable {
                    name: "myapp-cli".to_string(),
                    path: "bin/myapp-cli".to_string(),
                    full_path: "/opt/myapp/bin/myapp-cli".to_string(),
                    is_primary: false,
                },
            ],
        },
        args: ArgsConfig {
            pass_through: true,
            prepend: vec!["--global-config".to_string()],
            append: vec!["--log-format".to_string(), "json".to_string()],
        },
        env: env_vars,
        metadata: ShimMetadata {
            description: Some("Complete application with all features".to_string()),
            version: Some("3.2.1".to_string()),
            author: Some("MyApp Development Team".to_string()),
            tags: vec!["production".to_string(), "enterprise".to_string(), "cli".to_string()],
        },
        auto_update: Some(AutoUpdate {
            enabled: true,
            check_interval: 43200, // 12 hours
            provider: UpdateProvider::Github {
                repo: "mycompany/myapp".to_string(),
                asset_pattern: "myapp-{version}-{platform}-{arch}.tar.gz".to_string(),
                pre_release: false,
            },
            version_check: VersionCheck::Semantic,
            pre_update_command: Some("echo 'Preparing for update'".to_string()),
            post_update_command: Some("echo 'Update completed successfully'".to_string()),
            backup_count: 5,
        }),
    };

    // Test full serialization and deserialization
    let toml_str = toml::to_string(&config).unwrap();
    let deserialized: ShimConfig = toml::from_str(&toml_str).unwrap();

    // Verify all fields are preserved
    assert_eq!(deserialized.shim.name, config.shim.name);
    assert_eq!(deserialized.shim.path, config.shim.path);
    assert_eq!(deserialized.shim.args, config.shim.args);
    assert_eq!(deserialized.shim.cwd, config.shim.cwd);
    assert_eq!(deserialized.shim.download_url, config.shim.download_url);
    assert_eq!(deserialized.shim.source_type, config.shim.source_type);
    assert_eq!(deserialized.shim.extracted_executables.len(), 2);
    assert_eq!(deserialized.args.pass_through, config.args.pass_through);
    assert_eq!(deserialized.env.len(), 3);
    assert_eq!(deserialized.metadata.tags.len(), 3);
    assert!(deserialized.auto_update.is_some());
}
