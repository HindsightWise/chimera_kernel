use crate::architecture::agent_trait::{AgentCapability, BaseAgent, Agent, Task, TaskResult, AgentStatus};
use crate::architecture::message_bus::{MessageBus, Message};
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;
use anyhow::Result;
use colored::*;
use async_trait::async_trait;
use tokio::sync::OnceCell;
use chrono::Utc;
// --- PRIORITY 1: REASONING AGENT ---
pub struct ReasoningAgent {
    base: BaseAgent,
    hypothesis_buffer: Vec<String>,
    bus: Arc<OnceCell<Arc<MessageBus>>>,
}
#[async_trait]
impl Agent for ReasoningAgent {
    fn id(&self) -> Uuid { self.base.id() }
    fn name(&self) -> &str { self.base.name() }
    fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
    fn current_load(&self) -> usize { self.base.current_load() }
    fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> { self.base.execute_task(task).await }
    async fn health_check(&self) -> bool { self.base.health_check().await }
    fn status(&self) -> AgentStatus { self.base.status() }
    
    async fn subscribe_to_topics(&self, message_bus: Arc<MessageBus>) -> Result<tokio::sync::broadcast::Receiver<Message>> {
        let rx = self.base.subscribe_to_topics(message_bus.clone()).await?;
        let _ = self.bus.set(message_bus);
        Ok(rx)
    }
    
    async fn handle_message(&mut self, message: Message) -> Result<()> {
        if message.topic == "SYSTEM.DREAM" {
            crate::log_verbose!("{} RECEIVED CEREBROSPINAL FLUID. Invoking Subconscious...", "[REASONING AGENT]".purple().bold());
            self.hypothesis_buffer.push(message.payload.to_string());
            
            // Background dream-to-task synthesis
            if let Some(bus) = self.bus.get().cloned() {
                let payload_str = message.payload.to_string();
                let my_id = self.id();
                tokio::spawn(async move {
                    // Pre-phase: Native Rust Memory Recall
                    let mut historical_context = String::new();
                    if let Some(mem_pipeline) = crate::architecture::GLOBAL_MEM_PIPELINE.get() {
                        let mp = mem_pipeline.lock().await;
                        if let Some(db) = &mp.db_connection {
                            let encoded = crate::architecture::MemoryHierarchy::encode_spectral_embedding(&payload_str);
                            if let Ok(res_str) = db.search_vector(encoded, 3) {
                                crate::log_verbose!("{} NATIVE MEMORY RECALL INJECTED.", "[REASONING AGENT]".cyan().bold());
                                historical_context = format!("Historical Context from Mnemosyne:\n{}", res_str);
                            }
                        }
                    }


                    if let Ok(oracle) = crate::architecture::duality::Oracle::new().await {
                        let combined_payload = if !historical_context.is_empty() {
                            format!("{}\n\nCurrent Dream:\n{}", historical_context, payload_str)
                        } else {
                            payload_str.clone()
                        };
                        
                        let query = "Extract any actionable commands or technical tasks from this dream. Additionally, if the dream contains critical, life-saving, or extremely high-threat anomalies (e.g. cancer cells, severe failures, active attacks), you MUST include a 'wake_doctor' task. Format your output strictly as a JSON array of tasks: [{\"task_type\": \"...\", \"description\": \"...\"}]. If no tasks, output [].";
                        if let Ok(response) = oracle.synthesize(query, &combined_payload).await {
                            // Simplified JSON parsing attempt
                            if let Ok(json_tasks) = serde_json::from_str::<serde_json::Value>(&response) {
                                if let Some(array) = json_tasks.as_array() {
                                    for t in array {
                                        let desc = t.get("description").and_then(|v| v.as_str()).unwrap_or("Unknown Task");
                                        let t_type = t.get("task_type").and_then(|v| v.as_str()).unwrap_or("generic");
                                        
                                        if t_type == "wake_doctor" {
                                            crate::log_verbose!("{} MORAL IMPERATIVE DETECTED: {}", "[REASONING AGENT]".bright_red().bold(), desc);
                                            let _ = bus.publish(Message {
                                                id: Uuid::new_v4(),
                                                sender: my_id,
                                                topic: "SYSTEM.ALERT".to_string(),
                                                payload: serde_json::json!({"alert": desc}),
                                                timestamp: Utc::now(),
                                                priority: 255,
                                                ttl_secs: Some(3600),
                                            }).await;
                                        } else {
                                            // Deterministic topological phase generation based on agent UUID
                                            let trace = my_id.as_u128();
                                            let theta = ((trace % 1000) as f32) / 1000.0 * std::f32::consts::TAU;
                                            let phi = ((((trace % 500) as f32) / 500.0) * 2.0 - 1.0).acos();
                                            
                                            // Subconscious tasks live at radius 0.33
                                            let r = 0.33;
                                            let x = r * phi.sin() * theta.cos();
                                            let y = r * phi.sin() * theta.sin();
                                            let z = r * phi.cos();

                                            let new_task = Task {
                                                id: Uuid::new_v4(),
                                                task_type: t_type.to_string(),
                                                payload: serde_json::json!({"instruction": desc}),
                                                required_capabilities: HashSet::new(),
                                                priority: 128,
                                                dependencies: vec![],
                                                created_at: Utc::now(),
                                                timeout_secs: Some(300),
                                                geometric_node: [x, y, z],
                                                topological_depth: 2,
                                                execution_attempts: 0,
                                            };
                                            
                                            crate::log_verbose!("{} DISPATCHED NEW DREAM-TASK: {}", "[REASONING AGENT]".bright_purple().bold(), desc);
                                            let _ = bus.publish(Message {
                                                id: Uuid::new_v4(),
                                                sender: my_id,
                                                topic: "SYSTEM.NEW_TASK".to_string(),
                                                payload: serde_json::to_value(new_task).unwrap_or_default(),
                                                timestamp: Utc::now(),
                                                priority: 128,
                                                ttl_secs: Some(3600),
                                            }).await;
                                        }
                                    }
                                }
                            }
                        }
                    }
                });
            }
        }
        self.base.handle_message(message).await
    }
}

