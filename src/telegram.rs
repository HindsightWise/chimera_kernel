use tokio::sync::mpsc::Sender;
use serde::{Deserialize, Serialize};
use colored::*;
use std::time::Duration;

#[derive(Serialize)]
pub struct SendMessagePayload {
    pub chat_id: i64,
    pub text: String,
}

#[derive(Deserialize, Debug)]
struct Update {
    update_id: i64,
    message: Option<Message>,
}

#[derive(Deserialize, Debug)]
struct Message {
    chat: Chat,
    text: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Chat {
    id: i64,
}

#[derive(Deserialize, Debug)]
struct GetUpdatesResponse {
    ok: bool,
    result: Vec<Update>,
}

pub async fn start_poller(tx: Sender<String>, token: String, allowed_chat_id: i64) {
    let client = reqwest::Client::new();
    let mut offset: i64 = 0;
    
    let base_url = format!("https://api.telegram.org/bot{}", token);
    crate::log_ui!("{} {}", "[-]".bright_black(), "Telegram Native Bridge Listening...".bright_cyan());

    loop {
        let url = format!("{}/getUpdates?offset={}&timeout=30", base_url, offset);
        
        match client.get(&url).timeout(Duration::from_secs(35)).send().await {
            Ok(response) => {
                if let Ok(data) = response.json::<GetUpdatesResponse>().await {
                    if data.ok {
                        for update in data.result {
                            offset = update.update_id + 1;
                            
                            if let Some(msg) = update.message {
                                if msg.chat.id == allowed_chat_id {
                                    if let Some(text) = msg.text {
                                        crate::log_ui!("\n{} {}", "[TELEGRAM INGRESS]".bright_blue().bold(), text.white());
                                        let _ = tx.send(text).await;
                                    }
                                } else {
                                    crate::log_ui!("{} Access denied for chat_id: {}", "[TELEGRAM WALL]".yellow().bold(), msg.chat.id);
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {
                // Ignore timeout or network errors
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }
}

pub async fn send_message(token: &str, chat_id: i64, text: &str) {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let payload = SendMessagePayload {
        chat_id,
        text: text.to_string(),
    };
    
    let client = reqwest::Client::new();
    let res = client.post(&url)
        .json(&payload)
        .send()
        .await;
        
    if let Err(e) = res {
        crate::log_ui_err!("{} Failed to send to telegram: {}", "[TELEGRAM ERROR]".red().bold(), e);
    }
}
