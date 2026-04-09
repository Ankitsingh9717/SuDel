use std::path::PathBuf;

pub fn pick_targets() -> Vec<PathBuf> {
    maybe_prepare_picker();

    let mut results = Vec::new();

    if let Some(paths) = rfd::FileDialog::new()
        .set_title("SuDel: choose files to permanently delete")
        .pick_files()
    {
        results.extend(paths);
    }
    if results.is_empty() {
        if let Some(folder) = rfd::FileDialog::new()
            .set_title("SuDel: choose folder to permanently delete")
            .pick_folder()
        {
            results.push(folder);
        }
    }

    results
}

pub fn confirm_selected_delete(targets: &[PathBuf]) -> bool {
    let summary = if targets.len() == 1 {
        format!(
            "Permanently delete this item?\n\n{}",
            targets[0].to_string_lossy()
        )
    } else {
        format!(
            "Permanently delete these {} selected items?\n\nThis cannot be undone.",
            targets.len()
        )
    };

    matches!(
        rfd::MessageDialog::new()
            .set_title("SuDel Confirmation")
            .set_description(&summary)
            .set_buttons(rfd::MessageButtons::OkCancel)
            .set_level(rfd::MessageLevel::Warning)
            .show(),
        rfd::MessageDialogResult::Ok
    )
}

#[cfg(target_os = "macos")]
fn maybe_prepare_picker() {
    let _ = std::process::Command::new("osascript")
        .args(["-e", r#"tell application "Finder" to activate"#])
        .output();
}

#[cfg(not(target_os = "macos"))]
fn maybe_prepare_picker() {}
