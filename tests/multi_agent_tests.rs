use chimera_kernel::architecture::{AgentRegistry, BaseAgent, AgentCapability, Task, TaskManager, MessageBus, TaskStatus};
use uuid::Uuid;
use std::collections::HashSet;

#[tokio::test]
async fn test_agent_registration() {
    let registry = AgentRegistry::new();
    
    // Create test agents
    let capabilities1: HashSet<AgentCapability> = vec![AgentCapability::Reasoning, AgentCapability::Research].into_iter().collect();
    let agent1 = Box::new(BaseAgent::new("Reasoner".to_string(), capabilities1));
    
    let capabilities2: HashSet<AgentCapability> = vec![AgentCapability::ToolExecution].into_iter().collect();
    let agent2 = Box::new(BaseAgent::new("ToolExecutor".to_string(), capabilities2));
    
    // Register agents
    registry.register(agent1).await.expect("Failed to register agent1");
    registry.register(agent2).await.expect("Failed to register agent2");
    
    // Verify agent count
    assert_eq!(registry.agent_count().await, 2);
}

#[tokio::test]
async fn test_capability_based_routing() {
    let registry = AgentRegistry::new();
    
    // Create specialized agents
    let reasoning_caps: HashSet<AgentCapability> = vec![AgentCapability::Reasoning].into_iter().collect();
    let research_caps: HashSet<AgentCapability> = vec![AgentCapability::Research].into_iter().collect();
    let tool_caps: HashSet<AgentCapability> = vec![AgentCapability::ToolExecution].into_iter().collect();
    
    let _ = registry.register(Box::new(BaseAgent::new("Reasoner".to_string(), reasoning_caps.clone()))).await;
    let _ = registry.register(Box::new(BaseAgent::new("Researcher".to_string(), research_caps.clone()))).await;
    let _ = registry.register(Box::new(BaseAgent::new("ToolExecutor".to_string(), tool_caps.clone()))).await;
    
    // Test finding agents with specific capabilities
    let reasoning_req: HashSet<AgentCapability> = vec![AgentCapability::Reasoning].into_iter().collect();
    let capable_agents = registry.find_capable_agents(&reasoning_req).await;
    assert_eq!(capable_agents.len(), 1);
    
    // Test combined capabilities (should find none)
    let combined_req: HashSet<AgentCapability> = vec![AgentCapability::Reasoning, AgentCapability::Research].into_iter().collect();
    let capable_agents = registry.find_capable_agents(&combined_req).await;
    assert_eq!(capable_agents.len(), 0); // No single agent has both capabilities
}

#[tokio::test]
async fn test_task_manager_basic_flow() {
    let task_manager = TaskManager::new(100);
    
    // Create a simple task
    let task = Task {
        id: Uuid::new_v4(),
        task_type: "test_task".to_string(),
        payload: serde_json::json!({"test": "data"}),
        required_capabilities: HashSet::new(),
        priority: 50,
        dependencies: vec![],
        created_at: chrono::Utc::now(),
        timeout_secs: None,
    };
    
    // Submit task
    task_manager.submit_task(task.clone()).await.expect("Failed to submit task");
    
    // Check task status
    let status = task_manager.get_task_status(task.id).await;
    assert!(matches!(status, Some(TaskStatus::Pending)));
    
    // Get statistics
    let stats = task_manager.get_stats().await;
    assert_eq!(stats.pending_count, 1);
    assert_eq!(stats.running_count, 0);
    assert_eq!(stats.completed_count, 0);
}

