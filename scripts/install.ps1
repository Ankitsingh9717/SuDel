$ErrorActionPreference = "Stop"

function Test-MsvcLinker {
    $link = Get-Command link.exe -ErrorAction SilentlyContinue
    if ($link) {
        return $true
    }

    $vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    if (Test-Path $vswhere) {
        $installPath = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath 2>$null
        if ($installPath) {
            return $true
        }
    }

    return $false
}

$projectDir = Split-Path -Parent $PSScriptRoot
$bundledBinary = Join-Path $projectDir "SuDel.exe"
Push-Location $projectDir
try {
    if (Test-Path $bundledBinary) {
        Write-Host "Using bundled SuDel.exe..."
        & $bundledBinary --install @args
        Write-Host ""
        Write-Host "Install complete."
        Write-Host "On Windows, allow SuDel if Windows Security or SmartScreen prompts."
        exit 0
    }

    if (-not (Test-MsvcLinker)) {
        throw @"
Microsoft C++ Build Tools were not found.

SuDel needs Rust plus the MSVC C++ linker on Windows.

Install one of these:

1. Visual Studio Build Tools 2022
   Select: Desktop development with C++

2. Visual Studio Community 2022
   Select: Desktop development with C++

After installing:
1. Close PowerShell
2. Open a new PowerShell window
3. Run:
   rustup default stable-x86_64-pc-windows-msvc
   .\scripts\install.ps1 --passes 5
"@
    }

    Write-Host "Building SuDel release binary..."
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        throw "Cargo build failed."
    }

    $binary = "$projectDir\target\release\SuDel.exe"
    if (-not (Test-Path $binary)) {
        throw @"
SuDel.exe was not created.

On Windows, Rust needs the Microsoft C++ linker to build this project.
Install one of these, then run the installer again:

1. Visual Studio Build Tools 2019 or later with "Desktop development with C++"
2. Visual Studio Community with the C++ workload

After installing:
1. Reopen PowerShell
2. Run:
rustup default stable-x86_64-pc-windows-msvc
.\scripts\install.ps1 --passes 5
"@
    }

    Write-Host "Installing SuDel..."
    & $binary --install @args
    Write-Host ""
    Write-Host "Install complete."
    Write-Host "On Windows, allow SuDel if Windows Security or SmartScreen prompts."
} finally {
    Pop-Location
}
