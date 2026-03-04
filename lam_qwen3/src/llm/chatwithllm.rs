use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Deserialize, Debug)]
pub struct OllamaChatResponse {
    pub message: Message,
    pub done: bool,
}

// 补充缺失的结构体定义
pub struct OllamaChat {
    pub url: String,
    pub model: String,
    pub stream: bool,
}

impl OllamaChat {
    pub fn new(url: &str, model: &str) -> Self {
        Self {
            url: url.to_string(),
            model: model.to_string(),
            stream: false,
        }
    }

    pub fn chat_with_question(&self, question: &str) -> Result<OllamaChatResponse, Box<dyn Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(300))
            .connect_timeout(Duration::from_secs(10))
            .build()?;

        let request_body = OllamaChatRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: question.to_string(),
            }],
            stream: self.stream,
        };

        let ollama_url = format!("{}/api/chat", self.url);

        let response = client
            .post(&ollama_url)
            .json(&request_body)
            .send()?
            .json::<OllamaChatResponse>()?;

        Ok(response)
    }
}
