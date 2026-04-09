$ErrorActionPreference = "Stop"

$projectDir = Split-Path -Parent $PSScriptRoot
$bundledBinary = Join-Path $projectDir "SuDel.exe"
$confirmation = Read-Host "This will uninstall SuDel and remove its installed files. Continue? [y/N]"
if ($confirmation -notin @("y", "Y", "yes", "YES")) {
    Write-Host "Uninstall canceled."
    exit 0
}

Push-Location $projectDir
try {
    if (Test-Path $bundledBinary) {
        & $bundledBinary --uninstall
        Write-Host "Uninstall complete."
        exit 0
    }

    $binary = "$projectDir\target\release\SuDel.exe"
    if (-not (Test-Path $binary)) {
        Write-Host "Building SuDel release binary..."
        cargo build --release
    }
    & $binary --uninstall
    Write-Host "Uninstall complete."
} finally {
    Pop-Location
}
