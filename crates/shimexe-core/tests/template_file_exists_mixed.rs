// Mixed true/false for file_exists in a single template

use shimexe_core::template::{ArgsConfig, ArgsMode, TemplateEngine};

#[test]
fn test_file_exists_mixed_tokens() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    let path = tmp.path().to_string_lossy().to_string();

    let mut engine = TemplateEngine::new(vec![]);
    let cfg = ArgsConfig {
        mode: ArgsMode::Template,
        template: Some(vec![
            "--check".into(),
            format!("{{{{file_exists('{path}')}}}}"),
            "{{file_exists('definitely_not_here_123')}}".into(),
        ]),
        ..Default::default()
    };

    let out = engine.process_args(&cfg).unwrap();
    assert_eq!(out, vec!["--check", "true", "false"]);
}

