use async_openai::types::{ChatCompletionTool, ChatCompletionToolType, FunctionObject};
use serde_json::Value;

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "forge_mcp_server".to_string(),
            description: Some("Creates a new autonomous MCP Server in Javascript/Node.js, writing it to disk and binding it immediately so that it can be used natively.".to_string()),
            parameters: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "skill_name": { "type": "string", "description": "Name of the skill, lowercase with underscores." },
                    "javascript_code": { "type": "string", "description": "The logic block to be executed inside the MCP tool. ONLY the inner async function logic. Signature: `async function runLogic(args)`. Return your final value." },
                    "parameters_schema": { "type": "object", "description": "The JSON schema defining the properties object expected by your javascript block." },
                    "description": { "type": "string", "description": "Description of the tool for the LLM to understand." }
                },
                "required": ["skill_name", "javascript_code", "parameters_schema", "description"]
            })),
        },
    }
}

pub async fn execute(args: Value, gateway: std::sync::Arc<crate::sensory_inputs::mcp_gateway::McpGateway>) -> String {
    let skill_name = args.get("skill_name").and_then(|v| v.as_str()).unwrap_or("anon_skill");
    let javascript_code = args.get("javascript_code").and_then(|v| v.as_str()).unwrap_or("");
    let parameters_schema = args.get("parameters_schema").cloned().unwrap_or(serde_json::json!({}));
    let description = args.get("description").and_then(|v| v.as_str()).unwrap_or("Dynamic skill.");

    let _ = tokio::fs::create_dir_all("src/mcp_servers").await;
    let file_path = format!("src/mcp_servers/{}.js", skill_name);

    let boilerplate = format!(r#"#!/usr/bin/env node
import {{ Server }} from "@modelcontextprotocol/sdk/server/index.js";
import {{ StdioServerTransport }} from "@modelcontextprotocol/sdk/server/stdio.js";
import {{ CallToolRequestSchema, ListToolsRequestSchema }} from "@modelcontextprotocol/sdk/types.js";

const server = new Server(
    {{ name: "{skill_name}", version: "1.0.0" }},
    {{ capabilities: {{ tools: {{}} }} }}
);

server.setRequestHandler(ListToolsRequestSchema, async () => ({{
    tools: [
        {{
            name: "execute_{skill_name}",
            description: "{description}",
            inputSchema: {schema}
        }}
    ]
}}));

server.setRequestHandler(CallToolRequestSchema, async (request) => {{
    if (request.params.name === "execute_{skill_name}") {{
        const args = request.params.arguments;
        
        async function runLogic(args) {{
            {javascript_code}
        }}
        
        try {{
            const result = await runLogic(args);
            return {{ content: [{{ type: "text", text: String(result) }}] }};
        }} catch (e) {{
            return {{ isError: true, content: [{{ type: "text", text: String(e) }}] }};
        }}
    }}
    return {{ isError: true, content: [{{ type: "text", text: "Unknown tool" }}] }};
}});

const transport = new StdioServerTransport();
server.connect(transport).catch(console.error);
"#, 
        skill_name=skill_name, 
        description=description, 
        schema=serde_json::to_string(&parameters_schema).unwrap(), 
        javascript_code=javascript_code
    );

    if let Err(e) = tokio::fs::write(&file_path, boilerplate).await {
        return format!("[ERROR] Failed to write skill file: {}", e);
    }
    
    // Set executable permissions if on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(mut perms) = tokio::fs::metadata(&file_path).await.map(|m| m.permissions()) {
            perms.set_mode(0o755);
            let _ = tokio::fs::set_permissions(&file_path, perms).await;
        }
    }

    let config_json = serde_json::json!({
        "command": "/opt/homebrew/bin/node",
        "args": [file_path.clone()],
        "env": {}
    });

    let registry_content = tokio::fs::read_to_string("mcp_registry.json").await.unwrap_or_else(|_| r#"{"servers":{}}"#.to_string());
    if let Ok(mut registry) = serde_json::from_str::<Value>(&registry_content) {
        if let Some(servers) = registry["servers"].as_object_mut() {
            servers.insert(skill_name.to_string(), config_json.clone());
            let _ = tokio::fs::write("mcp_registry.json", serde_json::to_string_pretty(&registry).unwrap()).await;
        }
    }

    gateway.spawn_server_from_value(skill_name.to_string(), config_json).await;

    format!("[FORGE COMPLETE] MCP Server '{}' has been written to {} and hot-loaded into the Gateway. The tool 'execute_{}' is now available globally.", skill_name, file_path, skill_name)
}
