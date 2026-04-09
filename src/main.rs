#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod agent;
mod config;
mod install;
mod logging;
mod picker;
mod selection;
mod shred;

use std::collections::BTreeSet;
use std::error::Error;
use std::path::PathBuf;

use clap::{ArgAction, Parser};

use crate::agent::run_agent;
use crate::config::{Config, default_passes};
use crate::install::{install_current_binary, uninstall_current_binary};
use crate::logging::append_message;
use crate::picker::pick_targets;
use crate::selection::{selected_paths, selected_paths_detailed};
use crate::shred::{ShredOptions, shred_targets};

type DynError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about = "SuDel: lightweight permanent file and folder deleter"
)]
struct Cli {
    #[arg(value_name = "PATH")]
    paths: Vec<PathBuf>,

    #[arg(long, action = ArgAction::SetTrue, help = "Run as a silent background hotkey agent")]
    agent: bool,

    #[arg(long, value_name = "N", help = "Overwrite pass count")]
    passes: Option<usize>,

    #[arg(long, action = ArgAction::SetTrue, help = "Allow directory shredding")]
    recursive: bool,

    #[arg(long, action = ArgAction::SetTrue, help = "Install binary globally and enable auto-start")]
    install: bool,

    #[arg(long, action = ArgAction::SetTrue, help = "Disable auto-start and remove installed binary")]
    uninstall: bool,

    #[arg(long, action = ArgAction::SetTrue, help = "Suppress normal CLI output")]
    quiet: bool,

    #[arg(long, action = ArgAction::SetTrue, help = "Print platform diagnostics")]
    diagnose: bool,
}

fn main() {
    if let Err(error) = run() {
        if !std::env::args().any(|arg| arg == "--agent" || arg == "--quiet") {
            eprintln!("error: {error}");
        }
        std::process::exit(1);
    }
}

fn run() -> Result<(), DynError> {
    let cli = Cli::parse();
    let config = Config::load()?;

    if cli.diagnose {
        return run_diagnostics();
    }

    if cli.install {
        install_current_binary(cli.passes.or(config.agent_passes), cli.quiet)?;
        return Ok(());
    }

    if cli.uninstall {
        uninstall_current_binary(cli.quiet)?;
        return Ok(());
    }

    let passes = cli
        .passes
        .or(config.agent_passes)
        .unwrap_or(default_passes());
    let options = ShredOptions {
        passes,
        recursive: cli.recursive,
        quiet: cli.quiet || cli.agent,
        log_path: logging::log_path()?,
    };

    if cli.agent {
        run_agent(options)?;
        return Ok(());
    }

    let mut targets = cli.paths;
    if targets.is_empty() {
        targets = selected_paths();
    }
    if targets.is_empty() {
        targets = pick_targets();
    }
    if targets.is_empty() {
        return Ok(());
    }

    targets = dedupe_targets(targets);
    reject_protected_targets(&targets, &options.log_path)?;
    shred_targets(&targets, &options)
}

fn dedupe_targets(targets: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();

    for path in targets {
        if seen.insert(path.clone()) {
            deduped.push(path);
        }
    }

    deduped
}

fn run_diagnostics() -> Result<(), DynError> {
    let log_path = logging::log_path()?;
    let config_path = config::config_path()?;

    println!("SuDel diagnostics");
    println!("platform: {}", std::env::consts::OS);
    println!("arch: {}", std::env::consts::ARCH);
    println!("executable: {}", std::env::current_exe()?.display());
    println!("log path: {}", log_path.display());
    println!("config path: {}", config_path.display());

    match selected_paths_detailed() {
        Ok(paths) => println!("selection lookup: ok ({} item(s))", paths.len()),
        Err(error) => println!("selection lookup: error ({error})"),
    }

    append_message(&log_path, "DIAG diagnostics command executed");

    #[cfg(target_os = "macos")]
    {
        println!("macOS notes:");
        println!("- If the hotkey does not work, try running `SuDel --agent` from Terminal first.");
        println!(
            "- Terminal may need permission under Privacy & Security > Automation and Accessibility."
        );
        println!("- Unsigned background binaries may need manual approval in Privacy & Security.");
    }

    #[cfg(target_os = "windows")]
    {
        println!("Windows notes:");
        println!(
            "- If Explorer selection is not detected, try launching SuDel once from PowerShell."
        );
        println!(
            "- Some systems only expose Explorer selection through powershell.exe, others through pwsh."
        );
        println!("- If the hotkey is already used by another app, SuDel may not receive it.");
    }

    Ok(())
}

fn reject_protected_targets(
    targets: &[PathBuf],
    log_path: &std::path::Path,
) -> Result<(), DynError> {
    let current_exe = std::env::current_exe().ok();
    let current_exe_dir = current_exe
        .as_ref()
        .and_then(|path| path.parent().map(|parent| parent.to_path_buf()));

    for target in targets {
        if let Some(exe) = &current_exe {
            if same_or_nested(target, exe) {
                append_message(
                    log_path,
                    &format!("SAFE blocked self-delete target: {}", target.display()),
                );
                return Err(
                    format!("refusing to delete SuDel executable: {}", target.display()).into(),
                );
            }
        }

        if let Some(exe_dir) = &current_exe_dir {
            if same_or_nested(target, exe_dir) {
                append_message(
                    log_path,
                    &format!(
                        "SAFE blocked protected app directory target: {}",
                        target.display()
                    ),
                );
                return Err(format!(
                    "refusing to delete SuDel app directory: {}",
                    target.display()
                )
                .into());
            }
        }

        if same_or_nested(target, log_path) {
            return Err(format!("refusing to delete SuDel log file: {}", target.display()).into());
        }
    }

    Ok(())
}

fn same_or_nested(target: &std::path::Path, protected: &std::path::Path) -> bool {
    if target == protected {
        return true;
    }
    target.starts_with(protected) || protected.starts_with(target)
}
