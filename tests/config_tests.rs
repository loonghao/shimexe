use shimexe_core::{ExtractedExecutable, ShimConfig, ShimCore, SourceType};
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
