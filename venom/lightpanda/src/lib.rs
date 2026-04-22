pub mod cdp;
pub mod wss;
pub mod fingerprint;
pub mod orchestrator;

pub struct LightpandaDriver {
    _endpoint: String,
}

impl LightpandaDriver {
    pub fn new(endpoint: &str) -> Self {
        Self {
            _endpoint: endpoint.to_string(),
        }
    }
}
