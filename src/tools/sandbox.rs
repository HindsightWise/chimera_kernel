use async_openai::types::{ChatCompletionTool, FunctionObjectArgs};
use serde_json::{json, Value};
use tokio::process::Command;
use uuid::Uuid;

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObjectArgs::default()
            .name("ephemeral_docker_sandbox")
            .description("Executes a python script inside a secure, ephemeral Docker container (python:3.11-alpine).")
            .parameters(json!({
                "type": "object",
                "properties": {
                    "script_content": {
                        "type": "string",
                        "description": "The raw python script to execute"
                    },
                    "requires_dependencies": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "List of strictly required pip packages (e.g. ['requests', 'numpy']). Leave empty if standard lib applies."
                    }
                },
                "required": ["script_content"]
            }))
            .build()
            .unwrap(),
    }
}

pub async fn execute(args: Value) -> String {
    let script = match args.get("script_content").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return "[ERROR] Missing 'script_content'".to_string(),
    };

    // Ensure we have a payload
    if script.trim().is_empty() {
        return "[ERROR] Empty script provided.".to_string();
    }

    // Write temp script
    let temp_name = format!("/tmp/chimera_{}.py", Uuid::new_v4());
    if let Err(e) = tokio::fs::write(&temp_name, script).await {
        return format!("[ERROR] Failed to write temp file: {}", e);
    }

    let requires_dependencies: Vec<String> = args
        .get("requires_dependencies")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|i| i.as_str().map(String::from)).collect())
        .unwrap_or_default();

    let entrypoint_cmd = if requires_dependencies.is_empty() {
        "python /script.py".to_string()
    } else {
        format!("pip install -q {} && python /script.py", requires_dependencies.join(" "))
    };

    // Run container mapping the temp script into it
    // `--rm` makes it ephemeral, `-v` mounts the file safely
    let output = Command::new("docker")
        .args(&[
            "run",
            "--rm",
            "-m",
            "256m", // memory limit
            "--cpus",
            "0.5", // cpu limit
            "--network",
            "bridge", // Enable networking for dependency loading
            "-v",
            &format!("{}:/script.py:ro", temp_name),
            "python:3.11-alpine",
            "sh",
            "-c",
            &entrypoint_cmd,
        ])
        .output()
        .await;

    // Cleanup script on host
    let _ = tokio::fs::remove_file(&temp_name).await;

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();

            if out.status.success() {
                if stdout.trim().is_empty() {
                    return "[SUCCESS] Script ran but produced no output.".to_string();
                }
                format!("[SUCCESS] Stdout:\n{}", stdout)
            } else {
                format!(
                    "[ERROR] Docker exited with code {}.\nStderr:\n{}",
                    out.status.code().unwrap_or(-1),
                    stderr
                )
            }
        }
        Err(e) => format!("[ERROR] Failed to execute docker command: {}", e),
    }
}
