import os
from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse
import uvicorn

# -------------------------------------------------------------
# LOCAL ORBITAL NODE (26B NATIVE HUGGINGFACE PIPELINE)
# -------------------------------------------------------------
# You must install huggingface libraries: `pip install transformers accelerate`

print("[\u26A1 ORBITAL NODE] Booting Native Transformers Framework... (This will require ~52GB RAM or heavy Swap Memory)")

from transformers import AutoProcessor, AutoModelForCausalLM

app = FastAPI()

MODEL_ID = "google/gemma-4-26B-A4B-it"

import sys

# Initialize the 26B architecture directly to metal memory
try:
    processor = AutoProcessor.from_pretrained(MODEL_ID)
    model = AutoModelForCausalLM.from_pretrained(
        MODEL_ID,
        dtype="auto", 
        device_map="auto" # This attempts to spill matrix memory across CPU/GPU/Disk
    )
    print(f"[\u26A1 ORBITAL NODE] Native Matrix Loaded Successfully on {model.device}!")
except Exception as e:
    print(f"[FATAL ORBITAL ERROR] Memory Allocation Failed: {e}")
    print("The node is shutting down because it physically cannot load the 26 Billion parameters.")
    sys.exit(1)

@app.post("/v1/chat/completions")
async def chat_completion(request: Request):
    payload = await request.json()
    messages = payload.get("messages", [])
    
    try:
        # Process inputs matching the Gemma-4 instruction template
        text = processor.apply_chat_template(
            messages, 
            tokenize=False, 
            add_generation_prompt=True, 
            enable_thinking=False
        )
        inputs = processor(text=text, return_tensors="pt").to(model.device)
        input_len = inputs["input_ids"].shape[-1]
        
        # Generator execution (This will throttle CPU/SSD due to Memory Swap limitations)
        outputs = model.generate(**inputs, max_new_tokens=1024)
        response_text = processor.decode(outputs[0][input_len:], skip_special_tokens=True)
        
        # Package into standard OpenAI compatibility
        return JSONResponse(content={
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": response_text
                }
            }]
        })
    except Exception as e:
        print(f"[GENERATION PANIC] {e}")
        return JSONResponse(status_code=500, content={"error": str(e)})

if __name__ == "__main__":
    # Booting on 8082 to act as the second-tier intelligence node
    uvicorn.run(app, host="127.0.0.1", port=8082)
