use crate::commands::word::get_data_under_cursor;
use device_query::{DeviceQuery, DeviceState, Keycode};
use mouse_position::mouse_position::Mouse;
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, Runtime, WebviewWindow};
use tokio::time::{self, Duration};

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
        let device_state = DeviceState::new();
        let mut last_state = false;

        let mut interval = time::interval(Duration::from_millis(33));

        loop {
            // 等待计时器，防止占用 100% CPU
            interval.tick().await;

            let keys = device_state.get_keys();
            let is_pressed = keys.contains(&Keycode::LControl);

            if is_pressed != last_state {
                last_state = is_pressed;

                // 获取窗口句柄
                if let Some(window) = app_handle.get_webview_window("main") {
                    if is_pressed {
                        get_data_under_cursor(app_handle.state(), window);
                    } else {
                        let _ = window.hide();
                    }
                }
            }
        }
    });
}
