# Script to update package manager configurations with new version
param(
    [string]$Version
)

# Colors for output
$Red = "`e[31m"
$Green = "`e[32m"
$Yellow = "`e[33m"
$Blue = "`e[34m"
$Reset = "`e[0m"

# Get version from command line or Cargo.toml
if (-not $Version) {
    $CargoContent = Get-Content "Cargo.toml" -Raw
    if ($CargoContent -match 'version = "([^"]+)"') {
        $Version = $Matches[1]
    }
}

if (-not $Version) {
    Write-Host "${Red}Error: Could not determine version${Reset}"
    Write-Host "Usage: .\update-package-managers.ps1 [version]"
    exit 1
}

Write-Host "${Blue}Updating package managers for version ${Green}$Version${Reset}"

# Update Homebrew formula
Write-Host "${Yellow}Updating Homebrew formula...${Reset}"
$homebrewContent = Get-Content "pkg/homebrew/shimexe.rb" -Raw
$homebrewContent = $homebrewContent -replace 'version ".*"', "version `"$Version`""
Set-Content "pkg/homebrew/shimexe.rb" -Value $homebrewContent
Write-Host "${Green}✓ Updated pkg/homebrew/shimexe.rb${Reset}"

# Update Scoop manifest
Write-Host "${Yellow}Updating Scoop manifest...${Reset}"
$scoopContent = Get-Content "pkg/scoop/shimexe.json" -Raw
$scoopContent = $scoopContent -replace '"version": ".*"', "`"version`": `"$Version`""
$scoopContent = $scoopContent -replace '/v[0-9.]+/', "/v$Version/"
$scoopContent = $scoopContent -replace 'shimexe-[0-9.]+-', "shimexe-$Version-"
Set-Content "pkg/scoop/shimexe.json" -Value $scoopContent
Write-Host "${Green}✓ Updated pkg/scoop/shimexe.json${Reset}"

# Update Chocolatey templates
Write-Host "${Yellow}Updating Chocolatey templates...${Reset}"
$nuspecContent = Get-Content "pkg/chocolatey/shimexe.nuspec.template" -Raw
$nuspecContent = $nuspecContent -replace '{{VERSION}}', $Version
Set-Content "pkg/chocolatey/shimexe.nuspec.template" -Value $nuspecContent

$installContent = Get-Content "pkg/chocolatey/tools/chocolateyinstall.ps1.template" -Raw
$installContent = $installContent -replace '{{VERSION}}', $Version
Set-Content "pkg/chocolatey/tools/chocolateyinstall.ps1.template" -Value $installContent

$uninstallContent = Get-Content "pkg/chocolatey/tools/chocolateyuninstall.ps1.template" -Raw
$uninstallContent = $uninstallContent -replace '{{VERSION}}', $Version
Set-Content "pkg/chocolatey/tools/chocolateyuninstall.ps1.template" -Value $uninstallContent

Write-Host "${Green}✓ Updated pkg/chocolatey/ templates${Reset}"

Write-Host "${Green}✅ All package managers updated to version $Version${Reset}"
Write-Host "${Blue}Next steps:${Reset}"
Write-Host "  1. Commit the changes"
Write-Host "  2. Create and push a git tag: ${Yellow}git tag v$Version && git push origin v$Version${Reset}"
Write-Host "  3. The release workflow will automatically update the package repositories"
