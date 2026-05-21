use crate::AppState;
use mouse_position::mouse_position::Mouse;
use std::sync::atomic::Ordering;
use tauri::{Manager, PhysicalPosition, PhysicalSize, Runtime, State, WebviewWindow};
use tauri_nspanel::{
    tauri_panel, CollectionBehavior, PanelLevel, StyleMask, TrackingAreaOptions, WebviewWindowExt,
};

#[allow(unused_imports)]
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

tauri_panel! {
    panel!(BasicPanel {
        config: {
            can_become_key_window: true,
            is_floating_panel: true
        }
        with: {
            // Enable mouse tracking for the panel's content view
            // This allows the panel to receive mouse events even when not key/active
            tracking_area: {
                options: TrackingAreaOptions::new()
                    .active_always()           // Track mouse even when app is not active
                    .mouse_entered_and_exited() // Get notified when mouse enters/exits
                    .mouse_moved(),             // Track mouse movement
                auto_resize: true               // Resize tracking area with window
            }
        }
    })
}

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

#[tauri::command]
pub fn apply_window_effects(window: tauri::Window) {
    #[cfg(target_os = "macos")]
    {
        apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

        if let Some(web_window) = window.get_webview_window("main") {
            // 打开调试工具
            // web_window.open_devtools();

            let panel = web_window.to_panel::<BasicPanel>().unwrap();

            // Set the window to float level
            panel.set_level(PanelLevel::Floating.value());

            // Ensures the panel cannot activate the app
            panel.set_style_mask(StyleMask::empty().nonactivating_panel().into());

            // Allows the panel to:
            // - display on the same space as the full screen window
            // - join all spaces
            panel.set_collection_behavior(
                CollectionBehavior::new()
                    .full_screen_auxiliary()
                    .can_join_all_spaces()
                    .into(),
            );
        }
    }

    #[cfg(target_os = "windows")]
    {
        apply_blur(&window, Some((18, 18, 18, 125)))
            .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
    }
}

// 更新鼠标是否在窗口内
#[tauri::command]
pub fn update_hover_status(hovered: bool, state: State<'_, AppState>) {
    state.window_locked.store(hovered, Ordering::Relaxed);
}
