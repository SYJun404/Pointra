mod commands;
mod utils;
use commands::window::apply_window_effects;
use utils::shortcuts;

pub fn run() {
    tauri::Builder::default()
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
