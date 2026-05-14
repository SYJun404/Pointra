use crate::AppState;
use std::sync::atomic::Ordering;
use tauri::{Manager, State};
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
