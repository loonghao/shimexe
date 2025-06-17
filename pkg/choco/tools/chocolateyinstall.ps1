$ErrorActionPreference = 'Stop'

$packageName = 'shimexe'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64 = 'https://github.com/loonghao/shimexe/releases/download/v0.1.3/shimexe-x86_64-pc-windows-msvc.zip'
$checksum64 = ''  # Will be updated by release process

$packageArgs = @{
  packageName   = $packageName
  unzipLocation = $toolsDir
  fileType      = 'zip'
  url64bit      = $url64
  checksum64    = $checksum64
  checksumType64= 'sha256'
  silentArgs    = ''
  validExitCodes= @(0)
  softwareName  = 'shimexe*'
}

Install-ChocolateyZipPackage @packageArgs

# Add shimexe to PATH
$shimexePath = Join-Path $toolsDir "shimexe.exe"
if (Test-Path $shimexePath) {
    Write-Host "shimexe installed successfully to $shimexePath"
} else {
    throw "shimexe.exe not found after installation"
}
