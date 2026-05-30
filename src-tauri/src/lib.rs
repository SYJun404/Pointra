mod commands;
mod utils;
use commands::audio::{play_phonetic_url, AudioState};
use commands::config::{get_config, update_config};
use commands::translate::fetch_trans_res;
use commands::window::{apply_window_effects, stop_shortcuts, update_hover_status};
use reqwest::Client;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri_plugin_global_shortcut::ShortcutState;
use utils::define_config::{get_config_from_store, AppConfig};
use utils::ocr_mac::OcrState;
use utils::shortcuts::{handle_shortcut_event, init_point_listener, init_shortcuts};
use utils::tray;

pub struct AppState {
    ocr_state: Arc<OcrState>,
    client: Client,
    window_locked: Arc<AtomicBool>,
    audio_state: AudioState,
    config: Mutex<AppConfig>,
}

pub fn run() {
    tauri::Builder::default()
        // 注册命令
        .invoke_handler(tauri::generate_handler![
            update_hover_status,
            fetch_trans_res,
            play_phonetic_url,
            get_config,
            update_config,
            stop_shortcuts,
        ])
        // 初始化应用状态
        .setup(|app| {
            // 初始化AppState
            let config = get_config_from_store(app.handle()).unwrap_or_default();
            app.manage(AppState {
                ocr_state: OcrState::new(),
                client: Client::new(),
                window_locked: Arc::new(AtomicBool::new(false)),
                audio_state: AudioState::new(0.5),
                config: Mutex::new(config),
            });
            // 初始化托盘
            tray::init(app)?;

            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }
            apply_window_effects(app.get_webview_window("main").unwrap());

            init_point_listener(app.handle().clone());
            init_shortcuts(app.handle());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_nspanel::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_os::init())
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
