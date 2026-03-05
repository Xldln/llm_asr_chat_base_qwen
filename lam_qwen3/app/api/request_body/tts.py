from pydantic import BaseModel

# 1. 定义请求体结构
class TTSRequest(BaseModel):
    text: str
    instruct: str = ""