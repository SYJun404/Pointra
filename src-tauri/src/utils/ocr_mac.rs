use objc2::{rc::Retained, AnyThread};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_foundation::{NSArray, NSData, NSDictionary};
use objc2_vision::{
    VNImageRequestHandler, VNRecognizeTextRequest, VNRecognizedTextObservation, VNRequest,
    VNRequestTextRecognitionLevel,
};
use std::sync::{Arc, Mutex};

// ── 数据结构 ──────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct WordBox {
    pub text: String,
    /// 归一化坐标，Vision 坐标系（原点左下）
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

// ── Tauri State：缓存启动时初始化的 OCR 配置 ──────────────
pub struct OcrState {
    /// 复用的 VNRecognizeTextRequest（已配置好语言和精度）
    pub request: Mutex<Retained<VNRecognizeTextRequest>>,
}

// SAFETY: VNRecognizeTextRequest 在 Mutex 保护下跨线程使用
unsafe impl Send for OcrState {}
unsafe impl Sync for OcrState {}

impl OcrState {
    /// 应在 Tauri setup() 中调用一次，初始化并缓存 request。
    /// 内部会用一张小白图做一次预热推理，把 CoreML 模型加载
    /// 提前到启动阶段，消除首次识别的 1-3 秒延迟。
    pub fn new() -> Arc<Self> {
        let request = unsafe { build_english_request() };
        let state = Arc::new(Self {
            request: Mutex::new(request),
        });
        // clone 一份引用交给后台预热线程，不阻塞 UI 启动
        warmup_async(Arc::clone(&state));
        state
    }
}

/// 构建支持中英混排的 VNRecognizeTextRequest，只执行一次。
unsafe fn build_english_request() -> Retained<VNRecognizeTextRequest> {
    let request = VNRecognizeTextRequest::new();
    request.setRecognitionLevel(VNRequestTextRecognitionLevel::Accurate);
    request.setUsesLanguageCorrection(true);
    request.setAutomaticallyDetectsLanguage(true);
    request
}

/// 在独立线程里跑一次空白图推理，预热 CoreML / ANE
fn warmup_async(state: Arc<OcrState>) {
    std::thread::spawn(move || {
        if let Err(e) = warmup_once(&state) {
            eprintln!("[OCR warmup] 预热失败（不影响功能）: {e}");
        } else {
            eprintln!("[OCR warmup] 预热完成");
        }
    });
}

/// 构造一张 64×64 纯白 PNG，跑一次完整推理以触发模型加载
fn warmup_once(state: &OcrState) -> anyhow::Result<()> {
    use image::{ImageBuffer, Luma};
    let img = ImageBuffer::<Luma<u8>, _>::from_pixel(64, 64, Luma([255u8]));
    let dyn_img = image::DynamicImage::ImageLuma8(img);
    // 结果丢弃，只需模型完成一次完整推理
    let _ = recognize_words(&dyn_img, state)?;
    Ok(())
}

// ── 分词（中英混排感知）──────────────────────────────────

#[inline]
fn is_token_char(c: char) -> bool {
    c.is_ascii_alphabetic()
}

#[inline]
fn is_uppercase(c: char) -> bool {
    c.is_ascii_uppercase()
}

#[inline]
fn is_lowercase(c: char) -> bool {
    c.is_ascii_lowercase()
}

/// CamelCase 拆分：HelloWorld → [Hello, World]
fn refined_latin_ranges(s: &str) -> Vec<std::ops::Range<usize>> {
    let chars: Vec<(usize, char)> = s.char_indices().collect();
    let mut ranges = Vec::new();
    let mut start: Option<usize> = None;
    let mut prev: Option<char> = None;

    for i in 0..chars.len() {
        let (byte_idx, cur) = chars[i];
        let next = chars.get(i + 1).map(|&(_, c)| c);

        if is_token_char(cur) {
            if start.is_none() {
                start = Some(byte_idx);
            } else if let Some(p) = prev {
                let split = (is_lowercase(p) && is_uppercase(cur))
                    || (is_uppercase(p)
                        && is_uppercase(cur)
                        && next.map(is_lowercase).unwrap_or(false));
                if split {
                    ranges.push(start.unwrap()..byte_idx);
                    start = Some(byte_idx);
                }
            }
            prev = Some(cur);
        } else {
            if let Some(s) = start {
                ranges.push(s..byte_idx);
            }
            start = None;
            prev = None;
        }
    }

    if let Some(s) = start {
        ranges.push(s..s_byte_end(s, &chars));
    }
    ranges
}

fn s_byte_end(start: usize, chars: &[(usize, char)]) -> usize {
    // 找到 start 之后最后一个 token char 的结束字节
    chars
        .iter()
        .rev()
        .find(|(bi, _)| *bi >= start)
        .map(|(bi, c)| bi + c.len_utf8())
        .unwrap_or(start)
}

/// 脚本类型：用于区分拉丁字母段、汉字段、其他字母段
#[derive(PartialEq)]
enum TokenKind {
    Latin,
    Han,
    OtherLetter,
}

