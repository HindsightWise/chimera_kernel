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
    callback_query: Option<CallbackQuery>,
}

#[derive(Deserialize, Debug)]
struct CallbackQuery {
    id: String,
    data: Option<String>,
    #[allow(dead_code)]
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

#[derive(Serialize)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub callback_data: String,
}

#[derive(Serialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Serialize)]
pub struct SendMessagePayloadWithMarkup {
    pub chat_id: i64,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

lazy_static::lazy_static! {
    static ref APPROVAL_REGISTRY: std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<String, tokio::sync::oneshot::Sender<String>>>> = std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new()));
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
                            } else if let Some(cq) = update.callback_query {
                                if let Some(data) = cq.data {
                                    if data.starts_with("APPROVE_") || data.starts_with("DENY_") {
                                        let parts: Vec<&str> = data.split('_').collect();
                                        if parts.len() == 2 {
                                            let action = parts[0];
                                            let id = parts[1];
                                            let mut reg = APPROVAL_REGISTRY.lock().await;
                                            if let Some(tx_reply) = reg.remove(id) {
                                                let _ = tx_reply.send(action.to_string());
                                                crate::log_ui!("\n{} User pressed: {}", "[HITL GATEWAY]".bright_green().bold(), action);
                                            }
                                        }
                                        // Answer callback query to remove loading state
                                        let _ = client.get(&format!("{}/answerCallbackQuery?callback_query_id={}", base_url, cq.id)).send().await;
                                    }
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
    let payload = SendMessagePayloadWithMarkup {
        chat_id,
        text: text.to_string(),
        reply_markup: None,
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

pub async fn ask_permission(token: &str, chat_id: i64, action_desc: &str) -> bool {
    let action_id = uuid::Uuid::new_v4().to_string().replace("-", "")[0..8].to_string();
    let markup = InlineKeyboardMarkup {
        inline_keyboard: vec![vec![
            InlineKeyboardButton { text: "Approve \u{2705}".to_string(), callback_data: format!("APPROVE_{}", action_id) },
            InlineKeyboardButton { text: "Deny \u{274C}".to_string(), callback_data: format!("DENY_{}", action_id) }
        ]]
    };
    
    let (tx, rx) = tokio::sync::oneshot::channel();
    {
        let mut reg = APPROVAL_REGISTRY.lock().await;
        reg.insert(action_id.clone(), tx);
    }
    
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let payload = SendMessagePayloadWithMarkup {
        chat_id,
        text: format!("⚠️ IMMEDIATE INTERVENTION REQUIRED:\n\n{}\n\nDo you want to proceed?", action_desc),
        reply_markup: Some(markup),
    };
    
    let client = reqwest::Client::new();
    if let Err(e) = client.post(&url).json(&payload).send().await {
        crate::log_ui_err!("{} Failed to send permission request: {}", "[TELEGRAM ERROR]".red().bold(), e);
        return false;
    }
    
    match rx.await {
        Ok(result) => result == "APPROVE",
        Err(_) => false,
    }
}

pub async fn dispatch_proposal_alert(token: &str, chat_id: i64, topic: &str, file_path: &str) {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    let payload = SendMessagePayloadWithMarkup {
        chat_id,
        text: format!("🚨 [EVOLUTION REQUIRED] 🚨\n\nTopological Limitation Detected: {}\n\nA new implementation proposal has been synthesized and compiled by The Monad.\n\nPlease review the artifact at:\n`{}`", topic, file_path),
        reply_markup: None,
    };
    
    let client = reqwest::Client::new();
    let res = client.post(&url)
        .json(&payload)
        .send()
        .await;
        
    if let Err(e) = res {
        crate::log_ui_err!("{} Failed to dispatch proposal alert: {}", "[TELEGRAM ERROR]".red().bold(), e);
    }
}
