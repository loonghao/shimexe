// Test args('default') behavior with and without user args

use shimexe_core::template::{ArgsConfig, ArgsMode, TemplateEngine};

#[test]
fn test_args_default_when_user_args_empty() {
    let mut engine = TemplateEngine::new(vec![]);
    let cfg = ArgsConfig {
        mode: ArgsMode::Template,
        template: Some(vec!["{{args('--default')}}".into()]),
        ..Default::default()
    };
    let out = engine.process_args(&cfg).unwrap();
    assert_eq!(out, vec!["--default"]);
}

#[test]
fn test_args_default_when_user_args_present() {
    let mut engine = TemplateEngine::new(vec!["--help".into()]);
    let cfg = ArgsConfig {
        mode: ArgsMode::Template,
        template: Some(vec!["{{args('--default')}}".into()]),
        ..Default::default()
    };
    let out = engine.process_args(&cfg).unwrap();
    assert_eq!(out, vec!["--help"]);
}
