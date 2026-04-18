use axum::{
    routing::post,
    Router, Json, extract::State,
};
use serde::Deserialize;
use tokio::sync::mpsc::Sender;
use std::net::SocketAddr;
use colored::*;

#[derive(Deserialize)]
struct TriggerPayload {
    message: String,
}

pub async fn start_server(tx: Sender<String>) {
    let app = Router::new()
        .route("/v1/webhook", post(handle_webhook))
        .with_state(tx);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    crate::log_ui!("{} {}", "[-]".bright_black(), format!("Native Axum Webhook Server listening on {}", addr).bright_black());
    
    match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            if let Err(e) = axum::serve(listener, app).await {
                crate::log_ui_err!("Webhook server execution collapsed: {}", e);
            }
        }
        Err(e) => {
            crate::log_ui_err!("Aegis boundary conflict! Failed to bind webhook on {}: {}", addr, e);
        }
    }
}

async fn handle_webhook(
    State(tx): State<Sender<String>>,
    Json(payload): Json<TriggerPayload>,
) -> &'static str {
    crate::log_ui!("\n{} {}", "[WEBHOOK] Received thermodynamic pulse:".yellow().bold(), payload.message.white());
    let _ = tx.send(payload.message).await;
    "ok"
}
