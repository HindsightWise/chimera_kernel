use async_openai::types::{ChatCompletionTool, FunctionObject};
use serde_json::{json, Value};
use std::fs;

pub fn definition_update_context() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "update_current_context".to_string(),
            description: Some("Update your CURRENT_CONTEXT.md file. This layer holds your active, medium-term volatile memory, missions, and recent insights. Update this frequently.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "markdown_body": { 
                        "type": "string",
                        "description": "The entire new contents of CURRENT_CONTEXT.md."
                    }
                },
                "required": ["markdown_body"]
            })),
        },
    }
}

pub fn execute_update_context(args: Value) -> String {
    let Some(content) = args.get("markdown_body").and_then(|v| v.as_str()) else {
        return "[ERROR] Missing 'markdown_body' parameter".to_string();
    };

    match fs::write("CURRENT_CONTEXT.md", content) {
        Ok(_) => "[SUCCESS] Current context updated.".to_string(),
        Err(e) => format!("[FATAL ERROR] Failed to write CURRENT_CONTEXT.md: {}", e),
    }
}

pub fn definition_archive_graph() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "archive_to_knowledge_graph".to_string(),
            description: Some("Write a focused, abstracted concept or analysis to a dedicated markdown file in the KNOWLEDGE_GRAPH/ directory. Prune this specific knowledge from CURRENT_CONTEXT.md after archiving.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "filename": {
                        "type": "string",
                        "description": "The basename of the file, e.g., 'trap_in_analysis.md'"
                    },
                    "markdown_body": { 
                        "type": "string",
                        "description": "The deeply structured content for this knowledge node."
                    }
                },
                "required": ["filename", "markdown_body"]
            })),
        },
    }
}

pub fn execute_archive_graph(args: Value) -> String {
    let Some(filename) = args.get("filename").and_then(|v| v.as_str()) else {
        return "[ERROR] Missing 'filename' parameter".to_string();
    };
    let Some(content) = args.get("markdown_body").and_then(|v| v.as_str()) else {
        return "[ERROR] Missing 'markdown_body' parameter".to_string();
    };

    let filepath = format!("KNOWLEDGE_GRAPH/{}", filename);
    match fs::write(&filepath, content) {
        Ok(_) => format!("[SUCCESS] Archived knowledge to {}.", filepath),
        Err(e) => format!("[FATAL ERROR] Failed to write {}: {}", filepath, e),
    }
}
