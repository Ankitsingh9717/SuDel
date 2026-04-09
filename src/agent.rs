use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use tao::event::{Event, StartCause};
use tao::event_loop::{ControlFlow, EventLoopBuilder};

use crate::DynError;
use crate::config::Config;
use crate::logging::append_message;
use crate::picker::confirm_selected_delete;
use crate::selection::selected_paths_detailed;
use crate::shred::{ShredOptions, shred_targets};

pub fn run_agent(mut options: ShredOptions) -> Result<(), DynError> {
    append_message(&options.log_path, "AGENT starting");
    options.recursive = true;

    let manager = GlobalHotKeyManager::new()?;
    for hotkey in hotkeys_for_platform() {
        manager.register(hotkey)?;
    }
    append_message(&options.log_path, &registered_hotkey_message());

    let event_loop = EventLoopBuilder::<()>::with_user_event().build();
    let proxy = event_loop.create_proxy();
    let receiver = GlobalHotKeyEvent::receiver();
    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();

    thread::spawn(move || {
        let mut last_forwarded: Option<Instant> = None;
        loop {
            if shutdown_rx.try_recv().is_ok() {
                break;
            }
            if let Ok(_event) = receiver.recv() {
                let now = Instant::now();
                if last_forwarded
                    .map(|previous| now.duration_since(previous) < Duration::from_millis(700))
                    .unwrap_or(false)
                {
                    continue;
                }
                last_forwarded = Some(now);
                let _ = proxy.send_event(());
            }
        }
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {}
            Event::UserEvent(()) => {
                append_message(&options.log_path, "AGENT hotkey pressed");

                if let Ok(config) = Config::load() {
                    options.passes = config.agent_passes.unwrap_or(options.passes);
                }

                let targets = match selected_paths_detailed() {
                    Ok(paths) => {
                        if !paths.is_empty() {
                            append_message(&options.log_path, "AGENT using current OS selection");
                        }
                        paths
                    }
                    Err(error) => {
                        append_message(
                            &options.log_path,
                            &format!("AGENT selection lookup failed: {error}"),
                        );
                        Vec::new()
                    }
                };
                if targets.is_empty() {
                    append_message(&options.log_path, "AGENT no selected targets");
                    return;
                }

                if !confirm_selected_delete(&targets) {
                    append_message(&options.log_path, "AGENT delete canceled by user");
                    return;
                }

                if let Err(error) = shred_targets(&targets, &options) {
                    append_message(&options.log_path, &format!("AGENT shred failed: {error}"));
                }
            }
            Event::LoopDestroyed => {
                let _ = shutdown_tx.send(());
            }
            _ => {}
        }
    });
}

fn hotkeys_for_platform() -> Vec<HotKey> {
    let hotkeys = vec![HotKey::new(
        Some(Modifiers::SHIFT | Modifiers::ALT),
        Code::Delete,
    )];

    #[cfg(target_os = "macos")]
    {
        hotkeys.push(HotKey::new(
            Some(Modifiers::SHIFT | Modifiers::ALT),
            Code::Backspace,
        ));
    }

    hotkeys
}

fn registered_hotkey_message() -> String {
    #[cfg(target_os = "macos")]
    {
        return "AGENT hotkeys registered: Shift+Option+Delete and Shift+Option+Backspace"
            .to_string();
    }

    #[cfg(not(target_os = "macos"))]
    {
        "AGENT hotkey registered: Shift+Alt+Delete".to_string()
    }
}
