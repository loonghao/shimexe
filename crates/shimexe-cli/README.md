# shimexe

[![Crates.io](https://img.shields.io/crates/v/shimexe.svg)](https://crates.io/crates/shimexe)
[![Documentation](https://docs.rs/shimexe/badge.svg)](https://docs.rs/shimexe)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](../../LICENSE-MIT)

A modern, cross-platform executable shim manager with environment variable expansion and TOML configuration support.

## Overview

`shimexe` is a command-line tool that allows you to create and manage executable shims. Shims are lightweight wrapper executables that redirect to the actual target executable, allowing you to:

- Create aliases for executables with predefined arguments
- Set up environment variables for specific tools
- Manage multiple versions of the same tool
- Create portable development environments

## Features

- **Cross-platform**: Works on Windows, macOS, and Linux
- **TOML configuration**: Human-readable configuration files
- **Environment variable expansion**: Dynamic path resolution with shell-style variables
- **Auto-update support**: Keep your shims and target executables up-to-date
- **Flexible argument handling**: Support for various argument passing modes
- **System PATH integration**: Automatically add shim directories to your PATH

## Installation

### From Crates.io

```bash
cargo install shimexe
```

### From GitHub Releases

Download the latest binary from the [releases page](https://github.com/loonghao/shimexe/releases).

### Using Chocolatey (Windows)

```powershell
choco install shimexe
```

## Quick Start

### 1. Initialize shimexe

```bash
shimexe init
```

This creates the configuration directory and sets up the basic structure.

### 2. Add your first shim

```bash
shimexe add my-tool --target /path/to/my-tool --args "--config /path/to/config.yaml"
```

### 3. List your shims

```bash
shimexe list
```

### 4. Run your shim

```bash
my-tool additional-args
```

## Commands

### `shimexe add`
Create a new shim.

```bash
shimexe add <name> --target <path> [--args <args>] [--env <key=value>]
```

**Example:**
```bash
shimexe add node --target /usr/local/bin/node --env NODE_ENV=development
```

### `shimexe remove`
Remove an existing shim.

```bash
shimexe remove <name>
```

### `shimexe list`
List all configured shims.

```bash
shimexe list [--verbose]
```

### `shimexe update`
Update an existing shim configuration.

```bash
shimexe update <name> [--target <path>] [--args <args>] [--env <key=value>]
```

### `shimexe validate`
Validate a shim configuration file.

```bash
shimexe validate <config-file>
```

### `shimexe init`
Initialize shimexe configuration and optionally add to system PATH.

```bash
shimexe init [--add-system-path]
```

### `shimexe check-update`
Check for available updates for shims and shimexe itself.

```bash
shimexe check-update [--shim <name>]
```

### `shimexe auto-update`
Manage auto-update settings.

```bash
shimexe auto-update enable|disable [--shim <name>]
```

## Configuration

Shims are configured using TOML files stored in `~/.shimexe/shims/`. Each shim has its own configuration file.

### Example Configuration

```toml
# ~/.shimexe/shims/my-tool.shim.toml
[shim]
target = "${HOME}/bin/my-tool"
args = ["--config", "${CONFIG_DIR}/my-tool.yaml", "--verbose"]

[shim.env]
PATH = "${PATH}:${HOME}/bin"
MY_TOOL_HOME = "${HOME}/.my-tool"
LOG_LEVEL = "info"

[shim.metadata]
name = "my-tool"
version = "1.2.3"
description = "My awesome development tool"

[shim.auto_update]
enabled = true
provider = "github"
repository = "owner/my-tool"
check_interval = "24h"
```

### Environment Variable Expansion

shimexe supports shell-style environment variable expansion:

- `${VAR}` - Expands to the value of VAR
- `${VAR:-default}` - Expands to VAR or "default" if VAR is unset
- `${VAR:+value}` - Expands to "value" if VAR is set, empty otherwise

### Argument Modes

Configure how arguments are passed to the target executable:

```toml
[shim.args]
mode = "append"  # "append", "prepend", or "replace"
values = ["--config", "config.yaml"]
```

## Advanced Usage

### Custom Shim Directory

```bash
shimexe --shim-dir /custom/path add my-tool --target /path/to/tool
```

### Verbose Logging

```bash
shimexe --verbose list
```

### Running as a Shim

When you copy or link `shimexe` with a different name, it automatically runs as that shim:

```bash
cp shimexe my-tool
./my-tool  # Runs the "my-tool" shim configuration
```

## Integration with System PATH

Use the `--add-system-path` flag during initialization to automatically add the shim directory to your system PATH:

```bash
shimexe init --add-system-path
```

This allows you to run your shims from anywhere without specifying the full path.

## Contributing

Contributions are welcome! Please see the [main repository](../../README.md) for contribution guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE-MIT](../../LICENSE-MIT) file for details.
