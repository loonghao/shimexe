# shimexe

[![CI](https://github.com/loonghao/shimexe/workflows/CI/badge.svg)](https://github.com/loonghao/shimexe/actions)
[![Crates.io](https://img.shields.io/crates/v/shimexe.svg)](https://crates.io/crates/shimexe)
[![Documentation](https://docs.rs/shimexe/badge.svg)](https://docs.rs/shimexe)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/loonghao/shimexe#license)

[中文文档](README_zh.md)

A modern, cross-platform executable shim manager with dynamic template system and enhanced argument handling capabilities.

## Features

- 🚀 **Cross-platform**: Works on Windows, macOS, and Linux
- 📝 **TOML Configuration**: Human-readable configuration files
- 🔧 **Environment Variable Expansion**: Support for `${VAR:default}` syntax
- 🎯 **Single Binary**: All functionality in one executable
- 📦 **Package Manager Ready**: Available on crates.io and Chocolatey
- 🔗 **API Library**: Use as a crate in your own projects
- 🎨 **Custom Icon**: Beautiful SVG-based icon embedded in executables

## Installation

### From Crates.io

```bash
cargo install shimexe
```

### From Chocolatey (Windows)

```powershell
choco install shimexe
```

### From GitHub Releases

Download the latest binary from [GitHub Releases](https://github.com/loonghao/shimexe/releases).

## Quick Start

1. Initialize shimexe:
   ```bash
   shimexe init --examples
   ```

2. Add a new shim:
   ```bash
   shimexe add rust --path "${RUST_HOME:~/.cargo/bin}/rustc${EXE_EXT:.exe}" --args "--version"
   ```

3. List all shims:
   ```bash
   shimexe list --detailed
   ```

4. Run your shim:
   ```bash
   rust
   ```

## Configuration Format

Shims are configured using TOML files with the `.shim.toml` extension:

```toml
[shim]
name = "rust"
path = "${RUST_HOME:~/.cargo/bin}/rustc${EXE_EXT:.exe}"
args = ["--version"]
cwd = "/optional/working/directory"

[env]
RUST_LOG = "info"
CARGO_HOME = "${CARGO_HOME:~/.cargo}"

[metadata]
description = "Rust compiler shim"
version = "1.0.0"
author = "Your Name"
tags = ["rust", "compiler"]
```

## Environment Variable Expansion

shimexe supports powerful environment variable expansion:

- `${VAR}` - Expand VAR, error if not found
- `${VAR:default}` - Expand VAR, use default if not found
- Built-in variables:
  - `${EXE_EXT}` - Platform-specific executable extension (`.exe` on Windows)
  - `${PATH_SEP}` - Platform-specific path separator
  - `${HOME}` - User home directory
  - `${CONFIG_DIR}` - User configuration directory

## CLI Commands

```bash
# Add a new shim
shimexe add <name> --path <executable> [--args <args>] [--env KEY=VALUE]

# Remove a shim
shimexe remove <name> [--force]

# List all shims
shimexe list [--detailed]

# Update an existing shim
shimexe update <name> [--path <path>] [--args <args>]

# Validate a shim configuration
shimexe validate <shim-file>

# Initialize shimexe
shimexe init [--examples]
```

## Using as a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
shimexe-core = "0.1"
```

Example usage:

```rust
use shimexe_core::prelude::*;

// Load and run a shim
let runner = ShimRunner::from_file("my-app.shim.toml")?;
let exit_code = runner.execute(&["--help"])?;

// Create a shim programmatically
let config = ShimConfig {
    shim: ShimCore {
        name: "my-tool".to_string(),
        path: "/usr/bin/my-tool".to_string(),
        args: vec!["--default-arg".to_string()],
        cwd: None,
    },
    env: HashMap::new(),
    metadata: Default::default(),
};

config.to_file("my-tool.shim.toml")?;
```

## Integration Examples

### With vx

```rust
use shimexe_core::ShimRunner;

// In your vx integration
let shim_path = format!("{}.shim.toml", tool_name);
let runner = ShimRunner::from_file(&shim_path)?;
runner.execute(&args)?;
```

### With rye

```rust
use shimexe_core::{ShimConfig, ShimCore};

// Create Python tool shims
let config = ShimConfig {
    shim: ShimCore {
        name: "python".to_string(),
        path: "${RYE_HOME}/shims/python${EXE_EXT}".to_string(),
        args: vec![],
        cwd: None,
    },
    env: rye_env_vars(),
    metadata: Default::default(),
};
```

## Building with Icon

shimexe includes a beautiful custom icon that gets embedded into the Windows executable. The build process automatically handles icon generation:

1. **Automatic Generation**: If you have ImageMagick installed, the build script will automatically convert `assets/icon.svg` to `assets/icon.ico`
2. **Manual Generation**: You can also generate the icon manually:
   ```bash
   # Install ImageMagick first
   winget install ImageMagick.ImageMagick

   # Generate icon
   magick convert -background transparent -define icon:auto-resize=256,128,64,48,32,16 assets/icon.svg assets/icon.ico
   ```
3. **CI/CD**: GitHub Actions automatically installs ImageMagick and generates icons for all release builds

The icon represents shimexe's core concept: a central hub (the shim manager) connecting to multiple executables, with animated data flow indicators showing the dynamic nature of the tool.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

Inspired by:
- [Scoop](https://scoop.sh/) - Windows package manager
- [scoop-better-shimexe](https://github.com/71/scoop-better-shimexe) - Improved Scoop shim implementation
- [vx](https://github.com/loonghao/vx) - Version management tool
- [rye](https://github.com/astral-sh/rye) - Python project management
