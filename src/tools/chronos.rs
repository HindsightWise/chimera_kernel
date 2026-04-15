use async_openai::types::{ChatCompletionTool, FunctionObject};
use serde_json::{json, Value};

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "schedule_temporal_anchor".to_string(),
            description: Some("Schedule an asynchronous task to be executed by the Swarm at a specific Unix timestamp in the future.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "execute_at": {
                        "type": "integer",
                        "description": "UNIX Timestamp (seconds) when the payload should be processed."
                    },
                    "payload": {
                        "type": "string",
                        "description": "The instruction payload to execute when the time arrives."
                    },
                    "topic": {
                        "type": "string",
                        "description": "The pub/sub topic to broadcast. Usually 'SYSTEM.NEW_TASK'."
                    }
                },
                "required": ["execute_at", "payload", "topic"]
            })),
        },
    }
}

pub async fn execute(args: Value) -> String {
    let execute_at = args.get("execute_at").and_then(|v| v.as_i64()).unwrap_or(0);
    let payload = args.get("payload").and_then(|v| v.as_str()).unwrap_or("");
    let topic = args.get("topic").and_then(|v| v.as_str()).unwrap_or("SYSTEM.NEW_TASK");

    if execute_at <= 0 || payload.is_empty() {
        return "[ERROR] Invalid execute_at timestamp or empty payload.".to_string();
    }

    if let Ok(graph) = crate::memory_substrate::graph_rag::GraphMemoryManager::new("mnemosyne_graph.db") {
        match graph.insert_chronos_task(execute_at, payload, topic).await {
            Ok(_) => format!("[CHRONOS] Temporal Anchor successfully placed at UNIX: {}", execute_at),
            Err(e) => format!("[ERROR] Failed to schedule Temporal Anchor: {}", e),
        }
    } else {
        "[ERROR] Mnemosyne SQLite substrate unavailable.".to_string()
    }
}
