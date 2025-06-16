# PowerShell script to convert SVG to ICO using ImageMagick
# Make sure ImageMagick is installed: winget install ImageMagick.ImageMagick

param(
    [string]$SvgPath = "assets/icon.svg",
    [string]$OutputDir = "assets"
)

Write-Host "Building icons from $SvgPath..." -ForegroundColor Green

# Check if ImageMagick is available
try {
    $magickVersion = & magick -version 2>$null
    if ($LASTEXITCODE -ne 0) {
        throw "ImageMagick not found"
    }
    Write-Host "Found ImageMagick" -ForegroundColor Green
} catch {
    Write-Host "ImageMagick not found. Installing via winget..." -ForegroundColor Yellow
    try {
        winget install ImageMagick.ImageMagick
        Write-Host "ImageMagick installed. Please restart your terminal and run this script again." -ForegroundColor Green
        exit 0
    } catch {
        Write-Host "Failed to install ImageMagick. Please install it manually from https://imagemagick.org/" -ForegroundColor Red
        exit 1
    }
}

# Create output directory if it doesn't exist
if (!(Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}

# Convert SVG to various PNG sizes
$sizes = @(16, 24, 32, 48, 64, 128, 256)
$pngFiles = @()

Write-Host "Converting SVG to PNG files..." -ForegroundColor Yellow

foreach ($size in $sizes) {
    $pngFile = "$OutputDir/icon_${size}x${size}.png"
    $pngFiles += $pngFile
    
    Write-Host "  Creating ${size}x${size} PNG..." -ForegroundColor Cyan
    & magick convert -background transparent -size "${size}x${size}" "$SvgPath" "$pngFile"
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to create $pngFile" -ForegroundColor Red
        exit 1
    }
}

# Create ICO file from PNG files
$icoFile = "$OutputDir/icon.ico"
Write-Host "Creating ICO file: $icoFile" -ForegroundColor Yellow

$pngArgs = $pngFiles -join " "
& magick convert $pngFiles "$icoFile"

if ($LASTEXITCODE -ne 0) {
    Write-Host "Failed to create ICO file" -ForegroundColor Red
    exit 1
}

# Create a high-quality PNG for other uses
Write-Host "Creating high-quality PNG..." -ForegroundColor Yellow
& magick convert -background transparent -size "512x512" "$SvgPath" "$OutputDir/icon_512x512.png"

# Clean up intermediate PNG files (optional)
Write-Host "Cleaning up intermediate files..." -ForegroundColor Yellow
foreach ($pngFile in $pngFiles) {
    Remove-Item $pngFile -Force
}

Write-Host "Icon generation completed successfully!" -ForegroundColor Green
Write-Host "Generated files:" -ForegroundColor Green
Write-Host "  - $icoFile" -ForegroundColor White
Write-Host "  - $OutputDir/icon_512x512.png" -ForegroundColor White
