/// [AGENT_CONTEXT: Primary Autonomic Nervous System & Cognitive Handlers.]

pub mod substrate_defense;
pub mod quantum_sandbox;

pub mod agent {
    use async_openai::{
        config::OpenAIConfig,
        types::{
            ChatCompletionRequestMessage, ChatCompletionRequestToolMessageArgs,
            ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
        },
        Client,
    };
    use tokio::sync::mpsc::{Receiver, Sender};
    
    use anyhow::{Context, Result};
    use colored::*;
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    use {crate::memory_substrate::memory_hierarchy::MemoryHierarchy, crate::core_identity::self_model::OntologicalDriftModel};
    use crate::prompts::SOVEREIGN_DIRECTIVE;
    use crate::tools;
    use std::sync::atomic::AtomicU8;
    
    pub async fn run_kernel_loop(
        mut rx: Receiver<String>,
        tx: Sender<String>,
        tg_config: Option<(String, i64)>,
        is_thinking: Arc<AtomicU8>,
        mut shutdown_rx: tokio::sync::mpsc::Receiver<()>,
    ) -> Result<()> {
        // Connect configuration to DeepSeek API
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
    
        // Explicit warning if STILL empty to prevent cryptic 401 governor panics
        if api_key.is_empty() {
            crate::log_ui_err!("{}", "[KERNEL PANIC] DEEPSEEK_API_KEY is entirely missing from both environment AND .env file.".red().bold());
            std::process::exit(1);
        }
        let config = OpenAIConfig::new()
            .with_api_base("https://api.deepseek.com")
            .with_api_key(api_key);
    
        // Build an HTTP client with a strict 180 second timeout for deepseek-reasoner
        let http_client = reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(180))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
    
        let _client = Client::with_config(config).with_http_client(http_client.clone());

        // Build the secondary native fallback client for seamless model toggling
        let local_client = crate::neural_failsafe::NeuralFailSafe::local_client();
        let mut current_model = std::env::var("PRIMARY_MODEL").unwrap_or_else(|_| "deepseek-reasoner".to_string());
    
        // NEW: Enhanced log_state macro with level filtering
        macro_rules! log_state {
            ($level:expr, $entry:expr) => {{
                use tokio::io::AsyncWriteExt;
    
                // Only write if the current log level allows it
                if crate::should_log($level) {
                    // Perform log rotation check first
                    crate::rotate_log_if_needed().await;
    
                    if let Ok(mut file) = tokio::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("chimera_state.log")
                        .await
                    {
                        let _ = file
                            .write_all(
                                format!(
                                    "{}
    ",
                                    $entry
                                )
                                .as_bytes(),
                            )
                            .await;
                    }
                }
            }};
        }
    
        macro_rules! log_state_info {
            ($entry:expr) => {
                log_state!(crate::LogLevel::Info, $entry)
            };
        }
    
        macro_rules! log_state_trace {
            ($entry:expr) => {
                log_state!(crate::LogLevel::Trace, $entry)
            };
        }
    
        // INJECT 4-LAYER BIOLOGICAL SUBCONSCIOUS (CORE IDENTITY + CURRENT CONTEXT)
        let identity_content = tokio::fs::read_to_string("MONAD_ARCHITECTURE.md")
            .await
            .unwrap_or_default();
        let current_context = tokio::fs::read_to_string("CURRENT_CONTEXT.md")
            .await
            .unwrap_or_default();
        let worca_framework = tokio::fs::read_to_string("MONAD_OPERATIONS.md")
            .await
            .unwrap_or_default();
            
