mod commands;
mod utils;
use commands::audio::{play_phonetic_url, AudioState};
use commands::translate::fetch_trans_res;
use commands::window::{apply_window_effects, update_hover_status};
use reqwest::Client;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tauri_plugin_global_shortcut::ShortcutState;
use utils::ocr_mac::OcrState;
use utils::shortcuts::{handle_shortcut_event, init_ctrl_listener, init_shortcuts};
use utils::tray;

pub struct AppState {
    ocr_state: Arc<OcrState>,
    client: Client,
    window_locked: Arc<AtomicBool>,
    audio_state: AudioState,
}

pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            ocr_state: OcrState::new(),
            client: Client::new(),
            window_locked: Arc::new(AtomicBool::new(false)),
            audio_state: AudioState::new(0.5),
        })
        // 注册命令
        .invoke_handler(tauri::generate_handler![
            apply_window_effects,
            update_hover_status,
            fetch_trans_res,
            play_phonetic_url
        ])
        .setup(|app| {
            tray::init(app)?;
            #[cfg(target_os = "macos")]
            {
                // app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }
            let handle = app.handle().clone();
            init_ctrl_listener(handle);
            init_shortcuts(app.handle());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_nspanel::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        handle_shortcut_event(app, shortcut);
                    }
                })
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
