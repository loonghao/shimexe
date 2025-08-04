#!/usr/bin/env pwsh
# Update package manager configuration files with new version
# This script is called by release-please to update version numbers in package manager files

param(
    [Parameter(Mandatory = $true)]
    [string]$Version,
    
    [Parameter(Mandatory = $false)]
    [string]$ProjectRoot = "."
)

# Ensure we're in the project root
Set-Location $ProjectRoot

Write-Host "Updating package manager configurations to version $Version" -ForegroundColor Green

# Function to update JSON files
function Update-JsonFile {
    param(
        [string]$FilePath,
        [string]$Version
    )
    
    if (Test-Path $FilePath) {
        Write-Host "Updating $FilePath" -ForegroundColor Yellow
        $content = Get-Content $FilePath -Raw | ConvertFrom-Json
        $content.version = $Version
        
        # Update URLs and extract directories for Scoop
        if ($FilePath -like "*scoop*") {
            if ($content.architecture."64bit") {
                $content.architecture."64bit".url = $content.architecture."64bit".url -replace "v[0-9]+\.[0-9]+\.[0-9]+", "v$Version"
                $content.architecture."64bit".url = $content.architecture."64bit".url -replace "shimexe-[0-9]+\.[0-9]+\.[0-9]+", "shimexe-$Version"
                $content.architecture."64bit".extract_dir = $content.architecture."64bit".extract_dir -replace "shimexe-[0-9]+\.[0-9]+\.[0-9]+", "shimexe-$Version"
            }
            if ($content.architecture.arm64) {
                $content.architecture.arm64.url = $content.architecture.arm64.url -replace "v[0-9]+\.[0-9]+\.[0-9]+", "v$Version"
                $content.architecture.arm64.url = $content.architecture.arm64.url -replace "shimexe-[0-9]+\.[0-9]+\.[0-9]+", "shimexe-$Version"
                $content.architecture.arm64.extract_dir = $content.architecture.arm64.extract_dir -replace "shimexe-[0-9]+\.[0-9]+\.[0-9]+", "shimexe-$Version"
            }
        }
        
        $content | ConvertTo-Json -Depth 10 | Set-Content $FilePath -Encoding UTF8
        Write-Host "✓ Updated $FilePath" -ForegroundColor Green
    } else {
        Write-Warning "File not found: $FilePath"
    }
}

# Function to update Ruby files (Homebrew)
function Update-RubyFile {
    param(
        [string]$FilePath,
        [string]$Version
    )
    
    if (Test-Path $FilePath) {
        Write-Host "Updating $FilePath" -ForegroundColor Yellow
        $content = Get-Content $FilePath -Raw
        
        # Update version line
        $content = $content -replace 'version\s+"[^"]*"', "version `"$Version`""
        
        # Update URLs
        $content = $content -replace "shimexe-[0-9]+\.[0-9]+\.[0-9]+", "shimexe-$Version"
        $content = $content -replace "v[0-9]+\.[0-9]+\.[0-9]+", "v$Version"
        
        Set-Content $FilePath -Value $content -Encoding UTF8
        Write-Host "✓ Updated $FilePath" -ForegroundColor Green
    } else {
        Write-Warning "File not found: $FilePath"
    }
}

# Function to update XML template files (Chocolatey)
function Update-XmlTemplate {
    param(
        [string]$FilePath,
        [string]$Version
    )
    
    if (Test-Path $FilePath) {
        Write-Host "Updating $FilePath" -ForegroundColor Yellow
        $content = Get-Content $FilePath -Raw
        
        # Update version placeholders
        $content = $content -replace "{{VERSION}}", $Version
        
        Set-Content $FilePath -Value $content -Encoding UTF8
        Write-Host "✓ Updated $FilePath" -ForegroundColor Green
    } else {
        Write-Warning "File not found: $FilePath"
    }
}

# Update package manager files
try {
    # Update Scoop configuration
    Update-JsonFile -FilePath "pkg/scoop/shimexe.json" -Version $Version
    
    # Update Homebrew formula
    Update-RubyFile -FilePath "pkg/homebrew/shimexe.rb" -Version $Version
    
    # Update Chocolatey template
    Update-XmlTemplate -FilePath "pkg/chocolatey/shimexe.nuspec.template" -Version $Version
    
    Write-Host "All package manager configurations updated successfully!" -ForegroundColor Green
    
} catch {
    Write-Error "Failed to update package manager configurations: $_"
    exit 1
}
