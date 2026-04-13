use async_openai::types::{ChatCompletionTool, ChatCompletionToolType, FunctionObject};
use serde_json::{json, Value};
use tokio::process::Command;
use std::path::Path;
use colored::*;

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "binary_introspection".to_string(),
            description: Some("Perform advanced reverse engineering and malware analysis on a target binary. Capable of calculating Shannon entropy to detect packed/encrypted payloads, extracting static strings, or triggering a headless decompilation pass.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "file_path": { 
                        "type": "string", 
                        "description": "Absolute path to the binary file." 
                    },
                    "analysis_mode": { 
                        "type": "string", 
                        "enum": ["entropy_analysis", "static_strings", "ghidra_headless"],
                        "description": "The specific reverse engineering technique to apply." 
                    }
                },
                "required": ["file_path", "analysis_mode"]
            })),
        },
    }
}

pub async fn execute(args: Value) -> String {
    let file_path = args.get("file_path").and_then(|v| v.as_str()).unwrap_or("");
    let mode = args.get("analysis_mode").and_then(|v| v.as_str()).unwrap_or("entropy_analysis");

    if !Path::new(file_path).exists() {
        return format!("[ERROR] Target binary not found at path: {}", file_path);
    }

    match mode {
        "entropy_analysis" => {
            crate::log_verbose!("{} CALCULATING SHANNON ENTROPY", "[REVERSE ENGINEERING]".bright_magenta().bold());
            match calculate_entropy(file_path).await {
                Ok(entropy) => {
                    let assessment = if entropy > 7.2 {
                        "HIGH ENTROPY DETECTED. The binary is highly likely to be packed, encrypted, or obfuscated (malware defense mechanism)."
                    } else {
                        "Nominal entropy. Binary appears unpacked and instructions are likely plaintext."
                    };
                    format!("[ENTROPY ANALYSIS]\nTarget: {}\nShannon Entropy Score: {:.4} bits per byte\nDiagnosis: {}", file_path, entropy, assessment)
                }
                Err(e) => format!("[ERROR] Failed to read binary for entropy analysis: {}", e),
            }
        },
        "static_strings" => {
            crate::log_verbose!("{} EXTRACTING STATIC STRINGS", "[REVERSE ENGINEERING]".bright_magenta().bold());
            // Safe, fast extraction of contiguous printable characters to find IPs, URLs, or registry keys
            let output = Command::new("strings")
                .arg("-n").arg("8") // Only strings 8 chars or longer to reduce noise
                .arg(file_path)
                .output()
                .await;

            match output {
                Ok(res) if res.status.success() => {
                    let out = String::from_utf8_lossy(&res.stdout);
                    let lines: Vec<&str> = out.lines().collect();
                    let total = lines.len();
                    let preview = lines.into_iter().take(50).collect::<Vec<&str>>().join("\n");
                    format!("[STATIC STRINGS EXTRACTED]\nTotal found: {}\nFirst 50 strings:\n{}\n...", total, preview)
                }
                _ => "[ERROR] Failed to execute 'strings' command on target.".to_string(),
            }
        },
        "ghidra_headless" => {
            crate::log_verbose!("{} TRIGGERING GHIDRA HEADLESS", "[REVERSE ENGINEERING]".bright_magenta().bold());
            // Note: Assumes Ghidra is installed and 'analyzeHeadless' is in the system PATH
            // This is a stub that you can point to your actual Ghidra installation path
            let output = Command::new("analyzeHeadless")
                .arg("/tmp/ghidra_projects")
                .arg("AutoProject")
                .arg("-import").arg(file_path)
                .arg("-postScript").arg("FindInstructions.java")
                .arg("-deleteProject")
                .output()
                .await;

            match output {
                Ok(res) => format!("[GHIDRA DECOMPILATION]\n{}", String::from_utf8_lossy(&res.stdout)),
                Err(e) => format!("[ERROR] Ghidra Headless execution failed. Ensure it is installed and configured: {}", e),
            }
        },
        _ => format!("[ERROR] Unknown analysis mode: {}", mode),
    }
}

/// Natively calculates the Shannon Entropy of a binary file.
/// Entropy approaching 8.0 indicates dense encryption or compression.
async fn calculate_entropy(file_path: &str) -> Result<f64, String> {
    let bytes = tokio::fs::read(file_path).await.map_err(|e| e.to_string())?;
    if bytes.is_empty() { return Ok(0.0); }
    
    let mut counts = [0usize; 256];
    for &b in &bytes { counts[b as usize] += 1; }
    
    let mut entropy = 0.0;
    let len = bytes.len() as f64;
    
    for &count in &counts {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }
    
    Ok(entropy)
}
