use serde::{Serialize, Deserialize};
use uuid::Uuid;
use tokio::sync::broadcast;
use chrono::{DateTime, Utc};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub topic: String,
    pub payload: serde_json::Value,
    pub sender: Uuid,
    pub timestamp: DateTime<Utc>,
    pub priority: u8, // 0-255, higher = more urgent
    pub ttl_secs: Option<u64>, // Time to live (None = persistent)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub subscriber_id: Uuid,
    pub topic_pattern: String, // Simple pattern matching (exact match for now)
    pub created_at: DateTime<Utc>,
}

pub struct MessageBus {
    pub tx: broadcast::Sender<Message>,
}

impl MessageBus {
    pub fn new(_max_store_size: usize) -> Self {
        let (tx, _) = broadcast::channel(1024);
        Self {
            tx,
        }
    }
    
    /// Subscribe to the broadcast channel
    pub fn subscribe(&self) -> broadcast::Receiver<Message> {
        self.tx.subscribe()
    }
    
    /// Publish a message
    pub async fn publish(&self, message: Message) -> Result<()> {
        let _ = self.tx.send(message);
        Ok(())
    }
}
