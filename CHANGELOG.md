# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.3.2](https://github.com/loonghao/shimexe/compare/v0.3.1...v0.3.2) - 2025-06-17

### Fixed

- add retry logic to update-packages workflow

### Other

- reorganize CI workflow responsibilities

## [0.3.1](https://github.com/loonghao/shimexe/compare/v0.3.0...v0.3.1) - 2025-06-17

### Fixed

- correct release tag format in install scripts
- update README_zh.md and fix install script file naming
- correct install script URLs to use raw.githubusercontent.com
- resolve duplicate release issues and improve Chocolatey publishing

### Other

- update chocolatey-action to latest version v3.3.0

## [0.3.0](https://github.com/loonghao/shimexe/compare/v0.2.1...v0.3.0) - 2025-06-17

### Added

- add comprehensive archive support and package management

### Fixed

- *(deps)* update rust crate zip to v4
- remove needless borrow in archive path handling

### Other

- *(deps)* update softprops/action-gh-release action to v2

## [0.2.1](https://github.com/loonghao/shimexe/compare/v0.2.0...v0.2.1) - 2025-06-17

### Fixed

- resolve clippy warnings and implement Default trait for ConfigCache

## [0.2.0](https://github.com/loonghao/shimexe/compare/v0.1.4...v0.2.0) - 2025-06-17

### Added

- enhance HTTP URL support with persistent download tracking
- add HTTP URL support for automatic executable download

### Fixed

- optimize release workflow trigger conditions

### Other

- move GitHub token validation to CI workflow
- optimize CI/CD workflows and fix GitHub API auth
- update README and add comprehensive unit tests

## [0.1.4](https://github.com/loonghao/shimexe/compare/v0.1.3...v0.1.4) - 2025-06-17

### Added

- enhance CI workflow with cross-compilation testing
- enhance release workflow with advanced cross-compilation
- simplify release workflow with actions-rust-release

### Fixed

- resolve OpenSSL cross-compilation issues with rustls
- replace winget with chocolatey for ImageMagick installation
- enable release workflow triggering with PAT token

### Other

- optimize dependency installation with unified approach

## [0.1.3](https://github.com/loonghao/shimexe/compare/v0.1.2...v0.1.3) - 2025-06-17

### Added

- enhance release-plz validation with version update check
- modernize GitHub Actions to latest versions

### Fixed

- use official release-plz GitHub Action
- add git-token parameter to release-plz dry-run
- resolve git detached HEAD issue in release-plz dry-run
- update release-plz configuration and add dry-run checks
- improve release workflow to handle existing releases
- update release-plz configuration for standard tag format

### Other

- simplify PR validation to config-only check
- move release-plz dry-run to CI workflow
- move shimexe-cli to root src directory

## [0.1.2](https://github.com/loonghao/shimexe/compare/shimexe-v0.1.1...shimexe-v0.1.2) - 2025-06-16

### Added

- implement standalone shim execution with local config lookup

## [0.1.1](https://github.com/loonghao/shimexe/compare/shimexe-v0.1.0...shimexe-v0.1.1) - 2025-06-16

### Added

- setup release-please automation and add crate READMEs

## [0.1.0](https://github.com/loonghao/shimexe/releases/tag/shimexe-v0.1.0) - 2025-06-16

### Added

- enhance shim configuration with dynamic template system and improved args handling
