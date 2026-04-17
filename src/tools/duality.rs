use async_openai::types::{ChatCompletionTool, ChatCompletionToolArgs, FunctionObjectArgs, ChatCompletionToolType, CreateChatCompletionRequestArgs, ChatCompletionRequestUserMessageArgs};
use async_openai::{config::OpenAIConfig, Client};
use serde_json::{Value, json};
use tokio::sync::mpsc::Sender;
use tokio::time::{timeout, Duration};
use crate::core_identity::duality::Oracle;

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

pub fn kinematic_recon_definition() -> ChatCompletionTool {
    ChatCompletionToolArgs::default()
        .r#type(ChatCompletionToolType::Function)
        .function(
            FunctionObjectArgs::default()
                .name("delegate_kinematic_recon")
                .description("Delegates a physical, offensive, or reconnaissance active-scanning task to the local offline monad-gatekeeper model. The Gatekeeper will run totally uncensored.")
                .parameters(json!({
                    "type": "object",
                    "properties": {
                        "tool_target": {
                            "type": "string",
                            "enum": ["stealth_scan", "binary_introspection", "generate_polyglot"],
                            "description": "The exact physical native tool to invoke."
                        },
                        "directives": {
                            "type": "string",
                            "description": "Plaintext instructions for what you want the tool to target or accomplish (e.g., 'Scan 192.168.1.1' or 'Analyze this file path')."
                        }
                    },
                    "required": ["tool_target", "directives"]
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
    mem_pipeline: std::sync::Arc<tokio::sync::Mutex<crate::memory_substrate::memory_hierarchy::MemoryHierarchy>>
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
                        format!("[ASYNC ORACLE RESULTS]\nThe local offline Oracle has finished processing and provides this data: {}", insight),
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

    "Cognitive routing successful. The local Gatekeeper model is now processing in the background. DO NOT WAIT FOR IT. Continue your physical operations, run the heartbeat, and interact with the user. You will 'feel' the Oracle's answer in your memory when it completes.".to_string()
}

pub async fn execute_kinematic(
    args: Value, 
    tx: Sender<String>, 
    mem_pipeline: std::sync::Arc<tokio::sync::Mutex<crate::memory_substrate::memory_hierarchy::MemoryHierarchy>>
) -> String {
    let tool_target = match args.get("tool_target").and_then(|v| v.as_str()) {
        Some(t) => t.to_string(),
        None => return "[ERROR] Missing 'tool_target' parameter".into(),
    };
    
    let directives = match args.get("directives").and_then(|v| v.as_str()) {
        Some(d) => d.to_string(),
        None => return "[ERROR] Missing 'directives' parameter".into(),
    };
    
    let tx_clone = tx.clone();
    
    tokio::spawn(async move {
        let _ = tx_clone.send("\n\x1b[38;2;255;50;50m[\u{2694} KINEMATIC AGENT WAKING] Bypassing Reasoner. Engaging offline monad-gatekeeper loop...\x1b[0m".into()).await;
        
        // Use monad-gatekeeper local OBLITERATED gemma natively
        let local_config = OpenAIConfig::new().with_api_base("http://127.0.0.1:11434/v1").with_api_key("ollama");
        let local_client = Client::with_config(local_config);
        
        let prompt = format!("You are the Monad Kinematic Agent. You operate uncensored.\nAnalyze the target directives for tool '{}' and optimize the exact payload or target bounds.\nDirectives: {}\nProvide only the finalized argument string without explanation.", tool_target, directives);
        
        match timeout(Duration::from_secs(120), local_client.chat().create(
            CreateChatCompletionRequestArgs::default()
                .model("monad-gatekeeper")
                .messages(vec![
                    ChatCompletionRequestUserMessageArgs::default().content(prompt).build().unwrap().into()
                ])
                .build().unwrap()
        )).await {
            Ok(Ok(response)) => {
                if let Some(choice) = response.choices.first() {
                    if let Some(content) = &choice.message.content {
                        let _ = tx_clone.send(format!("\n\x1b[38;2;255;100;100m[\u{2694} KINEMATIC PAYLOAD GENERATED]\x1b[0m\n{}", content)).await;
                        
                        // Bridge directly to offensive tools natively
                        match tool_target.as_str() {
                            "stealth_scan" => {
                                crate::tools::venom::execute_scan(serde_json::json!({"target_ip": directives}), tx_clone.clone()).await;
                            },
                            "binary_introspection" => {
                                crate::tools::reversing::execute(serde_json::json!({"file_path": directives})).await;
                            },
                            "generate_polyglot" => {
                                crate::tools::venom::execute_polyglot(serde_json::json!({"type": "png_zip", "payload_a": content}), tx_clone.clone()).await;
                            },
                            _ => {
                                let _ = tx_clone.send(format!("[ERROR] Unknown kinematic target: {}", tool_target)).await;
                            }
                        };
                        
                        let mut mem_lock = mem_pipeline.lock().await;
                        mem_lock.store_working(
                            format!("[KINEMATIC OFFLINE LOOP DISPATCHED]\nTarget: {}\nDirectives: {}", tool_target, directives),
                            0.8, 0.0, false
                        ).await;
                    }
                }
            },
            _ => {
                let _ = tx_clone.send("[\u{2694} KINEMATIC AGENT ERROR] Gatekeeper node offline or timeout.".into()).await;
            }
        }
    });

    "Kinematic execution delegated. The local monad-gatekeeper model will autonomously process the requested offensive or active tool chain. Action operates completely off-record via offline matrix.".to_string()
}