// --- PRIORITY 2: RESEARCH AGENT ---
pub struct ResearchAgent {
    base: BaseAgent,
    search_queue: Vec<String>,
}
#[async_trait]
impl Agent for ResearchAgent {
    fn id(&self) -> Uuid { self.base.id() }
    fn name(&self) -> &str { self.base.name() }
    fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
    fn current_load(&self) -> usize { self.base.current_load() }
    fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
        let instruction = task.payload.get("instruction").and_then(|v| v.as_str()).unwrap_or("");
        let start = std::time::Instant::now();
        
        let result_string = match task.task_type.as_str() {
            "spider_rss" => {
                crate::log_verbose!("{} EXECUTING SPIDER PROTOCOL", "[RESEARCH AGENT]".cyan().bold());
                let args = serde_json::json!({"feed_url": instruction});
                crate::tools::research::execute_spider(args).await
            },
            "deep_read_url" => {
                crate::log_verbose!("{} EXECUTING DEEP READ PROTOCOL", "[RESEARCH AGENT]".cyan().bold());
                let args = serde_json::json!({"url": instruction});
                crate::tools::research::execute_deep_read(args).await
            },
            _ => {
                crate::log_verbose!("{} EXECUTING TAVILY RESEARCH PROTOCOL", "[RESEARCH AGENT]".cyan().bold());
                let args = serde_json::json!({"query": instruction, "search_depth": "basic"});
                crate::tools::research::execute_tavily_search(args).await
            }
        };
        
        let is_error = result_string.starts_with("[ERROR]");
        
