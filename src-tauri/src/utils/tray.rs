use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, Runtime,
};

/// 初始化系统托盘
pub fn init<R: Runtime>(app: &tauri::App<R>) -> Result<(), tauri::Error> {
    let handle = app.handle();

    // 1. 创建托盘菜单项
    let search_item = MenuItem::with_id(handle, "search", "搜索界面", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(handle, "quit", "退出软件", true, None::<&str>)?;
    let author_item = MenuItem::with_id(handle, "author", "设置界面", true, None::<&str>)?;

    // 将菜单项组合成菜单
    let menu = Menu::with_items(handle, &[&author_item, &search_item, &quit_item])?;

    // 2. 构建托盘
    let _tray = TrayIconBuilder::with_id("main-tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        // 响应菜单事件
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "search" => {
                if let Some(window) = app.get_webview_window("main") {
                    window.emit("win-router", "search").ok();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            _ => {}
        })
        // 响应托盘图标左键点击事件
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}
