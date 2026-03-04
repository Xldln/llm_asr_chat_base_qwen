## 配置Qwen3 ASR 环境


`conda create -n qwen3-asr python=3.12 -y`
`conda activate qwen3-asr`

## 这边选择使用vLLM 推理 以获得更快的推理速度和流式支持

`pip install -U qwen-asr[vllm]`


## 选择Qwen3 TTS 0.6Base CustomVoice Model


`pip install -U qwen-tts`

