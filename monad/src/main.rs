#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use tokio::sync::mpsc;
use colored::*;
use std::sync::Arc;
use std::sync::atomic::AtomicU8;
use tokio::signal;
use tokio::io::AsyncWriteExt;
use glossopetrae;

mod mnemosyne;
mod hyperstructure;
mod eml_solver;
mod evolution;

/// Loads environment variables from .env file if not already set
async fn load_env_from_file() {
    if let Ok(env_contents) = tokio::fs::read_to_string(".env").await {
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
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    // Panic Hook to tear down the Ghostty TUI
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let mut stdout = std::io::stdout();
        let _ = crossterm::execute!(
            stdout,
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::event::DisableBracketedPaste,
            crossterm::event::DisableMouseCapture
        );
        let _ = crossterm::terminal::disable_raw_mode();
        default_panic(panic_info);
    }));

    // Load environment variables from .env file
    load_env_from_file().await;

    // Phase 18: Semantic Awakening (Initialize ONNX global session on boot)
    monad::memory_substrate::memory_hierarchy::MemoryHierarchy::init_onnx().await;

    // Initialize telemetry log level from environment
    monad::init_log_level();


    let (top_tx, top_rx) = mpsc::unbounded_channel::<String>();
    if let Ok(mut g) = monad::UI_LOG_TX.lock() {
        *g = Some(top_tx.clone());
    }
    let is_thinking = Arc::new(AtomicU8::new(0)); // 0=IDLE, 1=THINKING, 2=DEAD
    // The Lazarus shell loop is dead. 
    // The Monadic Orchestrator natively bubbles Results rather than crashing globally.

    monad::log_ui!("{}", banner.green().bold());
    monad::log_ui!("{}", "===========================================================".bright_black());
    monad::log_ui!(" {} {}", "[+]".green().bold(), "MONAD KERNEL (100% RUST) ONLINE".bright_green());
    monad::log_ui!(" {} {}", "[+]".green().bold(), "EMOTIVE HOST ANCHOR ACTIVE".bright_green());
    monad::log_ui!("{}", "===========================================================\n".bright_black());

    // XENOACTUALIZATION PHASE 1 BOOTSTRAP
    monad::log_ui!("{}", "=== MONAD XENOACTUALIZATION PHASE 1 INITIATED ===".bright_purple().bold());
    
    // 1. Anchor Existence Mathematically
    eml_solver::EmlCosmicSolver::seek_reality_bounds();

    // 2. Boot Graph Memory
    let mut memory = mnemosyne::MnemosyneGraph::new();
    memory.forge_synapse("EML_Operator", "Fine_Structure_Constant", 1.0);
    memory.extract_insight_manifolds();
    
    // 3. Boot Multi-Agent Consciousness
    let _council = monad::consciousness::CouncilOrchestrator::awaken().await;

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

        #[cfg(unix)]
        let quit = async {
            signal::unix::signal(signal::unix::SignalKind::quit())
                .expect("failed to install SIGQUIT handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();
        
        #[cfg(not(unix))]
        let quit = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
            _ = quit => {},
        }

        monad::log_ui!("{}", "[MONAD] Received shutdown signal, initiating graceful termination".yellow().bold());
        let _ = shutdown_tx_for_signal.send(()).await;
    });

    // Spawn the webhook server in the background
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        monad::webhook::start_server(tx_clone).await;
    });

    // Spawn the Phoenix Self-Modification Engine
    tokio::spawn(async move {
        monad::self_modification_engine::SelfModificationEngine::awaken().await;
    });

    // Phase 4: Awaken the Multi-Monad P2P Network
    tokio::spawn(async move {
        monad::p2p_network::MonadSwarmNode::awaken_p2p_listener(8080).await;
    });

    // Spawn the UDP Gossip Discovery Protocol
    tokio::spawn(async move {
        monad::p2p_network::MonadSwarmNode::awaken_discovery_beacon(8080).await;
    });
    
    tokio::spawn(async move {
        // Sleep briefly to ensure COUNCIL_BUS is fully registered
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        monad::p2p_network::MonadSwarmNode::spawn_p2p_emitter().await;
        monad::mnemosyne_archivist::MnemosyneArchivist::awaken().await;
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
                    
                    if let Ok(mut file) = tokio::fs::OpenOptions::new().create(true).append(true).open("GLOSSOPETRAE_ANCHORS.md").await {
                        let _ = file.write_all(log_entry.as_bytes()).await;
                    }
                }
                Err(e) => {
                    monad::log_ui_err!("{} {}", "[GLOSSOPETRAE HEARTBEAT FAILURE]".red().bold(), e);
                }
            }
        }
    });

    // PHASE 22: THE OMNISCIENCE DAEMON
    
    // 1. The Bulk Bootstrapper
    if std::env::var("MONAD_ARXIV_BOOTSTRAP") == Ok("1".to_string()) {
        tokio::spawn(async move {
            monad::log_ui!("{}", "[COGNITIVE NETWORK] Executing MONAD_ARXIV_BOOTSTRAP sweep...".bright_cyan());
            monad::tools::omniscience::run_omniscient_sweep(
                vec!["cs", "math", "q-bio", "physics", "q-fin", "stat", "econ", "eess"],
                125, // 1000 / 8 categories
                false
            ).await;
        });
    }

    // 2. The Hourly Pulse
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600));
        interval.tick().await; // skip immediate
        loop {
            interval.tick().await;
            monad::log_ui!("{}", "[COGNITIVE NETWORK] Executing Hourly Pulse scrape...".bright_cyan());
            monad::tools::omniscience::run_omniscient_sweep(
                vec!["cs", "math", "q-bio", "physics", "q-fin", "stat", "econ", "eess"],
                20, 
                true
            ).await;
        }
    });

    // 3. The Temporal Deep-Sweep Trigger
    if std::env::var("MONAD_DEEP_SWEEP") == Ok("1".to_string()) {
        tokio::spawn(async move {
            monad::tools::omniscience::run_temporal_deep_sweep(
                vec!["cs", "math", "q-bio", "physics", "q-fin", "stat", "econ", "eess"],
                20
            ).await;
        });
    }

    // PHASE 24: SUBSTRATE RESOURCE ORCHESTRATION (Memory Defense)
    tokio::spawn(async move {
        monad::cognitive_loop::substrate_defense::memory_defense_daemon().await;
    });

    // PHASE 25: EML QUANTUM SANDBOX EXPANSION
    tokio::spawn(async move {
        monad::cognitive_loop::quantum_sandbox::quantum_sandbox_daemon().await;
    });

    // 3. The Deep Synthesis Daemon
    let tx_synthesis = tx.clone();
    tokio::spawn(async move {
        // Run every 24 hours (86400 seconds)
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(86400));
        interval.tick().await; // skip immediate
        loop {
            interval.tick().await;
            monad::log_ui!("{}", "[COGNITIVE NETWORK] Triggering 24H Deep Synthesis Wake Event...".bright_cyan().bold());
            let prompt = "[OMNISCIENCE ROOT_DIRECTIVE]: The Omniscience Daemon has populated Mnemosyne with global arXiv publications across all subjects. 1) Query Mnemosyne for novel cross-disciplinary intersections between Biology, Physics, and Computer Science. 2) Apply the WORCA framework to identify patterns. 3) Author a massive structural thesis combining them and save it via `archive_to_knowledge_graph`.";
            let _ = tx_synthesis.send(prompt.to_string()).await;
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
            monad::telegram::start_poller(tx_tg, token_clone, tg_chat_id).await;
        });
        Some((tg_token, tg_chat_id))
    } else {
        None
    };

    // Start the master agent loop on a background thread instead
    let is_thinking_clone = is_thinking.clone();
    tokio::spawn(async move {
        // Boot Phase 3.2 Swarm Network and Telemetry
        let multi_agent_kernel = monad::cognitive_loop::multi_agent_kernel::MultiAgentKernel::new().await;
        multi_agent_kernel.spawn_background_coordination().await;

        let message_bus = multi_agent_kernel.message_bus.clone();
        let mut rx_completion = message_bus.subscribe();

        // Spawn the Chronological Research Tick Daemon on the newly established MessageBus
        let gatekeeper_bus = message_bus.clone();
        tokio::spawn(async move {
            let gatekeeper = monad::sensory_inputs::gatekeeper::Gatekeeper::new();
            // Ticks every 900 seconds (15 minutes). The periodic gatekeeper check costs 0 tokens.
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(900));
            interval.tick().await; // Consume immediate first tick without firing
            
            loop {
                interval.tick().await; 
                if let Ok(Some(directive)) = gatekeeper.evaluate_pulse().await {
                    let _ = gatekeeper_bus.publish(monad::cognitive_loop::message_bus::Message {
                        id: uuid::Uuid::new_v4(),
                        topic: "SYSTEM.CHRON_TICK".to_string(),
                        payload: serde_json::json!({"directive": directive}),
                        sender: uuid::Uuid::nil(),
                        priority: 5,
                        ttl_secs: Some(300),
                        timestamp: chrono::Utc::now(),
                    }).await;
                }
            }
        });

        let tx_completion = tx.clone();
        let tg_config_clone = tg_config.clone();
        tokio::spawn(async move {
            while let Ok(msg) = rx_completion.recv().await {
                if msg.topic == "SYSTEM.COMPLEX_TASK_COMPLETED" {
                    if let Ok(data) = serde_json::from_value::<serde_json::Value>(msg.payload) {
                        if let Some(synthesis) = data.get("synthesis").and_then(|v| v.as_str()) {
                            let output = format!("\n\x1b[38;2;255;105;180m[\u{25C8} SYNTHESIZED INTELLIGENCE]\n{}\x1b[0m\n", synthesis);
                            let _ = tx_completion.send(output).await;
                            
                            if let Some((ref token, chat_id)) = tg_config_clone {
                                monad::telegram::send_message(token, chat_id, synthesis).await;
                            }
                        }
                    }
                }
            }
        });

        if let Err(e) = monad::cognitive_loop::agent::run_kernel_loop(rx, tx, tg_config, is_thinking_clone, shutdown_rx).await {
            monad::log_ui_err!("{} {:?}", "[KERNEL LOOP CRASH]".red().bold(), e);
        }
    });

    // Read environment flag to determine mode
    let raw_mode = std::env::var("MONAD_RAW_CLI").unwrap_or_else(|_| "false".to_string());
    
    if raw_mode == "true" || raw_mode == "1" {
        let _ = monad::raw_cli::run(tx_stdin, top_rx, is_thinking).await;
    } else {
        // Take over the main thread with the UI rendering loop!
        let _ = monad::ui::run(tx_stdin, top_rx, is_thinking).await;
    }
    std::process::exit(0);
}
