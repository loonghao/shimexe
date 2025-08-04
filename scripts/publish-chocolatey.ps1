# PowerShell script to publish shimexe to Chocolatey
# Usage: .\scripts\publish-chocolatey.ps1 -Version "0.1.3" -GitHubToken "token" -PublishChocolatey -ChocolateyApiKey "key"

param(
    [Parameter(Mandatory=$true)]
    [string]$Version,
    
    [Parameter(Mandatory=$true)]
    [string]$GitHubToken,
    
    [switch]$PublishChocolatey,
    [switch]$DryRun,
    
    [string]$ChocolateyApiKey = $env:CHOCOLATEY_API_KEY
)

$ErrorActionPreference = "Stop"

Write-Host "🍫 Publishing shimexe v$Version to Chocolatey..." -ForegroundColor Cyan

if (-not $PublishChocolatey) {
    Write-Host "PublishChocolatey flag not set, skipping Chocolatey publishing" -ForegroundColor Yellow
    exit 0
}

if (-not $ChocolateyApiKey) {
    Write-Host "❌ ChocolateyApiKey not provided" -ForegroundColor Red
    exit 1
}

# Navigate to chocolatey package directory
$chocoDir = "pkg/chocolatey"
if (-not (Test-Path $chocoDir)) {
    Write-Host "❌ Chocolatey package directory not found: $chocoDir" -ForegroundColor Red
    exit 1
}

Push-Location $chocoDir

try {
    # Update version in nuspec file
    $nuspecFile = "shimexe.nuspec"
    if (Test-Path $nuspecFile) {
        $content = Get-Content $nuspecFile -Raw
        $content = $content -replace '<version>.*</version>', "<version>$Version</version>"
        Set-Content $nuspecFile -Value $content -Encoding UTF8
        Write-Host "✅ Updated version in $nuspecFile" -ForegroundColor Green
    }

    # Pack the package
    Write-Host "📦 Packing Chocolatey package..." -ForegroundColor Yellow
    $packResult = choco pack $nuspecFile 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Failed to pack Chocolatey package" -ForegroundColor Red
        Write-Host $packResult -ForegroundColor Red
        exit 1
    }
    Write-Host "✅ Package packed successfully" -ForegroundColor Green

    # Find the generated package file
    $packageFile = "shimexe.$Version.nupkg"
    if (-not (Test-Path $packageFile)) {
        Write-Host "❌ Package file not found: $packageFile" -ForegroundColor Red
        exit 1
    }

    if ($DryRun) {
        Write-Host "🔍 DRY RUN - Would publish: $packageFile" -ForegroundColor Yellow
    } else {
        # Push to Chocolatey
        Write-Host "🚀 Publishing to Chocolatey..." -ForegroundColor Yellow
        $pushResult = choco push $packageFile --api-key $ChocolateyApiKey 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Host "❌ Failed to publish to Chocolatey" -ForegroundColor Red
            Write-Host $pushResult -ForegroundColor Red
            exit 1
        }
        Write-Host "✅ Successfully published to Chocolatey!" -ForegroundColor Green
    }

} finally {
    Pop-Location
}

Write-Host "🎉 Chocolatey publishing completed!" -ForegroundColor Green
