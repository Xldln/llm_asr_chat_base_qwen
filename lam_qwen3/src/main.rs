mod audio;
mod llm;
mod network;
mod tts;
use crate::config::Config;
use anyhow::{Context, Result};
use audio::AudioRecorder;
use llm::OllamaChat;
use network::AudioClient;
use std::io::{self, Write};
use std::{
    io::stdout,
    time::{Duration, Instant},
};
use tts::TransformAudioClient;
mod config;

fn main() -> anyhow::Result<()> {
    // 加载配置文件
    let config = Config::from_env();

    let chat_model = "qwen3:4b";
    let mut ollama_chat = OllamaChat::new(&config.ollama_url, &chat_model);

    println!("连接 Ollama 中，正在测试对话接口...");
    let question = "你叫什么名字？";

    let mut chat_response = String::new();
    match ollama_chat.chat_with_question(&question) {
        Ok(response) => {
            chat_response = response.message.content;
            println!("{} 回复: {}", chat_model, chat_response);
        }
        Err(e) => eprintln!("{} 请求失败: {}", chat_model, e),
    }

    // 测试回复转语音
    let tts_text = chat_response;
    let tts_instruct = "要求自然流畅";

    let mut tts_client = TransformAudioClient::new(&config.base_url);

    match tts_client.transform(&tts_text, &tts_instruct) {
        Ok(audio_data) => {
            let audio_data: Vec<u8> = audio_data;
            if audio_data.is_empty() {
                eprintln!("警告：收到的音频数据为空");
            } else {
                println!("TTS 转换成功，音频数据长度: {} 字节", audio_data.len());
                // 接下来你可以：std::fs::write("output.wav", audio_data).ok();
            }
        }
        Err(e) => {
            // 这里 e 会包含你在 client.rs 里写的 .context() 信息，非常易于排错
            eprintln!("TTS 转换异常: {:?}", e);
        }
    }

    // println!(">>> 开始实时采集，按 Ctrl+C 停止...");
    // // 1. 启动录音模块
    // // _recorder 必须在作用域内，否则 stream 会被析构导致录音停止
    // // 选择设备
    // let device = AudioRecorder::select_device()?;
    // let client = AudioClient::new(config.base_url);

    // loop {
    //     print!(">按回车键开始录音5s!");
    //     io::stdout().flush();
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input)?;

    //     // 启动录音模块
    //     let (_recorder, rx) = AudioRecorder::start(device.clone())?;

    //     println!(">>> 录音中...请说话，5秒后自动停止录音");
    //     let start_time = Instant::now();
    //     let record_duration = Duration::from_secs(5);
    //     let mut buffer = Vec::new();
    //     while start_time.elapsed() < record_duration {
    //         if let Ok(samples) = rx.recv_timeout(Duration::from_millis(100)) {
    //             buffer.extend(samples);
    //         }
    //     }
    //     println!(
    //         "录音结束，采集到 {} 个采样点。正在发送识别...",
    //         buffer.len()
    //     );

    //     match client.send_audio(buffer) {
    //         Ok(res) => println!("识别结果: {}", res),
    //         Err(e) => eprintln!("发送失败: {}", e),
    //     }
    // }

    Ok(())
}
