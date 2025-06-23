use shimexe_core::{ShimConfig, ShimCore, ShimRunner, SourceType};
use std::collections::HashMap;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_runner_from_config() {
    let config = ShimConfig {
        shim: ShimCore {
            name: "test".to_string(),
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
    };

    let runner = ShimRunner::from_config(config).unwrap();
    assert_eq!(runner.config().shim.name, "test");
}

#[test]
fn test_runner_from_file() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(
        temp_file,
        r#"
[shim]
name = "test"
path = "echo"
args = ["hello"]

[env]
TEST_VAR = "test_value"
        "#
    )
    .unwrap();

    let runner = ShimRunner::from_file(temp_file.path()).unwrap();
    assert_eq!(runner.config().shim.name, "test");
    assert_eq!(
        runner.config().env.get("TEST_VAR"),
        Some(&"test_value".to_string())
    );
}
