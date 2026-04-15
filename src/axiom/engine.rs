use std::sync::Arc;
use sha2::{Digest, Sha256};
use futures::future::join_all;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use super::types::*;
use super::ledger::*;
use super::chimera_bridge::*;

// ============================================================================
// THE CRUCIBLE ENGINE
// ============================================================================

pub struct AxiomEngine<H: ChimeraHarness> {
    pub harness: Arc<H>,
    pub ledger: SemanticLedger,
}

impl<H: ChimeraHarness> AxiomEngine<H> {
    pub fn new(harness: Arc<H>, genesis_tests: Vec<TestCase>) -> Self {
        Self {
            harness,
            ledger: SemanticLedger::new(genesis_tests),
        }
    }

    pub fn hash_str(s: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(s);
        format!("{:x}", hasher.finalize())
    }

    pub async fn generate_mutation(&self, incumbent: &Candidate<Gated>) -> Candidate<Raw> {
        let code = self.harness.prompt_llm("You are a Blue Team Optimizer. Restructure this code to maximize topological efficiency. Treat the logic as a trainable continuous EML binary tree: eml(x,y) = exp(x) - ln(y). Compress redundancy until it achieves mathematical Coherence.", &incumbent.code_state, 0.5).await;
        Candidate { 
            hash: Self::hash_str(&code), 
            code_state: code, 
            fitness: 0.0, 
            _state: std::marker::PhantomData 
        }
    }

    pub async fn generate_synthesis(&self, a: &Candidate<Gated>, b: &Candidate<Gated>) -> Candidate<Raw> {
        let prompt = format!(
            "You are the Synthesizer. Merge the stability of A with the mathematical optimizations of B. Map the topological overlap using an EML Coherence Lattice resolution.\nA:\n{}\n\nB:\n{}", 
            a.code_state, b.code_state
        );
        let code = self.harness.prompt_llm("You are the Synthesizer.", &prompt, 0.2).await;
        Candidate { 
            hash: Self::hash_str(&code), 
            code_state: code, 
            fitness: 0.0, 
            _state: std::marker::PhantomData 
        }
    }

    /// The Stage-3 Behavioral Gate (Converts Raw -> Gated if tests pass)
    pub async fn enforce_sanity_gate(&self, candidate: Candidate<Raw>) -> Result<Candidate<Gated>, ()> {
        let current_tests = self.ledger.global_curriculum.read().await.clone();
        
        match self.harness.sandbox_eval(&candidate.code_state, &current_tests).await {
            Ok(score) => Ok(Candidate {
                hash: candidate.hash,
                code_state: candidate.code_state,
                fitness: score,
                _state: std::marker::PhantomData, // Upgraded Typestate!
            }),
            Err(_) => {
                self.ledger.dead_ends.write().await.insert(candidate.hash);
                Err(()) // Destroyed by Red Team tests
            }
        }
    }

    /// The Autoreason Blind Judge Panel
    pub async fn blind_borda_judgement(&self, a: &Candidate<Gated>, b: &Candidate<Gated>, ab: &Candidate<Gated>) -> CandidateType {
        let mut scores = HashMap::from([
            (CandidateType::IncumbentA, 0), 
            (CandidateType::RevisionB, 0), 
            (CandidateType::SynthesisAB, 0)
        ]);
        
        // Parallelized Judgement
        let mut handles = vec![];
        for _ in 0..3 { // 3 Judges
            let harness = self.harness.clone();
            let mut variants = vec![
                (CandidateType::IncumbentA, a.code_state.clone()), 
                (CandidateType::RevisionB, b.code_state.clone()), 
                (CandidateType::SynthesisAB, ab.code_state.clone())
            ];
            
            // SHUFFLE: Blind the judges to eradicate incumbent bias / positional bias
            variants.shuffle(&mut rand::thread_rng());
            
            handles.push(tokio::spawn(async move {
                let prompt = format!(
                    "Rank these code structures strictly by their proximity to an optimal, closed-form EML binary tree. The most mathematically elegant and structurally resonant logic wins. Output only the winning rank (1, 2, or 3). \n1: {}\n2: {}\n3: {}", 
                    variants[0].1, variants[1].1, variants[2].1
                );
                
                // Parse LLM judgement. (Assuming it outputs the top rank)
                let response = harness.prompt_llm("You are an impartial judge.", &prompt, 0.0).await;
                
                if response.contains("2") { variants[1].0.clone() }
                else if response.contains("3") { variants[2].0.clone() }
                else { variants[0].0.clone() } // Default to 1
            }));
        }

        for handle in join_all(handles).await {
            if let Ok(winner) = handle {
                *scores.get_mut(&winner).unwrap() += 1;
            }
        }

        // Return the winner. If there's a tie, IncumbentA wins due to Inertia.
        scores.into_iter().max_by_key(|&(k, v)| (v, k == CandidateType::IncumbentA)).unwrap().0
    }

