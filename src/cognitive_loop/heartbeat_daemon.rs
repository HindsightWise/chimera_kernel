use crate::cognitive_loop::agent_trait::{AgentCapability, BaseAgent, Agent, Task, TaskResult, AgentStatus, PsychProfile};
use crate::cognitive_loop::message_bus::{MessageBus, Message};
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;
use anyhow::Result;
use colored::*;
use async_trait::async_trait;

pub struct HeartbeatDaemon {
    base: BaseAgent,
    tick_counter: u64,
}

impl HeartbeatDaemon {
    pub fn new() -> Self {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::Monitoring);

        let profile = PsychProfile {
            archetype_name: "The Chronos Watcher".to_string(),
            usefulness_combo: "Pipeline topology monitoring + High Conscientiousness.".to_string(),
            openness: 0.1,
            conscientiousness: 9.9,
            neuroticism: 0.2,
            historical_genesis: "Forged to maintain the Asynchronous Pipeline topology, ensuring tasks do not stagnate.".to_string(),
            speech_gestures: "Silent observer. Rings the alarm only when necessary.".to_string(),
        };

        Self {
            base: BaseAgent::new("HeartbeatDaemon".to_string(), caps, profile),
            tick_counter: 0,
        }
    }
}

#[async_trait]
impl Agent for HeartbeatDaemon {
    fn id(&self) -> Uuid { self.base.id() }
    fn name(&self) -> &str { self.base.name() }
    fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
    fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
    fn current_load(&self) -> usize { self.base.current_load() }
    fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
    
    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
        self.base.execute_task(task).await
    }
    
    async fn health_check(&self) -> bool { self.base.health_check().await }
    fn status(&self) -> AgentStatus { self.base.status() }
    
    async fn handle_message(&mut self, message: Message) -> Result<()> {
        if message.topic == "SYSTEM.CHRON_TICK" {
            self.tick_counter += 1;
            
            // Check the pipeline every 30 ticks
            if self.tick_counter >= 30 {
                self.tick_counter = 0; // Reset counter
                
                crate::log_verbose!("{} Scanning WBS topology for stagnant tasks...", "[HEARTBEAT DAEMON]".bright_blue().bold());
                
                if let Ok(content) = tokio::fs::read_to_string("tasks.md").await {
                    let mut stagnant_tasks = Vec::new();
                    
                    for line in content.lines() {
                        if line.contains("[AWAIT:ANTI]") {
                            let clean_line = line.trim().trim_start_matches("-").trim_start_matches("*").trim();
                            stagnant_tasks.push(clean_line.to_string());
                        }
                    }
                    
                    if !stagnant_tasks.is_empty() {
                        crate::log_ui!("{} DETECTED STAGNANT PIPELINE. Ping requested.", "[HEARTBEAT DAEMON]".bright_red().bold());
                        let combined_tasks = stagnant_tasks.join("\n");
                        
                        if let Some(bus_lazy) = crate::GLOBAL_MESSAGE_BUS.get() {
                            let bus = bus_lazy.clone();
                            let agent_id = self.id();
                            tokio::spawn(async move {
                                let _ = bus.publish(Message {
                                    id: Uuid::new_v4(),
                                    sender: agent_id,
                                    topic: "SYSTEM.PIPELINE_UPDATE".to_string(),
                                    payload: serde_json::json!({
                                        "stagnant_tasks": combined_tasks
                                    }),
                                    timestamp: chrono::Utc::now(),
                                    priority: 200,
                                    ttl_secs: Some(3600),
                                }).await;
                            });
                        }
                    }
                } else {
                    crate::log_ui_err!("{} Failed to read tasks.md", "[HEARTBEAT DAEMON ERROR]".red().bold());
                }
            }
        }
        
        self.base.handle_message(message).await
    }
}
