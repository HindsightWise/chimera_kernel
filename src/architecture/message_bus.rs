use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::{HashMap, VecDeque};
use tokio::sync::RwLock;
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
    subscriptions: RwLock<HashMap<String, Vec<Uuid>>>, // topic -> list of subscriber IDs
    message_queues: RwLock<HashMap<Uuid, VecDeque<Message>>>, // subscriber ID -> message queue
    message_store: RwLock<VecDeque<Message>>, // Persistent message history
    max_store_size: usize,
}

impl MessageBus {
    pub fn new(max_store_size: usize) -> Self {
        Self {
            subscriptions: RwLock::new(HashMap::new()),
            message_queues: RwLock::new(HashMap::new()),
            message_store: RwLock::new(VecDeque::with_capacity(max_store_size)),
            max_store_size,
        }
    }
    
    /// Subscribe to a topic
    pub async fn subscribe(&self, subscriber_id: Uuid, topic: &str) -> Result<()> {
        {
            let mut subscriptions = self.subscriptions.write().await;
            // Add to topic subscription list
            subscriptions
                .entry(topic.to_string())
                .or_insert_with(Vec::new)
                .push(subscriber_id);
        }
        
        {
            let mut queues = self.message_queues.write().await;
            // Ensure subscriber has a message queue
            queues.entry(subscriber_id).or_insert_with(VecDeque::new);
        }
        
        Ok(())
    }
    
    /// Unsubscribe from a topic
    pub async fn unsubscribe(&self, subscriber_id: Uuid, topic: &str) -> Result<()> {
        let mut subscriptions = self.subscriptions.write().await;
        
        if let Some(subscribers) = subscriptions.get_mut(topic) {
            subscribers.retain(|id| *id != subscriber_id);
            if subscribers.is_empty() {
                subscriptions.remove(topic);
            }
        }
        
        Ok(())
    }
    
    /// Publish a message to a topic
    pub async fn publish(&self, message: Message) -> Result<()> {
        // 1. Resolve subscriber IDs (Read Lock)
        let subscriber_ids: Vec<Uuid> = {
            let subscriptions = self.subscriptions.read().await;
            if let Some(ids) = subscriptions.get(&message.topic) {
                ids.clone()
            } else {
                Vec::new()
            }
        };
        
        // 2. Dispatch to queues (Write Lock 1)
        if !subscriber_ids.is_empty() {
            let mut queues = self.message_queues.write().await;
            for subscriber_id in subscriber_ids {
                if let Some(queue) = queues.get_mut(&subscriber_id) {
                    queue.push_back(message.clone());
                    
                    if queue.len() > 1000 {
                        queue.pop_front();
                    }
                }
            }
        }
        
        // 3. Store in persistent history (Write Lock 2)
        {
            let mut store = self.message_store.write().await;
            store.push_back(message);
            if store.len() > self.max_store_size {
                store.pop_front();
            }
        }
        
        Ok(())
    }
    
    /// Get next message for a subscriber
    pub async fn receive(&self, subscriber_id: Uuid) -> Option<Message> {
        let mut queues = self.message_queues.write().await;
        
        if let Some(queue) = queues.get_mut(&subscriber_id) {
            queue.pop_front()
        } else {
            None
        }
    }
    
    /// Check if subscriber has messages waiting
    pub async fn has_messages(&self, subscriber_id: Uuid) -> bool {
        let queues = self.message_queues.read().await;
        
        queues
            .get(&subscriber_id)
            .map_or(false, |queue| !queue.is_empty())
    }
    
    /// Get recent message history
    pub async fn get_recent_messages(&self, limit: usize) -> Vec<Message> {
        let store = self.message_store.read().await;
        store.iter().rev().take(limit).cloned().collect()
    }
    
    /// Clean up expired messages (TTL cleanup)
    pub async fn cleanup_expired(&self) -> usize {
        let now = Utc::now();
        let mut total_removed = 0;
        
        // Clean message queues
        {
            let mut queues = self.message_queues.write().await;
            for queue in queues.values_mut() {
                let before_len = queue.len();
                queue.retain(|msg| {
                    if let Some(ttl) = msg.ttl_secs {
                        let age = now.signed_duration_since(msg.timestamp).num_seconds();
                        age <= ttl as i64
                    } else {
                        true
                    }
                });
                total_removed += before_len - queue.len();
            }
        }
        
        // Clean message store
        {
            let mut store = self.message_store.write().await;
            let before_len = store.len();
            store.retain(|msg| {
                if let Some(ttl) = msg.ttl_secs {
                    let age = now.signed_duration_since(msg.timestamp).num_seconds();
                    age <= ttl as i64
                } else {
                    true
                }
            });
            total_removed += before_len - store.len();
        }
        total_removed
    }
}
