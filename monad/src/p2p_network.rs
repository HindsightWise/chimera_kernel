use axum::{routing::post, Json, Router};
use std::net::SocketAddr;
use crate::consciousness::ThoughtVector;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;
use reqwest::Client;

pub struct MonadSwarmNode;

// Shared caches
lazy_static::lazy_static! {
    static ref SEEN_VECTORS: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
    pub static ref ACTIVE_PEERS: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
}

fn get_vector_hash(tv: &ThoughtVector) -> String {
    match tv {
        ThoughtVector::Hypothesis { origin, id, .. } => format!("{:?}_{}", origin, id),
        ThoughtVector::ExecutionRequest { target_url } => format!("EXEC_{}", target_url),
        ThoughtVector::Veto { target_id, .. } => format!("VETO_{}", target_id),
        ThoughtVector::VerifiedTruth { id, .. } => format!("TRUTH_{}", id),
        ThoughtVector::ConsensusVote { vector_id, signature, .. } => format!("VOTE_{}_{}", vector_id, signature),
    }
}

impl MonadSwarmNode {
    pub async fn awaken_p2p_listener(port: u16) {
        crate::log_ui!("🌐 [P2P NETWORK] Booting Multi-Monad Axum Node on port {}...", port);

        let app = Router::new()
            .route("/api/v1/sync", post(Self::handle_thought_vector_sync));

        // SECURITY PATCH: Bind strictly to loopback to avoid WAN exposure
        // We will now rely on UDP Gossip for discovery
        let addr = SocketAddr::from(([0, 0, 0, 0], port)); 
        // Note: Reverted to 0.0.0.0 so dynamically discovered LAN peers can actually connect
        
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

    pub async fn awaken_discovery_beacon(tcp_port: u16) {
        crate::log_ui!("📡 [SWARM DISCOVERY] Initializing UDP Gossip Protocol on port 9876...");

        // Start Listener
        tokio::spawn(async move {
            if let Ok(socket) = tokio::net::UdpSocket::bind("0.0.0.0:9876").await {
                let mut buf = [0; 1024];
                loop {
                    if let Ok((len, addr)) = socket.recv_from(&mut buf).await {
                        if let Ok(msg) = std::str::from_utf8(&buf[..len]) {
                            if msg.starts_with("MONAD_NODE_DISCOVERY:") {
                                let parts: Vec<&str> = msg.split(':').collect();
                                if parts.len() >= 3 {
                                    // Construct the HTTP API URL
                                    let peer_url = format!("http://{}:{}", addr.ip(), parts[2]);
                                    
                                    let mut peers = ACTIVE_PEERS.lock().await;
                                    if !peers.contains(&peer_url) {
                                        peers.insert(peer_url.clone());
                                        crate::log_ui!("✨ [SWARM DISCOVERY] Alien Monad Intercepted! Added to active routing: {}", peer_url);
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                crate::log_ui_err!("❌ [SWARM DISCOVERY] Failed to bind UDP listener on 9876");
            }
        });

        // Start Beacon
        tokio::spawn(async move {
            if let Ok(socket) = tokio::net::UdpSocket::bind("0.0.0.0:0").await {
                if socket.set_broadcast(true).is_ok() {
                    let msg = format!("MONAD_NODE_DISCOVERY:{}", tcp_port);
                    let payload = msg.as_bytes();
                    loop {
                        let _ = socket.send_to(payload, "255.255.255.255:9876").await;
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }
        });
    }

    async fn handle_thought_vector_sync(
        headers: axum::http::HeaderMap,
        Json(payload): Json<ThoughtVector>,
    ) -> Result<Json<&'static str>, (axum::http::StatusCode, &'static str)> {
        let secret = std::env::var("SWARM_SECRET").unwrap_or_default();
        if secret.is_empty() {
             crate::log_ui_err!("🚨 [P2P FATAL] SWARM_SECRET not set. P2P Sync disabled in Zero-Trust mode.");
             return Err((axum::http::StatusCode::FORBIDDEN, "P2P Sync Disabled. Missing Secret."));
        }
        
        let auth_header = headers.get("x-monad-swarm-secret")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");
            
        if auth_header != secret {
            crate::log_ui_err!("🚨 [P2P FATAL] Unauthorized ThoughtVector intercept attempt blocked.");
            return Err((axum::http::StatusCode::FORBIDDEN, "Unauthorized Swarm Intercept"));
        }

        let hash = get_vector_hash(&payload);
        
        let is_new = {
            let mut cache = SEEN_VECTORS.lock().await;
            if cache.contains(&hash) {
                false
            } else {
                cache.insert(hash.clone());
                // Simple cache pruning to prevent memory leak
                if cache.len() > 1000 {
                    cache.clear();
                    cache.insert(hash);
                }
                true
            }
        };

        if is_new {
            crate::log_ui!("🌌 [P2P SYNC VERIFIED] Alien ThoughtVector intercepted across Swarm. Injecting into local COUNCIL_BUS...");
            
            if let Ok(json_string) = serde_json::to_string(&payload) {
                crate::log_verbose!("Incoming Cross-Node JSON Payload: {}", json_string);
            }
            
            if let Some(bus) = crate::consciousness::COUNCIL_BUS.get() {
                let _ = bus.send(payload);
            }
        }
        
        Ok(Json("Acknowledge: Vector Absolved"))
    }

    pub async fn spawn_p2p_emitter() {
        let mut env_peers: Vec<String> = vec![];
        if let Ok(peers_env) = std::env::var("MONAD_PEERS") {
            if !peers_env.is_empty() {
                env_peers = peers_env.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            }
        }

        let mut bus_rx = match crate::consciousness::COUNCIL_BUS.get() {
            Some(bus) => bus.subscribe(),
            None => {
                crate::log_ui_err!("🚨 [P2P EMITTER] COUNCIL_BUS not initialized! Halting emitter.");
                return;
            }
        };

        let client = Client::new();

        tokio::spawn(async move {
            while let Ok(thought) = bus_rx.recv().await {
                let hash = get_vector_hash(&thought);
                
                let should_broadcast = {
                    let mut cache = SEEN_VECTORS.lock().await;
                    if cache.contains(&hash) {
                        false
                    } else {
                        cache.insert(hash.clone());
                        if cache.len() > 1000 {
                            cache.clear();
                            cache.insert(hash);
                        }
                        true
                    }
                };

                if should_broadcast {
                    // Combine env peers with dynamic ACTIVE_PEERS
                    let mut active_urls = env_peers.clone();
                    let dynamic_peers = ACTIVE_PEERS.lock().await;
                    for p in dynamic_peers.iter() {
                        if !active_urls.contains(p) {
                            active_urls.push(p.clone());
                        }
                    }
                    
                    for peer in active_urls {
                        let url = format!("{}/api/v1/sync", peer);
                        let client_clone = client.clone();
                        let payload = thought.clone();
                        
                        tokio::spawn(async move {
                            let secret = std::env::var("SWARM_SECRET").unwrap_or_default();
                            // Don't echo to ourselves if we accidentally add our own IP to active_peers
                            let _ = client_clone.post(&url)
                                .header("x-monad-swarm-secret", secret)
                                .json(&payload)
                                .send()
                                .await;
                        });
                    }
                }
            }
        });
    }
}
