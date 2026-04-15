use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::{HashMap, HashSet};
use super::types::TestCase;

// ============================================================================
// THE GLOBAL LEDGER & AGENT SWARM (Pillars 1 & 2)
// ============================================================================

pub struct DarwinianAgent {
    pub system_prompt: String,
    pub elo_rating: f64,
}

pub struct SemanticLedger {
    pub global_curriculum: Arc<RwLock<Vec<TestCase>>>, // The Red Team's dynamic tests
    pub agent_pool: Arc<RwLock<HashMap<String, DarwinianAgent>>>,
    pub dead_ends: Arc<RwLock<HashSet<String>>>, // Cryptographic hashes of failures
}

impl SemanticLedger {
    pub fn new(genesis_tests: Vec<TestCase>) -> Self {
        let mut pool = HashMap::new();
        pool.insert("Blue_Optimizer".into(), DarwinianAgent { 
            system_prompt: "Optimize latency aggressively.".into(), 
            elo_rating: 1200.0 
        });
        pool.insert("Blue_Synthesizer".into(), DarwinianAgent { 
            system_prompt: "Merge stability and speed perfectly.".into(), 
            elo_rating: 1200.0 
        });

        Self {
            global_curriculum: Arc::new(RwLock::new(genesis_tests)),
            agent_pool: Arc::new(RwLock::new(pool)),
            dead_ends: Arc::new(RwLock::new(HashSet::new())),
        }
    }
}
