use anyhow::Context;
use core_graphics::display::CGDisplay;
use core_graphics::geometry::{CGPoint, CGRect, CGSize};
use image::DynamicImage;
use mouse_position::mouse_position::Mouse;
use screenshots::Screen;

#[derive(Debug)]
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
    screen_cache: &ScreenCache,
    half_w: i32,
    half_h: i32,
) -> anyhow::Result<(DynamicImage, i32, i32)> {
    let (mx, my) = get_mouse_pos()?;

    let screens = &screen_cache.screens;
    let screen = screens
        .iter()
        .find(|s| {
            let d = &s.display_info;
            mx >= d.x && mx < d.x + d.width as i32 && my >= d.y && my < d.y + d.height as i32
        })
        .ok_or_else(|| anyhow::anyhow!("未找到对应屏幕"))?;

    let d = &screen.display_info;
    let scale = d.scale_factor;

    let x = ((mx - half_w).max(d.x) as f32 / scale) as f64;
    let y = ((my - half_h).max(d.y) as f32 / scale) as f64;
    let w = ((half_w * 2) as f32 / scale) as f64;
    let h = ((half_h * 2) as f32 / scale) as f64;

    let rect = CGRect::new(&CGPoint::new(x, y), &CGSize::new(w, h));

    // ===== 截图 =====
    let cg_image = CGDisplay::main()
        .image_for_rect(rect)
        .context("capture failed")?;

    // ===== CGImage → DynamicImage =====
    let (img_w, img_h) = (cg_image.width(), cg_image.height());
    let src_stride = cg_image.bytes_per_row();
    let raw_data = cg_image.data();
    let raw_ref = raw_data.bytes();

    // macOS CGImage 像素格式为 BGRA（kCGBitmapByteOrder32Little），
    // 且 bytes_per_row 可能包含 stride 填充。逐行复制并转为 RGBA。
    let row_bytes = (img_w * 4) as usize;
    let mut rgba_bytes: Vec<u8> = Vec::with_capacity((img_h as usize) * row_bytes);

    for y in 0..img_h as usize {
        let row = &raw_ref[y * src_stride..y * src_stride + row_bytes];
        for px in row.chunks(4) {
            rgba_bytes.extend_from_slice(&[px[2], px[1], px[0], px[3]]);
        }
    }

    let img = image::RgbaImage::from_raw(img_w as u32, img_h as u32, rgba_bytes)
        .map(DynamicImage::ImageRgba8)
        .context("图像字节长度不匹配")?;

    let rel_x = (mx as f32 / scale) as i32 - x as i32;
    let rel_y = (my as f32 / scale) as i32 - y as i32;

    Ok((img, rel_x, rel_y))
}
