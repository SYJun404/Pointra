use objc2::rc::Retained;
use objc2::AnyThread;
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_foundation::{NSArray, NSData, NSDictionary, NSString};
use objc2_vision::{
    VNImageRequestHandler, VNRecognizeTextRequest, VNRecognizedTextObservation, VNRequest,
    VNRequestTextRecognitionLevel,
};

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

// ── 分词（移植自 OCRService.swift）────────────────────────

fn is_token_char(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn is_uppercase(c: char) -> bool {
    c.is_uppercase() && c.is_alphabetic()
}

fn is_lowercase(c: char) -> bool {
    c.is_lowercase() && c.is_alphabetic()
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
    // CJK Unified Ideographs 范围
    let cp = c as u32;
    if (0x4E00..=0x9FFF).contains(&cp)
        || (0x3400..=0x4DBF).contains(&cp)
        || (0x20000..=0x2A6DF).contains(&cp)
        || (0x3040..=0x30FF).contains(&cp)
    // Hiragana/Katakana
    {
        return Some(TokenKind::Han);
    }
    if c.is_alphabetic() {
        return Some(TokenKind::OtherLetter);
    }
    None
}

/// 脚本感知分词：拉丁/汉字/其他字母分段，拉丁段再做 CamelCase 拆分
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

    for (_i, &(byte_idx, c)) in chars.iter().enumerate() {
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
    // 末尾 flush
    if let (Some(ss), Some(ck)) = (seg_start, &cur_kind) {
        let end = text.len();
        flush(ss, end, ck, &mut ranges);
    }
    ranges
}

fn contains_letter(s: &str) -> bool {
    s.chars().any(|c| {
        c.is_alphabetic() || {
            let cp = c as u32;
            (0x4E00..=0x9FFF).contains(&cp)
        }
    })
}

// ── BBox 计算 ─────────────────────────────────────────────

/// 字符数比例 fallback
fn fallback_box(text_box: CGRect, text: &str, range: &std::ops::Range<usize>) -> Option<CGRect> {
    let total = text.chars().count();
    if total == 0 {
        return None;
    }

    let start_chars = text[..range.start].chars().count();
    let end_chars = text[..range.end].chars().count();
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

// ── Vision OCR ────────────────────────────────────────────

pub fn recognize_words(img: &image::DynamicImage) -> anyhow::Result<Vec<WordBox>> {
    let mut png: Vec<u8> = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)?;

    unsafe {
        let ns_data = NSData::with_bytes(&png);

        let request = VNRecognizeTextRequest::new();
        request.setRecognitionLevel(VNRequestTextRecognitionLevel::Accurate);
        request.setUsesLanguageCorrection(true);

        // macOS 13+ 自动语言检测
        #[cfg(target_os = "macos")]
        {
            use objc2::msg_send;
            let os_ver: f64 = {
                let ver: objc2_foundation::NSOperatingSystemVersion =
                    objc2_foundation::NSProcessInfo::processInfo().operatingSystemVersion();
                ver.majorVersion as f64 + ver.minorVersion as f64 / 10.0
            };
            if os_ver >= 13.0 {
                let _: () = msg_send![&*request, setAutomaticallyDetectsLanguage: true];
            } else {
                let en = NSString::from_str("en-US");
                let zh = NSString::from_str("zh-Hans");
                let langs = NSArray::from_retained_slice(&[en, zh]);
                request.setRecognitionLanguages(&langs);
            }
        }

        // 向上转型 VNRecognizeTextRequest → VNRequest
        let request_ptr = &*request as *const VNRecognizeTextRequest;
        let as_vn: Retained<VNRequest> = request.into_super().into_super();
        let requests = NSArray::from_retained_slice(&[as_vn]);

        let handler = VNImageRequestHandler::initWithData_options(
            VNImageRequestHandler::alloc(),
            &ns_data,
            &NSDictionary::new(),
        );

        handler
            .performRequests_error(&requests)
            .map_err(|e| anyhow::anyhow!("Vision 执行失败: {:?}", e))?;

        // 从原始指针取结果
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

            // 对整行文字做脚本感知分词
            for byte_range in script_aware_ranges(&text) {
                let token = &text[byte_range.clone()];
                if !contains_letter(token) {
                    continue;
                }

                // 尝试 Vision 精确 BBox
                let precise: Option<CGRect> = {
                    // 把字节 range 转为 NSRange（UTF-16 偏移）
                    let utf16_start = text[..byte_range.start].encode_utf16().count();
                    let utf16_len = text[byte_range.start..byte_range.end]
                        .encode_utf16()
                        .count();
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

// ── 命中检测（移植自 selectWord）─────────────────────────

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
