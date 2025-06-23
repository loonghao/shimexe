use thiserror::Error;

/// Result type alias for shimexe operations
pub type Result<T> = std::result::Result<T, ShimError>;

/// Error types for shimexe operations
#[derive(Error, Debug)]
pub enum ShimError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("Environment variable expansion error: {0}")]
    EnvExpansion(String),

    #[error("Shim configuration error: {0}")]
    Config(String),

    #[error("Executable not found: {0}")]
    ExecutableNotFound(String),

    #[error("Process execution error: {0}")]
    ProcessExecution(String),

    #[error("Invalid shim file: {0}")]
    InvalidShimFile(String),

    #[error("Shim not found: {0}")]
    ShimNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Template processing error: {0}")]
    TemplateError(String),

    #[error("Download error: {0}")]
    Download(#[from] anyhow::Error),
}
