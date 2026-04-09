use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use rand::{RngCore, rngs::OsRng};
use walkdir::WalkDir;

use crate::DynError;
use crate::logging::append_log;

const CHUNK_SIZE: usize = 1024 * 1024;

#[derive(Debug, Clone)]
pub struct ShredOptions {
    pub passes: usize,
    pub recursive: bool,
    pub quiet: bool,
    pub log_path: PathBuf,
}

pub fn shred_targets(targets: &[PathBuf], options: &ShredOptions) -> Result<(), DynError> {
    for target in targets {
        if !options.quiet {
            eprintln!("deleting {}", target.display());
        }

        match shred_target(target, options) {
            Ok(()) => append_log(&options.log_path, "SHRED", target, "OK"),
            Err(error) => {
                append_log(
                    &options.log_path,
                    "SHRED",
                    target,
                    &format!("ERROR: {error}"),
                );
                return Err(error);
            }
        }
    }

    Ok(())
}

fn shred_target(target: &Path, options: &ShredOptions) -> Result<(), DynError> {
    let metadata = fs::symlink_metadata(target)?;
    let file_type = metadata.file_type();

    if file_type.is_symlink() {
        delete_link(target)
    } else if metadata.is_file() {
        shred_file(target, options)
    } else if metadata.is_dir() {
        shred_directory(target, options)
    } else {
        Err(format!("unsupported path type: {}", target.display()).into())
    }
}

fn shred_directory(path: &Path, options: &ShredOptions) -> Result<(), DynError> {
    if !options.recursive {
        return Err(format!("{} is a directory; re-run with --recursive", path.display()).into());
    }

    for entry in WalkDir::new(path).contents_first(true) {
        let entry = entry?;
        let current = entry.path();
        let file_type = entry.file_type();

        if file_type.is_file() {
            if !options.quiet {
                eprintln!("  file {}", current.display());
            }
            shred_file(current, options)?;
        } else if file_type.is_symlink() {
            delete_link(current)?;
        } else if file_type.is_dir() {
            fs::remove_dir(current)?;
            sync_parent_of(current);
        }
    }

    Ok(())
}

fn shred_file(path: &Path, options: &ShredOptions) -> Result<(), DynError> {
    clear_readonly_if_needed(path)?;

    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    let size = file.metadata()?.len();
    let mut buffer = vec![0u8; CHUNK_SIZE];

    for pass in 0..options.passes {
        if !options.quiet {
            eprintln!("  pass {}/{}", pass + 1, options.passes);
        }
        file.seek(SeekFrom::Start(0))?;
        overwrite_contents(&mut file, size, &mut buffer)?;
        file.flush()?;
        file.sync_all()?;
    }

    drop(file);
    rename_then_delete(path)
}

fn overwrite_contents(file: &mut File, size: u64, buffer: &mut [u8]) -> io::Result<()> {
    let mut remaining = size;
    while remaining > 0 {
        let chunk = remaining.min(buffer.len() as u64) as usize;
        OsRng.fill_bytes(&mut buffer[..chunk]);
        file.write_all(&buffer[..chunk])?;
        remaining -= chunk as u64;
    }
    Ok(())
}

fn rename_then_delete(path: &Path) -> Result<(), DynError> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("cannot delete path without parent: {}", path.display()))?;

    let renamed = unique_random_name(parent)?;
    fs::rename(path, &renamed)?;
    sync_parent_dir(parent);
    fs::remove_file(renamed)?;
    sync_parent_dir(parent);
    Ok(())
}

fn delete_link(path: &Path) -> Result<(), DynError> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.is_dir() {
        fs::remove_dir(path)?;
    } else {
        fs::remove_file(path)?;
    }
    sync_parent_of(path);
    Ok(())
}

fn unique_random_name(parent: &Path) -> Result<PathBuf, DynError> {
    let mut bytes = [0u8; 12];
    for _ in 0..16 {
        OsRng.fill_bytes(&mut bytes);
        let name = hex_name(&bytes);
        let candidate = parent.join(name);
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
    Err("unable to generate unique temporary filename".into())
}

fn hex_name(bytes: &[u8]) -> OsString {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = Vec::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize]);
        output.push(HEX[(byte & 0x0f) as usize]);
    }
    OsString::from(String::from_utf8(output).unwrap_or_else(|_| "deleted".to_string()))
}

fn clear_readonly_if_needed(path: &Path) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    let permissions = metadata.permissions();
    if permissions.readonly() {
        #[cfg(target_family = "unix")]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = metadata.permissions().mode();
            let mut new_permissions = permissions;
            new_permissions.set_mode(mode | 0o200);
            fs::set_permissions(path, new_permissions)?;
        }

        #[cfg(target_family = "windows")]
        {
            let mut new_permissions = permissions;
            new_permissions.set_readonly(false);
            fs::set_permissions(path, new_permissions)?;
        }
    }
    Ok(())
}

fn sync_parent_dir(parent: &Path) {
    #[cfg(target_family = "unix")]
    {
        if let Ok(dir) = File::open(parent) {
            let _ = dir.sync_all();
        }
    }

    #[cfg(not(target_family = "unix"))]
    {
        let _ = parent;
    }
}

fn sync_parent_of(path: &Path) {
    if let Some(parent) = path.parent() {
        sync_parent_dir(parent);
    }
}
