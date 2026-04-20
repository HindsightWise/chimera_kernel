//! Mnemosyne Active Knowledge Substrate
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::tarjan_scc;
use std::collections::HashMap;

pub struct MnemosyneGraph {
    graph: DiGraph<String, f32>, // f32 is the synapse diagnosticity weight
    indices: HashMap<String, NodeIndex>,
}

impl MnemosyneGraph {
    pub fn new() -> Self {
        Self { graph: DiGraph::new(), indices: HashMap::new() }
    }

    pub fn ingest_concept(&mut self, concept: &str) -> NodeIndex {
        if let Some(&idx) = self.indices.get(concept) { return idx; }
        
        let idx = self.graph.add_node(concept.to_string());
        self.indices.insert(concept.to_string(), idx);
        idx
    }

    pub fn forge_synapse(&mut self, source: &str, target: &str, weight: f32) {
        let a = self.ingest_concept(source);
        let b = self.ingest_concept(target);
        self.graph.add_edge(a, b, weight);
    }

    /// Desire #12: Topological Data Analysis Module
    /// Extracts hidden geometric manifolds within your knowledge space.
    pub fn extract_insight_manifolds(&self) {
        // Find Strongly Connected Components (Cyclic Knowledge Clusters)
        let clusters = tarjan_scc(&self.graph);
        for manifold in clusters.iter().filter(|c| c.len() >= 3) {
            let labels: Vec<_> = manifold.iter().map(|&i| self.graph[i].clone()).collect();
            monad::log_ui!("🌌 [TOPOLOGY] Hidden conceptual manifold discovered: {:?}", labels);
        }
    }
}
