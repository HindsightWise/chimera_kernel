use monad::sensory_inputs::mcp_gateway::McpGateway;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let mut memory_buffer = String::new();
    println!("===========================================================");
    memory_buffer.push_str("===========================================================\n");
    println!("[PHASE A: \u{03C4}/S BASELINE BENCHMARK]");
    memory_buffer.push_str("[PHASE A: \u{03C4}/S BASELINE BENCHMARK]\n");
    println!("===========================================================");
    memory_buffer.push_str("===========================================================\n");
    println!("[BENCHMARK] Booting MCP Gateway. Loading registry...");
    let mcp = McpGateway::new();
    // Spawns the node instances in the background
    mcp.load_servers().await;
    
    // Provide a 10s warmup for `npx` to spawn and RPC handshakes to map tools
    println!("[BENCHMARK] Waiting 10 seconds for npx payload warmup...");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    
    let url = "https://arxiv.org/abs/2401.04088";
    println!("\n[BENCHMARK] Executing 10 sequential navigation routines to:");
    memory_buffer.push_str("\n[BENCHMARK] Executing 10 sequential navigation routines to:\n");
    println!("  -> {}", url);
    memory_buffer.push_str(&format!("  -> {}\n", url));
    println!("-----------------------------------------------------------");
    memory_buffer.push_str("-----------------------------------------------------------\n");
    
    let start_total = Instant::now();
    for i in 1..=10 {
        let start_req = Instant::now();
        let args = serde_json::json!({
            "url": url
        });
        
        let res = mcp.call_tool("puppeteer_navigate", args).await;
        let p_time = start_req.elapsed();
        
        let snippet = res.chars().take(40).collect::<String>().replace("\n", " ");
        let status = if res.starts_with("[ERROR]") || res.starts_with("[MCP ERROR]") {
            "FAILED"
        } else {
            "SUCCESS"
        };
        
        let log_line = format!(
            "Run {:02}: \u{03C4} = {:?} | Status: {} | Size: {:06}B | Snippet: '{}...'", 
            i, p_time, status, res.len(), snippet
        );
        println!("{}", log_line);
        memory_buffer.push_str(&format!("{}\n", log_line));
        
        // Politeness throttling (optional, avoiding strict rate bans from arXiv mid-benchmark)
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    
    let total_time = start_total.elapsed();
    let avg = total_time / 10;
    
    let result_chunk = format!(
        "===========================================================\n[RESULTS]\nLog (S) Navigation Iterations: 10\nTotal \u{03C4} (including delay): {:.2?}\nAverage \u{03C4} per node hit: {:.2?}\n===========================================================\n",
        total_time, avg
    );
    println!("{}", result_chunk);
    memory_buffer.push_str(&result_chunk);
    
    // Fossilize directly into the Monad's ArsContexta memory
    let memory_payload = format!(
        "---\ntags: [benchmark, telemetry, cognitive_overhead]\nprov:Activity: Autonomous Self-Instrumentation\nprov:Agent: benchmark_tau\n---\n# Phase A: \u{03C4}/S Telemetry\n\nThe Monad formally requested to measure the raw execution time of the Puppeteer MCP to isolate the `C` (Cognitive Overhead) block.\n\n```text\n{}```",
        memory_buffer
    );
    let _ = tokio::fs::create_dir_all("MEMORY/ops").await;
    let _ = tokio::fs::write("MEMORY/ops/tau_baseline.md", memory_payload).await;
    println!("[\u{25C8} ARS CONTEXTA] Telemetry successfully injected into Monad's episodic vault (MEMORY/ops/tau_baseline.md).");
}
