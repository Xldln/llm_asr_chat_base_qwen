
import torch 
from qwen_asr import Qwen3ASRModel
import os


class VoiceDetectService:

    def __init__(self):

        self.asr_model_weights_path = os.path.join(os.path.dirname(__file__), "..", "..", "models")
        self.asr_model = self.init_asr_model()

        
    def init_asr_model(self,device="cpu",model = "Qwen3-ASR-1.7B",max_new_tokens=256):

        model_path = os.path.join(self.asr_model_weights_path, model)
        try:
            model = Qwen3ASRModel.from_pretrained(
            model_path,
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