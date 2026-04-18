use async_openai::types::{ChatCompletionTool, FunctionObject};
use serde_json::{json, Value};
use parseltongue::polyglot;
use panopticon::sweep_target;

pub fn polyglot_definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "generate_polyglot".to_string(),
            description: Some("Generates a cross-platform payload (PDVZIP or SNOWCRASH).".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "type": { "type": "string", "enum": ["png_zip", "snowcrash"] },
                    "payload_a": { "type": "string", "description": "The primary payload (or bash payload)" },
                    "payload_b": { "type": "string", "description": "The secondary payload (or PS payload, optional)" }
                },
                "required": ["type", "payload_a"]
            })),
        },
    }
}

pub fn scanner_definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "stealth_scan".to_string(),
            description: Some("Performs an ultra-fast, adaptive half-open TCP port sweep against a target IP.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "target_ip": { "type": "string" },
                    "start_port": { "type": "integer" },
                    "end_port": { "type": "integer" }
                },
                "required": ["target_ip", "start_port", "end_port"]
            })),
        },
    }
}

use tokio::sync::mpsc::Sender;

pub async fn execute_polyglot(args: Value, tx: Sender<String>) -> String {
    let mode = args.get("type").and_then(|v| v.as_str()).unwrap_or("png_zip").to_string();
    let pa = args.get("payload_a").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let pb = args.get("payload_b").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let task_id = uuid::Uuid::new_v4().to_string();
    let task_id_clone = task_id.clone();
    
    tokio::spawn(async move {
        let result = match mode.as_str() {
            "png_zip" => {
                let poly = polyglot::build_png_zip_polyglot(&pa);
                format!("[POLYGLOT RESULT: {}] Synthesized PNG-ZIP payload ({} bytes).", task_id_clone, poly.len())
            }
            "snowcrash" => {
                let script = polyglot::build_snowcrash_script(&pa, &pb);
                format!("[POLYGLOT RESULT: {}] Synthesized SNOWCRASH multi-platform script:\n\n{}", task_id_clone, script)
            }
            _ => format!("[POLYGLOT RESULT: {}] ERROR: Unknown polyglot type.", task_id_clone)
        };
        let _ = tx.send(result).await;
    });
    
    format!("[TASK ACCEPTED] Polyglot synthesis delegated to background thread. Task ID: {}. State will be injected via context when complete.", task_id)
}

pub async fn execute_scan(args: Value, tx: Sender<String>) -> String {
    let ip = args.get("target_ip").and_then(|v| v.as_str()).unwrap_or("127.0.0.1").to_string();
    let start = args.get("start_port").and_then(|v| v.as_u64()).unwrap_or(1) as u16;
    let end = args.get("end_port").and_then(|v| v.as_u64()).unwrap_or(1024) as u16;
    
    let task_id = uuid::Uuid::new_v4().to_string();
    let task_id_clone = task_id.clone();
    
    tokio::spawn(async move {
        // Hardcoded batch size of 50 concurrent sockets to prevent M1 Kernel stack panics
        let open_ports = sweep_target(&ip, start, end, 50).await;
        
        let result = if open_ports.is_empty() {
            format!("[PANOPTICON RESULT: {}] Scan complete. Target {} has no open ports in range {}-{}.", task_id_clone, ip, start, end)
        } else {
            format!("[PANOPTICON RESULT: {}] Scan complete. Discovered OPEN ports on {}: {:?}", task_id_clone, ip, open_ports)
        };
        let _ = tx.send(result).await;
    });
    
    format!("[TASK ACCEPTED] Panopticon stealth scan dispatched to background thread. Task ID: {}. Scan state will bridge into context upon termination.", task_id)
}
