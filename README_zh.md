<div align="center">

<img src="assets/icon.svg" alt="shimexe logo" width="120" height="120">

# shimexe

**现代化可执行文件 Shim 管理器**

*将任何可执行文件转换为智能、便携的 shim，支持 HTTP 下载*

<!-- 构建与质量 -->
[![CI](https://github.com/loonghao/shimexe/workflows/CI/badge.svg)](https://github.com/loonghao/shimexe/actions)
[![Release](https://github.com/loonghao/shimexe/workflows/Release/badge.svg)](https://github.com/loonghao/shimexe/actions)
[![codecov](https://codecov.io/gh/loonghao/shimexe/branch/main/graph/badge.svg)](https://codecov.io/gh/loonghao/shimexe)
[![Security audit](https://github.com/loonghao/shimexe/workflows/Security%20audit/badge.svg)](https://github.com/loonghao/shimexe/actions)

<!-- 包版本 -->
[![Crates.io](https://img.shields.io/crates/v/shimexe.svg?logo=rust&logoColor=white)](https://crates.io/crates/shimexe)
[![Homebrew](https://img.shields.io/homebrew/v/shimexe?logo=homebrew&logoColor=white)](https://formulae.brew.sh/formula/shimexe)
[![Chocolatey](https://img.shields.io/chocolatey/v/shimexe?logo=chocolatey&logoColor=white)](https://chocolatey.org/packages/shimexe)
[![Scoop](https://img.shields.io/scoop/v/shimexe?logo=windows&logoColor=white)](https://scoop.sh/#/apps?q=shimexe)

<!-- 下载量与使用情况 -->
[![Crates.io Downloads](https://img.shields.io/crates/d/shimexe.svg?logo=rust&logoColor=white&label=cargo%20installs)](https://crates.io/crates/shimexe)
[![GitHub Downloads](https://img.shields.io/github/downloads/loonghao/shimexe/total?logo=github&logoColor=white&label=github%20downloads)](https://github.com/loonghao/shimexe/releases)
[![Homebrew Downloads](https://img.shields.io/homebrew/installs/dm/shimexe?logo=homebrew&logoColor=white&label=brew%20installs)](https://formulae.brew.sh/formula/shimexe)
[![Chocolatey Downloads](https://img.shields.io/chocolatey/dt/shimexe?logo=chocolatey&logoColor=white&label=choco%20installs)](https://chocolatey.org/packages/shimexe)

<!-- 文档与社区 -->
[![Documentation](https://docs.rs/shimexe/badge.svg)](https://docs.rs/shimexe)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/loonghao/shimexe#license)
[![GitHub Stars](https://img.shields.io/github/stars/loonghao/shimexe?style=social)](https://github.com/loonghao/shimexe)
[![GitHub Forks](https://img.shields.io/github/forks/loonghao/shimexe?style=social)](https://github.com/loonghao/shimexe/fork)

<!-- 平台支持 -->
[![Platform Support](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?logo=rust&logoColor=white)](https://github.com/loonghao/shimexe/releases)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange?logo=rust&logoColor=white)](https://www.rust-lang.org)

[English Documentation](README.md) · [快速开始](#快速开始) · [安装](#安装) · [示例](#示例)

</div>

---

## 什么是 shimexe？

**shimexe** 是一个跨平台的可执行文件 shim 管理器，连接本地工具与云分发应用程序。创建轻量级、便携的 shim，可以自动从 HTTP URL 下载、提取和执行工具，同时保持本地可执行文件的简单性。

### 为什么选择 shimexe？

- **云原生**: 直接从 GitHub releases、CDN 或任何 HTTP URL 下载工具
- **智能归档**: 自动提取 zip 文件并发现可执行文件
- **零配置**: 智能默认设置，强大的自定义选项
- **便携**: Shim 可独立工作，无需安装 shimexe
- **快速**: 高效缓存和智能重新下载逻辑
- **安全**: 使用 Rust 和 rustls-tls 构建，确保安全的 HTTPS 连接

## 核心特性

<table>
<tr>
<td width="50%">

### 云优先设计
- **HTTP URL 支持**: 从 GitHub、CDN、任何 URL 下载
- **归档提取**: 自动提取 zip 文件并查找可执行文件
- **智能缓存**: 高效的下载和存储管理
- **自动恢复**: 自动重新下载缺失的文件

### 开发者体验
- **零配置**: 开箱即用的智能默认设置
- **TOML 配置**: 人类可读、版本可控的配置
- **环境变量**: 强大的 `${VAR:default}` 扩展
- **模板系统**: 动态路径和参数解析

</td>
<td width="50%">

### 生产就绪
- **跨平台**: Windows、macOS、Linux 支持
- **静态链接**: 无需运行时依赖
- **便携 Shim**: 分发时可独立工作
- **安全**: 使用 Rust 和 rustls-tls 构建

### 集成友好
- **包管理器**: 可通过 Crates.io、Chocolatey、Scoop 安装
- **API 库**: 在您的项目中使用 `shimexe-core`
- **工具管理器**: 完美适配 vx、rye 等类似工具
- **CI/CD 就绪**: 适用于自动化环境

</td>
</tr>
</table>

## 安装

<div align="center">

### 一键安装 (推荐)

</div>

<table>
<tr>
<td width="50%">

**Unix/Linux/macOS**
```bash
curl -LsSf https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.sh | sh
```

**Windows (PowerShell)**
```powershell
irm https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.ps1 | iex
```

</td>
<td width="50%">

**指定版本**
```bash
# Unix/Linux/macOS
SHIMEXE_VERSION="0.3.1" curl -LsSf https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.sh | sh

# Windows
$env:SHIMEXE_VERSION="0.3.1"; irm https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.ps1 | iex
```

</td>
</tr>
</table>

### 包管理器

<table>
<tr>
<td width="25%">

**Cargo**
```bash
cargo install shimexe
```

</td>
<td width="25%">

**Homebrew**
```bash
# 首先添加 tap
brew tap loonghao/tap

# 安装 shimexe
brew install shimexe
```

</td>
<td width="25%">

**Chocolatey**
```powershell
choco install shimexe
```

</td>
<td width="25%">

**Scoop**
```powershell
# 首先添加 bucket
scoop bucket add loonghao https://github.com/loonghao/scoop-bucket

# 安装 shimexe
scoop install shimexe
```

</td>
</tr>
</table>

### 手动下载

从 [GitHub Releases](https://github.com/loonghao/shimexe/releases) 下载适合您平台的预构建二进制文件。

### Nix (可复现 + 缓存构建)

```bash
# 构建
nix build .#shimexe

# 开发环境
nix develop
```

详见 [`docs/NIX.md`](docs/NIX.md)。

---

## PATH 配置

<div align="center">

### 选择您的方式

</div>

<table>
<tr>
<td width="50%">

### 方式一：自动配置 PATH (推荐)

创建 shim 时添加 `--add-system-path`：

```bash
shimexe add uv --path https://github.com/astral-sh/uv/releases/download/0.7.13/uv-x86_64-pc-windows-msvc.zip --add-system-path

# 直接使用
uv --version
```

**优点：**
- 直接通过名称使用工具
- 像系统安装的工具一样工作
- 自动 PATH 管理

</td>
<td width="50%">

### 方式二：使用 `shimexe run` (无需设置)

通过 shimexe 命令运行工具：

```bash
shimexe add uv --path https://github.com/astral-sh/uv/releases/download/0.7.13/uv-x86_64-pc-windows-msvc.zip

# 通过 shimexe run 使用
shimexe run uv --version
```

**优点：**
- 无需修改 PATH
- 明确的工具执行
- 立即可用

</td>
</tr>
</table>

### 手动 PATH 设置

如果您喜欢手动配置，将 `~/.shimexe` 添加到您的 PATH：

**Unix/Linux/macOS:**
```bash
echo 'export PATH="$HOME/.shimexe:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Windows (PowerShell):**
```powershell
$env:PATH = "$env:USERPROFILE\.shimexe;$env:PATH"
# 或通过系统属性 > 环境变量永久添加
```

## 快速开始

<div align="center">

### 30 秒上手

*现已支持所有主流包管理器*

</div>

<table>
<tr>
<td width="50%">

### 云工具

**1. 从 GitHub Releases 下载**
```bash
# 自动下载并创建 shim，配置 PATH
shimexe add uv --path https://github.com/astral-sh/uv/releases/download/0.7.13/uv-x86_64-pc-windows-msvc.zip --add-system-path

# 现在可以直接使用
uv --version

# 或者不配置 PATH，使用 shimexe run：
shimexe run uv --version
```

**2. 自动推断工具名称**
```bash
# 创建 'installer-analyzer' shim，配置 PATH
shimexe add --path https://github.com/loonghao/installer-analyzer/releases/download/v0.7.0/installer-analyzer.exe --add-system-path

# 直接使用或通过 shimexe run
installer-analyzer --help
shimexe run installer-analyzer --help
```

**3. 归档提取**
```bash
# 提取 zip 并查找所有可执行文件
shimexe add devtools --path https://example.com/tools.zip --add-system-path

# 自动创建多个 shim
# 直接使用或通过 shimexe run
```

</td>
<td width="50%">

### 本地工具

**1. 使用示例初始化**
```bash
shimexe init --examples
```

**2. 创建本地 shim**
```bash
# Rust 编译器 shim
shimexe add rustc --path "${RUST_HOME:~/.cargo/bin}/rustc${EXE_EXT:.exe}"

# 带自定义参数的 Python
shimexe add py --path python --args "-u"
```

**3. 管理您的 shim**
```bash
# 列出所有 shim
shimexe list --detailed

# 更新现有 shim
shimexe update rustc --args "--version"

# 删除 shim
shimexe remove old-tool
```

</td>
</tr>
</table>

### 真实世界示例

```bash
# 您可以立即安装的热门工具（配置 PATH）：
shimexe add rg --path https://github.com/BurntSushi/ripgrep/releases/download/14.1.1/ripgrep-14.1.1-x86_64-pc-windows-msvc.zip --add-system-path
shimexe add fd --path https://github.com/sharkdp/fd/releases/download/v10.2.0/fd-v10.2.0-x86_64-pc-windows-msvc.zip --add-system-path
shimexe add bat --path https://github.com/sharkdp/bat/releases/download/v0.24.0/bat-v0.24.0-x86_64-pc-windows-msvc.zip --add-system-path

# 现在可以在任何地方使用它们
rg "TODO" --type rust
fd "*.rs" src/
bat README.md

# 或者通过 shimexe run 使用（无需 PATH 设置）：
shimexe run rg "TODO" --type rust
shimexe run fd "*.rs" src/
shimexe run bat README.md
```

## 配置格式

Shim 使用 TOML 文件配置，文件扩展名为 `.shim.toml`。

### 本地可执行文件配置

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
# Optional environment variables

[metadata]
description = "Installer analyzer tool from GitHub"
version = "0.7.0"
author = "loonghao"
tags = ["installer", "analyzer", "tool"]
```

### 压缩包配置

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

**注意**: 当使用 HTTP URL 或压缩包时，shimexe 会自动下载并解压到 `~/.shimexe/<app>/bin/` 目录，并更新路径指向本地文件。

## 环境变量扩展

shimexe 支持环境变量扩展：

- `${VAR}` - 扩展 VAR，如果不存在则报错
- `${VAR:default}` - 扩展 VAR，如果不存在则使用默认值
- 内置变量:
  - `${EXE_EXT}` - 平台特定的可执行文件扩展名 (Windows 上为 `.exe`)
  - `${PATH_SEP}` - 平台特定的路径分隔符
  - `${HOME}` - 用户主目录
  - `${CONFIG_DIR}` - 用户配置目录

## Shim 工作原理

shimexe 创建的独立可执行 shim 可以在分发时独立工作。

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
shimexe-core = "0.5"
```

示例用法：

```rust
use shimexe_core::prelude::*;

// 加载并运行 shim
let runner = ShimRunner::from_file("my-app.shim.toml")?;
let exit_code = runner.execute(&["--help".to_string()])?;

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
    std::path::Path::new("~/.shimexe"),
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

shimexe 包含一个自定义图标，会嵌入到 Windows 可执行文件中。构建过程自动处理图标生成：

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

欢迎贡献。请随时提交 Pull Request。

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 致谢

灵感来源于:
- [Scoop](https://scoop.sh/) - Windows 包管理器
- [scoop-better-shimexe](https://github.com/71/scoop-better-shimexe) - 改进的 Scoop shim 实现
- [vx](https://github.com/loonghao/vx) - 版本管理工具
- [rye](https://github.com/astral-sh/rye) - Python 项目管理工具
