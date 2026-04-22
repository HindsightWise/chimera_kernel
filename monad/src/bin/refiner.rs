use monad::consciousness::{Persona, ThoughtVector};
use std::io::{self, BufRead};

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if let Ok(ThoughtVector::Hypothesis {
                origin: _,
                id,
                content,
            }) = serde_json::from_str::<ThoughtVector>(&line)
            {
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
