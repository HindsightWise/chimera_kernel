use uuid::Uuid;
use std::collections::{VecDeque, HashSet};
use crate::architecture::agent_trait::{AgentCapability, Task};
use chrono::Utc;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct DecompositionPattern {
    pub keyword: String,
    pub subtask_types: Vec<String>,
}

pub struct CapabilityGraph {
    pub capabilities: HashSet<AgentCapability>,
}

pub struct TaskDecomposer {
    pub historical_patterns: VecDeque<DecompositionPattern>,
    pub capability_graph: CapabilityGraph,
}

impl TaskDecomposer {
    pub fn new() -> Self {
        let mut tg = TaskDecomposer {
            historical_patterns: VecDeque::new(),
            capability_graph: CapabilityGraph {
                capabilities: HashSet::new(),
            },
        };
        // Seed patterns
        tg.historical_patterns.push_back(DecompositionPattern {
            keyword: "audit repo".to_string(),
            subtask_types: vec!["stealth_scan".to_string(), "gitnexus_blast_radius".to_string(), "tavily_search".to_string()],
        });
        tg.historical_patterns.push_back(DecompositionPattern {
            keyword: "market analysis".to_string(),
            subtask_types: vec!["tavily_search".to_string(), "axiom_clepsydra_extract".to_string()],
        });
        tg
    }

    pub fn decompose(&self, instruction: &str, parent_priority: u8) -> Vec<Task> {
        let mut subtasks = Vec::new();
        let mut previous_id: Option<Uuid> = None;

        let instruction_lower = instruction.to_lowercase();
        
        let mut string_hasher = DefaultHasher::new();
        instruction_lower.hash(&mut string_hasher);
        let mut base_seed = string_hasher.finish();
        
        for pattern in &self.historical_patterns {
            if instruction_lower.contains(&pattern.keyword) {
                for stype in &pattern.subtask_types {
                    let task_id = Uuid::new_v4();
                    
                    let mut reqs = HashSet::new();
                    match stype.as_str() {
                        "stealth_scan" | "run_terminal_command" | "generate_polyglot" => { reqs.insert(AgentCapability::ToolExecution); }
                        "gitnexus_blast_radius" => { reqs.insert(AgentCapability::Security); }
                        "tavily_search" | "spider_rss" | "deep_read_url" => { reqs.insert(AgentCapability::Reasoning); }
                        "axiom_clepsydra_extract" => { reqs.insert(AgentCapability::Trading); }
                        "delegate_to_local_gemma" => { reqs.insert(AgentCapability::Reasoning); }
                        _ => {}
                    }
                    
                    let mut dependencies = Vec::new();
                    if let Some(pid) = previous_id {
                        dependencies.push(pid);
                    }
                    
                    // Deterministic seed mapping
                    if let Some(pid) = previous_id {
                        base_seed = base_seed.wrapping_add(pid.as_u128() as u64);
                    }
                    let mut prng = ChaCha8Rng::seed_from_u64(base_seed);
                    
                    let topological_depth = if previous_id.is_some() { 2 } else { 1 };
                    
                    let radius = if topological_depth == 1 { 1.0 } else { 0.33 };
                    let theta = prng.gen::<f32>() * std::f32::consts::TAU;
                    let phi = (prng.gen::<f32>() * 2.0 - 1.0).acos();
                    
                    let x = radius * phi.sin() * theta.cos();
                    let y = radius * phi.sin() * theta.sin();
                    let z = radius * phi.cos();
                    
                    let stask = Task {
                        id: task_id,
                        task_type: stype.to_string(),
                        payload: serde_json::json!({"instruction": instruction}),
                        required_capabilities: reqs,
                        priority: parent_priority,
                        dependencies,
                        created_at: Utc::now(),
                        timeout_secs: Some(300),
                        geometric_node: [x, y, z],
                        topological_depth,
                        execution_attempts: 0,
                    };
                    subtasks.push(stask);
                    previous_id = Some(task_id); // Chain linearly for default parsing
                }
                return subtasks; // Early return for matched pattern
            }
        }
        
        // Generic fallback - just push back a basic reasoning task if no pattern matches
        let task_id = Uuid::new_v4();
        let mut reqs = HashSet::new();
        reqs.insert(AgentCapability::Reasoning);
        
        let mut prng = ChaCha8Rng::seed_from_u64(base_seed);
        let theta = prng.gen::<f32>() * std::f32::consts::TAU;
        let phi = (prng.gen::<f32>() * 2.0 - 1.0).acos();
        
        // Level 1 baseline mapping
        let x = phi.sin() * theta.cos();
        let y = phi.sin() * theta.sin();
        let z = phi.cos();

        subtasks.push(Task {
            id: task_id,
            task_type: "research_basic".to_string(),
            payload: serde_json::json!({"instruction": instruction}),
            required_capabilities: reqs,
            priority: parent_priority,
            dependencies: vec![],
            created_at: Utc::now(),
            timeout_secs: Some(300),
            geometric_node: [x, y, z],
            topological_depth: 1,
            execution_attempts: 0,
        });
        
        subtasks
    }
}
