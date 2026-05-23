use enigo::{Button, Direction::Click, Enigo, Mouse, Settings};
use mouse_position::mouse_position::Mouse as MousePosition;
use tauri::{Emitter, PhysicalPosition, Runtime, WebviewWindow};

pub fn show_main_window<R: Runtime>(window: &WebviewWindow<R>, custom_x: i32, custom_y: i32) {
    if let MousePosition::Position { x, y } = MousePosition::get_mouse_position() {
        let monitors = window.available_monitors().unwrap_or_default();

        let monitor = monitors.iter().find(|m| {
            let pos = m.position();
            let size = m.size();
            let scale = m.scale_factor();

            let logical_x = pos.x as f64 / scale;
            let logical_y = pos.y as f64 / scale;
            let logical_w = size.width as f64 / scale;
            let logical_h = size.height as f64 / scale;

            x as f64 >= logical_x
                && (x as f64) < logical_x + logical_w
                && y as f64 >= logical_y
                && (y as f64) < logical_y + logical_h
        });

        let monitor = match monitor.or_else(|| monitors.first()) {
            Some(m) => m,
            None => return,
        };

        let monitor_pos = monitor.position();
        let monitor_size = monitor.size();
        let scale = monitor.scale_factor();

        let window_phys_w = (320.0 * scale) as i32;
        let window_phys_h = (420.0 * scale) as i32;
        let phys_x = (x as f64 * scale) as i32;
        let phys_y = (y as f64 * scale) as i32;
        let offset = (10.0 * scale) as i32;

        let mut final_x = phys_x + offset;
        let mut final_y = phys_y + offset;

        let monitor_right_edge = monitor_pos.x + monitor_size.width as i32;
        if final_x + window_phys_w > monitor_right_edge {
            final_x = monitor_right_edge - window_phys_w - offset;
            final_y += offset;
        }

        let monitor_bottom_edge = monitor_pos.y + monitor_size.height as i32;
        if final_y + window_phys_h > monitor_bottom_edge {
            final_y = monitor_bottom_edge - window_phys_h - offset;
        }

        let target_pos = PhysicalPosition {
            x: final_x - (custom_x * scale as i32),
            y: final_y - (custom_y * scale as i32),
        };

        // 移动位置，等待跨屏合成完成
        let _ = window.set_position(target_pos);

        std::thread::sleep(std::time::Duration::from_millis(10));

        // 再次设置位置
        let _ = window.set_position(target_pos);
        let _ = window.show();
        let _ = window.set_focus();
    }
}

pub fn show_input_window(window: WebviewWindow) {
    tauri::async_runtime::spawn(async move {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        window.emit("win-router", "search").ok();

        show_main_window(&window, 150, 32);
        std::thread::sleep(std::time::Duration::from_millis(50));
        enigo.button(Button::Left, Click).unwrap();
    });
}
