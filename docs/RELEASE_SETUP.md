# Release Setup Guide

This document explains how to set up the release workflow for shimexe, including the required GitHub secrets and token configuration.

## Overview

The shimexe project uses two workflows for releases:

1. **release-plz.yml**: Manages version updates and creates GitHub releases
2. **release.yml**: Builds cross-platform binaries using advanced Rust tooling:
   - `houseabsolute/actions-rust-cross`: Cross-compilation for multiple platforms
   - `houseabsolute/actions-rust-release`: Automated release management
   - `crazy-max/ghaction-chocolatey`: Chocolatey package publishing

## Supported Platforms

The release workflow builds binaries for the following platforms:

### Linux
- **x86_64-unknown-linux-gnu**: Standard Linux x86_64 (glibc)
- **x86_64-unknown-linux-musl**: Linux x86_64 with musl (static linking)
- **aarch64-unknown-linux-gnu**: Linux ARM64 (glibc)
- **aarch64-unknown-linux-musl**: Linux ARM64 with musl (static linking)

### Windows
- **x86_64-pc-windows-msvc**: Windows x86_64
- **aarch64-pc-windows-msvc**: Windows ARM64

### macOS
- **x86_64-apple-darwin**: macOS Intel
- **aarch64-apple-darwin**: macOS Apple Silicon (M1/M2)

### FreeBSD
- **x86_64-unknown-freebsd**: FreeBSD x86_64 (cross-compiled from Linux)

All platforms are built using `houseabsolute/actions-rust-cross` which provides robust cross-compilation support with proper toolchain management and caching.

## Required GitHub Secrets

### 1. RELEASE_PLZ_TOKEN (Required for triggering workflows)

This Personal Access Token (PAT) allows release-plz to trigger the binary build workflow.

**Why needed**: GitHub Actions using the default `GITHUB_TOKEN` cannot trigger other workflows. Without this PAT, release-plz will create tags and releases but won't trigger the binary build workflow (release.yml).

**How to create**:

1. Go to GitHub Settings → Developer settings → Personal access tokens → Fine-grained tokens
2. Click "Generate new token"
3. Configure the token:
   - **Repository access**: Select only the shimexe repository
   - **Permissions**:
     - Contents: Read and write
     - Pull requests: Read and write
     - Actions: Read (to trigger workflows)
4. Copy the generated token
5. Add it to repository secrets as `RELEASE_PLZ_TOKEN`

### 2. CARGO_REGISTRY_TOKEN (Required for crates.io publishing)

This token allows publishing packages to crates.io.

**How to create**:

1. Go to [crates.io](https://crates.io/) and log in
2. Go to Account Settings → API Tokens
3. Create a new token with scopes:
   - `publish-new`: Allow publishing new crates
   - `publish-update`: Allow updating existing crates
4. Copy the token
5. Add it to repository secrets as `CARGO_REGISTRY_TOKEN`

### 3. CHOCOLATEY_API_KEY (Optional, for Windows package manager)

This token allows publishing to the Chocolatey package manager.

**How to create**:

1. Create account on [chocolatey.org](https://chocolatey.org/)
2. Go to your profile → API Keys
3. Create a new API key
4. Add it to repository secrets as `CHOCOLATEY_API_KEY`

## Workflow Process

### Automatic Release Process

1. **Developer pushes commits** to main branch with conventional commit messages:
   - `feat:` for new features (minor version bump)
   - `fix:` for bug fixes (patch version bump)
   - `BREAKING CHANGE:` for breaking changes (major version bump)

2. **release-plz-pr job** runs and:
   - Analyzes commits since last release
   - Determines version bump based on conventional commits
   - Creates a PR with version updates and changelog

3. **Developer merges the release PR**

4. **release-plz-release job** runs and:
   - Publishes packages to crates.io
   - Creates GitHub release with changelog
   - Creates git tag

5. **release.yml workflow** is triggered by the release event and:
   - Uses `houseabsolute/actions-rust-cross` for cross-platform compilation
   - Supports 9 different platforms including FreeBSD and ARM architectures
   - Uses `houseabsolute/actions-rust-release` for automated release management
   - Automatically creates archives with checksums for all platforms
   - Uses `crazy-max/ghaction-chocolatey` for simplified Chocolatey publishing
   - Uploads binaries to the GitHub release

### Manual Release Process

If automatic process fails, you can manually trigger releases:

1. **Manual workflow dispatch**: Go to Actions → Release-plz → Run workflow
2. **Manual binary build**: Go to Actions → Release → Run workflow with tag name

## Troubleshooting

### Release created but no binaries

**Problem**: release-plz creates tags and releases but the binary build workflow doesn't run.

**Root Cause**: GitHub Actions security limitation - workflows triggered by the default `GITHUB_TOKEN` cannot trigger other workflows.

**Solution**:
1. Check if `RELEASE_PLZ_TOKEN` is set in repository secrets
2. Verify the token has correct permissions (Contents + Pull requests + Actions)
3. Check if the token is expired
4. Ensure the release workflow triggers on `push: tags: - 'v*'` (not just release events)

### Binary build fails

**Common issues**:
1. **Missing dependencies**: Check if ImageMagick installation steps work
2. **Cross-compilation errors**: Check if target platforms are correctly configured
3. **Upload failures**: Verify the release exists and workflow has write permissions

### Chocolatey publishing fails

**Common issues**:
1. **Missing API key**: Check if `CHOCOLATEY_API_KEY` is set
2. **Package validation**: Chocolatey has strict validation rules
3. **Version conflicts**: Package version might already exist

## Testing

To test the release process:

1. Create a test branch with version changes
2. Use workflow_dispatch to manually trigger workflows
3. Check logs for any errors
4. Verify all artifacts are created correctly

## Security Notes

- Use fine-grained PATs instead of classic tokens when possible
- Regularly rotate API tokens
- Monitor token usage in GitHub audit logs
- Never commit tokens to the repository
