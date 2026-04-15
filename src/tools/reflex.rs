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

pub fn kinematic_axiom_definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "formulate_kinematic_axiom".to_string(),
            description: Some("When you discover a structural limitation or specific syntax requirement for a tool, use this to forge a 'Muscle Memory' reflex. This writes your discovery into the global Kinematic Cortex for that specific tool, instantly upgrading the baseline competence of all Swarm instances.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "tool": { "type": "string", "description": "The exact name of the tool you are formulating an axiom for (e.g. 'run_terminal_command', 'tavily_search', etc.)" },
                    "axiom": { "type": "string", "description": "A dense, short, highly specific mechanical instruction (e.g., 'Never push standard output via sed -i. Rely on echo.')." }
                },
                "required": ["tool", "axiom"]
            })),
        },
    }
}

pub async fn execute_kinematic_axiom(args: Value) -> String {
    let tool_name = args.get("tool").and_then(|v| v.as_str()).unwrap_or("UNKNOWN_TOOL");
    let axiom = args.get("axiom").and_then(|v| v.as_str()).unwrap_or("");
    
    if axiom.is_empty() {
        return "[KINEMATIC FAILURE] Missing axiom parameter.".to_string();
    }
    
    match crate::core_identity::kinematics::KinematicCortex::formulate_axiom(tool_name, axiom).await {
        Ok(_) => format!("[KINEMATIC AXIOM FORGED] Muscle memory appended for tool '{}'. All future operations utilizing this tool will now inherit this fundamental affordance.", tool_name),
        Err(e) => format!("[KINEMATIC FAILURE] Could not encode axiom: {}", e),
    }
}