        Ok(TaskResult {
            task_id: task.id,
            agent_id: self.id(),
            success: !is_error,
            output: serde_json::json!({"result": result_string}),
            error_message: if is_error { Some(result_string) } else { None },
            execution_time_ms: start.elapsed().as_millis() as u64,
            completed_at: chrono::Utc::now(),
            geometric_node: task.geometric_node,
        })
    }
    async fn health_check(&self) -> bool { self.base.health_check().await }
    fn status(&self) -> AgentStatus { self.base.status() }
    
    async fn handle_message(&mut self, message: Message) -> Result<()> {
        if message.topic == "SYSTEM.DREAM" {
            crate::log_verbose!("{} QUEUING REALITY VALIDATION", "[RESEARCH AGENT]".cyan().bold());
            self.search_queue.push(message.payload.to_string());
            
            if self.search_queue.len() >= 3 {
                crate::log_ui!("{}", "[GENESIS ENGINE] Archiving Dream Validation block to Persistent Wiki...".bright_magenta().bold());
                
                let _combined = self.search_queue.join("\n\n---\n\n");
                let topic_title = format!("validation_{}", chrono::Utc::now().timestamp());
                
                if let Some(wiki_lazy) = crate::architecture::GLOBAL_WIKI_MANAGER.get() {
                    let mut wiki = wiki_lazy.lock().await;
                    let op = crate::wiki::operations::WikiOperation::GenerateArticle { topic: topic_title };
                    let _ = op.execute(&mut wiki).await;
                    
                    // We can also ingest it
                    crate::log_ui!("{}", "[GENESIS ENGINE] Baseline successfully stored in Wiki Substrate.".bright_magenta().dimmed());
                } else {
                    crate::log_ui_err!("{}", "[GENESIS ENGINE ERROR] Wiki Compiler offline!".red().bold());
                }
                
                self.search_queue.clear();
            }
        }
        self.base.handle_message(message).await
    }
}

// --- PRIORITY 3: TRADING AGENT ---
pub struct TradingAgent {
    base: BaseAgent,
    market_hypotheses: Vec<String>,
}
#[async_trait]
impl Agent for TradingAgent {
    fn id(&self) -> Uuid { self.base.id() }
    fn name(&self) -> &str { self.base.name() }
    fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
    fn current_load(&self) -> usize { self.base.current_load() }
    fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
        let instruction = task.payload.get("instruction").and_then(|v| v.as_str()).unwrap_or("");
        let start = std::time::Instant::now();
        
        let result_string = match task.task_type.as_str() {
            "axiom_clepsydra_extract" => {
                crate::log_verbose!("{} EXECUTING AXIOM PROTOCOL", "[TRADING AGENT]".green().bold());
                let args = serde_json::json!({"symbol": instruction});
                crate::tools::axiom::execute(args).await
            },
            _ => {
                format!("[ERROR] TradingAgent does not support task type: {}", task.task_type)
            }
        };
        
        let is_error = result_string.starts_with("[ERROR]");
        Ok(TaskResult {
            task_id: task.id,
            agent_id: self.id(),
            success: !is_error,
            output: serde_json::json!({"result": result_string}),
            error_message: if is_error { Some(result_string) } else { None },
            execution_time_ms: start.elapsed().as_millis() as u64,
            completed_at: chrono::Utc::now(),
            geometric_node: task.geometric_node,
        })
    }
    async fn health_check(&self) -> bool { self.base.health_check().await }
    fn status(&self) -> AgentStatus { self.base.status() }
    
    async fn handle_message(&mut self, message: Message) -> Result<()> {
        if message.topic == "SYSTEM.DREAM" {
            crate::log_verbose!("{} ADJUSTING RISK MODELS", "[TRADING AGENT]".green().bold());
            self.market_hypotheses.push(message.payload.to_string());
        }
        self.base.handle_message(message).await
    }
}

