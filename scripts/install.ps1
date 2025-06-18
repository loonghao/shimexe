# shimexe installer script for Windows
# Usage: powershell -c "irm https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.ps1 | iex"
# Usage with version: $env:SHIMEXE_VERSION="0.3.0"; powershell -c "irm https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.ps1 | iex"

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

# Detect platform and map to release naming convention
function Get-Platform {
    $arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
    return "$arch-pc-windows-msvc"
}

# Map platform to release file naming convention
function Get-ReleaseFileName {
    param([string]$Platform)

    # Map from Rust target triple to release file naming
    switch ($Platform) {
        "x86_64-pc-windows-msvc" { return "shimexe-Windows-msvc-x86_64.zip" }
        "i686-pc-windows-msvc" { return "shimexe-Windows-msvc-i686.zip" }
        "aarch64-pc-windows-msvc" { return "shimexe-Windows-msvc-arm64.zip" }
        default { return "shimexe-$Platform.zip" }
    }
}

# Get latest shimexe version from GitHub API with retry and fallback
function Get-LatestVersion {
    $maxRetries = 3
    $retryDelay = 2

    for ($i = 1; $i -le $maxRetries; $i++) {
        try {
            Write-Info "Attempting to get latest shimexe version (attempt $i/$maxRetries)..."
            # Use releases endpoint to get all releases and filter for shimexe-v*
            $apiUrl = "https://api.github.com/repos/$RepoOwner/$RepoName/releases"

            # Add headers to avoid rate limiting issues
            $headers = @{
                'User-Agent' = 'shimexe-installer/1.0'
                'Accept'     = 'application/vnd.github.v3+json'
            }

            $response = Invoke-RestMethod -Uri $apiUrl -Method Get -Headers $headers -TimeoutSec 10

            # Find all shimexe releases (both v* and shimexe-v* formats) and sort by version
            $shimexeReleases = @()
            foreach ($release in $response) {
                # Match both v0.3.3 and shimexe-v0.1.2 formats
                if ($release.tag_name -match "^(shimexe-)?v([0-9]+\.[0-9]+\.[0-9]+)$") {
                    $shimexeReleases += @{
                        Version       = [System.Version]$matches[2]
                        TagName       = $release.tag_name
                        VersionString = $matches[2]
                    }
                }
            }

            # Sort by version and get the latest
            if ($shimexeReleases.Count -gt 0) {
                $latest = $shimexeReleases | Sort-Object { $_.Version } -Descending | Select-Object -First 1
                $version = $latest.VersionString
                Write-Info "Found latest shimexe version: v$version"
                return $version
            }

            Write-Warn "No shimexe releases found in API response"
        }
        catch {
            $errorMessage = $_.Exception.Message
            Write-Warn "Attempt $i failed: $errorMessage"

            # Check if it's a rate limit error
            if ($errorMessage -like "*rate limit*" -or $errorMessage -like "*403*") {
                Write-Warn "GitHub API rate limit detected"
                if ($i -lt $maxRetries) {
                    Write-Info "Waiting $retryDelay seconds before retry..."
                    Start-Sleep -Seconds $retryDelay
                    $retryDelay *= 2  # Exponential backoff
                }
            }
            elseif ($i -lt $maxRetries) {
                Write-Info "Retrying in $retryDelay seconds..."
                Start-Sleep -Seconds $retryDelay
            }
        }
    }

    # Fallback: try to get version from releases page HTML
    Write-Warn "API failed, trying fallback method..."
    try {
        $releasesUrl = "https://github.com/$RepoOwner/$RepoName/releases"
        $response = Invoke-WebRequest -Uri $releasesUrl -UseBasicParsing -TimeoutSec 10

        # Extract version from page content (look for both v* and shimexe-v* patterns)
        if ($response.Content -match 'releases/tag/(shimexe-)?v([0-9]+\.[0-9]+\.[0-9]+)') {
            $version = $matches[2]
            Write-Info "Found shimexe version via fallback: v$version"
            return $version
        }
    }
    catch {
        Write-Warn "Fallback method also failed: $_"
    }

    Write-Error "Failed to get latest version after all attempts. Please specify a version manually using: `$env:SHIMEXE_VERSION='x.y.z'"
    Write-Host "Example: `$env:SHIMEXE_VERSION='0.3.1'; irm https://raw.githubusercontent.com/loonghao/shimexe/main/scripts/install.ps1 | iex"
    exit 1
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

    # Construct download URL using correct release file naming
    $archiveName = Get-ReleaseFileName -Platform $platform
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
            Write-Host "Get started:" -ForegroundColor Yellow
            Write-Host "  shimexe --help" -ForegroundColor Cyan
            Write-Host "  shimexe init --examples" -ForegroundColor Cyan
            Write-Host ""
            Write-Host "Add your first shim:" -ForegroundColor Yellow
            Write-Host "  # Option 1: Auto-configure PATH (recommended)" -ForegroundColor Gray
            Write-Host "  shimexe add my-tool --path /path/to/tool --add-system-path" -ForegroundColor Cyan
            Write-Host "  my-tool --version  # Use directly" -ForegroundColor Green
            Write-Host ""
            Write-Host "  # Option 2: Use shimexe run (no PATH setup)" -ForegroundColor Gray
            Write-Host "  shimexe add my-tool --path /path/to/tool" -ForegroundColor Cyan
            Write-Host "  shimexe run my-tool --version  # Use via shimexe" -ForegroundColor Green
            Write-Host ""
            Write-Host "Default shim directory: $env:USERPROFILE\.shimexe" -ForegroundColor Gray
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
