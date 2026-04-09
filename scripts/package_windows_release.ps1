$ErrorActionPreference = "Stop"

$projectDir = Split-Path -Parent $PSScriptRoot
$releaseDir = Join-Path $projectDir "release\windows"
$packageDir = Join-Path $releaseDir "SuDel-windows"
$binary = Join-Path $projectDir "target\release\SuDel.exe"

Push-Location $projectDir
try {
    Write-Host "Building Windows release binary..."
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        throw "Cargo build failed."
    }

    if (-not (Test-Path $binary)) {
        throw "SuDel.exe was not created."
    }

    Remove-Item $packageDir -Recurse -Force -ErrorAction SilentlyContinue
    New-Item -ItemType Directory -Path $packageDir | Out-Null
    New-Item -ItemType Directory -Path (Join-Path $packageDir "scripts") | Out-Null

    Copy-Item $binary (Join-Path $packageDir "SuDel.exe")
    Copy-Item (Join-Path $projectDir "scripts\install.bat") (Join-Path $packageDir "install.bat")
    Copy-Item (Join-Path $projectDir "scripts\uninstall.bat") (Join-Path $packageDir "uninstall.bat")
    Copy-Item (Join-Path $projectDir "scripts\install.ps1") (Join-Path $packageDir "scripts\install.ps1")
    Copy-Item (Join-Path $projectDir "scripts\uninstall.ps1") (Join-Path $packageDir "scripts\uninstall.ps1")
    Copy-Item (Join-Path $projectDir "WINDOWS_QUICKSTART.md") (Join-Path $packageDir "WINDOWS_QUICKSTART.md")
    Copy-Item (Join-Path $projectDir "LICENSE") (Join-Path $packageDir "LICENSE")
    Copy-Item (Join-Path $projectDir "COPYRIGHT.md") (Join-Path $packageDir "COPYRIGHT.md")

    $zipPath = Join-Path $releaseDir "SuDel-windows.zip"
    Remove-Item $zipPath -Force -ErrorAction SilentlyContinue
    Compress-Archive -Path "$packageDir\*" -DestinationPath $zipPath

    Write-Host ""
    Write-Host "Windows release package created:"
    Write-Host $zipPath
} finally {
    Pop-Location
}
