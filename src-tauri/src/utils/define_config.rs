use keytap::Key;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime};
use tauri_plugin_global_shortcut::{Code, Modifiers};
use tauri_plugin_store::StoreExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub theme: String,
    pub auto_hide: bool,
    pub auto_play: bool,

    pub point_key: Key,
    pub pinned_key: Key,
    pub hide_win_key: Key,

    pub select_text_modifiers: Modifiers,
    pub select_text_code: Code,

    pub search_win_modifiers: Modifiers,
    pub search_win_code: Code,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            auto_hide: true,
            auto_play: false,
            point_key: Key::F3,
            pinned_key: Key::F1,
            hide_win_key: Key::Tab,
            select_text_modifiers: Modifiers::SUPER,
            select_text_code: Code::Digit1,
            search_win_modifiers: Modifiers::SUPER,
            search_win_code: Code::Digit2,
        }
    }
}

// 定义一个常数作为 store 的文件名
const CONFIG_FILE: &str = "config.json";

/// 从 Store 中读取配置
pub fn get_config_from_store<R: Runtime>(app: &AppHandle<R>) -> Result<AppConfig, String> {
    let store = app
        .store(CONFIG_FILE)
        .map_err(|e| format!("Failed to open store: {}", e))?;

    if let Some(value) = store.get("config") {
        let config: AppConfig = serde_json::from_value(value.clone())
            .map_err(|e| format!("Failed to parse config JSON: {}", e))?;

        Ok(config)
    } else {
        Ok(AppConfig::default())
    }
}

/// 将配置保存回 Store
pub fn save_config_to_store<R: Runtime>(
    app: &AppHandle<R>,
    config: &AppConfig,
) -> Result<(), String> {
    let store = app.store(CONFIG_FILE).map_err(|e| e.to_string())?;

    let value = serde_json::to_value(config).map_err(|e| e.to_string())?;

    store.set("config".to_string(), value);

    store.save().map_err(|e| e.to_string())?;

    Ok(())
}
