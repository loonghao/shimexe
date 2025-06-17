# shimexe installer script for Windows
# Usage: powershell -c "irm https://github.com/loonghao/shimexe/install.ps1 | iex"
# Usage with version: $env:SHIMEXE_VERSION="0.1.3"; powershell -c "irm https://github.com/loonghao/shimexe/install.ps1 | iex"

param(
    [string]$Version = $env:SHIMEXE_VERSION,
    [string]$InstallDir = $env:SHIMEXE_INSTALL_DIR
)

# Default values
if (-not $Version) { $Version = "latest" }
if (-not $InstallDir) { $InstallDir = "$env:USERPROFILE\.local\bin" }

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

$RepoOwner = "loonghao"
$RepoName = "shimexe"
$BaseUrl = "https://github.com/$RepoOwner/$RepoName/releases"

# Logging functions
function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Blue
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] $Message" -ForegroundColor Green
}

# Detect platform
function Get-Platform {
    $arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
    return "$arch-pc-windows-msvc"
}

# Get latest version from GitHub API
function Get-LatestVersion {
    try {
        $apiUrl = "https://api.github.com/repos/$RepoOwner/$RepoName/releases/latest"
        $response = Invoke-RestMethod -Uri $apiUrl -Method Get
        return $response.tag_name -replace '^v', ''
    }
    catch {
        Write-Error "Failed to get latest version: $_"
        exit 1
    }
}

# Download and install shimexe
function Install-Shimexe {
    $platform = Get-Platform
    
    if ($Version -eq "latest") {
        Write-Info "Fetching latest version..."
        $Version = Get-LatestVersion
        if (-not $Version) {
            Write-Error "Failed to get latest version"
            exit 1
        }
    }
    
    Write-Info "Installing shimexe v$Version for $platform..."
    
    # Construct download URL
    $archiveName = "shimexe-$platform.zip"
    $downloadUrl = "$BaseUrl/download/v$Version/$archiveName"
    
    # Create temporary directory
    $tempDir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }
    
    try {
        # Download
        Write-Info "Downloading from $downloadUrl..."
        $archivePath = Join-Path $tempDir $archiveName
        Invoke-WebRequest -Uri $downloadUrl -OutFile $archivePath -UseBasicParsing
        
        # Extract
        Write-Info "Extracting to $InstallDir..."
        if (-not (Test-Path $InstallDir)) {
            New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        }
        
        Expand-Archive -Path $archivePath -DestinationPath $tempDir -Force
        
        # Find and copy the binary
        $binaryPath = Get-ChildItem -Path $tempDir -Name "shimexe.exe" -Recurse | Select-Object -First 1
        
        if (-not $binaryPath) {
            Write-Error "shimexe.exe not found in archive"
            exit 1
        }
        
        $sourcePath = Join-Path $tempDir $binaryPath
        $destPath = Join-Path $InstallDir "shimexe.exe"
        Copy-Item -Path $sourcePath -Destination $destPath -Force
        
        Write-Success "shimexe v$Version installed to $destPath"
        
        # Check if install directory is in PATH
        $currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
        if ($currentPath -notlike "*$InstallDir*") {
            Write-Warn "Add $InstallDir to your PATH to use shimexe from anywhere:"
            Write-Host "  Run this command in an elevated PowerShell:"
            Write-Host "  [Environment]::SetEnvironmentVariable('PATH', `$env:PATH + ';$InstallDir', 'User')"
            Write-Host ""
            Write-Host "Or add it manually through System Properties > Environment Variables"
        }
        
        # Verify installation
        try {
            & $destPath --version | Out-Null
            Write-Success "Installation verified successfully!"
            Write-Host ""
            Write-Host "Get started with: shimexe --help"
        }
        catch {
            Write-Error "Installation verification failed: $_"
            exit 1
        }
    }
    finally {
        # Cleanup
        Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    }
}

# Main execution
function Main {
    Write-Info "shimexe installer"
    Write-Host ""
    
    # Check PowerShell version
    if ($PSVersionTable.PSVersion.Major -lt 5) {
        Write-Error "PowerShell 5.0 or later is required"
        exit 1
    }
    
    Install-Shimexe
}

# Run main function
Main
