// capture.rs
use crate::AppState;
use image::DynamicImage;
use mouse_position::mouse_position::Mouse;
use screenshots::Screen;
use tauri::State;

pub struct ScreenCache {
    // 缓存所有屏幕对象
    pub screens: Vec<Screen>,
}

impl ScreenCache {
    pub fn new() -> Self {
        Self {
            screens: Screen::all().unwrap_or_default(),
        }
    }
    // todo
    // 提供一个刷新方法，万一用户插拔了显示器
    // pub fn refresh(&self) -> anyhow::Result<()> {
    //     let mut cache = self.screens.lock().unwrap();
    //     *cache = Screen::all().map_err(|e| anyhow::anyhow!(e.to_string()))?;
    //     Ok(())
    // }
}

pub fn get_mouse_pos() -> anyhow::Result<(i32, i32)> {
    match Mouse::get_mouse_position() {
        Mouse::Position { x, y } => Ok((x, y)),
        Mouse::Error => Err(anyhow::anyhow!("无法获取鼠标坐标")),
    }
}

pub fn capture_around_cursor(
    app_state: State<'_, AppState>,
    half_w: i32,
    half_h: i32,
) -> anyhow::Result<(DynamicImage, i32, i32)> {
    let (mx, my) = get_mouse_pos()?;

    let screens = &app_state.screen_cache.screens;
    let screen = screens
        .iter()
        .find(|s| {
            let d = &s.display_info;
            mx >= d.x && mx < d.x + d.width as i32 && my >= d.y && my < d.y + d.height as i32
        })
        .ok_or_else(|| anyhow::anyhow!("未找到对应屏幕"))?;

    let d = &screen.display_info;
    let scale = d.scale_factor;

    let x = ((mx - half_w).max(d.x) as f32 / scale) as i32;
    let y = ((my - half_h).max(d.y) as f32 / scale) as i32;
    let w = ((half_w * 2) as f32 / scale) as u32;
    let h = ((half_h * 2) as f32 / scale) as u32;

    let captured = screen.capture_area(x, y, w, h)?;

    // 通过原始字节 + 尺寸重建，完全绕过类型版本冲突
    let (img_w, img_h) = (captured.width(), captured.height());
    let raw_bytes: Vec<u8> = captured.into_raw(); // 消耗 captured，取出字节

    let img = image::RgbaImage::from_raw(img_w, img_h, raw_bytes)
        .map(DynamicImage::ImageRgba8)
        .ok_or_else(|| anyhow::anyhow!("图像字节长度不匹配"))?;

    let rel_x = (mx as f32 / scale) as i32 - x;
    let rel_y = (my as f32 / scale) as i32 - y;

    Ok((img, rel_x, rel_y))
}
