# Release Please Setup Guide

This document explains how to configure release-please for automated version management and releases.

## Problem: GitHub Actions Permission Error

If you encounter this error:

```
GitHub Actions is not permitted to create or approve pull requests.
Error: Process completed with exit code 1.
```

This means GitHub Actions doesn't have permission to create Pull Requests in your repository.

## Solutions

### Option 1: Enable GitHub Actions to Create PRs (Recommended)

1. Go to your repository settings
2. Navigate to **Actions** → **General**
3. Scroll down to **Workflow permissions**
4. Enable **"Allow GitHub Actions to create and approve pull requests"**
5. Click **Save**

This is the simplest solution and works with the default `GITHUB_TOKEN`.

### Option 2: Use Personal Access Token

If you prefer not to enable the repository setting, you can use a Personal Access Token:

1. **Create a Personal Access Token:**
   - Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
   - Click "Generate new token (classic)"
   - Give it a descriptive name like "Release Please Token"
   - Select these scopes:
     - `repo` (Full control of private repositories)
     - `workflow` (Update GitHub Action workflows)

2. **Add the token as a repository secret:**
   - Go to your repository settings
   - Navigate to **Secrets and variables** → **Actions**
   - Click **New repository secret**
   - Name: `RELEASE_PLEASE_TOKEN`
   - Value: Your personal access token
   - Click **Add secret**

3. The workflow will automatically use this token when available.

## How Release Please Works

1. **Commit Analysis**: Release-please analyzes commit messages following [Conventional Commits](https://www.conventionalcommits.org/)
2. **Version Bumping**: Based on commit types, it determines the appropriate version bump:
   - `feat:` → Minor version bump (0.1.0 → 0.2.0)
   - `fix:` → Patch version bump (0.1.0 → 0.1.1)
   - `feat!:` or `BREAKING CHANGE:` → Major version bump (0.1.0 → 1.0.0)
3. **Release PR Creation**: Creates a PR with updated versions and changelog
4. **Release Creation**: When the PR is merged, creates a GitHub release

## Conventional Commit Examples

```bash
# Feature (minor version bump)
git commit -m "feat: add new shim configuration option"

# Bug fix (patch version bump)
git commit -m "fix: resolve path resolution issue on Windows"

# Breaking change (major version bump)
git commit -m "feat!: change configuration file format"
# or
git commit -m "feat: change configuration file format

BREAKING CHANGE: Configuration files now use YAML instead of TOML"

# Other types (no version bump)
git commit -m "docs: update README with new examples"
git commit -m "chore: update dependencies"
git commit -m "ci: improve release workflow"
```

## Workspace Configuration

Our project uses a Rust workspace with two crates:
- `crates/shimexe-cli` (package: `shimexe`) - The CLI tool
- `crates/shimexe-core` (package: `shimexe-core`) - The core library

Release-please will:
- Track versions independently for each crate
- Create separate changelog entries
- Release both crates when their versions change

## Troubleshooting

### "is not a package manifest" Error
This usually means the workspace configuration is incorrect. Ensure:
- `Cargo.toml` has proper `[workspace]` section
- Each crate has a valid `Cargo.toml`
- Paths in `.release-please-manifest.json` match actual crate directories

### No Release PR Created
Release-please only creates PRs when it detects commits that should trigger a release:
- Make sure you're using conventional commit messages
- Check that commits are on the `main` branch
- Verify the workflow has proper permissions

### Multiple Releases
If you want to release only specific crates, you can:
- Use path-based commit prefixes: `feat(shimexe-core): add new feature`
- Manually edit the release PR to exclude certain crates

## Manual Release

If you need to create a release manually:

1. Update version in `Cargo.toml` files
2. Update `CHANGELOG.md` files
3. Commit changes with conventional commit message
4. Create and merge PR
5. Release-please will detect the version change and create a release