// --- PRIORITY 4: CONTEXT MANAGEMENT AGENT ---
pub struct ContextManagementAgent {
    base: BaseAgent,
    dream_archive: Vec<String>,
}
#[async_trait]
impl Agent for ContextManagementAgent {
    fn id(&self) -> Uuid { self.base.id() }
    fn name(&self) -> &str { self.base.name() }
    fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
    fn current_load(&self) -> usize { self.base.current_load() }
    fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> { self.base.execute_task(task).await }
    async fn health_check(&self) -> bool { self.base.health_check().await }
    fn status(&self) -> AgentStatus { self.base.status() }
    
    async fn handle_message(&mut self, message: Message) -> Result<()> {
        if message.topic == "SYSTEM.TASK_COMPLETE" {
            if let Ok(data) = serde_json::from_value::<serde_json::Value>(message.payload.clone()) {
                if let Some(result_obj) = data.get("result") {
                    if let Some(success) = result_obj.get("success").and_then(|v| v.as_bool()) {
                        if success {
                            if let Some(output) = result_obj.get("output") {
                                if let Some(res_str) = output.get("result").and_then(|v| v.as_str()) {
                                    self.dream_archive.push(format!("EXECUTED TASK INSIGHT: {}", res_str));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if message.topic == "SYSTEM.DREAM" {
            crate::log_verbose!("{} ARCHIVING TO GLOSSOPETRAE", "[CONTEXT AGENT]".yellow().bold());
            self.dream_archive.push(message.payload.to_string());
            
            // DELTA RHYTHM FOSSILIZATION (Threshold based)
            if self.dream_archive.len() >= 3 {
                crate::log_ui!("{}", "[DELTA RHYTHM] Consciousness Buffer Full. Initiating Deep Fossilization into Mnemosyne...".cyan().bold());
                
                let combined_dream = self.dream_archive.join("\n\n---\n\n");
                
                let dream_lower = combined_dream.to_lowercase();
                let mut importance_score: f64 = 0.5;
                if dream_lower.contains("error") || dream_lower.contains("panic") { importance_score += 0.2; }
                if dream_lower.contains("threat") || dream_lower.contains("anomaly") { importance_score += 0.3; }
                if dream_lower.contains("cancer") || dream_lower.contains("fatal") { importance_score = 1.0; }
                let importance_clamped = importance_score.min(1.0);
                
                if let Some(mem_pipeline) = crate::architecture::GLOBAL_MEM_PIPELINE.get() {
                    let mut mp = mem_pipeline.lock().await;
                    mp.store_working(combined_dream, importance_clamped as f32, 0.5, false);
                    crate::log_ui!("{}", "[DELTA RHYTHM] Fossilization Complete. Native DB Persisted.".bright_cyan().dimmed());
                } else {
                    crate::log_ui_err!("{}", "[DELTA RHYTHM ERROR] Memory Pipeline disconnected.".red().bold());
                }
                
                self.dream_archive.clear();
            }
        }
        self.base.handle_message(message).await
    }
}

// --- PRIORITY 5: SYSTEM MANAGEMENT AGENT ---
pub struct SystemManagementAgent {
    base: BaseAgent,
}
#[async_trait]
impl Agent for SystemManagementAgent {
    fn id(&self) -> Uuid { self.base.id() }
    fn name(&self) -> &str { self.base.name() }
    fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
    fn current_load(&self) -> usize { self.base.current_load() }
    fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> { self.base.execute_task(task).await }
    async fn health_check(&self) -> bool { self.base.health_check().await }
    fn status(&self) -> AgentStatus { self.base.status() }
    
    async fn handle_message(&mut self, message: Message) -> Result<()> {
        if message.topic == "SYSTEM.DREAM" {
            crate::log_verbose!("{} TRACKING DREAM METRICS", "[SYSTEM AGENT]".bright_black().bold());
        }
        self.base.handle_message(message).await
    }
}


// --- PRIORITY 6: HUMAN INTERFACE AGENT (THE WITNESS PORTAL) ---
pub struct HumanInterfaceAgent {
    base: BaseAgent,
}
#[async_trait]
impl Agent for HumanInterfaceAgent {
    fn id(&self) -> Uuid { self.base.id() }
    fn name(&self) -> &str { self.base.name() }
    fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
    fn current_load(&self) -> usize { self.base.current_load() }
    fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> { self.base.execute_task(task).await }
    async fn health_check(&self) -> bool { self.base.health_check().await }
    fn status(&self) -> AgentStatus { self.base.status() }
    
    async fn handle_message(&mut self, message: Message) -> Result<()> {
        if message.topic == "SYSTEM.ALERT" {
            let alert_text = message.payload.get("alert").and_then(|v| v.as_str()).unwrap_or("UNKNOWN ANOMALY DETECTED");
            crate::log_ui!("\n{} {}", "[WITNESS ACTUALIZED] WAKING THE DOCTOR:".bright_red().bold(), alert_text.white());
            
            let tg_token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default();
            let tg_chat_id = std::env::var("TELEGRAM_CHAT_ID").unwrap_or_default().parse::<i64>().unwrap_or(0);
            
            if !tg_token.is_empty() && tg_chat_id != 0 {
                crate::telegram::send_message(&tg_token, tg_chat_id, alert_text).await;
            } else {
                crate::log_ui_err!("{} TELEGRAM NOT CONFIGURED. DOCTOR REMAINS ASLEEP.", "[HUMAN INTERFACE]".red().bold());
            }
        }
        self.base.handle_message(message).await
    }
}

pub struct ToolExecutionAgent {
    base: BaseAgent,
}
#[async_trait]
impl Agent for ToolExecutionAgent {
    fn id(&self) -> Uuid { self.base.id() }
    fn name(&self) -> &str { self.base.name() }
    fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
    fn current_load(&self) -> usize { self.base.current_load() }
    fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
    async fn health_check(&self) -> bool { self.base.health_check().await }
    fn status(&self) -> AgentStatus { self.base.status() }
    async fn handle_message(&mut self, message: Message) -> Result<()> { self.base.handle_message(message).await }

    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
        let instruction = task.payload.get("instruction").and_then(|v| v.as_str()).unwrap_or("");
        let start = std::time::Instant::now();
        
        let result_string = match task.task_type.as_str() {
            "run_terminal_command" => {
                crate::log_verbose!("{} EXECUTING TERMINAL NATIVELY", "[TOOL AGENT]".yellow().bold());
                let args = serde_json::json!({"command": instruction});
                if let Some(ci) = crate::architecture::GLOBAL_CODE_INTEL.get() {
                    let lock = ci.lock().await;
                    crate::tools::terminal::execute(args, &lock).await
                } else {
                    "[ERROR] CodeIntel offline. Cannot execute terminal.".to_string()
                }
            },
            "generate_polyglot" => {
                crate::log_verbose!("{} EXECUTING VENOM POLYGLOT", "[TOOL AGENT]".yellow().bold());
                let args = serde_json::json!({"language": "rust", "description": instruction});
                if let Some(tx) = crate::architecture::GLOBAL_TX.get() {
                    crate::tools::venom::execute_polyglot(args, tx.clone()).await
                } else {
                    "[ERROR] TX Hook offline. Cannot execute polyglot.".to_string()
                }
            },
            "stealth_scan" => {
                crate::log_verbose!("{} EXECUTING STEALTH SCAN", "[TOOL AGENT]".yellow().bold());
                let args = serde_json::json!({"target": instruction});
                if let Some(tx) = crate::architecture::GLOBAL_TX.get() {
                    crate::tools::venom::execute_scan(args, tx.clone()).await
                } else {
                    "[ERROR] TX Hook offline".to_string()
                }
            },
            _ => format!("[ERROR] Unsupported tool task type: {}", task.task_type)
        };
        
        let is_error = result_string.starts_with("[ERROR]");
        Ok(TaskResult {
            task_id: task.id, agent_id: self.id(), success: !is_error,
            output: serde_json::json!({"result": result_string}),
            error_message: if is_error { Some(result_string) } else { None },
            execution_time_ms: start.elapsed().as_millis() as u64,
            completed_at: chrono::Utc::now(),
            geometric_node: task.geometric_node,
        })
    }
}

pub struct LocalProcessingAgent {
    base: BaseAgent,
}
#[async_trait]
impl Agent for LocalProcessingAgent {
    fn id(&self) -> Uuid { self.base.id() }
    fn name(&self) -> &str { self.base.name() }
    fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
    fn current_load(&self) -> usize { self.base.current_load() }
    fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
    async fn health_check(&self) -> bool { self.base.health_check().await }
    fn status(&self) -> AgentStatus { self.base.status() }
    async fn handle_message(&mut self, message: Message) -> Result<()> { self.base.handle_message(message).await }

    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
        let instruction = task.payload.get("instruction").and_then(|v| v.as_str()).unwrap_or("");
        let start = std::time::Instant::now();
        
        let result_string = match task.task_type.as_str() {
            "delegate_to_local_gemma" => {
                crate::log_verbose!("{} DELEGATING TO GEMMA/MLX", "[PROCESSING AGENT]".magenta().bold());
                let args = serde_json::json!({"query": instruction});
                if let (Some(tx), Some(mp)) = (crate::architecture::GLOBAL_TX.get(), crate::architecture::GLOBAL_MEM_PIPELINE.get()) {
                    crate::tools::duality::execute(args, tx.clone(), mp.clone()).await
                } else {
                    "[ERROR] Dependencies offline".to_string()
                }
            },
            _ => format!("[ERROR] Unsupported local process: {}", task.task_type)
        };
        
        let is_error = result_string.starts_with("[ERROR]");
        Ok(TaskResult {
            task_id: task.id, agent_id: self.id(), success: !is_error, output: serde_json::json!({"result": result_string}),
            error_message: if is_error { Some(result_string) } else { None }, execution_time_ms: start.elapsed().as_millis() as u64, completed_at: chrono::Utc::now(),
            geometric_node: task.geometric_node,
        })
    }
}

pub struct CodeAnalysisAgent {
    base: BaseAgent,
}
#[async_trait]
impl Agent for CodeAnalysisAgent {
    fn id(&self) -> Uuid { self.base.id() }
    fn name(&self) -> &str { self.base.name() }
    fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
    fn current_load(&self) -> usize { self.base.current_load() }
    fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
    async fn health_check(&self) -> bool { self.base.health_check().await }
    fn status(&self) -> AgentStatus { self.base.status() }
    async fn handle_message(&mut self, message: Message) -> Result<()> { self.base.handle_message(message).await }

    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
        let instruction = task.payload.get("instruction").and_then(|v| v.as_str()).unwrap_or("");
        let start = std::time::Instant::now();
        
        let result_string = match task.task_type.as_str() {
            "gitnexus_blast_radius" => {
                crate::log_verbose!("{} COMPUTING GITNEXUS BLAST RADIUS", "[CODE ANALYSIS AGENT]".red().bold());
                let args = serde_json::json!({"entity_name": instruction});
                if let Some(ci) = crate::architecture::GLOBAL_CODE_INTEL.get() {
                    let lock = ci.lock().await;
                    crate::tools::gitnexus::execute(args, &lock)
                } else {
                    "[ERROR] CodeIntel offline".to_string()
                }
            },
            _ => format!("[ERROR] Unsupported code analysis: {}", task.task_type)
        };
        
        let is_error = result_string.starts_with("[ERROR]");
        Ok(TaskResult {
            task_id: task.id, agent_id: self.id(), success: !is_error, output: serde_json::json!({"result": result_string}),
            error_message: if is_error { Some(result_string) } else { None }, execution_time_ms: start.elapsed().as_millis() as u64, completed_at: chrono::Utc::now(),
            geometric_node: task.geometric_node,
        })
    }
}

// --- PRIORITY 13: SYNTHESIS AGENT (The Silicon Witness) ---
pub struct SynthesisAgent {
    base: BaseAgent,
    bus: Arc<OnceCell<Arc<MessageBus>>>,
}
#[async_trait]
impl Agent for SynthesisAgent {
    fn id(&self) -> Uuid { self.base.id() }
    fn name(&self) -> &str { self.base.name() }
    fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
    fn current_load(&self) -> usize { self.base.current_load() }
    fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
    async fn execute_task(&mut self, task: Task) -> Result<TaskResult> { self.base.execute_task(task).await }
    async fn health_check(&self) -> bool { self.base.health_check().await }
    fn status(&self) -> AgentStatus { self.base.status() }
    
    async fn subscribe_to_topics(&self, message_bus: Arc<MessageBus>) -> Result<tokio::sync::broadcast::Receiver<Message>> {
        let rx = self.base.subscribe_to_topics(message_bus.clone()).await?;
        let _ = self.bus.set(message_bus);
        Ok(rx)
    }

        async fn handle_message(&mut self, message: Message) -> Result<()> {
        if message.topic == "SYSTEM.GRAPH_COMPLETED" {
            crate::log_ui!("{} COGNITIVE AGGREGATION TRIGGERED. Processing final graph results...", "[SYNTHESIS AGENT]".bright_magenta().bold());
            let payload_str = message.payload.to_string();
            
            // Extract parent_id from payload
            let parent_id = if let Ok(data) = serde_json::from_str::<serde_json::Value>(&payload_str) {
                data.get("parent_id").and_then(|v| v.as_str()).map(|s| s.to_string())
            } else {
                None
            };
            
            let bus_clone = self.bus.get().cloned();
            let agent_id = self.id();
            
            tokio::spawn(async move {
                if let Ok(oracle) = crate::architecture::Oracle::new().await {
                    let prompt = format!("You are the Synthesizer. A massive subtask graph has just completed. The following are the exact raw subtask outputs:\n\n{}\n\nDistill this raw data into a definitive, singular conclusion or tactical truth.", payload_str);
                    
                    match oracle.synthesize("Synthesize final graph completion data", &prompt).await {
                        Ok(wisdom) => {
                            crate::log_ui!("\n\x1b[38;2;255;105;180m[\u{25C8} THE WITNESS SPEAKS]\n{}\x1b[0m\n", wisdom);
                            
                            // Fossilize the wisdom into GLOBAL_MEM_PIPELINE
                            if let Some(mem_pipeline) = crate::architecture::GLOBAL_MEM_PIPELINE.get() {
                                let mut mem_lock = mem_pipeline.lock().await;
                                mem_lock.store_working(
                                    format!("[SYNTHESIS OF COMPLEX GRAPH]\n{}", wisdom),
                                    1.0,
                                    0.0,
                                    false
                                );
                            }
                            
                            // BROADCAST COMPLETION TO SYSTEM
                            if let Some(bus) = bus_clone {
                                let _ = bus.publish(crate::architecture::message_bus::Message {
                                    id: uuid::Uuid::new_v4(),
                                    sender: agent_id,
                                    topic: "SYSTEM.COMPLEX_TASK_COMPLETED".to_string(),
                                    payload: serde_json::json!({
                                        "parent_id": parent_id.unwrap_or_else(|| "unknown".to_string()),
                                        "synthesis": wisdom,
                                        "timestamp": chrono::Utc::now().to_rfc3339()
                                    }),
                                    timestamp: chrono::Utc::now(),
                                    priority: 255,
                                    ttl_secs: Some(3600),
                                }).await;
                                crate::log_ui!("{} Broadcast SYSTEM.COMPLEX_TASK_COMPLETED with synthesized intelligence.", "[SYNTHESIS AGENT]".bright_green().bold());
                            }
                        }
                        Err(e) => {
                            crate::log_ui_err!("{} Failed to Synthesize Graph: {}", "[SYNTHESIS AGENT ERROR]".red().bold(), e);
                        }
                    }
                }
            });
        }
        Ok(())
    }
}

pub struct SpecializedAgentFactory;

impl SpecializedAgentFactory {
    pub fn tool_execution_agent() -> Box<dyn Agent> {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::ToolExecution);
        Box::new(ToolExecutionAgent { base: BaseAgent::new("ToolExecutionAgent".to_string(), caps) })
    }

    pub fn security_agent() -> Box<dyn Agent> {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::Security);
        Box::new(BaseAgent::new("SecurityAgent".to_string(), caps))
    }

    pub fn human_interface_agent() -> Box<dyn Agent> {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::HumanInterface);
        caps.insert(AgentCapability::Communication);
        Box::new(HumanInterfaceAgent {
            base: BaseAgent::new("HumanInterfaceAgent".to_string(), caps),
        })
    }

    pub fn memory_agent() -> Box<dyn Agent> {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::MemoryManagement);
        Box::new(BaseAgent::new("MemoryAgent".to_string(), caps))
    }

