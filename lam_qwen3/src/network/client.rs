use anyhow::{Context, Result};
use reqwest::blocking::Client;
use std::time::Duration;

pub struct AudioClient {
    client: Client,
    url: String,
}

impl AudioClient {
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap(),
            url: url.to_string(),
        }
    }

    pub fn send_audio(&self, samples: Vec<f32>, sample_rate: u32) -> Result<String> {
        let bytes: Vec<u8> = samples
            .into_iter()
            .flat_map(|sample| sample.to_le_bytes())
            .collect();

        let audio_url = format!("{}/voice/detect", self.url);

        let response = self
            .client
            .post(&audio_url)
            .header("Content-Type", "application/octet-stream")
            .header("X-Sample-Rate", sample_rate.to_string())
            .body(bytes)
            .send()
            .context("Failed to send request to Python backend")?;
        if response.status().is_success() {
            let body = response.text()?;
            Ok(body)
        } else {
            Err(anyhow::anyhow!(
                "Server returned error: {}",
                response.status()
            ))
        }
    }
}
