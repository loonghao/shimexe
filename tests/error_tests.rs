use shimexe_core::error::{Result, ShimError};
use std::error::Error;
use std::io;

#[test]
fn test_error_display() {
    let io_error = ShimError::Io(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    assert!(io_error.to_string().contains("IO error"));
    assert!(io_error.to_string().contains("file not found"));

    let config_error = ShimError::Config("invalid configuration".to_string());
    assert_eq!(
        config_error.to_string(),
        "Shim configuration error: invalid configuration"
    );

    let executable_error = ShimError::ExecutableNotFound("test.exe".to_string());
    assert_eq!(
        executable_error.to_string(),
        "Executable not found: test.exe"
    );

    let process_error = ShimError::ProcessExecution("command failed".to_string());
    assert_eq!(
        process_error.to_string(),
        "Process execution error: command failed"
    );

    let shim_error = ShimError::ShimNotFound("test-shim".to_string());
    assert_eq!(shim_error.to_string(), "Shim not found: test-shim");

    let permission_error = ShimError::PermissionDenied("access denied".to_string());
    assert_eq!(
        permission_error.to_string(),
        "Permission denied: access denied"
    );

    let template_error = ShimError::TemplateError("template parsing failed".to_string());
    assert_eq!(
        template_error.to_string(),
        "Template processing error: template parsing failed"
    );

    let env_error = ShimError::EnvExpansion("variable not found".to_string());
    assert_eq!(
        env_error.to_string(),
        "Environment variable expansion error: variable not found"
    );

    let invalid_shim_error = ShimError::InvalidShimFile("corrupted shim".to_string());
    assert_eq!(
        invalid_shim_error.to_string(),
        "Invalid shim file: corrupted shim"
    );
}

#[test]
fn test_error_from_conversions() {
    // Test IO error conversion
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let shim_err: ShimError = io_err.into();
    assert!(matches!(shim_err, ShimError::Io(_)));

    // Test anyhow error conversion
    let anyhow_err = anyhow::anyhow!("download failed");
    let shim_err: ShimError = anyhow_err.into();
    assert!(matches!(shim_err, ShimError::Download(_)));
}

#[test]
fn test_result_type_alias() {
    fn test_function() -> Result<String> {
        Ok("success".to_string())
    }

    fn test_error_function() -> Result<String> {
        Err(ShimError::Config("test error".to_string()))
    }

    assert!(test_function().is_ok());
    assert!(test_error_function().is_err());

    match test_error_function() {
        Err(ShimError::Config(msg)) => assert_eq!(msg, "test error"),
        _ => panic!("Expected Config error"),
    }
}

#[test]
fn test_error_debug_format() {
    let error = ShimError::Config("debug test".to_string());
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("Config"));
    assert!(debug_str.contains("debug test"));
}

#[test]
fn test_error_chain() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "original error");
    let shim_error = ShimError::Io(io_error);

    // Test that the error chain is preserved
    assert!(shim_error.to_string().contains("original error"));

    // Test source error access
    let source = shim_error.source();
    assert!(source.is_some());
}

#[test]
fn test_all_error_variants() {
    // Test all error variants to ensure they're properly constructed
    let errors = vec![
        ShimError::Config("config".to_string()),
        ShimError::ExecutableNotFound("exe".to_string()),
        ShimError::ProcessExecution("process".to_string()),
        ShimError::ShimNotFound("shim".to_string()),
        ShimError::PermissionDenied("permission".to_string()),
        ShimError::TemplateError("template".to_string()),
        ShimError::EnvExpansion("env".to_string()),
        ShimError::InvalidShimFile("invalid".to_string()),
    ];

    for error in errors {
        // Ensure all errors can be displayed and debugged
        let _display = error.to_string();
        let _debug = format!("{:?}", error);
    }
}
