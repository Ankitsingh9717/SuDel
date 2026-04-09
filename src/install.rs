use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use auto_launch::AutoLaunchBuilder;

use crate::DynError;
use crate::config::Config;

const APP_NAME: &str = "SuDel";

pub fn install_current_binary(
    configured_passes: Option<usize>,
    quiet: bool,
) -> Result<(), DynError> {
    let current = env::current_exe()?;
    let destination = installed_binary_path(&current)?;

    if current == destination {
        if let Some(passes) = configured_passes {
            Config::save_agent_passes(passes)?;
        }
        auto_launcher(&destination)?.enable()?;
        post_install_platform_setup(&destination, quiet)?;
        if !quiet {
            eprintln!("installed {}", destination.display());
        }
        return Ok(());
    }

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(&current, &destination)?;

    if let Some(passes) = configured_passes {
        Config::save_agent_passes(passes)?;
    }

    auto_launcher(&destination)?.enable()?;
    post_install_platform_setup(&destination, quiet)?;

    if !quiet {
        eprintln!("installed {}", destination.display());
    }
    Ok(())
}

pub fn uninstall_current_binary(quiet: bool) -> Result<(), DynError> {
    let current = env::current_exe()?;
    let destination = installed_binary_path(&current)?;

    let launcher = auto_launcher(&destination)?;
    let _ = launcher.disable();
    pre_uninstall_platform_cleanup()?;

    remove_installed_artifacts(&current, &destination)?;

    if !quiet {
        eprintln!("uninstalled {}", destination.display());
    }
    Ok(())
}

fn auto_launcher(binary: &Path) -> Result<auto_launch::AutoLaunch, DynError> {
    Ok(AutoLaunchBuilder::new()
        .set_app_name(APP_NAME)
        .set_app_path(binary.to_string_lossy().as_ref())
        .set_use_launch_agent(true)
        .set_args(&["--agent", "--quiet"])
        .build()?)
}

fn installed_binary_path(current: &Path) -> Result<PathBuf, DynError> {
    let file_name = current
        .file_name()
        .ok_or_else(|| format!("invalid executable path: {}", current.display()))?;

    #[cfg(target_os = "windows")]
    {
        let root = env::var_os("APPDATA").ok_or("APPDATA is not set")?;
        return Ok(PathBuf::from(root).join(APP_NAME).join(file_name));
    }

    #[cfg(target_os = "macos")]
    {
        return Ok(macos_bundle_executable_path(file_name));
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let root = env::var_os("HOME").ok_or("HOME is not set")?;
        Ok(PathBuf::from(root)
            .join(".local")
            .join("bin")
            .join(file_name))
    }
}

fn post_install_platform_setup(destination: &Path, quiet: bool) -> Result<(), DynError> {
    #[cfg(target_os = "macos")]
    {
        ensure_macos_app_bundle(destination)?;
        ensure_launch_agent_keepalive()?;
        kickstart_launch_agent()?;
        open_full_disk_access_guidance(destination, quiet);
    }

    Ok(())
}

fn remove_installed_artifacts(_current: &Path, destination: &Path) -> Result<(), DynError> {
    #[cfg(target_os = "macos")]
    {
        let bundle = macos_bundle_root_from_executable(destination);
        if bundle.exists() {
            fs::remove_dir_all(bundle)?;
        }
        return Ok(());
    }

    #[cfg(target_os = "windows")]
    {
        if destination.exists() && current != destination {
            fs::remove_file(destination)?;
        }
        if let Some(parent) = destination.parent() {
            let _ = fs::remove_dir(parent);
        }
        return Ok(());
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        if destination.exists() && current != destination {
            fs::remove_file(destination)?;
        }
        Ok(())
    }
}

