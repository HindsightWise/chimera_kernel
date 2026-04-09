use std::sync::Arc;
use tokio::runtime::Runtime;
use chimera_kernel::architecture::multi_agent_kernel::MultiAgentKernel;

fn main() {
    println!("=== MINIMAL KERNEL STARTUP TEST ===");
    
    // Create a minimal runtime
    let rt = Runtime::new().unwrap();
    
    rt.block_on(async {
        println!("[1/3] Creating MultiAgentKernel...");
        let kernel = MultiAgentKernel::new().await;
        println!("[2/3] Kernel created successfully!");
        
        // Don't spawn background coordination yet
        // kernel.spawn_background_coordination().await;
        
        println!("[3/3] Waiting 5 seconds...");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        println!("✅ Test completed without crash!");
    });
}
