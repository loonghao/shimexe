# shimexe-core

[![Crates.io](https://img.shields.io/crates/v/shimexe-core.svg)](https://crates.io/crates/shimexe-core)
[![Documentation](https://docs.rs/shimexe-core/badge.svg)](https://docs.rs/shimexe-core)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSE-MIT)

Core library for shimexe - a modern, cross-platform executable shim manager with environment variable expansion and TOML configuration support.

## Overview

`shimexe-core` provides the foundational functionality for creating and managing executable shims. It offers a flexible and extensible architecture for building shim management tools.

## Features

- **Cross-platform support**: Works on Windows, macOS, and Linux
- **TOML configuration**: Easy-to-read and write configuration format
- **Environment variable expansion**: Dynamic path resolution with shell-style variable expansion
- **Template engine**: Flexible argument and environment variable templating
- **Auto-update support**: Built-in mechanisms for keeping shims up-to-date
- **Extensible architecture**: Trait-based design for customization
- **Async support**: Built with async/await for better performance

## Core Components

### ShimConfig
Manages shim configuration including target executable paths, arguments, and environment variables.

### ShimRunner
Executes shims with proper argument handling and environment setup.

### TemplateEngine
Provides variable expansion and templating for dynamic configuration.

### ShimUpdater
Handles automatic updates for shim configurations and target executables.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
shimexe-core = "0.1.0"
```

### Basic Example

```rust
use shimexe_core::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Load shim configuration
    let config = ShimConfig::from_file("path/to/shim.toml")?;
    
    // Create and run shim
    let runner = ShimRunner::new(config);
    let exit_code = runner.run(&["arg1", "arg2"]).await?;
    
    std::process::exit(exit_code);
}
```

### Configuration Example

```toml
# shim.toml
[shim]
target = "${HOME}/bin/my-tool"
args = ["--config", "${CONFIG_DIR}/config.yaml"]

[shim.env]
PATH = "${PATH}:${HOME}/bin"
MY_TOOL_HOME = "${HOME}/.my-tool"

[shim.metadata]
name = "my-tool"
version = "1.0.0"
description = "My awesome tool"
```

### Advanced Usage

```rust
use shimexe_core::*;

// Custom shim runner with additional features
struct CustomRunner {
    inner: ShimRunner,
}

impl CustomizableShimRunner for CustomRunner {
    fn pre_run(&mut self, args: &[String]) -> Result<()> {
        // Custom pre-execution logic
        println!("Running with args: {:?}", args);
        Ok(())
    }
    
    fn post_run(&mut self, exit_code: i32) -> Result<()> {
        // Custom post-execution logic
        println!("Finished with exit code: {}", exit_code);
        Ok(())
    }
}
```

## API Documentation

For detailed API documentation, visit [docs.rs/shimexe-core](https://docs.rs/shimexe-core).

## Contributing

Contributions are welcome! Please see the [main repository](../../README.md) for contribution guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE-MIT](../../LICENSE-MIT) file for details.
