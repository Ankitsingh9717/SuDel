# SuDel

`SuDel` is a lightweight open-source CLI tool for permanently deleting selected files and folders on macOS and Windows.

It supports:

- permanent deletion instead of Trash or Recycle Bin
- overwrite passes with random data
- recursive folder deletion
- selected-item deletion from Finder and File Explorer
- global hotkey background agent
- confirmation before delete

## Developer and License

- Original creator: `sinc`
- Real name: `Ankit Singh`
- License: `MIT`
- Status: `Free to use and open source`

See:

- [LICENSE](LICENSE)
- [COPYRIGHT.md](COPYRIGHT.md)

## Support the Developer

If you would like to support the project, you can donate from India using UPI.

- Payment method: `UPI`
- UPI ID: `sinc@sbi`

## Important Security Note

`SuDel` is best-effort secure deletion, not a universal mathematical guarantee.

On modern SSDs, NVMe drives, copy-on-write filesystems, snapshots, journaling filesystems, RAID controllers, cloud-synced folders, and drives with firmware remapping, no software-only overwrite tool can honestly promise zero recovery chance in every case.

For the strongest protection:

- use full-disk encryption before sensitive data is created
- use device-level secure erase or crypto-erase when retiring an SSD
- account for backups, snapshots, and sync copies

## Step-by-Step Install

### 1. Install Rust

macOS and Linux:

```bash
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
```

Windows PowerShell:

```powershell
winget install Rustlang.Rustup
rustup default stable
```

Check that Rust and Cargo are available:

```bash
rustc --version
cargo --version
```

### 2. Install SuDel with Cargo

From any terminal:

```bash
cargo install --git https://github.com/Ankitsingh9717/SuDel sudel --bin SuDel
```

If Cargo's bin directory is not already in your `PATH`, add it:

macOS and Linux:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Windows PowerShell:

```powershell
$env:Path += ";$HOME\.cargo\bin"
```

Check that `SuDel` is installed:

```bash
SuDel --version
```

### 3. Enable the Background Agent

After installing with Cargo, run:

```bash
SuDel --install --passes 5
```

This will:

- install the background agent setup
- enable auto-start on login
- prepare the app for hotkey-based deletion

## macOS Setup

The installer places a bundle at:

```text
~/Applications/SuDel.app
```

For protected folders like Downloads, enable both of these:

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

## Windows Setup

If Windows Security or SmartScreen prompts you, allow `SuDel`.

No extra permission panel is usually needed beyond normal approval and startup access.

## Hotkeys

Windows:

- `Shift + Alt + Delete`

macOS:

- `Shift + Option + Delete`
- on many MacBook keyboards: `Shift + Option + Backspace`

## How to Use

### Hotkey mode

1. Select one or more files or folders in Finder or File Explorer.
2. Press the hotkey.
3. Confirm the delete dialog.
4. `SuDel` permanently deletes the selected items.

If nothing is selected, `SuDel` does nothing.

### Direct CLI mode

```bash
SuDel file1.txt folder2 --recursive --passes 5
SuDel --diagnose
```

## Config

The background agent reads its overwrite pass count from:

- macOS: `~/Library/Application Support/SuDel/config/sudel.conf`
- Windows: `%APPDATA%\SuDel\config\sudel.conf`

Format:

```text
passes=5
```

## Logs

- macOS: `~/.sudel.log`
- Windows: `%APPDATA%\SuDel.log`

The background agent writes startup, hotkey, selection, confirmation, and failure diagnostics there.

## Uninstall

Disable the background setup:

```bash
SuDel --uninstall
```

Remove the Cargo-installed binary:

```bash
cargo uninstall sudel
```

## Manuals

- [MANUAL_MACOS.md](MANUAL_MACOS.md)
- [MANUAL_WINDOWS.md](MANUAL_WINDOWS.md)
