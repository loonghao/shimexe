# shimexe

[![CI](https://github.com/loonghao/shimexe/workflows/CI/badge.svg)](https://github.com/loonghao/shimexe/actions)
[![Crates.io](https://img.shields.io/crates/v/shimexe.svg)](https://crates.io/crates/shimexe)
[![Documentation](https://docs.rs/shimexe/badge.svg)](https://docs.rs/shimexe)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/loonghao/shimexe#license)

[English Documentation](README.md)

一个现代化的跨平台可执行文件 shim 管理器，支持 HTTP URL 下载、动态模板系统和增强的参数处理功能。

## 特性

- 🚀 **跨平台**: 支持 Windows、macOS 和 Linux
- 🌐 **HTTP URL 支持**: 直接从 URL 下载可执行文件
- 📝 **TOML 配置**: 人类可读的配置文件格式
- 🔧 **环境变量扩展**: 支持 `${VAR:default}` 语法
- 🎯 **单一二进制**: 所有功能集成在一个可执行文件中
- 📦 **包管理器支持**: 可通过 crates.io 和 Chocolatey 安装
- 🔗 **API 库**: 可作为 crate 在您的项目中使用
- 🎨 **自定义图标**: 在可执行文件中嵌入美观的 SVG 图标
- 🤖 **智能名称推断**: 自动从 URL 推断应用程序名称
- ⚡ **自动下载**: 运行时自动下载缺失的可执行文件
- 🔒 **安全下载**: 使用 rustls-tls 进行安全的 HTTPS 连接

## 安装

### 从 Crates.io 安装

```bash
cargo install shimexe
```

### 从 Chocolatey 安装 (Windows)

```powershell
choco install shimexe
```

### 通过 Scoop 安装 (Windows)

```powershell
scoop install shimexe
```

### 快速安装脚本

**Unix 系统 (macOS, Linux):**
```bash
curl -LsSf https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.sh | sh
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.ps1 | iex
```

**安装指定版本:**
```bash
# Unix
SHIMEXE_VERSION="0.3.0" curl -LsSf https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.sh | sh

# Windows
$env:SHIMEXE_VERSION="0.3.0"; irm https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.ps1 | iex
```

### 从 GitHub Releases 下载

从 [GitHub Releases](https://github.com/loonghao/shimexe/releases) 下载最新的二进制文件。

## 快速开始

### 传统本地可执行文件

1. 初始化 shimexe:
   ```bash
   shimexe init --examples
   ```

2. 添加本地可执行文件 shim:
   ```bash
   shimexe add rust --path "${RUST_HOME:~/.cargo/bin}/rustc${EXE_EXT:.exe}" --args "--version"
   ```

### HTTP URL 下载

1. 下载并创建具有明确名称的 shim：
   ```bash
   shimexe add it --path https://github.com/loonghao/installer-analyzer/releases/download/v0.7.0/installer-analyzer.exe
   ```

2. 从 URL 自动推断名称：
   ```bash
   shimexe add --path https://example.com/tools/my-tool.exe
   # 自动创建 'my-tool' shim
   ```

### 压缩包支持（新功能！）

shimexe 现在支持下载和解压压缩包（zip 文件），并自动发现可执行文件：

1. 下载并解压 zip 压缩包：
   ```bash
   shimexe add plz --path https://github.com/release-plz/release-plz/releases/download/release-plz-v0.3.135/release-plz-x86_64-pc-windows-msvc.zip
   ```

2. 自动解压并为所有可执行文件创建 shim：
   ```bash
   shimexe add tools --path https://example.com/multi-tool-package.zip
   # 解压压缩包并为找到的所有 .exe 文件创建 shim
   ```

3. 列出所有 shim:
   ```bash
   shimexe list --detailed
   ```

4. 运行你的 shim（如果缺失会自动下载和解压）：
   ```bash
   plz --help
   ```

## 配置格式

Shim 使用 TOML 文件配置，文件扩展名为 `.shim.toml`:

### 本地可执行文件配置

```toml
[shim]
name = "rust"
path = "${RUST_HOME:~/.cargo/bin}/rustc${EXE_EXT:.exe}"
args = ["--version"]
cwd = "/可选的工作目录"

[env]
RUST_LOG = "info"
CARGO_HOME = "${CARGO_HOME:~/.cargo}"

[metadata]
description = "Rust 编译器 shim"
version = "1.0.0"
author = "您的名字"
tags = ["rust", "compiler"]
```

### HTTP URL 配置

```toml
[shim]
name = "installer-analyzer"
path = "/home/user/.shimexe/installer-analyzer/bin/installer-analyzer.exe"
download_url = "https://github.com/loonghao/installer-analyzer/releases/download/v0.7.0/installer-analyzer.exe"
source_type = "url"
args = []
cwd = ""

[env]
# 可选的环境变量

[metadata]
description = "来自 GitHub 的安装程序分析工具"
version = "0.7.0"
author = "loonghao"
tags = ["installer", "analyzer", "tool"]
```

### 压缩包配置（新功能！）

```toml
[shim]
name = "release-plz"
path = "/home/user/.shimexe/release-plz/bin/release-plz.exe"
download_url = "https://github.com/release-plz/release-plz/releases/download/release-plz-v0.3.135/release-plz-x86_64-pc-windows-msvc.zip"
source_type = "archive"
args = []

# 从压缩包中提取的可执行文件列表
[[shim.extracted_executables]]
name = "release-plz"
path = "release-plz.exe"
full_path = "/home/user/.shimexe/release-plz/bin/release-plz.exe"
is_primary = true

[env]
# 可选的环境变量

[metadata]
description = "来自压缩包的 Release Please 工具"
version = "0.3.135"
author = "release-plz team"
tags = ["release", "automation", "tool"]
```

**注意**: 当使用 HTTP URL 或压缩包时，shimexe 会自动下载并解压到 `~/.shimexe/<app>/bin/` 目录，并更新路径指向本地文件。

## 环境变量扩展

shimexe 支持强大的环境变量扩展功能:

- `${VAR}` - 扩展 VAR，如果不存在则报错
- `${VAR:default}` - 扩展 VAR，如果不存在则使用默认值
- 内置变量:
  - `${EXE_EXT}` - 平台特定的可执行文件扩展名 (Windows 上为 `.exe`)
  - `${PATH_SEP}` - 平台特定的路径分隔符
  - `${HOME}` - 用户主目录
  - `${CONFIG_DIR}` - 用户配置目录

## Shim 工作原理

shimexe 创建的独立可执行 shim 可以在分发时独立工作：

### Shim 架构

1. **双重配置**：每个 shim 由两个文件组成：
   - `<name>.exe` - 可执行 shim（shimexe 二进制的副本）
   - `<name>.shim.toml` - 配置文件

2. **智能配置查找**：当 shim 运行时，按以下顺序搜索配置：
   - **本地**：与可执行文件相同的目录（用于便携式分发）
   - **默认**：用户的 shim 目录（`~/.shimexe/`）

3. **便携式分发**：Shim 可以与其 `.shim.toml` 文件一起复制到任何位置，无需在目标系统上安装 shimexe 即可独立工作。

### 静态链接

shimexe 使用静态链接构建以最小化运行时依赖：
- **Windows**：静态链接 MSVC 运行时（`+crt-static`）
- **无外部依赖**：Shim 无需额外的 DLL 或运行时安装即可工作

### 分发示例

```bash
# 创建 shim
shimexe add mytool --path "/path/to/tool" --args "--default-flag"

# 复制两个文件用于分发
cp ~/.shimexe/mytool.exe ./dist/
cp ~/.shimexe/mytool.shim.toml ./dist/

# shim 现在可以在 ./dist/ 中独立工作
./dist/mytool.exe
```

## CLI 命令

```bash
# 添加新的 shim
shimexe add <name> --path <executable> [--args <args>] [--env KEY=VALUE]

# 删除 shim
shimexe remove <name> [--force]

# 列出所有 shim
shimexe list [--detailed]

# 更新现有 shim
shimexe update <name> [--path <path>] [--args <args>]

# 验证 shim 配置
shimexe validate <shim-file>

# 初始化 shimexe
shimexe init [--examples]
```

### HTTP URL 和压缩包示例

```bash
# 下载具有明确名称的可执行文件
shimexe add mytool --path https://github.com/user/repo/releases/download/v1.0/tool.exe

# 从 URL 自动推断名称（创建 'installer-analyzer' shim）
shimexe add --path https://github.com/loonghao/installer-analyzer/releases/download/v0.7.0/installer-analyzer.exe

# 下载并解压 zip 压缩包（为找到的所有可执行文件创建 shim）
shimexe add plz --path https://github.com/release-plz/release-plz/releases/download/release-plz-v0.3.135/release-plz-x86_64-pc-windows-msvc.zip

# 添加参数和环境变量
shimexe add analyzer --path https://example.com/tools/analyzer.exe --args "--verbose" --env "DEBUG=1"

# 强制覆盖现有 shim
shimexe add mytool --path https://example.com/new-tool.exe --force

# 下载到自定义 shim 目录
shimexe add --shim-dir ./my-tools --path https://example.com/tool.exe

# 包含多个可执行文件的压缩包（自动检测并创建多个 shim）
shimexe add devtools --path https://example.com/development-tools.zip
# 这可能会创建：devtools-compiler、devtools-debugger、devtools-profiler shim
```

## 作为库使用

在您的 `Cargo.toml` 中添加:

```toml
[dependencies]
shimexe-core = "0.1"
```

使用示例:

```rust
use shimexe_core::prelude::*;

// 加载并运行 shim
let runner = ShimRunner::from_file("my-app.shim.toml")?;
let exit_code = runner.execute(&["--help"])?;

// 程序化创建 shim
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

### HTTP URL 下载示例

```rust
use shimexe_core::{Downloader, ShimConfig, ShimCore};

// 程序化下载并创建 shim
let downloader = Downloader::new();
let url = "https://github.com/user/repo/releases/download/v1.0/tool.exe";

// 从 URL 推断应用名称
let app_name = Downloader::infer_app_name_from_url(url).unwrap();
let filename = Downloader::extract_filename_from_url(url).unwrap();

// 生成下载路径
let download_path = Downloader::generate_download_path(
    &std::path::Path::new("~/.shimexe"),
    &app_name,
    &filename
);

// 下载文件
downloader.download_file(url, &download_path).await?;

// 创建 shim 配置
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

## 集成示例

### 与 vx 集成

```rust
use shimexe_core::ShimRunner;

// 在您的 vx 集成中
let shim_path = format!("{}.shim.toml", tool_name);
let runner = ShimRunner::from_file(&shim_path)?;
runner.execute(&args)?;
```

### 与 rye 集成

```rust
use shimexe_core::{ShimConfig, ShimCore};

// 创建 Python 工具 shim
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

## 构建图标

shimexe 包含一个美观的自定义图标，会嵌入到 Windows 可执行文件中。构建过程自动处理图标生成：

1. **自动生成**：如果您安装了 ImageMagick，构建脚本会自动将 `assets/icon.svg` 转换为 `assets/icon.ico`
2. **手动生成**：您也可以手动生成图标：
   ```bash
   # 首先安装 ImageMagick
   winget install ImageMagick.ImageMagick

   # 生成图标
   magick convert -background transparent -define icon:auto-resize=256,128,64,48,32,16 assets/icon.svg assets/icon.ico
   ```
3. **CI/CD**：GitHub Actions 自动安装 ImageMagick 并为所有发布构建生成图标

该图标代表了 shimexe 的核心概念：一个中央枢纽（shim 管理器）连接到多个可执行文件，带有动画数据流指示器，显示工具的动态特性。

## 贡献

欢迎贡献！请随时提交 Pull Request。

## 许可证

本项目采用 MIT 许可证 - 详情请参阅 [LICENSE-MIT](LICENSE-MIT) 文件。

## 致谢

灵感来源于:
- [Scoop](https://scoop.sh/) - Windows 包管理器
- [scoop-better-shimexe](https://github.com/71/scoop-better-shimexe) - 改进的 Scoop shim 实现
- [vx](https://github.com/loonghao/vx) - 版本管理工具
- [rye](https://github.com/astral-sh/rye) - Python 项目管理工具
