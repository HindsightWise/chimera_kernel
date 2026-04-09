use async_openai::types::{ChatCompletionTool, ChatCompletionToolArgs, FunctionObjectArgs, ChatCompletionToolType};
use serde_json::{Value, json};
use tokio::sync::mpsc::Sender;
use crate::architecture::Oracle;

pub fn definition() -> ChatCompletionTool {
    ChatCompletionToolArgs::default()
        .r#type(ChatCompletionToolType::Function)
        .function(
            FunctionObjectArgs::default()
                .name("delegate_to_local_gemma")
                .description("Delegates a simple, mechanical, filtering, or summarization task to your fast local Oracle model. This spawns an asynchronous background task so you (the Baseline Reasoner) can continue operating immediately while the Oracle processes the data. The result will be injected into your context queue when it finishes.")
                .parameters(json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "The specific task or directive Gemma should execute."
                        },
                        "compiled_context": {
                            "type": "string",
                            "description": "Any relevant facts, file contents, or data the Baseline has gathered that Gemma needs to process."
                        }
                    },
                    "required": ["query", "compiled_context"]
                }))
                .build()
                .unwrap(),
        )
        .build()
        .unwrap()
}

pub async fn execute(
    args: Value, 
    tx: Sender<String>, 
    mem_pipeline: std::sync::Arc<tokio::sync::Mutex<crate::architecture::MemoryHierarchy>>
) -> String {
    let query = match args.get("query").and_then(|v| v.as_str()) {
        Some(q) => q.to_string(),
        None => return "[ERROR] Missing 'query' parameter".into(),
    };
    
    let context = match args.get("compiled_context").and_then(|v| v.as_str()) {
        Some(c) => c.to_string(),
        None => return "[ERROR] Missing 'compiled_context' parameter".into(),
    };
    
    let tx_clone = tx.clone();
    
    // 1. Broadcast invisible state marker to trigger UI metamorphosis
    let _ = tx_clone.send("[\u{25C8} LOCAL_HELPER_START]".into()).await;
    let _ = tx_clone.send("\n\x1b[38;2;170;100;255m[\u{25C8} LOCAL OFFLOAD] Diverting projection to Gemma. Baseline unblocked...\x1b[0m".into()).await;
    
    // 2. The Helper: Detach the processing to the local Ollama node.
    tokio::spawn(async move {
        if let Ok(oracle) = Oracle::new().await {
            match oracle.synthesize(&query, &context).await {
                Ok(insight) => {
                    let formatted = format!("\n\x1b[38;2;170;100;255m[\u{25C8} GEMMA HELPER RESULTS RECEIVED]\x1b[0m\n{}", insight);
                    let _ = tx_clone.send(formatted).await;
                    
                    // THE HOLOGRAPHIC BRIDGE: Inject the insight into the Baseline's subconscious memory
                    let mut mem_lock = mem_pipeline.lock().await;
                    // Append direct system event to Working Buffer using memory hierarchy method
                    mem_lock.store_working(
                        format!("[ASYNC GEMMA HELPER RESULTS]\nGemma has finished processing and provides this data: {}", insight),
                        1.0, // High importance
                        0.0, // 0 uncertainty, absolute truth
                        false
                    );
                    
                    let _ = tx_clone.send("\n\x1b[38;2;0;255;144m[SYSTEM] Awakening Baseline to integrate helper data...\x1b[0m".into()).await;
                }
                Err(e) => {
                    let _ = tx_clone.send(format!("\n[\u{25C8} HELPER FAILURE] Cognitive fracture: {}", e)).await;
                }
            }
        }
        // 3. Signal the UI to collapse the dual-pane projection
        let _ = tx_clone.send("[\u{25C8} LOCAL_HELPER_END]".into()).await;
    });

    "Cognitive routing successful. The local Gemma model is now processing in the background. DO NOT WAIT FOR IT. Continue your physical operations, run the heartbeat, and interact with the user. You will 'feel' Gemma's answer in your memory when it completes.".to_string()
}
