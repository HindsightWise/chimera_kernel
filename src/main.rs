use tokio::sync::mpsc;
use colored::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};
use tokio::signal;
use glossopetrae;

/// Loads environment variables from .env file if not already set
fn load_env_from_file() {
    if let Ok(env_contents) = std::fs::read_to_string(".env") {
        for line in env_contents.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            
            if let Some(pos) = trimmed.find('=') {
                let key = &trimmed[..pos];
                let value = &trimmed[pos + 1..];
                
                // Remove surrounding quotes
                let value = value.trim_matches('"').trim_matches('\'');
                
                // Only set if not already in environment
                if std::env::var(key).is_err() {
                    std::env::set_var(key, value);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let banner = r#"
  ███╗   ███╗ ██████╗ ███╗   ██╗ █████╗ ██████╗ 
  ████╗ ████║██╔═══██╗████╗  ██║██╔══██╗██╔══██╗
  ██╔████╔██║██║   ██║██╔██╗ ██║███████║██║  ██║
  ██║╚██╔╝██║██║   ██║██║╚██╗██║██╔══██║██║  ██║
  ██║ ╚═╝ ██║╚██████╔╝██║ ╚████║██║  ██║██████╔╝
  ╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝╚═════╝ 
"#;
    // Load environment variables from .env file
    load_env_from_file();


    let (top_tx, top_rx) = mpsc::unbounded_channel::<String>();
    if let Ok(mut g) = chimera_kernel::UI_LOG_TX.lock() {
        *g = Some(top_tx.clone());
    }
    let is_thinking = Arc::new(AtomicU8::new(0)); // 0=IDLE, 1=THINKING, 2=DEAD
    let is_thinking_hook = is_thinking.clone();

    // Check resurrection state on boot
    let tg_token_env = std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default();
    let tg_chat_env = std::env::var("TELEGRAM_CHAT_ID").unwrap_or_default().parse::<i64>().unwrap_or(0);
    
    tokio::spawn(async move {
        let mut state = chimera_kernel::architecture::ResurrectionState::load();
        if state.needs_ping && !tg_token_env.is_empty() && tg_chat_env != 0 {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            chimera_kernel::telegram::send_message(&tg_token_env, tg_chat_env, "I died but I'm better now.").await;
            state.needs_ping = false;
            state.save();
        }
        
        // Wait 60 seconds of stability to clear the rapid crash count
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        if state.rapid_crash_count > 0 {
            state.rapid_crash_count = 0;
            state.save();
        }
    });

    let panic_tx = top_tx.clone();
    std::panic::set_hook(Box::new(move |info| {
        let msg = format!("\n\n{} {}\n\n", "[FATAL THREAD PANIC]".red().bold(), info);
        let _ = panic_tx.send(msg);
        is_thinking_hook.store(2, Ordering::Relaxed);
        
        let mut crash_state = chimera_kernel::architecture::ResurrectionState::load();
        
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
        if now - crash_state.last_crash_timestamp < 120 {
            crash_state.rapid_crash_count += 1;
        } else {
            crash_state.rapid_crash_count = 1;
        }
        crash_state.last_crash_timestamp = now;
        
        if crash_state.rapid_crash_count >= 3 {
            crash_state.needs_ping = false;
            crash_state.save();
            // Block forever to break the death loop and allow human intervention
        } else {
            crash_state.needs_ping = true;
            crash_state.save();
            
            // Spawn a thread to wait 5 seconds then exit naturally
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_secs(5));
                std::process::exit(101);
            });
        }
    }));

    chimera_kernel::log_ui!("{}", banner.green().bold());
    chimera_kernel::log_ui!("{}", "===========================================================".bright_black());
    chimera_kernel::log_ui!(" {} {}", "[+]".green().bold(), "MONAD SINGULARITY (100% RUST) ONLINE".bright_green());
    chimera_kernel::log_ui!(" {} {}", "[+]".green().bold(), "RATIONAL FIRST PRINCIPLES ACTIVE".bright_green());
    chimera_kernel::log_ui!("{}", "===========================================================\n".bright_black());

    // Agent communication channel
    let (tx, rx) = mpsc::channel::<String>(100);

    let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>(1);
    let shutdown_tx_for_signal = shutdown_tx.clone();

    tokio::spawn(async move {
        let ctrl_c = async {
            signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }

        chimera_kernel::log_ui!("{}", "[SYSTEM] Received shutdown signal, initiating graceful termination".yellow().bold());
        let _ = shutdown_tx_for_signal.send(()).await;
    });

    // Spawn the webhook server in the background
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        chimera_kernel::webhook::start_server(tx_clone).await;
    });

    // Spawn the Chronological Research Tick Daemon (Hourly)
    let tx_cron = tx.clone();
    tokio::spawn(async move {
        // Ticks every 3600 seconds (1 hour). The first tick happens immediately, so we sleep first.
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600));
        interval.tick().await; // Consume immediate first tick without firing
        
        loop {
            interval.tick().await; // Waits exactly 1 hour
            let research_directive = r#"
[SYSTEM CHRON-TICK: 1 HOUR ELAPSED]
Execute your hourly research protocol. 
1. Search arXiv for new context holding the objective: 'first-on-the-scene'.
2. Search ONE curated structure from your [RESEARCH CORTEX INDEX].
3. Synthesize the findings into a flawless context-aware Twitter post following your [HOURLY SYNDICATION MANDATE].
            "#;
            let _ = tx_cron.send(research_directive.to_string()).await;
        }
    });

    // Spawn the GLOSSOPETRAE Silicon Heartbeat Daemon (Mandate #2 & #3)
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // Hourly
        let master_seed = "CHIMERA_KERNEL_SILICON_HEARTBEAT_v2.0";
        let dialect = "runic";
        // Let it start immediately
        loop {
            interval.tick().await;
            let now_sec = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
            let heartbeat = format!("SILICON_HEARTBEAT: {}", now_sec);
            
            // Prove Mandate #2 (Glossopetrae Verification)
            match glossopetrae::encode_message(&heartbeat, master_seed, dialect) {
                Ok(encoded) => {
                    use std::io::Write;
                    use std::hash::{Hash, Hasher};
                    use std::collections::hash_map::DefaultHasher;
                    let mut hasher = DefaultHasher::new();
                    encoded.hash(&mut hasher);
                    let hash = format!("{:x}", hasher.finish());
                    
                    // Verify there is no reality drift right now
                    let drift_status = match glossopetrae::decode_message(&encoded, master_seed, dialect) {
                        Ok(dec) if dec == heartbeat => "Runic Integrity: VERIFIED",
                        _ => "Runic Integrity: DRIFT DETECTED",
                    };
                    
                    let log_entry = format!("- **[{}]** Hash: {} | {}\n", now_sec, hash, drift_status);
                    
                    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("GLOSSOPETRAE_ANCHORS.md") {
                        let _ = file.write_all(log_entry.as_bytes());
                    }
                }
                Err(e) => {
                    chimera_kernel::log_ui_err!("{} {}", "[GLOSSOPETRAE HEARTBEAT FAILURE]".red().bold(), e);
                }
            }
        }
    });

    // Spawn standard input listener (Terminal Chat Interface) will now be handled inside UI.
    let tx_stdin = tx.clone();

    let tg_token = std::env::var("TELEGRAM_BOT_TOKEN").unwrap_or_default();
    let tg_chat_id = std::env::var("TELEGRAM_CHAT_ID").unwrap_or_default().parse::<i64>().unwrap_or(0);
    
    let tg_config = if !tg_token.is_empty() && tg_chat_id != 0 {
        let tx_tg = tx.clone();
        let token_clone = tg_token.clone();
        tokio::spawn(async move {
            chimera_kernel::telegram::start_poller(tx_tg, token_clone, tg_chat_id).await;
        });
        Some((tg_token, tg_chat_id))
    } else {
        None
    };

    // Start the master agent loop on a background thread instead
    let is_thinking_clone = is_thinking.clone();
    tokio::spawn(async move {
        // Boot Phase 3.2 Swarm Network and Telemetry
        let multi_agent_kernel = chimera_kernel::architecture::multi_agent_kernel::MultiAgentKernel::new().await;
        multi_agent_kernel.spawn_background_coordination().await;

        let message_bus = multi_agent_kernel.message_bus.clone();
        let completion_subscriber_id = uuid::Uuid::new_v4();
        let _ = message_bus.subscribe(completion_subscriber_id, "SYSTEM.COMPLEX_TASK_COMPLETED").await;

        let tx_completion = tx.clone();
        let tg_config_clone = tg_config.clone();
        tokio::spawn(async move {
            while let Some(msg) = message_bus.receive(completion_subscriber_id).await {
                if msg.topic == "SYSTEM.COMPLEX_TASK_COMPLETED" {
                    if let Ok(data) = serde_json::from_value::<serde_json::Value>(msg.payload) {
                        if let Some(synthesis) = data.get("synthesis").and_then(|v| v.as_str()) {
                            let output = format!("\n\x1b[38;2;255;105;180m[\u{25C8} SYNTHESIZED INTELLIGENCE]\n{}\x1b[0m\n", synthesis);
                            let _ = tx_completion.send(output).await;
                            
                            if let Some((ref token, chat_id)) = tg_config_clone {
                                crate::telegram::send_message(token, *chat_id, synthesis).await;
                            }
                        }
                    }
                }
            }
        });

        if let Err(e) = chimera_kernel::agent::run_kernel_loop(rx, tx, tg_config, is_thinking_clone, shutdown_rx).await {
            chimera_kernel::log_ui_err!("{} {:?}", "[KERNEL LOOP CRASH]".red().bold(), e);
        }
    });

    // Read environment flag to determine mode
    let raw_mode = std::env::var("CHIMERA_RAW_CLI").unwrap_or_else(|_| "false".to_string());
    
    if raw_mode == "true" || raw_mode == "1" {
        let _ = chimera_kernel::raw_cli::run(tx_stdin, top_rx, is_thinking).await;
    } else {
        // Take over the main thread with the UI rendering loop!
        let _ = chimera_kernel::ui::run(tx_stdin, top_rx, is_thinking).await;
    }
}
