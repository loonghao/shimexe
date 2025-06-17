# PowerShell script to publish shimexe packages to various package managers
# Usage: .\scripts\publish-packages.ps1 -Version "0.1.3" [-PublishChocolatey] [-PublishScoop]

param(
    [Parameter(Mandatory=$true)]
    [string]$Version,
    
    [switch]$PublishChocolatey,
    [switch]$PublishScoop,
    [switch]$DryRun
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
        return $hash.Hash
    }
    catch {
        Write-Warning "Failed to get hash for $Url: $_"
        return ""
    }
}

# Update Chocolatey package
if ($PublishChocolatey) {
    Write-Host "Updating Chocolatey package..." -ForegroundColor Yellow
    
    $chocoDir = "pkg/choco"
    $url64 = "$BaseUrl/releases/download/v$Version/shimexe-x86_64-pc-windows-msvc.zip"
    
    Write-Host "Getting checksum for $url64..."
    $checksum64 = Get-FileHash256 -Url $url64
    
    if ($checksum64) {
        Write-Host "Checksum: $checksum64" -ForegroundColor Green
        
        # Update nuspec version
        $nuspecPath = "$chocoDir/shimexe.nuspec"
        $nuspecContent = Get-Content $nuspecPath -Raw
        $nuspecContent = $nuspecContent -replace '<version>[\d\.]+</version>', "<version>$Version</version>"
        Set-Content -Path $nuspecPath -Value $nuspecContent
        
        # Update install script
        $installScriptPath = "$chocoDir/tools/chocolateyinstall.ps1"
        $installScript = Get-Content $installScriptPath -Raw
        $installScript = $installScript -replace "url64 = 'https://github\.com/[^']+/releases/download/v[\d\.]+/[^']+\.zip'", "url64 = '$url64'"
        $installScript = $installScript -replace "checksum64 = '[^']*'", "checksum64 = '$checksum64'"
        Set-Content -Path $installScriptPath -Value $installScript
        
        Write-Host "Updated Chocolatey package files" -ForegroundColor Green
        
        if (-not $DryRun) {
            # Pack and push to Chocolatey
            Push-Location $chocoDir
            try {
                choco pack shimexe.nuspec
                if ($env:CHOCOLATEY_API_KEY) {
                    choco push "shimexe.$Version.nupkg" --source https://push.chocolatey.org/ --api-key $env:CHOCOLATEY_API_KEY
                    Write-Host "Published to Chocolatey successfully!" -ForegroundColor Green
                } else {
                    Write-Warning "CHOCOLATEY_API_KEY not set, skipping push"
                }
            }
            finally {
                Pop-Location
            }
        }
    } else {
        Write-Error "Failed to get checksum for Chocolatey package"
    }
}

# Update Scoop package
if ($PublishScoop) {
    Write-Host "Updating Scoop package..." -ForegroundColor Yellow
    
    $scoopFile = "pkg/scoop/shimexe.json"
    $url = "$BaseUrl/releases/download/v$Version/shimexe-x86_64-pc-windows-msvc.zip"
    
    Write-Host "Getting hash for $url..."
    $hash = Get-FileHash256 -Url $url
    
    if ($hash) {
        Write-Host "Hash: $hash" -ForegroundColor Green
        
        # Update scoop manifest
        $scoopContent = Get-Content $scoopFile -Raw | ConvertFrom-Json
        $scoopContent.version = $Version
        $scoopContent.url = $url
        $scoopContent.hash = $hash
        $scoopContent.autoupdate.url = $scoopContent.autoupdate.url -replace 'v[\d\.]+', "v`$version"
        
        $scoopContent | ConvertTo-Json -Depth 10 | Set-Content $scoopFile
        
        Write-Host "Updated Scoop manifest" -ForegroundColor Green
        
        if (-not $DryRun) {
            Write-Host "Scoop package updated. Submit PR to scoop-extras or your bucket repository." -ForegroundColor Yellow
        }
    } else {
        Write-Error "Failed to get hash for Scoop package"
    }
}

# Update package versions in README
Write-Host "Updating README with new version..." -ForegroundColor Yellow
$readmeFiles = @("README.md", "README_zh.md")

foreach ($readmeFile in $readmeFiles) {
    if (Test-Path $readmeFile) {
        $content = Get-Content $readmeFile -Raw
        
        # Update Chocolatey install command
        $content = $content -replace 'choco install shimexe --version [\d\.]+', "choco install shimexe --version $Version"
        
        # Update download URLs
        $content = $content -replace 'releases/download/v[\d\.]+/', "releases/download/v$Version/"
        
        # Update install script URLs
        $content = $content -replace 'shimexe/[\d\.]+/install', "shimexe/$Version/install"
        
        Set-Content -Path $readmeFile -Value $content
        Write-Host "Updated $readmeFile" -ForegroundColor Green
    }
}

if ($DryRun) {
    Write-Host "Dry run completed. No packages were actually published." -ForegroundColor Yellow
} else {
    Write-Host "Package publishing completed!" -ForegroundColor Green
}

Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "1. Commit and push the updated package files"
Write-Host "2. For Scoop: Submit PR to scoop-extras or your bucket repository"
Write-Host "3. Test installation: choco install shimexe --version $Version"
