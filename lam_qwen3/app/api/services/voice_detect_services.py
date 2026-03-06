
import torch 
from qwen_asr import Qwen3ASRModel
import os
from pathlib import Path

class VoiceDetectService:

    def __init__(self,model="Qwen3-ASR-1.7B",max_new_tokens=256):

        self.asr_model_weights_path = Path(__file__).parent /".." / ".." / ".." / "models"
        self.asr_model = self.init_asr_model(model=model,max_new_tokens=max_new_tokens)

        
    def init_asr_model(self,device="cpu",model = "Qwen3-ASR-1.7B",max_new_tokens=256):

        model_path = (self.asr_model_weights_path / model).resolve()

        model_path_str = str(model_path).replace("\\", "/")

        try:
            model = Qwen3ASRModel.from_pretrained(
            model_path_str,
            dtype=torch.bfloat16,
            device_map = device,
            # attn_implementation="flash_attention_2",
            max_inference_batch_size=32, # Batch size limit for inference. -1 means unlimited. Smaller values can help avoid OOM.
            max_new_tokens=max_new_tokens, # Maximum number of tokens to generate. Set a larger value for long audio input.
        )
        except Exception as e:
            print(f"加载 ASR 模型失败: {e}")
            raise e
        
        return model

    
    def transcribe(self, audio,language="Chinese"):
        # 您可以将音频输入作为本地路径、URL、base64 数据或 (np.ndarray, sr) 元组传入
        
        try:
            results = self.asr_model.transcribe(
                audio = audio,
                language = language,
                )
            
        except Exception as e:
            print(f"ASR 转录失败: {e}")
            raise e
        
        return results[0].text