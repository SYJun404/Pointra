mod commands;
mod utils;
use commands::audio::{play_phonetic_url, AudioState};
use commands::window::{apply_window_effects, update_hover_status};
use reqwest::Client;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use utils::capture::ScreenCache;
use utils::ocr_mac::OcrState;
use utils::shortcuts::init_ctrl_listener;
use utils::translate::fetch_trans_res;

pub struct AppState {
    screen_cache: ScreenCache,
    ocr_state: Arc<OcrState>,
    client: Client,
    window_locked: Arc<AtomicBool>,
    audio_state: AudioState,
}

pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            screen_cache: ScreenCache::new(),
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
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }
            let handle = app.handle().clone();
            init_ctrl_listener(handle);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_nspanel::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
