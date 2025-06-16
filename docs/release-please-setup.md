# GitHub Actions & Release Automation Setup Guide

This document explains how to configure GitHub Actions permissions and automated release tools for the shimexe project.

## 🚨 Critical Permission Issues

If you encounter these errors:

```
GitHub Actions is not permitted to create or approve pull requests.
Error: Process completed with exit code 1.
```

```
remote: Permission to git denied to github-actions[bot].
fatal: unable to access 'https://github.com/...': The requested URL returned error: 403
```

This means GitHub Actions doesn't have sufficient permissions in your repository.

## 🔧 Release Tools in This Project

We use **TWO** release automation tools:

1. **release-please** - Google's general-purpose release tool
2. **release-plz** - Rust-specific release tool

Both tools can create PRs, so they need proper permissions.

## 🛠️ Complete Solution (3 Steps Required)

### Step 1: Repository Settings (CRITICAL)

**You MUST enable this setting for any release automation to work:**

1. Go to your repository **Settings**
2. Navigate to **Actions** → **General**
3. Scroll down to **Workflow permissions**
4. Select **"Read and write permissions"**
5. ✅ **Enable "Allow GitHub Actions to create and approve pull requests"**
6. Click **Save**

**Without this setting, ALL release tools will fail with 403 errors.**

### Step 2: Personal Access Tokens (RECOMMENDED)

Even with repository settings enabled, using Personal Access Tokens provides better reliability:

#### For release-please:
1. **Create a Personal Access Token:**
   - Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
   - Click "Generate new token (classic)"
   - Name: "Release Please Token"
   - Expiration: Choose appropriate duration
   - Select these scopes:
     - ✅ `repo` (Full control of private repositories)
     - ✅ `workflow` (Update GitHub Action workflows)
     - ✅ `write:packages` (Upload packages to GitHub Package Registry)

2. **Add as repository secret:**
   - Repository Settings → **Secrets and variables** → **Actions**
   - Click **New repository secret**
   - Name: `RELEASE_PLEASE_TOKEN`
   - Value: Your PAT
   - Click **Add secret**

#### For release-plz:
1. **Create another PAT** (or reuse the same one):
   - Same scopes as above
2. **Add as repository secret:**
   - Name: `RELEASE_PLZ_TOKEN`
   - Value: Your PAT

### Step 3: Verify All Secrets

Ensure you have these secrets configured:
- ✅ `RELEASE_PLEASE_TOKEN` (for release-please)
- ✅ `RELEASE_PLZ_TOKEN` (for release-plz)
- ✅ `CARGO_REGISTRY_TOKEN` (for crates.io publishing)
- ✅ `CHOCOLATEY_API_KEY` (for Chocolatey publishing)

## 🔐 Workflow Permissions Explained

All workflows now have explicit permissions configured:

### CI Workflow (`ci.yml`)
```yaml
permissions:
  contents: read        # Read repository contents
  actions: read         # Read workflow status
  security-events: write # Write security audit results
```

### Release Please (`release-please.yml`)
```yaml
permissions:
  contents: write           # Create releases and tags
  pull-requests: write      # Create and update PRs
  issues: write            # Create and update issues
  repository-projects: write # Update project boards
```

### Release PLZ (`release-plz.yml`)
```yaml
permissions:
  contents: write           # Create releases and tags
  pull-requests: write      # Create and update PRs
  issues: write            # Create and update issues
  repository-projects: write # Update project boards
```

### Release Build (`release.yml`)
```yaml
permissions:
  contents: write    # Upload release assets
  actions: read      # Read workflow status
  id-token: write    # For OIDC authentication
```

## 🚀 How Release Automation Works

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

## 🔍 Troubleshooting Guide

### 403 Permission Errors

**Error**: `GitHub Actions is not permitted to create or approve pull requests`

**Solutions** (try in order):
1. ✅ Enable "Allow GitHub Actions to create and approve pull requests" in repository settings
2. ✅ Verify workflow permissions are correctly set
3. ✅ Create and configure Personal Access Tokens
4. ✅ Check that PAT has correct scopes (`repo`, `workflow`, `write:packages`)
5. ✅ Ensure PAT hasn't expired

### Token Authentication Issues

**Error**: `remote: Permission to git denied to github-actions[bot]`

**Solutions**:
1. Use Personal Access Token instead of `GITHUB_TOKEN`
2. Verify token has `repo` scope
3. Check token expiration date
4. Ensure token is added as correct secret name

### Workflow Conflicts

**Problem**: Both release-please and release-plz running simultaneously

**Solutions**:
1. **Recommended**: Use only one release tool
2. **Alternative**: Configure different trigger conditions
3. **Current setup**: Both tools use different tokens to avoid conflicts

### "is not a package manifest" Error
This usually means the workspace configuration is incorrect. Ensure:
- `Cargo.toml` has proper `[workspace]` section
- Each crate has a valid `Cargo.toml`
- Paths in `.release-please-manifest.json` match actual crate directories

### No Release PR Created
Release tools only create PRs when they detect commits that should trigger a release:
- Make sure you're using conventional commit messages
- Check that commits are on the `main` branch
- Verify the workflow has proper permissions
- Check workflow logs for detailed error messages

### Multiple Releases
If you want to release only specific crates, you can:
- Use path-based commit prefixes: `feat(shimexe-core): add new feature`
- Manually edit the release PR to exclude certain crates

### Debugging Steps

1. **Check workflow logs** in Actions tab
2. **Verify repository settings** are correct
3. **Test with a simple conventional commit**
4. **Check secret configuration**
5. **Verify token permissions and expiration**

## Manual Release

If you need to create a release manually:

1. Update version in `Cargo.toml` files
2. Update `CHANGELOG.md` files
3. Commit changes with conventional commit message
4. Create and merge PR
5. Release-please will detect the version change and create a release
