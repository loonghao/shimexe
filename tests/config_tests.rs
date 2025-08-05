use shimexe_core::template::{ArgsConfig, ArgsMode};
use shimexe_core::{
    AutoUpdate, ExtractedExecutable, ShimConfig, ShimCore, ShimMetadata, SourceType,
    UpdateProvider, VersionCheck,
};
use std::collections::HashMap;

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
        template: Some(vec![
            "--debug".to_string(),
            "{args}".to_string(),
            "--output".to_string(),
            "result.txt".to_string(),
        ]),
        inline: Some("--verbose {args}".to_string()),
        mode: ArgsMode::Template,
        default: vec!["--help".to_string()],
        prefix: vec!["--prefix".to_string()],
        suffix: vec!["--suffix".to_string()],
    };

    let toml_str = toml::to_string(&args_config).unwrap();
    let deserialized: ArgsConfig = toml::from_str(&toml_str).unwrap();

    assert_eq!(deserialized.template, args_config.template);
    assert_eq!(deserialized.inline, args_config.inline);
    assert_eq!(deserialized.mode, args_config.mode);
    assert_eq!(deserialized.default, args_config.default);
    assert_eq!(deserialized.prefix, args_config.prefix);
    assert_eq!(deserialized.suffix, args_config.suffix);
}

#[test]
fn test_shim_metadata_full() {
    let metadata = ShimMetadata {
        description: Some("A comprehensive test tool".to_string()),
        version: Some("2.1.0".to_string()),
        author: Some("Test Team <test@example.com>".to_string()),
        tags: vec![
            "testing".to_string(),
            "cli".to_string(),
            "utility".to_string(),
        ],
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
        provider: UpdateProvider::Github {
            repo: "owner/repository".to_string(),
            asset_pattern: "app-{version}-{platform}-{arch}.tar.gz".to_string(),
            include_prerelease: false,
        },
        download_url: "https://github.com/owner/repository/releases/download/v{version}/app-{version}-{platform}-{arch}.tar.gz".to_string(),
        version_check: VersionCheck::GithubLatest {
            repo: "owner/repository".to_string(),
            include_prerelease: false,
        },
        check_interval_hours: 24,
        pre_update_command: Some("echo 'Starting update'".to_string()),
        post_update_command: Some("echo 'Update completed'".to_string()),
    };

    let toml_str = toml::to_string(&auto_update).unwrap();
    let deserialized: AutoUpdate = toml::from_str(&toml_str).unwrap();

    assert_eq!(deserialized.enabled, auto_update.enabled);
    assert_eq!(
        deserialized.check_interval_hours,
        auto_update.check_interval_hours
    );

    if let UpdateProvider::Github {
        repo,
        asset_pattern,
        include_prerelease,
    } = deserialized.provider
    {
        assert_eq!(repo, "owner/repository");
        assert_eq!(asset_pattern, "app-{version}-{platform}-{arch}.tar.gz");
        assert!(!include_prerelease);
    } else {
        panic!("Expected Github provider");
    }
}

#[test]
fn test_auto_update_https_provider() {
    let auto_update = AutoUpdate {
        enabled: true,
        provider: UpdateProvider::Https {
            base_url: "https://releases.example.com".to_string(),
            version_url: Some("https://releases.example.com/latest".to_string()),
        },
        download_url: "https://releases.example.com/v{version}/app-{platform}.zip".to_string(),
        version_check: VersionCheck::Http {
            url: "https://releases.example.com/latest".to_string(),
            json_path: Some("version".to_string()),
            regex_pattern: None,
        },
        check_interval_hours: 1,
        pre_update_command: None,
        post_update_command: None,
    };

    let toml_str = toml::to_string(&auto_update).unwrap();
    let deserialized: AutoUpdate = toml::from_str(&toml_str).unwrap();

    assert!(matches!(
        deserialized.version_check,
        VersionCheck::Http { .. }
    ));
    assert!(deserialized.pre_update_command.is_none());
    assert!(deserialized.post_update_command.is_none());

    if let UpdateProvider::Https {
        base_url,
        version_url,
    } = deserialized.provider
    {
        assert_eq!(base_url, "https://releases.example.com");
        assert_eq!(
            version_url,
            Some("https://releases.example.com/latest".to_string())
        );
    } else {
        panic!("Expected Https provider");
    }
}

#[test]
fn test_auto_update_custom_provider() {
    let auto_update = AutoUpdate {
        enabled: false,
        provider: UpdateProvider::Custom {
            update_command: "custom-updater --install {version}".to_string(),
            version_command: "custom-checker --latest".to_string(),
        },
        download_url: "https://custom.example.com/download/{version}".to_string(),
        version_check: VersionCheck::Command {
            command: "custom-checker".to_string(),
            args: vec!["--latest".to_string()],
        },
        check_interval_hours: 2,
        pre_update_command: Some("systemctl stop myapp".to_string()),
        post_update_command: Some("systemctl start myapp".to_string()),
    };

    let toml_str = toml::to_string(&auto_update).unwrap();
    let deserialized: AutoUpdate = toml::from_str(&toml_str).unwrap();

    assert!(!deserialized.enabled);
    assert_eq!(deserialized.check_interval_hours, 2);

    if let UpdateProvider::Custom {
        update_command,
        version_command,
    } = deserialized.provider
    {
        assert_eq!(update_command, "custom-updater --install {version}");
        assert_eq!(version_command, "custom-checker --latest");
    } else {
        panic!("Expected Custom provider");
    }

    if let VersionCheck::Command { command, args } = deserialized.version_check {
        assert_eq!(command, "custom-checker");
        assert_eq!(args, vec!["--latest"]);
    } else {
        panic!("Expected Command version check");
    }
}

#[test]
fn test_complete_config_serialization() {
    let mut env_vars = HashMap::new();
    env_vars.insert("APP_ENV".to_string(), "production".to_string());
    env_vars.insert("LOG_LEVEL".to_string(), "info".to_string());
    env_vars.insert(
        "CONFIG_PATH".to_string(),
        "/etc/myapp/config.json".to_string(),
    );

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
            template: Some(vec!["--global-config".to_string(), "{args}".to_string(), "--log-format".to_string(), "json".to_string()]),
            inline: Some("--verbose {args}".to_string()),
            mode: ArgsMode::Template,
            default: vec![],
            prefix: vec!["--prefix".to_string()],
            suffix: vec!["--suffix".to_string()],
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
            provider: UpdateProvider::Github {
                repo: "mycompany/myapp".to_string(),
                asset_pattern: "myapp-{version}-{platform}-{arch}.tar.gz".to_string(),
                include_prerelease: false,
            },
            download_url: "https://github.com/mycompany/myapp/releases/download/v{version}/myapp-{version}-{platform}-{arch}.tar.gz".to_string(),
            version_check: VersionCheck::GithubLatest {
                repo: "mycompany/myapp".to_string(),
                include_prerelease: false,
            },
            check_interval_hours: 12,
            pre_update_command: Some("echo 'Preparing for update'".to_string()),
            post_update_command: Some("echo 'Update completed successfully'".to_string()),
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
    assert_eq!(deserialized.args.template, config.args.template);
    assert_eq!(deserialized.env.len(), 3);
    assert_eq!(deserialized.metadata.tags.len(), 3);
    assert!(deserialized.auto_update.is_some());
}
