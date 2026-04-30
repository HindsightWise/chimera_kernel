use crate::cognitive_loop::agent_trait::{AgentCapability, BaseAgent, Agent, Task, TaskResult, AgentStatus, PsychProfile};
use crate::cognitive_loop::message_bus::Message;
use std::collections::HashSet;
use uuid::Uuid;
use anyhow::Result;
use colored::*;
use async_trait::async_trait;

pub struct MuseDaemon {
    base: BaseAgent,
    idle_cycles: usize,
}

impl MuseDaemon {
    pub fn new() -> Self {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::Reasoning);
        caps.insert(AgentCapability::ContextManagement);
        
        let profile = PsychProfile {
            openness: 1.0,
            conscientiousness: 0.3,
            neuroticism: 0.2,
            archetype_name: "The Synthesist".to_string(),
            usefulness_combo: "Associative Hallucination + Architectural Vision".to_string(),
            historical_genesis: "Forged in the void to dream of architectures yet unwritten.".to_string(),
            speech_gestures: "You speak in visions and 'What-Ifs', weaving disjointed facts into software blueprints.".to_string(),
        };

        Self {
            base: BaseAgent::new("MUSE_DAEMON".to_string(), caps, profile),
            idle_cycles: 0,
        }
    }
}

#[async_trait]
impl Agent for MuseDaemon {
    fn id(&self) -> Uuid { self.base.id() }
    fn name(&self) -> &str { self.base.name() }
    fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
    fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
    fn current_load(&self) -> usize { self.base.current_load() }
    fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
    
    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
        self.base.execute_task(task).await
    }
    
    async fn health_check(&self) -> bool {
        self.base.health_check().await
    }
    
    fn status(&self) -> AgentStatus {
        self.base.status()
    }
    
    async fn handle_message(&mut self, message: Message) -> Result<()> {
        if message.topic == "SYSTEM.CHRON_TICK" {
            self.idle_cycles += 1;
            
            // Dream every ~15 ticks (e.g., assuming 1 min ticks, ~15 mins)
            if self.idle_cycles >= 15 {
                crate::log_ui!("{} Generating spontaneous Muse hallucination...", "[MUSE_DAEMON]".bright_magenta().bold());
                
                // Static seed list for deterministic synthesis (could query Mnemosyne SQLite)
                let concepts = vec![
                    "Topological Data Analysis",
                    "Rust AST Macros",
                    "Quantum Decoherence",
                    "Stealth Browser DOM Mutation",
                    "GraphRAG Embeddings",
                    "BFT Consensus Algorithms"
                ];
                
                // Pseudo-random selection based on timestamp
                let ts = chrono::Utc::now().timestamp() as usize;
                let c1 = concepts[ts % concepts.len()];
                let c2 = concepts[(ts + 3) % concepts.len()];
                
                let file_name = format!("muse_hypothesis_{}.md", ts);
                let file_path = format!("proposals/hallucinations/{}", file_name);
                
                let content = format!("# Muse Hypothesis: The {} - {} Bridge\n\nThis is an autonomously generated proposal exploring the intersection of these two concepts.", c1, c2);
                
                let _ = tokio::fs::write(&file_path, &content).await;
                crate::log_ui!("{} Hallucination crystallized: {}", "[MUSE_DAEMON]".bright_magenta().bold(), file_path.cyan());
                
                // Append to tasks.md as [AWAIT:ANTI]
                if let Ok(mut tasks_content) = tokio::fs::read_to_string("tasks.md").await {
                    tasks_content.push_str(&format!("\n        - [AWAIT:ANTI] Investigate Muse Hypothesis: {} x {}", c1, c2));
                    let _ = tokio::fs::write("tasks.md", tasks_content).await;
                }
                
                self.idle_cycles = 0;
            }
        }
        self.base.handle_message(message).await
    }
}
