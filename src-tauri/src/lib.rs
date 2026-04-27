mod commands;
mod utils;
use commands::window::apply_window_effects;
use utils::shortcuts;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![apply_window_effects]) // 注册命令
        .setup(|app| {
            shortcuts::setup_shortcuts(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
