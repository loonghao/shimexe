# Package Manager Configurations

This directory contains all package manager configurations and related files for shimexe.

## 📁 Directory Structure

```
@pkg/
├── homebrew/           # Homebrew Formula
│   └── shimexe.rb
├── scoop/             # Scoop Manifest
│   └── shimexe.json
├── chocolatey/        # Chocolatey Package
│   ├── shimexe.nuspec.template
│   └── tools/
│       ├── chocolateyinstall.ps1.template
│       └── chocolateyuninstall.ps1.template
├── scripts/           # Package Management Scripts
│   ├── update-package-managers.sh
│   └── update-package-managers.ps1
└── README.md          # This file
```

## 🍺 Homebrew

**Location**: `@pkg/homebrew/shimexe.rb`

Homebrew formula for macOS and Linux installation. This file is automatically updated by the release workflow and copied to the `loonghao/homebrew-tap` repository.

**Installation**:
```bash
brew tap loonghao/tap
brew install shimexe
```

## 🥄 Scoop

**Location**: `@pkg/scoop/shimexe.json`

Scoop manifest for Windows installation. This file is automatically updated by the release workflow and copied to the `loonghao/scoop-bucket` repository.

**Installation**:
```powershell
scoop bucket add loonghao https://github.com/loonghao/scoop-bucket
scoop install shimexe
```

## 🍫 Chocolatey

**Location**: `@pkg/chocolatey/`

Chocolatey package templates that are processed during the release workflow to create the final package.

**Templates**:
- `shimexe.nuspec.template` - Package metadata
- `tools/chocolateyinstall.ps1.template` - Installation script
- `tools/chocolateyuninstall.ps1.template` - Uninstallation script

**Installation**:
```powershell
choco install shimexe
```

## 🔧 Scripts

**Location**: `@pkg/scripts/`

Utility scripts for managing package configurations:

- `update-package-managers.sh` - Unix/Linux/macOS version updater
- `update-package-managers.ps1` - Windows PowerShell version updater

**Usage**:
```bash
# Unix/Linux/macOS
./pkg/scripts/update-package-managers.sh 0.3.6

# Windows
.\@pkg\scripts\update-package-managers.ps1 0.3.6
```

## 🚀 Automated Release Process

When a version tag (e.g., `v0.3.6`) is pushed, the GitHub Actions workflow automatically:

1. **Builds** binaries for all platforms
2. **Creates** GitHub release with binaries
3. **Updates** Homebrew formula in the tap repository
4. **Creates and publishes** Chocolatey package
5. **Updates** Scoop manifest in the bucket repository

## 🛠️ Manual Updates

To manually update package configurations:

1. **Update version numbers** using the scripts in `@pkg/scripts/`
2. **Test locally** if possible
3. **Commit changes** to the repository
4. **Create and push** a version tag to trigger automated release

## 📋 Template Variables

The Chocolatey templates use the following variables:

- `{{VERSION}}` - Version number (e.g., "0.3.6")
- `{{CHECKSUM_X64}}` - SHA256 checksum for x64 binary
- `{{CHECKSUM_ARM64}}` - SHA256 checksum for ARM64 binary

These are automatically replaced during the release process.

## 🔗 Related Repositories

- **Homebrew Tap**: https://github.com/loonghao/homebrew-tap
- **Scoop Bucket**: https://github.com/loonghao/scoop-bucket
- **Chocolatey Package**: https://chocolatey.org/packages/shimexe

## 📝 Notes

- All templates use placeholder values that are replaced during the release process
- The `@pkg` directory name uses the `@` prefix to clearly distinguish it as a special directory
- Package manager configurations are kept separate from the main source code for better organization
- Scripts support both Unix-style and Windows-style path separators
