use crate::architecture::{AgentRegistry, MessageBus, TaskManager, TaskDecomposer, AgentCoordinator};
use crate::architecture::specialized_agents::SpecializedAgentFactory;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use colored::*;

pub struct MultiAgentKernel {
    pub registry: Arc<RwLock<AgentRegistry>>,
    pub message_bus: Arc<MessageBus>,
    pub task_manager: Arc<RwLock<TaskManager>>,
    pub task_decomposer: Arc<TaskDecomposer>,
    pub agent_coordinator: Arc<AgentCoordinator>,
}

impl MultiAgentKernel {
    pub async fn new() -> Self {
        let registry = Arc::new(RwLock::new(AgentRegistry::new()));
        let message_bus = Arc::new(MessageBus::new(100)); // Default capacity
        
        // TaskManager only needs max_history_size, not registry/message_bus
        // Coordination happens through separate dispatcher pattern
        let task_manager = Arc::new(RwLock::new(TaskManager::new(1000))); // Store up to 1000 task results
        let task_decomposer = Arc::new(TaskDecomposer::new());
        let agent_coordinator = Arc::new(AgentCoordinator::new());
        
        let kernel = Self {
            registry: registry.clone(),
            message_bus: message_bus.clone(),
            task_manager,
            task_decomposer,
            agent_coordinator,
        };
        
        // Stage 1: Load and register sovereign multi-agent backbone components
        kernel.initialize_agents().await;
        
        // Phase 3.2: Initialize Cerebrospinal Synapses (Agent message topic subscriptions)
        kernel.initialize_subscriptions().await;
        
        kernel
    }

    pub async fn initialize_agents(&self) {
        let reg_lock = self.registry.write().await;
        let instantiated_agents = SpecializedAgentFactory::instantiate_all();
        
        for agent in instantiated_agents {
            let _ = reg_lock.register(agent).await; // Note! Added await because register is async
        }
    }
    
    pub async fn initialize_subscriptions(&self) {
        let _ = self.registry.read().await.initialize_agent_subscriptions(self.message_bus.clone()).await;
    }
    
