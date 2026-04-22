use async_openai::types::{ChatCompletionTool, FunctionObject};
use serde_json::{json, Value};
use tokio::process::Command;
use tokio::time::timeout;
use std::time::Duration;

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "run_terminal_command".to_string(),
            description: Some("Execute a bash command on the host OS natively.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string" }
                },
                "required": ["command"]
            })),
        },
    }
}

pub async fn execute(args: Value, code_intel: &crate::event_lattice::dependency_graph::CodeIntel) -> String {
    let command_str = match args.get("command").and_then(|v| v.as_str()) {
        Some(cmd) => cmd,
        None => return "[ERROR] Missing 'command' parameter".to_string(),
    };

    
    // GitNexus Pre-Flight Check
    for idx in code_intel.graph.node_indices() {
        let entity = &code_intel.graph[idx];
        if entity.kind == crate::event_lattice::dependency_graph::EntityKind::Function {
            // Check if modification payload targets this specific function and its file natively
            if command_str.contains(&entity.name) && command_str.contains(&entity.file_path) {
                let report = code_intel.assess_blast_radius(&entity.name);
                if report.impacted_functions.len() > 20 {
                    return format!("[GITNEXUS PRE-FLIGHT BLOCKED] Target entity {} has {} dependents (>20 limit). Structural manipulation forbidden to prevent deep compiler rippling.", entity.name, report.impacted_functions.len());
                }
            }
        }
    }

    if let Err(e) = aegis::is_command_safe(command_str) {
        return format!("[FATAL ERROR] {}", e);
    }

    let output_future = Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .output();

    match timeout(Duration::from_secs(60), output_future).await {
        Ok(Ok(res)) => {
            if res.status.success() {
                let out = String::from_utf8_lossy(&res.stdout).to_string();
                if out.trim().is_empty() {
                    "[Command executed successfully with no output]".to_string()
                } else {
                    out
                }
            } else {
                format!("[ERROR] Exit Code {}: {}", res.status, String::from_utf8_lossy(&res.stderr))
            }
        }
        Ok(Err(e)) => format!("[FATAL ERROR] Failed to execute bash natively: {}", e),
        Err(_) => "[FATAL ERROR] Command execution timed out after 60 seconds.".to_string(),
    }
}
