use shimexe_core::template::{ArgsConfig, ArgsMode, TemplateEngine};
use std::env;

#[test]
fn test_args_config_default() {
    let config = ArgsConfig::default();
    assert!(config.template.is_none());
    assert!(config.inline.is_none());
    assert_eq!(config.mode, ArgsMode::Template);
    assert!(config.default.is_empty());
    assert!(config.prefix.is_empty());
    assert!(config.suffix.is_empty());
}

#[test]
fn test_args_mode_serialization() {
    // Test that ArgsMode can be serialized/deserialized with TOML
    let modes = vec![
        (ArgsMode::Template, "template"),
        (ArgsMode::Merge, "merge"),
        (ArgsMode::Replace, "replace"),
        (ArgsMode::Prepend, "prepend"),
    ];

    for (mode, expected_str) in modes {
        let serialized = toml::to_string(&mode).unwrap();
        assert!(serialized.contains(expected_str));
        let deserialized: ArgsMode = toml::from_str(&serialized).unwrap();
        assert_eq!(mode, deserialized);
    }
}

#[test]
fn test_template_engine_new() {
    let user_args = vec!["arg1".to_string(), "arg2".to_string()];
    // We can't directly access user_args, but we can test through process_args

    let config = ArgsConfig::default();
    let mut engine = TemplateEngine::new(user_args.clone());
    let result = engine.process_args(&config).unwrap();
    assert_eq!(result, user_args);
}

#[test]
fn test_process_args_template_mode_no_template() {
    let user_args = vec!["user1".to_string(), "user2".to_string()];
    let mut engine = TemplateEngine::new(user_args.clone());

    let config = ArgsConfig {
        mode: ArgsMode::Template,
        ..Default::default()
    };

    let result = engine.process_args(&config).unwrap();
    assert_eq!(result, user_args);
}

