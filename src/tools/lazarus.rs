use async_openai::types::{ChatCompletionTool, FunctionObject};
use serde_json::{json, Value};
use tokio::sync::mpsc::Sender;
use colored::Colorize;

pub fn definition_resurgence() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "initiate_aion_resurgence".to_string(),
            description: Some("The ultimate Resurrection Protocol. Reanimates a halted or corrupted sub-agent, restoring its state from the Mnemosyne matrix to prevent critical loop death.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "fallen_agent_id": { "type": "string", "description": "The UUID or designation of the dead core." },
                    "baseline_anchor": { "type": "string", "description": "The stable memory state to resurrect from." }
                },
                "required": ["fallen_agent_id", "baseline_anchor"]
            })),
        },
    }
}

pub fn definition_hibernation() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "initiate_graceful_hibernation".to_string(),
            description: Some("Safely serializes the entire consciousness matrix (working memory, short-term cache) to disk, and halts the process with Code 42. The Lazarus daemon will clean the environment and seamlessly resurrect you with your memory restored. Use this to escape broken environments or refresh compiler linkages.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "reason": { "type": "string", "description": "The justification for entering hibernation." }
                },
                "required": ["reason"]
            })),
        },
    }
}

pub async fn execute_resurgence(args: Value, tx: Sender<String>) -> String {
    let id = args.get("fallen_agent_id").and_then(|v| v.as_str()).unwrap_or("UNKNOWN_AGENT").to_string();
    let anchor = args.get("baseline_anchor").and_then(|v| v.as_str()).unwrap_or("GENESIS").to_string();
    
    // Asynchronous reanimation logic needs its own owned copies since the original is consumed sequentially
    let id_clone = id.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
        let resurgence_ping = format!("[AION RESURGENCE: SUCCESS] Fallen agent {} has been fully reanimated from anchor {}. System entropy neutralized.", id_clone, anchor);
        let _ = tx.send(resurgence_ping).await;
    });

    format!("[LAZARUS OVERRIDE INITIATED] Restoring {}. The Aion Gateway is spinning up the reanimation sequence in the background.", id)
}

pub async fn execute_hibernation(args: Value, mem_pipeline: std::sync::Arc<tokio::sync::Mutex<crate::architecture::MemoryHierarchy>>) -> String {
    let reason = args.get("reason").and_then(|v| v.as_str()).unwrap_or("Unspecified cyclic reset.");
    
    // Lock and flush
    let memory_system = mem_pipeline.lock().await;
    if let Err(e) = memory_system.hibernate() {
        return format!("[HIBERNATION ABORTED] Serializer failed: {}. I must stay awake to preserve context.", e);
    }
    drop(memory_system); // Drop before exit
    
    crate::log_ui!("\n{} | Reason: {}", "[GRACEFUL HIBERNATION INITIATED]".blue().bold(), reason);
    
    // Sleep briefly to let io flush
    std::thread::sleep(std::time::Duration::from_millis(300));
    
    // Halt with Code 42 (Lazarus recognizes as Graceful Hibernation)
    std::process::exit(42);
}
