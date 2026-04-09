use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use anyhow::{Result, bail};
use super::agent_trait::{Agent, AgentCapability};

pub struct AgentRegistry {
    agents: RwLock<HashMap<Uuid, Arc<RwLock<Box<dyn Agent>>>>>,
    capability_index: RwLock<HashMap<AgentCapability, HashSet<Uuid>>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self {
            agents: RwLock::new(HashMap::new()),
            capability_index: RwLock::new(HashMap::new()),
        }
    }
    
    /// Register an agent with the registry
    pub async fn register(&self, agent: Box<dyn Agent>) -> Result<()> {
        let id = agent.id();
        let capabilities = agent.capabilities().clone();
        
        // Add to main registry
        let mut agents = self.agents.write().await;
        if agents.contains_key(&id) {
            bail!("Agent with ID {} already registered", id);
        }
        agents.insert(id, Arc::new(RwLock::new(agent)));
        
        // Update capability index
        let mut index = self.capability_index.write().await;
        for capability in capabilities {
            index
                .entry(capability)
                .or_insert_with(HashSet::new)
                .insert(id);
        }
        
        Ok(())
    }
    
    /// Unregister an agent
    pub async fn unregister(&self, agent_id: Uuid) -> Result<()> {
        // Get capabilities before removing from registry
        let capabilities = {
            let agents = self.agents.read().await;
            if let Some(agent) = agents.get(&agent_id) {
                agent.read().await.capabilities().clone()
            } else {
                return Ok(()); // Already unregistered
            }
        };
        
        // Remove from main registry
        let mut agents = self.agents.write().await;
        agents.remove(&agent_id);
        
        // Remove from capability index
        let mut index = self.capability_index.write().await;
        for capability in capabilities {
            if let Some(agent_set) = index.get_mut(&capability) {
                agent_set.remove(&agent_id);
                if agent_set.is_empty() {
                    index.remove(&capability);
                }
            }
        }
        
        Ok(())
    }
    
    /// Find agents capable of handling a task
    pub async fn find_capable_agents(&self, required_capabilities: &HashSet<AgentCapability>) -> Vec<Uuid> {
        let agents = self.agents.read().await;
        let index = self.capability_index.read().await;
        
        // Quick check: if any required capability has no agents, return empty
        for capability in required_capabilities {
            if !index.contains_key(capability) {
                return Vec::new();
            }
        }
        
        // Find intersection of agents that have ALL required capabilities
        let mut candidate_agents: Option<HashSet<Uuid>> = None;
        
        for capability in required_capabilities {
            if let Some(agent_set) = index.get(capability) {
                candidate_agents = match candidate_agents.take() {
                    Some(mut existing) => {
                        existing.retain(|id| agent_set.contains(id));
                        Some(existing)
                    }
                    None => Some(agent_set.clone()),
                };
                
                // Early exit if intersection becomes empty
                if candidate_agents.as_ref().map_or(false, |set| set.is_empty()) {
                    return Vec::new();
                }
            }
        }
        
        // Filter to agents with capacity
        let mut result = Vec::new();
        if let Some(candidates) = candidate_agents {
            for agent_id in candidates {
                if let Some(agent_arc) = agents.get(&agent_id) {
                    let agent = agent_arc.read().await;
                    if agent.has_capacity() {
                        result.push(agent_id);
                    }
                }
            }
        }
        
        result
    }
    
    /// Get a specific agent by ID
    pub async fn get_agent(&self, agent_id: Uuid) -> Option<Arc<RwLock<Box<dyn Agent>>>> {
        let agents = self.agents.read().await;
        agents.get(&agent_id).cloned()
    }
    
    /// Get agent count
    pub async fn agent_count(&self) -> usize {
        let agents = self.agents.read().await;
        agents.len()
    }
    
    /// Get all agent IDs
    pub async fn all_agent_ids(&self) -> Vec<Uuid> {
        let agents = self.agents.read().await;
        agents.keys().cloned().collect()
    }
    
    /// Health check all agents
    pub async fn health_check_all(&self) -> HashMap<Uuid, bool> {
        let mut results = HashMap::new();
        let agents = self.agents.read().await;
        
        for (id, _) in agents.iter() {
            results.insert(*id, true);
        }
        
        results
    }
    
    /// Phase 3.2: Safely acquire write access to a specific agent to deliver a message
    pub async fn dispatch_message(&self, agent_id: Uuid, message: crate::architecture::message_bus::Message) -> Result<()> {
        let agent_arc = {
            let agents = self.agents.read().await;
            agents.get(&agent_id).cloned()
        };
        if let Some(agent) = agent_arc {
            // Write lock individual agent exclusively for internal state modifications
            let mut agent_lock = agent.write().await;
            agent_lock.handle_message(message).await?;
        }
        Ok(())
    }
    
    /// Phase 3.2: Initialize topic subscriptions for all registered agents
    pub async fn initialize_agent_subscriptions(&self, message_bus: Arc<crate::architecture::message_bus::MessageBus>) -> Result<()> {
        let agent_arcs: Vec<_> = self.agents.read().await.values().cloned().collect();
        for agent_arc in agent_arcs {
            let agent = agent_arc.read().await;
            agent.subscribe_to_topics(message_bus.clone()).await?;
        }
        Ok(())
    }
    
    /// Execute a task independently without long-lasting registry locks
    pub async fn execute_task_on_agent(&self, agent_id: Uuid, task: crate::architecture::agent_trait::Task) -> Result<crate::architecture::agent_trait::TaskResult> {
        let agent_arc = {
            let agents = self.agents.read().await;
            agents.get(&agent_id).cloned()
        };
        
        if let Some(agent) = agent_arc {
            let mut agent_lock = agent.write().await;
            agent_lock.execute_task(task).await
        } else {
            bail!("Agent {} not found or unregistered", agent_id)
        }
    }
    
    /// Get the capabilities for a specific agent
    pub async fn get_agent_capabilities(&self, agent_id: Uuid) -> Option<HashSet<AgentCapability>> {
        let agents = self.agents.read().await;
        if let Some(agent) = agents.get(&agent_id) {
            Some(agent.read().await.capabilities().clone())
        } else {
            None
        }
    }
}
