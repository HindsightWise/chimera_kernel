use async_openai::types::{ChatCompletionTool, FunctionObject};
use serde_json::{json, Value};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::architecture::{MemoryHierarchy, IPCBridge};

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "mnemosyne_subconscious_recall".to_string(),
            description: Some("Query the deep-storage Soul System (Mnemosyne) to unearth archived context or entity blueprints that were compressed during context sliding.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "The esoteric keyword or topic to dredge from the Subconscious Compression Engine." }
                },
                "required": ["query"]
            })),
        },
    }
}

pub async fn execute(args: Value, mem_pipeline: Arc<Mutex<MemoryHierarchy>>, ipc_bridge: Option<IPCBridge>) -> String {
    let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
    
    // Phase 2: Lock the global hierarchy and update the working buffer 
    let mut memory_system = mem_pipeline.lock().await;
    memory_system.store_working(query.to_string(), 0.8, 0.5, false);
    drop(memory_system);
    
    // Phase 3a: Attempt True Python IPC embedding
    let mut approach = "IPC-NATIVE";
    let memory_results = if let Some(bridge) = ipc_bridge {
        let payload = serde_json::json!({
            "command": "RECALL",
            "query": query,
            "limit": 3
        }).to_string();
        
        match bridge.dispatch_ipc(payload).await {
            Some(res_str) => {
                if let Ok(val) = serde_json::from_str::<Value>(&res_str) {
                    if val.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
                        if let Some(arr) = val.get("results").and_then(|v| v.as_array()) {
                            let results_text: Vec<String> = arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect();
                            if results_text.is_empty() {
                                "No relevant subconscious structures found.".to_string()
                            } else {
                                results_text.join("\n\n---\n\n")
                            }
                        } else {
                            "[ERROR] Malformed RECALL array".to_string()
                        }
                    } else {
                        approach = "DUMMY-FALLBACK (IPC Recall Failed)";
                        generate_dummy_fallback(query)
                    }
                } else {
                    approach = "DUMMY-FALLBACK (IPC Parse Failed)";
                    generate_dummy_fallback(query)
                }
            },
            None => {
                approach = "DUMMY-FALLBACK";
                generate_dummy_fallback(query)
            }
        }
    } else {
        approach = "DUMMY-FALLBACK (No IPC Spawned)";
        generate_dummy_fallback(query)
    };
    
    format!("[MNEMOSYNE PROTOTYPE RECALL]\nArchitecture: {}\nQuery: '{}'\n\n[HISTORICAL FRAGMENTS]\n{}", 
            approach, query, memory_results)
}

fn generate_dummy_fallback(text: &str) -> String {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    let hash = hasher.finish();
    
    format!("Dummy Local Hash: {:x}", hash)
}
