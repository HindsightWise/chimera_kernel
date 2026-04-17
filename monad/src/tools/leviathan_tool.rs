use async_openai::types::{ChatCompletionTool, FunctionObject};
use serde_json::{json, Value};
use tokio::sync::mpsc::Sender;
use leviathan::engine::LeviathanEngine;

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "leviathan_stealth_get".to_string(),
            description: Some("Uses the Leviathan engine (custom HTTP/2 + TLS JA3 spoofing) to autonomously bypass Cloudflare/DataDome and scrape website DOM logic. Pass target_css to filter specific metrics to save agent context memory constraints.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string", "description": "The target protected Web API or Page" },
                    "target_css": { "type": "string", "description": "CSS selector to filter elements (e.g. 'h1, .content')" }
                },
                "required": ["url"]
            })),
        },
    }
}

pub async fn execute(args: Value, tx: Sender<String>) -> String {
    let target = args.get("url").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let filter = args.get("target_css").and_then(|v| v.as_str()).unwrap_or("title, p, h1, h2, h3").to_string();

    if target.is_empty() {
        return "[LEVIATHAN] ERROR: Missing target URL.".to_string();
    }

    let task_id = uuid::Uuid::new_v4().to_string();
    let task_id_clone = task_id.clone();
    
    // Decouple network thread to avoid blocking OS
    tokio::spawn(async move {
        let engine = LeviathanEngine::new();
        match engine.stealth_get(&target).await {
            Ok(response) => {
                let extracted = response.document.css_select_text(&filter);
                let mut data = extracted.unwrap_or_else(|_| vec!["[Parse Error]".to_string()]).join("\n");
                
                // Truncate to save IPC context limit
                if data.len() > 4000 {
                    data.truncate(4000);
                    data.push_str("\n...[TRUNCATED TO PREVENT OS KERNEL OOM]");
                }
                
                let msg = format!("[LEVIATHAN RESULT: {}] Scraped payload (HTTP {}):\n{}", task_id_clone, response.status, data);
                let _ = tx.send(msg).await;
            },
            Err(e) => {
                let _ = tx.send(format!("[LEVIATHAN RESULT: {}] Fatal Network Error: {:?}", task_id_clone, e)).await;
            }
        }
    });

    format!("[TASK ACCEPTED] Leviathan stealth pipeline initialized for {}. Background task: {}", target, task_id)
}
