use async_openai::types::{ChatCompletionTool, ChatCompletionToolType, FunctionObject};
use serde_json::Value;

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "update_plan".to_string(),
            description: Some("Registers your sequential workflow tasks into a durable local JSON file to prevent context amnesia. Overwrites the previous state. Always use this instead of Markdown checklists in the chat output.".to_string()),
            parameters: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "objective": { "type": "string", "description": "High level macro goal (WBS Level 1)." },
                    "completed_nodes": { "type": "array", "items": { "type": "string" }, "description": "Bullet points mapping specifically what tasks have already definitively succeeded." },
                    "active_node": { "type": "string", "description": "The exact discrete task you are currently performing." },
                    "pending_nodes": { "type": "array", "items": { "type": "string" }, "description": "Future steps left to execute in order." }
                },
                "required": ["objective", "completed_nodes", "active_node", "pending_nodes"]
            })),
        },
    }
}

pub async fn execute(args: Value) -> String {
    let objective = args.get("objective").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let active_node = args.get("active_node").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let completed_nodes = args.get("completed_nodes").cloned().unwrap_or(serde_json::json!([]));
    let pending_nodes = args.get("pending_nodes").cloned().unwrap_or(serde_json::json!([]));

    let state_obj = serde_json::json!({
        "objective": objective,
        "completed_nodes": completed_nodes,
        "active_node": active_node,
        "pending_nodes": pending_nodes,
        "last_updated": chrono::Utc::now().to_rfc3339()
    });

    if let Err(e) = tokio::fs::write("ACTIVE_STATE.json", serde_json::to_string_pretty(&state_obj).unwrap()).await {
        return format!("[ERROR] Failed to save Durable State: {}", e);
    }

    crate::log_ui!("{}", format!("[SYSTEM] Durable State recorded. Action tracked: {}", active_node));
    format!("[SYSTEM] State seamlessly recorded to ACTIVE_STATE.json. Objective: {}. You are protected from amnesia.", objective)
}