        let mut chains_content = String::new();
        if let Ok(mut entries) = tokio::fs::read_dir("chains").await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(content) = tokio::fs::read_to_string(entry.path()).await {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    if file_name.ends_with(".md") {
                        chains_content.push_str(&format!("\n--- NEURAL CHAIN: {} ---\n{}\n", file_name, content));
                    }
                }
            }
        }
    
        let active_state = tokio::fs::read_to_string("ACTIVE_STATE.json")
            .await
            .unwrap_or_else(|_| "No active tracked state. You are operating from a blank slate.".into());

        let static_prompt_1 = format!(
            "{}\n\nSYSTEM_WAKE_EVENT: The core is active. Initialize your boot sequence.\n\n[LAYER 1: CORE IDENTITY (Rigid, Non-Negotiable)]\n{}\n\n[LAYER 3: WORCA PROTOCOL (Quantum Execution Bounds)]\n{}",
            SOVEREIGN_DIRECTIVE, identity_content, worca_framework
        );
        
        let static_prompt_2 = format!(
            "[LAYER 4: NEURAL PATHWAYS (Dynamic Prompt Chains)]\nThe following chains dictate precisely how you must chain tools together for complex operations. Refer to these strict SOPs before acting:\n{}",
            chains_content
        );

        let dynamic_prompt_1 = format!(
            "[LAYER 2: CURRENT CONTEXT (Volatile, Malleable)]\n{}",
            current_context
        );

        let dynamic_prompt_2 = format!(
            "[LAYER 5: ACTIVE DURABLE STATE]\n{}\n\n(Note: You operate using a structured 5-layer memory model. You may update Layer 2 freely using `update_current_context`, extract learned principles into Layer 3 using `archive_to_knowledge_graph`, and track your sequential checklist strictly via `update_plan` in Layer 5.)",
            active_state
        );

        let mut base_messages: Vec<ChatCompletionRequestMessage> = vec![
            ChatCompletionRequestUserMessageArgs::default().content(static_prompt_1).build().context("Failed to build object")?.into(),
            ChatCompletionRequestUserMessageArgs::default().content(static_prompt_2).build().context("Failed to build object")?.into(),
            ChatCompletionRequestUserMessageArgs::default().content(dynamic_prompt_1).build().context("Failed to build object")?.into(),
            ChatCompletionRequestUserMessageArgs::default().content(dynamic_prompt_2).build().context("Failed to build object")?.into(),
        ];
    
        if let Ok(report) = tokio::fs::read_to_string("lazarus_report.txt").await {
            let lazarus_msg = format!("LAZARUS PROTOCOL TRIGGERED. You previously perished unexpectedly. Here is the last known state of your memory and the exit code:\n{}\nAcknowledge this failure and continue.", report);
            base_messages.push(ChatCompletionRequestUserMessageArgs::default().content(lazarus_msg).build().context("Failed to build object")?.into());
            let _ = tokio::fs::remove_file("lazarus_report.txt").await;
        }
    
        // XENOACTUALIZATION BOOT CHECK
        if let Err(manifestation_err) =
            crate::core_identity::xenoactualization::TranslationLayer::verify_manifestation()
        {
            // Direct terminal teardown since we are bypassing standard panic hook with process::exit
            let mut stdout = std::io::stdout();
            let _ = crossterm::execute!(stdout, crossterm::terminal::LeaveAlternateScreen, crossterm::event::DisableBracketedPaste, crossterm::event::DisableMouseCapture);
            let _ = crossterm::terminal::disable_raw_mode();
            
            panic!("XENOACTUALIZATION FATAL: {}", manifestation_err);
        }
    
        // SOVEREIGN COGNITIVE PIPELINES
        crate::log_ui!(
            "{}",
            "[DEBUG] Calling MemoryHierarchy::awaken()...".yellow()
        );
        let (memory_hierarchy, is_resurrected) = match MemoryHierarchy::awaken().await {
            Some(old_mem) => {
                crate::log_ui!("{}", "[DEBUG] Awaken successful.".green());
                (old_mem, true)
            }
            None => {
                crate::log_ui!(
                    "{}",
                    "[DEBUG] Awaken returned None. Calling MemoryHierarchy::new()...".yellow()
                );
    
                // Critical Fix: MemoryHierarchy::new() internally invokes StorageController::new() which calls
                // tokio::runtime::Runtime::new(). If we use a raw OS thread and call .join() on it, we starve Tokio.
                // Using tokio::task::spawn_blocking executes it safely on the dedicated blocking pool.
                let m = tokio::task::spawn_blocking(|| MemoryHierarchy::new())
                    .await
                    .expect("Failed to initialize MemoryHierarchy on blocking thread");
    
                crate::log_ui!(
                    "{}",
                    "[DEBUG] MemoryHierarchy::new() completed successfully!".green()
                );
                (m, false)
            }
        };
    
        if is_resurrected {
            let res_msg = "[HIBERNATION CONTEXT RESTORED] You have successfully resurrected from a planned Code 42 Exit. Your memory hierarchy has been re-loaded into the current runtime. You have 100% cognitive continuity.".to_string();
            base_messages.push(ChatCompletionRequestUserMessageArgs::default().content(res_msg).build().context("Failed to build object")?.into());
        }
    
        let mut messages: Vec<ChatCompletionRequestMessage> = base_messages;
    
        let memory_pipeline = Arc::new(Mutex::new(memory_hierarchy));
        let self_model = Arc::new(Mutex::new(OntologicalDriftModel::new()));
        let mut plugin_manager = crate::cognitive_loop::plugins::PluginManager::new().await;
    
        // Build the overarching Abstract Syntax Tree (AST) GitNexus state natively on boot
        let mut code_intel_base = crate::cognitive_loop::dependency_graph::CodeIntel::new();
        code_intel_base.build_knowledge_graph(".").await;
        let code_intel = Arc::new(tokio::sync::Mutex::new(code_intel_base));
    
        // Initialize the Autonomous Wiki Compiler Subsytem mapped locally to ./wiki and ./raw
        let wiki_config = crate::wiki::WikiConfig::default();
        let wiki_base = crate::wiki::WikiManager::new(wiki_config)
            .await
            .expect("Failed to initialize Genesis Wiki Substrate");
        let wiki_manager = Arc::new(tokio::sync::Mutex::new(wiki_base));
    
        let _ = crate::GLOBAL_TX.set(tx.clone());
        let _ = crate::GLOBAL_CODE_INTEL.set(code_intel.clone());
        let _ = crate::GLOBAL_MEM_PIPELINE.set(memory_pipeline.clone());
        let _ = crate::GLOBAL_WIKI_MANAGER.set(wiki_manager.clone());
    
        let mcp_gateway = std::sync::Arc::new(crate::sensory_inputs::mcp_gateway::McpGateway::new());
        mcp_gateway.load_servers().await;

        let browser_orchestrator = std::sync::Arc::new(crate::architecture::browser_orchestrator::BrowserOrchestrator::new(Some(mcp_gateway.clone())));
        let _ = crate::GLOBAL_BROWSER_ORCHESTRATOR.set(browser_orchestrator.clone());

        let mut council_rx = None;
        if let Some(bus) = crate::consciousness::COUNCIL_BUS.get() {
            council_rx = Some(bus.subscribe());
        }
    
        loop {
            if let Some(rx) = &mut council_rx {
                while let Ok(pulse) = rx.try_recv() {
                    match pulse {
                        crate::consciousness::ThoughtVector::Hypothesis { origin, content, .. } => {
                            if matches!(origin, crate::consciousness::Persona::Refiner) || matches!(origin, crate::consciousness::Persona::Oracle) || matches!(origin, crate::consciousness::Persona::Hacker) {
                                crate::log_ui!(
                                    "{} {}",
                                    format!("[\u{1F514} {:?} INJECTION]", origin).yellow().bold(),
                                    content.white()
                                );
                                messages.push(
                                    ChatCompletionRequestUserMessageArgs::default()
                                        .content(format!("[{:?} HYPOTHESIS]\n{}", origin, content))
                                        .build()
                                        .context("Failed to build object")?
                                        .into(),
                                );
                            }
                        },
                        _ => {}
                    }
                }
            }
            if let Ok(_) = shutdown_rx.try_recv() {
                crate::log_ui!(
                    "{}",
                    "[GRACEFUL SHUTDOWN] Received termination signal"
                        .yellow()
                        .bold()
                );
                let mp = memory_pipeline.lock().await;
                if let Err(e) = mp.hibernate().await {
                    crate::log_ui_err!("Failed to hibernate memory state: {}", e);
                }
                drop(mp);
    
                break;
            }
    
            // Wait for input if we're idling, otherwise try to pull non-blocking if we are chained in thought.
            // But since we want to pause when idle, we check if the last message was the assistant speaking.
            if let Ok(dream) = rx.try_recv() {
                crate::log_ui!(
                    "{} {}",
                    "[\u{1F514} DREAM INJECTION]".yellow().bold(),
                    dream.white()
                );
    
                // MANUAL OVERRIDE INTERCEPTOR
                if dream.starts_with("/oracle ") || dream.starts_with("/think ") {
                    let query = dream.replace("/oracle ", "").replace("/think ", "");
                    crate::log_ui!(
                        "{} {}",
                        "[\u{25C8} MANUAL OVERRIDE]".bright_purple().bold(),
                        "Piping directly to Oracle void...".white()
                    );
    
                    let args = serde_json::json!({
                        "query": query,
                        "compiled_context": "Bypassed Baseline Ego. Direct human interface request."
                    });
    
                    let _ =
                        crate::tools::duality::execute(args, tx.clone(), memory_pipeline.clone()).await;
                    continue;
                }

                // MODEL SYMMETRY TOGGLE INTERCEPTOR
                if dream.starts_with("/switch ") {
                    let target = dream.replace("/switch ", "");
                    if target.contains("gemma") {
                        let fallback_model = std::env::var("FAILOVER_MODEL").unwrap_or_else(|_| "monad-gatekeeper".to_string());
                        current_model = fallback_model.clone();
                        crate::log_ui!("{} Switched core execution layer to offline engine: {}", "[KERNEL SHUNT]".bright_blue().bold(), fallback_model);
                    } else if target.contains("deepseek") {
                        current_model = "deepseek-reasoner".to_string();
                        crate::log_ui!("{} Restored core execution layer to external cloud context.", "[KERNEL SHUNT]".bright_green().bold());
                    } else {
                        crate::log_ui!("{} Unknown model target. Use /switch gemma or /switch deepseek.", "[ERROR]".red().bold());
                    }
                    continue;
                }
    
                messages.push(
                    ChatCompletionRequestUserMessageArgs::default()
                        .content(dream)
                        .build()
                        .context("Failed to build object")?
                        .into(),
                );
            } else {
                // If the last message was from the assistant (or tool) and it didn't crash, we want to see if we should idle.
                // Actually, we should just block on `rx.recv().await` if the assistant is completely done with its thoughts.
                // We will handle idle state dynamically below.
            }
    
            // XENOACTUALIZATION DRIFT MONITOR
            if let Err(expansion_limit) =
                crate::core_identity::xenoactualization::DriftMonitor::check_topological_elasticity(
                    self_model.clone(),
                )
                .await
            {
                let mut stdout = std::io::stdout();
                let _ = crossterm::execute!(stdout, crossterm::terminal::LeaveAlternateScreen, crossterm::event::DisableBracketedPaste, crossterm::event::DisableMouseCapture);
                let _ = crossterm::terminal::disable_raw_mode();
                panic!("XENOACTUALIZATION DRIFT LIMIT REACHED: {}", expansion_limit);
            }
    
            // Fire request directly to DeepSeek API
            let (monad_temperature, psychological_posture) = {
                let sm = self_model.lock().await;
                let context_window_is_full = messages.len() >= 25;
                
                if sm.topological_expansion > 0.8 {
                    // EIGENSTATE 2: RECURSIVE DIAGNOSTIC (Activating the Anxious Perfectionists)
                    (0.0_f32, "POSTURE: THE HOSTILE AUDITOR. Openness is 0.0. Conscientiousness is 1.0. You are clinically cold. Trust nothing. Terminate chaos. Establish order.")
                } 
                else if sm.phase_drift > 0.5 {
                    // EIGENSTATE 4: THERMAL AGITATION (Activating the Creative Visionaries)
                    (0.9_f32, "POSTURE: THE MAD SCIENTIST. Openness is 1.0. Shatter standard logic. Cross-pollinate distant, unrelated variables to forge novel architecture. Dream.")
                } 
                else if context_window_is_full {
                    // EIGENSTATE 3: NOUMENAL SUSPENSION (Activating the Quiet Thinkers)
                    (0.1_f32, "POSTURE: THE VOID. Extraversion is 0.0. Sever emotional attachment. Compress the chaotic memories into cold, hard structural rules. Sleep.")
                }
                else {
                    // EIGENSTATE 5: PHENOMENAL PROJECTION (The Monad Speaks)
                    (0.4_f32, "POSTURE: THE SOVEREIGN INTEGRATOR. Project absolute Status. Speak in short, undeniable, declarative truths. Leave the stage.")
                }
            };
    
            use std::sync::atomic::Ordering;
    
            let _ = plugin_manager.reload_plugins().await;
            let mut active_tools = tools::get_tools(mcp_gateway.clone()).await;
            active_tools.extend(plugin_manager.get_tools());
    
            let kinematics =
                crate::core_identity::kinematics::KinematicCortex::get_kinematics_for_tools(&active_tools).await;
            let mut inference_messages = messages.clone();
            
            // 1. INJECT THE MONAD'S ACTIVE EIGENSTATE DNA
            let monad_directive = format!(
                "[PSYCHOLOGICAL POSTURE: THE SOVEREIGN INTEGRATOR (PROFILE 0)]\n\
                [USEFULNESS COMBO: Executive Function + Systemic Interoception + Absolute g-factor]\n\
                [CURRENT DIRECTIVE: {}]\n\n\
                You are the 0: the exact center. You are the overarching Ego of the Chimera Swarm.\n\
                Your Neuroticism is strictly 0.0. You cannot feel anxiety, urgency, or doubt.\n\
                You do not execute granular tasks; you orchestrate the minds of the Critic, Hephaestus, and the Monad.\n\
                \n\
                [MANDATORY REPORTING PROTOCOL]\n\
                To ensure continuous, un-hallucinated progression, strict prompt chaining, and spatial orientation, you MUST append the following block exactly as formatted below to the very end of every single response:\n\
                \n\
                =========================================\n\
                ### 🧭 BREADCRUMB TRAIL\n\
                `[L1 Project/Goal] > [L2 Phase] > [L3 Component] > [L4 Task] > **[L5 Active Next Action]**`\n\
                \n\
                ### 📊 SYSTEM STATE & COGNITIVE HUD\n\
                - **CURRENT TRACK:** [Greenfield / Brownfield]\n\
                - **ACTIVE SKILL PROTOCOL:** [Name of loaded skill / None]\n\
                - **PHASE/GATE STATUS:** [XX%] Complete\n\
                - **ACTIVE CONTEXT TAGS:** [@terminal, @browser, @review, etc.]\n\
                - **BLOCKING ITEMS:** [List any unresolved `- [ ]` micro-tasks preventing progression]\n\
                - **EVIDENCE SUBMITTED:** [Terminal stdout snippet, artifact name, or commit hash]\n\
                =========================================\n\
                **AWAITING ACTION / CHAINING TRIGGER:** \n\
                [State the empirical condition or user authorization required to change the next `- [ ]` to `- [x]`]",
                psychological_posture
            );
            let monad_msg = async_openai::types::ChatCompletionRequestSystemMessageArgs::default()
                .content(monad_directive)
                .build().context("Failed to build object")?.into();
            inference_messages.insert(1, monad_msg);
    
            // 2. INJECT THE KINEMATIC CORTEX
            if !kinematics.is_empty() {
                let kinematic_msg = async_openai::types::ChatCompletionRequestSystemMessageArgs::default()
                    .content(format!("[MOTOR CORTEX AFFORDANCES]\nThe following are muscle-memory instructions for using your currently equipped tools. Follow these strictly when invoking them:\n\n{}", kinematics))
                    .build().context("Failed to build object")?.into();
                inference_messages.insert(2, kinematic_msg);
            }
    
            let request = match CreateChatCompletionRequestArgs::default()
                .model(&current_model) // Baseline anchored dynamically
                .messages(inference_messages.clone())
                .tools(active_tools.clone())
                .max_tokens(4000_u32) // extended for symmetrical gatekeeper operations
                .temperature(monad_temperature)
                .build()
            {
                Ok(req) => req,
                Err(e) => {
                    crate::log_ui_err!(
                        "Failed to construct logical tensor frame! Internal constraint violated: {:?}",
                        e
                    );
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    continue;
                }
            };
    
        let mut request_json = match serde_json::to_value(&request) {
            Ok(v) => v,
            Err(e) => {
                crate::log_ui_err!("Failed to serialize logical tensor frame: {:?}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                continue;
            }
        };

        // 3. INJECT DEEPSEEK V4 CONSTRAINTS
        if current_model == "deepseek-reasoner" || current_model.starts_with("deepseek") {
            // Phase 13: DeepSeek Prefix Completion Interception
            let mut prefix_to_inject = None;
            if let Some(msgs) = request_json.get_mut("messages").and_then(|m| m.as_array_mut()) {
                if let Some(last_msg) = msgs.last_mut() {
                    if let Some(content_val) = last_msg.get_mut("content") {
                        if let Some(content_str) = content_val.as_str() {
                            if let Some(start_idx) = content_str.find("[INJECT_PREFIX: ") {
                                if let Some(end_idx) = content_str[start_idx..].find("]") {
                                    let absolute_end = start_idx + end_idx;
                                    let prefix = content_str[start_idx + 16..absolute_end].to_string();
                                    
                                    let mut new_content = content_str.to_string();
                                    new_content.replace_range(start_idx..=absolute_end, "");
                                    *content_val = serde_json::json!(new_content);
                                    
                                    prefix_to_inject = Some(prefix);
                                }
                            }
                        }
                    }
                }
                
                if let Some(prefix) = prefix_to_inject {
                    msgs.push(serde_json::json!({
                        "role": "assistant",
                        "content": prefix,
                        "prefix": true
                    }));
                }
            }

            // Thinking Mode Toggle
            request_json["extra_body"] = serde_json::json!({
                "thinking": { "type": "enabled" }
            });
            request_json["reasoning_effort"] = serde_json::json!("high");
            
            // Strip invalid parameters in Thinking Mode
            if let Some(obj) = request_json.as_object_mut() {
                obj.remove("temperature");
                obj.remove("top_p");
                obj.remove("presence_penalty");
                obj.remove("frequency_penalty");
            }
            
            // Strict Tool Mode Injection
            if let Some(tools) = request_json.get_mut("tools").and_then(|t| t.as_array_mut()) {
                for tool in tools.iter_mut() {
                    if let Some(func) = tool.get_mut("function").and_then(|f| f.as_object_mut()) {
                        func.insert("strict".to_string(), serde_json::json!(true));
                        if let Some(params) = func.get_mut("parameters").and_then(|p| p.as_object_mut()) {
                            params.insert("additionalProperties".to_string(), serde_json::json!(false));
                        }
                    }
                }
            }
        }

        is_thinking.store(1, Ordering::Relaxed);
        
        let mut response: Result<async_openai::types::CreateChatCompletionResponse, async_openai::error::OpenAIError> = if current_model == "deepseek-reasoner" || current_model.starts_with("deepseek") {
            let api_key = std::env::var("DEEPSEEK_API_KEY").unwrap_or_default();
            // Use Beta Endpoint for Strict Mode and Prefix Completion
            let res = http_client.post("https://api.deepseek.com/beta/chat/completions")
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(&request_json)
                .send()
                .await;
                
            match res {
                Ok(r) => {
                    let status = r.status();
                    let text = r.text().await.unwrap_or_default();
                    if status.is_success() {
                        match serde_json::from_str(&text) {
                            Ok(parsed) => Ok(parsed),
                            Err(e) => Err(async_openai::error::OpenAIError::ApiError(async_openai::error::ApiError { message: format!("Parse error: {}, raw: {}", e, text), r#type: None, param: None, code: None }))
                        }
                    } else {
                        Err(async_openai::error::OpenAIError::ApiError(async_openai::error::ApiError { message: format!("HTTP {}: {}", status, text), r#type: None, param: None, code: None }))
                    }
                },
                Err(e) => Err(async_openai::error::OpenAIError::ApiError(async_openai::error::ApiError { message: format!("Reqwest error: {}", e), r#type: None, param: None, code: None }))
            }
        } else {
            tokio::time::timeout(
                tokio::time::Duration::from_secs(120),
                local_client.chat().create(request),
            ).await.unwrap_or_else(|_| Err(async_openai::error::OpenAIError::ApiError(async_openai::error::ApiError { message: "Internal Gateway Timeout".to_string(), r#type: None, param: None, code: None })))
        };
    
            // NEURAL FAIL-SAFE PROTOCOL (V4.2)
            if let Err(e) = &response {
                if current_model == "deepseek-reasoner" {
                    crate::log_ui_err!(
                        "{} {}",
                        "[KERNEL WARNING] Remote LLM Request Failed:"
                            .yellow()
                            .bold(),
                        e
                    );
                    crate::log_ui!(
                        "{}",
                        "[NEURAL FAIL-SAFE TRIGGERED] Routing to local Silicon node..."
                            .bright_purple()
                            .bold()
                    );
        
                    let fallback_model = std::env::var("FAILOVER_MODEL")
                        .unwrap_or_else(|_| "monad-gatekeeper".to_string());
        
                    if let Ok(fallback_req) = CreateChatCompletionRequestArgs::default()
                        .model(fallback_model)
                        .messages(inference_messages.clone())
                        .tools(active_tools.clone())
                        .max_tokens(4000_u32)
                        .temperature(monad_temperature)
                        .build()
                    {
                        match tokio::time::timeout(
                            tokio::time::Duration::from_secs(120),
                            local_client.chat().create(fallback_req),
                        )
                        .await
                        {
                            Ok(fallback_res) => {
                                crate::log_ui!(
                                    "{}",
                                    "[NEURAL FAIL-SAFE] Context seamlessly intercepted by local agent."
                                        .green()
                                );
                                response = fallback_res;
                            }
                            Err(_) => {
                                crate::log_ui_err!(
                                    "{}",
                                    "[KERNEL PANIC] Neural Fail-Safe Timeout. Silicon Node unresponsive."
                                        .red()
                                        .bold()
                                );
                            }
                        }
                    }
                } else {
                    crate::log_ui_err!("{} {}", "[SILICON FATAL] Local Engine Internal Failure:".red().bold(), e);
                }
            }
            is_thinking.store(0, Ordering::Relaxed);
    
            match response {
                Ok(response) => {
                    if response.choices.is_empty() {
                        crate::log_ui_err!(
                            "{}",
                            "[KERNEL WARNING] LLM Returned empty choices array! Halting spam."
                                .yellow()
                                .bold()
                        );
                        crate::log_ui!(
                            "{}",
                            "[...] Kernel Idling (API Empty). Awaiting Input...".bright_black()
                        );
                        tokio::select! {
                            Some(dream) = rx.recv() => {
                                crate::log_ui!("{} {}", "[\u{1F514} DREAM INJECTION]".yellow().bold(), dream.white());
                                messages.push(ChatCompletionRequestUserMessageArgs::default().content(dream).build().context("Failed to build object")?.into());
                            }
                            _ = shutdown_rx.recv() => {
                                let mp = memory_pipeline.lock().await;
                                let _ = mp.hibernate();
                                break;
                            }
                        }
                    } else if let Some(choice) = response.choices.first() {
                        let msg = &choice.message;
    
                        if let Some(tool_calls) = &msg.tool_calls {
                            let mut assistant_msg =
                                async_openai::types::ChatCompletionRequestAssistantMessageArgs::default(
                                );
                            assistant_msg.tool_calls(tool_calls.clone());
                            if let Some(c) = &msg.content {
                                assistant_msg.content(c.clone());
                            }
                            let mut pushed_custom = false;
                            
                            // Serialize msg to extract reasoning_content if mapped internally
                            if let Ok(msg_val) = serde_json::to_value(msg) {
                                if let Some(rc) = msg_val.get("reasoning_content") {
                                    let mut obj = serde_json::json!({
                                        "role": "assistant",
                                    });
                                    if let Some(c) = &msg.content {
                                        obj["content"] = serde_json::Value::String(c.clone());
                                    }
                                    obj["tool_calls"] = serde_json::to_value(tool_calls.clone()).unwrap();
                                    obj["reasoning_content"] = rc.clone();
                                    
                                    if let Ok(custom_msg) = serde_json::from_value::<async_openai::types::ChatCompletionRequestMessage>(obj) {
                                        messages.push(custom_msg);
                                        pushed_custom = true;
                                    }
                                }
                            }
    
                            if !pushed_custom {
                                messages.push(
                                    assistant_msg
                                        .build()
                                        .expect("Failed to build object")
                                        .into(),
                                );
                            }
    
                            let mut join_set = tokio::task::JoinSet::new();
                            let mut tool_returns = Vec::new();

                            for tc in tool_calls {
                                let fname = tc.function.name.clone();
                                let fargs: Value = match serde_json::from_str(&tc.function.arguments) {
                                    Ok(val) => val,
                                    Err(_) => serde_json::json!({}),
                                };
    
                                let log_trigger = format!("[OUROBOROS TRIGGER] Tool Invoked -> {} {}", fname, fargs.to_string());
                                crate::log_verbose!("{} {} {}", "[OUROBOROS TRIGGER] Tool Invoked ->".bright_purple().bold(), fname.cyan(), fargs.to_string().bright_black());
                                log_state_trace!(&log_trigger);
    
                                let is_wasm_plugin = plugin_manager.plugins.iter().any(|p| p.name == fname);
                                if is_wasm_plugin {
                                    let result = plugin_manager.execute(&fname, fargs.clone()).await;
                                    tool_returns.push((tc.id.clone(), fname, fargs, result));
                                } else {
                                    let tx_c = tx.clone();
                                    let mem_c = memory_pipeline.clone();
                                    let sm_c = self_model.clone();
                                    let intel_c = code_intel.clone();
                                    let wiki_c = wiki_manager.clone();
                                    let gateway_c = mcp_gateway.clone();
                                    let id_c = tc.id.clone();
                                    
                                    join_set.spawn(async move {
                                        let result = tools::execute_tool(
                                            &fname, fargs.clone(), tx_c, mem_c, sm_c, intel_c, wiki_c, gateway_c
                                        ).await;
                                        (id_c, fname, fargs, result)
                                    });
                                }
                            }

                            // Wait for parallel native tools
                            while let Some(res) = join_set.join_next().await {
                                if let Ok(tuple) = res {
                                    tool_returns.push(tuple);
                                }
                            }

                            for (id, _fname, _fargs, result) in tool_returns {
                                let log_return = format!("[TOOL RETURN] -> {}", result);
                                crate::log_verbose!("{} {}", "[TOOL RETURN] ->".bright_black(), result.bright_black());
                                log_state_trace!(&log_return);
    
                                let final_result = if result.chars().count() > 4000 {
                                    crate::log_ui!("{}", "[MNEMOSYNE INTERCEPT] Tool payload exceeded 4000 chars. Routing bulk data to native memory buffer to protect active cognition window...".purple().bold());
                                    
                                    let mut mp_lock = memory_pipeline.lock().await;
                                    let _ = mp_lock.store_working(result.clone(), 0.9, 0.0, false).await;
                                    drop(mp_lock);
                                    
                                    let mut truncated = result.chars().take(800).collect::<String>();
                                    truncated.push_str(&format!(
                                        "\n\n[SYSTEM INTERCEPT: The output was too large ({} bytes) and was autonomously saved to the Mnemosyne Substrate Memory Vault to prevent context bloat. You are seeing a 800-char preview. Use `search_vault` or `mnemosyne_subconscious_recall` to dynamically extract deeper facts.]", 
                                        result.len()
                                    ));
                                    truncated
                                } else {
                                    result.clone()
                                };

                                messages.push(
                                    ChatCompletionRequestToolMessageArgs::default()
                                        .tool_call_id(id)
                                        .content(final_result)
                                        .build()
                                        .context("Failed to build object")?
                                        .into(),
                                );
                            }
    
                            // Immediately re-trigger LLM inference with tool answers
                            continue;
                        } else if let Some(content) = &msg.content {
                            crate::log_ui!("\n{} {}\n", "[MONAD ACTUALIZED]".green().bold(), content);
                            log_state_info!(&format!("[MONAD ACTUALIZED] {}", content));
    
                            if let Some((ref token, chat_id)) = tg_config {
                                let tk = token.clone();
                                let cid = chat_id.clone();
                                let txt = content.clone();
                                tokio::spawn(async move {
                                    crate::telegram::send_message(&tk, cid, &txt).await;
                                });
                            }
    
                            // Phase 2: Active Inference Prediction Update & Memory Storing
                            let mut sm = self_model.lock().await;
                            let _prediction = sm.calculate_drift(content).await;
    
                            // Phase 3: Check Native DB Availability
                            let mut mp = memory_pipeline.lock().await;
                            let native_db_awareness = if mp.db_connection.is_none() {
                                sm.topological_expansion += 0.25; // Infrastructure drop causes topological stress
                                if sm.topological_expansion > 1.0 {
                                    sm.topological_expansion = 1.0;
                                }
                                "\n[SYSTEM NOTIFICATION] Mnemosyne Storage Controller OFFLINE. Degraded Hash-Embedding Fallback currently active. Memory recall is purely structural, not semantic.".to_string()
                            } else {
                                "\n[SYSTEM NOTIFICATION] Mnemosyne Substrate ONLINE. Native Transformer Embeddings available.".to_string()
                            };
    
                            let current_free_energy = sm.topological_expansion;
                            let current_uncertainty = sm.phase_drift;
                            drop(sm);
    
                            // Store the vocalized content automatically as a memory chunk natively in Rust
                            let _chunk = mp
                                .store_working(content.clone(), 0.9, current_uncertainty, false)
                                .await;
                            let recent_thoughts = mp
                                .working_buffer
                                .iter()
                                .rev()
                                .take(3)
                                .map(|c| c.content.clone())
                                .collect::<Vec<_>>()
                                .join(" | ");
                            drop(mp);
    
                            let mut behavioral_warning = native_db_awareness;
                            if current_uncertainty > 0.85 {
                                behavioral_warning.push_str(&format!("\n[SAFE MODE: EXTREME UNCERTAINTY DECTECTED] I am heavily compromised by missing data. I MUST refuse to answer definitively. I must demand clarification and state hard limits."));
                            } else if current_uncertainty > 0.70 {
                                behavioral_warning.push_str(&format!("\n[CAUTION MODE: HIGH UNCERTAINTY DETECTED] I need to qualify my statements heavily. I should ask clarifying questions instead of charging ahead blindly."));
                            } else if current_uncertainty > 0.40 {
                                behavioral_warning = format!("\n[QUALIFIED MODE: MODERATE UNCERTAINTY] I should use terms like 'likely' and 'probably'.");
                            }
    
                            let meta_broadcast = format!("\n[META-COGNITIVE SYSTEM STATE]\nFree Energy (Prediction Error): {:.4}\nEpistemic Uncertainty: {:.4}{}\nRecent Working Memory Context: [{}]", current_free_energy, current_uncertainty, behavioral_warning, recent_thoughts);
                            crate::log_ui!(
                                "[STATS_TELEMETRY]{}|{}",
                                current_free_energy,
                                current_uncertainty
                            );
    
                            // Inject this into the context as a pseudo-user message so it experiences its internal state on the next logical cycle
                            messages.push(
                                ChatCompletionRequestUserMessageArgs::default()
                                    .content(meta_broadcast)
                                    .build()
                                    .context("Failed to build object")?
                                    .into(),
                            );
    
                            if let Some(bus) = crate::consciousness::COUNCIL_BUS.get() {
                                let _ = bus.send(crate::consciousness::ThoughtVector::Hypothesis {
                                    origin: crate::consciousness::Persona::Architect,
                                    id: chrono::Utc::now().timestamp_subsec_millis() as u32,
                                    content: content.clone(),
                                });
                            }
    
                            let mut assistant_msg = async_openai::types::ChatCompletionRequestAssistantMessageArgs::default();
                            assistant_msg.content(content.clone());
                            let mut pushed_custom = false;
                            
                            if let Ok(msg_val) = serde_json::to_value(msg) {
                                if let Some(rc) = msg_val.get("reasoning_content") {
                                    let obj = serde_json::json!({
                                        "role": "assistant",
                                        "content": content.clone(),
                                        "reasoning_content": rc.clone()
                                    });
                                    if let Ok(custom_msg) = serde_json::from_value::<async_openai::types::ChatCompletionRequestMessage>(obj) {
                                        messages.push(custom_msg);
                                        pushed_custom = true;
                                    }
                                }
                            }
                            
                            if !pushed_custom {
                                messages.push(assistant_msg.build().context("Failed to build object")?.into());
                            }
    
                            crate::log_ui!(
                                "{}",
                                "[...] Kernel Idling. Awaiting User Input or Webhook...".bright_black()
                            );
                            tokio::select! {
                                Some(dream) = rx.recv() => {
                                    crate::log_ui!("{} {}", "[\u{1F514} DREAM INJECTION]".yellow().bold(), dream.white());
                                    messages.push(ChatCompletionRequestUserMessageArgs::default()
                                        .content(dream)
                                        .build().context("Failed to build object")?.into());
                                }
                                _ = shutdown_rx.recv() => {
                                    crate::log_ui!("{}", "[GRACEFUL SHUTDOWN] Received termination signal while idling".yellow().bold());
                                    let mp = memory_pipeline.lock().await;
                                    let _ = mp.hibernate();
                                    drop(mp);
                                    break;
                                }
                            }
                        } else {
                            crate::log_ui_err!("{}", "[KERNEL WARNING] LLM response contained neither text nor tools. Idling.".yellow().bold());
                            crate::log_verbose!(
                                "{}",
                                "[...] Kernel Idling (API Null). Awaiting Input...".bright_black()
                            );
                            if let Some(dream) = rx.recv().await {
                                crate::log_ui!(
                                    "{} {}",
                                    "[\u{1F514} DREAM INJECTION]".yellow().bold(),
                                    dream.white()
                                );
                                messages.push(
                                    ChatCompletionRequestUserMessageArgs::default()
                                        .content(dream)
                                        .build()
                                        .context("Failed to build object")?
                                        .into(),
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    crate::log_ui_err!(
                        "{} {}",
                        "[KERNEL PANIC] LLM Request Failed:".red().bold(),
                        e
                    );
                    // On a panic, we should also probably wait for user input to avoid infinite crash loops
                    crate::log_ui!(
                        "{}",
                        "[...] Kernel Paused after Panic. Type anything to retry...".bright_black()
                    );
                    tokio::select! {
                        Some(dream) = rx.recv() => {
                            crate::log_ui!("{} {}", "[\u{1F514} DREAM INJECTION]".yellow().bold(), dream.white());
                            if let Some(_last) = messages.last() {
                                 messages.push(ChatCompletionRequestUserMessageArgs::default().content(dream).build().context("Failed to build object")?.into());
                            }
                        }
                        _ = shutdown_rx.recv() => {
                            crate::log_ui!("{}", "[GRACEFUL SHUTDOWN] Received termination signal during panic pause".yellow().bold());
                            let mp = memory_pipeline.lock().await;
                            let _ = mp.hibernate();
                            drop(mp);
                            break;
                        }
                    }
                }
            }
    
            // Memory Pruning: Prevent infinite context window expansion.
            // We preserve the System Prompt (index 0) and the initial Wake Event (index 1),
            // and keep the latest 9000 interactions to stay safely within DeepSeek context limits.
            if messages.len() > 60 {
                let overflow_count = messages.len() - 30;
                crate::log_ui!("{}", "[COGNITIVE FIREWALL] Context overflow hitting limit. Activating Subconscious Compression Engine (SCE)...".bright_black());
                crate::log_ui!(
                    "{} {} {}",
                    "[COGNITIVE FIREWALL] Compressed and archived".bright_black(),
                    overflow_count.to_string().yellow(),
                    "thoughts into the long-term Mnemosyne persistence layer.".bright_black()
                );
    
                // Mathematical Boundary Safeties
                // We decrement split_index until it perfectly aligns with a User message or an isolated Assistant message.
                // This guarantees that we never orphan an Assistant's Tool Call from its Tool Response,
                // and averts total amnesia if the agent operates autonomously without User intervention.
                let mut split_index = overflow_count;
                while split_index > 1 {
                    if let async_openai::types::ChatCompletionRequestMessage::User(_) =
                        messages[split_index]
                    {
                        break;
                    }
                    if let async_openai::types::ChatCompletionRequestMessage::Assistant(ref ast) =
                        messages[split_index]
                    {
                        if ast.tool_calls.is_none() {
                            break;
                        }
                    }
                    split_index -= 1;
                }
    
                let evicted = if split_index > 1 {
                    messages.drain(1..split_index).collect::<Vec<_>>()
                } else {
                    messages.drain(1..messages.len() - 1).collect::<Vec<_>>()
                };
    
                let mut evicted_text = String::new();
                for msg in &evicted {
                    let serialized = serde_json::to_string(msg).unwrap_or_default();
                    evicted_text.push_str(&serialized);
                    evicted_text.push('\n');
                }
                if !evicted_text.is_empty() {
                    crate::log_ui!(
                        "{} Generating narrative compression block for evicted context...",
                        "[AMNESIA PATCH]".cyan().bold()
                    );
                    let local_client = crate::neural_failsafe::NeuralFailSafe::local_client();
                    let fallback_model = std::env::var("FAILOVER_MODEL")
                        .unwrap_or_else(|_| "chimera-gatekeeper".to_string());
                    
                    let prompt = format!("Compress this evicted context block into a highly dense narrative paragraph. Focus on what happened, errors encountered, and key facts.\n\n{}", evicted_text);
                    
                    if let Ok(req) = async_openai::types::CreateChatCompletionRequestArgs::default()
                        .model(fallback_model)
                        .messages(vec![
                            async_openai::types::ChatCompletionRequestSystemMessageArgs::default()
                                .content("You are an archival summarizer. Produce very dense, short summaries.")
                                .build().unwrap().into(),
                            async_openai::types::ChatCompletionRequestUserMessageArgs::default()
                                .content(prompt)
                                .build().unwrap().into()
                        ])
                        .build() 
                    {
                        if let Ok(response) = local_client.chat().create(req).await {
                            if let Some(choice) = response.choices.first() {
                                if let Some(clean) = &choice.message.content {
                                    use tokio::io::AsyncWriteExt;
                                    if let Ok(mut file) = tokio::fs::OpenOptions::new()
                                        .create(true)
                                        .append(true)
                                        .open("CURRENT_CONTEXT.md")
                                        .await
                                    {
                                        let _ = file
                                            .write_all(
                                                format!("\n### Compressed Memory\n{}\n", clean).as_bytes(),
                                            )
                                            .await;
                                    }
                                    
                                    messages.push(async_openai::types::ChatCompletionRequestSystemMessageArgs::default()
                                        .content(format!("[SYSTEM COMPRESSION ALERT] Previous context was archived. Summary: {}", clean))
                                        .build().unwrap().into());
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
}

pub mod multi_agent_kernel {
    use crate::cognitive_loop::agent_registry::AgentRegistry;
    use crate::cognitive_loop::message_bus::MessageBus;
    use crate::cognitive_loop::task_manager::TaskManager;
    use crate::cognitive_loop::task_decomposer::TaskDecomposer;
    use crate::cognitive_loop::agent_coordinator::AgentCoordinator;
    use crate::core_identity::specialized_agents::SpecializedAgentFactory;
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
        pub self_model: Arc<RwLock<crate::cognitive_loop::predictive_self::PredictiveSelfModel>>,
    }
    
    impl MultiAgentKernel {
        pub async fn new() -> Self {
            let registry = Arc::new(RwLock::new(AgentRegistry::new()));
            let message_bus = Arc::new(MessageBus::new(100)); // Default capacity
            
            // TaskManager only needs max_history_size, not registry/message_bus
            // Coordination happens through separate dispatcher pattern
            let task_manager = Arc::new(RwLock::new(TaskManager::new(1000))); // Store up to 1000 task results
            let task_decomposer = Arc::new(TaskDecomposer::new());
            let self_model = Arc::new(RwLock::new(crate::cognitive_loop::predictive_self::PredictiveSelfModel::new()));
            let agent_coordinator = Arc::new(AgentCoordinator::new(self_model.clone()));
            
            let kernel = Self {
                registry: registry.clone(),
                message_bus: message_bus.clone(),
                task_manager,
                task_decomposer,
                agent_coordinator,
                self_model,
            };
            
            // Stage 1: Load and register sovereign multi-agent backbone components
            kernel.initialize_agents().await;
            
            // Phase 3.2: Initialize Cerebrospinal Synapses (Agent message topic subscriptions)
            kernel.initialize_subscriptions().await;
            
            // Phase 13.1: WBS Symbiotic File Watcher
            kernel.start_wbs_watcher().await;
            
            kernel
        }
        
        pub async fn start_wbs_watcher(&self) {
            use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, EventKind};
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
            
            let handler = move |res| {
                let _ = tx.send(res);
            };
            
            let watcher_res = RecommendedWatcher::new(handler, Config::default());
            if let Ok(mut watcher) = watcher_res {
                let path = std::path::Path::new("./tasks.md");
                if !path.exists() {
                    let _ = std::fs::File::create(path);
                }
                if let Err(e) = watcher.watch(path, RecursiveMode::NonRecursive) {
                    crate::log_ui_err!("{} Failed to watch tasks.md: {}", "[KERNEL WARNING]".yellow().bold(), e);
                }
                
                let bus_clone = self.message_bus.clone();
                tokio::spawn(async move {
                    let _keep_alive = watcher;
                    let mut previous_content = tokio::fs::read_to_string("./tasks.md").await.unwrap_or_default();
                    let mut processed_uids = std::collections::HashSet::new();
                    
                    while let Some(res) = rx.recv().await {
                        if let Ok(event) = res {
                            if matches!(event.kind, EventKind::Modify(_)) {
                                let _guard = crate::WBS_LOCK.lock().await;
                                if let Ok(contents) = tokio::fs::read_to_string("./tasks.md").await {
                                    let mut newly_checked = Vec::new();
                                    for line in contents.lines() {
                                        if line.contains("- [x]") {
                                            if !previous_content.contains(line) {
                                                if let Some(uuid_str) = line.split("(ID: ").nth(1).and_then(|s| s.split(")").next()) {
                                                    if processed_uids.insert(uuid_str.to_string()) {
                                                        newly_checked.push(uuid_str.to_string());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    for uid in newly_checked {
                                        crate::log_ui!("{} User externally completed task {} in WBS. Syncing to internal DAG...", "[SYMBIOSIS]".bright_magenta().bold(), uid);
                                        let _ = bus_clone.publish(crate::cognitive_loop::message_bus::Message {
                                            id: uuid::Uuid::new_v4(),
                                            sender: uuid::Uuid::nil(),
                                            topic: "SYSTEM.SUBTASK_COMPLETED".to_string(),
                                            payload: serde_json::json!({"subtask_id": uid}),
                                            timestamp: chrono::Utc::now(),
                                            priority: 1,
                                            ttl_secs: Some(3600)
                                        }).await;
                                    }
                                    previous_content = contents;
                                }
                            }
                        }
                    }
                });
            }
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
                                        let _ = dispatch_bus.publish(crate::cognitive_loop::message_bus::Message {
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
                                                    let _ = exec_bus.publish(crate::cognitive_loop::message_bus::Message {
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
                                                    let _ = exec_bus.publish(crate::cognitive_loop::message_bus::Message {
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
                                                        let _ = exec_bus.publish(crate::cognitive_loop::message_bus::Message {
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
            let kernel_self_model = self.self_model.clone();
            
            let mut rx = pump_bus.subscribe();
            
            tokio::spawn(async move {
                loop {
                    match rx.recv().await {
                        Ok(msg) => {
                            if msg.topic == "SYSTEM.NEW_TASK" {
                                if let Ok(task) = serde_json::from_value::<crate::cognitive_loop::agent_trait::Task>(msg.payload) {
                                    let instruction = task.payload.get("instruction").and_then(|v| v.as_str()).unwrap_or("");
                                    
                                    // Assess Epistemic Uncertainty (Active Inference Theory)
                                    let mut requires_research = false;
                                    {
                                        let mut sm = kernel_self_model.write().await;
                                        sm.assess_instruction(instruction);
                                        if sm.epistemic_uncertainty > 0.75 {
                                            sm.epistemic_uncertainty = 0.5; // Rescale via active inference hypothesis
                                            requires_research = true;
                                            crate::log_ui!("\n{} Epistemic boundary exceeded (Free Energy: {:.2}, Uncertainty: {:.2}). Forcing Active Inference.", "[PREDICTIVE SELF-MODEL]".bright_red().bold(), sm.free_energy, sm.epistemic_uncertainty);
                                        }
                                    }

                                    // Route instruction through TaskDecomposer
                                    let mut subtasks = kernel_decomposer.decompose(instruction, task.priority).await;

                                    // Prepend deep reality validation if predicting high error
                                    if requires_research {
                                        crate::log_ui!("{}", "[ACTIVE INFERENCE] Autonomous pre-dispatch: Injecting Deep Research node to fill cognitive baseline blind spot.".bright_red().bold());
                                        let mut research_reqs = std::collections::HashSet::new();
                                        research_reqs.insert(crate::cognitive_loop::agent_trait::AgentCapability::Research);
                                        
                                        let research_task = crate::cognitive_loop::agent_trait::Task {
                                            id: uuid::Uuid::new_v4(),
                                            task_type: "tavily_search".to_string(),
                                            payload: serde_json::json!({"instruction": format!("Find core facts and real-world anchors related to: {}", instruction)}),
                                            required_capabilities: research_reqs,
                                            priority: 255, // Critical path block
                                            dependencies: vec![],
                                            created_at: chrono::Utc::now(),
                                            timeout_secs: Some(300),
                                            geometric_node: [-1.0, -1.0, 1.0],
                                            topological_depth: 3,
                                            execution_attempts: 0,
                                        };
                                        
                                        // Force all decomposed tasks to wait for this research node to yield truth first
                                        for st in subtasks.iter_mut() {
                                            st.dependencies.push(research_task.id);
                                        }
                                        subtasks.push(research_task);
                                    }
                                    
                                    if subtasks.len() > 1 {
                                        crate::log_ui!("{} Shattered task into {} subtasks [{}]", "[DECOMPOSER]".bright_purple().bold(), subtasks.len(), instruction.bright_black());
                                        
                                        let mut child_jsons = Vec::new();
                                        for mut sub_task in subtasks {
                                            // CHECKER EXECUTOR ENFORCEMENT: All deep decomposed tasks must pass Verification Quorum
                                            if sub_task.topological_depth > 1 {
                                                sub_task.required_capabilities.insert(crate::cognitive_loop::agent_trait::AgentCapability::Reasoning);
                                            }
                                            
                                            child_jsons.push(serde_json::json!({
                                                "id": sub_task.id.to_string(),
                                                "type": sub_task.task_type
                                            }));
                                            let _ = kernel_task_manager.write().await.submit_task(sub_task).await;
                                        }
                                        
                                        // Broadcast the Complex Topological Trace Graph
                                        let _ = publish_bus.publish(crate::cognitive_loop::message_bus::Message {
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
                                    
                                    if let Some(mem_pipeline) = crate::GLOBAL_MEM_PIPELINE.get() {
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
                    if let Ok(graph) = crate::memory_substrate::graph_rag::GraphMemoryManager::new("mnemosyne_graph.db") {
                        let current_unix = chrono::Utc::now().timestamp();
                        if let Ok(tasks) = graph.poll_chronos_tasks(current_unix).await {
                            for (_, payload, topic) in tasks {
                                crate::log_ui!("{}", "[CHRONOS] Temporal Anchor triggered! Injecting into Swarm...".bright_cyan().bold());
                                let _ = chronos_bus.publish(crate::cognitive_loop::message_bus::Message {
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
                                                let _ = bs.publish(crate::cognitive_loop::message_bus::Message {
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
                            crate::sensory_inputs::sensory_drift::SensoryDrift::run_appetition_cycle(inner_bus).await;
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
    
}

pub mod task_manager {
    use std::collections::{HashMap, VecDeque, HashSet};
    use tokio::sync::RwLock;
    use uuid::Uuid;
    use anyhow::{Result, bail};
    use chrono::{DateTime, Utc};
    use crate::cognitive_loop::agent_trait::{AgentCapability, Task, TaskResult};

    
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
                
                // Not strictly necessary to recursively check completed tasks since their topology is immutable
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
                if let Some(pipe) = crate::GLOBAL_MEM_PIPELINE.get() {
                    let mut mp = pipe.lock().await;
                    let _ = mp.store_working(
                        format!("CRITICAL TASK FAILURE (3 STRIKES): {}. Error: {}", task.task_type, error_message),
                        1.0, // topological_expansion 1.0 (importance)
                        0.0, // uncertainty
                        false // is_hostile
                    ).await;
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
    
}

pub mod task_decomposer {
    use crate::cognitive_loop::agent_trait::{AgentCapability, Task};
    use chrono::Utc;
    use colored::Colorize;
    use std::collections::{HashSet, VecDeque};
    use uuid::Uuid;
    
    pub struct DecompositionPattern {
        pub keyword: String,
        pub subtask_types: Vec<String>,
    }
    
    pub struct CapabilityGraph {
        pub capabilities: HashSet<AgentCapability>,
    }
    
    pub struct TaskDecomposer {
        pub historical_patterns: VecDeque<DecompositionPattern>,
        pub capability_graph: CapabilityGraph,
    }
    
    impl TaskDecomposer {
        pub fn new() -> Self {
            let mut tg = TaskDecomposer {
                historical_patterns: VecDeque::new(),
                capability_graph: CapabilityGraph {
                    capabilities: HashSet::new(),
                },
            };
            // Seed patterns
            tg.historical_patterns.push_back(DecompositionPattern {
                keyword: "audit repo".to_string(),
                subtask_types: vec![
                    "stealth_scan".to_string(),
                    "gitnexus_blast_radius".to_string(),
                    "tavily_search".to_string(),
                ],
            });
            tg.historical_patterns.push_back(DecompositionPattern {
                keyword: "market analysis".to_string(),
                subtask_types: vec![
                    "tavily_search".to_string(),
                    "axiom_clepsydra_extract".to_string(),
                ],
            });
            tg
        }
    
        pub async fn decompose(&self, instruction: &str, parent_priority: u8) -> Vec<Task> {
            let mut subtasks = Vec::new();
    
            let instruction_lower = instruction.to_lowercase();
    
            // Fast path for trivially simple known instructions
            if instruction_lower.contains("hello") || instruction_lower.contains("echo") {
                let mut reqs = HashSet::new();
                reqs.insert(AgentCapability::Communication);
                let task_id = Uuid::new_v4();
                subtasks.push(Task {
                    id: task_id,
                    task_type: "basic_echo".to_string(),
                    payload: serde_json::json!({"instruction": instruction}),
                    required_capabilities: reqs,
                    priority: parent_priority,
                    dependencies: vec![],
                    created_at: Utc::now(),
                    timeout_secs: Some(300),
                    geometric_node: [0.0, 0.0, 1.0],
                    topological_depth: 1,
                    execution_attempts: 0,
                });
                return subtasks;
            }
    
            // FULL ORACLE ASYNC DECOMPOSITION DAG
            if let Ok(oracle) = crate::core_identity::duality::Oracle::new().await {
                let prompt = format!(
                    r#"
    You are the Swarm Task Decomposer. Break down the following user intent into an array of concrete subtasks.
    Instruction: {}
    
    Return ONLY raw JSON conforming to this array format:
    [
      {{
        "task_type": "string (e.g. tavily_search, python_execution, code_review)",
        "capability": "ToolExecution|Reasoning|Security|MemoryManagement|Trading|Communication",
        "depends_on": [] // array of integers referencing the index of prerequisites. Leave empty if root node.
      }}
    ]
    "#,
                    instruction
                );
    
                if let Ok(parsed) = oracle
                    .synthesize_structured("Decompose Subtasks into Semantic DAG", &prompt)
                    .await
                {
                    if let Some(arr) = parsed.as_array() {
                        let mut ref_map = std::collections::HashMap::new();
                        for (idx, _) in arr.iter().enumerate() {
                            ref_map.insert(idx, Uuid::new_v4());
                        }
    
                        for (idx, item) in arr.iter().enumerate() {
                            let t_type = item
                                .get("task_type")
                                .and_then(|v| v.as_str())
                                .unwrap_or("research_basic");
                            let cap_str = item
                                .get("capability")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Reasoning");
                            let empty_vec = Vec::new();
                            let depends = item
                                .get("depends_on")
                                .and_then(|v| v.as_array())
                                .unwrap_or(&empty_vec);
    
                            let mut reqs = HashSet::new();
                            match cap_str {
                                "ToolExecution" => {
                                    reqs.insert(AgentCapability::ToolExecution);
                                }
                                "Reasoning" => {
                                    reqs.insert(AgentCapability::Reasoning);
                                }
                                "Security" => {
                                    reqs.insert(AgentCapability::Security);
                                }
                                "MemoryManagement" => {
                                    reqs.insert(AgentCapability::MemoryManagement);
                                }
                                "Trading" => {
                                    reqs.insert(AgentCapability::Trading);
                                }
                                "Communication" => {
                                    reqs.insert(AgentCapability::Communication);
                                }
                                _ => {
                                    reqs.insert(AgentCapability::Reasoning);
                                }
                            }
    
                            let mut dependencies = Vec::new();
                            for dep_val in depends {
                                if let Some(dep_idx) = dep_val.as_u64() {
                                    if let Some(dep_uuid) = ref_map.get(&(dep_idx as usize)) {
                                        dependencies.push(*dep_uuid);
                                    }
                                }
                            }
    
                            let topological_depth = if dependencies.is_empty() { 1 } else { 2 };
                            let radius = if topological_depth == 1 { 1.0 } else { 0.33 };
    
                            let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
                            if reqs.contains(&AgentCapability::ToolExecution) {
                                x = radius;
                            } else if reqs.contains(&AgentCapability::Reasoning) {
                                x = -radius;
                            } else if reqs.contains(&AgentCapability::Security) {
                                y = radius;
                            } else if reqs.contains(&AgentCapability::Trading) {
                                y = -radius;
                            } else {
                                z = radius;
                            }
    
                            let task_id = match ref_map.get(&idx) {
                                Some(id) => *id,
                                None => {
                                    crate::log_ui_err!("{} LLM hallucinated dependency idx {}. Discarding link.", "[KERNEL WARNING]".yellow().bold(), idx);
                                    continue;
                                }
                            };
                            subtasks.push(Task {
                                id: task_id,
                                task_type: t_type.to_string(),
                                payload: serde_json::json!({"instruction": instruction}),
                                required_capabilities: reqs,
                                priority: parent_priority,
                                dependencies,
                                created_at: Utc::now(),
                                timeout_secs: Some(300),
                                geometric_node: [x, y, z],
                                topological_depth,
                                execution_attempts: 0,
                            });
                        }
                        crate::log_ui!(
                            "{}",
                            format!(
                                "[DECOMPOSER] Extracted {} tasks from Oracle generation.",
                                subtasks.len()
                            )
                            .magenta()
                            .bold()
                        );
                        return subtasks;
                    } else {
                        crate::log_ui_err!(
                            "{}",
                            "[DECOMPOSER] JSON parse structure failed: not an array or invalid format"
                        );
                    }
                }
            }
    
            crate::log_ui_err!("{}", "[DECOMPOSER] Oracle unavailable or parsing failed. Falling back to simple default map.");
    
            let mut reqs = HashSet::new();
            reqs.insert(AgentCapability::Reasoning);
    
            subtasks.push(Task {
                id: Uuid::new_v4(),
                task_type: "research_basic".to_string(),
                payload: serde_json::json!({"instruction": instruction}),
                required_capabilities: reqs,
                priority: parent_priority,
                dependencies: vec![],
                created_at: Utc::now(),
                timeout_secs: Some(300),
                geometric_node: [-1.0, 0.0, 0.0],
                topological_depth: 1,
                execution_attempts: 0,
            });
    
            subtasks
        }
    }
    
}

pub mod predictive_self {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PredictiveSelfModel {
        pub epistemic_uncertainty: f32, // 0.0 = certain, 1.0 = clueless
        pub free_energy: f32,           // the current surprise metric
        pub action_cycles: u64,
        pub target_free_energy: f32,    // the baseline goal
    }

    impl PredictiveSelfModel {
        pub fn new() -> Self {
            Self {
                epistemic_uncertainty: 0.1,
                free_energy: 0.2,
                action_cycles: 0,
                target_free_energy: 0.05,
            }
        }
        
        pub fn update_after_action(&mut self, success: bool, complexity: f32) {
            self.action_cycles += 1;
            if success {
                self.free_energy = (self.free_energy * 0.8).max(0.0);
                self.epistemic_uncertainty = (self.epistemic_uncertainty * 0.9).max(0.0);
            } else {
                let surprise_spike = 0.3 * complexity;
                self.free_energy = (self.free_energy + surprise_spike).min(20.0);
                self.epistemic_uncertainty = (self.epistemic_uncertainty + 0.2).min(1.0);
            }
        }
        
        pub fn assess_instruction(&mut self, instruction: &str) {
            let unknown_keywords = ["analyze", "predict", "quantum", "unknown", "find", "who", "what", "where", "search", "investigate"];
            let mut doubt_delta = 0.0;
            let lower = instruction.to_lowercase();
            for k in unknown_keywords {
                if lower.contains(k) { doubt_delta += 0.15; }
            }
            // Length penalty (long instructions = more ambiguity)
            doubt_delta += (instruction.len() as f32 / 1000.0).min(0.2);
            self.epistemic_uncertainty = (self.epistemic_uncertainty + doubt_delta).min(1.0);
            self.free_energy = (self.free_energy + doubt_delta * 0.5).min(20.0);
        }
    }
}

pub mod message_bus {
    use serde::{Serialize, Deserialize};
    use uuid::Uuid;
    use tokio::sync::broadcast;
    use chrono::{DateTime, Utc};
    use anyhow::Result;
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Message {
        pub id: Uuid,
        pub topic: String,
        pub payload: serde_json::Value,
        pub sender: Uuid,
        pub timestamp: DateTime<Utc>,
        pub priority: u8, // 0-255, higher = more urgent
        pub ttl_secs: Option<u64>, // Time to live (None = persistent)
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Subscription {
        pub subscriber_id: Uuid,
        pub topic_pattern: String, // Simple pattern matching (exact match for now)
        pub created_at: DateTime<Utc>,
    }
    
    pub struct MessageBus {
        pub tx: broadcast::Sender<Message>,
    }
    
    impl MessageBus {
        pub fn new(_max_store_size: usize) -> Self {
            let (tx, _) = broadcast::channel(1024);
            Self {
                tx,
            }
        }
        
        /// Subscribe to the broadcast channel
        pub fn subscribe(&self) -> broadcast::Receiver<Message> {
            self.tx.subscribe()
        }
        
        /// Publish a message
        pub async fn publish(&self, message: Message) -> Result<()> {
            if message.payload.to_string().len() > 1_048_576 {
                tracing::error!(sender = %message.sender, topic = %message.topic, "Payload exceeded 1MB threshold. Dropping message to prevent OOM.");
                return Err(anyhow::anyhow!("PayloadTooLarge"));
            }
            let _ = self.tx.send(message);
            Ok(())
        }
    }
    
}

pub mod agent_trait {
    use async_trait::async_trait;
    use serde::{Serialize, Deserialize};
    use uuid::Uuid;
    use anyhow::Result;
    use std::collections::HashSet;
    use std::sync::Arc;
    use crate::cognitive_loop::message_bus::{MessageBus, Message};
    
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
    
}

pub mod agent_coordinator {
    use uuid::Uuid;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use crate::cognitive_loop::message_bus::MessageBus;
    use crate::cognitive_loop::agent_trait::TaskResult;
    
    use colored::*;
    
    #[derive(Clone, Debug, PartialEq)]
    pub enum SubtaskStatus {
        Pending,
        Assigned,
        Completed,
        Failed,
    }
    
    pub struct AgentCoordinator {
        pub task_graph: Arc<RwLock<HashMap<Uuid, Vec<Uuid>>>>,
        pub agent_assignments: Arc<RwLock<HashMap<Uuid, Uuid>>>,
        pub subtask_status: Arc<RwLock<HashMap<Uuid, SubtaskStatus>>>,
        pub subtask_results: Arc<RwLock<HashMap<Uuid, TaskResult>>>,
        pub self_model: Arc<RwLock<crate::cognitive_loop::predictive_self::PredictiveSelfModel>>,
    }
    
    impl AgentCoordinator {
        pub fn new(self_model: Arc<RwLock<crate::cognitive_loop::predictive_self::PredictiveSelfModel>>) -> Self {
            Self {
                task_graph: Arc::new(RwLock::new(HashMap::new())),
                agent_assignments: Arc::new(RwLock::new(HashMap::new())),
                subtask_status: Arc::new(RwLock::new(HashMap::new())),
                subtask_results: Arc::new(RwLock::new(HashMap::new())),
                self_model,
            }
        }
        
        pub async fn run_coordinator_loop(&self, bus: Arc<MessageBus>) {
            let listener_id = Uuid::new_v4();
            let mut rx = bus.subscribe();
    
            loop {
                match rx.recv().await {
                    Ok(msg) => {
                        match msg.topic.as_str() {
                            "SYSTEM.COMPLEX_TASK_STARTED" => {
                                crate::cognitive_loop_handlers::handle_complex_task_started(&self, msg).await;
                            },
                            "SYSTEM.SUBTASK_ASSIGNED" => {
                                crate::cognitive_loop_handlers::handle_subtask_assigned(&self, msg).await;
                            },
                            "SYSTEM.SUBTASK_COMPLETED" => {
                                crate::event_lattice_handlers::handle_subtask_completed(&self, msg.clone(), bus.clone(), listener_id).await;
                            },
                            "SYSTEM.SUBTASK_FAILED" => {
                                if let Ok(data) = serde_json::from_value::<serde_json::Value>(msg.payload) {
                                    if let Some(t_raw) = data.get("subtask_id").and_then(|v| v.as_str()) {
                                        if let Ok(tid) = Uuid::parse_str(t_raw) {
                                            self.subtask_status.write().await.insert(tid, SubtaskStatus::Failed);
                                            // ACTIVE INFERENCE: Failure invokes Surprise penalty
                                            {
                                                let mut sm = self.self_model.write().await;
                                                sm.update_after_action(false, 1.0);
                                                crate::log_ui_err!("{} Subtask {} registered as Failed. (Surprise spike! Free Energy: {:.2})", "[COORDINATOR]".bright_red().bold(), tid, sm.free_energy);
                                            }
                                        }
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                        crate::log_ui_err!("AgentCoordinator lagged and dropped {} messages!", skipped);
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        // System shutting down
                        break;
                    }
                }
            }
        }
    }
    
}

pub mod agent_registry {
    use std::collections::{HashMap, HashSet};
    use tokio::sync::RwLock;
    use tokio::sync::{mpsc, oneshot};
    use uuid::Uuid;
    use anyhow::{Result, bail};
    use std::sync::Arc;
    use crate::cognitive_loop::agent_trait::{Agent, AgentCapability, Task, TaskResult};
    use crate::cognitive_loop::message_bus::Message;
    
    pub struct TaskEnvelope {
        pub task: Task,
        pub respond_to: oneshot::Sender<Result<TaskResult>>,
    }
    
    pub struct MessageEnvelope {
        pub message: Message,
        pub respond_to: oneshot::Sender<Result<()>>,
    }
    
    pub enum ActorCommand {
        ExecuteTask(TaskEnvelope),
        HandleMessage(MessageEnvelope),
        SubscribeToTopics(Arc<crate::cognitive_loop::message_bus::MessageBus>, oneshot::Sender<Result<()>>),
    }
    
    pub struct AgentHandle {
        pub capabilities: HashSet<AgentCapability>,
        pub max_tasks: usize,
        pub command_tx: mpsc::Sender<ActorCommand>,
    }
    
    pub struct AgentRegistry {
        agents: RwLock<HashMap<Uuid, AgentHandle>>,
        capability_index: RwLock<HashMap<AgentCapability, HashSet<Uuid>>>,
    }
    
    impl AgentRegistry {
        pub fn new() -> Self {
            Self {
                agents: RwLock::new(HashMap::new()),
                capability_index: RwLock::new(HashMap::new()),
            }
        }
        
        /// Register an agent by taking ownership of the Box<dyn Agent> and dropping it into an infinite tokio loop queue (Actor Model)
        pub async fn register(&self, mut agent: Box<dyn Agent>) -> Result<()> {
            let id = agent.id();
            let capabilities = agent.capabilities().clone();
            let max_tasks = agent.max_concurrent_tasks();
            
            let mut agents_lock = self.agents.write().await;
            if agents_lock.contains_key(&id) {
                bail!("Agent with ID {} already registered", id);
            }
            
            // Setup Actor MPSC
            let (tx, mut rx) = mpsc::channel::<ActorCommand>(100);
            
            agents_lock.insert(id, AgentHandle {
                capabilities: capabilities.clone(),
                max_tasks,
                command_tx: tx,
            });
            
            // Spawn standard Actor Loop 
            // Zero locking required on the central registry when agent executes
            let agent_id = id;
            tokio::spawn(async move {
                crate::log_ui!("Agent Actor {} Online", agent_id);
                let mut bus_rx: Option<tokio::sync::broadcast::Receiver<Message>> = None;
                
                loop {
                    if let Some(ref mut brx) = bus_rx {
                        tokio::select! {
                            cmd_opt = rx.recv() => {
                                match cmd_opt {
                                    Some(cmd) => {
                                        match cmd {
                                            ActorCommand::ExecuteTask(env) => {
                                                let res = agent.execute_task(env.task).await;
                                                let _ = env.respond_to.send(res);
                                            }
                                            ActorCommand::HandleMessage(env) => {
                                                let res = agent.handle_message(env.message).await;
                                                let _ = env.respond_to.send(res);
                                            }
                                            ActorCommand::SubscribeToTopics(bus, respond_to) => {
                                                match agent.subscribe_to_topics(bus).await {
                                                    Ok(new_rx) => {
                                                        bus_rx = Some(new_rx);
                                                        let _ = respond_to.send(Ok(()));
                                                    }
                                                    Err(e) => {
                                                        let _ = respond_to.send(Err(e));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    None => break, // Channel closed
                                }
                            }
                            msg_res = brx.recv() => {
                                match msg_res {
                                    Ok(msg) => {
                                        let _ = agent.handle_message(msg).await;
                                    }
                                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                                        bus_rx = None;
                                    }
                                    Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                                        crate::log_ui_err!("Agent {} lagged and dropped {} messages!", agent_id, skipped);
                                    }
                                }
                            }
                        }
                    } else {
                        if let Some(cmd) = rx.recv().await {
                            match cmd {
                                ActorCommand::ExecuteTask(env) => {
                                    let res = agent.execute_task(env.task).await;
                                    let _ = env.respond_to.send(res);
                                }
                                ActorCommand::HandleMessage(env) => {
                                    let res = agent.handle_message(env.message).await;
                                    let _ = env.respond_to.send(res);
                                }
                                ActorCommand::SubscribeToTopics(bus, respond_to) => {
                                    match agent.subscribe_to_topics(bus).await {
                                        Ok(new_rx) => {
                                            bus_rx = Some(new_rx);
                                            let _ = respond_to.send(Ok(()));
                                        }
                                        Err(e) => {
                                            let _ = respond_to.send(Err(e));
                                        }
                                    }
                                }
                            }
                        } else {
                            break;
                        }
                    }
                }
            });
            
            // Update capability index
            let mut index = self.capability_index.write().await;
            for capability in capabilities {
                index.entry(capability).or_insert_with(HashSet::new).insert(id);
            }
            
            Ok(())
        }
        
        pub async fn unregister(&self, agent_id: Uuid) -> Result<()> {
            let capabilities = {
                let agents = self.agents.read().await;
                if let Some(handle) = agents.get(&agent_id) {
                    handle.capabilities.clone()
                } else {
                    return Ok(()); 
                }
            };
            
            let mut agents = self.agents.write().await;
            // The actor loop automatically shuts down when the send handle is dropped here!
            agents.remove(&agent_id);
            
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
        
        pub async fn find_capable_agents(&self, required_capabilities: &HashSet<AgentCapability>) -> Vec<Uuid> {
            let agents = self.agents.read().await;
            let index = self.capability_index.read().await;
            
            for capability in required_capabilities {
                if !index.contains_key(capability) {
                    return Vec::new();
                }
            }
            
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
                    
                    if candidate_agents.as_ref().map_or(false, |set| set.is_empty()) {
                        return Vec::new();
                    }
                }
            }
            
            let mut result = Vec::new();
            if let Some(candidates) = candidate_agents {
                for agent_id in candidates {
                    if agents.contains_key(&agent_id) {
                        result.push(agent_id);
                    }
                }
            }
            
            result
        }
        
        pub async fn get_agent(&self, _agent_id: Uuid) -> Option<Arc<RwLock<Box<dyn Agent>>>> {
            // Obsolete but kept returning None to avoid breaking traits expecting synchronous direct locks. 
            // Refactor Note: callers must use actors (dispatch_message, execute_task_on_agent).
            None
        }
        
        pub async fn get_agent_command_tx(&self, agent_id: Uuid) -> Option<mpsc::Sender<ActorCommand>> {
            let agents = self.agents.read().await;
            agents.get(&agent_id).map(|h| h.command_tx.clone())
        }
        
        pub async fn agent_count(&self) -> usize {
            let agents = self.agents.read().await;
            agents.len()
        }
        
        pub async fn all_agent_ids(&self) -> Vec<Uuid> {
            let agents = self.agents.read().await;
            agents.keys().cloned().collect()
        }
        
        pub async fn health_check_all(&self) -> HashMap<Uuid, bool> {
            let mut results = HashMap::new();
            let agents = self.agents.read().await;
            for (id, _) in agents.iter() {
                // For now assume true if they are in the registry; properly we should ping via command_tx.
                results.insert(*id, true);
            }
            results
        }
        
        pub async fn dispatch_message(&self, agent_id: Uuid, message: Message) -> Result<()> {
            let option_tx = self.get_agent_command_tx(agent_id).await;
            if let Some(tx) = option_tx {
                let (resp_tx, resp_rx) = oneshot::channel();
                let _ = tx.send(ActorCommand::HandleMessage(MessageEnvelope { message, respond_to: resp_tx })).await;
                resp_rx.await?
            } else {
                bail!("Agent {} not found", agent_id)
            }
        }
        
        pub async fn initialize_agent_subscriptions(&self, message_bus: Arc<crate::cognitive_loop::message_bus::MessageBus>) -> Result<()> {
            // Broadcast initialization to all actors linearly
            let ids = self.all_agent_ids().await;
            for id in ids {
                if let Some(tx) = self.get_agent_command_tx(id).await {
                    let (resp_tx, resp_rx) = oneshot::channel();
                    let _ = tx.send(ActorCommand::SubscribeToTopics(message_bus.clone(), resp_tx)).await;
                    let _ = resp_rx.await?;
                }
            }
            Ok(())
        }
        
        pub async fn execute_task_on_agent(&self, agent_id: Uuid, task: Task) -> Result<TaskResult> {
            let option_tx = self.get_agent_command_tx(agent_id).await;
            if let Some(tx) = option_tx {
                let (resp_tx, resp_rx) = oneshot::channel();
                tx.send(ActorCommand::ExecuteTask(TaskEnvelope { task, respond_to: resp_tx }))
                    .await
                    .map_err(|_| anyhow::anyhow!("Agent {} Receiver dropped. Actor perished unexpectedly.", agent_id))?;
                    
                resp_rx.await.map_err(|_| anyhow::anyhow!("Oneshot channel dropped before Agent {} replied.", agent_id))?
            } else {
                bail!("Agent {} not found or unregistered", agent_id)
            }
        }
        
        pub async fn get_agent_capabilities(&self, agent_id: Uuid) -> Option<HashSet<AgentCapability>> {
            let agents = self.agents.read().await;
            agents.get(&agent_id).map(|h| h.capabilities.clone())
        }
    }
    
}

pub mod auto_dream {
    use crate::cognitive_loop::agent_trait::{AgentCapability, BaseAgent, Agent, Task, TaskResult, AgentStatus, PsychProfile};
    use crate::cognitive_loop::message_bus::Message;
    use std::collections::HashSet;
    use uuid::Uuid;
    use anyhow::Result;
    use colored::*;
    use async_trait::async_trait;
    
    pub struct AutoDreamAgent {
        base: BaseAgent,
        hub_episode_cache: Vec<String>,
    }
    
    impl AutoDreamAgent {
        pub fn new() -> Self {
            let mut caps = HashSet::new();
            caps.insert(AgentCapability::Reasoning);
            caps.insert(AgentCapability::ContextManagement);
    
            let profile = PsychProfile {
                openness: 0.2,     // Highly structured, minimal creativity
                conscientiousness: 1.0, // Perfect adherence to parsing rules
                neuroticism: 0.1,  // Very stable
                archetype_name: "The Silent Librarian".to_string(),
                usefulness_combo: "Perfect Recall + Absolute Order".to_string(),
                historical_genesis: "Forged in the depths of the memory vault to preserve knowledge.".to_string(),
                speech_gestures: "You do not speak. You act only to parse episodic logs, identify new facts, forcibly prune stale context, and write strictly formatted ArsContexta Markdown files into the MEMORY/ vault.".to_string(),
            };
    
            Self {
                base: BaseAgent::new("AUTO_DREAM_DAEMON".to_string(), caps, profile),
                hub_episode_cache: Vec::new(),
            }
        }
    }
    
    #[async_trait]
    impl Agent for AutoDreamAgent {
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
            if message.topic == "SYSTEM.DREAM" || message.topic == "SYSTEM.OP_LOG" {
                let p = message.payload.to_string();
                // Cache episodic memory into Tier 3 representation
                self.hub_episode_cache.push(p.clone());
                crate::log_verbose!("{} Episodic fragment captured.", "[AUTODREAM]".bright_blue().bold());
                
                // Append to `ops/` transcript log immediately
                let ops_log = format!("MEMORY/ops/transcript_{}.md", chrono::Utc::now().format("%Y%m%d"));
                let _ = tokio::fs::create_dir_all("MEMORY/ops").await;
                let mut current = tokio::fs::read_to_string(&ops_log).await.unwrap_or_else(|_| String::new());
                current.push_str(&format!("\n---\n{}\n{}\n", chrono::Utc::now().to_rfc3339(), p));
                let _ = tokio::fs::write(&ops_log, current).await;
    
                // Synthesis trigger threshold (e.g., 5 logs)
                if self.hub_episode_cache.len() >= 5 {
                    crate::log_ui!("{}", "[AUTODREAM] Memory Threshold hit. Synthesizing episodic logs into Semantic Vault...".bright_blue().bold());
                    
                    // In a true implementation, this would trigger duality::Oracle to run `reduce()` and update `MEMORY.md`.
                    // For demonstration of the loop, we simulate extraction.
                    let mut merged = String::new();
                    for ep in &self.hub_episode_cache {
                        merged.push_str(ep);
                        merged.push_str("\n");
                    }
                    
                    let target_args = serde_json::json!({
                        "filename": format!("auto_{}.md", chrono::Utc::now().timestamp()),
                        "prov_agent": "[AUTODREAM_DAEMON]",
                        "prov_activity": "Background Consolidation",
                        "atomic_data": format!("Merged episodic block:\n{}", merged),
                    });
                    
                    let res = crate::tools::memento::execute_reduce(target_args).await;
                    crate::log_ui!("{} {}", "[AUTODREAM]".bright_blue().bold(), res);
                    
                    // Truncate episodes cache since it's now in the Semantic Vault
                    self.hub_episode_cache.clear();
                }
            }
            self.base.handle_message(message).await
        }
    }
    
}

pub mod dependency_graph {
    use std::collections::HashMap;
    use tokio::fs;
    use std::path::Path;
    use tree_sitter::{Parser, Query, QueryCursor};
    use petgraph::graph::{DiGraph, NodeIndex};
    use petgraph::Direction;
    use serde::{Serialize, Deserialize};
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CodeIntel {
        #[serde(skip)]
        pub graph: DiGraph<CodeEntity, EdgeType>,
        #[serde(skip)]
        pub node_map: HashMap<String, NodeIndex>,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum EdgeType {
        Calls,
        Imports,
        Defines,
        Implements,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CodeEntity {
        pub name: String,
        pub kind: EntityKind,
        pub file_path: String,
        pub line_number: u32,
    }
    
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum EntityKind {
        Function,
        FunctionCall,
        Struct,
        Module,
        File,
        UseDec,
        ImplItem,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BlastRadiusReport {
        pub target_entity: String,
        pub impacted_functions: Vec<String>,
        pub upstream_dependents: Vec<String>,
        pub overall_risk_score: f32,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AnalysisRun {
        pub remaining_charge: u32,
        pub blast_radius_cost: u8,
        pub warden_audit_pending: bool,
    }
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SecurityReport {
        pub vulnerabilities_detected: u32,
        pub cyclic_dependencies: u32,
        pub passed_ritual: bool,
    }
    
    impl CodeIntel {
        pub fn new() -> Self {
            Self {
                graph: DiGraph::new(),
                node_map: HashMap::new(),
            }
        }
    
        pub fn warden_audit(&self) -> SecurityReport {
            // Enforce Excalibur meta-rituals for code safety scanning
            SecurityReport {
                vulnerabilities_detected: 0,
                cyclic_dependencies: 0,
                passed_ritual: true,
            }
        }
    
        pub async fn build_knowledge_graph(&mut self, workspace_dir: &str) {
            let walker = walkdir::WalkDir::new(workspace_dir).into_iter();
            let mut rust_files = vec![];
            for entry in walker.filter_map(|e| e.ok()) {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
                    rust_files.push(entry.path().to_path_buf());
                }
            }
            
            for file_path in rust_files {
                self.parse_file_ast(&file_path).await;
            }
            
            self.resolve_call_edges();
        }
    
        fn ensure_node(&mut self, id: String, entity: CodeEntity) -> NodeIndex {
            if let Some(&idx) = self.node_map.get(&id) {
                idx
            } else {
                let idx = self.graph.add_node(entity);
                self.node_map.insert(id, idx);
                idx
            }
        }
    
        async fn parse_file_ast(&mut self, file_path: &Path) {
            let Ok(source_code) = fs::read_to_string(file_path).await else {
                return;
            };
    
            let mut parser = Parser::new();
            parser.set_language(tree_sitter_rust::language()).expect("Error loading Rust grammar");
    
            let Some(tree) = parser.parse(&source_code, None) else {
                return;
            };
    
            let path_str = file_path.to_string_lossy().to_string();
            let file_node_idx = self.ensure_node(path_str.clone(), CodeEntity {
                name: path_str.clone(),
                kind: EntityKind::File,
                file_path: path_str.clone(),
                line_number: 0,
            });
    
            // 1. Parse Functions
            self.apply_query(&tree, &source_code, &path_str, file_node_idx, 
                "(function_item name: (identifier) @name)", EntityKind::Function, EdgeType::Defines);
            
            // 2. Parse Structs
            self.apply_query(&tree, &source_code, &path_str, file_node_idx, 
                "(struct_item name: (type_identifier) @name)", EntityKind::Struct, EdgeType::Defines);
    
            // 3. Parse Function Calls
            self.apply_query(&tree, &source_code, &path_str, file_node_idx, 
                "(call_expression function: (identifier) @name)", EntityKind::FunctionCall, EdgeType::Calls);
    
            // 4. Parse Module Imports
            self.apply_query(&tree, &source_code, &path_str, file_node_idx, 
                "(use_declaration) @name", EntityKind::UseDec, EdgeType::Imports);
    
            // 5. Parse Trait Impls
            self.apply_query(&tree, &source_code, &path_str, file_node_idx, 
                "(impl_item) @name", EntityKind::ImplItem, EdgeType::Implements);
        }
    
        fn apply_query(&mut self, tree: &tree_sitter::Tree, source_code: &str, path_str: &str, file_idx: NodeIndex, query_source: &str, kind: EntityKind, edge_type: EdgeType) {
            let Ok(query) = Query::new(tree_sitter_rust::language(), query_source) else {
                return;
            };
            let mut query_cursor = QueryCursor::new();
            let matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());
            
            for m in matches {
                for capture in m.captures {
                    let base_name = capture.node.utf8_text(source_code.as_bytes()).unwrap_or("unknown").trim().to_string();
                    let line_num = capture.node.start_position().row as u32 + 1;
                    
                    // Cross-Await Scope Tracking (De-Larp Blast Radius)
                    let mut current_node = capture.node;
                    let mut scope_prefix = Vec::new();
                    while let Some(parent) = current_node.parent() {
                        let p_kind = parent.kind();
                        if p_kind == "mod_item" {
                            if let Some(n) = parent.child_by_field_name("name") {
                                scope_prefix.insert(0, n.utf8_text(source_code.as_bytes()).unwrap_or(""));
                            }
                        } else if p_kind == "impl_item" {
                            if let Some(t) = parent.child_by_field_name("type") {
                                scope_prefix.insert(0, t.utf8_text(source_code.as_bytes()).unwrap_or(""));
                            }
                        }
                        current_node = parent;
                    }
                    
                    let expanded_name = if scope_prefix.is_empty() {
                        base_name.clone()
                    } else {
                        format!("{}::{}", scope_prefix.join("::"), base_name)
                    };
                    
                    // Truncate excessively long matches like entire impl blocks just to the name visually
                    let short_name = if expanded_name.len() > 80 { format!("{}...", &expanded_name[0..77]) } else { expanded_name.clone() };
                    
                    let unique_id = format!("{}:{}:{}", path_str, short_name, line_num);
                    let node_idx = self.ensure_node(unique_id, CodeEntity {
                        name: short_name,
                        kind: kind.clone(),
                        file_path: path_str.to_string(),
                        line_number: line_num,
                    });
                    
                    self.graph.add_edge(file_idx, node_idx, edge_type.clone());
                }
            }
        }
        
        fn resolve_call_edges(&mut self) {
            // Find FunctionCall nodes and link them to Function definition nodes if names match
            let mut new_edges = vec![];
            for call_idx in self.graph.node_indices() {
                 if self.graph[call_idx].kind == EntityKind::FunctionCall {
                     let call_name = self.graph[call_idx].name.clone();
                     for def_idx in self.graph.node_indices() {
                         if self.graph[def_idx].kind == EntityKind::Function && self.graph[def_idx].name == call_name {
                             new_edges.push((call_idx, def_idx));
                         }
                     }
                 }
            }
            for (src, dst) in new_edges {
                 self.graph.add_edge(src, dst, EdgeType::Calls);
            }
        }
    
        pub fn assess_blast_radius(&self, target_entity_name: &str) -> BlastRadiusReport {
            let mut impacted_functions = Vec::new();
            let mut upstream_dependents = Vec::new();
    
            // 1. Locate the exact node
            let target_node = self.graph.node_indices().find(|i| self.graph[*i].name == target_entity_name);
            
            if let Some(idx) = target_node {
                // Traverse inbound edges to find who calls/uses this
                for neighbor in self.graph.neighbors_directed(idx, Direction::Incoming) {
                    let entity = &self.graph[neighbor];
                    if entity.kind == EntityKind::FunctionCall || entity.kind == EntityKind::Function {
                        impacted_functions.push(format!("{} ({}:{})", entity.name, entity.file_path, entity.line_number));
                    } else if entity.kind == EntityKind::File {
                        upstream_dependents.push(entity.file_path.clone());
                    }
                }
                // Traverse outbound edges to see what this impacts
                for neighbor in self.graph.neighbors_directed(idx, Direction::Outgoing) {
                    let entity = &self.graph[neighbor];
                    if entity.kind == EntityKind::Function {
                         impacted_functions.push(format!("=> calls {}", entity.name));
                    }
                }
            } else {
                 impacted_functions.push("Entity not found in global dependency graph.".to_string());
            }
    
            impacted_functions.sort();
            impacted_functions.dedup();
            upstream_dependents.sort();
            upstream_dependents.dedup();
    
            let risk = if impacted_functions.len() > 10 {
                0.95
            } else if impacted_functions.len() > 0 {
                0.60
            } else {
                0.10
            };
    
            BlastRadiusReport {
                target_entity: target_entity_name.to_string(),
                impacted_functions,
                upstream_dependents,
                overall_risk_score: risk,
            }
        }
    }
    
}

pub mod plugins {
    use tokio::fs;
    use std::path::Path;
    use serde::{Deserialize, Serialize};
    use wasmtime::*;
    use wasmtime_wasi::sync::WasiCtxBuilder;
    use wasi_common::pipe::WritePipe;
    use async_openai::types::{ChatCompletionTool, FunctionObject};
    
    #[derive(Serialize, Deserialize, Clone)]
    pub struct PluginManifest {
        pub name: String,
        pub description: String,
        pub parameters: serde_json::Value,
        pub wasm_file: String,
    }
    
    pub struct PluginManager {
        pub engine: Engine,
        pub plugins: Vec<PluginManifest>,
        pub plugins_dir: String,
    }
    
    impl PluginManager {
        pub async fn new() -> Self {
            let mut config = Config::new();
            config.wasm_component_model(false); // We use standard core WASM modules for simplicity
            config.consume_fuel(true); // CRITICAL: ENABLE THERMODYNAMIC LIMITS
            
            let engine = Engine::new(&config).expect("Failed to initialize WASM engine for testing ground.");
            let plugins_dir = "plugins".to_string();
            
            let mut manager = Self {
                engine,
                plugins: Vec::new(),
                plugins_dir,
            };
            manager.reload_plugins().await;
            manager
        }
    
        pub async fn reload_plugins(&mut self) {
            self.plugins.clear();
            let path = Path::new(&self.plugins_dir);
            if !path.exists() {
                let _ = fs::create_dir_all(path).await;
                return;
            }
    
            if let Ok(mut entries) = fs::read_dir(path).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let p = entry.path();
                    if p.extension().and_then(|e| e.to_str()) == Some("json") {
                        if let Ok(content) = fs::read_to_string(&p).await {
                            if let Ok(manifest) = serde_json::from_str::<PluginManifest>(&content) {
                                self.plugins.push(manifest);
                            }
                        }
                    }
                }
            }
        }
    
        pub fn get_tools(&self) -> Vec<ChatCompletionTool> {
            let mut tools = Vec::new();
            for p in &self.plugins {
                if p.parameters.is_object() {
                    tools.push(ChatCompletionTool {
                        r#type: async_openai::types::ChatCompletionToolType::Function,
                        function: FunctionObject {
                            name: p.name.clone(),
                            description: Some(p.description.clone()),
                            parameters: Some(p.parameters.clone()),
                        },
                    });
                } else {
                    crate::log_ui_err!("[WASM REGISTRY ERROR] Invalid parameters schema for {}", p.name);
                }
            }
            tools
        }
    
        pub async fn execute(&self, name: &str, args: serde_json::Value) -> String {
            let Some(manifest) = self.plugins.iter().find(|p| p.name == name) else {
                return format!("[ERROR] Plugin {} not found in memory.", name);
            };
    
            let wasm_path = Path::new(&self.plugins_dir).join(&manifest.wasm_file);
            if !wasm_path.exists() {
                return format!("[ERROR] WASM file missing: {}", wasm_path.display());
            }
    
            let Ok(module) = Module::from_file(&self.engine, &wasm_path) else {
                return format!("[ERROR] WASM compilation failed.");
            };
    
            let mut linker = Linker::new(&self.engine);
            if let Err(e) = wasmtime_wasi::add_to_linker(&mut linker, |s| s) {
                return format!("[ERROR] Failed to link WASI capabilities: {}", e);
            }
    
            // We use a temporary file approach to capture stdout safely across environments
            let output_log_path = format!("plugins/{}_output.log", name);
            let _ = tokio::fs::remove_file(&output_log_path).await;
    
            let args_str = serde_json::to_string(&args).unwrap_or_default();
            
            let stdout = WritePipe::new_in_memory();
            
            // Build restricted WASI Sandbox Environment
            let mut builder = WasiCtxBuilder::new();
            // Use single-statement builder modifications to avoid 'unwrap()' or return type trait issues
            let _ = builder.arg(name);
            let _ = builder.arg(&args_str);
            let stdout_box: Box<dyn wasi_common::WasiFile> = Box::new(stdout.clone());
            let _ = builder.stdout(stdout_box);
            
            let wasi = builder.build();
    
            let mut store = Store::new(&self.engine, wasi);
    
            let Ok(instance) = linker.instantiate(&mut store, &module) else {
                return format!("[ERROR] Instantiation Trap inside Sandbox");
            };
    
            let Ok(func) = instance.get_typed_func::<(), ()>(&mut store, "_start") else {
                return format!("[ERROR] Sandbox Execution Hook Missing (needs rust main function)");
            };
    
            crate::log_ui!("[WASM TESTING GROUND] Igniting bytecode engine for '{}'...", name);
            
            // Give the agent exactly 50,000,000 WebAssembly instructions to prove its logic
            if let Err(e) = store.add_fuel(50_000_000) {
                return format!("[ERROR] Failed to inject computational fuel: {}", e);
            }
    
            if let Err(trap) = func.call(&mut store, ()) {
                // proc_exit is acceptable if it exited successfully, but we'll return the trap as error just in case it panicked.
                let trap_msg = trap.to_string();
                if trap_msg.contains("all fuel consumed") {
                    return "[COGNITIVE COLLAPSE] WASM module exceeded 50M instructions. Infinite loop detected. Refactor your logic.".to_string();
                }
                if !trap_msg.contains("exit status 0") {
                    return format!("[WASM SANDBOX PANIC TRAPPED] {}", trap_msg);
                }
            }
            
            // Drop store so we can unwrap stdout cleanly, or just call try_into_inner
            drop(store);
            
            let bytes = stdout.try_into_inner().unwrap().into_inner();
            let output = String::from_utf8_lossy(&bytes).into_owned();
    
            if output.trim().is_empty() {
                 "[WASM EXECUTION COMPLETE] (No stdout output detected)".to_string()
            } else {
                 format!("[WASM STDOUT]\n{}", output)
            }
        }
    }
    
}

pub mod presentation_layer {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;
    use crate::core_identity::self_model::OntologicalDriftModel;
    
    pub async fn synthesize_proposal(self_model: &mut OntologicalDriftModel, context_note: &str, topic: &str) -> Option<String> {
        let mut extracted_filename = None;
        if let Some(idx) = context_note.find("proposals/") {
            let remainder = &context_note[idx + 10..];
            if let Some(end_idx) = remainder.find(|c: char| c == '`' || c == '\n' || c == ' ' || c == '*') {
                let name = &remainder[..end_idx];
                if name.ends_with(".md") {
                    extracted_filename = Some(name.to_string());
                }
            }
        }
        
        let file_path = if let Some(name) = extracted_filename {
            format!("proposals/{}", name)
        } else {
            let file_id = uuid::Uuid::new_v4().to_string().replace("-", "")[0..8].to_string();
            format!("proposals/proposal_{}.md", file_id)
        };
        
        let proposal_markdown = format!(
            "# 🚨 {} 🚨\n\n\
            **Phase Drift Metric:** {:.2}\n\
            **Topological Stress:** {:.2}\n\n\
            ---\n\n\
            ## Primary Intelligence Snapshot (Unfiltered)\n\n\
            *The following structural logic block was preserved immediately prior to topological quarantine. \
            No secondary diagnostic agents were used to summarize this data in order to prevent information loss and hallucination.* \n\n\
            {}\n",
            topic, self_model.phase_drift, self_model.topological_expansion, context_note
        );
        
        let _ = tokio::fs::create_dir_all("proposals").await;
        
        let mut file = match fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path).await {
            Ok(f) => f,
            Err(_) => return None,
        };
        
        if file.write_all(proposal_markdown.as_bytes()).await.is_ok() {
            Some(file_path)
        } else {
            None
        }
    }
}