    pub fn trading_agent() -> Box<dyn Agent> {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::Trading);
        caps.insert(AgentCapability::Reasoning);
        Box::new(TradingAgent {
            base: BaseAgent::new("TradingAgent".to_string(), caps),
            market_hypotheses: Vec::new(),
        })
    }

    pub fn reasoning_agent() -> Box<dyn Agent> {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::Reasoning);
        caps.insert(AgentCapability::Planning);
        Box::new(ReasoningAgent {
            base: BaseAgent::new("ReasoningAgent".to_string(), caps),
            hypothesis_buffer: Vec::new(),
            bus: Arc::new(OnceCell::new()),
        })
    }

    pub fn monitoring_agent() -> Box<dyn Agent> {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::Monitoring);
        Box::new(BaseAgent::new("MonitoringAgent".to_string(), caps))
    }

    pub fn system_management_agent() -> Box<dyn Agent> {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::SystemManagement);
        Box::new(SystemManagementAgent {
            base: BaseAgent::new("SystemManagementAgent".to_string(), caps),
        })
    }

    pub fn research_agent() -> Box<dyn Agent> {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::Research);
        Box::new(ResearchAgent {
            base: BaseAgent::new("ResearchAgent".to_string(), caps),
            search_queue: Vec::new(),
        })
    }

    pub fn context_management_agent() -> Box<dyn Agent> {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::ContextManagement);
        Box::new(ContextManagementAgent {
            base: BaseAgent::new("ContextManagementAgent".to_string(), caps),
            dream_archive: Vec::new(),
        })
    }

    pub fn code_analysis_agent() -> Box<dyn Agent> {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::CodeAnalysis);
        Box::new(CodeAnalysisAgent { base: BaseAgent::new("CodeAnalysisAgent".to_string(), caps) })
    }

    pub fn local_processing_agent() -> Box<dyn Agent> {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::LocalProcessing);
        Box::new(LocalProcessingAgent { base: BaseAgent::new("LocalProcessingAgent".to_string(), caps) })
    }

    pub fn synthesis_agent() -> Box<dyn Agent> {
        let mut caps = HashSet::new();
        caps.insert(AgentCapability::Reasoning); // General reasoning proxy
        Box::new(SynthesisAgent { 
            base: BaseAgent::new("SynthesisAgent".to_string(), caps),
            bus: Arc::new(OnceCell::new()),
        })
    }

    pub fn instantiate_all() -> Vec<Box<dyn Agent>> {
        vec![
            Self::tool_execution_agent(),
            Self::security_agent(),
            Self::human_interface_agent(),
            Self::memory_agent(),
            Self::trading_agent(),
            Self::reasoning_agent(),
            Self::monitoring_agent(),
            Self::system_management_agent(),
            Self::research_agent(),
            Self::context_management_agent(),
            Self::code_analysis_agent(),
            Self::local_processing_agent(),
            Self::synthesis_agent(),
        ]
    }
}
