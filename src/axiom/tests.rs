use super::*;
use async_trait::async_trait;
use std::sync::Arc;
use std::marker::PhantomData;

struct MockHarness;

#[async_trait]
impl ChimeraHarness for MockHarness {
    async fn prompt_llm(&self, _sys: &str, _user: &str, _temp: f32) -> String {
        "fn main() { println!(\"Hello Axiom\"); }".into()
    }
    
    async fn sandbox_eval(&self, _code: &str, _tests: &[TestCase]) -> Result<f64, String> {
        Ok(1.0)
    }
    
    async fn fork_worktree(&self, _base_hash: &str) -> String {
        "fork_123".into()
    }
}

#[tokio::test]
async fn test_axiom_ignition_convergence() {
    let harness = Arc::new(MockHarness);
    let engine = AxiomEngine::new(harness.clone(), vec![]);
    
    let incumbent = Candidate {
        hash: "genesis".into(),
        code_state: "fn main() {}".into(),
        fitness: 0.0,
        _state: PhantomData::<Gated>,
    };

    // The mock harness always returns the same code, so it should converge quickly.
    // However, the judge returns IncumbentA by default if no logic is found.
    // We expect it to halt eventually.
    let _final_candidate = engine.ignite(incumbent).await;
    
    assert!(true); // If it reaches here, it didn't loop forever
}
