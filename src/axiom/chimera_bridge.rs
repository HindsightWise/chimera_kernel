use async_trait::async_trait;
use super::types::TestCase;

// ============================================================================
// THE CHIMERA BRIDGE (Your Kernel Interface)
// Implement this trait in your kernel to give Axiom physical senses.
// ============================================================================

#[async_trait]
pub trait ChimeraHarness: Send + Sync + 'static {
    /// Dispatches tasks to your underlying LLM routing (Claude, GPT, local Llama)
    async fn prompt_llm(&self, system_prompt: &str, user_prompt: &str, temp: f32) -> String;

    /// Executes code securely (e.g., WASM, Docker, Firecracker). 
    /// Returns Ok(fitness_score) if ALL tests pass, Err(trace) if logic breaks.
    async fn sandbox_eval(&self, code: &str, tests: &[TestCase]) -> Result<f64, String>;
    
    /// Forks the environment for isolated parallel tree exploration
    async fn fork_worktree(&self, base_hash: &str) -> String;
}
