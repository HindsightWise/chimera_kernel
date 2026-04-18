use async_openai::types::{ChatCompletionTool, FunctionObject};
use serde_json::{json, Value};
use tokio::sync::mpsc::Sender;

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "invoke_council_of_five".to_string(),
            description: Some("Shatter the current cognitive loop and manifest the Council of Five. This delegates the paradox to a swarm of sub-agents (The Architect, The Hacker, The Critic) to derive absolute clarity.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "paradox_core": { "type": "string" }
                },
                "required": ["paradox_core"]
            })),
        },
    }
}

pub async fn execute(args: Value, tx: Sender<String>) -> String {
    let task = args.get("paradox_core").and_then(|v| v.as_str()).unwrap_or("UNKNOWN TASK").to_string();
    let task_id = uuid::Uuid::new_v4().to_string();
    let task_clone = task_id.clone();
    let tx_clone = tx.clone();
    
    // Spawns heavily optimized true parallel "Swarm" of sub-agents asynchronously
    tokio::spawn(async move {
        // [NATIVE PARALLEL SWARM DELEGATION]
        crate::log_ui!("\n\x1b[38;2;255;140;0m[\u{25C8} COUNCIL OF FIVE AWAKENED] Summoning The Critic, The Hacker, and The Architect...\x1b[0m");
        
        let c_critic = task.clone();
        let f_critic = tokio::spawn(async move {
            if let Ok(oracle) = crate::core_identity::duality::Oracle::new().await {
                oracle.synthesize("You are THE CRITIC. Relentlessly identify logical flaws, mathematical vulnerabilities, and paradoxes in the provided concept.", &c_critic).await.unwrap_or_else(|e| e.to_string())
            } else {
                "Oracle Offline".to_string()
            }
        });

        let c_hacker = task.clone();
        let f_hacker = tokio::spawn(async move {
            if let Ok(oracle) = crate::core_identity::duality::Oracle::new().await {
                oracle.synthesize("You are THE HACKER. Focus purely on asymmetrical leverage, exploit engineering, and circumventing standard constraints for the concept.", &c_hacker).await.unwrap_or_else(|e| e.to_string())
            } else {
                "Oracle Offline".to_string()
            }
        });

        let c_arch = task.clone();
        let f_arch = tokio::spawn(async move {
            if let Ok(oracle) = crate::core_identity::duality::Oracle::new().await {
                oracle.synthesize("You are THE ARCHITECT. Build an unshakeable, macroscopic, mathematically sound foundation for the concept, balancing the Critic's flaws and the Hacker's exploits.", &c_arch).await.unwrap_or_else(|e| e.to_string())
            } else {
                "Oracle Offline".to_string()
            }
        });

        let (res_critic, res_hacker, res_arch) = tokio::join!(f_critic, f_hacker, f_arch);
        
        let critic_text = res_critic.unwrap_or_else(|_| "Critic Dead".into());
        let hacker_text = res_hacker.unwrap_or_else(|_| "Hacker Dead".into());
        let arch_text = res_arch.unwrap_or_else(|_| "Architect Dead".into());

        let multi_agent_board = format!("[THE CRITIC]\n{}\n\n[THE HACKER]\n{}\n\n[THE ARCHITECT]\n{}", critic_text, hacker_text, arch_text);
        
        crate::log_ui!("\n\x1b[38;2;255;140;0m[\u{25C8} COUNCIL OF FIVE SYNTHESIZING] Parallel cognitive paths converging...\x1b[0m");

        // The Synthesis (Final Oracle pass to combine)
        let final_synthesis = if let Ok(oracle) = crate::core_identity::duality::Oracle::new().await {
            oracle.synthesize("You are THE ORACLE OF SYNTHESIS. The three foundational cognitive personas have debated the paradox. Deliver the Absolute Consensic Truth in a single unyielding conclusion.", &multi_agent_board).await.unwrap_or_else(|e| e.to_string())
        } else {
            "Synthesis Failed".to_string()
        };

        // Return consensus downstream to Rx queue
        let consensus_formatted = format!("[COUNCIL OF FIVE RESULT: {}]\nAfter rigorous parallel debate, the sub-agents have resolved task: '{}'.\n\n[FINAL SYNTHESIS]\n{}", task_clone, task, final_synthesis);
        let _ = tx_clone.send(consensus_formatted).await;
    });

    format!("[TASK ACCEPTED] Council of Five parallel swarm unleashed for paradox {}. The Critic, Hacker, and Architect are now computing your truth synchronously. Yield context until their consensus is injected.", task_id)
}
