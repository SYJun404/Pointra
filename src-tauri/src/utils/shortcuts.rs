use crate::{commands::word::get_data_under_cursor, AppState};
use keytap::{EventKind, Key, Tap};
use mouse_position::mouse_position::Mouse;
use std::sync::atomic::Ordering;
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, Runtime, WebviewWindow};

pub fn show_window<R: Runtime>(window: &WebviewWindow<R>) {
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
                // 让窗口贴住右边框，并留出 offset 间隙
                final_x = monitor_right_edge - (window_size.width as i32) - offset;
                // 轴向微调：让窗口稍微向下错位，避免遮挡光标中心
                final_y += offset;
            }

            let monitor_bottom_edge = monitor_pos.y + monitor_size.height as i32;
            if final_y + (window_size.height as i32) > monitor_bottom_edge {
                final_y = monitor_bottom_edge - (window_size.height as i32) - offset;
            }

            // --- 兜底逻辑：防止窗口超出左边缘或顶边缘 ---
            // final_x = final_x.max(monitor_pos.x);
            // final_y = final_y.max(monitor_pos.y);

            // 4. 执行移动和显示
            let _ = window.set_position(PhysicalPosition {
                x: final_x,
                y: final_y,
            });

            std::thread::sleep(std::time::Duration::from_millis(10));
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

pub fn init_ctrl_listener(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut cached_window = None;

        let tap_init = Tap::new();
        if let Ok(tap) = tap_init {
            for event in tap.iter() {
                match event.kind {
                    EventKind::KeyDown(Key::ShiftRight) => {
                        let window = cached_window
                            .get_or_insert_with(|| app_handle.get_webview_window("main"));

                        if let Some(win) = window {
                            get_data_under_cursor(app_handle.state(), win.clone());
                        }
                    }
                    EventKind::KeyUp(Key::ShiftRight) => {
                        let app_state = app_handle.state::<AppState>();
                        let is_hovered = app_state.window_locked.load(Ordering::Relaxed);
                        if !is_hovered {
                            let window = cached_window
                                .get_or_insert_with(|| app_handle.get_webview_window("main"));
                            if let Some(win) = window {
                                let _ = win.hide();
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    });
}
