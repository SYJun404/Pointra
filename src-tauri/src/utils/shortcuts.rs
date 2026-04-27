use std::error::Error;
use tauri::{App, Manager, Runtime};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

pub fn setup_shortcuts<R: Runtime>(app: &mut App<R>) -> Result<(), Box<dyn Error>> {
    // 1. 获取 AppHandle
    let handle = app.handle().clone();

    // 2. 定义快捷键
    let ctrl_alt_k = Shortcut::new(Some(Modifiers::CONTROL), Code::F1);

    // 3. 注册插件并设置处理器
    // 注意：在 Tauri 2.0 中，可以在 setup 之前或之中初始化插件
    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app_handle, shortcut, event| {
                if shortcut == &ctrl_alt_k && event.state() == ShortcutState::Pressed {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let is_visible = window.is_visible().unwrap_or(false);
                        if is_visible {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
            })
            .build(),
    )?;

    // 4. 注册快捷键
    handle.global_shortcut().register(ctrl_alt_k)?;

    Ok(())
}
