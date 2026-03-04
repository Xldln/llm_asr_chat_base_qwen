mod audio;
mod llm;
mod network;

use anyhow::{Context, Result};
use audio::AudioRecorder;
use llm::OllamaChat;
use network::AudioClient;
use std::io::{self, Write};
use std::{
    io::stdout,
    time::{Duration, Instant},
};

fn main() -> anyhow::Result<()> {

    
    let chat_model = "qwen3:4b";
    let mut ollama_chat = OllamaChat::new("http://127.0.0.1:11434", chat_model);

    println!("连接 Ollama 中，正在测试对话接口...");
    let question = "what's your name? and tell me a joke.";

    match ollama_chat.chat_with_question(question) {
        Ok(response) => println!("{} 回复: {}", chat_model, response.message.content),
        Err(e) => eprintln!("{} 请求失败: {}", chat_model, e),
    }

    println!(">>> 开始实时采集，按 Ctrl+C 停止...");
    // 1. 启动录音模块
    // _recorder 必须在作用域内，否则 stream 会被析构导致录音停止
    // 选择设备
    let device = AudioRecorder::select_device()?;
    let client = AudioClient::new("http://127.0.0.1:8081/voice/test");

    loop {
        print!(">按回车键开始录音5s!");
        io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // 启动录音模块
        let (_recorder, rx) = AudioRecorder::start(device.clone())?;

        println!(">>> 录音中...请说话，5秒后自动停止录音");
        let start_time = Instant::now();
        let record_duration = Duration::from_secs(5);
        let mut buffer = Vec::new();
        while start_time.elapsed() < record_duration {
            if let Ok(samples) = rx.recv_timeout(Duration::from_millis(100)) {
                buffer.extend(samples);
            }
        }
        println!(
            "录音结束，采集到 {} 个采样点。正在发送识别...",
            buffer.len()
        );

        match client.send_audio(buffer) {
            Ok(res) => println!("识别结果: {}", res),
            Err(e) => eprintln!("发送失败: {}", e),
        }
    }

    Ok(())
}