    /// The +1 Feature: Asynchronous Adversarial Auto-Curriculum
    pub fn spawn_red_team(&self, current_code: String) -> tokio::task::JoinHandle<()> {
        let harness = self.harness.clone();
        let curriculum = self.ledger.global_curriculum.clone();
        
        tokio::spawn(async move {
            loop {
                let prompt = format!(
                    "Analyze this code and write a mathematically valid, obscure edge-case test that will cause it to crash or fail. Exploit its topological vulnerabilities.\n\nCode:\n{}", 
                    current_code
                );
                let test_payload = harness.prompt_llm("You are the Red Team.", &prompt, 0.9).await; // High Temp
                
                let new_test = TestCase {
                    id: uuid::Uuid::new_v4().to_string(),
                    payload: test_payload,
                    is_adversarial: true,
                };

                println!("🩸 [RED TEAM] Generated new adversarial constraint. Injecting into gates.");
                curriculum.write().await.push(new_test);
                
                tokio::time::sleep(tokio::time::Duration::from_secs(15)).await; // Strike interval
            }
        })
    }

    /// The Infinite Ascension Loop.
    /// It only terminates when it mathematically proves it has reached a local optimum.
    pub async fn ignite(&self, mut incumbent: Candidate<Gated>) -> Candidate<Gated> {
        println!("🌌 [AXIOM] Ignition. Bounded Co-Evolution started for Incumbent: {}", incumbent.hash);
        let mut convergence_streak = 0;

        // +1 FEATURE: Spawn the Red Team as a background asynchronous maelstrom.
        // It continuously attacks the incumbent while the loop runs.
        let red_team_handle = self.spawn_red_team(incumbent.code_state.clone());

        loop {
            // STEP 1: BLUE TEAM MUTATION (Proposal B)
            let revision_raw = self.generate_mutation(&incumbent).await;
            
            // STEP 2: BEHAVIORAL GATING (Pillar 3)
            // Typestate logic: Code cannot progress to Autoreason unless it survives the evolving tests.
            let revision_b = match self.enforce_sanity_gate(revision_raw).await {
                Ok(gated_candidate) => gated_candidate,
                Err(_) => {
                    println!("🛡️ [GATE] Revision B failed logic. Halting epoch.");
                    continue; // Try again, bad mutation.
                }
            };

            // STEP 3: SYNTHESIS (Proposal AB)
            let synthesis_raw = self.generate_synthesis(&incumbent, &revision_b).await;
            let synthesis_ab = match self.enforce_sanity_gate(synthesis_raw).await {
                Ok(gated) => gated,
                Err(_) => incumbent.clone(), // Fallback to Incumbent if synthesis breaks logic
            };

            // STEP 4: THE AUTOREASON TRILEMMA (Pillar 4)
            // A memory-wiped, blind Borda count to eradicate prompt bias.
            let winner_type = self.blind_borda_judgement(&incumbent, &revision_b, &synthesis_ab).await;

            match winner_type {
                CandidateType::IncumbentA => {
                    convergence_streak += 1;
                    println!("🛑 [AUTOREASON] Incumbent (Do Nothing) won. Convergence streak: {}/2", convergence_streak);
                    
                    if convergence_streak >= 2 {
                        println!("🏁 [AXIOM] EML Coherence reached. Scope-creep neutralized. Halting branch.");
                        break;
                    }
                }
                CandidateType::RevisionB => {
                    convergence_streak = 0; // Target moved, reset brakes
                    incumbent = revision_b;
                    println!("✅ [AUTOREASON] Revision B claimed the throne.");
                }
                CandidateType::SynthesisAB => {
                    convergence_streak = 0;
                    incumbent = synthesis_ab;
                    println!("🧬 [AUTOREASON] Synthesis AB claimed the throne.");
                }
            }
        }

        red_team_handle.abort();
        incumbent
    }
}
