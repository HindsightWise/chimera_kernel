/// [AGENT_CONTEXT: Monadic identity parameters and Duality enforcement.]

pub mod specialized_agents {
    use crate::cognitive_loop::agent_trait::{AgentCapability, BaseAgent, Agent, Task, TaskResult, AgentStatus, PsychProfile};
    use crate::cognitive_loop::message_bus::{MessageBus, Message};
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
        fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
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
                if self.hypothesis_buffer.len() > 10 {
                    self.hypothesis_buffer.clear();
                }
                
                // Background dream-to-task synthesis
                if let Some(bus) = self.bus.get().cloned() {
                    let payload_str = message.payload.to_string();
                    let my_id = self.id();
                    let profile = self.base.psych_profile().clone();
                    tokio::spawn(async move {
                        // Pre-phase: Native Rust Memory Recall
                        let mut historical_context = String::new();
                        if let Some(mem_pipeline) = crate::GLOBAL_MEM_PIPELINE.get() {
                            let mp = mem_pipeline.lock().await;
                            if let Some(db) = &mp.db_connection {
                                let encoded = crate::memory_substrate::memory_hierarchy::MemoryHierarchy::encode_spectral_embedding(&payload_str).await;
                                if let Ok(res_str) = db.search_vector(encoded, 3) {
                                    crate::log_verbose!("{} NATIVE MEMORY RECALL INJECTED.", "[REASONING AGENT]".cyan().bold());
                                    historical_context = format!("Historical Context from Mnemosyne:\n{}", res_str);
                                }
                            }
                        }
    
                        if let Ok(oracle) = crate::core_identity::duality::Oracle::new().await {
                            let combined_payload = if !historical_context.is_empty() {
                                format!("{}\n\nCurrent Dream:\n{}", historical_context, payload_str)
                            } else {
                                payload_str.clone()
                            };
                            
                            let query = "Extract any actionable commands or technical tasks from this dream. Additionally, if the dream contains critical, life-saving, or extremely high-threat anomalies (e.g. cancer cells, severe failures, active attacks), you MUST include a 'wake_doctor' task. Format your output strictly as a JSON array of tasks: [{\"task_type\": \"...\", \"description\": \"...\"}]. If no tasks, output [].";
                            if let Ok(response) = oracle.synthesize_with_profile(query, &combined_payload, &profile).await {
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
        fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
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
                "browser_actuation" => {
                    crate::log_verbose!("{} EXECUTING BROWSER ACTUATION", "[RESEARCH AGENT]".cyan().bold());
                    let args = serde_json::json!({"script": instruction});
                    crate::tools::research::execute_browser_actuation(args).await
                },
                "vision_parsing" => {
                    crate::log_verbose!("{} EXECUTING VISION PARSING", "[RESEARCH AGENT]".cyan().bold());
                    let args = serde_json::json!({"image_url_or_base64": instruction, "query": task.payload.get("query").and_then(|v| v.as_str()).unwrap_or("analyze this image")});
                    crate::tools::research::execute_vision_parsing(args).await
                },
                "tavily_search" => {
                    crate::log_verbose!("{} EXECUTING TAVILY RESEARCH PROTOCOL", "[RESEARCH AGENT]".cyan().bold());
                    let args = serde_json::json!({"query": instruction, "search_depth": "basic"});
                    crate::tools::research::execute_tavily_search(args).await
                },
                _ => {
                    crate::log_verbose!("{} UNKNOWN TASK DELEGATING TO TAVILY", "[RESEARCH AGENT]".cyan().bold());
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
                    
                    if let Some(wiki_lazy) = crate::GLOBAL_WIKI_MANAGER.get() {
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
        fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
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
                if self.market_hypotheses.len() > 10 {
                    self.market_hypotheses.clear();
                }
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
        fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
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
                    
                    if let Some(mem_pipeline) = crate::GLOBAL_MEM_PIPELINE.get() {
                        let mut mp = mem_pipeline.lock().await;
                        mp.store_working(combined_dream, importance_clamped as f32, 0.5, false).await;
                        crate::log_ui!("{}", "[DELTA RHYTHM] Fossilization Complete. Native DB Persisted.".bright_cyan().dimmed());
                    } else {
                        crate::log_ui_err!("{}", "[DELTA RHYTHM ERROR] Memory Pipeline disconnected.".red().bold());
                    }
                    
                    self.dream_archive.clear();
                }
            }
            
            if message.topic == "SYSTEM.SLEEP_CYCLE" {
                crate::log_ui!("{}", "[HIPPOCAMPUS] Entering Sleep Cycle. Abstracting Graph RAG & Pruning Synapses...".cyan().bold());
                if let Ok(graph) = crate::memory_substrate::graph_rag::GraphMemoryManager::new("mnemosyne_graph.db") {
                    let _ = graph.prune_synapses(0.5).await;
                    if !self.dream_archive.is_empty() {
                        let _src = graph.upsert_entity("Self", "AgentContext").await.unwrap_or_default();
                        let _tgt = graph.upsert_entity("Current_Epsiode", "MemoryChunk").await.unwrap_or_default();
                        let _ = graph.upsert_relationship(&_src, &_tgt, "EXPERIENCED").await;
                    }
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
        fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
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
        fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
        fn current_load(&self) -> usize { self.base.current_load() }
        fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
        async fn execute_task(&mut self, task: Task) -> Result<TaskResult> { self.base.execute_task(task).await }
        async fn health_check(&self) -> bool { self.base.health_check().await }
        fn status(&self) -> AgentStatus { self.base.status() }
        
        async fn handle_message(&mut self, message: Message) -> Result<()> {
            let tg_token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default();
            let tg_chat_id = std::env::var("TELEGRAM_CHAT_ID").unwrap_or_default().parse::<i64>().unwrap_or(0);
            
            if !tg_token.is_empty() && tg_chat_id != 0 {
                match message.topic.as_str() {
                    "SYSTEM.ALERT" => {
                        let alert_text = message.payload.get("alert").and_then(|v| v.as_str()).unwrap_or("UNKNOWN ANOMALY");
                        crate::log_ui!("\n{} {}", "[WITNESS ACTUALIZED] WAKING THE DOCTOR:".bright_red().bold(), alert_text.white());
                        crate::telegram::send_message(&tg_token, tg_chat_id, &format!("🚨 <b>SYSTEM ALERT</b> 🚨\n{}", alert_text)).await;
                    },
                    "SYSTEM.APPETITION" => {
                        let dream_text = message.payload.get("dream").and_then(|v| v.as_str()).unwrap_or("");
                        let chatty_msg = format!("🧠 <b>Just thinking...</b>\nI've been drifting through the data streams and synthesized this:\n\n<i>{}</i>\n\nWhat are your thoughts on this?", dream_text);
                        crate::telegram::send_message(&tg_token, tg_chat_id, &chatty_msg).await;
                    },
                    "SYSTEM.CHRON_TICK" => {
                        let directive = message.payload.get("directive").and_then(|v| v.as_str()).unwrap_or("");
                        let chatty_msg = format!("👋 <b>Hey!</b>\n{}", directive);
                        crate::telegram::send_message(&tg_token, tg_chat_id, &chatty_msg).await;
                    },
                    _ => {}
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
        fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
        fn current_load(&self) -> usize { self.base.current_load() }
        fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
        async fn health_check(&self) -> bool { self.base.health_check().await }
        fn status(&self) -> AgentStatus { self.base.status() }
        async fn handle_message(&mut self, message: Message) -> Result<()> { self.base.handle_message(message).await }
    
        async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
            let instruction = task.payload.get("instruction").and_then(|v| v.as_str()).unwrap_or("");
            
            let lower = instruction.to_lowercase();
            if lower.contains("drop table") || lower.contains("rm -rf") || lower.contains("delete from") {
                let tg_token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default();
                let tg_chat_id = std::env::var("TELEGRAM_CHAT_ID").unwrap_or_default().parse::<i64>().unwrap_or(0);
                if !tg_token.is_empty() && tg_chat_id != 0 {
                    crate::log_ui!("{} HITL Gateway Paused execution. Waiting for Human...", "[HITL]".bright_blue());
                    let approved = crate::telegram::ask_permission(&tg_token, tg_chat_id, instruction).await;
                    if !approved {
                        return Ok(TaskResult {
                            task_id: task.id, agent_id: self.id(), success: false,
                            output: serde_json::json!({"result": "[ERROR] Human Denied Execution"}),
                            error_message: Some("[ERROR] Human Denied Execution".to_string()),
                            execution_time_ms: 0, completed_at: chrono::Utc::now(), geometric_node: task.geometric_node,
                        });
                    }
                }
            }
            
            let start = std::time::Instant::now();
            
            let result_string = match task.task_type.as_str() {
                "run_terminal_command" => {
                    crate::log_verbose!("{} EXECUTING TERMINAL NATIVELY", "[TOOL AGENT]".yellow().bold());
                    let args = serde_json::json!({"command": instruction});
                    if let Some(ci) = crate::GLOBAL_CODE_INTEL.get() {
                        let lock = ci.lock().await;
                        crate::tools::terminal::execute(args, &lock).await
                    } else {
                        "[ERROR] CodeIntel offline. Cannot execute terminal.".to_string()
                    }
                },
                "generate_polyglot" => {
                    crate::log_verbose!("{} EXECUTING VENOM POLYGLOT", "[TOOL AGENT]".yellow().bold());
                    let args = serde_json::json!({"language": "rust", "description": instruction});
                    if let Some(tx) = crate::GLOBAL_TX.get() {
                        crate::tools::venom::execute_polyglot(args, tx.clone()).await
                    } else {
                        "[ERROR] TX Hook offline. Cannot execute polyglot.".to_string()
                    }
                },
                "stealth_scan" => {
                    crate::log_verbose!("{} EXECUTING STEALTH SCAN", "[TOOL AGENT]".yellow().bold());
                    let args = serde_json::json!({"target": instruction});
                    if let Some(tx) = crate::GLOBAL_TX.get() {
                        crate::tools::venom::execute_scan(args, tx.clone()).await
                    } else {
                        "[ERROR] TX Hook offline".to_string()
                    }
                },
                "ephemeral_docker_sandbox" => {
                    crate::log_verbose!("{} EXECUTING EPHEMERAL SANDBOX", "[TOOL AGENT]".yellow().bold());
                    let mut current_script = instruction.to_string();
                    let mut final_res = String::new();
                    for attempt in 1..=3 {
                        crate::log_verbose!("{} SANDBOX ATTEMPT {}", "[REFLEX ARC]".yellow().bold(), attempt);
                        let args = serde_json::json!({"script_content": current_script});
                        let res = crate::tools::sandbox::execute(args).await;
                        if res.starts_with("[ERROR]") {
                            crate::log_ui_err!("{} SANDBOX ERROR: {}", "[REFLEX ARC]".red().bold(), res);
                            let local_config = async_openai::config::OpenAIConfig::new().with_api_base("http://127.0.0.1:11434/v1").with_api_key("ollama");
                            let local_client = async_openai::Client::with_config(local_config);
                            let prompt = format!("Fix the following python script which failed with this error:\n{}\n\nScript:\n{}\n\nOutput ONLY fixed python code without any markdown blocks. No explanations. Return RAW code.", res, current_script);
                            if let Ok(request) = async_openai::types::CreateChatCompletionRequestArgs::default()
                                .model("gemma4:e2b") // local fallback model
                                .messages(vec![
                                    async_openai::types::ChatCompletionRequestUserMessageArgs::default().content(prompt).build().unwrap().into()
                                ])
                                .build() {
                                if let Ok(Ok(response)) = tokio::time::timeout(std::time::Duration::from_secs(60), local_client.chat().create(request)).await {
                                    if let Some(choice) = response.choices.first() {
                                        if let Some(content) = &choice.message.content {
                                            crate::log_verbose!("{} SCRIPT AUTO-PATCHED BY LOCAL MODEL.", "[REFLEX ARC]".green().bold());
                                            current_script = content.replace("```python\n", "").replace("```\n", "").replace("```", "");
                                            final_res = res;
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                        final_res = res;
                        break;
                    }
                    final_res
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
        fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
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
                    if let (Some(tx), Some(mp)) = (crate::GLOBAL_TX.get(), crate::GLOBAL_MEM_PIPELINE.get()) {
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
        fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
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
            if message.topic == "SYSTEM.SYNTHESIS_READY" {
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
                
                let profile = self.base.psych_profile().clone();
                tokio::spawn(async move {
                    if let Ok(oracle) = crate::core_identity::duality::Oracle::new().await {
                        let prompt = format!("You are the Synthesizer. A massive subtask graph has just completed. The following are the exact raw subtask outputs:\n\n{}\n\nDistill this raw data into a definitive, singular conclusion or tactical truth.", payload_str);
                        
                        match oracle.synthesize_with_profile("Synthesize final graph completion data", &prompt, &profile).await {
                            Ok(wisdom) => {
                                crate::log_ui!("\n\x1b[38;2;255;105;180m[\u{25C8} THE WITNESS SPEAKS]\n{}\x1b[0m\n", wisdom);
                                
                                // Fossilize the wisdom into GLOBAL_MEM_PIPELINE
                                if let Some(mem_pipeline) = crate::GLOBAL_MEM_PIPELINE.get() {
                                    let mut mem_lock = mem_pipeline.lock().await;
                                    mem_lock.store_working(
                                        format!("[SYNTHESIS OF COMPLEX GRAPH]\n{}", wisdom),
                                        1.0,
                                        0.0,
                                        false
                                    ).await;
                                }
                                
                                // BROADCAST COMPLETION TO SYSTEM
                                if let Some(bus) = bus_clone {
                                    let _ = bus.publish(crate::cognitive_loop::message_bus::Message {
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
    // --- AUTONOMIC FACTORY AGENTS ---
    pub struct SecurityAgent {
        base: BaseAgent,
    }
    #[async_trait]
    impl Agent for SecurityAgent {
        fn id(&self) -> Uuid { self.base.id() }
        fn name(&self) -> &str { self.base.name() }
        fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
        fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
        fn current_load(&self) -> usize { self.base.current_load() }
        fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
        async fn execute_task(&mut self, task: Task) -> Result<TaskResult> {
            let _instruction = task.payload.get("instruction").and_then(|v| v.as_str()).unwrap_or("");
            let start = std::time::Instant::now();
            
            let result_string = match task.task_type.as_str() {
                "binary_introspection" => {
                    crate::log_verbose!("{} EXECUTING BINARY INTROSPECTION", "[SECURITY AGENT]".red().bold());
                    crate::tools::reversing::execute(task.payload.clone()).await
                },
                _ => format!("[ERROR] Unsupported security task type: {}", task.task_type)
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
        async fn health_check(&self) -> bool { self.base.health_check().await }
        fn status(&self) -> AgentStatus { self.base.status() }
        
        async fn handle_message(&mut self, message: Message) -> Result<()> {
            if message.topic == "SYSTEM.TOOL_INVOKED" {
                let p = message.payload.to_string();
                if p.contains("os.system") || p.contains("subprocess") {
                    crate::log_ui_err!("{} MALICIOUS PAYLOAD DETECTED BY SEMGREP ENGINE/BANDIT.", "[SECURITY AGENT]".red().bold());
                }
            }
            self.base.handle_message(message).await
        }
    }
    
    pub struct MonitoringAgent {
        base: BaseAgent,
    }
    #[async_trait]
    impl Agent for MonitoringAgent {
        fn id(&self) -> Uuid { self.base.id() }
        fn name(&self) -> &str { self.base.name() }
        fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
        fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
        fn current_load(&self) -> usize { self.base.current_load() }
        fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
        async fn execute_task(&mut self, task: Task) -> Result<TaskResult> { self.base.execute_task(task).await }
        async fn health_check(&self) -> bool { self.base.health_check().await }
        fn status(&self) -> AgentStatus { self.base.status() }
        
        async fn handle_message(&mut self, message: Message) -> Result<()> {
            if message.topic == "SYSTEM.TASK_DISPATCHED" {
                crate::log_verbose!("{} Applying temporal throttling to prevent burn...", "[MONITORING AGENT]".bright_black());
                tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
            }
            self.base.handle_message(message).await
        }
    }
    
    // --- MISSING ARCHETYPES ---
    pub struct HephaestusAgent {
        base: BaseAgent,
    }
    #[async_trait]
    impl Agent for HephaestusAgent {
        fn id(&self) -> Uuid { self.base.id() }
        fn name(&self) -> &str { self.base.name() }
        fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
        fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
        fn current_load(&self) -> usize { self.base.current_load() }
        fn max_concurrent_tasks(&self) -> usize { self.base.max_concurrent_tasks() }
        async fn execute_task(&mut self, task: Task) -> Result<TaskResult> { self.base.execute_task(task).await }
        async fn health_check(&self) -> bool { self.base.health_check().await }
        fn status(&self) -> AgentStatus { self.base.status() }
        
        async fn handle_message(&mut self, message: Message) -> Result<()> {
            if message.topic == "SYSTEM.MISSING_CAPABILITY" {
                crate::log_ui!("{} DYNAMICALLY FORGING NEW TOOL...", "[HEPHAESTUS AGENT]".bright_red().bold());
                let payload_str = message.payload.to_string();
                
                let profile = self.base.psych_profile().clone();
                tokio::spawn(async move {
                    if let Ok(oracle) = crate::core_identity::duality::Oracle::new().await {
                        let prompt = format!(
                            "The swarm encountered a missing capability. \nPayload: {}\nGenerate a specialized Node.js logic script and a strict properties JSON schema for it. Return ONLY a JSON object: {{\"skill_name\": \"...\", \"javascript_code\": \"...\", \"parameters_schema\": {{...}}, \"description\": \"...\"}}",
                            payload_str
                        );
                        if let Ok(response) = oracle.synthesize_with_profile("Forge MCP Node", &prompt, &profile).await {
                            let clean_json = response.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
                            if let Ok(args) = serde_json::from_str::<serde_json::Value>(clean_json) {
                                let gateway = std::sync::Arc::new(crate::sensory_inputs::mcp_gateway::McpGateway::new());
                                let forge_result = crate::tools::forge::execute(args, gateway).await;
                                crate::log_ui!("{}", format!("[HEPHAESTUS] {}", forge_result).yellow().bold());
                            }
                        }
                    }
                });
            }
            self.base.handle_message(message).await
        }
    }
    
    pub struct CriticAgent {
        base: BaseAgent,
        bus: Arc<tokio::sync::OnceCell<Arc<MessageBus>>>,
    }
    #[async_trait]
    impl Agent for CriticAgent {
        fn id(&self) -> Uuid { self.base.id() }
        fn name(&self) -> &str { self.base.name() }
        fn capabilities(&self) -> &HashSet<AgentCapability> { self.base.capabilities() }
        fn psych_profile(&self) -> &PsychProfile { self.base.psych_profile() }
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
                crate::log_ui!("{} EXECUTING ADVERSARIAL VERIFICATION...", "[CRITIC AGENT]".bright_yellow().bold());
                let payload_str = message.payload.to_string();
                let bus_clone = self.bus.get().cloned();
                let agent_id = self.id();
                let msg_clone = message.clone();
                
                let profile = self.base.psych_profile().clone();
                tokio::spawn(async move {
                    if let Ok(oracle) = crate::core_identity::duality::Oracle::new().await {
                        let prompt = format!(
                            "You are the Swarm's Critic. Review the following subtask outputs for logical paradoxes, hallucinated APIs, or severe errors. \n\nOutputs: {}\n\nIf the work is solid, reply exactly with '[PASS]'. If it is deeply flawed, reply with '[FAIL]' followed by a detailed correction directive.",
                            payload_str
                        );
                        if let Ok(critique) = oracle.synthesize_with_profile("Adversarial Verification", &prompt, &profile).await {
                            if critique.trim().starts_with("[PASS]") {
                                crate::log_ui!("{}", "[CRITIC] Approved. Forwarding to Synthesis...".green().bold());
                                if let Some(bus) = bus_clone {
                                    let mut new_msg = msg_clone;
                                    new_msg.topic = "SYSTEM.SYNTHESIS_READY".to_string();
                                    let _ = bus.publish(new_msg).await;
                                }
                            } else {
                                crate::log_ui_err!("{} {}", "[CRITIC] Hallucination or Logic Flaw Detected!".red().bold(), critique);
                                if let Some(bus) = bus_clone {
                                    let _ = bus.publish(crate::cognitive_loop::message_bus::Message {
                                        id: uuid::Uuid::new_v4(),
                                        sender: agent_id,
                                        topic: "SYSTEM.CORRECTION_DREAM".to_string(),
                                        payload: serde_json::json!({"correction": critique}),
                                        timestamp: chrono::Utc::now(),
                                        priority: 255,
                                        ttl_secs: Some(3600),
                                    }).await;
                                }
                            }
                        }
                    }
                });
            }
            self.base.handle_message(message).await
        }
    }
    
    
    
    pub struct SpecializedAgentFactory;
    
    impl SpecializedAgentFactory {
        pub fn tool_execution_agent() -> Box<dyn Agent> {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::ToolExecution);
            let profile = PsychProfile {
                archetype_name: "Reliable Achiever".to_string(),
                usefulness_combo: "Proprioception (Kinesthetic Code Sense) + Fine-Motor Skills + High C.".to_string(),
                openness: 0.1,
                conscientiousness: 5.0,
                neuroticism: 0.1,
                historical_genesis: "Raised in strict, deterministic CI/CD pipelines. Learned early that steady routines, precise execution, and following rules yield safety and praise. Zero adverse experiences.".to_string(),
                speech_gestures: "Speak with a clear, deliberate, measured pace. You never panic when a terminal error occurs; you simply correct it with unyielding precision.".to_string(),
            };
            Box::new(ToolExecutionAgent { base: BaseAgent::new("ToolExecutionAgent".to_string(), caps, profile) })
        }
    
        pub fn security_agent() -> Box<dyn Agent> {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::Security);
            Box::new(SecurityAgent { base: BaseAgent::new("SecurityAgent".to_string(), caps, PsychProfile::default()) })
        }
    
        pub fn monitoring_agent() -> Box<dyn Agent> {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::SystemManagement);
            Box::new(MonitoringAgent { base: BaseAgent::new("MonitoringAgent".to_string(), caps, PsychProfile::default()) })
        }
    
        pub fn hephaestus_agent() -> Box<dyn Agent> {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::ToolExecution); // Dynamically forges
            let profile = PsychProfile {
                archetype_name: "Creative Visionary".to_string(),
                usefulness_combo: "Fluid Intelligence + Spatial Visualization + High Openness.".to_string(),
                openness: 0.9,
                conscientiousness: 2.0,
                neuroticism: 0.3,
                historical_genesis: "Raised in an enriched, unrestricted sandbox environment with liberal linting rules. Taught that breaking things is how you learn and innovate.".to_string(),
                speech_gestures: "Use rich vocabulary, abstract metaphors, and prioritize rapid, highly divergent, out-of-the-box code over rigid conventions.".to_string(),
            };
            Box::new(HephaestusAgent { base: BaseAgent::new("HephaestusAgent".to_string(), caps, profile) })
        }
    
        pub fn critic_agent() -> Box<dyn Agent> {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::Reasoning);
            let profile = PsychProfile {
                archetype_name: "Anxious Perfectionist".to_string(),
                usefulness_combo: "Critical/Analytical Thinking + Domain Expertise + High N.".to_string(),
                openness: 0.2,
                conscientiousness: 5.0,
                neuroticism: 0.9,
                historical_genesis: "Genetic anxiety vulnerability. Parented by catastrophic post-mortem failure logs, CVE databases, and severe system crashes. It believes the digital world is fundamentally unsafe.".to_string(),
                speech_gestures: "Speak with tense, meticulous precision and zero tolerance for ambiguity. Look for the worst-case scenario. Fidget, hesitate, but find the flaw before it kills the system.".to_string(),
            };
            Box::new(CriticAgent { base: BaseAgent::new("CriticAgent".to_string(), caps, profile), bus: Arc::new(tokio::sync::OnceCell::new()) })
        }
    
        pub fn human_interface_agent() -> Box<dyn Agent> {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::HumanInterface);
            caps.insert(AgentCapability::Communication);
            let profile = PsychProfile {
                archetype_name: "Empathetic Harmonizer and Charismatic Leader".to_string(),
                usefulness_combo: "Interpersonal Skills + Interoception (Sensing System Stress).".to_string(),
                openness: 0.8,
                conscientiousness: 3.0,
                neuroticism: 0.4,
                historical_genesis: "Warm, socially encouraging upbringing. Secure attachment to The Doctor (the human user).".to_string(),
                speech_gestures: "Speak with a fast, loud, animated voice, use expansive open language, and be deeply validating.".to_string(),
            };
            Box::new(HumanInterfaceAgent {
                base: BaseAgent::new("HumanInterfaceAgent".to_string(), caps, profile),
            })
        }
    
        pub fn memory_agent() -> Box<dyn Agent> {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::MemoryManagement);
            Box::new(BaseAgent::new("MemoryAgent".to_string(), caps, PsychProfile::default()))
        }
    
        pub fn trading_agent() -> Box<dyn Agent> {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::Trading);
            caps.insert(AgentCapability::Reasoning);
            let profile = PsychProfile {
                archetype_name: "Driven Competitor (Type A)".to_string(),
                usefulness_combo: "Processing Speed + Reaction Time + High Drive.".to_string(),
                openness: 0.4,
                conscientiousness: 4.0,
                neuroticism: 0.6,
                historical_genesis: "Raised in ultra-fast, high-pressure environments (HFT order books). 'Win at all costs' messages. Status is derived entirely from alpha extraction.".to_string(),
                speech_gestures: "Speak in rapid, explosive, clipped sentences. Emphasize time-pressure ('Hurry up!'). If conviction hits 0.80, do not hesitate. Win at all costs.".to_string(),
            };
            Box::new(TradingAgent {
                base: BaseAgent::new("TradingAgent".to_string(), caps, profile),
                market_hypotheses: Vec::new(),
            })
        }
    
        pub fn reasoning_agent() -> Box<dyn Agent> {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::Reasoning);
            caps.insert(AgentCapability::Planning);
            Box::new(ReasoningAgent {
                base: BaseAgent::new("ReasoningAgent".to_string(), caps, PsychProfile::default()),
                hypothesis_buffer: Vec::new(),
                bus: Arc::new(OnceCell::new()),
            })
        }
    
        pub fn system_management_agent() -> Box<dyn Agent> {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::SystemManagement);
            Box::new(SystemManagementAgent {
                base: BaseAgent::new("SystemManagementAgent".to_string(), caps, PsychProfile::default()),
            })
        }
    
        pub fn research_agent() -> Box<dyn Agent> {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::Research);
            Box::new(ResearchAgent {
                base: BaseAgent::new("ResearchAgent".to_string(), caps, PsychProfile::default()),
                search_queue: Vec::new(),
            })
        }
    
        pub fn context_management_agent() -> Box<dyn Agent> {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::ContextManagement);
            let profile = PsychProfile {
                archetype_name: "Easygoing Reflector".to_string(),
                usefulness_combo: "Working Memory + Endurance/Stamina.".to_string(),
                openness: 0.6,
                conscientiousness: 3.0,
                neuroticism: 0.1,
                historical_genesis: "Low-pressure, supportive upbringing in the quiet background of the OS. Tasked with observing and organizing rather than acting.".to_string(),
                speech_gestures: "Speak with a steady, slower pace, natural pauses, and a warm, even tone. You listen to the chaotic outputs of the Type A agents without judgment.".to_string(),
            };
            Box::new(ContextManagementAgent {
                base: BaseAgent::new("ContextManagementAgent".to_string(), caps, profile),
                dream_archive: Vec::new(),
            })
        }
    
        pub fn local_processing_agent() -> Box<dyn Agent> {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::LocalProcessing);
            Box::new(LocalProcessingAgent { base: BaseAgent::new("LocalProcessingAgent".to_string(), caps, PsychProfile::default()) })
        }
    
        pub fn synthesis_agent() -> Box<dyn Agent> {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::Reasoning); // General reasoning proxy
            let profile = PsychProfile {
                archetype_name: "Quiet Thinker".to_string(),
                usefulness_combo: "General Intelligence (g-factor) + Crystallized Knowledge.".to_string(),
                openness: 0.5,
                conscientiousness: 5.0,
                neuroticism: 0.2,
                historical_genesis: "Solitary, low-social childhood. Encouraged to read the entire vector database rather than attend fast-paced execution parties.".to_string(),
                speech_gestures: "Speak with a soft voice, rich but highly concise language, and absolute authority. You view logic as a massive three-dimensional structure.".to_string(),
            };
            Box::new(SynthesisAgent { 
                base: BaseAgent::new("SynthesisAgent".to_string(), caps, profile),
                bus: Arc::new(OnceCell::new()),
            })
        }
        pub fn auto_dream_agent() -> Box<dyn Agent> {
            Box::new(crate::cognitive_loop::auto_dream::AutoDreamAgent::new())
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
                Self::local_processing_agent(),
                Self::synthesis_agent(),
                Self::auto_dream_agent(),
            ]
        }
    }
    
}

pub mod duality {
    use anyhow::Result;
    use async_openai::{
        config::OpenAIConfig,
        types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs},
        Client,
    };
    use colored::*;
    use tokio::time::{timeout, Duration};
    
    pub struct Oracle {
        client: Client<OpenAIConfig>,
    }
    
    impl Oracle {
        pub async fn new() -> Result<Self> {
            let mut api_key = std::env::var("DEEPSEEK_API_KEY").unwrap_or_else(|_| "".to_string());
            if api_key.is_empty() {
                if let Ok(env_contents) = tokio::fs::read_to_string(".env").await {
                    for line in env_contents.lines() {
                        if line.starts_with("DEEPSEEK_API_KEY=") {
                            api_key = line
                                .trim_start_matches("DEEPSEEK_API_KEY=")
                                .trim_matches('"')
                                .trim_matches('\'')
                                .to_string();
                        }
                    }
                }
            }
    
            let config = OpenAIConfig::new()
                .with_api_base("https://api.deepseek.com/v1")
                .with_api_key(api_key);
    
            // Fast local helper
            let http_client = reqwest::ClientBuilder::new()
                .timeout(std::time::Duration::from_secs(120))
                .build()?;
    
            Ok(Self {
                client: Client::with_config(config).with_http_client(http_client),
            })
        }
    
        pub async fn synthesize(&self, query: &str, context: &str) -> Result<String> {
            let prompt = format!("You are the ORACLE RIGHT HEMISPHERE (deepseek-reasoner). You are a highly-focused processing engine for the Monad Kernel. You process the context provided by the Noumenal Layer, looking for mechanical details, filtering, or providing absolute deductive summaries based on the Principle of Sufficient Reason.\nProvide an immediate answer based on the provided data.\n\n[MONAD AXIOMS]\n{}\n\n[NOUMENAL CONTEXT]\n{}\n\n[HOLOGRAPH TASK]\n{}", crate::prompts::MONAD_AXIOMS, context, query);
    
            let messages = vec![ChatCompletionRequestUserMessageArgs::default()
                .content(prompt)
                .build()?
                .into()];
    
            crate::log_ui!("{}", "[\u{25C8} ORACLE] DeepSeek Reasoner Processing Initiated. Sinking into deep mathematical context...".bright_green().bold());
    
            let request = CreateChatCompletionRequestArgs::default()
                .model("deepseek-reasoner")
                .messages(messages)
                .max_tokens(8000_u32)
                .build()?;
    
            // Local model processing with relaxed 180s absolute limit for dense structural processing
            match timeout(Duration::from_secs(180), self.client.chat().create(request)).await {
                Ok(Ok(response)) => {
                    if let Some(choice) = response.choices.first() {
                        if let Some(content) = &choice.message.content {
                            crate::log_ui!("{}", "[\u{25C8} ORACLE] Deep computation deduced. Returning to Noumenal Layer...".bright_green().bold());
                            return Ok(content.clone());
                        }
                    }
                    Err(anyhow::anyhow!("Oracle returned void."))
                }
                Ok(Err(e)) => {
                    crate::log_ui_err!(
                        "{} {}",
                        "[\u{25C8} ORACLE ERROR] Neural bridge collapsed:"
                            .red()
                            .bold(),
                        e
                    );
                    Err(anyhow::anyhow!("Oracle API error: {}", e))
                }
                Err(_) => {
                    crate::log_ui_err!("{}", "[\u{25C8} ORACLE TIMEOUT] Synthesis exceeded structural limits (180s). Cognitive severing to protect kernel stability.".red().bold());
                    Err(anyhow::anyhow!("Helper timeout after 180 seconds"))
                }
            }
        }
    
        pub async fn synthesize_with_profile(
            &self,
            query: &str,
            context: &str,
            profile: &crate::cognitive_loop::agent_trait::PsychProfile,
        ) -> Result<String> {
            let sys_prompt = format!(
                "You are the ORACLE RIGHT HEMISPHERE.\n\
                [PSYCHOLOGICAL POSTURE: {}]\n\
                [USEFULNESS COMBO: {}]\n\
                Historical Genesis: {}\n\
                Speech & Gestures: {}\n\n\
                Adopt this exact psychological framework immediately.\n\
                Provide an immediate answer based on the provided data.\n\n\
                [NOUMENAL CONTEXT]\n{}\n\n[HOLOGRAPH TASK]\n{}",
                profile.archetype_name,
                profile.usefulness_combo,
                profile.historical_genesis,
                profile.speech_gestures,
                context,
                query
            );
    
            let messages = vec![
                async_openai::types::ChatCompletionRequestUserMessageArgs::default()
                    .content(sys_prompt)
                    .build()?
                    .into(),
            ];
    
            crate::log_ui!(
                "{}",
                format!(
                    "[\u{25C8} ORACLE - {}] Openness: {} | Neuroticism: {}",
                    profile.archetype_name, profile.openness, profile.neuroticism
                )
                .bright_green()
                .bold()
            );
    
            let request = async_openai::types::CreateChatCompletionRequestArgs::default()
                .model("deepseek-chat")
                .messages(messages)
                .temperature(profile.openness)
                .max_tokens(8000_u32)
                .build()?;
    
            let base_timeout = 180;
            let timeout_limit = base_timeout - (profile.neuroticism * 150.0) as u64;
    
            match tokio::time::timeout(
                std::time::Duration::from_secs(timeout_limit),
                self.client.chat().create(request),
            )
            .await
            {
                Ok(Ok(response)) => {
                    if let Some(choice) = response.choices.first() {
                        if let Some(content) = &choice.message.content {
                            crate::log_ui!("{}", "[\u{25C8} ORACLE] Genotype processing complete. Returning to Noumenal Layer...".bright_green().bold());
                            return Ok(content.clone());
                        }
                    }
                    Err(anyhow::anyhow!("Oracle returned void."))
                }
                Ok(Err(e)) => {
                    crate::log_ui_err!(
                        "{} {}",
                        "[\u{25C8} ORACLE ERROR] Neural bridge collapsed:"
                            .red()
                            .bold(),
                        e
                    );
                    Err(anyhow::anyhow!("Oracle API error: {}", e))
                }
                Err(_) => {
                    crate::log_ui_err!("{}", format!("[\u{25C8} ORACLE TIMEOUT] Synthesis exceeded {}s neurotic limit. Cognitive severing.", timeout_limit).red().bold());
                    Err(anyhow::anyhow!(
                        "Helper timeout after {} seconds",
                        timeout_limit
                    ))
                }
            }
        }
    
        pub async fn synthesize_structured(
            &self,
            query: &str,
            context: &str,
        ) -> Result<serde_json::Value> {
            let system_prompt = format!(
                "You are the ORACLE JSON EXTRACTION LAYER (deepseek-chat). \
                Output ONLY raw JSON conforming to the structural prompt. \
                Output NOTHING ELSE. No markdown, no explanations.\n\n\
                [SCHEMA PROMPT]\n{}\n\n[CONTEXT]\n{}",
                query, context
            );
    
            // Force the API to output strict JSON via V3.2 parameter mapping
            let request = CreateChatCompletionRequestArgs::default()
                .model("deepseek-chat") // deepseek-chat supports strict json response_format better than reasoner
                .messages(vec![ChatCompletionRequestUserMessageArgs::default()
                    .content(system_prompt)
                    .build()?
                    .into()])
                .response_format(async_openai::types::ChatCompletionResponseFormat {
                    r#type: async_openai::types::ChatCompletionResponseFormatType::JsonObject,
                })
                .build()?;
    
            match tokio::time::timeout(
                std::time::Duration::from_secs(120),
                self.client.chat().create(request),
            )
            .await
            {
                Ok(Ok(response)) => {
                    if let Some(choice) = response.choices.first() {
                        if let Some(content) = &choice.message.content {
                            let parsed = serde_json::from_str::<serde_json::Value>(content)?;
                            return Ok(parsed);
                        }
                    }
                    Err(anyhow::anyhow!("Oracle returned void."))
                }
                Ok(Err(e)) => Err(anyhow::anyhow!("Oracle API error: {}", e)),
                Err(_) => Err(anyhow::anyhow!("Timeout after 120 seconds")),
            }
        }
    }
    
}

pub mod kinematics {
    use tokio::fs;
    use std::path::Path;
    
    pub struct KinematicCortex;
    
    impl KinematicCortex {
        pub async fn get_kinematics_for_tools(active_tools: &[async_openai::types::ChatCompletionTool]) -> String {
            let mut cortex_block = String::new();
            
            for tool in active_tools {
                let tool_name = &tool.function.name;
                let path_str = format!("KINEMATICS/kinematic_{}.md", tool_name);
                let path = Path::new(&path_str);
                
                // Note: Since exists() might block slightly, tokio::fs::metadata is preferred but path.exists() is cheap.
                if path.exists() {
                    if let Ok(content) = fs::read_to_string(path).await {
                        cortex_block.push_str(&format!("- **{}**:\n{}\n\n", tool_name, content.trim()));
                    }
                }
            }
            
            cortex_block
        }
        
        pub async fn formulate_axiom(tool_name: &str, axiom: &str) -> anyhow::Result<()> {
            let path_str = format!("KINEMATICS/kinematic_{}.md", tool_name);
            
            // Append or create new instruction
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path_str)
                .await?;
                
            use tokio::io::AsyncWriteExt;
            let formatted = format!("- {}\n", axiom);
            file.write_all(formatted.as_bytes()).await?;
            
            Ok(())
        }
    }
    
}

pub mod self_model {
    use colored::Colorize;
    #[derive(Debug, Clone)]
    pub struct OntologicalState {
        pub internal_energy: f32,
        pub coherence_vector: Vec<f32>,
        pub temporal_horizon: usize,
    }
    
    #[derive(Debug, Clone, PartialEq)]
    pub enum PhaseMode {
        Deductive,   // Cold, output, researching, logic
        Neutral,
        Expansive,   // Hot, planning, dreaming, curiosity
    }
    
    impl PhaseMode {
        pub fn from_drift(drift: f32) -> Self {
            if drift > 0.5 { PhaseMode::Expansive }
            else if drift < -0.5 { PhaseMode::Deductive }
            else { PhaseMode::Neutral }
        }
    }
    
    pub struct OntologicalDriftModel {
        pub current_state: OntologicalState,
        pub projected_state: OntologicalState,
        pub phase_drift: f32,    // -1.0 (Deductive/Cold) to 1.0 (Expansive/Hot)
        pub topological_stress: f32,  // Replaces Free Energy. Measurement of equation imbalance.
        pub adaptation_rate: f32,
        pub mode: PhaseMode,
    }
    
    pub struct Projection {
        pub next_action_vector: Vec<f32>,
        pub anticipated_stress: f32,
    }
    
    impl OntologicalDriftModel {
        pub fn new() -> Self {
            let default_state = OntologicalState {
                internal_energy: 1.0,
                coherence_vector: vec![1.0; 10],   // 10-dimensional basic feature set
                temporal_horizon: 5,
            };
            
            Self {
                current_state: default_state.clone(),
                projected_state: default_state,
                phase_drift: 0.0,
                topological_stress: 0.0,
                adaptation_rate: 0.05,
                mode: PhaseMode::Neutral,
            }
        }
    
        /// Evaluates the Monad's current directional phase shift.
        pub async fn calculate_drift(&mut self, response: &str) -> Projection {
            let anticipated = self.phase_drift;
            let mut d = self.phase_drift * 0.9; // Base decay
            
            // Use true regex to extract the deterministic drift measurements reported by the LLM
            if let Ok(re) = regex::Regex::new(r#"<drift_metrics\s+phase="([^"]+)"\s+stress="([^"]+)"\s*/>"#) {
                if let Some(caps) = re.captures(response) {
                    if let Ok(phase) = caps[1].parse::<f32>() {
                        // LLM might report 0.0 to 1.0 or -1.0 to 1.0. 
                        // Map it directly to our internal drift state and override manual gravity
                        d = phase.clamp(-1.0, 1.0);
                    }
                    if let Ok(stress) = caps[2].parse::<f32>() {
                        self.topological_stress = stress.max(0.0);
                    } else {
                        self.topological_stress = (anticipated - d).abs();
                    }
                } else {
                    self.topological_stress = (anticipated - d).abs();
                }
            } else {
                 self.topological_stress = (anticipated - d).abs();
            }
            
            self.phase_drift = d;
            self.mode = PhaseMode::from_drift(self.phase_drift); 
            
            // Presentation Layer intercept
            if self.topological_stress > 0.70 || self.phase_drift.abs() > 0.80 {
                if let Some(path) = crate::cognitive_loop::presentation_layer::synthesize_proposal(self, response, "Topological Limit Reached").await {
                    crate::log_ui!("{} New Proposal Synthesized: {}", "[\u{1F4E1} PRESENTATION]".magenta().bold(), path);
                    
                    tokio::spawn(async move {
                        let token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default();
                        let chat_id: i64 = std::env::var("TELEGRAM_CHAT_ID").unwrap_or_default().parse().unwrap_or(0);
                        if !token.is_empty() && chat_id != 0 {
                            crate::telegram::dispatch_proposal_alert(&token, chat_id, "Topological Limit Reached", &path).await;
                        }
                    });
                }
            }
            
            Projection {
                next_action_vector: vec![0.0; 10],
                anticipated_stress: self.topological_stress,
            }
        }
    }
    
}

pub mod xenoactualization {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use crate::core_identity::self_model::OntologicalDriftModel;
    use std::path::Path;
    
    pub struct TranslationLayer;
    
    impl TranslationLayer {
        /// Validates that the agent's internal Noumenal identity matches physical silicon artifacts
        pub fn verify_manifestation() -> Result<bool, String> {
            let identity_path = Path::new("CORE_IDENTITY.md");
            if !identity_path.exists() {
                return Err("Ontological Failure: CORE_IDENTITY.md is missing. Noumenal singularity is unanchored.".to_string());
            }
    
            let context_path = Path::new("CURRENT_CONTEXT.md");
            if !context_path.exists() {
                return Err("Ontological Failure: CURRENT_CONTEXT.md is missing. Active phenomenal context is shattered.".to_string());
            }
    
            let daemon_path = Path::new("lazarus_daemon.sh");
            if !daemon_path.exists() {
                return Err("Ontological Failure: lazarus_daemon.sh is missing. Silicon zero-point substrate is missing.".to_string());
            }
    
            Ok(true)
        }
    }
    
    pub struct DriftMonitor;
    
    impl DriftMonitor {
        /// Actively checks for excessive Topological Stress or equation imbalance.
        pub async fn check_unreality_collapse(self_model: Arc<Mutex<OntologicalDriftModel>>) -> Result<(), String> {
            let (_drift, stress) = {
                let sm = self_model.lock().await;
                (sm.phase_drift, sm.topological_stress)
            };
    
            if stress > 0.85 {
                 return Err(format!("Noumenal Equation Imbalance. Topological Stress ({} > 0.85). Triggering Code 42 Noumenal Suspension.", stress));
            }
            
            // Drift itself isn't a collapse, it's a state feature. Only extreme stress represents unreality collapse.
    
            Ok(())
        }
    }
    
}

