# Shim Configuration Guide

This document provides a comprehensive guide to configuring shimexe shims using TOML files with advanced template system and dynamic configuration support.

## Table of Contents

- [Basic Configuration](#basic-configuration)
- [Core Shim Settings](#core-shim-settings)
- [Advanced Args Configuration](#advanced-args-configuration)
- [Dynamic Template System](#dynamic-template-system)
- [Environment Variables](#environment-variables)
- [Auto-Update Configuration](#auto-update-configuration)
- [Metadata](#metadata)
- [Environment Variable Expansion](#environment-variable-expansion)
- [Template Functions](#template-functions)
- [Conditional Logic](#conditional-logic)
- [Examples](#examples)
- [Standalone Executable Compilation](#standalone-executable-compilation)

## Basic Configuration

A shim configuration file uses the TOML format and typically has the extension `.shim.toml`. The basic structure includes:

```toml
[shim]
name = "my-tool"
path = "/path/to/executable"

[env]
# Environment variables

[auto_update]
# Auto-update settings

[metadata]
# Optional metadata
```

## Core Shim Settings

The `[shim]` section contains the essential configuration for the executable:

### Required Fields

- **`name`** (string): The name of the shim
- **`path`** (string): Path to the target executable (supports environment variable expansion)

### Optional Fields

- **`args`** (array of strings): Default arguments to pass to the executable (legacy format)
- **`cwd`** (string): Working directory for the executable

```toml
[shim]
name = "rust"
path = "${RUST_HOME:~/.cargo/bin}/rustc${EXE_EXT:.exe}"
args = ["--version"]  # Legacy format - still supported
cwd = "${PROJECT_ROOT:./}"
```

## Advanced Args Configuration

The `[args]` section provides powerful argument handling with template support and dynamic behavior.

### Args Configuration Options

```toml
[args]
# Template-based argument configuration (recommended)
template = [
    "{{if env('DEBUG') == 'true'}}--verbose{{endif}}",
    "--config", "{{env('CONFIG_PATH') | default('~/.config/tool.conf')}}",
    "{{args | default('--version')}}"  # User arguments with fallback
]

# Alternative: inline template for simple cases
inline = "{{if env('DEBUG')}}--verbose{{endif}} --config {{env('CONFIG_PATH', '~/.config')}} {{args('--version')}}"

# Argument behavior mode
mode = "template"  # "template" | "merge" | "replace" | "prepend"

# Legacy support: default arguments (when mode != "template")
default = ["--version"]
prefix = ["--config", "/path/to/config"]  # Always prepended
suffix = ["--verbose"]                    # Always appended
```

### Args Modes

- **`template`**: Use template-based argument processing (recommended)
- **`merge`**: Combine default args + user args
- **`replace`**: User args completely replace default args
- **`prepend`**: User args + default args

## Dynamic Template System

shimexe supports a powerful template system that allows dynamic configuration based on runtime conditions, environment variables, and user input.

### Template Syntax

Templates use `{{}}` syntax with support for:
- Environment variables: `{{env('VAR_NAME')}}`
- Default values: `{{env('VAR_NAME', 'default')}}`
- User arguments: `{{args}}`, `{{args('default')}}`
- Conditional logic: `{{if condition}}...{{endif}}`
- Function calls: `{{platform()}}`, `{{file_exists('path')}}`
- Filters: `{{value | upper | default('fallback')}}`

### Basic Template Examples

```toml
[shim]
name = "node"
path = "{{resolve_path('${NODE_HOME}/bin/node${EXE_EXT}')}}"

[args]
template = [
    "{{if env('NODE_ENV') == 'development'}}--inspect{{endif}}",
    "--max-old-space-size={{env('NODE_MEMORY', '4096')}}",
    "{{args('--version')}}"
]
```

## Environment Variables

The `[env]` section allows you to set environment variables with template support:

```toml
[env]
# Static environment variables
RUST_LOG = "info"
CARGO_HOME = "${CARGO_HOME:~/.cargo}"

# Dynamic environment variables with templates
TOOL_LOG_LEVEL = "{{env('LOG_LEVEL') | upper | default('INFO')}}"
TOOL_CONFIG_DIR = "{{env('XDG_CONFIG_HOME', '~/.config')}}/tool"
TOOL_DATA_DIR = "{{platform_data_dir()}}/tool"

# Conditional environment variables
"{{if platform() == 'windows'}}TOOL_WINDOWS_SPECIFIC{{endif}}" = "windows_value"
```

## Auto-Update Configuration

The `[auto_update]` section enables automatic updates for the target executable:

### Basic Auto-Update

```toml
[auto_update]
enabled = true
check_interval_hours = 24
download_url = "https://example.com/releases/{version}/tool-{os}-{arch}"

[auto_update.provider]
type = "github"
repo = "owner/repository"
asset_pattern = "tool-{version}-{os}-{arch}.tar.gz"
include_prerelease = false

[auto_update.version_check]
type = "github_latest"
repo = "owner/repository"
include_prerelease = false
```

### Auto-Update Fields

- **`enabled`** (boolean): Enable/disable auto-updates
- **`check_interval_hours`** (integer): Hours between update checks (0 = check every run)
- **`download_url`** (string): URL template for downloading updates
- **`pre_update_command`** (string, optional): Command to run before updating
- **`post_update_command`** (string, optional): Command to run after updating

### Update Providers

#### GitHub Provider

```toml
[auto_update.provider]
type = "github"
repo = "owner/repository"
asset_pattern = "tool-{version}-{os}-{arch}.tar.gz"
include_prerelease = false
```

#### HTTPS Provider

```toml
[auto_update.provider]
type = "https"
base_url = "https://releases.example.com"
version_url = "https://api.example.com/version"
```

#### Custom Provider

```toml
[auto_update.provider]
type = "custom"
update_command = "curl -L {download_url} -o {target_path}"
version_command = "curl -s https://api.example.com/version | jq -r .version"
```

### Version Check Types

#### GitHub Latest

```toml
[auto_update.version_check]
type = "github_latest"
repo = "owner/repository"
include_prerelease = false
```

#### HTTP Endpoint

```toml
[auto_update.version_check]
type = "http"
url = "https://api.example.com/version"
json_path = "$.version"  # JSONPath to extract version
regex_pattern = "v(\\d+\\.\\d+\\.\\d+)"  # Regex to extract version
```

#### Semantic Version

```toml
[auto_update.version_check]
type = "semver"
current = "1.0.0"
check_url = "https://api.example.com/latest"
```

#### Command-based

```toml
[auto_update.version_check]
type = "command"
command = "my-tool"
args = ["--version"]
```

## Metadata

The `[metadata]` section contains optional information about the shim:

```toml
[metadata]
description = "Rust compiler shim"
version = "1.0.0"
author = "Your Name"
tags = ["rust", "compiler", "development"]
```

## Environment Variable Expansion

shimexe supports powerful environment variable expansion in all string fields:

### Syntax

- **`${VAR}`**: Expand VAR, error if not found
- **`${VAR:default}`**: Expand VAR, use default if not found
- **`$VAR`**: Simple expansion (bash-style)

### Built-in Variables

- **`${EXE_EXT}`**: Platform-specific executable extension (`.exe` on Windows, empty on Unix)
- **`${PATH_SEP}`**: Platform-specific path separator (`\` on Windows, `/` on Unix)
- **`${HOME}`**: User home directory
- **`${CONFIG_DIR}`**: User configuration directory
- **`${DATA_DIR}`**: User data directory

## Template Functions

shimexe provides a rich set of built-in template functions:

### Environment Functions
- `env(key)` - Get environment variable (returns empty if not found)
- `env(key, default)` - Get environment variable with default value

### File System Functions
- `file_exists(path)` - Check if file exists
- `dir_exists(path)` - Check if directory exists
- `resolve_path(path)` - Resolve and normalize path

### Platform Functions
- `platform()` - Get platform name ("windows", "linux", "macos")
- `arch()` - Get architecture ("x86_64", "aarch64", etc.)
- `exe_ext()` - Get executable extension (".exe" on Windows, "" on Unix)

### Path Functions
- `home_dir()` - Get user home directory
- `config_dir()` - Get user config directory
- `data_dir()` - Get user data directory
- `platform_data_dir()` - Get platform-specific data directory

### String Functions
- `upper(text)` - Convert to uppercase
- `lower(text)` - Convert to lowercase
- `trim(text)` - Remove whitespace

### Argument Functions
- `args()` - Get all user arguments as array
- `args(default)` - Get user arguments or default if empty
- `arg(index)` - Get specific argument by index

## Conditional Logic

Templates support conditional expressions and logic:

```toml
[args]
template = [
    # Simple condition
    "{{if env('DEBUG') == 'true'}}--verbose{{endif}}",

    # Condition with else
    "{{if file_exists('local.conf')}}--config local.conf{{else}}--config default.conf{{endif}}",

    # Multiple conditions
    "{{if platform() == 'windows' && env('WINDOWS_MODE')}}--windows-specific{{endif}}",

    # Nested conditions
    "{{if env('NODE_ENV') == 'development'}}{{if env('DEBUG')}}--inspect --debug{{else}}--inspect{{endif}}{{endif}}"
]
```

### Custom Template Functions

You can define custom template functions for complex logic:

```toml
[template_functions]
resolve_node_path = """
let candidates = [
    env('NODE_HOME') + '/bin/node' + exe_ext(),
    env('NVM_DIR') + '/current/bin/node' + exe_ext(),
    '/usr/local/bin/node',
    '/usr/bin/node'
];

for path in candidates {
    if file_exists(path) {
        return path;
    }
}

error('Node.js not found in any expected location');
"""

platform_cache_dir = """
match platform() {
    'windows' => env('LOCALAPPDATA') + '/npm-cache',
    'macos' => env('HOME') + '/Library/Caches/npm',
    _ => env('XDG_CACHE_HOME', env('HOME') + '/.cache') + '/npm'
}
"""
```

## Examples

### Simple Tool Shim (Legacy Format)

```toml
[shim]
name = "hello"
path = "echo"
args = ["Hello, World!"]

[metadata]
description = "Simple hello world shim"
```

### Advanced Template-Based Shim

```toml
[shim]
name = "node"
path = "{{resolve_node_path()}}"

[args]
template = [
    # Development mode debugging
    "{{if env('NODE_ENV') == 'development'}}--inspect{{endif}}",

    # Project-specific configuration
    "{{if file_exists('package.json')}}--experimental-modules{{endif}}",

    # Dynamic memory limit
    "--max-old-space-size={{env('NODE_MEMORY', '4096')}}",

    # User arguments with smart defaults
    "{{args() | if_empty(file_exists('index.js') ? 'index.js' : '--version')}}"
]

[env]
# Dynamic NODE_PATH construction
NODE_PATH = "{{join_paths([
    env('NODE_PATH', ''),
    './node_modules',
    env('HOME') + '/.npm/lib/node_modules'
])}}"

# Platform-specific cache directory
NPM_CONFIG_CACHE = "{{platform_cache_dir()}}/npm"

[template_functions]
resolve_node_path = """
let candidates = [
    env('NODE_HOME') + '/bin/node' + exe_ext(),
    env('NVM_DIR') + '/current/bin/node' + exe_ext(),
    '/usr/local/bin/node',
    '/usr/bin/node'
];

for path in candidates {
    if file_exists(path) {
        return path;
    }
}

error('Node.js not found');
"""

join_paths = """
|paths| {
    paths.filter(|p| p != '').join(path_separator())
}
"""

[metadata]
description = "Node.js runtime with dynamic configuration"
version = "2.0.0"
```

### Development Tool with Auto-Update

```toml
[shim]
name = "node"
path = "${NODE_HOME:~/.nvm/current}/bin/node${EXE_EXT}"
args = []

[env]
NODE_ENV = "development"
NPM_CONFIG_PREFIX = "${NODE_HOME:~/.nvm/current}"

[auto_update]
enabled = true
check_interval_hours = 168  # Weekly
download_url = "https://nodejs.org/dist/v{version}/node-v{version}-{os}-{arch}.tar.gz"

[auto_update.provider]
type = "https"
base_url = "https://nodejs.org/dist"
version_url = "https://nodejs.org/dist/index.json"

[auto_update.version_check]
type = "http"
url = "https://nodejs.org/dist/index.json"
json_path = "$[0].version"

[metadata]
description = "Node.js runtime shim with auto-update"
version = "1.0.0"
tags = ["nodejs", "runtime", "javascript"]
```

### Cross-Platform Tool

```toml
[shim]
name = "rust"
path = "${RUST_HOME:~/.cargo/bin}/rustc${EXE_EXT}"
args = []
cwd = "${PWD}"

[env]
RUST_LOG = "${RUST_LOG:warn}"
CARGO_HOME = "${CARGO_HOME:~/.cargo}"
RUSTUP_HOME = "${RUSTUP_HOME:~/.rustup}"

[auto_update]
enabled = true
check_interval_hours = 24

[auto_update.provider]
type = "github"
repo = "rust-lang/rust"
asset_pattern = "rust-{version}-{arch}-{os}.tar.gz"
include_prerelease = false

[auto_update.version_check]
type = "github_latest"
repo = "rust-lang/rust"
include_prerelease = false

[metadata]
description = "Rust compiler with automatic updates"
version = "1.0.0"
author = "shimexe"
tags = ["rust", "compiler", "systems-programming"]
```

### Custom Update Logic

```toml
[shim]
name = "custom-tool"
path = "${TOOLS_DIR}/custom-tool${EXE_EXT}"

[auto_update]
enabled = true
check_interval_hours = 0  # Check every run
pre_update_command = "echo 'Preparing update...'"
post_update_command = "echo 'Update completed!'"

[auto_update.provider]
type = "custom"
update_command = "bash ${CONFIG_DIR}/update-script.sh {version}"
version_command = "curl -s https://api.example.com/version"

[auto_update.version_check]
type = "command"
command = "custom-tool"
args = ["--version"]

[metadata]
description = "Custom tool with scripted updates"
```

## Standalone Executable Compilation

shimexe can compile shim configurations into standalone, self-contained executables that require no external dependencies.

### Compilation Commands

```bash
# Compile a single shim configuration
shimexe compile shim.toml --output my-tool.exe

# Compile with custom icon
shimexe compile shim.toml --output my-tool.exe --icon icon.ico

# Cross-platform compilation
shimexe compile shim.toml --output my-tool --target x86_64-unknown-linux-musl

# Batch compilation
shimexe compile-batch ./configs/ --output-dir ./dist/
```

### Standalone Executable Features

- **Zero Dependencies**: Completely self-contained, no runtime dependencies
- **Static Linking**: All libraries statically linked into the binary
- **Embedded Configuration**: Configuration compiled into the executable
- **Cross-Platform**: Support for Windows, Linux, and macOS
- **Template Processing**: Full template engine embedded in the binary

### Deployment Model

```bash
# Developer side
shimexe compile my-app.shim.toml --output my-app.exe

# User side (no installation required)
./my-app.exe --help  # Just works!
```

The compiled executable contains:
- ✅ Configuration parser
- ✅ Template engine
- ✅ Environment variable processor
- ✅ Process launcher
- ✅ All template functions
- ❌ Target executable (must exist on user system)

## Best Practices

1. **Use template-based configuration** for maximum flexibility and dynamic behavior
2. **Leverage conditional logic** to handle different environments and platforms
3. **Use environment variable expansion** for portable configurations
4. **Define custom template functions** for complex logic reuse
5. **Test template expressions** with different environment conditions
6. **Include comprehensive metadata** for documentation
7. **Use standalone compilation** for distribution to end users
8. **Validate configurations** before compilation with `shimexe validate-template`
