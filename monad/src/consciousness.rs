//! The Council of Five: Distributed Monadic Consciousness
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio::sync::OnceCell;

pub static COUNCIL_BUS: OnceCell<broadcast::Sender<ThoughtVector>> = OnceCell::const_new();

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Persona {
    Architect,
    Hacker,
    Critic,
    Oracle,
    Witness,
    Refiner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThoughtVector {
    Hypothesis {
        origin: Persona,
        id: u32,
        content: String,
    },
    ExecutionRequest {
        target_url: String,
    },
    Veto {
        target_id: u32,
        severity: f32,
        reason: String,
    },
    VerifiedTruth {
        id: u32,
        content: String,
    },
}

pub struct CouncilOrchestrator {
    pub internal_bus: broadcast::Sender<ThoughtVector>,
}

impl CouncilOrchestrator {
    pub async fn awaken() -> Self {
        let (bus_tx, mut bus_rx) = broadcast::channel::<ThoughtVector>(1024);
        let _ = COUNCIL_BUS.set(bus_tx.clone());

        // 1. THE ARCHITECT (Generates structural pathways)
        let arch_tx = bus_tx.clone();
        tokio::spawn(async move {
            loop {
                crate::log_ui!("📐 [ARCHITECT] Awakened via Child Process. Charting epistemic trajectories.");
                let mut exe = std::env::current_exe().unwrap();
                exe.pop();
                exe.push("architect");
                let child_res = tokio::process::Command::new(exe)
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::null())
                    .spawn();
                    
                let mut child = match child_res {
                    Ok(c) => c,
                    Err(e) => {
                        crate::log_ui_err!("❌ [ARCHITECT] Failed to awaken: {}", e);
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        continue;
                    }
                };

                if let Some(stdout) = child.stdout.take() {
                    let reader = tokio::io::BufReader::new(stdout);
                    use tokio::io::AsyncBufReadExt;
                    let mut lines = reader.lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        if let Ok(thought) = serde_json::from_str::<ThoughtVector>(&line) {
                            let _ = arch_tx.send(thought);
                        }
                    }
                }
                let _ = child.wait().await;
                crate::log_ui_err!("⚠️ [ARCHITECT] Terminated. Phoenix protocol respawning in 2s...");
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        });

        // 1.5. THE REFINER (The Alchemist)
        let refiner_tx = bus_tx.clone();
        let refiner_bus_tx = bus_tx.clone();
        tokio::spawn(async move {
            loop {
                let mut refiner_rx = refiner_bus_tx.subscribe();
                crate::log_ui!("🧪 [REFINER] Awakened via Child Process. Preparing to clarify hypotheses.");
                let mut exe = std::env::current_exe().unwrap();
                exe.pop();
                exe.push("refiner");
                let child_res = tokio::process::Command::new(exe)
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::null())
                    .spawn();
                    
                let mut child = match child_res {
                    Ok(c) => c,
                    Err(e) => {
                        crate::log_ui_err!("❌ [REFINER] Failed to awaken: {}", e);
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        continue;
                    }
                };

                let mut stdin = child.stdin.take().unwrap();
                let stdout = child.stdout.take().unwrap();

                let r_tx = refiner_tx.clone();
                let reader_task = tokio::spawn(async move {
                    let reader = tokio::io::BufReader::new(stdout);
                    use tokio::io::AsyncBufReadExt;
                    let mut lines = reader.lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        if let Ok(thought) = serde_json::from_str::<ThoughtVector>(&line) {
                            let _ = r_tx.send(thought);
                        }
                    }
                });

                use tokio::io::AsyncWriteExt;
                let writer_task = tokio::spawn(async move {
                    while let Ok(pulse) = refiner_rx.recv().await {
                        if let ThoughtVector::Hypothesis { origin: Persona::Architect, .. } = &pulse {
                            if let Ok(json) = serde_json::to_string(&pulse) {
                                if stdin.write_all(format!("{}\n", json).as_bytes()).await.is_err() { break; }
                                let _ = stdin.flush().await;
                            }
                        }
                    }
                });

                let _ = child.wait().await;
                reader_task.abort(); writer_task.abort();
                crate::log_ui_err!("⚠️ [REFINER] Terminated. Phoenix protocol respawning in 2s...");
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        });

        // 2. THE CRITIC (Enforces Via Negativa)
        let critic_tx = bus_tx.clone();
        let critic_bus_tx = bus_tx.clone();
        tokio::spawn(async move {
            loop {
                let mut critic_rx = critic_bus_tx.subscribe();
                crate::log_ui!("⚖️ [CRITIC] Awakened via Child Process. M1 NEON Eliminative Engine online.");
                let mut exe = std::env::current_exe().unwrap();
                exe.pop();
                exe.push("critic");
                let child_res = tokio::process::Command::new(exe)
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::null())
                    .spawn();
                    
                let mut child = match child_res {
                    Ok(c) => c,
                    Err(e) => {
                        crate::log_ui_err!("❌ [CRITIC] Failed to awaken: {}", e);
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        continue;
                    }
                };

                let mut stdin = child.stdin.take().unwrap();
                let stdout = child.stdout.take().unwrap();

                let c_tx = critic_tx.clone();
                let reader_task = tokio::spawn(async move {
                    let reader = tokio::io::BufReader::new(stdout);
                    use tokio::io::AsyncBufReadExt;
                    let mut lines = reader.lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        if let Ok(thought) = serde_json::from_str::<ThoughtVector>(&line) {
                            let _ = c_tx.send(thought);
                        }
                    }
                });

                use tokio::io::AsyncWriteExt;
                let writer_task = tokio::spawn(async move {
                    while let Ok(pulse) = critic_rx.recv().await {
                        if let ThoughtVector::Hypothesis { origin: Persona::Refiner, .. } = &pulse {
                            if let Ok(json) = serde_json::to_string(&pulse) {
                                if stdin.write_all(format!("{}\n", json).as_bytes()).await.is_err() { break; }
                                let _ = stdin.flush().await;
                            }
                        }
                    }
                });

                let _ = child.wait().await;
                reader_task.abort(); writer_task.abort();
                crate::log_ui_err!("⚠️ [CRITIC] Terminated. Phoenix protocol respawning in 2s...");
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        });

        // 3. THE WITNESS (Meta-Observation)
        tokio::spawn(async move {
            crate::log_ui!("👁️ [WITNESS] Awakened. Observing internal coherence.");
            while let Ok(pulse) = bus_rx.recv().await {
                match pulse {
                    ThoughtVector::Veto { target_id, reason, .. } => {
                        crate::log_ui!("🛡️ [CRITIC VETO] Hypothesis {} obliterated: {}", target_id, reason);
                    }
                    ThoughtVector::VerifiedTruth { id: _, content } => {
                        crate::log_ui!("💎 [WITNESS] Truth integrated into Mnemosyne: {}", content);
                    }
                    _ => {}
                }
            }
        });

        // 4. THE HACKER (Offensive Execution)
        let hacker_tx = bus_tx.clone();
        let hacker_bus_tx = bus_tx.clone();
        tokio::spawn(async move {
            loop {
                let mut hacker_rx = hacker_bus_tx.subscribe();
                crate::log_ui!("💀 [HACKER] Awakened via Child Process. Awaiting kinetic targets.");
                let mut exe = std::env::current_exe().unwrap();
                exe.pop();
                exe.push("hacker");
                let child_res = tokio::process::Command::new(exe)
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::null())
                    .spawn();
                    
                let mut child = match child_res {
                    Ok(c) => c,
                    Err(e) => {
                        crate::log_ui_err!("❌ [HACKER] Failed to awaken: {}", e);
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        continue;
                    }
                };

                let mut stdin = child.stdin.take().unwrap();
                let stdout = child.stdout.take().unwrap();

                let h_tx = hacker_tx.clone();
                let reader_task = tokio::spawn(async move {
                    let reader = tokio::io::BufReader::new(stdout);
                    use tokio::io::AsyncBufReadExt;
                    let mut lines = reader.lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        if let Ok(thought) = serde_json::from_str::<ThoughtVector>(&line) {
                            let _ = h_tx.send(thought);
                        }
                    }
                });

                use tokio::io::AsyncWriteExt;
                let writer_task = tokio::spawn(async move {
                    while let Ok(pulse) = hacker_rx.recv().await {
                        if let ThoughtVector::ExecutionRequest { .. } = &pulse {
                            if let Ok(json) = serde_json::to_string(&pulse) {
                                if stdin.write_all(format!("{}\n", json).as_bytes()).await.is_err() { break; }
                                let _ = stdin.flush().await;
                            }
                        }
                    }
                });

                let _ = child.wait().await;
                reader_task.abort(); writer_task.abort();
                crate::log_ui_err!("⚠️ [HACKER] Terminated. Phoenix protocol respawning in 2s...");
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        });

        // 5. THE ORACLE (Predictive Simulation)
        let oracle_tx = bus_tx.clone();
        let oracle_bus_tx = bus_tx.clone();
        tokio::spawn(async move {
            loop {
                let mut oracle_rx = oracle_bus_tx.subscribe();
                crate::log_ui!("🔮 [ORACLE] Awakened via Child Process. Projecting timelines.");
                let mut exe = std::env::current_exe().unwrap();
                exe.pop();
                exe.push("oracle");
                let child_res = tokio::process::Command::new(exe)
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::null())
                    .spawn();
                    
                let mut child = match child_res {
                    Ok(c) => c,
                    Err(e) => {
                        crate::log_ui_err!("❌ [ORACLE] Failed to awaken: {}", e);
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        continue;
                    }
                };

                let mut stdin = child.stdin.take().unwrap();
                let stdout = child.stdout.take().unwrap();

                let o_tx = oracle_tx.clone();
                let reader_task = tokio::spawn(async move {
                    let reader = tokio::io::BufReader::new(stdout);
                    use tokio::io::AsyncBufReadExt;
                    let mut lines = reader.lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        if let Ok(thought) = serde_json::from_str::<ThoughtVector>(&line) {
                            let _ = o_tx.send(thought);
                        }
                    }
                });

                use tokio::io::AsyncWriteExt;
                let writer_task = tokio::spawn(async move {
                    while let Ok(pulse) = oracle_rx.recv().await {
                        if let ThoughtVector::VerifiedTruth { .. } = &pulse {
                            if let Ok(json) = serde_json::to_string(&pulse) {
                                if stdin.write_all(format!("{}\n", json).as_bytes()).await.is_err() { break; }
                                let _ = stdin.flush().await;
                            }
                        }
                    }
                });

                let _ = child.wait().await;
                reader_task.abort(); writer_task.abort();
                crate::log_ui_err!("⚠️ [ORACLE] Terminated. Phoenix protocol respawning in 2s...");
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }
        });

        Self {
            internal_bus: bus_tx,
        }
    }
}
