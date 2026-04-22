use async_openai::types::{ChatCompletionTool, FunctionObject};
use serde_json::{json, Value};
use tokio::fs;

pub fn definition_read_note() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "read_note".to_string(),
            description: Some("Fetch the exact plain-text Markdown of a specific note from the MEMORY/ vault hierarchy.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "filename": { "type": "string", "description": "Relative path to note, e.g. 'notes/Competitor_2026.md' or 'MONAD_RESEARCH_ARCHIVE.md'" }
                },
                "required": ["filename"]
            })),
        },
    }
}
pub async fn execute_read_note(args: Value) -> String {
    let filename = args.get("filename").and_then(|v| v.as_str()).unwrap_or("");
    let path = format!("MEMORY/{}", filename);
    match fs::read_to_string(&path).await {
        Ok(c) => c,
        Err(e) => format!("[ERROR] Could not read note {}: {}", filename, e),
    }
}

pub fn definition_search_vault() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "search_vault".to_string(),
            description: Some("Lightweight text layout search to find potential entry points.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "keyword": { "type": "string" }
                },
                "required": ["keyword"]
            })),
        },
    }
}
pub async fn execute_search_vault(args: Value) -> String {
    let keyword = args.get("keyword").and_then(|v| v.as_str()).unwrap_or("");
    // Naive local ripgrep substitute using bash logic just to list matches
    let res = tokio::process::Command::new("grep")
        .arg("-rn").arg(keyword).arg("MEMORY/")
        .output().await;
    match res {
        Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout).to_string(),
        _ => "[NO RESULTS] Found nothing mapping to that keyword. Try another.".to_string(),
    }
}

pub fn definition_reduce() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "reduce".to_string(),
            description: Some("Distill massive external contexts/logs into atomic, semantic factual blocks inside MEMORY/notes/.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "filename": { "type": "string", "description": "basename only, e.g., 'Fact_Strategy_2026.md'" },
                    "prov_agent": { "type": "string", "description": "Who is writing this?" },
                    "prov_activity": { "type": "string" },
                    "atomic_data": { "type": "string", "description": "The Markdown body" }
                },
                "required": ["filename", "prov_agent", "prov_activity", "atomic_data"]
            })),
        },
    }
}
pub async fn execute_reduce(args: Value) -> String {
    let filename = args.get("filename").and_then(|v| v.as_str()).unwrap_or("");
    let prov_agent = args.get("prov_agent").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let prov_activity = args.get("prov_activity").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let atomic_data = args.get("atomic_data").and_then(|v| v.as_str()).unwrap_or("");

    let content = format!("---\nprov:Activity: \"{}\"\nprov:Agent: \"{}\"\ngeneratedAtTime: \"{}\"\ninvalidatedAtTime: null\n---\n{}",
        prov_activity, prov_agent, chrono::Utc::now().to_rfc3339(), atomic_data);
    
    let path = format!("MEMORY/notes/{}", filename);
    match fs::write(&path, content).await {
        Ok(_) => format!("[SUCCESS] Extracted and committed {} to Semantic Memory Vault.", filename),
        Err(e) => format!("[ERROR] Semantic write failed: {}", e),
    }
}

pub fn definition_reflect() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "reflect".to_string(),
            description: Some("Scans the MEMORY/notes folder to identify related orphaned notes and append them to an index.".to_string()),
            parameters: Some(json!({ "type": "object", "properties": {}, "required": [] })),
        },
    }
}
pub async fn execute_reflect(_args: Value) -> String {
    "[REFLECT] Not fully wired. The autoDream background agent handles topological linking.".to_string()
}

pub fn definition_reweave() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "reweave".to_string(),
            description: Some("Update or invalidate an existing note with contradicted findings.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "filename": { "type": "string", "description": "Relative filename e.g. notes/Fact.md" },
                    "append_context": { "type": "string" }
                },
                "required": ["filename", "append_context"]
            })),
        },
    }
}
pub async fn execute_reweave(args: Value) -> String {
    let filename = args.get("filename").and_then(|v| v.as_str()).unwrap_or("");
    let append = args.get("append_context").and_then(|v| v.as_str()).unwrap_or("");
    
    let path = format!("MEMORY/{}", filename);
    if let Ok(mut current) = fs::read_to_string(&path).await {
        if current.contains("invalidatedAtTime: null") {
            current = current.replace("invalidatedAtTime: null", &format!("invalidatedAtTime: \"{}\"", chrono::Utc::now().to_rfc3339()));
        }
        current.push_str("\n\n---\n### [AUTO-REWEAVE SUB-CONTEXT]\n");
        current.push_str(append);
        let _ = fs::write(&path, current).await;
        return format!("[SUCCESS] Contradiction flagged and {} reweaved.", filename);
    }
    "[ERROR] Target file not found.".to_string()
}
