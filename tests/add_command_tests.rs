use shimexe_core::Downloader;
use std::collections::HashMap;

#[test]
fn test_url_detection_and_parsing() {
    let path = "https://example.com/tool.exe";

    // Test URL detection and parsing logic
    assert!(Downloader::is_url(path));
    assert_eq!(
        Downloader::extract_filename_from_url(path),
        Some("tool.exe".to_string())
    );
    assert_eq!(
        Downloader::infer_app_name_from_url(path),
        Some("tool".to_string())
    );
}

#[test]
fn test_env_parsing() {
    let env_vars_input = vec![
        "KEY1=value1".to_string(),
        "KEY2=value2".to_string(),
        "PATH=/usr/bin:/bin".to_string(),
    ];

    // Test environment variable parsing logic
    let mut env_vars = HashMap::new();
    for env_var in &env_vars_input {
        if let Some((key, value)) = env_var.split_once('=') {
            env_vars.insert(key.to_string(), value.to_string());
        }
    }

    assert_eq!(env_vars.get("KEY1"), Some(&"value1".to_string()));
    assert_eq!(env_vars.get("KEY2"), Some(&"value2".to_string()));
    assert_eq!(env_vars.get("PATH"), Some(&"/usr/bin:/bin".to_string()));
}
