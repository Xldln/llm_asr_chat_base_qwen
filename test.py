import torch
from qwen_asr import Qwen3ASRModel

if __name__ == '__main__':


    import llguidance
    import vllm
    print("llguidance and vllm imported successfully!")

    import torch
    from qwen_asr import Qwen3ASRModel

    model = Qwen3ASRModel.from_pretrained(
        "./lam_qwen3/models/Qwen3-ASR-1.7B",
        dtype=torch.bfloat16,
        device_map="cpu",
        # attn_implementation="flash_attention_2",
        max_inference_batch_size=32, # Batch size limit for inference. -1 means unlimited. Smaller values can help avoid OOM.
        max_new_tokens=256, # Maximum number of tokens to generate. Set a larger value for long audio input.
    )

    results = model.transcribe(
        audio="https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen3-ASR-Repo/asr_en.wav",
        language=None, # set "English" to force the language
    )

    print(results[0].language)
    print(results[0].text)
    # model = Qwen3ASRModel.LLM(
    #     model="./models/Qwen3-ASR-1.7B",
    #     gpu_memory_utilization=0.7,
    #     max_inference_batch_size=128, # Batch size limit for inference. -1 means unlimited. Smaller values can help avoid OOM.
    #     max_new_tokens=4096, # Maximum number of tokens to generate. Set a larger value for long audio input.
    #     forced_aligner="Qwen/Qwen3-ForcedAligner-0.6B",
    #     forced_aligner_kwargs=dict(
    #         dtype=torch.bfloat16,
    #         device_map="cuda:0",
    #         # attn_implementation="flash_attention_2",
    #     ),
    # )

    # results = model.transcribe(
    #     audio=[
    #     "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen3-ASR-Repo/asr_zh.wav",
    #     "https://qianwen-res.oss-cn-beijing.aliyuncs.com/Qwen3-ASR-Repo/asr_en.wav",
    #     ],
    #     language=["Chinese", "English"], # can also be set to None for automatic language detection
    #     return_time_stamps=True,
    # )

    # for r in results:
    #     print(r.language, r.text, r.time_stamps[0])