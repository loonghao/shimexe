$ErrorActionPreference = 'Stop'

$packageName = 'shimexe'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"

# Remove shimexe from PATH if it was added
$shimexePath = Join-Path $toolsDir "shimexe.exe"
if (Test-Path $shimexePath) {
    Write-Host "Removing shimexe from $shimexePath"
    Remove-Item $shimexePath -Force -ErrorAction SilentlyContinue
}

Write-Host "shimexe has been uninstalled successfully"
