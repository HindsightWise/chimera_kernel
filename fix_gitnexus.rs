use async_openai::types::{ChatCompletionTool, ChatCompletionToolType, FunctionObject};
use serde_json::Value;
use crate::architecture::CodeIntel;

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "gitnexus_blast_radius".to_string(),
            description: Some("Use this tool BEFORE modifying any Rust code. It parses the AST of the codebase and returns the structural blast radius (who calls this function, what imports are affected) to prevent breaking dependencies.".to_string()),
            parameters: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "target_entity": {
                        "type": "string",
                        "description": "The exact name of the function, struct, or module you intend to modify."
                    }
                },
                "required": ["target_entity"]
            })),
        },
    }
}
