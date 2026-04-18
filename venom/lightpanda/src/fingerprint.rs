pub struct StealthFingerprint {
    pub user_agent: String,
    pub vendor: String,
    pub platform: String,
}

impl StealthFingerprint {
    pub fn new() -> Self {
        Self {
            user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            vendor: "Google Inc.".to_string(),
            platform: "MacIntel".to_string(),
        }
    }
}
