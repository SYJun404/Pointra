use crate::utils::define_config::{get_config_from_store, save_config_to_store, AppConfig};
use crate::utils::shortcuts::restart_shortcuts;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_config(app: AppHandle) -> Result<AppConfig, String> {
    get_config_from_store(&app)
}

#[tauri::command]
pub async fn update_config(app: AppHandle, new_config: AppConfig) -> Result<bool, String> {
    save_config_to_store(&app, &new_config)?;
    restart_shortcuts(&app, new_config);
    Ok(true)
}
