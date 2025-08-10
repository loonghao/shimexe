// Negative branch for TemplateEngine if condition

use shimexe_core::template::TemplateEngine;

#[test]
fn test_if_condition_negative() {
    // Ensure ENVY is not set
    std::env::remove_var("ENVY");

    let mut engine = TemplateEngine::new(vec![]);
    let out = engine
        .render_template("cond={{if env('ENVY') == '1'}}")
        .unwrap();
    assert_eq!(out, "cond=false");
}
