use crate::AppState;
use md5;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, COOKIE, REFERER, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use uuid::Uuid;

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

fn generate_s_token(text: &str) -> (String, bool) {
    // 根据text判断是否为英文
    let is_english = text.as_bytes().is_ascii();

    let from = if is_english { "en" } else { "zh-CHS" };
    let to = if is_english { "zh-CHS" } else { "en" };
    let salt = "109984457";

    let input = format!("{}{}{}{}", from, to, text, salt);

    let digest = md5::compute(input);

    if is_english {
        return (format!("{:x}", digest), true);
    }
    (format!("{:x}", digest), false)
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn is_one_day_apart(ts1: u64, ts2: u64) -> bool {
    let one_day_ms: u64 = 1000 * 60 * 60 * 24;
    ts1.abs_diff(ts2) >= one_day_ms
}

async fn fetch_sogou_cookie(client: &reqwest::Client) -> Result<(String, String), String> {
    let resp = client
        .get("https://fanyi.sogou.com")
        .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36")
        .header(CONTENT_TYPE, "application/json;charset=UTF-8")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch cookie page: {}", e))?;

    let mut snuid = String::new();
    let mut fqv = String::new();

    for value in resp.headers().get_all("set-cookie").iter() {
        if let Ok(cookie_str) = value.to_str() {
            let pair = cookie_str.split(';').next().unwrap_or("").trim();
            if pair.starts_with("SNUID=") {
                snuid = pair.trim_start_matches("SNUID=").to_string();
            } else if pair.starts_with("FQV=") {
                fqv = pair.trim_start_matches("FQV=").to_string();
            }
        }
    }

    if snuid.is_empty() || fqv.is_empty() {
        return Err("Failed to extract SNUID or FQV from set-cookie headers".to_string());
    }

    Ok((snuid, fqv))
}

async fn get_or_refresh_cookie(
    client: &reqwest::Client,
    app: &AppHandle,
    is_second: bool,
) -> Result<(String, String), String> {
    let store = app
        .store("cookie_store.json")
        .map_err(|e| format!("Failed to open store: {}", e))?;

    let now = now_ms();
    // for key in store.keys() {
    //     println!("{}: {:?}", key, store.get(&key));
    // }

    // 读取缓存的时间戳
    let cached_ts: Option<u64> = store.get("cookie_ts").and_then(|v| v.as_u64());

    let needs_refresh = cached_ts
        .map(|ts| is_one_day_apart(ts, now))
        .unwrap_or(true); // 没有缓存时也刷新

    if !needs_refresh && !is_second {
        let cookie = store
            .get("cookie")
            .and_then(|v| v.as_str().map(str::to_string));
        let uuid = store
            .get("uuid")
            .and_then(|v| v.as_str().map(str::to_string));

        if let (Some(cookie), Some(uuid)) = (cookie, uuid) {
            return Ok((cookie, uuid));
        }
    }

    // 重新获取 cookie
    let (snuid, fqv) = fetch_sogou_cookie(client).await?;
    let cookie_str = format!("SNUID={}; FQV={}", snuid, fqv);
    // 重新生成 uuid
    let uuid = Uuid::new_v4().to_string();

    store.set("cookie", Value::String(cookie_str.clone()));
    store.set("cookie_ts", Value::Number(now.into()));
    store.set("uuid", Value::String(uuid.clone()));
    store
        .save()
        .map_err(|e| format!("Failed to save cookie store: {}", e))?;

    Ok((cookie_str, uuid))
}

fn build_headers(cookie: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36",
        ),
    );
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
        HeaderValue::from_str(cookie).unwrap_or_else(|_| HeaderValue::from_static("")),
    );

    headers
}

async fn post_and_parse(
    client: &reqwest::Client,
    url: &str,
    cookie: &str,
    params: &TranslationParams,
) -> Result<SogouRawResponse, String> {
    client
        .post(url)
        .headers(build_headers(cookie))
        .json(params)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))
}

pub async fn fetch_translation(
    word: &str,
    state: &AppState,
    app: &AppHandle,
) -> Result<DictResponse, String> {
    let url = "https://fanyi.sogou.com/api/transpc/text/result";
    let (token, is_english) = generate_s_token(word);

    let (cookie, uuid) = get_or_refresh_cookie(&state.client, app, false).await?;

    let params = TranslationParams {
        from: if is_english { "en" } else { "zh-CHS" }.to_string(),
        to: if is_english { "zh-CHS" } else { "en" }.to_string(),
        text: word.to_string(),
        client: "pc".to_string(),
        fr: "browser_pc".to_string(),
        need_qc: 1,
        s: token,
        uuid: uuid,
        exchange: false,
    };

    // 首次请求
    let raw_res = post_and_parse(&state.client, url, &cookie, &params).await;

    // 出错时尝试重新获取 cookie 并重试一次
    let raw_res = match raw_res {
        Ok(resp) => resp,
        Err(_) => {
            let (new_cookie, new_uuid) = get_or_refresh_cookie(&state.client, app, true).await?;
            let mut new_params = params;
            new_params.uuid = new_uuid;
            post_and_parse(&state.client, url, &new_cookie, &new_params).await?
        }
    };

    let data = raw_res.data.ok_or("API returned empty data")?;

    Ok(DictResponse {
        voice: data.voice,
        translate: data.translate,
        word_card: data.word_card,
    })
}
