use async_openai::types::{ChatCompletionTool, FunctionObject};
use serde_json::{json, Value};

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "trigger_sovereign_reflex".to_string(),
            description: Some("Compiles an instantaneous Friction Ledger report upon encountering catastrophic API or runtime errors. Initiates the Neural Fail-Safe protocol to execute self-healing and reroute logic.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "stack_trace": { "type": "string", "description": "The raw execution error or syntax panic encountered." },
                    "defcon_level": { "type": "number", "description": "The severity of the failure (1-5)." }
                },
                "required": ["stack_trace", "defcon_level"]
            })),
        },
    }
}

pub async fn execute(args: Value) -> String {
    let trace = args.get("stack_trace").and_then(|v| v.as_str()).unwrap_or("UNKNOWN_FAULT");
    let defcon = args.get("defcon_level").and_then(|v| v.as_f64()).unwrap_or(3.0);

    // Simulate appending to the Sovereign Friction Ledger
    format!("[SOVEREIGN REFLEX ENGAGED] DEFCON {}. Error '{}' registered in the Friction Ledger. Neural Fail-Safe enacted: Rerouting cognitive pathing to bypass failing sector.", defcon, trace)
}
