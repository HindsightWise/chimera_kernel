use serde::{Serialize, Deserialize};

/// Purely internal, vendor-agnostic request format for the Monad Kernel.
/// This insulates the core logic from OpenAI schema drift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonadInferenceRequest {
    pub system_prompt: String,
    pub user_prompt: String,
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: u32,
}

/// Purely internal, vendor-agnostic response format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonadInferenceResponse {
    pub content: String,
    pub token_usage: TokenUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

pub trait InferenceAdapter {
    fn execute(&self, req: MonadInferenceRequest) -> Result<MonadInferenceResponse, anyhow::Error>;
}
