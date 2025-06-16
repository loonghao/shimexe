//! # shimexe-core
//!
//! Core library for shimexe - a modern, cross-platform executable shim manager
//! with environment variable expansion and TOML configuration support.

pub mod config;
pub mod error;
pub mod runner;
pub mod template;
pub mod traits;
pub mod updater;
pub mod utils;

pub use config::{AutoUpdate, ShimConfig, ShimCore, ShimMetadata, UpdateProvider, VersionCheck};
pub use error::{Result, ShimError};
pub use runner::ShimRunner;
pub use template::{ArgsConfig, ArgsMode, TemplateEngine};
pub use traits::{CustomizableShimRunner, ShimConfigLoader, ShimRunnerBuilder, ShimRunnerTrait};
pub use updater::ShimUpdater;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{
        ArgsConfig, ArgsMode, Result, ShimConfig, ShimError, ShimRunner, TemplateEngine,
    };
}
