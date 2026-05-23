use crate::{
    utils::get_text::{get_data_from_selected_text, get_data_under_cursor},
    utils::show_window::show_input_window,
    AppState,
};
use keytap::{EventKind, Key, Tap};
use std::sync::atomic::Ordering;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

pub fn init_ctrl_listener(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut cached_window = None;

        let tap_init = Tap::new();
        if let Ok(tap) = tap_init {
            for event in tap.iter() {
                match event.kind {
                    EventKind::KeyDown(Key::F3) => {
                        let window = cached_window
                            .get_or_insert_with(|| app_handle.get_webview_window("main"));

                        if let Some(win) = window {
                            get_data_under_cursor(app_handle.state(), win.clone());
                        }
                    }
                    EventKind::KeyUp(Key::F3) => {
                        let app_state = app_handle.state::<AppState>();
                        let is_hovered = app_state.window_locked.load(Ordering::Relaxed);
                        if !is_hovered {
                            let window = cached_window
                                .get_or_insert_with(|| app_handle.get_webview_window("main"));
                            if let Some(win) = window {
                                let _ = win.hide();
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    });
}

pub fn init_shortcuts<R: Runtime>(app: &AppHandle<R>) {
    let shortcut_manager = app.global_shortcut();

    let shortcut_select_text = Shortcut::new(Some(Modifiers::SUPER), Code::Digit1);
    let shortcut_show_input = Shortcut::new(Some(Modifiers::SUPER), Code::Digit2);

    let _ = shortcut_manager.register(shortcut_select_text);
    let _ = shortcut_manager.register(shortcut_show_input);
}

pub fn handle_shortcut_event(app: &AppHandle, shortcut: &Shortcut) {
    if let Some(win) = app.get_webview_window("main") {
        if shortcut.matches(Modifiers::SUPER, Code::Digit1) {
            get_data_from_selected_text(win);
        } else if shortcut.matches(Modifiers::SUPER, Code::Digit2) {
            show_input_window(win);
        }
    }
}