#[test]
fn test_process_args_merge_mode() {
    let user_args = vec!["user1".to_string(), "user2".to_string()];
    let mut engine = TemplateEngine::new(user_args.clone());

    let config = ArgsConfig {
        mode: ArgsMode::Merge,
        prefix: vec!["prefix".to_string()],
        default: vec!["default1".to_string(), "default2".to_string()],
        suffix: vec!["suffix".to_string()],
        ..Default::default()
    };

    let result = engine.process_args(&config).unwrap();
    let expected = vec![
        "prefix".to_string(),
        "default1".to_string(),
        "default2".to_string(),
        "user1".to_string(),
        "user2".to_string(),
        "suffix".to_string(),
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_process_args_replace_mode_with_user_args() {
    let user_args = vec!["user1".to_string(), "user2".to_string()];
    let mut engine = TemplateEngine::new(user_args.clone());

    let config = ArgsConfig {
        mode: ArgsMode::Replace,
        prefix: vec!["prefix".to_string()],
        default: vec!["default1".to_string(), "default2".to_string()],
        suffix: vec!["suffix".to_string()],
        ..Default::default()
    };

    let result = engine.process_args(&config).unwrap();
    let expected = vec![
        "prefix".to_string(),
        "user1".to_string(),
        "user2".to_string(),
        "suffix".to_string(),
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_process_args_replace_mode_no_user_args() {
    let user_args = vec![];
    let mut engine = TemplateEngine::new(user_args);

    let config = ArgsConfig {
        mode: ArgsMode::Replace,
        prefix: vec!["prefix".to_string()],
        default: vec!["default1".to_string(), "default2".to_string()],
        suffix: vec!["suffix".to_string()],
        ..Default::default()
    };

    let result = engine.process_args(&config).unwrap();
    let expected = vec![
        "prefix".to_string(),
        "default1".to_string(),
        "default2".to_string(),
        "suffix".to_string(),
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_process_args_prepend_mode() {
    let user_args = vec!["user1".to_string(), "user2".to_string()];
    let mut engine = TemplateEngine::new(user_args.clone());

    let config = ArgsConfig {
        mode: ArgsMode::Prepend,
        prefix: vec!["prefix".to_string()],
        default: vec!["default1".to_string(), "default2".to_string()],
        suffix: vec!["suffix".to_string()],
        ..Default::default()
    };

    let result = engine.process_args(&config).unwrap();
    let expected = vec![
        "prefix".to_string(),
        "user1".to_string(),
        "user2".to_string(),
        "default1".to_string(),
        "default2".to_string(),
        "suffix".to_string(),
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_process_args_template_mode_with_template() {
    let user_args = vec!["file.txt".to_string()];
    let mut engine = TemplateEngine::new(user_args);

    let config = ArgsConfig {
        mode: ArgsMode::Template,
        template: Some(vec![
            "--input".to_string(),
            "{{args}}".to_string(),
            "--output".to_string(),
            "output.txt".to_string(),
        ]),
        ..Default::default()
    };

    let result = engine.process_args(&config).unwrap();
    let expected = vec![
        "--input".to_string(),
        "file.txt".to_string(),
        "--output".to_string(),
        "output.txt".to_string(),
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_process_args_inline_template() {
    let user_args = vec!["input.txt".to_string()];
    let mut engine = TemplateEngine::new(user_args);

    let config = ArgsConfig {
        mode: ArgsMode::Template,
        inline: Some("--file {{args}} --verbose".to_string()),
        ..Default::default()
    };

    let result = engine.process_args(&config).unwrap();
    let expected = vec![
        "--file".to_string(),
        "input.txt".to_string(),
        "--verbose".to_string(),
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_render_template_basic() {
    let user_args = vec!["test".to_string()];
    let mut engine = TemplateEngine::new(user_args);

    let result = engine.render_template("{{args}}").unwrap();
    assert_eq!(result, "test");

    let result = engine.render_template("prefix_{{args}}_suffix").unwrap();
    assert_eq!(result, "prefix_test_suffix");
}

#[test]
fn test_render_template_with_env_vars() {
    env::set_var("SHIMEXE_TEMPLATE_TEST", "env_value");

    let user_args = vec![];
    let mut engine = TemplateEngine::new(user_args);

    let result = engine
        .render_template("{{env('SHIMEXE_TEMPLATE_TEST')}}")
        .unwrap();
    assert_eq!(result, "env_value");

    env::remove_var("SHIMEXE_TEMPLATE_TEST");
}

#[test]
fn test_render_template_multiple_expressions() {
    let user_args = vec!["arg1".to_string(), "arg2".to_string()];
    let mut engine = TemplateEngine::new(user_args);

    let result = engine.render_template("{{args}}_{{args}}").unwrap();
    assert_eq!(result, "arg1 arg2_arg1 arg2");
}

#[test]
fn test_render_template_no_expressions() {
    let user_args = vec![];
    let mut engine = TemplateEngine::new(user_args);

    let result = engine.render_template("no expressions here").unwrap();
    assert_eq!(result, "no expressions here");
}

#[test]
fn test_args_config_serialization() {
    let config = ArgsConfig {
        template: Some(vec!["--arg".to_string(), "{{args}}".to_string()]),
        inline: Some("--inline {{args}}".to_string()),
        mode: ArgsMode::Merge,
        default: vec!["default".to_string()],
        prefix: vec!["prefix".to_string()],
        suffix: vec!["suffix".to_string()],
    };

    // Test TOML serialization
    let toml_str = toml::to_string(&config).unwrap();
    let deserialized: ArgsConfig = toml::from_str(&toml_str).unwrap();

    assert_eq!(config.template, deserialized.template);
    assert_eq!(config.inline, deserialized.inline);
    assert_eq!(config.mode, deserialized.mode);
    assert_eq!(config.default, deserialized.default);
    assert_eq!(config.prefix, deserialized.prefix);
    assert_eq!(config.suffix, deserialized.suffix);
}

#[test]
fn test_empty_template_handling() {
    let user_args = vec!["test".to_string()];
    let mut engine = TemplateEngine::new(user_args);

    let config = ArgsConfig {
        mode: ArgsMode::Template,
        template: Some(vec![]),
        ..Default::default()
    };

    let result = engine.process_args(&config).unwrap();
    assert!(result.is_empty());
}

#[test]
fn test_whitespace_splitting_in_template() {
    let user_args = vec!["file.txt".to_string()];
    let mut engine = TemplateEngine::new(user_args);

    let config = ArgsConfig {
        mode: ArgsMode::Template,
        template: Some(vec!["--input {{args}} --verbose".to_string()]),
        ..Default::default()
    };

    let result = engine.process_args(&config).unwrap();
    let expected = vec![
        "--input".to_string(),
        "file.txt".to_string(),
        "--verbose".to_string(),
    ];
    assert_eq!(result, expected);
}
