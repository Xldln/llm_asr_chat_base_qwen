from fastapi import FastAPI, HTTPException, Response,APIRouter, Depends,UploadFile, File, Form,Request,Header
from typing import List
from datetime import datetime
import os
import shutil
from fastapi.responses import JSONResponse,FileResponse,PlainTextResponse,StreamingResponse
import pandas as pd
from ..services.voice_detect_services import VoiceDetectService
import cv2
import io
import numpy as np
import soundfile as sf

import tempfile


voice_router = APIRouter(
    prefix="/voice",  # 所有接口路径前缀，例如 /photon/visualize_photons
    tags=["Voice Detect API"]  # 文档分组标题
)


asr_service = VoiceDetectService("Qwen3-ASR-1.7B")

@voice_router.post("/test")
async def get_voice_result_test(request: Request):
    # 1. 直接从内存读取二进制流（极快，无磁盘IO）
    body = await request.body()
    
    # 2. 将 bytes 转回 numpy 数组
    # 注意：dtype 必须与 Rust 端发送的类型一致（f32 对应 float32）
    audio_np = np.frombuffer(body, dtype=np.float32)
    return JSONResponse(content={"message": "音频数据接收成功", "shape": audio_np.shape, "dtype": str(audio_np.dtype)})
    


@voice_router.post("/detect")
async def get_voice_detect_result(
    request: Request, 
    x_sample_rate: str = Header(None) # 自动从 Header 读取 X-Sample-Rate
):
    body = await request.body()
    
    try:
        sampling_rate = int(x_sample_rate) if x_sample_rate else 16000
        print(f"收到音频请求，采样率: {sampling_rate}Hz")
    except ValueError:
        sampling_rate = 16000
    audio_np = np.frombuffer(body, dtype=np.float32)
    with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as tmp_file:
        tmp_path = tmp_file.name

    try:
        sf.write(tmp_path, audio_np, sampling_rate)
        result_text = asr_service.transcribe(audio=tmp_path)
    except Exception as e:
        print(f"处理出错: {e}")
        return JSONResponse(content={"message": str(e), "code": 500}, status_code=500)
    
    finally:
        if os.path.exists(tmp_path):
            os.remove(tmp_path)

    return JSONResponse(content={
        "message": "转录成功",
        "transcription": result_text,
        "code": 200
    })
    