#[tokio::test]
async fn test_message_bus_communication() {
    let message_bus = MessageBus::new(100);
    
    // Create test subscriber IDs
    let subscriber1 = Uuid::new_v4();
    let subscriber2 = Uuid::new_v4();
    
    // Subscribe to topics
    message_bus.subscribe(subscriber1, "test.topic").await.expect("Failed to subscribe");
    message_bus.subscribe(subscriber2, "test.topic").await.expect("Failed to subscribe");
    message_bus.subscribe(subscriber2, "other.topic").await.expect("Failed to subscribe");
    
    // Create and publish a message
    let message = chimera_kernel::architecture::Message {
        id: Uuid::new_v4(),
        topic: "test.topic".to_string(),
        payload: serde_json::json!({"content": "test message"}),
        sender: Uuid::new_v4(),
        timestamp: chrono::Utc::now(),
        priority: 50,
        ttl_secs: None,
    };
    
    message_bus.publish(message.clone()).await.expect("Failed to publish");
    
    // Check that both subscribers received the message
    assert!(message_bus.has_messages(subscriber1).await);
    assert!(message_bus.has_messages(subscriber2).await);
    
    // Receive messages
    let received1 = message_bus.receive(subscriber1).await;
    let received2 = message_bus.receive(subscriber2).await;
    
    assert!(received1.is_some());
    assert!(received2.is_some());
    assert_eq!(received1.unwrap().id, message.id);
    assert_eq!(received2.unwrap().id, message.id);
}

#[tokio::test]
async fn test_task_dependencies() {
    let task_manager = TaskManager::new(100);
    
    // Create dependent tasks
    let task1_id = Uuid::new_v4();
    let task2_id = Uuid::new_v4();
    
    let task1 = Task {
        id: task1_id,
        task_type: "first_task".to_string(),
        payload: serde_json::json!({}),
        required_capabilities: HashSet::new(),
        priority: 50,
        dependencies: vec![],
        created_at: chrono::Utc::now(),
        timeout_secs: None,
    };
    
    let task2 = Task {
        id: task2_id,
        task_type: "dependent_task".to_string(),
        payload: serde_json::json!({}),
        required_capabilities: HashSet::new(),
        priority: 50,
        dependencies: vec![task1_id],
        created_at: chrono::Utc::now(),
        timeout_secs: None,
    };
    
    // Submit both tasks
    task_manager.submit_task(task1.clone()).await.expect("Failed to submit task1");
    task_manager.submit_task(task2.clone()).await.expect("Failed to submit task2");
    
    // Try to get next task with empty capabilities
    let empty_caps = HashSet::new();
    let next_task = task_manager.get_next_task(&empty_caps).await;
    
    // Should get task1 (no dependencies)
    assert!(next_task.is_some());
    assert_eq!(next_task.unwrap().id, task1_id);
    
    // Task2 should not be available until task1 is completed
    let next_task2 = task_manager.get_next_task(&empty_caps).await;
    assert!(next_task2.is_none()); // No more tasks without satisfied dependencies
}

#[tokio::test]
async fn test_circular_dependency_prevention() {
    let task_manager = TaskManager::new(100);
    
    // Create circular dependency
    let task1_id = Uuid::new_v4();
    let task2_id = Uuid::new_v4();
    
    let task1 = Task {
        id: task1_id,
        task_type: "task1".to_string(),
        payload: serde_json::json!({}),
        required_capabilities: HashSet::new(),
        priority: 50,
        dependencies: vec![task2_id], // Depends on task2
        created_at: chrono::Utc::now(),
        timeout_secs: None,
    };
    
    let task2 = Task {
        id: task2_id,
        task_type: "task2".to_string(),
        payload: serde_json::json!({}),
        required_capabilities: HashSet::new(),
        priority: 50,
        dependencies: vec![task1_id], // Depends on task1
        created_at: chrono::Utc::now(),
        timeout_secs: None,
    };
    
    // Try to submit task1 (should work since task2 doesn't exist yet)
    let result1 = task_manager.submit_task(task1).await;
    assert!(result1.is_ok());
    
    // Try to submit task2 (should fail due to circular dependency now that task1 exists)
    let result2 = task_manager.submit_task(task2).await;
    assert!(result2.is_err());
    assert!(result2.unwrap_err().to_string().contains("Circular dependency"));
}