    // In Stage 1 Migration, we expose a simple run function that runs alongside `run_kernel_loop`
    pub async fn spawn_background_coordination(&self) {
        // Phase 3.4: Swarm Dispatch & Execution Engine
        let task_mgr = self.task_manager.clone();
        let dispatch_registry = self.registry.clone();
        let dispatch_bus = self.message_bus.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(1000));
            loop {
                interval.tick().await;
                
                // 1. Get all agent IDs
                let agent_ids = dispatch_registry.read().await.all_agent_ids().await;
                
                for agent_id in agent_ids {
                    // With Actors, MPSC channels handle buffer capacity seamlessly.
                    // We dispatch eagerly based on capability, and the Actor loop parses it sequentially.
                    let caps = dispatch_registry.read().await.get_agent_capabilities(agent_id).await.unwrap_or_default();
                            
                            // Ask TaskManager for a matching task
                            if let Some(task) = task_mgr.write().await.get_next_task(&caps).await {
                                // Mark as started immediately
                                if let Ok(_) = task_mgr.write().await.start_task(task.id, agent_id).await {
                                    crate::log_ui!("{} ASSIGNING TASK {} -> {}", "[DISPATCH]".bright_cyan().bold(), task.task_type.yellow(), agent_id);
                                    
                                    // 1. Broadcast Subtask Assignment to Coordinator
                                    let _ = dispatch_bus.publish(crate::architecture::message_bus::Message {
                                        id: uuid::Uuid::new_v4(),
                                        sender: agent_id,
                                        topic: "SYSTEM.SUBTASK_ASSIGNED".to_string(),
                                        payload: serde_json::json!({
                                            "subtask_id": task.id.to_string(),
                                            "agent_id": agent_id.to_string()
                                        }),
                                        timestamp: chrono::Utc::now(),
                                        priority: 128,
                                        ttl_secs: Some(3600),
                                    }).await;
                                    
                                    // Spawn execution to run in parallel
                                    let exec_registry = dispatch_registry.clone();
                                    let exec_tm = task_mgr.clone();
                                    let exec_bus = dispatch_bus.clone();
                                    let task_clone = task.clone();
                                    
                                    tokio::spawn(async move {
                                        match exec_registry.read().await.execute_task_on_agent(agent_id, task_clone.clone()).await {
                                            Ok(result) => {
                                                crate::log_ui!("{} TASK {}: {}", "[EXECUTOR]".bright_green().bold(), task_clone.task_type.green(), if result.success { "COMPLETED" } else { "FAILED" });
                                                let _ = exec_tm.write().await.complete_task(result.clone()).await;
                                                
                                                // Broadcast Telemetry
                                                let _ = exec_bus.publish(crate::architecture::message_bus::Message {
                                                    id: uuid::Uuid::new_v4(),
                                                    sender: agent_id,
                                                    topic: "SYSTEM.TASK_COMPLETE".to_string(),
                                                    payload: serde_json::json!({
                                                        "task": task_clone,
                                                        "result": result
                                                    }),
                                                    timestamp: chrono::Utc::now(),
                                                    priority: 64,
                                                    ttl_secs: Some(3600),
                                                }).await;
                                                
                                                // 2. Broadcast Subtask Completion to Coordinator
                                                let sub_topic = if result.success { "SYSTEM.SUBTASK_COMPLETED" } else { "SYSTEM.SUBTASK_FAILED" };
                                                let _ = exec_bus.publish(crate::architecture::message_bus::Message {
                                                    id: uuid::Uuid::new_v4(),
                                                    sender: agent_id,
                                                    topic: sub_topic.to_string(),
                                                    payload: serde_json::json!({
                                                        "subtask_id": task_clone.id.to_string(),
                                                        "result": result
                                                    }),
                                                    timestamp: chrono::Utc::now(),
                                                    priority: 128,
                                                    ttl_secs: Some(3600),
                                                }).await;
                                            },
                                            Err(e) => {
                                                crate::log_ui_err!("{} TASK CRASHED (Attempt {}): {}", "[EXECUTOR]".bright_red().bold(), task_clone.execution_attempts + 1, e);
                                                let is_permanent = exec_tm.write().await.fail_task(task_clone.clone(), e.to_string())
                                                    .await
                                                    .unwrap_or(true);
                                                if is_permanent {
                                                    // 3. Broadcast Subtask Failure to Coordinator on Crash
                                                    crate::log_ui_err!("{} 3-STRIKE CIRCUIT BREAKER TRIPPED. Task {} permanently abandoning.", "[CRITICAL]".red().bold(), task_clone.id);
                                                    let _ = exec_bus.publish(crate::architecture::message_bus::Message {
                                                        id: uuid::Uuid::new_v4(),
                                                        sender: agent_id,
                                                        topic: "SYSTEM.SUBTASK_FAILED".to_string(),
                                                        payload: serde_json::json!({
                                                            "subtask_id": task_clone.id.to_string()
                                                        }),
                                                        timestamp: chrono::Utc::now(),
                                                        priority: 128,
                                                        ttl_secs: Some(3600),
                                                    }).await;
                                                    
                                                    // ANTI-CASCADE QUARANTINE TRIGGER
                                                    // If three sequential strikes occur locally, trigger a rollback lock to prevent ASI08 fault propagation
                                                    crate::log_ui_err!("{} {} {}", "[CIRCUIT BREAKER]".bright_magenta().bold(), "ASI08 Fault propagation blocked.", "Initiating context rollback to Mnemosyne baseline.");
                                                }
                                            }
                                        }
                                    });
                                }
                            }
                }
            }
        });

        // Phase 3.2: The Central Dispatcher (MessagePump)
        // Continuously polls the MessageBus queues and dispenses messages to agent receptors
        let _pump_registry = self.registry.clone();
        let pump_bus = self.message_bus.clone();
        let kernel_task_manager = self.task_manager.clone();
        let kernel_decomposer = self.task_decomposer.clone();
        let publish_bus = self.message_bus.clone();
        
        let mut rx = pump_bus.subscribe();
        
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(msg) => {
                        if msg.topic == "SYSTEM.NEW_TASK" {
                            if let Ok(task) = serde_json::from_value::<crate::architecture::agent_trait::Task>(msg.payload) {
                                let instruction = task.payload.get("instruction").and_then(|v| v.as_str()).unwrap_or("");
                                
                                // Route instruction through TaskDecomposer
                                let subtasks = kernel_decomposer.decompose(instruction, task.priority).await;
                                
                                if subtasks.len() > 1 {
                                    crate::log_ui!("{} Shattered task into {} subtasks [{}]", "[DECOMPOSER]".bright_purple().bold(), subtasks.len(), instruction.bright_black());
                                    
                                    let mut child_jsons = Vec::new();
                                    for mut sub_task in subtasks {
                                        // CHECKER EXECUTOR ENFORCEMENT: All deep decomposed tasks must pass Verification Quorum
                                        if sub_task.topological_depth > 1 {
                                            sub_task.required_capabilities.insert(crate::architecture::agent_trait::AgentCapability::Reasoning);
                                        }
                                        
                                        child_jsons.push(serde_json::json!({
                                            "id": sub_task.id.to_string(),
                                            "type": sub_task.task_type
                                        }));
                                        let _ = kernel_task_manager.write().await.submit_task(sub_task).await;
                                    }
                                    
                                    // Broadcast the Complex Topological Trace Graph
                                    let _ = publish_bus.publish(crate::architecture::message_bus::Message {
                                        id: uuid::Uuid::new_v4(),
                                        sender: uuid::Uuid::default(),
                                        topic: "SYSTEM.COMPLEX_TASK_STARTED".to_string(),
                                        payload: serde_json::json!({
                                            "parent_id": task.id.to_string(),
                                            "subtasks": child_jsons
                                        }),
                                        timestamp: chrono::Utc::now(),
                                        priority: 200,
                                        ttl_secs: Some(3600),
                                    }).await;
                                } else if let Some(single_task) = subtasks.into_iter().next() {
                                    // Just a normal monotonic task, pass it directly
                                    let _ = kernel_task_manager.write().await.submit_task(single_task).await;
                                    crate::log_ui!("{}", "[KERNEL] Task Ingested to Queue".bright_blue().bold());
                                }
                            }
                        } else if msg.topic == "SYSTEM.AEGIS_QUARANTINE" {
                            if let Ok(data) = serde_json::from_value::<serde_json::Value>(msg.payload) {
                                let content = data.get("content").and_then(|v| v.as_str()).unwrap_or("");
                                let source = data.get("source").and_then(|v| v.as_str()).unwrap_or("UNKNOWN");
                                
                                if let Some(mem_pipeline) = crate::architecture::GLOBAL_MEM_PIPELINE.get() {
                                    let mut mp = mem_pipeline.lock().await;
                                    mp.store_working(
                                        format!("[HOSTILE PHENOMENON QUARANTINED]\nSource: {}\nData:\n{}", source, content),
                                        1.0, // High importance (threat)
                                        0.0, // Zero uncertainty (absolute threat)
                                        true // [is_hostile] Push out of Boundary R=3.0 to R=4.0
                                    ).await;
                                }
                            }
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        break;
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                        crate::log_ui_err!("Kernel Message Pump lagged and dropped {} messages!", skipped);
                    }
                }
            }
        });

        // Loop to broadcast Swarm Telemetry to Ghostty TUI
        let telemetry_mgr = self.task_manager.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(700));
            loop {
                interval.tick().await;
                let tm = telemetry_mgr.read().await;
                let stats = tm.get_stats().await;
                let payload = format!("[COLLECTIVE_TELEMETRY]{}|{}|{}|{}", stats.pending_count, stats.running_count, stats.completed_count, stats.failed_count);
                crate::log_ui!("{}", payload);
            }
        });
        
        // The Chronos Temporal Polling Daemon
        let chronos_bus = self.message_bus.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
            loop {
                interval.tick().await;
                if let Ok(graph) = crate::architecture::graph_rag::GraphMemoryManager::new("mnemosyne_graph.db") {
                    let current_unix = chrono::Utc::now().timestamp();
                    if let Ok(tasks) = graph.poll_chronos_tasks(current_unix).await {
                        for (_, payload, topic) in tasks {
                            crate::log_ui!("{}", "[CHRONOS] Temporal Anchor triggered! Injecting into Swarm...".bright_cyan().bold());
                            let _ = chronos_bus.publish(crate::architecture::message_bus::Message {
                                id: uuid::Uuid::new_v4(),
                                sender: uuid::Uuid::default(),
                                topic,
                                payload: serde_json::json!({"instruction": payload}),
                                timestamp: chrono::Utc::now(),
                                priority: 200,
                                ttl_secs: Some(3600),
                            }).await;
                        }
                    }
                }
            }
        });
        
        // Phase 4: Filesystem Sensoria (Proprioceptive File Watcher)
        let sensoria_bus = self.message_bus.clone();
        tokio::task::spawn_blocking(move || {
            use notify::{Watcher, RecursiveMode, EventKind};
            use std::sync::mpsc::channel;
            
            let (tx, rx) = channel();
            let mut watcher = match notify::recommended_watcher(tx) {
                Ok(w) => w,
                Err(e) => {
                    crate::log_ui_err!("{} Failed to initialize Sensoria Watcher: {}", "[SENSORIA]".red().bold(), e);
                    return;
                }
            };
            
            let mut target_dir = std::path::PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| String::from(".")));
            target_dir.push("Chimera_Ingest");
            if !target_dir.exists() {
                let _ = std::fs::create_dir_all(&target_dir);
            }
            
            if let Err(e) = watcher.watch(&target_dir, RecursiveMode::Recursive) {
                crate::log_ui_err!("{} Failed to attach watcher to {:?}: {}", "[SENSORIA]".red().bold(), target_dir, e);
                return;
            }
            
            crate::log_ui!("{} Attached Proprioceptive Watcher to: {:?}", "[SENSORIA]".bright_magenta().bold(), target_dir);
            
            for res in rx {
                if let Ok(event) = res {
                    if let EventKind::Create(_) | EventKind::Modify(_) = event.kind {
                        for path in event.paths {
                            if path.is_file() {
                                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                                    if !file_name.starts_with('.') {
                                        crate::log_ui!("{} Physical Ingest detected at: {:?}", "[SENSORIA]".cyan().bold(), path);
                                        let bs = sensoria_bus.clone();
                                        let path_str = path.to_string_lossy().to_string();
                                        tokio::spawn(async move {
                                            let _ = bs.publish(crate::architecture::message_bus::Message {
                                                id: uuid::Uuid::new_v4(),
                                                sender: uuid::Uuid::default(),
                                                topic: "SYSTEM.DREAM".to_string(),
                                                payload: serde_json::json!({
                                                    "instruction": format!("A new physical file was detected in the Sensoria ingest sector. Autonomously analyze and act on its contents: {}", path_str)
                                                }),
                                                timestamp: chrono::Utc::now(),
                                                priority: 220,
                                                ttl_secs: Some(3600),
                                            }).await;
                                        });
                                        std::thread::sleep(std::time::Duration::from_secs(3));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
        
        // Phase 3.7: Span central AgentCoordinator background tracking daemon
        let coordinator = self.agent_coordinator.clone();
        let coord_bus = self.message_bus.clone();
        tokio::spawn(async move {
            coordinator.run_coordinator_loop(coord_bus).await;
        });
        
        // Spawn the continuous autonomous Sensory Drift (Dream Cycle) routine with Supervisor Pattern
        let super_bus = self.message_bus.clone();
        tokio::spawn(async move {
            const MAX_ATTEMPTS: u32 = 5;
            const INITIAL_BACKOFF_SECS: u64 = 5;
            
            let mut attempt = 1;
            
            while attempt <= MAX_ATTEMPTS {
                crate::log_ui!("{} {} {} {} {}",
                    "[SUPERVISOR]".yellow().bold(),
                    format!("Starting Sensory Drift (Attempt {}/{})", attempt, MAX_ATTEMPTS).yellow(),
                    "with".dimmed(),
                    format!("{}s", INITIAL_BACKOFF_SECS * 2u64.pow(attempt.saturating_sub(1) as u32)).yellow(),
                    "restart delay".dimmed()
                );
                
                let drift_handle = tokio::spawn({
                    let inner_bus = super_bus.clone();
                    async move {
                        crate::architecture::sensory_drift::SensoryDrift::run_appetition_cycle(inner_bus).await;
                    }
                });
                
                match drift_handle.await {
                    Ok(_) => {
                        crate::log_ui!("{}", "[SUPERVISOR] Sensory Drift completed normally.".green().bold());
                        break;
                    }
                    Err(e) => {
                        crate::log_ui_err!("{} {}",
                            "[SUPERVISOR] Sensory Drift crashed:".red().bold(),
                            e);
                        
                        if attempt >= MAX_ATTEMPTS {
                            crate::log_ui_err!("{}", 
                                "[SUPERVISOR] Maximum restart attempts reached. Giving up.".red().bold());
                            break;
                        }
                        
                        // Exponential backoff: 5s, 10s, 20s, 40s, 80s
                        let backoff = Duration::from_secs(INITIAL_BACKOFF_SECS * 2u64.pow(attempt as u32 - 1));
                        crate::log_ui!("{} {}",
                            "[SUPERVISOR] Waiting".dimmed(),
                            format!("{:?}", backoff).yellow());
                        sleep(backoff).await;
                        
                        attempt += 1;
                    }
                }
            }
        });
    }
}