fn pre_uninstall_platform_cleanup() -> Result<(), DynError> {
    #[cfg(target_os = "macos")]
    {
        bootout_launch_agent()?;
        remove_launch_agent_plist()?;
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn ensure_launch_agent_keepalive() -> Result<(), DynError> {
    let home = env::var_os("HOME").ok_or("HOME is not set")?;
    let plist_path = PathBuf::from(home)
        .join("Library")
        .join("LaunchAgents")
        .join("SuDel.plist");

    let contents = fs::read_to_string(&plist_path)?;
    if contents.contains("<key>KeepAlive</key>") {
        return Ok(());
    }

    let injected = contents.replace(
        "  <key>RunAtLoad</key>\n  <true/>\n",
        "  <key>RunAtLoad</key>\n  <true/>\n  <key>KeepAlive</key>\n  <true/>\n",
    );
    fs::write(plist_path, injected)?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn kickstart_launch_agent() -> Result<(), DynError> {
    let uid = launchctl_uid()?;

    let _ = Command::new("launchctl")
        .args(["bootout", &format!("gui/{uid}/SuDel")])
        .output();

    let _ = Command::new("launchctl")
        .args([
            "bootstrap",
            &format!("gui/{uid}"),
            &launch_agent_plist_path()?,
        ])
        .output();

    let _ = Command::new("launchctl")
        .args(["kickstart", "-k", &format!("gui/{uid}/SuDel")])
        .output();

    Ok(())
}

#[cfg(target_os = "macos")]
fn bootout_launch_agent() -> Result<(), DynError> {
    let uid = launchctl_uid()?;
    let _ = Command::new("launchctl")
        .args(["bootout", &format!("gui/{uid}/SuDel")])
        .output();
    Ok(())
}

#[cfg(target_os = "macos")]
fn launchctl_uid() -> Result<String, DynError> {
    let uid = Command::new("id").arg("-u").output()?;
    Ok(String::from_utf8_lossy(&uid.stdout).trim().to_string())
}

#[cfg(target_os = "macos")]
fn launch_agent_plist_path() -> Result<String, DynError> {
    let home = env::var_os("HOME").ok_or("HOME is not set")?;
    Ok(PathBuf::from(home)
        .join("Library")
        .join("LaunchAgents")
        .join("SuDel.plist")
        .to_string_lossy()
        .to_string())
}

#[cfg(target_os = "macos")]
fn macos_bundle_executable_path(file_name: &std::ffi::OsStr) -> PathBuf {
    let root = env::var_os("HOME").expect("HOME is not set");
    PathBuf::from(root)
        .join("Applications")
        .join("SuDel.app")
        .join("Contents")
        .join("MacOS")
        .join(file_name)
}

#[cfg(target_os = "macos")]
fn macos_bundle_root_from_executable(executable: &Path) -> PathBuf {
    executable
        .parent()
        .and_then(|path| path.parent())
        .and_then(|path| path.parent())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| executable.to_path_buf())
}

#[cfg(target_os = "macos")]
fn ensure_macos_app_bundle(destination: &Path) -> Result<(), DynError> {
    let contents_dir = destination
        .parent()
        .and_then(|macos| macos.parent())
        .ok_or("invalid macOS app bundle path")?;
    let resources_dir = contents_dir.join("Resources");
    fs::create_dir_all(&resources_dir)?;

    let plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleExecutable</key>
  <string>SuDel</string>
  <key>CFBundleIdentifier</key>
  <string>com.sudel.agent</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>SuDel</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>0.1.0</string>
  <key>CFBundleVersion</key>
  <string>0.1.0</string>
  <key>LSBackgroundOnly</key>
  <true/>
</dict>
</plist>
"#
    );
    fs::write(contents_dir.join("Info.plist"), plist)?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn remove_launch_agent_plist() -> Result<(), DynError> {
    let plist_path = PathBuf::from(launch_agent_plist_path()?);
    if plist_path.exists() {
        fs::remove_file(plist_path)?;
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn open_full_disk_access_guidance(destination: &Path, quiet: bool) {
    let _ = Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles")
        .output();

    let _ = Command::new("open")
        .args(["-R", &destination.to_string_lossy()])
        .output();

    if !quiet {
        eprintln!("macOS requires manual approval for protected folders like Downloads.");
        eprintln!("Add this binary to Full Disk Access and leave it enabled:");
        eprintln!("{}", destination.display());
    }
}
