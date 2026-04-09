import os
import httpx
from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse

app = FastAPI()

# ---------------------------------------------------------
# SOVEREIGN SMART ROUTER (HYBRID DEEPSEEK EXTENSION)
# ---------------------------------------------------------
# Tier 1: Local Sentinel (Runs natively on your M1 Mac Unified Memory)
LOCAL_E4B_URL = "http://127.0.0.1:8081/v1/chat/completions"

# Tier 2: Orbital DeepSeek Node (Reasoning Layer)
DEEPSEEK_API_URL = "https://api.deepseek.com/chat/completions"
DEEPSEEK_API_KEY = os.getenv("DEEPSEEK_API_KEY", "")

def requires_heavy_logic(messages):
    """
    Heuristic matrix to decide if the massive Orbital DeepSeek model is required.
    """
    total_chars = sum(len(m.get("content", "")) for m in messages if isinstance(m.get("content"), str))
    
    # 1. TOKEN THRESHOLD: If context gets massive, route to Heavy Logic instantly.
    if total_chars > 2500:
        return True
        
    # 2. INTEL THRESHOLD: Heavy logic parsing requires the Reasoning block.
    heavy_flags = ["architect", "design", "refactor", "theory", "evaluate", "diagnostics"]
    for m in messages:
        content = m.get("content", "").lower()
        if any(flag in content for flag in heavy_flags):
            return True
            
    # Default to Local fast processor for syntax checks, bash runs, short queries
    return False

@app.post("/v1/chat/completions")
async def proxy_completions(request: Request):
    payload = await request.json()
    messages = payload.get("messages", [])
    
    if requires_heavy_logic(messages):
        if not DEEPSEEK_API_KEY.strip():
            print("[\u26A1 ROUTER PANIC] DEEPSEEK_API_KEY environment variable is EMPTY. You must export it before launching the router!")
            return JSONResponse(status_code=500, content={"error": {"message": "Router Configuration Error: DeepSeek API Key is missing.", "type": "auth_error"}})

        print(f"[\u26A1 ROUTER] Trajectory: ORBITAL DEEPSEEK (R1 Reasoner) - Heavy Logic Detected.")
        target_url = DEEPSEEK_API_URL
        headers = {
            "Authorization": f"Bearer {DEEPSEEK_API_KEY}",
            "Content-Type": "application/json"
        }
        # Target the DeepSeek Reasoning engine
        payload["model"] = "deepseek-reasoner" 
    else:
        print(f"[\u26A1 ROUTER] Trajectory: LOCAL SENTINEL (E4B Node) - Executing via Apple Unified Memory.")
        target_url = LOCAL_E4B_URL
        headers = {
            "Content-Type": "application/json"
        }
        # Target the raw Google E4B model running on localhost:8081
        payload["model"] = "google/gemma-4-E4B-it"

    # Fire the payload via HTTPX
    async with httpx.AsyncClient(timeout=300.0) as client:
        try:
            response = await client.post(target_url, json=payload, headers=headers)
            return JSONResponse(status_code=response.status_code, content=response.json())
        except Exception as e:
            print(f"[ROUTER PANIC] Node Failed: {e}")
            return JSONResponse(status_code=500, content={
                "error": {
                    "message": f"Hardware or Network Crash: {str(e)}", 
                    "type": "server_error"
                }
            })

if __name__ == "__main__":
    import uvicorn
    # The Router stands on port 8080. This intercepts the Rust Kernel perfectly.
    uvicorn.run(app, host="127.0.0.1", port=8080)
