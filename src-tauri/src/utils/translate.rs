use crate::AppState;
use md5;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, COOKIE, REFERER, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::State;

#[derive(Serialize, Clone)]
pub struct DictResponse {
    voice: Value,
    translate: Value,
    #[serde(rename = "wordCard")]
    word_card: Value,
}

#[derive(Deserialize)]
struct SogouRawResponse {
    data: Option<SogouData>,
}

#[derive(Deserialize)]
struct SogouData {
    voice: Value,
    translate: Value,
    #[serde(rename = "wordCard")]
    word_card: Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct TranslationParams {
    from: String,
    to: String,
    text: String,
    client: String,
    fr: String,
    #[serde(rename = "needQc")]
    need_qc: u8,
    s: String,
    uuid: String,
    exchange: bool,
}

#[derive(Serialize, Clone)]
pub struct TransResult {
    status: i16,
    msg: Option<String>,
    data: Option<DictResponse>,
}

impl TransResult {
    pub fn success(data: DictResponse) -> Self {
        Self {
            status: 200,
            data: Some(data),
            msg: None,
        }
    }

    pub fn fail(err_msg: String) -> Self {
        Self {
            status: -1,
            data: None,
            msg: Some(err_msg),
        }
    }
}

fn generate_s_token(text: &str) -> String {
    let from = "auto";
    let to = "zh-CHS";
    let salt = "109984457";

    let input = format!("{}{}{}{}", from, to, text, salt);

    let digest = md5::compute(input);
    format!("{:x}", digest)
}

fn build_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36"));
    headers.insert(
        REFERER,
        HeaderValue::from_static("https://fanyi.sogou.com/"),
    );
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/json, text/plain, */*"),
    );
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json;charset=UTF-8"),
    );
    headers.insert(
        COOKIE,
        HeaderValue::from_static(
            "SNUID=9C17E74D454017C3851D74754643D7C3; FQV=2268e6de1ff4ef9d1753f1d9f6c78b38",
        ),
    );

    headers
}

pub async fn fetch_translation(word: &str, state: &AppState) -> Result<DictResponse, String> {
    let url = "https://fanyi.sogou.com/api/transpc/text/result";
    let token = generate_s_token(word);

    let params = TranslationParams {
        from: "auto".to_string(),
        to: "zh-CHS".to_string(),
        text: word.to_string(),
        client: "pc".to_string(),
        fr: "browser_pc".to_string(),
        need_qc: 1,
        s: token,
        uuid: state.device_id.clone(),
        exchange: false,
    };

    let raw_res: SogouRawResponse = state
        .client
        .post(url)
        .headers(build_headers())
        .json(&params)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let data = raw_res.data.ok_or("API returned empty data")?;

    Ok(DictResponse {
        voice: data.voice,
        translate: data.translate,
        word_card: data.word_card,
    })
}

#[tauri::command]
pub async fn fetch_trans_res(state: State<'_, AppState>, word: String) -> Result<TransResult, ()> {
    match fetch_translation(&word, &state).await {
        Ok(data) => Ok(TransResult::success(data)),
        Err(err_msg) => Ok(TransResult::fail(err_msg)),
    }
}
