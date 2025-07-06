# Enhanced PowerShell script to publish shimexe packages to various package managers
# Usage: .\scripts\publish-packages-enhanced.ps1 -Version "0.1.3" -GitHubToken "token" [-PublishChocolatey] [-PublishScoop] [-PublishHomebrew]

param(
    [Parameter(Mandatory=$true)]
    [string]$Version,
    
    [Parameter(Mandatory=$true)]
    [string]$GitHubToken,
    
    [switch]$PublishChocolatey,
    [switch]$PublishScoop,
    [switch]$PublishHomebrew,
    [switch]$DryRun,
    
    [string]$ChocolateyApiKey = $env:CHOCOLATEY_API_KEY,
    [string]$ScoopBucketRepo = "loonghao/scoop-bucket",
    [string]$HomebrewTapRepo = "loonghao/homebrew-tap"
)

$ErrorActionPreference = "Stop"

# Configuration
$RepoOwner = "loonghao"
$RepoName = "shimexe"
$BaseUrl = "https://github.com/$RepoOwner/$RepoName"

Write-Host "Publishing shimexe packages for version $Version" -ForegroundColor Green

# Function to get file hash
function Get-FileHash256 {
    param([string]$Url)
    
    try {
        $tempFile = [System.IO.Path]::GetTempFileName()
        Invoke-WebRequest -Uri $Url -OutFile $tempFile -UseBasicParsing
        $hash = Get-FileHash -Path $tempFile -Algorithm SHA256
        Remove-Item $tempFile -Force
        return $hash.Hash.ToLower()
    }
    catch {
        Write-Warning "Failed to get hash for $Url: $_"
        return ""
    }
}

# Function to update GitHub repository file
function Update-GitHubFile {
    param(
        [string]$Repo,
        [string]$Path,
        [string]$Content,
        [string]$Message,
        [string]$Token,
        [string]$Branch = "main"
    )
    
    $headers = @{
        "Authorization" = "token $Token"
        "Accept" = "application/vnd.github.v3+json"
        "User-Agent" = "shimexe-publisher"
    }
    
    # Get current file to get SHA
    $getUrl = "https://api.github.com/repos/$Repo/contents/$Path"
    try {
        $currentFile = Invoke-RestMethod -Uri $getUrl -Headers $headers -Method Get
        $sha = $currentFile.sha
    }
    catch {
        # File doesn't exist, create new
        $sha = $null
    }
    
    # Prepare update payload
    $payload = @{
        "message" = $Message
        "content" = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes($Content))
        "branch" = $Branch
    }
    
    if ($sha) {
        $payload["sha"] = $sha
    }
    
    # Update file
    $updateUrl = "https://api.github.com/repos/$Repo/contents/$Path"
    Invoke-RestMethod -Uri $updateUrl -Headers $headers -Method Put -Body ($payload | ConvertTo-Json) | Out-Null
}

# Define release URLs and calculate hashes
$Assets = @{
    "windows-x64" = @{
        "url" = "$BaseUrl/releases/download/v$Version/shimexe-$Version-x86_64-pc-windows-msvc.zip"
        "extract_dir" = "shimexe-$Version-x86_64-pc-windows-msvc"
    }
    "windows-arm64" = @{
        "url" = "$BaseUrl/releases/download/v$Version/shimexe-$Version-aarch64-pc-windows-msvc.zip"
        "extract_dir" = "shimexe-$Version-aarch64-pc-windows-msvc"
    }
    "macos-x64" = @{
        "url" = "$BaseUrl/releases/download/v$Version/shimexe-$Version-x86_64-apple-darwin.tar.xz"
        "extract_dir" = "shimexe-$Version-x86_64-apple-darwin"
    }
    "macos-arm64" = @{
        "url" = "$BaseUrl/releases/download/v$Version/shimexe-$Version-aarch64-apple-darwin.tar.xz"
        "extract_dir" = "shimexe-$Version-aarch64-apple-darwin"
    }
    "linux-x64" = @{
        "url" = "$BaseUrl/releases/download/v$Version/shimexe-$Version-x86_64-unknown-linux-gnu.tar.xz"
        "extract_dir" = "shimexe-$Version-x86_64-unknown-linux-gnu"
    }
    "linux-arm64" = @{
        "url" = "$BaseUrl/releases/download/v$Version/shimexe-$Version-aarch64-unknown-linux-gnu.tar.xz"
        "extract_dir" = "shimexe-$Version-aarch64-unknown-linux-gnu"
    }
}

