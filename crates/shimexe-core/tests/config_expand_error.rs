// Negative path: expand_env_vars error on unclosed ${

use shimexe_core::config::ShimConfig;
use shimexe_core::config::ShimCore;
use shimexe_core::config::SourceType;

#[test]
fn test_expand_env_vars_error_unclosed_brace() {
    let mut cfg = ShimConfig {
        shim: ShimCore {
            name: "bad".into(),
            path: "${UNCLOSED".into(),
            args: vec!["${ALSO_BAD".into()],
            cwd: Some("${BROKEN".into()),
            download_url: None,
            source_type: SourceType::File,
            extracted_executables: vec![],
        },
        args: Default::default(),
        env: Default::default(),
        metadata: Default::default(),
        auto_update: None,
    };

    let err = cfg.expand_env_vars().unwrap_err();
    let msg = format!("{}", err);
    assert!(msg.contains("Unclosed ${") || msg.contains("EnvExpansion"));
}

