use std::collections::{HashMap, VecDeque, HashSet};
use tokio::sync::RwLock;
use uuid::Uuid;
use anyhow::{Result, bail};
use chrono::{DateTime, Utc};
use super::agent_trait::{AgentCapability, Task, TaskResult};

pub struct TaskManager {
    pending_tasks: RwLock<VecDeque<Task>>,
    running_tasks: RwLock<HashMap<Uuid, (Uuid, DateTime<Utc>)>>, // task_id -> (agent_id, started_at)
    completed_tasks: RwLock<VecDeque<TaskResult>>,
    failed_tasks: RwLock<VecDeque<(Task, String)>>, // (task, error_message)
    max_history_size: usize,
}

impl TaskManager {
    pub fn new(max_history_size: usize) -> Self {
        Self {
            pending_tasks: RwLock::new(VecDeque::new()),
            running_tasks: RwLock::new(HashMap::new()),
            completed_tasks: RwLock::new(VecDeque::with_capacity(max_history_size)),
            failed_tasks: RwLock::new(VecDeque::with_capacity(max_history_size)),
            max_history_size,
        }
    }
    
    /// Submit a new task for execution
    pub async fn submit_task(&self, task: Task) -> Result<()> {
        let mut pending = self.pending_tasks.write().await;
        
        // Check for circular dependencies
        if self.has_circular_dependency(&task, &pending).await {
            bail!("Circular dependency detected in task {}", task.id);
        }
        
        // Insert based on priority (higher priority first)
        let mut inserted = false;
        let task_clone = task.clone(); // Clone for potential insertion
        
        for i in 0..pending.len() {
            if pending[i].priority < task_clone.priority {
                pending.insert(i, task_clone);
                inserted = true;
                break;
            }
        }
        
        if !inserted {
            pending.push_back(task);
        }
        
        Ok(())
    }
    
    /// Check for circular dependencies
    async fn has_circular_dependency(&self, task: &Task, pending: &VecDeque<Task>) -> bool {
        let mut visited = HashSet::new();
        visited.insert(task.id);
        
        let mut to_check = task.dependencies.clone();
        
        while let Some(dep_id) = to_check.pop() {
            if visited.contains(&dep_id) {
                return true; // Circular dependency
            }
            visited.insert(dep_id);
            
            // Check if dependency is in pending tasks
            for pending_task in pending.iter() {
                if pending_task.id == dep_id {
                    to_check.extend(pending_task.dependencies.clone());
                }
            }
            
            // TODO: Also check running and completed tasks
        }
        
        false
    }
    
    /// Get next available task that has all dependencies satisfied
    pub async fn get_next_task(&self, agent_capabilities: &HashSet<AgentCapability>) -> Option<Task> {
        let mut pending = self.pending_tasks.write().await;
        let _running = self.running_tasks.read().await; // Keep for consistency
        let completed = self.completed_tasks.read().await;
        
        for i in 0..pending.len() {
            let task = &pending[i];
            
            // Check capabilities
            if !agent_capabilities.is_superset(&task.required_capabilities) {
                continue;
            }
            
            // Check dependencies
            let mut dependencies_satisfied = true;
            for dep_id in &task.dependencies {
                // Check if dependency is completed
                let mut found = false;
                for completed_task in completed.iter() {
                    if completed_task.task_id == *dep_id && completed_task.success {
                        found = true;
                        break;
                    }
                }
                
                // Check if dependency is still running or pending
                if !found {
                    // It might still be pending or failed
                    // For now, we'll assume unsatisfied
                    dependencies_satisfied = false;
                    break;
                }
            }
            
            if dependencies_satisfied {
                return pending.remove(i);
            }
        }
        
        None
    }
    
    /// Mark a task as started by an agent
    pub async fn start_task(&self, task_id: Uuid, agent_id: Uuid) -> Result<()> {
        let mut running = self.running_tasks.write().await;
        
        if running.contains_key(&task_id) {
            bail!("Task {} is already running", task_id);
        }
        
        running.insert(task_id, (agent_id, Utc::now()));
        Ok(())
    }
    
