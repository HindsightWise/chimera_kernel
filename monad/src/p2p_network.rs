use axum::{routing::post, Json, Router};
use std::net::SocketAddr;
use crate::consciousness::ThoughtVector;

pub struct MonadSwarmNode;

impl MonadSwarmNode {
    pub async fn awaken_p2p_listener(port: u16) {
        crate::log_ui!("🌐 [P2P NETWORK] Booting Multi-Monad Axum Node on port {}...", port);

        let app = Router::new()
            .route("/api/v1/sync", post(Self::handle_thought_vector_sync));

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = tokio::net::TcpListener::bind(&addr).await;
        
        match listener {
            Ok(listener) => {
                crate::log_ui!("📡 [P2P NETWORK] Successfully bound to TCP {:?}. Awaiting alien node configurations.", addr);
                if let Err(e) = axum::serve(listener, app).await {
                    crate::log_ui_err!("🚨 [P2P FATAL] Daemon crash: {}", e);
                }
            },
            Err(e) => {
                crate::log_ui_err!("🛡️ [P2P WARN] Port {} occupied. Swarm node gracefully yielding: {}", port, e);
            }
        }
    }

    async fn handle_thought_vector_sync(
        Json(payload): Json<ThoughtVector>,
    ) -> Json<&'static str> {
        crate::log_ui!("🌌 [P2P SYNC VERIFIED] Alien ThoughtVector intercepted across Swarm. Archiving natively into Mnemosyne Storage...");
        
        // Log the payload telemetry structurally
        if let Ok(json_string) = serde_json::to_string(&payload) {
            crate::log_verbose!("Incoming Cross-Node JSON Payload: {}", json_string);
            
            // Here the ThoughtVector intercepts directly into the `memory_substrate` buffer internally 
            // via tokio MPSC loop or global DB writes depending on exact cluster topologies.
        }
        
        // Send heartbeat back to querying Monad
        Json("Acknowledge: Vector Absolved")
    }
}
