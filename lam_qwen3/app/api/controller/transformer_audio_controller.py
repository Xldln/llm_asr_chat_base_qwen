from fastapi import FastAPI, HTTPException, Response,APIRouter, Depends,UploadFile, File, Form,Request
from typing import List
from datetime import datetime
import os
import shutil
from fastapi.responses import JSONResponse,FileResponse,PlainTextResponse,StreamingResponse
import pandas as pd
from ..services.transformer_audio_services import TransformerAudioService
import cv2
import io
import numpy as np
import torch
from scipy.io.wavfile import write
from ..request_body.tts import TTSRequest

tts_router = APIRouter(
    prefix="/tts",  # 所有接口路径前缀，例如 /photon/visualize_photons
    tags=["Text-to-Speech API"]  # 文档分组标题
)


tts_service = TransformerAudioService()

@tts_router.post("/transform")
async def transform_text_to_speech(request: TTSRequest):
    try:
        tts_service = TransformerAudioService()

        wavs_p, sr_p = tts_service.transform(request.text, request.instruct)
        audio_np = wavs_p[0].cpu().detach().numpy().flatten()

        # 3. 使用 BytesIO 在内存中构建真正的 WAV 文件
        byte_io = io.BytesIO()
        write(byte_io, sr_p, audio_np) # 这一步会自动添加 WAV 文件头
        byte_io.seek(0) # 指针回到开头以便读取

        return StreamingResponse(byte_io, media_type="audio/wav")
    except Exception as e:
        import traceback
        error_msg = traceback.format_exc()
        print(f"TTS Error Log:\n{error_msg}") # 在服务端后台打印完整错误堆栈
        return JSONResponse(
            status_code=500,
            content={"detail": "TTS Process Failed", "error": str(e)}
        )

