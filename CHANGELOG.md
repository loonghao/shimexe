# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.5.5](https://github.com/loonghao/shimexe/compare/shimexe-v0.5.4...shimexe-v0.5.5) - 2025-07-06

### Fixed

- completely disable codecov status checks to prevent CI failures

## [0.5.4](https://github.com/loonghao/shimexe/compare/shimexe-v0.5.3...shimexe-v0.5.4) - 2025-07-06

### Fixed

- make codecov informational only, don't fail CI
- resolve package manager publishing issues

### Other

- *(deps)* update peter-evans/create-pull-request action to v7

## [0.5.3](https://github.com/loonghao/shimexe/compare/shimexe-v0.5.2...shimexe-v0.5.3) - 2025-07-06

### Fixed

- optimize release-plz config for mixed workspace

## [0.5.2](https://github.com/loonghao/shimexe/compare/shimexe-v0.5.1...shimexe-v0.5.2) - 2025-07-06

### Added

- enhance package publishing and fix env config

## [0.5.1](https://github.com/loonghao/shimexe/compare/shimexe-v0.5.0...shimexe-v0.5.1) - 2025-07-05

### Fixed

- resolve workflow conflicts between release.yml and update-packages.yml

## [0.5.0](https://github.com/loonghao/shimexe/compare/shimexe-v0.4.0...shimexe-v0.5.0) - 2025-07-05

### Added

- disable clippy uninlined_format_args lint
- upgrade turbo-cdn to 0.4.3 and improve verbose logging control

### Fixed

- resolve clippy and rustdoc warnings for CI compatibility

### Other

- *(deps)* update crazy-max/ghaction-chocolatey action to v3.4.0

## [0.4.0](https://github.com/loonghao/shimexe/compare/v0.3.5...v0.4.0) - 2025-06-23

### Added

- enhance auto-update with turbo-cdn integration
- add multi-platform package management and enhanced badges
- integrate turbo-cdn and restructure tests

### Fixed

- resolve CI configuration and testing issues

### Other

- rename @pkg to pkg directory and fix code quality
- reorganize package manager files to @pkg directory

## [0.3.5](https://github.com/loonghao/shimexe/compare/v0.3.4...v0.3.5) - 2025-06-19

### Other

- *(deps)* update rust crate thiserror to v2

## [0.3.4](https://github.com/loonghao/shimexe/compare/v0.3.3...v0.3.4) - 2025-06-18

### Fixed

- support both v* and shimexe-v* tag formats in install scripts
- improve version detection and release changelog formatting
- configure release-plz to show only current version changelog in releases
- resolve version detection issues in install scripts and update Scoop manifest

## [0.3.3](https://github.com/loonghao/shimexe/compare/v0.3.2...v0.3.3) - 2025-06-18

### Added

- add comprehensive vx integration example
- add high-level ShimManager API and update Chinese README
- enhance README with logo and improve vx integration

### Fixed

- resolve clippy warnings and compilation errors
- correct PATH usage and improve user guidance
- add retry logic and fallback for GitHub API rate limits

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
