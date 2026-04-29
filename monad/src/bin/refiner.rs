use monad::consciousness::{Persona, ThoughtVector};
use std::collections::HashSet;
use std::io::{self, BufRead};

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let mut seen_hypotheses: HashSet<(Persona, u32)> = HashSet::new();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if let Ok(ThoughtVector::Hypothesis {
                origin,
                id,
                content,
            }) = serde_json::from_str::<ThoughtVector>(&line)
            {
                // CIRCUIT BREAKER: Only refine each unique (origin, id) pair once
                if !seen_hypotheses.insert((origin.clone(), id)) {
                    continue; // Dedup: skip already-processed hypothesis
                }
                
                // Simulate the "Alchemist" refining the idea
                let refined_content =
                    format!("{} [REFINED: Structurally optimized & clarified]", content);

                let refined_thought = ThoughtVector::Hypothesis {
                    origin: Persona::Refiner,
                    id,
                    content: refined_content,
                };

                println!("{}", serde_json::to_string(&refined_thought).unwrap());
            }
        }
    }
}