Write-Host "Calculating hashes for release assets..." -ForegroundColor Yellow

# Calculate hashes for all assets
$Hashes = @{}
foreach ($platform in $Assets.Keys) {
    $asset = $Assets[$platform]
    Write-Host "  Getting hash for $platform..." -ForegroundColor Gray
    $Hashes[$platform] = Get-FileHash256 -Url $asset.url
    if ($Hashes[$platform]) {
        Write-Host "    $($Hashes[$platform])" -ForegroundColor Green
    } else {
        Write-Error "Failed to get hash for $platform"
    }
}

# Update Chocolatey package
if ($PublishChocolatey) {
    Write-Host "Updating Chocolatey package..." -ForegroundColor Yellow
    
    if (-not $Hashes["windows-x64"]) {
        Write-Error "Windows x64 hash required for Chocolatey"
    }
    
    # Create Chocolatey install script
    $installScript = @"
`$ErrorActionPreference = 'Stop'

`$packageName = 'shimexe'
`$url64 = '$($Assets["windows-x64"].url)'
`$checksum64 = '$($Hashes["windows-x64"])'
`$checksumType64 = 'sha256'

`$packageArgs = @{
    packageName   = `$packageName
    unzipLocation = `$toolsDir
    url64bit      = `$url64
    checksum64    = `$checksum64
    checksumType64= `$checksumType64
}

Install-ChocolateyZipPackage @packageArgs

# Add to PATH
`$binPath = Join-Path `$toolsDir '$($Assets["windows-x64"].extract_dir)'
Install-ChocolateyPath `$binPath
"@

    # Create nuspec content
    $nuspecContent = @"
<?xml version="1.0" encoding="utf-8"?>
<package xmlns="http://schemas.microsoft.com/packaging/2015/06/nuspec.xsd">
  <metadata>
    <id>shimexe</id>
    <version>$Version</version>
    <packageSourceUrl>https://github.com/loonghao/shimexe</packageSourceUrl>
    <owners>loonghao</owners>
    <title>shimexe - The Modern Executable Shim Manager</title>
    <authors>loonghao</authors>
    <projectUrl>https://github.com/loonghao/shimexe</projectUrl>
    <iconUrl>https://raw.githubusercontent.com/loonghao/shimexe/main/assets/icon.png</iconUrl>
    <copyright>2024 loonghao</copyright>
    <licenseUrl>https://github.com/loonghao/shimexe/blob/main/LICENSE</licenseUrl>
    <requireLicenseAcceptance>false</requireLicenseAcceptance>
    <projectSourceUrl>https://github.com/loonghao/shimexe</projectSourceUrl>
    <docsUrl>https://github.com/loonghao/shimexe/blob/main/README.md</docsUrl>
    <bugTrackerUrl>https://github.com/loonghao/shimexe/issues</bugTrackerUrl>
    <tags>shimexe executable shim manager rust cli tool portable download http</tags>
    <summary>The Modern Executable Shim Manager - Transform any executable into a smart, portable shim</summary>
    <description><![CDATA[
# shimexe - The Modern Executable Shim Manager

**shimexe** is a revolutionary executable shim manager that bridges the gap between local tools and cloud-distributed applications. Create lightweight, portable shims that can automatically download, extract, and execute tools from HTTP URLs - while maintaining the simplicity of local executables.

## Key Features

- **🌐 Cloud-Native**: Download tools directly from GitHub releases, CDNs, or any HTTP URL
- **📦 Smart Archives**: Automatically extract zip files and discover executables
- **🔧 Zero Config**: Smart defaults with powerful customization options
- **🚀 Portable**: Shims work independently without requiring shimexe installation
- **⚡ Fast**: Efficient caching and smart re-download logic
- **🔒 Secure**: Built with Rust and rustls-tls for secure HTTPS connections

## Quick Start

```bash
# Add a tool from GitHub releases
shimexe add uv --path https://github.com/astral-sh/uv/releases/download/0.7.13/uv-x86_64-pc-windows-msvc.zip --add-system-path

# Use it directly
uv --version

# Or through shimexe run
shimexe run uv --version
```

For more information, visit: https://github.com/loonghao/shimexe
    ]]></description>
    <releaseNotes>https://github.com/loonghao/shimexe/releases/tag/v$Version</releaseNotes>
  </metadata>
  <files>
    <file src="tools\**" target="tools" />
  </files>
</package>
"@

    if (-not $DryRun) {
        # Create temporary directory for Chocolatey package
        $chocoTempDir = Join-Path $env:TEMP "shimexe-choco-$Version"
        New-Item -ItemType Directory -Path $chocoTempDir -Force | Out-Null
        New-Item -ItemType Directory -Path "$chocoTempDir\tools" -Force | Out-Null
        
        # Write files
        $installScript | Out-File -FilePath "$chocoTempDir\tools\chocolateyinstall.ps1" -Encoding UTF8
        $nuspecContent | Out-File -FilePath "$chocoTempDir\shimexe.nuspec" -Encoding UTF8
        
        # Pack and push
        Push-Location $chocoTempDir
        try {
            Write-Host "Packing Chocolatey package..." -ForegroundColor Yellow
            $packResult = choco pack shimexe.nuspec
            if ($LASTEXITCODE -ne 0) {
                Write-Error "Failed to pack Chocolatey package. Exit code: $LASTEXITCODE"
                return
            }

            if ($ChocolateyApiKey) {
                Write-Host "Pushing to Chocolatey..." -ForegroundColor Yellow
                $pushResult = choco push "shimexe.$Version.nupkg" --source https://push.chocolatey.org/ --api-key $ChocolateyApiKey
                if ($LASTEXITCODE -eq 0) {
                    Write-Host "Published to Chocolatey successfully!" -ForegroundColor Green
                } else {
                    Write-Error "Failed to push to Chocolatey. Exit code: $LASTEXITCODE"
                }
            } else {
                Write-Warning "CHOCOLATEY_API_KEY not set, skipping push"
            }
        }
        catch {
            Write-Error "Error during Chocolatey publishing: $_"
        }
        finally {
            Pop-Location
            Remove-Item $chocoTempDir -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
    
    Write-Host "Chocolatey package updated successfully" -ForegroundColor Green
}

# Update Scoop package
if ($PublishScoop) {
    Write-Host "Updating Scoop package..." -ForegroundColor Yellow

    if (-not $Hashes["windows-x64"] -or -not $Hashes["windows-arm64"]) {
        Write-Error "Windows hashes required for Scoop"
    }

    # Create Scoop manifest
    $ScoopManifest = @{
        "version" = $Version
        "description" = "The Modern Executable Shim Manager - Transform any executable into a smart, portable shim with HTTP download support"
        "homepage" = "https://github.com/loonghao/shimexe"
        "license" = "MIT"
        "architecture" = @{
            "64bit" = @{
                "url" = $Assets["windows-x64"].url
                "hash" = $Hashes["windows-x64"]
                "extract_dir" = $Assets["windows-x64"].extract_dir
            }
            "arm64" = @{
                "url" = $Assets["windows-arm64"].url
                "hash" = $Hashes["windows-arm64"]
                "extract_dir" = $Assets["windows-arm64"].extract_dir
            }
        }
        "bin" = "shimexe.exe"
        "checkver" = @{
            "github" = "https://github.com/loonghao/shimexe"
        }
        "autoupdate" = @{
            "architecture" = @{
                "64bit" = @{
                    "url" = "https://github.com/loonghao/shimexe/releases/download/v`$version/shimexe-`$version-x86_64-pc-windows-msvc.zip"
                    "extract_dir" = "shimexe-`$version-x86_64-pc-windows-msvc"
                }
                "arm64" = @{
                    "url" = "https://github.com/loonghao/shimexe/releases/download/v`$version/shimexe-`$version-aarch64-pc-windows-msvc.zip"
                    "extract_dir" = "shimexe-`$version-aarch64-pc-windows-msvc"
                }
            }
        }
        "notes" = @(
            "shimexe has been installed successfully!",
            "",
            "Quick start:",
            "  shimexe init --examples",
            "  shimexe add mytool --path https://example.com/tool.exe",
            "",
            "For more information, visit: https://github.com/loonghao/shimexe"
        )
    }

    $ScoopJson = $ScoopManifest | ConvertTo-Json -Depth 10

    if (-not $DryRun) {
        # Update Scoop bucket repository
        try {
            Update-GitHubFile -Repo $ScoopBucketRepo -Path "bucket/shimexe.json" -Content $ScoopJson -Message "chore: update shimexe to v$Version" -Token $GitHubToken
            Write-Host "Updated Scoop bucket repository successfully!" -ForegroundColor Green
        }
        catch {
            Write-Warning "Failed to update Scoop bucket: $_"
            Write-Host "Scoop manifest content:" -ForegroundColor Yellow
            Write-Host $ScoopJson
        }
    } else {
        Write-Host "Scoop manifest content:" -ForegroundColor Yellow
        Write-Host $ScoopJson
    }

    Write-Host "Scoop package updated successfully" -ForegroundColor Green
}

# Update Homebrew formula
if ($PublishHomebrew) {
    Write-Host "Updating Homebrew formula..." -ForegroundColor Yellow

    $requiredPlatforms = @("macos-x64", "macos-arm64", "linux-x64", "linux-arm64")
    foreach ($platform in $requiredPlatforms) {
        if (-not $Hashes[$platform]) {
            Write-Error "Hash for $platform required for Homebrew"
        }
    }

    # Create Homebrew formula
    $HomebrewFormula = @"
class Shimexe < Formula
  desc "The Modern Executable Shim Manager"
  homepage "https://github.com/loonghao/shimexe"
  version "$Version"
  license "MIT"

  if OS.mac?
    if Hardware::CPU.arm?
      url "$($Assets["macos-arm64"].url)"
      sha256 "$($Hashes["macos-arm64"])"
    else
      url "$($Assets["macos-x64"].url)"
      sha256 "$($Hashes["macos-x64"])"
    end
  elsif OS.linux?
    if Hardware::CPU.arm?
      url "$($Assets["linux-arm64"].url)"
      sha256 "$($Hashes["linux-arm64"])"
    else
      url "$($Assets["linux-x64"].url)"
      sha256 "$($Hashes["linux-x64"])"
    end
  end

  def install
    bin.install "shimexe"

    # Install shell completions if available
    if (buildpath/"completions").exist?
      bash_completion.install "completions/shimexe.bash" => "shimexe"
      zsh_completion.install "completions/_shimexe"
      fish_completion.install "completions/shimexe.fish"
    end

    # Install man page if available
    if (buildpath/"man").exist?
      man1.install "man/shimexe.1"
    end
  end

  test do
    system "#{bin}/shimexe", "--version"
    system "#{bin}/shimexe", "--help"

    # Test basic functionality
    system "#{bin}/shimexe", "init"
    assert_predicate testpath/".shimexe", :exist?
  end
end
"@

    if (-not $DryRun) {
        # Update Homebrew tap repository
        try {
            Update-GitHubFile -Repo $HomebrewTapRepo -Path "Formula/shimexe.rb" -Content $HomebrewFormula -Message "chore: update shimexe to v$Version" -Token $GitHubToken
            Write-Host "Updated Homebrew tap repository successfully!" -ForegroundColor Green
        }
        catch {
            Write-Warning "Failed to update Homebrew tap: $_"
            Write-Host "Homebrew formula content:" -ForegroundColor Yellow
            Write-Host $HomebrewFormula
        }
    } else {
        Write-Host "Homebrew formula content:" -ForegroundColor Yellow
        Write-Host $HomebrewFormula
    }

    Write-Host "Homebrew formula updated successfully" -ForegroundColor Green
}

# Update local package files
Write-Host "Updating local package files..." -ForegroundColor Yellow

# Update local Scoop manifest
if (Test-Path "pkg/scoop/shimexe.json") {
    $localScoopContent = Get-Content "pkg/scoop/shimexe.json" -Raw | ConvertFrom-Json
    $localScoopContent.version = $Version
    $localScoopContent.architecture."64bit".url = $Assets["windows-x64"].url
    $localScoopContent.architecture."64bit".hash = $Hashes["windows-x64"]
    $localScoopContent.architecture."64bit".extract_dir = $Assets["windows-x64"].extract_dir
    $localScoopContent.architecture.arm64.url = $Assets["windows-arm64"].url
    $localScoopContent.architecture.arm64.hash = $Hashes["windows-arm64"]
    $localScoopContent.architecture.arm64.extract_dir = $Assets["windows-arm64"].extract_dir

    $localScoopContent | ConvertTo-Json -Depth 10 | Set-Content "pkg/scoop/shimexe.json"
    Write-Host "Updated local Scoop manifest" -ForegroundColor Green
}

# Update local Homebrew formula
if (Test-Path "pkg/homebrew/shimexe.rb") {
    $homebrewContent = Get-Content "pkg/homebrew/shimexe.rb" -Raw
    $homebrewContent = $homebrewContent -replace 'version "[\d\.]+"', "version `"$Version`""
    $homebrewContent = $homebrewContent -replace 'PLACEHOLDER_ARM64_SHA256', $Hashes["macos-arm64"]
    $homebrewContent = $homebrewContent -replace 'PLACEHOLDER_X86_64_SHA256', $Hashes["macos-x64"]
    $homebrewContent = $homebrewContent -replace 'PLACEHOLDER_LINUX_ARM64_SHA256', $Hashes["linux-arm64"]
    $homebrewContent = $homebrewContent -replace 'PLACEHOLDER_LINUX_X86_64_SHA256', $Hashes["linux-x64"]

    Set-Content -Path "pkg/homebrew/shimexe.rb" -Value $homebrewContent
    Write-Host "Updated local Homebrew formula" -ForegroundColor Green
}

# Update local Chocolatey template
if (Test-Path "pkg/chocolatey/shimexe.nuspec.template") {
    $chocoTemplate = Get-Content "pkg/chocolatey/shimexe.nuspec.template" -Raw
    $chocoTemplate = $chocoTemplate -replace '{{VERSION}}', $Version

    Set-Content -Path "pkg/chocolatey/shimexe.nuspec" -Value $chocoTemplate
    Write-Host "Updated local Chocolatey nuspec" -ForegroundColor Green
}

# Summary
Write-Host "`nPackage publishing summary:" -ForegroundColor Cyan
Write-Host "=========================" -ForegroundColor Cyan

if ($PublishChocolatey) {
    Write-Host "✓ Chocolatey: Updated" -ForegroundColor Green
} else {
    Write-Host "- Chocolatey: Skipped" -ForegroundColor Gray
}

if ($PublishScoop) {
    Write-Host "✓ Scoop: Updated" -ForegroundColor Green
} else {
    Write-Host "- Scoop: Skipped" -ForegroundColor Gray
}

if ($PublishHomebrew) {
    Write-Host "✓ Homebrew: Updated" -ForegroundColor Green
} else {
    Write-Host "- Homebrew: Skipped" -ForegroundColor Gray
}

if ($DryRun) {
    Write-Host "`nDry run completed. No packages were actually published." -ForegroundColor Yellow
} else {
    Write-Host "`nPackage publishing completed!" -ForegroundColor Green
}

Write-Host "`nNext steps:" -ForegroundColor Cyan
Write-Host "1. Commit and push the updated local package files" -ForegroundColor White
Write-Host "2. Test installations:" -ForegroundColor White
if ($PublishChocolatey) {
    Write-Host "   - choco install shimexe --version $Version" -ForegroundColor Gray
}
if ($PublishScoop) {
    Write-Host "   - scoop bucket add loonghao https://github.com/$ScoopBucketRepo" -ForegroundColor Gray
    Write-Host "   - scoop install shimexe" -ForegroundColor Gray
}
if ($PublishHomebrew) {
    Write-Host "   - brew tap $HomebrewTapRepo" -ForegroundColor Gray
    Write-Host "   - brew install shimexe" -ForegroundColor Gray
}
Write-Host "3. Monitor package manager repositories for any issues" -ForegroundColor White
