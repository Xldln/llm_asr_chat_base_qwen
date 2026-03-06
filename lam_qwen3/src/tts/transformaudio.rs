use anyhow::{Context, Result};
use regex::Regex;
use reqwest::blocking::Client;
use serde::Serialize;
use std::sync::LazyLock;
use std::time::Duration; // Rust 1.70+ 推荐使用，旧版本可用 lazy_static

#[derive(Serialize)]
struct TtsRequest {
    text: String,
    instruct: String,
}

pub struct TransformAudioClient {
    client: Client,
    url: String,
}

static RE_MARKDOWN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[*_]{1,2}(.*?)[*_]{1,2}").unwrap());

static RE_EMOJI: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[\u{1F600}-\u{1F64F}\u{1F300}-\u{1F5FF}\u{1F680}-\u{1F6FF}\u{2600}-\u{26FF}\u{2700}-\u{27BF}]").unwrap()
});

static RE_WHITESPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());

impl TransformAudioClient {
    pub fn new(url: &str) -> Self {
        let client: Client = Client::builder()
            .timeout(Duration::from_secs(600))
            .build()
            .unwrap();
        let url: String = url.to_string();
        Self { client, url }
    }

    pub fn transform(&self, text: &str, instruct: &str) -> Result<Vec<u8>> {
        let filtered_text = self.filter_text(text).context("文本清洗失败")?;

        let request_body = TtsRequest {
            text: filtered_text,
            instruct: instruct.to_string(),
        };

        let transform_url = format!("{}/tts/transform", self.url);

        // 调试部分
        println!("Sending TTS request to: {}", transform_url);
        let json_payload = serde_json::to_string_pretty(&request_body)?;

        println!("Debug: 发送给 TTS 服务的完整请求体:");
        println!("{}", json_payload);

        let response = self
            .client
            .post(&transform_url)
            .json(&request_body)
            .send()
            .context("Failed to send request to Axum backend")?;

        if response.status().is_success() {
            let audio_data_bytes = response.bytes().context("Failed to read response bytes")?;
            Ok(audio_data_bytes.to_vec())
        } else {
            let status = response.status();
            let error_msg = response.text().unwrap_or_default();
            Err(anyhow::anyhow!(
                "Axum server returned error: {} - {}",
                status,
                error_msg
            ))
        }
    }

    pub fn filter_text(&self, text: &str) -> Result<String> {
        let step1 = RE_MARKDOWN.replace_all(text, "$1");
        let step2 = RE_EMOJI.replace_all(&step1, "");
        let step3 = RE_WHITESPACE.replace_all(&step2, " ");
        let result = step3.trim().to_string();
        if result.is_empty() {
            return Err(anyhow::anyhow!("清洗后文本内容为空"));
        }
        Ok(result)
    }
}
