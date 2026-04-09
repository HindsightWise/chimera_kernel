use async_openai::types::ChatCompletionTool;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::wiki::WikiManager;
use crate::wiki::operations::WikiOperation;

pub fn definition() -> ChatCompletionTool {
    crate::wiki::get_wiki_tool()
}

pub async fn execute(args: Value, manager_lock: Arc<Mutex<WikiManager>>) -> String {
    let operation = match args.get("operation").and_then(|v| v.as_str()) {
        Some(op) => op,
        None => return "[ERROR] Missing required parameter: 'operation'".to_string(),
    };
    
    let mut manager = manager_lock.lock().await;

    let op = match operation {
        "ingest" => {
            let path = args.get("document_path").and_then(|v| v.as_str()).unwrap_or("");
            if path.is_empty() { return "[ERROR] Missing 'document_path'".to_string(); }
            WikiOperation::Ingest { document_path: path.to_string() }
        },
        "query" => {
            let q = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
            if q.is_empty() { return "[ERROR] Missing 'query'".to_string(); }
            WikiOperation::Query { query: q.to_string() }
        },
        "compile" => WikiOperation::Compile,
        "health_check" => WikiOperation::HealthCheck,
        "generate_article" => {
            let t = args.get("topic").and_then(|v| v.as_str()).unwrap_or("");
            if t.is_empty() { return "[ERROR] Missing 'topic'".to_string(); }
            WikiOperation::GenerateArticle { topic: t.to_string() }
        },
        _ => return format!("[ERROR] Unknown wiki operation: {}", operation)
    };

    match op.execute(&mut manager).await {
        Ok(result) => format!("[WIKI SUCCESS]\n{}", result),
        Err(e) => format!("[WIKI ERROR]\n{}", e),
    }
}
