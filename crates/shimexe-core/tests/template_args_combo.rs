// args('default') combined with prefix/suffix

use shimexe_core::template::{ArgsConfig, ArgsMode, TemplateEngine};

#[test]
fn test_args_default_with_prefix_suffix() {
    // user args empty → pick default
    let mut engine = TemplateEngine::new(vec![]);
    let cfg = ArgsConfig {
        mode: ArgsMode::Template,
        template: Some(vec!["{{args('--def')}}".into()]),
        ..Default::default()
    };
    let out = engine.process_args(&cfg).unwrap();
    // In Template mode, prefix/suffix are not applied; only template output is used
    assert_eq!(out, vec!["--def"]);

    // user args present → use user args (still no prefix/suffix in Template mode)
    let mut engine2 = TemplateEngine::new(vec!["--user".into()]);
    let out2 = engine2.process_args(&cfg).unwrap();
    assert_eq!(out2, vec!["--user"]);
}

