#!/usr/bin/env pwsh

# Performance benchmark runner for shimexe-core
# This script runs the benchmark suite and generates performance reports

param(
    [string]$Target = "shimexe-core",
    [switch]$Open = $false,
    [string]$Filter = "",
    [switch]$Baseline = $false,
    [string]$BaselineName = "main"
)

Write-Host "🚀 Running shimexe-core performance benchmarks..." -ForegroundColor Green

# Change to the crate directory
$CrateDir = Join-Path $PSScriptRoot ".." "crates" $Target
if (-not (Test-Path $CrateDir)) {
    Write-Error "Crate directory not found: $CrateDir"
    exit 1
}

Push-Location $CrateDir

try {
    # Build the benchmarks first
    Write-Host "📦 Building benchmarks..." -ForegroundColor Yellow
    cargo build --release --benches
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to build benchmarks"
        exit 1
    }

    # Prepare benchmark command
    $BenchCmd = @("cargo", "bench")
    
    if ($Filter) {
        $BenchCmd += "--", $Filter
    }
    
    if ($Baseline) {
        $BenchCmd += "--save-baseline", $BaselineName
    }

    # Run benchmarks
    Write-Host "⚡ Running benchmarks..." -ForegroundColor Yellow
    & $BenchCmd[0] $BenchCmd[1..($BenchCmd.Length-1)]
    
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Benchmarks failed"
        exit 1
    }

    # Open results if requested
    if ($Open) {
        $ReportPath = Join-Path $PWD "target" "criterion" "report" "index.html"
        if (Test-Path $ReportPath) {
            Write-Host "📊 Opening benchmark report..." -ForegroundColor Green
            Start-Process $ReportPath
        } else {
            Write-Warning "Benchmark report not found at: $ReportPath"
        }
    }

    Write-Host "✅ Benchmarks completed successfully!" -ForegroundColor Green
    Write-Host "📊 View detailed results at: target/criterion/report/index.html" -ForegroundColor Cyan

} finally {
    Pop-Location
}
