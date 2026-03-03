use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<Message>, // 必须是数组
    stream: bool,
}

#[derive(Deserialize, Debug)]
struct OllamaChatResponse {
    message: Message, // 注意：chat 接口返回的是 message 对象
    done: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let ollama_url = "http://localhost:11434/api/chat";

    let user_input = "What's your name?";

    let request_body = OllamaChatRequest {
        model: "qwen3:4b".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: user_input.to_string(),
        }],
        stream: false,
    };

    println!("正在发送请求到 Ollama Chat API...");

    let res = client.post(ollama_url).json(&request_body).send().await?;

    if res.status().is_success() {
        let response_json: OllamaChatResponse = res.json().await?;
        println!("\nAssistant: {}", response_json.message.content);
    } else {
        println!("请求失败: {}", res.status());
    }

    Ok(())
}
