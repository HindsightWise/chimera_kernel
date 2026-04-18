use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CdpRequest {
    pub id: u64,
    pub method: String,
    pub params: serde_json::Value,
}
