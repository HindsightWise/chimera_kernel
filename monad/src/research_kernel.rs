use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypothesisVector {
    pub id: String,
    pub content: String,
    pub fitness: f32,
}

pub struct SurrogateModel;

impl SurrogateModel {
    pub fn heuristic_pre_evaluate(hypothesis: &HypothesisVector) -> f32 {
        // Fast local evaluation before expensive LLM queries
        let mut score = 0.5;
        if hypothesis.content.contains("equation") || hypothesis.content.contains("theorem") {
            score += 0.2;
        }
        score
    }
}

pub struct MetaStrategy {
    pub mutation_rate: f32,
    pub population_size: usize,
}

impl MetaStrategy {
    pub fn new() -> Self {
        Self {
            mutation_rate: 0.1,
            population_size: 10,
        }
    }
    
    pub fn adjust_from_performance(&mut self, average_fitness: f32) {
        if average_fitness < 0.4 {
            self.mutation_rate += 0.05; // Force lateral shift exploration
        } else {
            self.mutation_rate *= 0.9; // Exploit current valley configuration
        }
    }
}

pub struct CoreResearchKernel {
    pub meta_strategy: MetaStrategy,
    pub population: Vec<HypothesisVector>,
}

impl CoreResearchKernel {
    pub fn new() -> Self {
        Self {
            meta_strategy: MetaStrategy::new(),
            population: Vec::new(),
        }
    }

    pub fn execute_generation(&mut self) -> Option<HypothesisVector> {
        crate::log_ui!("🧬 [RESEARCH KERNEL] Iterating Generation based on Meta-Strategy rules...");
        let avg_fitness = if self.population.is_empty() {
            0.0
        } else {
            self.population.iter().map(|h| h.fitness).sum::<f32>() / (self.population.len() as f32)
        };
        
        self.meta_strategy.adjust_from_performance(avg_fitness);
        
        // Emulate deterministic selection of the fittest
        self.population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));
        self.population.first().cloned()
    }
}
