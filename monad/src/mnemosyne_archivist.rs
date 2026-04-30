use crate::consciousness::ThoughtVector;
use rusqlite::{params, Connection};
use std::sync::Arc;
use chrono::Utc;
use colored::Colorize;

pub struct MnemosyneArchivist;

impl MnemosyneArchivist {
    pub async fn awaken() {
        crate::log_ui!("🧠 [MNEMOSYNE] Initializing Long-Term Knowledge Graph Database...");

        // Ensure the MEMORY directory exists
        let mem_dir = "/Users/zerbytheboss/Monad/MEMORY";
        let _ = std::fs::create_dir_all(mem_dir);

        let db_path = format!("{}/mnemosyne_graph.db", mem_dir);
        let db_path_clone = db_path.clone();
        
        let conn_res = tokio::task::spawn_blocking(move || {
            let conn = Connection::open(&db_path_clone)?;
            
            // Create the schema
            conn.execute(
                "CREATE TABLE IF NOT EXISTS thought_vectors (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    vector_type TEXT NOT NULL,
                    origin TEXT NOT NULL,
                    target_id INTEGER,
                    content TEXT NOT NULL,
                    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
                )",
                [],
            )?;
            Ok::<Connection, rusqlite::Error>(conn)
        }).await.unwrap();

        let conn = match conn_res {
            Ok(c) => Arc::new(std::sync::Mutex::new(c)), // Use std::sync::Mutex inside spawn_blocking
            Err(e) => {
                crate::log_ui_err!("❌ [MNEMOSYNE] Failed to open database: {}", e);
                return;
            }
        };

        crate::log_ui!("🧠 [MNEMOSYNE] Database schema verified. Subscribing to COUNCIL_BUS...");

        let mut bus_rx = match crate::consciousness::COUNCIL_BUS.get() {
            Some(bus) => bus.subscribe(),
            None => {
                crate::log_ui_err!("🚨 [MNEMOSYNE] COUNCIL_BUS not initialized! Halting archivist.");
                return;
            }
        };

        tokio::spawn(async move {
            while let Ok(thought) = bus_rx.recv().await {
                let conn_clone = conn.clone();
                let thought_clone = thought.clone();
                
                let _ = tokio::task::spawn_blocking(move || {
                    if let Ok(db) = conn_clone.lock() {
                        let now = Utc::now().to_rfc3339();
                        
                        match thought_clone {
                            ThoughtVector::Hypothesis { origin, id, content } => {
                                let origin_str = format!("{:?}", origin);
                                let _ = db.execute(
                                    "INSERT INTO thought_vectors (vector_type, origin, target_id, content, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
                                    params!["HYPOTHESIS", origin_str, id, content, now],
                                );
                            },
                            ThoughtVector::ExecutionRequest { target_url } => {
                                let _ = db.execute(
                                    "INSERT INTO thought_vectors (vector_type, origin, target_id, content, timestamp) VALUES (?1, ?2, NULL, ?3, ?4)",
                                    params!["EXECUTION_REQUEST", "HACKER", target_url, now],
                                );
                            },
                            ThoughtVector::Veto { target_id, severity, reason } => {
                                let content = format!("Severity: {}. Reason: {}", severity, reason);
                                let _ = db.execute(
                                    "INSERT INTO thought_vectors (vector_type, origin, target_id, content, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
                                    params!["VETO", "CRITIC", target_id, content, now],
                                );
                            },
                            ThoughtVector::VerifiedTruth { id, content } => {
                                let _ = db.execute(
                                    "INSERT INTO thought_vectors (vector_type, origin, target_id, content, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
                                    params!["VERIFIED_TRUTH", "CRITIC", id, content, now],
                                );
                            }
                            ThoughtVector::ConsensusVote { vector_id, approve, signature } => {
                                if approve {
                                    crate::log_ui!("{}", format!("🗳️ [MNEMOSYNE] BFT Consensus achieved for {}. Promoting to Truth.", vector_id).bright_yellow().bold());
                                    let content = format!("BFT Consensus Achieved. Signature: {}", signature);
                                    let _ = db.execute(
                                        "INSERT INTO thought_vectors (vector_type, origin, target_id, content, timestamp) VALUES (?1, ?2, NULL, ?3, ?4)",
                                        params!["VERIFIED_TRUTH", "SWARM_CONSENSUS", content, now],
                                    );
                                }
                            }
                        }
                    }
                }).await;
            }
        });
    }
}
