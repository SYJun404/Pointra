mod commands;
mod utils;
use commands::window::apply_window_effects;
use std::sync::Arc;
use utils::capture::ScreenCache;
use utils::ocr_mac::OcrState;
use utils::shortcuts;

pub struct AppState {
    screen_cache: ScreenCache,
    ocr_state: Arc<OcrState>,
}

pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            screen_cache: ScreenCache::new(),
            ocr_state: OcrState::new(),
        })
        // 注册命令
        .invoke_handler(tauri::generate_handler![apply_window_effects])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }
            // 注册快捷键
            shortcuts::setup_shortcuts(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_nspanel::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
