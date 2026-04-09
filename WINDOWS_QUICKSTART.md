# SuDel Windows Quick Start

## Option 1. Easy install for non-technical users

1. Download the `SuDel-windows.zip` release from GitHub Releases.
2. Extract it.
3. Open the extracted folder.
4. Double-click:

```text
install.bat
```

If Windows asks for permission, allow it.

If the installer reports that Microsoft C++ Build Tools are missing, install:

- Visual Studio Build Tools 2022 with `Desktop development with C++`

Then run `install.bat` again.

If you downloaded the prebuilt release package, it should already include `SuDel.exe` and normally should not need Rust or Visual Studio.

## Option 2. Terminal install for technical users

Open PowerShell and run:

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo install --git https://github.com/Ankitsingh9717/SuDel
SuDel --install --passes 5
```

## Use

1. Select a file or folder in File Explorer.
2. Press:

```text
Shift + Alt + Delete
```

3. Confirm the dialog.

## Uninstall

Easy way:

```text
uninstall.bat
```

Terminal way:

```powershell
SuDel --uninstall
cargo uninstall sudel
```
