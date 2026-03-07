mod audio;
mod llm;
mod network;
mod tts;
use crate::config::Config;
use anyhow::{Context, Result};
use audio::AudioRecorder;
use cpal::traits::DeviceTrait;
use llm::OllamaChat;
use network::AudioClient;
use std::f32::consts::E;
use std::io::{self, Write};
use std::{
    io::stdout,
    time::{Duration, Instant},
};
use tts::TransformAudioClient;
mod config;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Deserialize)]
struct AsrResponse {
    message: String,
    transcription: String,
    code: i32,
}
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

fn main() -> anyhow::Result<()> {
    // 加载配置文件
    let config = Config::from_env();

    let to_tts = true;
    // 初始化 OLLAMA连接
    let chat_model = "qwen3:4b";
    let mut ollama_chat = OllamaChat::new(&config.ollama_url, &chat_model);
    // println!("连接 Ollama 中，正在测试对话接口...");
    let mut chat_response = String::new();
    // match ollama_chat.chat_with_question(&question) {
    //     Ok(response) => {
    //         chat_response = response.message.content;
    //         println!("{} 回复: {}", chat_model, chat_response);
    //     }
    //     Err(e) => eprintln!("{} 请求失败: {}", chat_model, e),
    // }
    //let question = "你叫什么名字？";
    // 测试回复转语音
    // let tts_text = chat_response;
    // let tts_instruct = "要求自然流畅";
    let mut tts_client = TransformAudioClient::new(&config.base_url);

    println!(">>> 开始实时采集，按 Ctrl+C 停止...");
    // 1. 启动录音模块
    // 选择设备
    let device = AudioRecorder::select_device()?;
    let client = AudioClient::new(&config.base_url);
    let config: cpal::StreamConfig = device.default_input_config()?.into();
    let sample_rate = config.sample_rate.0;

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

        match client.send_audio(buffer, sample_rate) {
            Ok(json_str) => match serde_json::from_str::<AsrResponse>(&json_str) {
                Ok(res) => {
                    if res.code == 200 {
                        let question = res.transcription;
                        match ollama_chat.chat_with_question(&question) {
                            Ok(response) => {
                                chat_response = response.message.content;
                                println!("{} 回复: {}", chat_model, chat_response);

                                if to_tts {
                                    match chat_2_tts(chat_response, &mut tts_client) {
                                        Ok(_) => println!("TTS 转换并播放成功"),
                                        Err(e) => eprintln!("TTS 转换失败: {}", e),
                                    };
                                }
                            }
                            Err(e) => eprintln!("{} 请求失败: {}", chat_model, e),
                        }
                    }
                }
                Err(e) => eprintln!("解析 ASR 响应失败: {}", e),
            },
            Err(e) => eprintln!("发送失败: {}", e),
        }
    }
    Ok(())
}

pub fn chat_2_tts(chat_response: String, tts_client: &mut TransformAudioClient) -> Result<()> {
    let tts_text = chat_response;
    let tts_instruct = "要求自然流畅";

    match tts_client.transform(&tts_text, &tts_instruct) {
        Ok(audio_data) => {
            if audio_data.is_empty() {
                eprintln!("警告：收到的音频数据为空");
                return Ok(());
            }

            println!("TTS 转换成功，音频数据长度: {} 字节", audio_data.len());

            // 1. 确保目录存在
            let output_dir = "outputs";
            std::fs::create_dir_all(output_dir)?;

            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("时间倒流了")
                .as_secs();

            let file_path = format!("{}/output_{}.wav", output_dir, timestamp);

            // 2. 保存文件
            std::fs::write(&file_path, &audio_data)?;
            println!("音频已成功保存至: {}", file_path);

            // 3. 音频自动播放
            // 注意：_stream 必须在函数结束前保持存活，sink.sleep_until_end() 会阻塞直到播放完
            let (_stream, stream_handle) = OutputStream::try_default()
                .map_err(|e| anyhow::anyhow!("无法获取音频输出设备: {}", e))?;

            let sink = Sink::try_new(&stream_handle)
                .map_err(|e| anyhow::anyhow!("无法创建 Sink: {}", e))?;

            // 使用 clone 或直接移动，取决于后面是否还需要数据
            let cursor = Cursor::new(audio_data);
            let source =
                Decoder::new(cursor).map_err(|e| anyhow::anyhow!("音频解码失败: {}", e))?;

            sink.append(source);
            println!("正在播放 AI 回复...");
            sink.sleep_until_end(); // 阻塞直到播放结束
        }
        Err(e) => {
            eprintln!("TTS 转换异常: {:?}", e);
        }
    }

    Ok(())
}
