from fastapi import FastAPI, HTTPException, Response,APIRouter, Depends,UploadFile, File, Form,Request
from typing import List
from datetime import datetime
import os
import shutil
from fastapi.responses import JSONResponse,FileResponse,PlainTextResponse,StreamingResponse
import pandas as pd

import cv2
import io
import numpy as np

voice_router = APIRouter(
    prefix="/voice",  # 所有接口路径前缀，例如 /photon/visualize_photons
    tags=["Voice Detect API"]  # 文档分组标题
)

@voice_router.post("/test")
async def get_voice_result_test(request: Request):
    # 1. 直接从内存读取二进制流（极快，无磁盘IO）
    body = await request.body()
    
    # 2. 将 bytes 转回 numpy 数组
    # 注意：dtype 必须与 Rust 端发送的类型一致（f32 对应 float32）
    audio_np = np.frombuffer(body, dtype=np.float32)
    
    # 3. 直接喂给 Qwen-ASR 模型
    # 假设你已经定义好了 model 和采样率 SR
    # results = model.transcribe(
    #     audio=(audio_np, 16000), # 确保采样率匹配
    #     language=None
    # )
    return JSONResponse(content={"message": "音频数据接收成功", "shape": audio_np.shape, "dtype": str(audio_np.dtype)})
    


# @voice_router.post("/detect")
# async def get_voice_detect_result(request: Request):
#     # 1. 直接从内存读取二进制流（极快，无磁盘IO）
#     body = await request.body()
    
#     # 2. 将 bytes 转回 numpy 数组
#     # 注意：dtype 必须与 Rust 端发送的类型一致（f32 对应 float32）
#     audio_np = np.frombuffer(body, dtype=np.float32)
    
#     # 3. 直接喂给 Qwen-ASR 模型
#     # 假设你已经定义好了 model 和采样率 SR
#     # results = model.transcribe(
#     #     audio=(audio_np, 16000), # 确保采样率匹配
#     #     language=None
#     # )
    