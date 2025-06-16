# Release Workflow Architecture

This document explains the release automation architecture for the shimexe project.

## Overview

We use a **two-workflow system** to handle different aspects of the release process:

1. **`release-plz.yml`** - Handles version management and crates.io publishing
2. **`release.yml`** - Handles binary distribution and package management

## Workflow Responsibilities

### 🦀 release-plz.yml (Rust-specific)

**Purpose**: Automated version management and crates.io publishing

**Triggers**:
- Push to `main` branch
- Manual dispatch

**Responsibilities**:
- ✅ Analyzes commit messages (conventional commits)
- ✅ Bumps version numbers in Cargo.toml files
- ✅ Updates CHANGELOG.md files
- ✅ Creates release PRs with version changes
- ✅ Creates GitHub releases when PRs are merged
- ✅ Publishes crates to crates.io
- ✅ Creates git tags (e.g., `shimexe-core-v0.1.1`)

**Token**: Uses `RELEASE_PLZ_TOKEN` or `GITHUB_TOKEN`

### 📦 release.yml (Binary distribution)

**Purpose**: Cross-platform binary builds and distribution

**Triggers**:
- Tag pushes (`v*`)
- Release published events
- Manual dispatch

**Responsibilities**:
- ✅ Creates GitHub release (if triggered by tag)
- ✅ Builds cross-platform binaries:
  - Windows (x86_64)
  - Linux (x86_64, x86_64-musl)
  - macOS (x86_64, aarch64)
- ✅ Uploads binaries to GitHub releases
- ✅ Publishes to Chocolatey package manager

**Token**: Uses `GITHUB_TOKEN`

## Release Flow

### Automatic Release (Recommended)

1. **Developer commits** with conventional commit message:
   ```bash
   git commit -m "feat: add new shim configuration option"
   ```

2. **release-plz.yml triggers** on push to main:
   - Analyzes commits since last release
   - Determines version bump (patch/minor/major)
   - Creates release PR with updated versions

3. **Maintainer merges** the release PR:
   - release-plz.yml triggers again
   - Creates GitHub release
   - Publishes crates to crates.io
   - Creates git tag (e.g., `shimexe-core-v0.1.2`)

4. **release.yml triggers** on tag creation:
   - Builds cross-platform binaries
   - Uploads to GitHub release
   - Publishes to Chocolatey

### Manual Release

If you need to create a release manually:

1. **Create and push a tag**:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **release.yml will trigger** and:
   - Create GitHub release
   - Build and upload binaries
   - Publish to Chocolatey

## Tag Naming Convention

- **Workspace releases**: `v1.0.0` (for main CLI tool)
- **Individual crates**: `shimexe-core-v0.1.1`, `shimexe-v1.0.0`

## Supported Platforms

### Binary Releases
- **Windows**: `shimexe-windows-x86_64.exe`
- **Linux**: `shimexe-linux-x86_64`, `shimexe-linux-x86_64-musl`
- **macOS**: `shimexe-macos-x86_64`, `shimexe-macos-aarch64`

### Package Managers
- **Rust**: crates.io (via release-plz)
- **Windows**: Chocolatey (via release.yml)

## Configuration Requirements

### Secrets
- `RELEASE_PLZ_TOKEN` - Personal Access Token for release-plz
- `CARGO_REGISTRY_TOKEN` - Token for crates.io publishing
- `CHOCOLATEY_API_KEY` - API key for Chocolatey publishing

### Repository Settings
- Enable "Allow GitHub Actions to create and approve pull requests"
- Set workflow permissions to "Read and write permissions"

## Troubleshooting

### No Release Created
- Check that commits use conventional commit format
- Verify repository permissions are correctly set
- Check workflow logs for detailed error messages

### Binary Upload Failed
- Ensure tag follows `v*` pattern
- Check that GitHub release was created successfully
- Verify workflow permissions include `contents: write`

### Crates.io Publishing Failed
- Check `CARGO_REGISTRY_TOKEN` is valid and not expired
- Ensure crate versions are properly incremented
- Verify no duplicate versions exist on crates.io

### Chocolatey Publishing Failed
- Check `CHOCOLATEY_API_KEY` is valid
- Ensure package version doesn't already exist
- Verify Windows binary was uploaded successfully

## Workflow Files

- `.github/workflows/release-plz.yml` - Version management and crates.io
- `.github/workflows/release.yml` - Binary distribution
- `.github/workflows/ci.yml` - Continuous integration testing

## Benefits of This Architecture

✅ **Separation of Concerns**: Each workflow has a clear, focused responsibility  
✅ **Rust-Native**: release-plz understands Rust workspaces and conventions  
✅ **Cross-Platform**: Comprehensive binary distribution  
✅ **Automated**: Minimal manual intervention required  
✅ **Flexible**: Supports both automatic and manual releases  
✅ **Reliable**: Explicit permissions and error handling  
