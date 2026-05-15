use crate::AppState;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use tauri::State;

pub struct AudioState {
    pub sink: Arc<Mutex<Option<Sink>>>,
    pub _stream: Arc<Mutex<Option<OutputStream>>>,
    pub volume: Arc<Mutex<f32>>,
}

unsafe impl Send for AudioState {}
unsafe impl Sync for AudioState {}

impl AudioState {
    pub fn new(initial_volume: f32) -> Self {
        AudioState {
            sink: Arc::new(Mutex::new(None)),
            _stream: Arc::new(Mutex::new(None)),
            volume: Arc::new(Mutex::new(initial_volume)),
        }
    }
}

/// 播放 URL 音频
#[tauri::command]
pub async fn play_phonetic_url(url: String, state: State<'_, AppState>) -> Result<(), String> {
    // 下载音频数据
    let bytes = reqwest::get(&url)
        .await
        .map_err(|e| format!("请求失败: {}", e))?
        .bytes()
        .await
        .map_err(|e| format!("读取数据失败: {}", e))?;

    let cursor = Cursor::new(bytes.to_vec());

    // 创建输出流（必须在同一线程保持存活）
    let (stream, stream_handle) =
        OutputStream::try_default().map_err(|e| format!("音频设备初始化失败: {}", e))?;

    let sink = Sink::try_new(&stream_handle).map_err(|e| format!("创建 Sink 失败: {}", e))?;

    let vol = *state.audio_state.volume.lock().unwrap();
    sink.set_volume(vol);

    let source = Decoder::new(cursor).map_err(|e| format!("解码音频失败: {}", e))?;

    sink.append(source);

    // 保存到状态
    *state.audio_state.sink.lock().unwrap() = Some(sink);
    *state.audio_state._stream.lock().unwrap() = Some(stream);

    Ok(())
}

/// 设置音量 (0.0 ~ 1.0)
#[tauri::command]
pub fn set_volume(volume: f32, state: State<'_, AppState>) -> Result<(), String> {
    let vol = volume.clamp(0.0, 1.0);

    *state.audio_state.volume.lock().unwrap() = vol;

    // 如果当前有 sink 也立即生效
    if let Some(sink) = state.audio_state.sink.lock().unwrap().as_ref() {
        sink.set_volume(vol);
    }
    Ok(())
}
