use std::path::PathBuf;
use std::process::Command;

pub fn selected_paths() -> Vec<PathBuf> {
    selected_paths_detailed().unwrap_or_default()
}

pub fn selected_paths_detailed() -> Result<Vec<PathBuf>, String> {
    #[cfg(target_os = "macos")]
    {
        return macos_selected_paths();
    }

    #[cfg(target_os = "windows")]
    {
        return windows_selected_paths();
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Ok(Vec::new())
    }
}

#[cfg(target_os = "macos")]
fn macos_selected_paths() -> Result<Vec<PathBuf>, String> {
    let script = r#"
tell application "Finder"
    set selectedItems to selection
    if (count of selectedItems) is 0 then
        return ""
    end if
    set output to {}
    repeat with currentItem in selectedItems
        set end of output to POSIX path of (currentItem as alias)
    end repeat
    set AppleScript's text item delimiters to linefeed
    return output as text
end tell
"#;

    command_lines("osascript", &["-e", script])
}

#[cfg(target_os = "windows")]
fn windows_selected_paths() -> Result<Vec<PathBuf>, String> {
    let script = r#"
$shell = New-Object -ComObject Shell.Application
$results = @()
foreach ($window in $shell.Windows()) {
  try {
    $document = $window.Document
    if ($document -and $document.SelectedItems) {
      foreach ($item in $document.SelectedItems()) {
        $results += $item.Path
      }
    }
  } catch {}
}
$results | Select-Object -Unique
"#;

    command_lines(
        "powershell.exe",
        &[
            "-NoProfile",
            "-NonInteractive",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            script,
        ],
    )
    .or_else(|_| {
        command_lines(
            "pwsh",
            &[
                "-NoProfile",
                "-NonInteractive",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                script,
            ],
        )
    })
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn command_lines(program: &str, args: &[&str]) -> Result<Vec<PathBuf>, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|error| format!("{program} launch failed: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            format!("{program} exited with {}", output.status)
        } else {
            stderr
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(PathBuf::from)
        .collect())
}
