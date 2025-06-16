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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load shim configuration
    let config = ShimConfig::from_file("path/to/shim.toml")?;

    // Create and run shim
    let runner = ShimRunner::from_config(config)?;
    let exit_code = runner.execute(&["arg1", "arg2"])?;

    std::process::exit(exit_code);
}
```

### Configuration Example

```toml
# shim.toml
[shim]
name = "my-tool"
path = "${HOME}/bin/my-tool"
args = ["--config", "${CONFIG_DIR}/config.yaml"]
cwd = "/optional/working/directory"

[env]
PATH = "${PATH}:${HOME}/bin"
MY_TOOL_HOME = "${HOME}/.my-tool"

[metadata]
description = "My awesome tool"
version = "1.0.0"
author = "Your Name"
tags = ["tool", "utility"]
```

### Creating Shim Configurations

```rust
use shimexe_core::{ShimConfig, ShimCore};
use std::collections::HashMap;

// Create a shim programmatically
let config = ShimConfig {
    shim: ShimCore {
        name: "my-tool".to_string(),
        path: "/usr/bin/my-tool".to_string(),
        args: vec!["--default-arg".to_string()],
        cwd: Some("/working/directory".to_string()),
    },
    env: {
        let mut env = HashMap::new();
        env.insert("MY_VAR".to_string(), "value".to_string());
        env
    },
    metadata: Default::default(),
    args: Default::default(),
    auto_update: None,
};

// Save to file
config.to_file("my-tool.shim.toml")?;
```

### Environment Variable Expansion

```rust
use shimexe_core::utils::expand_env_vars;

// Expand environment variables
let expanded = expand_env_vars("${HOME}/bin/${EXE_EXT}")?;

// Built-in variables:
// ${EXE_EXT} - Platform-specific executable extension (.exe on Windows)
// ${PATH_SEP} - Platform-specific path separator
// ${HOME} - User home directory
// ${CONFIG_DIR} - User configuration directory
```

## API Documentation

For detailed API documentation, visit [docs.rs/shimexe-core](https://docs.rs/shimexe-core).

## Contributing

Contributions are welcome! Please see the [main repository](../../README.md) for contribution guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE-MIT](../../LICENSE-MIT) file for details.
