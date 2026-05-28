use crate::{
    utils::define_config::get_config_from_store,
    utils::get_text::{get_data_from_selected_text, get_data_under_cursor},
    utils::show_window::show_input_window,
    AppState,
};
use keytap::{EventKind, Key, Tap};
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

pub struct Point {
    pub key: Mutex<Key>,
}

impl Point {
    pub fn new() -> Self {
        Self {
            key: Mutex::new(Key::F3),
        }
    }

    /// 设置 point key
    pub fn store(&self, key: Key) {
        *self.key.lock().unwrap() = key;
    }

    /// 获取当前 point key
    pub fn load(&self) -> Key {
        *self.key.lock().unwrap()
    }
}

/// 初始化Point快捷键
pub fn init_point_listener(app_handle: AppHandle) {
    let config = get_config_from_store(&app_handle).unwrap_or_default();

    // 从配置中读取 point_key 并更新 Point
    let point = app_handle.state::<AppState>().point_key.clone();
    point.store(config.point_key);

    tauri::async_runtime::spawn(async move {
        let mut cached_window = None;

        let tap_init = Tap::new();
        if let Ok(tap) = tap_init {
            for event in tap.iter() {
                let current_key = point.load();

                match event.kind {
                    EventKind::KeyDown(k) if k == current_key => {
                        let window = cached_window
                            .get_or_insert_with(|| app_handle.get_webview_window("main"));

                        if let Some(win) = window {
                            get_data_under_cursor(app_handle.state(), win.clone());
                        }
                    }
                    EventKind::KeyUp(k) if k == current_key => {
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

/// 初始化快捷键
pub fn init_shortcuts<R: Runtime>(app: &AppHandle<R>) {
    let shortcut_manager = app.global_shortcut();

    let shortcut_select_text = Shortcut::new(Some(Modifiers::SUPER), Code::Digit1);
    let shortcut_show_input = Shortcut::new(Some(Modifiers::SUPER), Code::Digit2);

    let _ = shortcut_manager.register(shortcut_select_text);
    let _ = shortcut_manager.register(shortcut_show_input);
}

/// 处理快捷键事件
pub fn handle_shortcut_event(app: &AppHandle, shortcut: &Shortcut) {
    if let Some(win) = app.get_webview_window("main") {
        if shortcut.matches(Modifiers::SUPER, Code::Digit1) {
            get_data_from_selected_text(win);
        } else if shortcut.matches(Modifiers::SUPER, Code::Digit2) {
            show_input_window(win);
        }
    }
}

/// 停止快捷键监听事件
pub fn stop_shortcuts<R: Runtime>(app: &AppHandle<R>) {
    // app.state::<AppState>()
    //     .shortcut_stopped
    //     .store(true, Relaxed);
    let _ = app.global_shortcut().unregister_all();
}

/// 重启两类快捷键
pub fn restart_shortcuts<R: Runtime>(app: &AppHandle<R>) {
    // app.state::<AppState>()
    //     .shortcut_stopped
    //     .store(false, Relaxed);
    let point = app.state::<AppState>().point_key.clone();
    point.store(Key::F4);

    init_shortcuts(app);
}
