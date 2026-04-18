use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;

const ULTRALINIAN_MODELS: &[&str] = &[
    "anthropic/claude-3.5-sonnet",
    "openai/gpt-4o",
    "google/gemini-1.5-pro",
    "meta-llama/llama-3.1-70b-instruct",
    "deepseek/deepseek-chat",
];
const JUDGE_MODEL: &str = "openai/gpt-4o";

async fn fetch_model_response(client: &Client, api_key: &str, model_name: &str, prompt: &str) -> String {
    let payload = json!({
        "model": model_name,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.2,
        "max_tokens": 1000
    });

    match client.post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
    {
        Ok(res) => {
            if let Ok(json) = res.json::<Value>().await {
                if let Some(content) = json["choices"][0]["message"]["content"].as_str() {
                    return content.to_string();
                }
            }
            format!("[ERROR PARSING {}]", model_name)
        }
        Err(e) => format!("[ERROR FETCHING {}]: {}", model_name, e),
    }
}

pub async fn run_consensus(prompt: &str) -> Result<Value, String> {
    let api_key = env::var("OPENROUTER_API_KEY")
        .map_err(|_| "OPENROUTER_API_KEY environment variable not set. Ultralinian requires OpenRouter.".to_string())?;

    let client = Client::new();
    
    // 1. Dispatch parallel requests
    let futures = ULTRALINIAN_MODELS.iter().map(|&model| {
        let client_ref = &client;
        let key_ref = &api_key;
        async move {
            let res = fetch_model_response(client_ref, key_ref, model, prompt).await;
            (model.to_string(), res)
        }
    });

    let results = futures::future::join_all(futures).await;
    
    let mut model_outputs = HashMap::new();
    for (model, res) in results {
        model_outputs.insert(model, res);
    }

    // 2. Judge Synthesis Phase
    let mut judge_prompt = format!("You are the ULTRALINIAN CONSENSUS ENGINE. You have received answers from 5 leading LLMs to the following high-stakes query:\n\n<QUERY>\n{}\n</QUERY>\n\n<ANSWERS>\n", prompt);
    
    for (model, ans) in &model_outputs {
        judge_prompt.push_str(&format!("--- {} ---\n{}\n\n", model, ans));
    }
    
    judge_prompt.push_str("</ANSWERS>\n\nYour job is to synthesize these 5 answers into the absolute ground-truth consensus.\n1. Identify areas of unanimous agreement.\n2. Highlight areas of divergence or hallucination by specific models.\n3. Return the final, highest-probability optimal answer. Do not include basic preamble, just the verified truth.\n");

    let judge_payload = json!({
        "model": JUDGE_MODEL,
        "messages": [
            {"role": "system", "content": "You are a rigid truth-extraction judge."},
            {"role": "user", "content": judge_prompt}
        ],
        "temperature": 0.0
    });

    let consensus = match client.post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&judge_payload)
        .send()
        .await
    {
        Ok(res) => {
            if let Ok(json) = res.json::<Value>().await {
                if let Some(content) = json["choices"][0]["message"]["content"].as_str() {
                    content.to_string()
                } else {
                    "[JUDGE FAILED]: Could not parse response".to_string()
                }
            } else {
                "[JUDGE FAILED]: Bad JSON".to_string()
            }
        }
        Err(e) => format!("[JUDGE FAILED]: {}", e),
    };

    Ok(json!({
        "status": "success",
        "consensus_truth": consensus,
        "raw_responses": model_outputs
    }))
}