fn token_kind(c: char) -> Option<TokenKind> {
    if c.is_ascii_alphabetic() {
        return Some(TokenKind::Latin);
    }
    let cp = c as u32;
    if (0x4E00..=0x9FFF).contains(&cp)   // CJK 统一表意文字
        || (0x3400..=0x4DBF).contains(&cp) // CJK 扩展 A
        || (0x20000..=0x2A6DF).contains(&cp) // CJK 扩展 B
        || (0x3040..=0x30FF).contains(&cp)
    // 平假名 / 片假名
    {
        return Some(TokenKind::Han);
    }
    if c.is_alphabetic() {
        return Some(TokenKind::OtherLetter);
    }
    None
}

/// 脚本感知分词：拉丁段做 CamelCase 拆分，汉字段每字单独成 token。
/// 这样 "调用APIRequest时" 会产生 ["API", "Request", "时"] 三个 token，
/// 保证中英混排行里的英文单词都能被正确提取并定位。
fn script_aware_ranges(text: &str) -> Vec<std::ops::Range<usize>> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let mut ranges = Vec::new();
    let mut seg_start: Option<usize> = None;
    let mut cur_kind: Option<TokenKind> = None;

    let flush = |seg_start: usize, end: usize, kind: &TokenKind, ranges: &mut Vec<_>| {
        if seg_start >= end {
            return;
        }
        match kind {
            TokenKind::Latin => {
                ranges.extend(
                    refined_latin_ranges(&text[seg_start..end])
                        .into_iter()
                        .map(|r| (seg_start + r.start)..(seg_start + r.end)),
                );
            }
            _ => ranges.push(seg_start..end),
        }
    };

    for &(byte_idx, c) in &chars {
        match token_kind(c) {
            Some(k) => match &cur_kind {
                None => {
                    seg_start = Some(byte_idx);
                    cur_kind = Some(k);
                }
                Some(ck) if *ck != k => {
                    let ss = seg_start.unwrap();
                    flush(ss, byte_idx, ck, &mut ranges);
                    seg_start = Some(byte_idx);
                    cur_kind = Some(k);
                }
                _ => {}
            },
            None => {
                if let (Some(ss), Some(ck)) = (seg_start, &cur_kind) {
                    flush(ss, byte_idx, ck, &mut ranges);
                }
                seg_start = None;
                cur_kind = None;
            }
        }
    }
    if let (Some(ss), Some(ck)) = (seg_start, &cur_kind) {
        flush(ss, text.len(), ck, &mut ranges);
    }
    ranges
}

/// token 包含至少一个英文字母（汉字行里的中文 token 会被过滤掉，只保留英文）
#[inline]
fn contains_letter(s: &str) -> bool {
    s.bytes().any(|b| b.is_ascii_alphabetic())
}

// ── BBox 计算 ─────────────────────────────────────────────

fn fallback_box(text_box: CGRect, text: &str, range: &std::ops::Range<usize>) -> Option<CGRect> {
    // 纯 ASCII 路径：字符数 == 字节数，避免 chars().count() 的 O(n) 扫描
    let total = if text.is_ascii() {
        text.len()
    } else {
        text.chars().count()
    };
    if total == 0 {
        return None;
    }

    let start_chars = if text.is_ascii() {
        range.start
    } else {
        text[..range.start].chars().count()
    };
    let end_chars = if text.is_ascii() {
        range.end
    } else {
        text[..range.end].chars().count()
    };

    if end_chars <= start_chars {
        return None;
    }

    let sf = start_chars as f64 / total as f64;
    let ef = end_chars as f64 / total as f64;
    let x = text_box.origin.x + text_box.size.width * sf;
    let w = text_box.size.width * (ef - sf);
    if w <= 0.0 {
        return None;
    }

    Some(CGRect {
        origin: CGPoint {
            x,
            y: text_box.origin.y,
        },
        size: CGSize {
            width: w,
            height: text_box.size.height,
        },
    })
}

/// 验证精确 BBox 是否合理
fn is_valid_box(b: CGRect, parent: CGRect) -> bool {
    if b.size.width <= 0.0 || b.size.height <= 0.0 {
        return false;
    }
    let tol = 0.02_f64;
    b.origin.x >= parent.origin.x - tol
        && b.origin.y >= parent.origin.y - tol
        && (b.origin.x + b.size.width) <= (parent.origin.x + parent.size.width + tol)
        && (b.origin.y + b.size.height) <= (parent.origin.y + parent.size.height + tol)
}

fn is_compatible_subrange(precise: CGRect, fallback: CGRect, parent: CGRect) -> bool {
    if precise.size.width <= 0.0 || fallback.size.width <= 0.0 {
        return false;
    }
    if precise.size.width >= parent.size.width * 0.8
        && precise.size.width > fallback.size.width * 1.8
    {
        return false;
    }
    let tol = (parent.size.width * 0.01_f64).max(fallback.size.width * 0.35);
    if precise.origin.x < fallback.origin.x - tol {
        return false;
    }
    if precise.origin.x + precise.size.width > fallback.origin.x + fallback.size.width + tol {
        return false;
    }
    let overlap = (precise.origin.x + precise.size.width)
        .min(fallback.origin.x + fallback.size.width)
        - precise.origin.x.max(fallback.origin.x);
    let ratio = overlap.max(0.0) / precise.size.width.min(fallback.size.width);
    ratio >= 0.35
}

