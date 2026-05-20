use crate::utils::capture::capture_around_cursor;
use crate::utils::ocr_mac::{recognize_words, select_word};
use crate::utils::shortcuts::show_window;
use crate::AppState;
use arboard::Clipboard;
use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use std::{
    thread,
    time::{Duration, Instant},
};
use tauri::{Emitter, State, WebviewWindow};

// 通过剪贴板获取选中文本
fn get_selected_text_via_clipboard(window: &WebviewWindow) -> Option<String> {
    let mut enigo = Enigo::new(&Settings::default()).ok()?;
    let mut clipboard = Clipboard::new().ok()?;

    // 【关键】保存当前剪贴板内容，用于后续恢复
    let old_content = clipboard.get_text().ok();

    // 【关键】清空剪贴板，以便判断 Cmd+C 是否触发了新内容
    let _ = clipboard.set_text("");

    // 模拟 Command + C
    #[cfg(target_os = "macos")]
    {
        let _ = window.run_on_main_thread(move || {
            let _ = enigo.key(Key::Meta, Press);
            let _ = enigo.key(Key::Unicode('c'), Press);
            let _ = enigo.key(Key::Unicode('c'), Release);
            let _ = enigo.key(Key::Meta, Release);
        });
    }

    // 轮询检查：由于系统写入剪贴板有延迟，循环检查比死等更高效
    let mut result = None;
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(150) {
        // 最多等 150ms
        if let Ok(text) = clipboard.get_text() {
            if !text.is_empty() {
                result = Some(text);
                break;
            }
        }
        thread::sleep(Duration::from_millis(20));
    }

    if let Some(old) = old_content {
        let _ = clipboard.set_text(old);
    }

    result
}

pub fn get_data_from_selected_text(window: WebviewWindow) {
    tauri::async_runtime::spawn(async move {
        let text = get_selected_text_via_clipboard(&window);
        if let Some(text) = text {
            show_window(&window);
            window.emit("from-cursor", text).ok();
        }
    });
}

pub fn get_data_under_cursor(app_state: State<'_, AppState>, window: WebviewWindow) {
    if window.is_visible().unwrap() {
        return;
    }

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
