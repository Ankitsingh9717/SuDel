$ErrorActionPreference = "Stop"

$projectDir = Split-Path -Parent $PSScriptRoot
Push-Location $projectDir
try {
    Write-Host "Building SuDel release binary..."
    cargo build --release
    Write-Host "Installing SuDel..."
    & "$projectDir\target\release\SuDel.exe" --install @args
    Write-Host ""
    Write-Host "Install complete."
    Write-Host "On Windows, allow SuDel if Windows Security or SmartScreen prompts."
} finally {
    Pop-Location
}
