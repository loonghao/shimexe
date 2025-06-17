# shimexe

[![CI](https://github.com/loonghao/shimexe/workflows/CI/badge.svg)](https://github.com/loonghao/shimexe/actions)
[![Crates.io](https://img.shields.io/crates/v/shimexe.svg)](https://crates.io/crates/shimexe)
[![Documentation](https://docs.rs/shimexe/badge.svg)](https://docs.rs/shimexe)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/loonghao/shimexe#license)

[中文文档](README_zh.md)

A modern, cross-platform executable shim manager with HTTP URL download support, dynamic template system, and enhanced argument handling capabilities.

## Features

- 🚀 **Cross-platform**: Works on Windows, macOS, and Linux
- 🌐 **HTTP URL Support**: Download executables directly from URLs
- 📝 **TOML Configuration**: Human-readable configuration files
- 🔧 **Environment Variable Expansion**: Support for `${VAR:default}` syntax
- 🎯 **Single Binary**: All functionality in one executable
- 📦 **Package Manager Ready**: Available on crates.io and Chocolatey
- 🔗 **API Library**: Use as a crate in your own projects
- 🎨 **Custom Icon**: Beautiful SVG-based icon embedded in executables
- 🤖 **Smart Name Inference**: Automatically infer application names from URLs
- ⚡ **Auto-Download**: Missing executables are downloaded automatically at runtime
- 🔒 **Secure Downloads**: Uses rustls-tls for secure HTTPS connections

## Installation

### From Crates.io

```bash
cargo install shimexe
```

### From Chocolatey (Windows)

```powershell
choco install shimexe
```

### From Scoop (Windows)

```powershell
scoop install shimexe
```

### Quick Install Scripts

**Unix-like systems (macOS, Linux):**
```bash
curl -LsSf https://github.com/loonghao/shimexe/install.sh | sh
```

**Windows (PowerShell):**
```powershell
irm https://github.com/loonghao/shimexe/install.ps1 | iex
```

**Install specific version:**
```bash
# Unix
curl -LsSf https://github.com/loonghao/shimexe/0.2.1/install.sh | sh

# Windows
$env:SHIMEXE_VERSION="0.2.1"; irm https://github.com/loonghao/shimexe/install.ps1 | iex
```

### From GitHub Releases

Download the latest binary from [GitHub Releases](https://github.com/loonghao/shimexe/releases).

## Quick Start

### Traditional Local Executables

1. Initialize shimexe:
   ```bash
   shimexe init --examples
   ```

2. Add a local executable shim:
   ```bash
   shimexe add rust --path "${RUST_HOME:~/.cargo/bin}/rustc${EXE_EXT:.exe}" --args "--version"
   ```

### HTTP URL Downloads

1. Download and create shim with explicit name:
   ```bash
   shimexe add it --path https://github.com/loonghao/installer-analyzer/releases/download/v0.7.0/installer-analyzer.exe
   ```

2. Auto-infer name from URL:
   ```bash
   shimexe add --path https://example.com/tools/my-tool.exe
   # Creates 'my-tool' shim automatically
   ```

### Archive Support (New!)

shimexe now supports downloading and extracting archives (zip files) with automatic executable discovery:

1. Download and extract zip archive:
   ```bash
   shimexe add plz --path https://github.com/release-plz/release-plz/releases/download/release-plz-v0.3.135/release-plz-x86_64-pc-windows-msvc.zip
   ```

2. Auto-extract and create shims for all executables:
   ```bash
   shimexe add tools --path https://example.com/multi-tool-package.zip
   # Extracts archive and creates shims for all .exe files found
   ```

3. List all shims:
   ```bash
   shimexe list --detailed
   ```

4. Run your shim (auto-downloads and extracts if missing):
   ```bash
   plz --help
   ```

## Configuration Format

Shims are configured using TOML files with the `.shim.toml` extension:

### Local Executable Configuration

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

### HTTP URL Configuration

```toml
[shim]
name = "installer-analyzer"
path = "/home/user/.shimexe/installer-analyzer/bin/installer-analyzer.exe"
download_url = "https://github.com/loonghao/installer-analyzer/releases/download/v0.7.0/installer-analyzer.exe"
source_type = "url"
args = []
cwd = ""

[env]
# Optional environment variables

[metadata]
description = "Installer analyzer tool from GitHub"
version = "0.7.0"
author = "loonghao"
tags = ["installer", "analyzer", "tool"]
```

### Archive Configuration (New!)

```toml
[shim]
name = "release-plz"
path = "/home/user/.shimexe/release-plz/bin/release-plz.exe"
download_url = "https://github.com/release-plz/release-plz/releases/download/release-plz-v0.3.135/release-plz-x86_64-pc-windows-msvc.zip"
source_type = "archive"
args = []

# List of extracted executables from the archive
[[shim.extracted_executables]]
name = "release-plz"
path = "release-plz.exe"
full_path = "/home/user/.shimexe/release-plz/bin/release-plz.exe"
is_primary = true

[env]
# Optional environment variables

[metadata]
description = "Release Please tool from archive"
version = "0.3.135"
author = "release-plz team"
tags = ["release", "automation", "tool"]
```

**Note**: When using HTTP URLs or archives, shimexe automatically downloads and extracts to `~/.shimexe/<app>/bin/` and updates the path to point to the local file(s).

## Environment Variable Expansion

shimexe supports powerful environment variable expansion:

- `${VAR}` - Expand VAR, error if not found
- `${VAR:default}` - Expand VAR, use default if not found
- Built-in variables:
  - `${EXE_EXT}` - Platform-specific executable extension (`.exe` on Windows)
  - `${PATH_SEP}` - Platform-specific path separator
  - `${HOME}` - User home directory
  - `${CONFIG_DIR}` - User configuration directory

## How Shims Work

shimexe creates standalone executable shims that can work independently when distributed:

### Shim Architecture

1. **Dual Configuration**: Each shim consists of two files:
   - `<name>.exe` - The executable shim (copy of shimexe binary)
   - `<name>.shim.toml` - The configuration file

2. **Smart Configuration Lookup**: When a shim runs, it searches for configuration in this order:
   - **Local**: Same directory as the executable (for portable distribution)
   - **Default**: User's shim directory (`~/.shimexe/`)

3. **Portable Distribution**: Shims can be copied to any location along with their `.shim.toml` files and will work independently without requiring shimexe to be installed on the target system.

### Static Linking

shimexe is built with static linking to minimize runtime dependencies:
- **Windows**: Statically links MSVC runtime (`+crt-static`)
- **No External Dependencies**: Shims work without requiring additional DLLs or runtime installations

### Distribution Examples

```bash
# Create a shim
shimexe add mytool --path "/path/to/tool" --args "--default-flag"

# Copy both files for distribution
cp ~/.shimexe/mytool.exe ./dist/
cp ~/.shimexe/mytool.shim.toml ./dist/

# The shim now works independently in ./dist/
./dist/mytool.exe
```

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

### HTTP URL and Archive Examples

```bash
# Download executable with explicit name
shimexe add mytool --path https://github.com/user/repo/releases/download/v1.0/tool.exe

# Auto-infer name from URL (creates 'installer-analyzer' shim)
shimexe add --path https://github.com/loonghao/installer-analyzer/releases/download/v0.7.0/installer-analyzer.exe

# Download and extract zip archive (creates shims for all executables found)
shimexe add plz --path https://github.com/release-plz/release-plz/releases/download/release-plz-v0.3.135/release-plz-x86_64-pc-windows-msvc.zip

# Add arguments and environment variables
shimexe add analyzer --path https://example.com/tools/analyzer.exe --args "--verbose" --env "DEBUG=1"

# Force overwrite existing shim
shimexe add mytool --path https://example.com/new-tool.exe --force

# Download to custom shim directory
shimexe add --shim-dir ./my-tools --path https://example.com/tool.exe

# Archive with multiple executables (auto-detects and creates multiple shims)
shimexe add devtools --path https://example.com/development-tools.zip
# This might create: devtools-compiler, devtools-debugger, devtools-profiler shims
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

### HTTP URL Download Example

```rust
use shimexe_core::{Downloader, ShimConfig, ShimCore};

// Download and create shim programmatically
let downloader = Downloader::new();
let url = "https://github.com/user/repo/releases/download/v1.0/tool.exe";

// Infer app name from URL
let app_name = Downloader::infer_app_name_from_url(url).unwrap();
let filename = Downloader::extract_filename_from_url(url).unwrap();

// Generate download path
let download_path = Downloader::generate_download_path(
    &std::path::Path::new("~/.shimexe"),
    &app_name,
    &filename
);

// Download the file
downloader.download_file(url, &download_path).await?;

// Create shim configuration
let config = ShimConfig {
    shim: ShimCore {
        name: app_name,
        path: download_path.to_string_lossy().to_string(),
        args: vec![],
        cwd: None,
    },
    env: HashMap::new(),
    metadata: Default::default(),
};
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

This project is licensed under the MIT License - see the [LICENSE-MIT](LICENSE-MIT) file for details.

## Acknowledgments

Inspired by:
- [Scoop](https://scoop.sh/) - Windows package manager
- [scoop-better-shimexe](https://github.com/71/scoop-better-shimexe) - Improved Scoop shim implementation
- [vx](https://github.com/loonghao/vx) - Version management tool
- [rye](https://github.com/astral-sh/rye) - Python project management
