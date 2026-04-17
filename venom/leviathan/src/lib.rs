pub mod network;
pub mod tls;
pub mod dom;
pub mod error;

pub struct LeviathanDriver {
    _endpoint: String,
}

impl LeviathanDriver {
    pub fn new(endpoint: &str) -> Self {
        Self {
            _endpoint: endpoint.to_string(),
        }
    }
}