    /// Mark a task as completed
    pub async fn complete_task(&self, result: TaskResult) -> Result<()> {
        let mut running = self.running_tasks.write().await;
        let mut completed = self.completed_tasks.write().await;
        
        // Remove from running tasks
        running.remove(&result.task_id);
        
        // Add to completed history
        completed.push_back(result);
        if completed.len() > self.max_history_size {
            completed.pop_front();
        }
        
        Ok(())
    }
    
    /// Mark a task as failed
    pub async fn fail_task(&self, mut task: Task, error_message: String) -> Result<bool> {
        let mut running = self.running_tasks.write().await;
        let mut failed = self.failed_tasks.write().await;
        
        // Remove from running tasks
        running.remove(&task.id);
        
        task.execution_attempts += 1;
        
        if task.execution_attempts >= 3 {
            // Add to failed history (Permanent Failure)
            failed.push_back((task.clone(), error_message.clone()));
            if failed.len() > self.max_history_size {
                failed.pop_front();
            }
            
            // Log topologically
            if let Some(pipe) = crate::architecture::GLOBAL_MEM_PIPELINE.get() {
                let mut mp = pipe.lock().await;
                let _ = mp.store_working(
                    format!("CRITICAL TASK FAILURE (3 STRIKES): {}. Error: {}", task.task_type, error_message),
                    1.0, // topological_stress 1.0 (importance)
                    0.0, // uncertainty
                    false // is_hostile
                );
            }
            
            Ok(true) // Signifies permanent failure
        } else {
            // Requeue the task for another attempt
            let mut pending = self.pending_tasks.write().await;
            pending.push_front(task);
            Ok(false) // Signifies retried
        }
    }
    
    /// Get task status
    pub async fn get_task_status(&self, task_id: Uuid) -> Option<TaskStatus> {
        let pending = self.pending_tasks.read().await;
        let running = self.running_tasks.read().await;
        let completed = self.completed_tasks.read().await;
        let failed = self.failed_tasks.read().await;
        
        // Check pending
        for task in pending.iter() {
            if task.id == task_id {
                return Some(TaskStatus::Pending);
            }
        }
        
        // Check running
        if running.contains_key(&task_id) {
            return Some(TaskStatus::Running);
        }
        
        // Check completed
        for result in completed.iter() {
            if result.task_id == task_id {
                return Some(TaskStatus::Completed(result.success));
            }
        }
        
        // Check failed
        for (task, _) in failed.iter() {
            if task.id == task_id {
                return Some(TaskStatus::Failed);
            }
        }
        
        None
    }
    
    /// Get statistics
    pub async fn get_stats(&self) -> TaskManagerStats {
        let pending = self.pending_tasks.read().await;
        let running = self.running_tasks.read().await;
        let completed = self.completed_tasks.read().await;
        let failed = self.failed_tasks.read().await;
        
        TaskManagerStats {
            pending_count: pending.len(),
            running_count: running.len(),
            completed_count: completed.len(),
            failed_count: failed.len(),
            success_rate: if completed.len() + failed.len() > 0 {
                completed.len() as f32 / (completed.len() + failed.len()) as f32
            } else {
                0.0
            },
        }
    }
    
    /// Clean up old completed/failed tasks
    pub async fn cleanup_old_tasks(&self, max_age_hours: u32) -> usize {
        let now = Utc::now();
        let cutoff = now - chrono::Duration::hours(max_age_hours as i64);
        
        let mut removed = 0;
        
        // Clean completed tasks
        let mut completed = self.completed_tasks.write().await;
        completed.retain(|result| result.completed_at > cutoff);
        removed += self.max_history_size - completed.len();
        
        // Clean failed tasks
        let mut failed = self.failed_tasks.write().await;
        // Note: Failed tasks store (Task, error), we don't have timestamp
        // For now, we'll just limit by max_history_size
        while failed.len() > self.max_history_size {
            failed.pop_front();
            removed += 1;
        }
        
        removed
    }
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed(bool), // success
    Failed,
}

#[derive(Debug, Clone)]
pub struct TaskManagerStats {
    pub pending_count: usize,
    pub running_count: usize,
    pub completed_count: usize,
    pub failed_count: usize,
    pub success_rate: f32,
}
