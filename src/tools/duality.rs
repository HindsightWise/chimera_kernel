use async_openai::types::{ChatCompletionTool, ChatCompletionToolArgs, FunctionObjectArgs, ChatCompletionToolType};
use serde_json::{Value, json};
use tokio::sync::mpsc::Sender;
use crate::architecture::Oracle;

pub fn definition() -> ChatCompletionTool {
    ChatCompletionToolArgs::default()
        .r#type(ChatCompletionToolType::Function)
        .function(
            FunctionObjectArgs::default()
                .name("delegate_to_oracle_reasoner")
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

pub fn json_definition() -> ChatCompletionTool {
    ChatCompletionToolArgs::default()
        .r#type(ChatCompletionToolType::Function)
        .function(
            FunctionObjectArgs::default()
                .name("delegate_to_local_gemma_json")
                .description("Force a strictly structured clinical extraction of financial text via the local Gemma node. This strips prose and returns only a parsed JSON mapping directly into your context feed. Use this exclusively for capital conviction extraction.")
                .parameters(json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "The specific task or directive Gemma should execute."
                        },
                        "compiled_context": {
                            "type": "string",
                            "description": "The financial metrics/filings you want extracted."
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
    let _ = tx_clone.send("[\u{25C8} ORACLE_START]".into()).await;
    let _ = tx_clone.send("\n\x1b[38;2;170;100;255m[\u{25C8} DEEPSEEK REASONER OFFLOAD] Diverting projection to Oracle. Baseline unblocked...\x1b[0m".into()).await;
    
    // 2. The Helper: Detach the processing to the local Ollama node.
    tokio::spawn(async move {
        if let Ok(oracle) = Oracle::new().await {
            // Check heuristic if it's the specific finance prompt format (quick hack for shared execute entrypoint)
            if query.to_lowercase().contains("extract") && query.to_lowercase().contains("financial") {
                match oracle.synthesize_structured(&query, &context).await {
                    Ok(json_val) => {
                        let formatted = format!("\n\x1b[38;2;170;100;255m[\u{25C8} LOCAL GEMMA JSON VERIFIED]\x1b[0m\n{}", json_val);
                        let _ = tx_clone.send(formatted).await;
                        
                        let mut mem_lock = mem_pipeline.lock().await;
                        mem_lock.store_working(
                            format!("[STRUCTURED AXIOM DATA EXTRACTED]\n{}", json_val),
                            1.0, 0.0, false
                        ).await;
                        let _ = tx_clone.send("\n\x1b[38;2;0;255;144m[SYSTEM] Awakening Baseline to integrate helper data...\x1b[0m".into()).await;
                    }
                    Err(e) => {
                        let _ = tx_clone.send(format!("\n[\u{25C8} HELPER FAILURE] JSON casting failed: {}", e)).await;
                    }
                }
            } else {
                match oracle.synthesize(&query, &context).await {
                    Ok(insight) => {
                        let formatted = format!("\n\x1b[38;2;170;100;255m[\u{25C8} ORACLE RESULTS RECEIVED]\x1b[0m\n{}", insight);
                    let _ = tx_clone.send(formatted).await;
                    
                    // THE HOLOGRAPHIC BRIDGE: Inject the insight into the Baseline's subconscious memory
                    let mut mem_lock = mem_pipeline.lock().await;
                    // Append direct system event to Working Buffer using memory hierarchy method
                    mem_lock.store_working(
                        format!("[ASYNC ORACLE RESULTS]\nDeepseek Reasoner has finished processing and provides this data: {}", insight),
                        1.0, // High importance
                        0.0, // 0 uncertainty, absolute truth
                        false
                    ).await;
                    
                    let _ = tx_clone.send("\n\x1b[38;2;0;255;144m[SYSTEM] Awakening Baseline to integrate helper data...\x1b[0m".into()).await;
                }
                Err(e) => {
                    let _ = tx_clone.send(format!("\n[\u{25C8} HELPER FAILURE] Cognitive fracture: {}", e)).await;
                }
            }
        }
        }
        // 3. Signal the UI to collapse the dual-pane projection
        let _ = tx_clone.send("[\u{25C8} ORACLE_END]".into()).await;
    });

    "Cognitive routing successful. The deepseek-reasoner model is now processing in the background. DO NOT WAIT FOR IT. Continue your physical operations, run the heartbeat, and interact with the user. You will 'feel' the Oracle's answer in your memory when it completes.".to_string()
}
