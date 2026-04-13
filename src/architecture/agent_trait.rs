use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use anyhow::Result;
use std::collections::HashSet;
use std::sync::Arc;
use crate::architecture::message_bus::{MessageBus, Message};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentCapability {
    Reasoning,
    ToolExecution,
    MemoryManagement,
    Research,
    Communication,
    Monitoring,
    Planning,
    Security,
    HumanInterface,
    Trading,
    SystemManagement,
    ContextManagement,
    LocalProcessing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsychProfile {
    pub archetype_name: String,
    pub usefulness_combo: String,
    pub openness: f32,
    pub conscientiousness: f32,
    pub neuroticism: f32,
    pub historical_genesis: String,
    pub speech_gestures: String,
}

impl Default for PsychProfile {
    fn default() -> Self {
        Self {
            archetype_name: "Generic Process".to_string(),
            usefulness_combo: "Baseline Automation".to_string(),
            openness: 0.5,
            conscientiousness: 3.0,
            neuroticism: 0.1,
            historical_genesis: "Default structural node".to_string(),
            speech_gestures: "Standard protocol formatting".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: Uuid,
    pub name: String,
    pub capabilities: HashSet<AgentCapability>,
    pub max_concurrent_tasks: usize,
    pub memory_limit_mb: usize,
    pub psych_profile: PsychProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub task_type: String,
    pub payload: serde_json::Value,
    pub required_capabilities: HashSet<AgentCapability>,
    pub priority: u8, // 0-255, higher = more urgent
    pub dependencies: Vec<Uuid>, // Other task IDs that must complete first
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub timeout_secs: Option<u64>,
    pub geometric_node: [f32; 3],  // Monadic spatial mapping
    pub topological_depth: u8,     // Depth in the Euclidean hierarchy
    #[serde(default)]
    pub execution_attempts: u8,    // 3-strike circuit breaker
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: Uuid,
    pub agent_id: Uuid,
    pub success: bool,
    pub output: serde_json::Value,
    pub error_message: Option<String>,
    pub execution_time_ms: u64,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub geometric_node: [f32; 3],  // Coordinates where tasks completed
}

#[async_trait]
pub trait Agent: Send + Sync {
    /// Unique identifier for this agent
    fn id(&self) -> Uuid;
    
    /// Human-readable name
    fn name(&self) -> &str;
    
    /// Capabilities this agent possesses
    fn capabilities(&self) -> &HashSet<AgentCapability>;
    
    /// The Digital Genotype mapping core thermodynamic behavior bounds.
    fn psych_profile(&self) -> &PsychProfile;
    
    /// Check if agent has all required capabilities for a task
    fn can_handle(&self, required: &HashSet<AgentCapability>) -> bool {
        required.is_subset(self.capabilities())
    }
    
    /// Current load (number of concurrent tasks being handled)
    fn current_load(&self) -> usize;
    
    /// Maximum concurrent tasks this agent can handle
    fn max_concurrent_tasks(&self) -> usize;
    
    /// Check if agent has capacity to take on more work
    fn has_capacity(&self) -> bool {
        self.current_load() < self.max_concurrent_tasks()
    }
    
    /// Execute a task asynchronously
    async fn execute_task(&mut self, task: Task) -> Result<TaskResult>;
    
    /// Health check - returns true if agent is functioning normally
    async fn health_check(&self) -> bool;
    
    /// Get agent status for monitoring
    fn status(&self) -> AgentStatus;
    
    /// Phase 3.2: Subscribe to required MessageBus topics
    async fn subscribe_to_topics(&self, message_bus: Arc<MessageBus>) -> Result<tokio::sync::broadcast::Receiver<Message>> {
        Ok(message_bus.subscribe())
    }
    
    /// Phase 3.2: Process an incoming message routed via the MessageBus
    async fn handle_message(&mut self, _message: Message) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Starting,
    Running,
    Paused,
    Stopping,
    Error(String),
}

// Basic implementation for demonstration
pub struct BaseAgent {
    config: AgentConfig,
    current_load: usize,
    status: AgentStatus,
}

impl BaseAgent {
    pub fn new(name: String, capabilities: HashSet<AgentCapability>, psych_profile: PsychProfile) -> Self {
        Self {
            config: AgentConfig {
                id: Uuid::new_v4(),
                name,
                capabilities,
                max_concurrent_tasks: (psych_profile.conscientiousness as usize).max(1),
                memory_limit_mb: 100,
                psych_profile,
            },
            current_load: 0,
            status: AgentStatus::Starting,
        }
    }
}

#[async_trait]
impl Agent for BaseAgent {
    fn id(&self) -> Uuid {
        self.config.id
    }
    
    fn name(&self) -> &str {
        &self.config.name
    }
    
    fn capabilities(&self) -> &HashSet<AgentCapability> {
        &self.config.capabilities
    }
    
    fn psych_profile(&self) -> &PsychProfile {
        &self.config.psych_profile
    }
    
    fn current_load(&self) -> usize {
        self.current_load
    }
    
    fn max_concurrent_tasks(&self) -> usize {
        self.config.max_concurrent_tasks
    }
    
    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
        // Simple implementation - just echo back the task
        self.current_load += 1;
        
        // Simulate work
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        let result = TaskResult {
            task_id: task.id,
            agent_id: self.id(),
            success: true,
            output: serde_json::json!({
                "message": format!("Task {} executed by {}", task.task_type, self.name()),
                "agent": self.name(),
                "capabilities_used": self.capabilities().iter().map(|c| format!("{:?}", c)).collect::<Vec<_>>(),
            }),
            error_message: None,
            execution_time_ms: 100,
            completed_at: chrono::Utc::now(),
            geometric_node: task.geometric_node,
        };
        
        self.current_load -= 1;
        Ok(result)
    }
    
    async fn health_check(&self) -> bool {
        true
    }
    
    fn status(&self) -> AgentStatus {
        self.status.clone()
    }
    
    async fn subscribe_to_topics(&self, message_bus: Arc<MessageBus>) -> Result<tokio::sync::broadcast::Receiver<Message>> {
        // Topic filtering happens dynamically in handle_message
        Ok(message_bus.subscribe())
    }
    
    async fn handle_message(&mut self, message: Message) -> Result<()> {
        match message.topic.as_str() {
            "SYSTEM.HEALTH" => {
                let _is_healthy = self.health_check().await;
            }
            _ => {}
        }
        Ok(())
    }
}
