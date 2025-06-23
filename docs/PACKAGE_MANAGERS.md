# Package Manager Setup Guide

This document explains how to set up and maintain shimexe across different package managers.

## 🍺 Homebrew Setup

### Prerequisites
1. Create a Homebrew tap repository: `https://github.com/loonghao/homebrew-tap`
2. Set up `HOMEBREW_TAP_TOKEN` secret in GitHub repository settings

### Repository Structure
```
homebrew-tap/
├── Formula/
│   └── shimexe.rb
└── README.md
```

### Secrets Required
- `HOMEBREW_TAP_TOKEN`: Personal access token with `repo` scope for the tap repository

### Manual Update Process
1. Update the formula in `homebrew/shimexe.rb`
2. Copy to the tap repository: `cp homebrew/shimexe.rb ../homebrew-tap/Formula/`
3. Commit and push to the tap repository

## 🍫 Chocolatey Setup

### Prerequisites
1. Create a Chocolatey account at https://chocolatey.org/
2. Get API key from your Chocolatey profile
3. Set up `CHOCOLATEY_API_KEY` secret in GitHub repository settings

### Package Structure
The release workflow automatically creates:
```
chocolatey/
├── shimexe.nuspec
└── tools/
    ├── chocolateyinstall.ps1
    └── chocolateyuninstall.ps1
```

### Secrets Required
- `CHOCOLATEY_API_KEY`: API key from your Chocolatey account
- `CHOCOLATEY_LICENSE_KEY`: (Optional) For Chocolatey Pro features

### Manual Update Process
1. Update version in the workflow or use the update script
2. The workflow will automatically create and push the package

## 🥄 Scoop Setup

### Prerequisites
1. Create a Scoop bucket repository: `https://github.com/loonghao/scoop-bucket`
2. Set up `SCOOP_BUCKET_TOKEN` secret in GitHub repository settings

### Repository Structure
```
scoop-bucket/
├── bucket/
│   └── shimexe.json
└── README.md
```

### Secrets Required
- `SCOOP_BUCKET_TOKEN`: Personal access token with `repo` scope for the bucket repository

### Manual Update Process
1. Update the manifest in `scoop/shimexe.json`
2. Copy to the bucket repository: `cp scoop/shimexe.json ../scoop-bucket/bucket/`
3. Commit and push to the bucket repository

## 🔄 Automated Release Process

When you push a tag (e.g., `v0.3.6`), the GitHub Actions workflow will:

1. **Build binaries** for all platforms
2. **Create GitHub release** with binaries
3. **Update Homebrew formula** in the tap repository
4. **Create and publish Chocolatey package**
5. **Update Scoop manifest** in the bucket repository

## 🛠️ Manual Version Updates

Use the provided scripts to update all package managers at once:

### Unix/Linux/macOS
```bash
chmod +x @pkg/scripts/update-package-managers.sh
./@pkg/scripts/update-package-managers.sh 0.3.6
```

### Windows
```powershell
.\@pkg\scripts\update-package-managers.ps1 0.3.6
```

## 📊 Download Statistics

Track downloads across platforms:

- **Crates.io**: https://crates.io/crates/shimexe
- **GitHub Releases**: https://github.com/loonghao/shimexe/releases
- **Homebrew**: https://formulae.brew.sh/formula/shimexe
- **Chocolatey**: https://chocolatey.org/packages/shimexe

## 🔍 Troubleshooting

### Homebrew Issues
- Ensure the tap repository exists and is public
- Check that `HOMEBREW_TAP_TOKEN` has correct permissions
- Verify the formula syntax with `brew audit --strict shimexe`

### Chocolatey Issues
- Verify API key is valid and has push permissions
- Check package validation at https://chocolatey.org/packages/shimexe
- Ensure binary URLs are accessible

### Scoop Issues
- Ensure the bucket repository exists and is public
- Check that `SCOOP_BUCKET_TOKEN` has correct permissions
- Verify manifest syntax with `scoop checkver shimexe`

## 📝 Adding New Package Managers

To add support for additional package managers:

1. Create the package configuration files
2. Add a new job to `.github/workflows/release.yml`
3. Set up required secrets in repository settings
4. Update this documentation
5. Add badges to README.md

## 🔗 Useful Links

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Chocolatey Package Creation](https://docs.chocolatey.org/en-us/create/create-packages)
- [Scoop Manifest Reference](https://github.com/ScoopInstaller/Scoop/wiki/App-Manifests)
- [GitHub Actions Marketplace](https://github.com/marketplace?type=actions)
