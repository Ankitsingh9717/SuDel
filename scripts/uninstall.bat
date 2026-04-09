@echo off
setlocal
if exist "%~dp0SuDel.exe" (
  cd /d "%~dp0"
  powershell -ExecutionPolicy Bypass -File ".\scripts\uninstall.ps1" %*
) else (
  cd /d "%~dp0\.."
  powershell -ExecutionPolicy Bypass -File ".\scripts\uninstall.ps1" %*
)
endlocal
