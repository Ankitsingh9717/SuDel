use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::DynError;

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub agent_passes: Option<usize>,
}

pub fn default_passes() -> usize {
    3
}

impl Config {
    pub fn load() -> Result<Self, DynError> {
        let path = config_path()?;
        match fs::read_to_string(path) {
            Ok(contents) => Ok(parse_config(&contents)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Self::default()),
            Err(error) => Err(error.into()),
        }
    }

    pub fn save_agent_passes(passes: usize) -> Result<(), DynError> {
        let path = config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, format!("passes={passes}\n"))?;
        Ok(())
    }
}

fn parse_config(contents: &str) -> Config {
    let mut config = Config::default();

    for line in contents.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if key.trim() == "passes" {
            config.agent_passes = value
                .trim()
                .parse::<usize>()
                .ok()
                .filter(|passes| *passes > 0);
        }
    }

    config
}

pub fn config_path() -> Result<PathBuf, DynError> {
    let mut base = data_root()?;
    base.push("config");
    base.push("sudel.conf");
    Ok(base)
}

fn data_root() -> Result<PathBuf, DynError> {
    #[cfg(target_os = "windows")]
    {
        let root = env::var_os("APPDATA").ok_or("APPDATA is not set")?;
        return Ok(PathBuf::from(root).join("SuDel"));
    }

    #[cfg(target_os = "macos")]
    {
        let root = env::var_os("HOME").ok_or("HOME is not set")?;
        return Ok(PathBuf::from(root)
            .join("Library")
            .join("Application Support")
            .join("SuDel"));
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let root = env::var_os("HOME").ok_or("HOME is not set")?;
        Ok(PathBuf::from(root).join(".config").join("SuDel"))
    }
}
