use super::capture::capture_around_cursor;
use super::ocr_mac::{recognize_words, select_word};
use crate::AppState;
use mouse_position::mouse_position::Mouse;
use std::error::Error;
use tauri::State;
use tauri::{App, Manager, PhysicalPosition, PhysicalSize, Runtime, WebviewWindow};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

fn show_window<R: Runtime>(window: WebviewWindow<R>) {
    // 1. 获取鼠标当前全局坐标
    if let Mouse::Position { x, y } = Mouse::get_mouse_position() {
        // 2. 获取当前显示器信息
        if let Ok(Some(monitor)) = window.current_monitor() {
            let monitor_pos = monitor.position(); // 显示器起始位置（多屏时很重要）

            let monitor_size = monitor.size(); // 显示器物理像素大小

            // 3. 获取窗口当前的物理大小
            // let window_size = window.outer_size().unwrap_or_default();
            let window_size: PhysicalSize<u32> = PhysicalSize {
                width: 320,
                height: 420,
            };

            let offset = 10; // 离鼠标的偏移距离
            let mut final_x = x + offset;
            let mut final_y = y + offset;

            // --- 碰撞检测逻辑 ---

            // 检测右边缘：鼠标位置 + 偏移 + 窗口宽度 > 显示器右边界
            let monitor_right_edge = monitor_pos.x + monitor_size.width as i32;
            if final_x + (window_size.width as i32) > monitor_right_edge {
                // 改为显示在右侧但有间隙
                final_x = (monitor_size.width - window_size.width) as i32 + offset;
                final_y = final_y + offset;
            }

            // 检测底边缘：鼠标位置 + 偏移 + 窗口高度 > 显示器底边界
            let monitor_bottom_edge = monitor_pos.y + monitor_size.height as i32;
            if final_y + (window_size.height as i32) > monitor_bottom_edge {
                // 改为显示在鼠标上方
                final_y = y - (window_size.height as i32) + offset;
            }

            // --- 兜底逻辑：防止窗口超出左边缘或顶边缘 ---
            // final_x = final_x.max(monitor_pos.x);
            // final_y = final_y.max(monitor_pos.y);

            // 4. 执行移动和显示
            let _ = window.set_position(PhysicalPosition {
                x: final_x,
                y: final_y,
            });

            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(10)); // 10ms延迟
                let _ = window.show();
                let _ = window.set_focus();
            });
        }
    }
}

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
pub fn setup_shortcuts<R: Runtime>(app: &mut App<R>) -> Result<(), Box<dyn Error>> {
    // 1. 获取 AppHandle
    let handle = app.handle().clone();

    // 2. 定义快捷键
    let ctrl_f1 = Shortcut::new(Some(Modifiers::CONTROL), Code::F1);

    // 3. 注册插件
    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app_handle, shortcut, event| {
                if shortcut == &ctrl_f1 && event.state() == ShortcutState::Pressed {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let is_visible = window.is_visible().unwrap_or(false);
                        if is_visible {
                            let _ = window.hide();
                        } else {
                            let test = get_word_under_cursor(app_handle.state::<AppState>());
                            if let Ok(result) = test {
                                println!("word: {:#?}", result);
                            } else {
                                println!("error")
                            }
                            // show_window(window);
                        }
                    }
                }
            })
            .build(),
    )?;

    // 4. 注册快捷键
    handle.global_shortcut().register(ctrl_f1)?;

    Ok(())
}
