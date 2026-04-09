@echo off
setlocal
if exist "%~dp0SuDel.exe" (
  cd /d "%~dp0"
  powershell -ExecutionPolicy Bypass -File ".\scripts\install.ps1" %*
) else (
  cd /d "%~dp0\.."
  powershell -ExecutionPolicy Bypass -File ".\scripts\install.ps1" %*
)
endlocal
