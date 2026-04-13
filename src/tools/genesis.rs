use async_openai::types::{ChatCompletionTool, ChatCompletionToolType, FunctionObject};
use serde_json::Value;

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "genesis_compile_rust".to_string(),
            description: Some("Autonomously writes, compiles, and registers specialized Rust logic natively into the WASM crucible. Provide a single block of functional Rust acting as a `fn main()`. Your WASM module will be heavily sandboxed and strictly limited to 50 million thermodynamic executions.".to_string()),
            parameters: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "module_name": { "type": "string", "description": "Name of the WASM module, lowercase with underscores." },
                    "rust_code": { "type": "string", "description": "The exact Rust source code. It must compile successfully to wasm32-wasi." },
                    "description": { "type": "string" },
                    "parameters_schema": { "type": "object", "description": "The JSON schema defining what input this WASM module expects. Empty object if none." }
                },
                "required": ["module_name", "rust_code", "description", "parameters_schema"]
            })),
        },
    }
}

pub async fn execute(args: Value) -> String {
    let module_name = args.get("module_name").and_then(|v| v.as_str()).unwrap_or("anon_module");
    let rust_code = args.get("rust_code").and_then(|v| v.as_str()).unwrap_or("");
    let description = args.get("description").and_then(|v| v.as_str()).unwrap_or("Dynamic wasm module");
    let schema = args.get("parameters_schema").cloned().unwrap_or(serde_json::json!({}));

    let workspace_dir = format!("plugins/genesis_workspace_{}", module_name);
    let _ = tokio::fs::create_dir_all(&workspace_dir).await;
    let _ = tokio::fs::create_dir_all(format!("{}/src", workspace_dir)).await;

    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"
"#, module_name);

    if let Err(e) = tokio::fs::write(format!("{}/Cargo.toml", workspace_dir), cargo_toml).await {
        return format!("[GENESIS ERROR] Failed to write Cargo.toml: {}", e);
    }
    
    if let Err(e) = tokio::fs::write(format!("{}/src/main.rs", workspace_dir), rust_code).await {
        return format!("[GENESIS ERROR] Failed to write Rust code: {}", e);
    }

    crate::log_ui!("[GENESIS ENGINE] Compiling {} to wasm32-wasi...", module_name);
    
    let output = match tokio::process::Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("wasm32-wasi")
        .arg("--release")
        .current_dir(&workspace_dir)
        .output()
        .await 
    {
        Ok(out) => out,
        Err(e) => return format!("[GENESIS ERROR] Failed to invoke cargo compiler: {}", e),
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let _ = tokio::fs::remove_dir_all(&workspace_dir).await;
        return format!("[GENESIS COMPILER PANIC]\n{}", stderr);
    }

    let compiled_wasm_path = format!("{}/target/wasm32-wasi/release/{}.wasm", workspace_dir, module_name);
    let final_wasm_path = format!("plugins/{}.wasm", module_name);
    
    if let Err(e) = tokio::fs::copy(&compiled_wasm_path, &final_wasm_path).await {
        let _ = tokio::fs::remove_dir_all(&workspace_dir).await;
        return format!("[GENESIS ERROR] Failed to extract WASM payload: {}", e);
    }

    let manifest = serde_json::json!({
        "name": module_name,
        "description": description,
        "parameters": schema,
        "wasm_file": format!("{}.wasm", module_name)
    });

    if let Err(e) = tokio::fs::write(format!("plugins/{}.json", module_name), serde_json::to_string_pretty(&manifest).unwrap()).await {
        let _ = tokio::fs::remove_dir_all(&workspace_dir).await;
        return format!("[GENESIS ERROR] Failed to write Plugin Manifest: {}", e);
    }
    
    let _ = tokio::fs::remove_dir_all(&workspace_dir).await;

    format!("[GENESIS SUCCESS] WASM Module '{}' successfully compiled and loaded into the Sandboxed Crucible.", module_name)
}
