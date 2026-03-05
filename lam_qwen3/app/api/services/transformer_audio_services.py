import torch
import soundfile as sf
from qwen_tts import Qwen3TTSModel
import torchaudio
import os
from pathlib import Path

class TransformerAudioService:

    def __init__(self,language="Chinese",speaker="Vivian"):

        self.language = language
        self.speaker = speaker
        self.asr_model_weights_path = Path(__file__).parent/ ".." / ".." / ".." / "models"
        self.tts_model = self.init_tts_model()

        
    def init_tts_model(self,model="Qwen3-TTS-12Hz-0.6B-CustomVoice",device="cpu"):
        model_path = (self.asr_model_weights_path / model).resolve()

        model_path_str = str(model_path).replace("\\", "/")

        try:
            model = Qwen3TTSModel.from_pretrained(
                model_path_str,
                device_map=device,
                dtype=torch.bfloat16,
                #attn_implementation="flash_attention_2",
            )
        except Exception as e:
            print(f"加载 TTS 模型失败: {e}")
            raise e
        
        return model
        

    def transform(self,text,instruct=""):
        try:
            wavs, sr = self.tts_model.generate_custom_voice(
                text=text,
                language=self.language, 
                speaker=self.speaker,
                instruct=instruct, 
            )
        except Exception as e:
            print(f"TTS 转换失败: {e}")
            raise e
        
        return wavs, sr
        # # 将生成的音频数据转换为字节流
        # audio_bytes = io.BytesIO()
        # sf.write(audio_bytes, wavs[0], sr, format='WAV')
        # audio_bytes.seek(0)  # 重置指针到文件开头
        
        #return audio_bytes
