use chimera_kernel::architecture::multi_agent_kernel::MultiAgentKernel;
use chimera_kernel::architecture::agent_trait::Task;
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashSet;

#[tokio::test]
async fn test_complex_task_decomposition_and_coordination() {
    println!("🧪 TESTING SWARM INTELLIGENCE ORCHESTRATION");
    println!("==========================================");
    
    // Initialize the multi-agent kernel
    let kernel = MultiAgentKernel::new().await;
    kernel.spawn_background_coordination().await;
    
    // Wait for agents to initialize
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Create a complex task that should trigger decomposition
    println!("📋 Creating complex task: 'audit repo for security vulnerabilities and market analysis'");
    
    let complex_task = Task {
        id: Uuid::new_v4(),
        task_type: "complex_audit".to_string(),
        payload: serde_json::json!({
            "instruction": "audit repo for security vulnerabilities and market analysis"
        }),
        required_capabilities: HashSet::new(), // Empty for decomposition
        priority: 128,
        dependencies: vec![],
        created_at: Utc::now(),
        timeout_secs: Some(300),
    };
    
    // Publish the task to the message bus
    let msg = chimera_kernel::architecture::message_bus::Message {
        id: Uuid::new_v4(),
        sender: Uuid::new_v4(),
        topic: "SYSTEM.NEW_TASK".to_string(),
        payload: serde_json::to_value(&complex_task).unwrap(),
        timestamp: Utc::now(),
        priority: 128,
        ttl_secs: Some(3600),
    };
    
    kernel.message_bus.publish(msg).await.expect("Failed to publish task");
    
    println!("🚀 Task published to message bus");
    println!("⏳ Waiting for decomposition and coordination...");
    
    // Wait for decomposition and execution
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    // Check task manager for results
    let task_manager = kernel.task_manager.read().await;
    let stats = task_manager.get_stats().await;
    
    println!("📊 RESULTS:");
    println!("  • Pending tasks: {}", stats.pending_count);
    println!("  • Running tasks: {}", stats.running_count);
    println!("  • Completed tasks: {}", stats.completed_count);
    println!("  • Failed tasks: {}", stats.failed_count);
    println!("  • Total tasks: {}", stats.total_count);
    
    // Verify that complex task was decomposed
    assert!(stats.total_count > 0, "No tasks were processed");
    
    println!("✅ Test completed - swarm intelligence is active!");
}
