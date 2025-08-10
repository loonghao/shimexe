// 2025 tech: Rust stable. Focus: expand TemplateEngine branches.

use shimexe_core::template::{ArgsConfig, ArgsMode, TemplateEngine};

#[test]
fn test_args_mode_prepend() {
    let mut engine = TemplateEngine::new(vec!["u1".into(), "u2".into()]);
    let cfg = ArgsConfig {
        mode: ArgsMode::Prepend,
        default: vec!["d1".into()],
        prefix: vec!["p".into()],
        suffix: vec!["s".into()],
        ..Default::default()
    };
    let out = engine.process_args(&cfg).unwrap();
    assert_eq!(out, vec!["p", "u1", "u2", "d1", "s"]);
}

#[test]
fn test_inline_template_render() {
    let mut engine = TemplateEngine::new(vec!["--flag".into()]);
    let cfg = ArgsConfig {
        mode: ArgsMode::Template,
        inline: Some("run {{args('--default')}}".into()),
        ..Default::default()
    };
    let out = engine.process_args(&cfg).unwrap();
    assert_eq!(out, vec!["run", "--flag"]);
}

#[test]
fn test_if_condition_in_template() {
    std::env::set_var("ENVX", "1");
    let mut engine = TemplateEngine::new(vec![]);
    let rendered = engine
        .render_template("cond={{if env('ENVX') == '1'}}")
        .unwrap();
    assert_eq!(rendered, "cond=true");
    std::env::remove_var("ENVX");
}