fn resolved_box(
    precise: Option<CGRect>,
    fallback: Option<CGRect>,
    parent: CGRect,
    is_sub: bool,
) -> Option<CGRect> {
    if let Some(p) = precise {
        if is_valid_box(p, parent) {
            if is_sub {
                if let Some(f) = fallback {
                    if !is_compatible_subrange(p, f, parent) {
                        return fallback;
                    }
                }
            }
            return Some(p);
        }
    }
    fallback
}

// ── Vision OCR（使用缓存 State）──────────────────────────
pub fn recognize_words(
    img: &image::DynamicImage,
    state: &OcrState,
) -> anyhow::Result<Vec<WordBox>> {
    let mut png: Vec<u8> = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)?;

    unsafe {
        let ns_data = NSData::with_bytes(&png);

        // 从 State 取出已配置好的 request，无需重建
        let request = state
            .request
            .lock()
            .map_err(|_| anyhow::anyhow!("OcrState mutex poisoned"))?;

        // 向上转型 VNRecognizeTextRequest → VNRequest
        // clone retained pointer 以构造 NSArray，不移动 request
        let as_vn: Retained<VNRequest> = {
            let ptr = &**request as *const VNRecognizeTextRequest;
            // SAFETY: VNRecognizeTextRequest 继承自 VNRequest
            Retained::retain(ptr as *mut VNRequest)
                .ok_or_else(|| anyhow::anyhow!("retain VNRequest failed"))?
        };
        let requests = NSArray::from_retained_slice(&[as_vn]);

        let handler = VNImageRequestHandler::initWithData_options(
            VNImageRequestHandler::alloc(),
            &ns_data,
            &NSDictionary::new(),
        );

        handler
            .performRequests_error(&requests)
            .map_err(|e| anyhow::anyhow!("Vision 执行失败: {:?}", e))?;

        let request_ptr = &**request as *const VNRecognizeTextRequest;
        let results: Retained<NSArray<VNRecognizedTextObservation>> = (*request_ptr)
            .results()
            .ok_or_else(|| anyhow::anyhow!("OCR 无结果"))?;

        let mut words = Vec::new();

        for obs in results.iter() {
            let candidates = obs.topCandidates(1);
            let candidate = match candidates.firstObject() {
                Some(c) => c,
                None => continue,
            };

            let text: String = candidate.string().to_string();
            let text_box: CGRect = obs.boundingBox();

            // 脚本感知分词：中英混排行里拉丁段做 CamelCase 拆分
            for byte_range in script_aware_ranges(&text) {
                let token = &text[byte_range.clone()];
                if !contains_letter(token) {
                    continue;
                }

                // 尝试 Vision 精确 BBox（UTF-16 偏移）
                let precise: Option<CGRect> = {
                    // 纯 ASCII 时 UTF-16 偏移 == 字节偏移，直接用 len()
                    let utf16_start = if text[..byte_range.start].is_ascii() {
                        byte_range.start
                    } else {
                        text[..byte_range.start].encode_utf16().count()
                    };
                    let utf16_len = if text[byte_range.start..byte_range.end].is_ascii() {
                        byte_range.end - byte_range.start
                    } else {
                        text[byte_range.start..byte_range.end]
                            .encode_utf16()
                            .count()
                    };
                    let ns_range = objc2_foundation::NSRange {
                        location: utf16_start,
                        length: utf16_len,
                    };
                    candidate
                        .boundingBoxForRange_error(ns_range)
                        .ok()
                        .map(|obs| obs.boundingBox())
                };

                let fallback = fallback_box(text_box, &text, &byte_range);
                let is_sub = byte_range != (0..text.len());

                if let Some(b) = resolved_box(precise, fallback, text_box, is_sub) {
                    words.push(WordBox {
                        text: token.to_string(),
                        x: b.origin.x,
                        y: b.origin.y,
                        w: b.size.width,
                        h: b.size.height,
                    });
                }
            }
        }

        Ok(words)
    }
}

// ── 命中检测 ──────────────────────────────────────
pub fn select_word(words: &[WordBox], nx: f64, ny: f64) -> Option<String> {
    let nearest = |tolerance: f64| -> Option<&WordBox> {
        let candidates: Vec<&WordBox> = words
            .iter()
            .filter(|w| {
                nx >= w.x - tolerance
                    && nx <= w.x + w.w + tolerance
                    && ny >= w.y - tolerance
                    && ny <= w.y + w.h + tolerance
            })
            .collect();
        candidates.into_iter().min_by(|a, b| {
            let da = ((a.x + a.w / 2.0 - nx).powi(2) + (a.y + a.h / 2.0 - ny).powi(2)).sqrt();
            let db = ((b.x + b.w / 2.0 - nx).powi(2) + (b.y + b.h / 2.0 - ny).powi(2)).sqrt();
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
    };

    nearest(0.0)
        .or_else(|| nearest(0.004))
        .map(|w| w.text.clone())
}
