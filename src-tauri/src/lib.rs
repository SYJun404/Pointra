mod commands;
mod utils;
use commands::window::apply_window_effects;
use commands::word::get_word_under_cursor;
use reqwest::Client;
use std::sync::Arc;
use utils::capture::ScreenCache;
use utils::ocr_mac::OcrState;
use utils::shortcuts::init_ctrl_listener;
use utils::translate::query_word;
use uuid::Uuid;

pub struct AppState {
    screen_cache: ScreenCache,
    ocr_state: Arc<OcrState>,
    client: Client,
    device_id: String,
}

pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            screen_cache: ScreenCache::new(),
            ocr_state: OcrState::new(),
            client: Client::new(),
            device_id: Uuid::new_v4().to_string(),
        })
        // 注册命令
        .invoke_handler(tauri::generate_handler![
            apply_window_effects,
            get_word_under_cursor,
            query_word
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
