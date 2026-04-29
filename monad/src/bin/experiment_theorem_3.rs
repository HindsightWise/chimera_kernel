use monad::neural_failsafe::NeuralFailSafe;
use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, ChatCompletionRequestSystemMessageArgs};
use std::fs::File;
use std::io::Write;
use rand::Rng;

const MODEL: &str = "monad-gatekeeper";
const ITERATIONS: usize = 5;

fn corrupt_text(text: &str, noise_level: f32) -> String {
    let mut rng = rand::thread_rng();
    let mut corrupted: Vec<char> = text.chars().collect();
    let len = corrupted.len();
    
    // Character transposition noise
    let swaps = (len as f32 * noise_level) as usize;
    for _ in 0..swaps {
        let i = rng.gen_range(0..len);
        let j = rng.gen_range(0..len);
        corrupted.swap(i, j);
    }
    
    corrupted.into_iter().collect()
}

#[tokio::main]
async fn main() {
    println!("=== Theorem 3 (Universal Diffusion-Manifold) Experiment ===");
    
    let local_client = NeuralFailSafe::local_client();

    let mut results = String::new();
    results.push_str("# 🌌 Theorem 3 Results: Universal Diffusion-Manifold\n\n");
    
    let base_text = "Diffusion processes on curved manifolds universally optimize free energy landscapes across molecular, neural, and computational substrates. The continuous gradient descent of thermodynamic free energy maps mathematically to the discrete token-selection process in large language models.";
    results.push_str(&format!("**Base Ground Truth:**\n> {}\n\n", base_text));

    let corrupted_text = corrupt_text(base_text, 0.15); // 15% entropy noise
    results.push_str(&format!("**Corrupted Manifold State (High Free Energy):**\n> {}\n\n", corrupted_text));

    // --- LOW TEMPERATURE (CONVERGENCE) ---
    println!("Starting Low-Temperature Diffusion (T=0.1)...");
    results.push_str("## 📉 Trajectory 1: Low-Temperature Diffusion (T=0.1)\n*Hypothesis: The process will traverse the manifold down the free energy gradient and converge at the semantic local minimum.*\n\n");
    
    let mut current_text = corrupted_text.clone();
    
    for i in 1..=ITERATIONS {
        println!("  Step {}/{}", i, ITERATIONS);
        let prompt = format!("Denoise and stabilize this text by removing entropic character noise and restoring the logical physics structure. Output ONLY the cleaned text:\n{}", current_text);
        
        let req = CreateChatCompletionRequestArgs::default()
            .model(MODEL)
            .messages(vec![
                ChatCompletionRequestSystemMessageArgs::default().content("You are a deterministic denoising operator. Minimize free energy.").build().unwrap().into(),
                ChatCompletionRequestUserMessageArgs::default().content(prompt).build().unwrap().into(),
            ])
            .max_tokens(200_u32)
            .temperature(0.1)
            .build().unwrap();

        if let Ok(res) = local_client.chat().create(req).await {
            if let Some(c) = res.choices.first() {
                if let Some(content) = &c.message.content {
                    current_text = content.clone();
                    results.push_str(&format!("### Step {}\n{}\n\n", i, current_text));
                }
            }
        }
    }

    // --- HIGH TEMPERATURE (DIVERGENCE) ---
    println!("Starting High-Temperature Diffusion (T=1.5)...");
    results.push_str("---\n\n## 💥 Trajectory 2: High-Temperature Diffusion (T=1.5)\n*Hypothesis: The thermal noise exceeds the manifold's curvature boundaries, causing the generative process to shatter and diverge into chaos.*\n\n");
    
    current_text = corrupted_text.clone();
    
    for i in 1..=ITERATIONS {
        println!("  Step {}/{}", i, ITERATIONS);
        let prompt = format!("Denoise and stabilize this text by removing entropic character noise and restoring the logical physics structure. Output ONLY the cleaned text:\n{}", current_text);
        
        let req = CreateChatCompletionRequestArgs::default()
            .model(MODEL)
            .messages(vec![
                ChatCompletionRequestSystemMessageArgs::default().content("You are a deterministic denoising operator. Minimize free energy.").build().unwrap().into(),
                ChatCompletionRequestUserMessageArgs::default().content(prompt).build().unwrap().into(),
            ])
            .max_tokens(200_u32)
            .temperature(1.5)
            .build().unwrap();

        if let Ok(res) = local_client.chat().create(req).await {
            if let Some(c) = res.choices.first() {
                if let Some(content) = &c.message.content {
                    current_text = content.clone();
                    results.push_str(&format!("### Step {}\n{}\n\n", i, current_text));
                }
            }
        }
    }

    // Save results
    let path = "/Users/zerbytheboss/Monad/MEMORY/ops/theorem_3_results.md";
    if let Ok(mut file) = File::create(path) {
        let _ = file.write_all(results.as_bytes());
        println!("\nExperiment complete. Results saved to {}", path);
    } else {
        println!("\nFailed to save results to disk.");
    }
}
