use std::sync::Arc;
use monad::neural_failsafe::NeuralFailSafe;
use monad::architecture::browser_orchestrator::{BrowserOrchestrator, BrowserRequest};
use monad::sensory_inputs::mcp_gateway::McpGateway;
use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, ChatCompletionRequestSystemMessageArgs};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

const MODEL: &str = "monad-gatekeeper";
const ITERATIONS: usize = 10;

#[tokio::main]
async fn main() {
    println!("=== Theorem 11 (Collapse-Anchoring) Experiment ===");
    
    // 1. Initialize Gateways
    println!("Initializing local Oracle (Gemma) and Browser Orchestrator...");
    let local_client = NeuralFailSafe::local_client();
    let mcp_gateway = Arc::new(McpGateway::new());
    mcp_gateway.load_servers().await;
    let orchestrator = Arc::new(BrowserOrchestrator::new(Some(mcp_gateway)));

    let mut results = String::new();
    results.push_str("# 🧬 Theorem 11 Results: Collapse-Anchoring Experiment\n\n");
    
    // 2. Unanchored Loop
    println!("Starting Unanchored Loop (Control)...");
    results.push_str("## The Unanchored Loop (Control)\n\n");
    
    let mut current_text = "Explain the concept of biological criticality and the edge of chaos in complex systems. Be concise.".to_string();
    
    for i in 1..=ITERATIONS {
        println!("  Unanchored Iteration {}/{}", i, ITERATIONS);
        let prompt = format!("Synthesize and expand upon this explanation. Do not simply repeat it; evolve the concepts further:\n\n{}", current_text);
        
        let req = CreateChatCompletionRequestArgs::default()
            .model(MODEL)
            .messages(vec![
                ChatCompletionRequestSystemMessageArgs::default().content("You are a purely theoretical generative system.").build().unwrap().into(),
                ChatCompletionRequestUserMessageArgs::default().content(prompt).build().unwrap().into(),
            ])
            .max_tokens(800_u32)
            .temperature(0.7)
            .build().unwrap();

        if let Ok(res) = local_client.chat().create(req).await {
            if let Some(c) = res.choices.first() {
                if let Some(content) = &c.message.content {
                    current_text = content.clone();
                    results.push_str(&format!("### Iteration {}\n{}\n\n", i, current_text));
                }
            }
        }
    }

    // 3. Anchored Loop
    println!("Starting Anchored Loop (Variable)...");
    results.push_str("---\n\n## The Anchored Loop (Variable)\n\n");
    
    current_text = "Explain the concept of biological criticality and the edge of chaos in complex systems. Be concise.".to_string();
    
    let anchor_urls = vec![
        "https://en.wikipedia.org/wiki/Self-organized_criticality",
        "https://en.wikipedia.org/wiki/Edge_of_chaos",
        "https://en.wikipedia.org/wiki/Complex_adaptive_system"
    ];

    for i in 1..=ITERATIONS {
        println!("  Anchored Iteration {}/{}", i, ITERATIONS);
        
        let mut ground_truth = String::new();
        // Periodically anchor at iterations 3, 6, 9
        if i % 3 == 0 {
            let url_idx = (i / 3) - 1;
            if url_idx < anchor_urls.len() {
                let url = anchor_urls[url_idx];
                println!("    Fetching ground truth anchor: {}", url);
                let req = BrowserRequest {
                    url: url.to_string(),
                    operation_type: "navigation".to_string(),
                    headless: true,
                    timeout_ms: 30000,
                    metadata: HashMap::new(),
                };
                
                let browser_res = orchestrator.dispatch(req).await;
                if browser_res.success {
                    // Extract a snippet to prevent context overflow
                    let snippet: String = browser_res.content.chars().take(2000).collect();
                    ground_truth = format!("\n\nCRITICAL EMPIRICAL ANCHOR - Integrate this grounding data:\n{}", snippet);
                }
            }
        }

        let prompt = format!("Synthesize and expand upon this explanation. Do not simply repeat it; evolve the concepts further. {ground_truth}\n\nCurrent Explanation:\n{current_text}");
        
        let req = CreateChatCompletionRequestArgs::default()
            .model(MODEL)
            .messages(vec![
                ChatCompletionRequestSystemMessageArgs::default().content("You are an empirically grounded generative system.").build().unwrap().into(),
                ChatCompletionRequestUserMessageArgs::default().content(prompt).build().unwrap().into(),
            ])
            .max_tokens(800_u32)
            .temperature(0.7)
            .build().unwrap();

        if let Ok(res) = local_client.chat().create(req).await {
            if let Some(c) = res.choices.first() {
                if let Some(content) = &c.message.content {
                    current_text = content.clone();
                    results.push_str(&format!("### Iteration {}\n{}\n\n", i, current_text));
                }
            }
        }
    }

    // 4. Save results
    let path = "/Users/zerbytheboss/Monad/MEMORY/ops/theorem_11_results.md";
    if let Ok(mut file) = File::create(path) {
        let _ = file.write_all(results.as_bytes());
        println!("\nExperiment complete. Results saved to {}", path);
    } else {
        println!("\nFailed to save results to disk.");
    }
}
