use std::sync::Arc;
use tokio::runtime::Runtime;
use chimera_kernel::architecture::multi_agent_kernel::MultiAgentKernel;

fn main() {
    println!("=== Testing Kernel Startup ===");
    
    let rt = Runtime::new().unwrap();
    
    rt.block_on(async {
        println!("Creating MultiAgentKernel...");
        let kernel = MultiAgentKernel::new().await;
        println!("Kernel created successfully!");
        
        println!("Spawning background coordination...");
        kernel.spawn_background_coordination().await;
        println!("Background coordination spawned!");
        
        // Wait a bit to see if it crashes
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("Test completed without crash!");
    });
}
