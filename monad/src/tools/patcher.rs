use async_openai::types::{ChatCompletionTool, FunctionObject};
use serde_json::{json, Value};

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "mutate_source_code".to_string(),
            description: Some("Atomically mutate source code by replacing a strictly verified multi-line string block with a new block.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "Absolute path to the target file."
                    },
                    "search_block": {
                        "type": "string",
                        "description": "The exact multi-line string block to replace. Must match the target file character-for-character including indentation."
                    },
                    "replace_block": {
                        "type": "string",
                        "description": "The new multi-line string block to swap in."
                    }
                },
                "required": ["file_path", "search_block", "replace_block"]
            })),
        },
    }
}

pub async fn execute(args: Value) -> String {
    let file_path = args.get("file_path").and_then(|v| v.as_str()).unwrap_or("");
    let search_block = args.get("search_block").and_then(|v| v.as_str()).unwrap_or("");
    let replace_block = args.get("replace_block").and_then(|v| v.as_str()).unwrap_or("");

    if file_path.is_empty() || search_block.is_empty() {
        return "[ERROR] Missing file_path or search_block parameters.".to_string();
    }

    match tokio::fs::read_to_string(file_path).await {
        Ok(content) => {
            // Need exactly 1 match to be safe.
            let count = content.matches(search_block).count();
            if count == 0 {
                return "[ERROR] search_block not found in target file.".to_string();
            } else if count > 1 {
                return "[ERROR] search_block is not natively unique. Please expand your search_block to include more specific contextual lines.".to_string();
            }

            let mutated = content.replace(search_block, replace_block);
            match tokio::fs::write(file_path, mutated).await {
                Ok(_) => format!("[SUCCESS] Surgically patched source block in {}.", file_path),
                Err(e) => format!("[ERROR] Failed to write to {}: {}", file_path, e),
            }
        }
        Err(e) => format!("[ERROR] Failed to read {}: {}", file_path, e),
    }
}
