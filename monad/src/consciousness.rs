//! The Council of Five: Distributed Monadic Consciousness
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio::sync::OnceCell;

pub static COUNCIL_BUS: OnceCell<broadcast::Sender<ThoughtVector>> = OnceCell::const_new();

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            crate::log_ui!(
                "📐 [ARCHITECT] Awakened via Child Process. Charting epistemic trajectories."
            );
            let mut child = tokio::process::Command::new("cargo")
                .args(["run", "--release", "--bin", "architect"])
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("Failed to spawn Architect process");

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
        });

        // 1.5. THE REFINER (The Alchemist)
        let refiner_tx = bus_tx.clone();
        let mut refiner_rx = bus_tx.subscribe();
        tokio::spawn(async move {
            crate::log_ui!(
                "🧪 [REFINER] Awakened via Child Process. Preparing to clarify hypotheses."
            );
            let mut child = tokio::process::Command::new("cargo")
                .args(["run", "--release", "--bin", "refiner"])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("Failed to spawn Refiner process");

            let mut stdin = child.stdin.take().expect("Failed to open refiner stdin");
            let stdout = child.stdout.take().expect("Failed to open refiner stdout");

            // Read from Refiner and broadcast
            let r_tx = refiner_tx.clone();
            tokio::spawn(async move {
                let reader = tokio::io::BufReader::new(stdout);
                use tokio::io::AsyncBufReadExt;
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    if let Ok(thought) = serde_json::from_str::<ThoughtVector>(&line) {
                        let _ = r_tx.send(thought);
                    }
                }
            });

            // Feed Architect hypotheses to Refiner
            use tokio::io::AsyncWriteExt;
            while let Ok(pulse) = refiner_rx.recv().await {
                if let ThoughtVector::Hypothesis { origin: Persona::Architect, .. } = &pulse {
                    if let Ok(json) = serde_json::to_string(&pulse) {
                        let _ = stdin.write_all(format!("{}\n", json).as_bytes()).await;
                        let _ = stdin.flush().await;
                    }
                }
            }
        });

        // 2. THE CRITIC (Enforces Via Negativa & Formal Verification)
        let critic_tx = bus_tx.clone();
        let mut critic_rx = bus_tx.subscribe();
        tokio::spawn(async move {
            crate::log_ui!(
                "⚖️ [CRITIC] Awakened via Child Process. M1 NEON Eliminative Engine online."
            );
            let mut child = tokio::process::Command::new("cargo")
                .args(["run", "--release", "--bin", "critic"])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("Failed to spawn Critic process");

            let mut stdin = child.stdin.take().expect("Failed to open critic stdin");
            let stdout = child.stdout.take().expect("Failed to open critic stdout");

            // Read output from Critic and route to internal bus
            let c_tx = critic_tx.clone();
            tokio::spawn(async move {
                let reader = tokio::io::BufReader::new(stdout);
                use tokio::io::AsyncBufReadExt;
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    if let Ok(thought) = serde_json::from_str::<ThoughtVector>(&line) {
                        let _ = c_tx.send(thought);
                    }
                }
            });

            // Write Hypothesis to Critic
            use tokio::io::AsyncWriteExt;
            while let Ok(pulse) = critic_rx.recv().await {
                if let ThoughtVector::Hypothesis { origin: Persona::Refiner, .. } = &pulse {
                    if let Ok(json) = serde_json::to_string(&pulse) {
                        let _ = stdin.write_all(format!("{}\n", json).as_bytes()).await;
                        let _ = stdin.flush().await;
                    }
                }
            }
        });

        // 3. THE WITNESS (Meta-Observation & Coherence)
        tokio::spawn(async move {
            crate::log_ui!("👁️ [WITNESS] Awakened. Observing internal coherence.");
            while let Ok(pulse) = bus_rx.recv().await {
                match pulse {
                    ThoughtVector::Veto {
                        target_id, reason, ..
                    } => {
                        crate::log_ui!(
                            "🛡️ [CRITIC VETO] Hypothesis {} obliterated: {}",
                            target_id,
                            reason
                        );
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
        let mut hacker_rx = bus_tx.subscribe();
        tokio::spawn(async move {
            crate::log_ui!("💀 [HACKER] Awakened via Child Process. Awaiting kinetic targets.");
            let mut child = tokio::process::Command::new("cargo")
                .args(["run", "--release", "--bin", "hacker"])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("Failed to spawn Hacker process");

            let mut stdin = child.stdin.take().expect("Failed to open hacker stdin");
            let stdout = child.stdout.take().expect("Failed to open hacker stdout");

            let h_tx = hacker_tx.clone();
            tokio::spawn(async move {
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
            while let Ok(pulse) = hacker_rx.recv().await {
                if let ThoughtVector::ExecutionRequest { .. } = &pulse {
                    if let Ok(json) = serde_json::to_string(&pulse) {
                        let _ = stdin.write_all(format!("{}\n", json).as_bytes()).await;
                        let _ = stdin.flush().await;
                    }
                }
            }
        });

        // 5. THE ORACLE (Predictive Simulation)
        let oracle_tx = bus_tx.clone();
        let mut oracle_rx = bus_tx.subscribe();
        tokio::spawn(async move {
            crate::log_ui!("🔮 [ORACLE] Awakened via Child Process. Projecting timelines.");
            let mut child = tokio::process::Command::new("cargo")
                .args(["run", "--release", "--bin", "oracle"])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("Failed to spawn Oracle process");

            let mut stdin = child.stdin.take().expect("Failed to open oracle stdin");
            let stdout = child.stdout.take().expect("Failed to open oracle stdout");

            let o_tx = oracle_tx.clone();
            tokio::spawn(async move {
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
            while let Ok(pulse) = oracle_rx.recv().await {
                if let ThoughtVector::VerifiedTruth { .. } = &pulse {
                    if let Ok(json) = serde_json::to_string(&pulse) {
                        let _ = stdin.write_all(format!("{}\n", json).as_bytes()).await;
                        let _ = stdin.flush().await;
                    }
                }
            }
        });

        Self {
            internal_bus: bus_tx,
        }
    }
}
