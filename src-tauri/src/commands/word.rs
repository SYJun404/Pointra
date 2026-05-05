use crate::utils::capture::capture_around_cursor;
use crate::utils::ocr_mac::{recognize_words, select_word};
use crate::AppState;
use tauri::State;

#[tauri::command]
pub fn get_word_under_cursor(app_state: State<'_, AppState>) -> Result<String, String> {
    let (img, rel_x, rel_y) =
        capture_around_cursor(&app_state.screen_cache, 200, 40).map_err(|e| e.to_string())?;

    #[cfg(target_os = "macos")]
    {
        let words = recognize_words(&img, &app_state.ocr_state).map_err(|e| e.to_string())?;

        // 归一化坐标；Vision Y 轴翻转
        let nx = rel_x as f64 / img.width() as f64;
        let ny = 1.0 - rel_y as f64 / img.height() as f64;

        let word = select_word(&words, nx, ny).unwrap_or_default();
        return Ok(word);
    }
}
