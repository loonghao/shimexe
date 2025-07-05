# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.5.0](https://github.com/loonghao/shimexe/compare/shimexe-core-v0.4.0...shimexe-core-v0.5.0) - 2025-07-05

### Added

- disable clippy uninlined_format_args lint
- upgrade turbo-cdn to 0.4.3 and improve verbose logging control

### Fixed

- resolve remaining clippy uninlined format args warnings
- resolve clippy and rustdoc warnings for CI compatibility
- adjust performance test timeouts for Windows environment

## [0.4.0](https://github.com/loonghao/shimexe/compare/shimexe-core-v0.3.5...shimexe-core-v0.4.0) - 2025-06-23

### Added

- enhance auto-update with turbo-cdn integration
- integrate turbo-cdn and restructure tests

### Fixed

- resolve CI configuration and testing issues

### Other

- rename @pkg to pkg directory and fix code quality

## [0.3.5](https://github.com/loonghao/shimexe/compare/shimexe-core-v0.3.4...shimexe-core-v0.3.5) - 2025-06-19

### Other

- update Cargo.toml dependencies

## [0.3.3](https://github.com/loonghao/shimexe/compare/shimexe-core-v0.3.2...shimexe-core-v0.3.3) - 2025-06-18

### Added

- add high-level ShimManager API and update Chinese README

### Fixed

- resolve clippy warnings and compilation errors

## [0.3.0](https://github.com/loonghao/shimexe/compare/shimexe-core-v0.2.1...shimexe-core-v0.3.0) - 2025-06-17

### Added

- add comprehensive archive support and package management

### Fixed

- *(deps)* update rust crate zip to v4

## [0.2.1](https://github.com/loonghao/shimexe/compare/shimexe-core-v0.2.0...shimexe-core-v0.2.1) - 2025-06-17

### Added

- optimize performance and fix workflow

### Fixed

- resolve clippy warnings and implement Default trait for ConfigCache

## [0.2.0](https://github.com/loonghao/shimexe/compare/shimexe-core-v0.1.4...shimexe-core-v0.2.0) - 2025-06-17

### Added

- enhance HTTP URL support with persistent download tracking
- add HTTP URL support for automatic executable download

### Other

- update README and add comprehensive unit tests

## [0.1.4](https://github.com/loonghao/shimexe/compare/shimexe-core-v0.1.3...shimexe-core-v0.1.4) - 2025-06-17

### Fixed

- resolve OpenSSL cross-compilation issues with rustls

## [0.1.1](https://github.com/loonghao/shimexe/compare/shimexe-core-v0.1.0...shimexe-core-v0.1.1) - 2025-06-16

### Added

- setup release-please automation and add crate READMEs

## [0.1.0](https://github.com/loonghao/shimexe/releases/tag/shimexe-core-v0.1.0) - 2025-06-16

### Added

- enhance shim configuration with dynamic template system and improved args handling
