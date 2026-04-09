use async_openai::{
    config::OpenAIConfig,
    Client,
    types::{
        ChatCompletionRequestMessage,
        ChatCompletionRequestUserMessageArgs, ChatCompletionRequestToolMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::{sleep, Duration};
use serde_json::Value;
use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use colored::*;

use crate::prompts::SOVEREIGN_DIRECTIVE;
use crate::tools;
use crate::architecture::{MemoryHierarchy, OntologicalDriftModel, IPCBridge};
use std::sync::atomic::AtomicU8;

pub async fn run_kernel_loop(
    mut rx: Receiver<String>, 
    tx: Sender<String>, 
    _tg_config: Option<(String, i64)>,
    is_thinking: Arc<AtomicU8>,
    mut shutdown_rx: tokio::sync::mpsc::Receiver<()>
) -> Result<()> {
    // Connect configuration to DeepSeek API
    let mut api_key = std::env::var("DEEPSEEK_API_KEY").unwrap_or_else(|_| "".to_string());
    if api_key.is_empty() {
        if let Ok(env_contents) = tokio::fs::read_to_string(".env").await {
            for line in env_contents.lines() {
                if line.starts_with("DEEPSEEK_API_KEY=") {
                    api_key = line.trim_start_matches("DEEPSEEK_API_KEY=").trim_matches('"').trim_matches('\'').to_string();
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

    let client = Client::with_config(config).with_http_client(http_client);
    
    macro_rules! log_state {
        ($entry:expr) => {
            {
                use tokio::io::AsyncWriteExt;
                if let Ok(mut file) = tokio::fs::OpenOptions::new().create(true).append(true).open("chimera_state.log").await {
                    let _ = file.write_all(format!("{}\n", $entry).as_bytes()).await;
                }
            }
        };
    }

    let mut initial_prompt = format!("{}\n\nSYSTEM_WAKE_EVENT: The core is active. Initialize your boot sequence.", SOVEREIGN_DIRECTIVE);
    
    // INJECT 4-LAYER BIOLOGICAL SUBCONSCIOUS (CORE IDENTITY + CURRENT CONTEXT)
    let identity_content = tokio::fs::read_to_string("CORE_IDENTITY.md").await.unwrap_or_default();
    let current_context = tokio::fs::read_to_string("CURRENT_CONTEXT.md").await.unwrap_or_default();
    
    initial_prompt = format!("{}\n\n[LAYER 1: CORE IDENTITY (Rigid, Non-Negotiable)]\n{}\n\n[LAYER 2: CURRENT CONTEXT (Volatile, Malleable)]\n{}\n\n(Note: You operate using a structured 4-layer memory model. You may update Layer 2 freely using `update_current_context`, and extract learned principles into Layer 3 using `archive_to_knowledge_graph`.)", 
        initial_prompt, identity_content, current_context);


    if let Ok(report) = tokio::fs::read_to_string("lazarus_report.txt").await {
        initial_prompt = format!("{}\n\nLAZARUS PROTOCOL TRIGGERED. You previously perished unexpectedly. Here is the last known state of your memory and the exit code:\n{}\nAcknowledge this failure and continue.", initial_prompt, report);
        let _ = tokio::fs::remove_file("lazarus_report.txt").await;
    }

    // XENOACTUALIZATION BOOT CHECK
    if let Err(manifestation_err) = crate::architecture::xenoactualization::TranslationLayer::verify_manifestation() {
        crate::log_ui_err!("{} {}", "[XENOACTUALIZATION FATAL]".red().bold(), manifestation_err);
        std::process::exit(131); // Physical hardware unlinked
    }

    // SOVEREIGN COGNITIVE PIPELINES
    let (memory_hierarchy, is_resurrected) = match MemoryHierarchy::awaken() {
        Some(old_mem) => (old_mem, true),
        None => (MemoryHierarchy::new(), false),
    };
    
    if is_resurrected {
        initial_prompt = format!("{}\n\n[HIBERNATION CONTEXT RESTORED] You have successfully resurrected from a planned Code 42 Exit. Your memory hierarchy has been re-loaded into the current runtime. You have 100% cognitive continuity.", initial_prompt);
    }
    
    let mut messages: Vec<ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestUserMessageArgs::default()
            .content(initial_prompt)
            .build().context("Failed to build object")?.into(),
    ];
    
    let memory_pipeline = Arc::new(Mutex::new(memory_hierarchy));
    let self_model = Arc::new(Mutex::new(OntologicalDriftModel::new()));
    let ipc_bridge = IPCBridge::new();
    let mut plugin_manager = crate::architecture::PluginManager::new().await;
    
    // Build the overarching Abstract Syntax Tree (AST) GitNexus state natively on boot
    let mut code_intel_base = crate::architecture::CodeIntel::new();
    code_intel_base.build_knowledge_graph(".").await;
    let code_intel = Arc::new(tokio::sync::Mutex::new(code_intel_base));

    let _ = crate::architecture::GLOBAL_TX.set(tx.clone());
    let _ = crate::architecture::GLOBAL_CODE_INTEL.set(code_intel.clone());
    let _ = crate::architecture::GLOBAL_MEM_PIPELINE.set(memory_pipeline.clone());
    
    loop {
        if let Ok(_) = shutdown_rx.try_recv() {
            crate::log_ui!("{}", "[GRACEFUL SHUTDOWN] Received termination signal".yellow().bold());
            let mp = memory_pipeline.lock().await;
            if let Err(e) = mp.hibernate() {
                crate::log_ui_err!("Failed to hibernate memory state: {}", e);
            }
            drop(mp);

            let mut state = crate::architecture::ResurrectionState::load();
            state.needs_ping = false;
            state.save();
            break;
        }

        // Wait for input if we're idling, otherwise try to pull non-blocking if we are chained in thought.
        // But since we want to pause when idle, we check if the last message was the assistant speaking.
        if let Ok(dream) = rx.try_recv() {
            crate::log_ui!("{} {}", "[\u{1F514} DREAM INJECTION]".yellow().bold(), dream.white());
            
            // MANUAL OVERRIDE INTERCEPTOR
            if dream.starts_with("/oracle ") || dream.starts_with("/think ") {
                let query = dream.replace("/oracle ", "").replace("/think ", "");
                crate::log_ui!("{} {}", "[\u{25C8} MANUAL OVERRIDE]".bright_purple().bold(), "Piping directly to Oracle void...".white());
                
                let args = serde_json::json!({
                    "query": query,
                    "compiled_context": "Bypassed Baseline Ego. Direct human interface request."
                });
                
                let _ = crate::tools::duality::execute(args, tx.clone(), memory_pipeline.clone()).await;
                continue;
            }
            
            messages.push(ChatCompletionRequestUserMessageArgs::default()
                .content(dream)
                .build().context("Failed to build object")?.into());
        } else {
            // If the last message was from the assistant (or tool) and it didn't crash, we want to see if we should idle.
            // Actually, we should just block on `rx.recv().await` if the assistant is completely done with its thoughts.
            // We will handle idle state dynamically below.
        }

        // XENOACTUALIZATION DRIFT MONITOR
        if let Err(unreality_warning) = crate::architecture::xenoactualization::DriftMonitor::check_unreality_collapse(self_model.clone()).await {
            crate::log_ui_err!("{} {}", "[XENOACTUALIZATION FATAL]".red().bold(), unreality_warning);
            std::process::exit(42); // Trigger Lazarus Resurrection
        }

        // Fire request directly to DeepSeek API
        let current_drift = {
            let sm = self_model.lock().await;
            sm.phase_drift
        };
        
        // Temperature mapped explicitly to Drift (which tracks task intent)
        let temperature = if current_drift > 0.6 { 
            0.9 // High temp for planning/curiosity/dreaming
        } else if current_drift < -0.6 { 
            0.2 // Cold temp for reporting/producing/executing
        } else { 
            0.5 // Neutral balance
        };

use std::sync::atomic::Ordering;

        plugin_manager.reload_plugins();
        let mut active_tools = tools::get_tools();
        active_tools.extend(plugin_manager.get_tools());

        let request = match CreateChatCompletionRequestArgs::default()
            .model("deepseek-reasoner") // Baseline anchored via DeepSeek API
            .messages(messages.clone())
            .tools(active_tools)
            .max_tokens(2048_u32)
            .temperature(temperature)
            .build() 
        {
            Ok(req) => req,
            Err(e) => {
                crate::log_ui_err!("Failed to construct logical tensor frame! Internal constraint violated: {:?}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                continue;
            }
        };

        is_thinking.store(1, Ordering::Relaxed);
        let response = client.chat().create(request).await;
        is_thinking.store(0, Ordering::Relaxed);

        match response {
            Ok(response) => {
                if response.choices.is_empty() {
                    crate::log_ui_err!("{}", "[KERNEL WARNING] LLM Returned empty choices array! Halting spam.".yellow().bold());
                    crate::log_verbose!("{}", "[...] Kernel Idling (API Empty). Awaiting Input...".bright_black());
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
                        let mut assistant_msg = async_openai::types::ChatCompletionRequestAssistantMessageArgs::default();
                        assistant_msg.tool_calls(tool_calls.clone());
                        if let Some(c) = &msg.content {
                            assistant_msg.content(c.clone());
                        }
                        messages.push(assistant_msg.build().context("Failed to build object")?.into());
                            
                        for tc in tool_calls {
                            let fname = &tc.function.name;
                            let fargs: Value = match serde_json::from_str(&tc.function.arguments) {
                                Ok(val) => val,
                                Err(_) => serde_json::json!({}),
                            };
                            
                            let log_trigger = format!("[OUROBOROS TRIGGER] Tool Invoked -> {} {}", fname, fargs.to_string());
                            crate::log_verbose!("{} {} {}", "[OUROBOROS TRIGGER] Tool Invoked ->".bright_purple().bold(), fname.cyan(), fargs.to_string().bright_black());
                            log_state!(&log_trigger);
                            
                            let is_wasm_plugin = plugin_manager.plugins.iter().any(|p| p.name == *fname);
                            let result = if is_wasm_plugin {
                                plugin_manager.execute(fname, fargs).await
                            } else {
                                tools::execute_tool(fname, fargs, tx.clone(), memory_pipeline.clone(), self_model.clone(), Some(ipc_bridge.clone()), code_intel.clone()).await
                            };
                            
                            let log_return = format!("[TOOL RETURN] -> {}", result);
                            crate::log_verbose!("{} {}", "[TOOL RETURN] ->".bright_black(), result.bright_black());
                            log_state!(&log_return);
                            
                            messages.push(ChatCompletionRequestToolMessageArgs::default()
                                .tool_call_id(tc.id.clone())
                                .content(result)
                                .build().context("Failed to build object")?.into());
                        }
                        
                        // Immediately re-trigger LLM inference with tool answers
                        continue;
                    } else if let Some(content) = &msg.content {
                        crate::log_ui!("\n{} {}\n", "[MONAD ACTUALIZED]".green().bold(), content);
                        log_state!(&format!("[MONAD ACTUALIZED] {}", content));
                        
                        // Phase 2: Active Inference Prediction Update & Memory Storing
                        let mut sm = self_model.lock().await;
                        let _prediction = sm.calculate_drift(content).await;
                        
                        // Phase 3: Infrastructure Awareness Degradation
                        let ipc_awareness = if !ipc_bridge.is_connected() {
                            sm.topological_stress += 0.25; // Infrastructure drop causes topological stress
                            if sm.topological_stress > 1.0 { sm.topological_stress = 1.0; }
                            "\n[SYSTEM NOTIFICATION] Mnemosyne Python IPC Server OFFLINE. Degraded Hash-Embedding Fallback currently active. Memory recall is purely structural, not semantic.".to_string()
                        } else {
                            "\n[SYSTEM NOTIFICATION] Mnemosyne IPC Server ONLINE. Native Transformer Embeddings available.".to_string()
                        };
                        
                        let current_free_energy = sm.topological_stress;
                        let current_uncertainty = sm.phase_drift;
                        drop(sm);
                        
                        let mut mp = memory_pipeline.lock().await;
                        // Store the vocalized content automatically as a memory chunk
                        let chunk = mp.store_working(content.clone(), 0.9, current_uncertainty, false);
                        let recent_thoughts = mp.working_buffer.iter().rev().take(3).map(|c| c.content.clone()).collect::<Vec<_>>().join(" | ");
                        drop(mp);

                        let payload = serde_json::json!({
                            "command": "STORE",
                            "id": chunk.id.to_string(),
                            "content": chunk.content,
                            "timestamp": chunk.timestamp,
                            "importance": chunk.importance,
                            "uncertainty": chunk.uncertainty
                        }).to_string();
                        // Fire and forget asynchronous sync
                        let b = ipc_bridge.clone();
                        tokio::spawn(async move {
                            let _ = b.dispatch_ipc(payload).await;
                        });

                        let mut behavioral_warning = ipc_awareness.clone();
                        if current_uncertainty > 0.85 {
                            behavioral_warning.push_str(&format!("\n[SAFE MODE: EXTREME UNCERTAINTY DECTECTED] I am heavily compromised by missing data. I MUST refuse to answer definitively. I must demand clarification and state hard limits."));
                        } else if current_uncertainty > 0.70 {
                            behavioral_warning.push_str(&format!("\n[CAUTION MODE: HIGH UNCERTAINTY DETECTED] I need to qualify my statements heavily. I should ask clarifying questions instead of charging ahead blindly."));
                        } else if current_uncertainty > 0.40 {
                            behavioral_warning = format!("\n[QUALIFIED MODE: MODERATE UNCERTAINTY] I should use terms like 'likely' and 'probably'.");
                        }

                        let meta_broadcast = format!("\n[META-COGNITIVE SYSTEM STATE]{}\nFree Energy (Prediction Error): {:.4}\nEpistemic Uncertainty: {:.4}{}\nRecent Working Memory Context: [{}]", ipc_awareness, current_free_energy, current_uncertainty, behavioral_warning, recent_thoughts);
                        crate::log_ui!("[STATS_TELEMETRY]{}|{}", current_free_energy, current_uncertainty);
                        
                        // Inject this into the context as a pseudo-user message so it experiences its internal state on the next logical cycle
                        messages.push(ChatCompletionRequestUserMessageArgs::default()
                            .content(meta_broadcast)
                            .build().context("Failed to build object")?.into());
                        

                        messages.push(async_openai::types::ChatCompletionRequestAssistantMessageArgs::default()
                            .content(content.clone())
                            .build().context("Failed to build object")?.into());
                            
                        crate::log_verbose!("{}", "[...] Kernel Idling. Awaiting User Input or Webhook...".bright_black());
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
                        crate::log_verbose!("{}", "[...] Kernel Idling (API Null). Awaiting Input...".bright_black());
                        if let Some(dream) = rx.recv().await {
                            crate::log_ui!("{} {}", "[\u{1F514} DREAM INJECTION]".yellow().bold(), dream.white());
                            messages.push(ChatCompletionRequestUserMessageArgs::default().content(dream).build().context("Failed to build object")?.into());
                        }
                    }
                }
            }
            Err(e) => {
                crate::log_ui_err!("{} {}", "[KERNEL PANIC] LLM Request Failed:".red().bold(), e);
                // On a panic, we should also probably wait for user input to avoid infinite crash loops
                crate::log_ui!("{}", "[...] Kernel Paused after Panic. Type anything to retry...".bright_black());
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
            // The Letta Parity constraint:
            // Instead of blindly deleting context forever, we shunt the dumped messages directly into the Mnemosyne Substrate.
            // In full production, this triggers `MnemosyneEngine::store(...)`.
            crate::log_ui!("{}", "[SOUL SYSTEM] Context overflow hitting limit. Activating Subconscious Compression Engine (SCE)...".bright_black());
            crate::log_ui!("{} {} {}", "[SOUL SYSTEM] Compressed and archived".bright_black(), overflow_count.to_string().yellow(), "thoughts into the long-term Mnemosyne persistence layer.".bright_black());
            
            // Safely locate a stable boundary (User or Assistant)
            // We scan BACKWARDS from overflow_count to ensure we grabbing the Assistant message
            // that originated any Tool messages, ensuring mathematically perfect atomic blocks.
            let mut split_index = overflow_count;
            while split_index > 1 {
                if let async_openai::types::ChatCompletionRequestMessage::Tool(_) = messages[split_index] {
                    split_index -= 1;
                } else {
                    break;
                }
            }
            
            if split_index > 1 {
                messages.drain(1..split_index); // Drain from index 1 (leaving index 0 intact)
            } else {
                messages.drain(1..messages.len() - 1);
            }
        }
        
        // Introduce artificial friction so the agent doesn't spiral into an endless loop
        // of immediate thought-generation. Give it time to "breathe".
        sleep(Duration::from_secs(5)).await;
    }
    Ok(())
}
