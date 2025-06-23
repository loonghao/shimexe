//! # shimexe-core
//!
//! Core library for shimexe - a modern, cross-platform executable shim manager
//! with environment variable expansion and TOML configuration support.

pub mod archive;
pub mod config;
pub mod downloader;
pub mod error;
pub mod manager;
pub mod runner;
pub mod template;
pub mod traits;
pub mod updater;
pub mod utils;



pub use archive::ArchiveExtractor;
pub use config::{
    AutoUpdate, ExtractedExecutable, ShimConfig, ShimCore, ShimMetadata, SourceType,
    UpdateProvider, VersionCheck,
};
pub use downloader::Downloader;
pub use error::{Result, ShimError};
pub use manager::{ShimBuilder, ShimInfo, ShimManager};
pub use runner::ShimRunner;
pub use template::{ArgsConfig, ArgsMode, TemplateEngine};
pub use traits::{CustomizableShimRunner, ShimConfigLoader, ShimRunnerBuilder, ShimRunnerTrait};
pub use updater::ShimUpdater;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{
        ArgsConfig, ArgsMode, Result, ShimBuilder, ShimConfig, ShimError, ShimInfo, ShimManager,
        ShimRunner, TemplateEngine,
    };
}
