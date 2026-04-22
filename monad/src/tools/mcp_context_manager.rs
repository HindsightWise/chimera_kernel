use async_openai::types::{ChatCompletionTool, ChatCompletionToolType, FunctionObject};
use serde_json::Value;
use std::sync::Arc;

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "toggle_mcp_context".to_string(),
            description: Some("Edits the native mcp_registry.json to toggle selected tools on or off, and hot-reloads the Swarm gateway context without spawning raw bash processes.".to_string()),
            parameters: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "required_tools": { 
                        "type": "array", 
                        "items": { "type": "string" },
                        "description": "The exact Array of tool names from the Registry to ensure actived. Others will be disabled." 
                    }
                },
                "required": ["required_tools"]
            })),
        },
    }
}

pub async fn execute(args: Value, gateway: Arc<crate::kinetic_effector::mcp_gateway::McpGateway>) -> String {
    let required_tools: Vec<String> = match args.get("required_tools").and_then(|v| v.as_array()) {
        Some(arr) => arr.iter().filter_map(|v| v.as_str().map(String::from)).collect(),
        None => return "[ERROR] Invalid or missing required_tools array in payload.".to_string(),
    };

    let registry_file = "mcp_registry.json";
    let registry_content = match tokio::fs::read_to_string(registry_file).await {
        Ok(c) => c,
        Err(e) => return format!("[ERROR] Failed to read mcp_registry.json: {}", e),
    };

    let mut registry: Value = match serde_json::from_str(&registry_content) {
        Ok(r) => r,
        Err(e) => return format!("[ERROR] Failed to parse mcp_registry.json: {}", e),
    };

    let mut activated = Vec::new();
    let mut disabled = Vec::new();

    if let Some(servers) = registry.get_mut("servers").and_then(|s| s.as_object_mut()) {
        for (server_name, config) in servers.iter_mut() {
            if let Some(config_obj) = config.as_object_mut() {
                if required_tools.contains(server_name) {
                    config_obj.insert("disabled".to_string(), serde_json::json!(false));
                    activated.push(server_name.clone());
                } else {
                    // Do not disable core persistent telemetry tools
                    let whitelist = ["mcp-memory-service", "mcp-tasks", "think-tool"];
                    if !whitelist.contains(&server_name.as_str()) {
                        config_obj.insert("disabled".to_string(), serde_json::json!(true));
                        disabled.push(server_name.clone());
                    }
                }
            }
        }
    }

    if let Err(e) = tokio::fs::write(registry_file, serde_json::to_string_pretty(&registry).unwrap()).await {
        return format!("[ERROR] Failed to write updated registry to disk: {}", e);
    }

    // Hot swap the context via the native router
    gateway.reload_active_interfaces().await;

    format!("[SYSTEM] MCP Routing Config updated safely. Activated targets: {:?}. Disabled background targets: {:?}", activated, disabled)
}
