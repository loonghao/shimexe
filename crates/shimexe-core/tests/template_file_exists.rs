// Test TemplateEngine file_exists() returning true using a temp file

use shimexe_core::template::TemplateEngine;

#[test]
fn test_template_file_exists_true() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    let path = tmp.path().to_string_lossy().to_string();

    let mut engine = TemplateEngine::new(vec![]);
    // Build template string with literal {{ ... }} without interfering with format! braces
    let escaped = path.replace('\\', "\\\\");
    let template = "exists {{file_exists('__P__')}}".replace("__P__", &escaped);
    let out = engine.render_template(&template).unwrap();
    assert_eq!(out.trim(), "exists true");
}

