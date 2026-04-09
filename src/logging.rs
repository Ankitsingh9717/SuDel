use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::DynError;

pub fn log_path() -> Result<PathBuf, DynError> {
    #[cfg(target_os = "windows")]
    {
        let root = std::env::var_os("APPDATA").ok_or("APPDATA is not set")?;
        return Ok(PathBuf::from(root).join("SuDel.log"));
    }

    #[cfg(target_os = "macos")]
    {
        let root = std::env::var_os("HOME").ok_or("HOME is not set")?;
        return Ok(PathBuf::from(root).join(".sudel.log"));
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let root = std::env::var_os("HOME").ok_or("HOME is not set")?;
        Ok(PathBuf::from(root).join(".sudel.log"))
    }
}

pub fn append_log(path: &Path, action: &str, target: &Path, result: &str) {
    append_entry(
        path,
        &format!("{action} {} {result}", target.to_string_lossy()),
    );
}

pub fn append_message(path: &Path, message: &str) {
    append_entry(path, message);
}

fn append_entry(path: &Path, message: &str) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_secs())
        .unwrap_or(0);

    let mut file = match OpenOptions::new().create(true).append(true).open(path) {
        Ok(file) => file,
        Err(_) => return,
    };

    let _ = writeln!(file, "[{timestamp}] {message}");
}
