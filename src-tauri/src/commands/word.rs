use crate::utils::capture::capture_around_cursor;
use crate::utils::ocr_mac::{recognize_words, select_word};
use crate::utils::shortcuts::show_window;
use crate::AppState;
use tauri::{Emitter, State, WebviewWindow};

pub fn get_data_under_cursor(app_state: State<'_, AppState>, window: WebviewWindow) {
    // 1. 处理截图逻辑，捕获错误并通过事件通知前端
    let capture_res = capture_around_cursor(&app_state.screen_cache, 200, 40);

    let Ok((img, rel_x, rel_y)) = capture_res else {
        return;
    };

    #[cfg(target_os = "macos")]
    {
        // 2. OCR 识别
        let words_res = recognize_words(&img, &app_state.ocr_state);

        let Ok(words) = words_res else {
            return;
        };

        // 3. 坐标归一化与单词选择
        let nx = rel_x as f64 / img.width() as f64;
        let ny = 1.0 - rel_y as f64 / img.height() as f64;

        if let Some(word) = select_word(&words, nx, ny) {
            if !word.is_empty() {
                // 显示窗口
                show_window(&window);
                window.emit("from-cursor", word).ok();
            }
        } else {
            return;
        }
    }
}
