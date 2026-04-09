use async_openai::types::{ChatCompletionTool, FunctionObject};
use serde_json::{json, Value};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::architecture::MemoryHierarchy;

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

pub async fn execute(args: Value, mem_pipeline: Arc<Mutex<MemoryHierarchy>>) -> String {
    let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
    
    // Phase 2: Lock the global hierarchy and update the working buffer 
    let mut memory_system = mem_pipeline.lock().await;
    memory_system.store_working(query.to_string(), 0.8, 0.5, false);
    
    // Phase 3a: Native Deep Storage Query
    let mut approach = "NATIVE-RUST";
    let memory_results = if let Some(db) = &memory_system.db_connection {
        let encoded = crate::architecture::MemoryHierarchy::encode_spectral_embedding(&query);
        match db.search_vector(encoded, 3) {
            Ok(res_str) => res_str,
            Err(e) => {
                approach = "DUMMY-FALLBACK (Search Error)";
                format!("Failed to search native memory: {}", e)
            }
        }
    } else {
        approach = "DUMMY-FALLBACK (Storage Controller Offline)";
        generate_dummy_fallback(query)
    };
    drop(memory_system);
    
    format!("[MNEMOSYNE SUBSTRATE RECALL]\nArchitecture: {}\nQuery: '{}'\n\n[HISTORICAL FRAGMENTS]\n{}", 
            approach, query, memory_results)
}

fn generate_dummy_fallback(text: &str) -> String {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    let hash = hasher.finish();
    
    format!("Dummy Local Hash: {:x}", hash)
}
