use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct ResurrectionState {
    pub rapid_crash_count: u32,
    pub last_crash_timestamp: u64,
    pub needs_ping: bool,
}

impl ResurrectionState {
    pub fn load() -> Self {
        if let Ok(data) = fs::read_to_string(".kernel_death_record.json") {
            if let Ok(state) = serde_json::from_str(&data) {
                return state;
            }
        }
        Self {
            rapid_crash_count: 0,
            last_crash_timestamp: 0,
            needs_ping: false,
        }
    }

    pub fn save(&self) {
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write(".kernel_death_record.json", data);
        }
    }
}
