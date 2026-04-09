# SuDel macOS Manual

## Requirements

- macOS
- Rust installed from [rustup.rs](https://rustup.rs) if you are building from source

## Install

From the `SuDel` project folder:

```bash
./scripts/install.sh --passes 5
```

What this does:

- builds the release binary
- installs `SuDel` as `~/Applications/SuDel.app`
- sets up auto-start on login
- starts the background agent
- opens the Full Disk Access settings page

## Permissions

For reliable operation on protected folders like Downloads, enable both of these:

1. `System Settings > Privacy & Security > Full Disk Access`
2. `System Settings > Privacy & Security > Accessibility`

Add and enable:

```text
~/Applications/SuDel.app
```

If macOS does not accept the app bundle, add:

```text
~/Applications/SuDel.app/Contents/MacOS/SuDel
```

After changing permissions, log out and back in if needed.

## Use

Select one or more files or folders in Finder, then press:

- `Shift + Option + Delete`
- or on many MacBook keyboards: `Shift + Option + Backspace`

`SuDel` will:

- use only the current Finder selection
- show a confirmation dialog
- permanently delete the selected items after confirmation

If nothing is selected, it does nothing.

## Direct CLI usage

```bash
./target/release/SuDel file1.txt folder2 --recursive --passes 5
./target/release/SuDel --diagnose
```

## Uninstall

```bash
./scripts/uninstall.sh
```

This will:

- ask for confirmation
- disable auto-start
- remove the installed app bundle
- remove the LaunchAgent file

## Troubleshooting

If the hotkey does not work:

```bash
./target/release/SuDel --diagnose
tail -n 50 ~/.sudel.log
launchctl print gui/$(id -u)/SuDel
```
