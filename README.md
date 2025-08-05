<div align="center">

<img src="assets/icon.svg" alt="shimexe logo" width="120" height="120">

# shimexe

**🚀 The Modern Executable Shim Manager**

*Transform any executable into a smart, portable shim with HTTP download support*

<!-- Build & Quality -->
[![CI](https://github.com/loonghao/shimexe/workflows/CI/badge.svg)](https://github.com/loonghao/shimexe/actions)
[![Release](https://github.com/loonghao/shimexe/workflows/Release/badge.svg)](https://github.com/loonghao/shimexe/actions)
[![codecov](https://codecov.io/gh/loonghao/shimexe/branch/main/graph/badge.svg)](https://codecov.io/gh/loonghao/shimexe)
[![Security audit](https://github.com/loonghao/shimexe/workflows/Security%20audit/badge.svg)](https://github.com/loonghao/shimexe/actions)

<!-- Package Versions -->
[![Crates.io](https://img.shields.io/crates/v/shimexe.svg?logo=rust&logoColor=white)](https://crates.io/crates/shimexe)
[![Homebrew](https://img.shields.io/homebrew/v/shimexe?logo=homebrew&logoColor=white)](https://formulae.brew.sh/formula/shimexe)
[![Chocolatey](https://img.shields.io/chocolatey/v/shimexe?logo=chocolatey&logoColor=white)](https://chocolatey.org/packages/shimexe)
[![Scoop](https://img.shields.io/scoop/v/shimexe?logo=windows&logoColor=white)](https://scoop.sh/#/apps?q=shimexe)

<!-- Downloads & Usage -->
[![Crates.io Downloads](https://img.shields.io/crates/d/shimexe.svg?logo=rust&logoColor=white&label=cargo%20installs)](https://crates.io/crates/shimexe)
[![GitHub Downloads](https://img.shields.io/github/downloads/loonghao/shimexe/total?logo=github&logoColor=white&label=github%20downloads)](https://github.com/loonghao/shimexe/releases)
[![Homebrew Downloads](https://img.shields.io/homebrew/installs/dm/shimexe?logo=homebrew&logoColor=white&label=brew%20installs)](https://formulae.brew.sh/formula/shimexe)
[![Chocolatey Downloads](https://img.shields.io/chocolatey/dt/shimexe?logo=chocolatey&logoColor=white&label=choco%20installs)](https://chocolatey.org/packages/shimexe)

<!-- Documentation & Community -->
[![Documentation](https://docs.rs/shimexe/badge.svg)](https://docs.rs/shimexe)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/loonghao/shimexe#license)
[![GitHub Stars](https://img.shields.io/github/stars/loonghao/shimexe?style=social)](https://github.com/loonghao/shimexe)
[![GitHub Forks](https://img.shields.io/github/forks/loonghao/shimexe?style=social)](https://github.com/loonghao/shimexe/fork)

<!-- Platform Support -->
[![Platform Support](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?logo=rust&logoColor=white)](https://github.com/loonghao/shimexe/releases)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange?logo=rust&logoColor=white)](https://www.rust-lang.org)

[📖 中文文档](README_zh.md) • [🚀 Quick Start](#quick-start) • [📦 Installation](#installation) • [🔧 Examples](#examples)

</div>

---

## 🌟 What is shimexe?

**shimexe** is a revolutionary executable shim manager that bridges the gap between local tools and cloud-distributed applications. Create lightweight, portable shims that can automatically download, extract, and execute tools from HTTP URLs - all while maintaining the simplicity of local executables.

### 💡 Why shimexe?

- **🌐 Cloud-Native**: Download tools directly from GitHub releases, CDNs, or any HTTP URL
- **📦 Archive Smart**: Automatically extract zip files and discover executables
- **🔧 Zero Config**: Smart defaults with powerful customization options
- **🚀 Portable**: Shims work independently without requiring shimexe installation
- **⚡ Fast**: Efficient caching and smart re-download logic
- **🔒 Secure**: Built with Rust and rustls-tls for secure HTTPS connections

## ✨ Key Features

<table>
<tr>
<td width="50%">

### 🌐 **Cloud-First Design**
- **HTTP URL Support**: Download from GitHub, CDNs, any URL
- **Archive Extraction**: Auto-extract zip files and find executables
- **Smart Caching**: Efficient download and storage management
- **Auto-Recovery**: Re-download missing files automatically

### 🔧 **Developer Experience**
- **Zero Configuration**: Works out of the box with smart defaults
- **TOML Configuration**: Human-readable, version-controllable configs
- **Environment Variables**: Powerful `${VAR:default}` expansion
- **Template System**: Dynamic path and argument resolution

</td>
<td width="50%">

### 🚀 **Production Ready**
- **Cross-Platform**: Windows, macOS, Linux support
- **Static Linking**: No runtime dependencies required
- **Portable Shims**: Work independently when distributed
- **Secure**: Built with Rust and rustls-tls

### 📦 **Integration Friendly**
- **Package Managers**: Available on Crates.io, Chocolatey, Scoop
- **API Library**: Use `shimexe-core` in your own projects
- **Tool Managers**: Perfect for vx, rye, and similar tools
- **CI/CD Ready**: Ideal for automated environments

</td>
</tr>
</table>

## 📦 Installation

<div align="center">

### 🚀 **One-Line Install** (Recommended)

</div>

<table>
<tr>
<td width="50%">

**🐧 Unix/Linux/macOS**
```bash
curl -LsSf https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.sh | sh
```

**🪟 Windows (PowerShell)**
```powershell
irm https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.ps1 | iex
```

</td>
<td width="50%">

**📌 Specific Version**
```bash
# Unix/Linux/macOS
SHIMEXE_VERSION="0.3.1" curl -LsSf https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.sh | sh

# Windows
$env:SHIMEXE_VERSION="0.3.1"; irm https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.ps1 | iex
```

</td>
</tr>
</table>

### 📦 **Package Managers**

<table>
<tr>
<td width="25%">

**🦀 Cargo**
```bash
cargo install shimexe
```

</td>
<td width="25%">

**🍺 Homebrew**
```bash
# Add tap first
brew tap loonghao/tap

# Install shimexe
brew install shimexe
```

</td>
<td width="25%">

**🍫 Chocolatey**
```powershell
choco install shimexe
```

</td>
<td width="25%">

**🥄 Scoop**
```powershell
# Add bucket first
scoop bucket add loonghao https://github.com/loonghao/scoop-bucket

# Install shimexe
scoop install shimexe
```

</td>
</tr>
</table>

### 📥 **Manual Download**

Download pre-built binaries from [**GitHub Releases**](https://github.com/loonghao/shimexe/releases) for your platform.

---

## ⚙️ **PATH Configuration**

<div align="center">

### **Choose Your Approach**

</div>

<table>
<tr>
<td width="50%">

### 🔧 **Option 1: Auto-configure PATH** (Recommended)

Add `--add-system-path` when creating shims:

```bash
shimexe add uv --path https://github.com/astral-sh/uv/releases/download/0.7.13/uv-x86_64-pc-windows-msvc.zip --add-system-path

# Use directly
uv --version
```

**Benefits:**
- ✅ Use tools directly by name
- ✅ Works like system-installed tools
- ✅ Automatic PATH management

</td>
<td width="50%">

### 🏃 **Option 2: Use `shimexe run`** (No setup)

Run tools via shimexe command:

```bash
shimexe add uv --path https://github.com/astral-sh/uv/releases/download/0.7.13/uv-x86_64-pc-windows-msvc.zip

# Use via shimexe run
shimexe run uv --version
```

**Benefits:**
- ✅ No PATH modification needed
- ✅ Explicit tool execution
- ✅ Works immediately

</td>
</tr>
</table>

### 📍 **Manual PATH Setup**

If you prefer manual configuration, add `~/.shimexe` to your PATH:

**Unix/Linux/macOS:**
```bash
echo 'export PATH="$HOME/.shimexe:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Windows (PowerShell):**
```powershell
$env:PATH = "$env:USERPROFILE\.shimexe;$env:PATH"
# Or add permanently via System Properties > Environment Variables
```

## 🚀 Quick Start

> **🔧 CI/CD Improvement**: Release automation has been enhanced with proper release-please configuration for better reliability and streamlined package management.

<div align="center">

### **Get Started in 30 Seconds!**

*Now available on all major package managers!*

</div>

<table>
<tr>
<td width="50%">

### 🌐 **Cloud Tools** (The Magic!)

**1. Download from GitHub Releases**
```bash
# Download and create shim with PATH setup
shimexe add uv --path https://github.com/astral-sh/uv/releases/download/0.7.13/uv-x86_64-pc-windows-msvc.zip --add-system-path

# Now use it directly!
uv --version

# Or without PATH setup, use shimexe run:
shimexe run uv --version
```

**2. Auto-infer tool names**
```bash
# Creates 'installer-analyzer' shim with PATH setup
shimexe add --path https://github.com/loonghao/installer-analyzer/releases/download/v0.7.0/installer-analyzer.exe --add-system-path

# Use directly or via shimexe run
installer-analyzer --help
shimexe run installer-analyzer --help
```

**3. Archive extraction**
```bash
# Extracts zip and finds all executables
shimexe add devtools --path https://example.com/tools.zip --add-system-path

# Creates multiple shims automatically!
# Use directly or via shimexe run
```

</td>
<td width="50%">

### 🔧 **Local Tools** (Traditional)

**1. Initialize with examples**
```bash
shimexe init --examples
```

**2. Create local shims**
```bash
# Rust compiler shim
shimexe add rustc --path "${RUST_HOME:~/.cargo/bin}/rustc${EXE_EXT:.exe}"

# Python with custom args
shimexe add py --path python --args "-u"
```

**3. Manage your shims**
```bash
# List all shims
shimexe list --detailed

# Update existing shim
shimexe update rustc --args "--version"

# Remove shim
shimexe remove old-tool
```

</td>
</tr>
</table>

### 🎯 **Real-World Examples**

```bash
# Popular tools you can install instantly (with PATH setup):
shimexe add rg --path https://github.com/BurntSushi/ripgrep/releases/download/14.1.1/ripgrep-14.1.1-x86_64-pc-windows-msvc.zip --add-system-path
shimexe add fd --path https://github.com/sharkdp/fd/releases/download/v10.2.0/fd-v10.2.0-x86_64-pc-windows-msvc.zip --add-system-path
shimexe add bat --path https://github.com/sharkdp/bat/releases/download/v0.24.0/bat-v0.24.0-x86_64-pc-windows-msvc.zip --add-system-path

# Now use them anywhere!
rg "TODO" --type rust
fd "*.rs" src/
bat README.md

# Or use via shimexe run (no PATH setup needed):
shimexe run rg "TODO" --type rust
shimexe run fd "*.rs" src/
shimexe run bat README.md
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
