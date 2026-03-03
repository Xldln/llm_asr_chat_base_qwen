use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::{Receiver, unbounded};
use std::io::{self, Write};
pub struct AudioRecorder {
    stream: cpal::Stream,
}

impl AudioRecorder {
    /// 初始化并开始录音，返回一个接收音频数据的 Receiver
    pub fn start(device: cpal::Device) -> Result<(Self, Receiver<Vec<f32>>)> {
        // let host = cpal::default_host();
        // let device = host
        //     .default_input_device()
        //     .context("未能找到默认输入设备")?;
        // let config: cpal::StreamConfig = device.default_input_config()?.into();
        // 强制使用单声道，采样率尽量贴近 16000 (Qwen3 所需)
        let config: cpal::StreamConfig = device.default_input_config()?.into();
        let sample_rate = config.sample_rate.0;
        println!(
            "正在开启录音: {} 通道, 采样率 {}Hz",
            config.channels, sample_rate
        );
        let (tx, rx) = unbounded::<Vec<f32>>();

        // 定义音频处理回调
        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                // 将原始采样数据克隆并发送到 Channel
                // 这里只做最小化的内存拷贝
                let _ = tx.send(data.to_vec());
            },
            |err| eprintln!("录音流出错: {}", err),
            None,
        )?;

        stream.play()?;

        Ok((Self { stream }, rx))
    }

    pub fn select_device() -> Result<cpal::Device> {
        let host = cpal::default_host();
        let devices: Vec<cpal::Device> = host.input_devices()?.collect();
        if devices.is_empty() {
            return Err(anyhow::anyhow!("未发现任何输入设备"));
        }
        println!("\n=== 可用输入设备列表 ===");
        for (index, device) in devices.iter().enumerate() {
            // 这里的 index + 1 是为了符合用户从 1 开始计数的习惯
            println!(
                "{}: {}",
                index + 1,
                device.name().unwrap_or_else(|_| "未知设备".into())
            );
        }
        // 2. 交互循环：直到用户输入正确的数字为止
        loop {
            print!("请输入设备编号 (1-{}): ", devices.len());
            io::stdout().flush()?; // 确保提示文字立即显示
            let mut input = String::new();
            io::stdin().read_line(&mut input).context("读取输入失败")?;
            match input.trim().parse::<usize>() {
                Ok(num) if num > 0 && num <= devices.len() => {
                    // 用户输入的是 1-based，转回 0-based 索引
                    let selected_device = devices.into_iter().nth(num - 1).unwrap();
                    println!("已选择设备: {}", selected_device.name()?);
                    return Ok(selected_device);
                }
                _ => {
                    println!("输入无效，请输入 1 到 {} 之间的数字。", devices.len());
                }
            }
        }
    }
}
