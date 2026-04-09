# SuDel Windows Manual

## Requirements

- Windows
- Rust installed from [rustup.rs](https://rustup.rs) if you are building from source

## Install

Open PowerShell in the `SuDel` project folder and run:

```powershell
.\scripts\install.ps1 --passes 5
```

What this does:

- builds the release binary
- installs `SuDel` into your user profile app-data location
- sets up auto-start on login
- starts the background agent on future logins

## Use

Select one or more files or folders in File Explorer, then press:

- `Shift + Alt + Delete`

`SuDel` will:

- use the current File Explorer selection
- show a confirmation dialog
- permanently delete the selected items after confirmation

If nothing is selected, it does nothing.

## Direct CLI usage

```powershell
.\target\release\SuDel.exe file1.txt folder2 --recursive --passes 5
.\target\release\SuDel.exe --diagnose
```

## Uninstall

```powershell
.\scripts\uninstall.ps1
```

This will:

- ask for confirmation
- disable auto-start
- remove the installed `SuDel` binary and install directory

## Troubleshooting

If the hotkey does not work:

- check if another app already uses `Shift + Alt + Delete`
- run `SuDel --diagnose`
- allow the app if Windows Security or SmartScreen prompts
- make sure File Explorer is the active app when testing selected-item deletion
