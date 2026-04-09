// Standalone test of the new dependency_graph implementation
mod dependency_graph {
    // Include the actual implementation
    include!("src/architecture/dependency_graph.rs");
}

fn main() {
    println!("Testing GitNexus Phase 1 Implementation");
    
    // Create a new CodeIntel instance
    let mut intel = dependency_graph::CodeIntel::new();
    
    println!("Building knowledge graph...");
    intel.build_knowledge_graph(".");
    
    println!("Graph stats: {} nodes, {} edges", 
             intel.graph.node_count(), 
             intel.graph.edge_count());
    
    // Test with known entities
    let test_targets = ["execute", "run_kernel_loop", "main", "CodeIntel"];
    
    for target in test_targets {
        println!("\n=== Testing: {} ===", target);
        let report = intel.assess_blast_radius(target);
        
        println!("Risk score: {}", report.overall_risk_score);
        println!("Impacted functions: {}", report.impacted_functions.len());
        if !report.impacted_functions.is_empty() {
            for func in report.impacted_functions.iter().take(5) {
                println!("  - {}", func);
            }
            if report.impacted_functions.len() > 5 {
                println!("  ... and {} more", report.impacted_functions.len() - 5);
            }
        }
        
        println!("Upstream dependents: {}", report.upstream_dependents.len());
        if !report.upstream_dependents.is_empty() {
            for dep in report.upstream_dependents.iter().take(3) {
                println!("  - {}", dep);
            }
        }
        
        // Check if we got real results vs hardcoded
        if report.overall_risk_score == 0.85 
            && report.impacted_functions.len() == 2 
            && report.upstream_dependents.len() == 1 {
            println!("⚠️  WARNING: This looks like HARDCODED results!");
        } else {
            println!("✓ Looks like REAL graph analysis");
        }
    }
    
    // Show some nodes for verification
    println!("\n=== Sample graph nodes ===");
    let mut count = 0;
    for node_idx in intel.graph.node_indices() {
        let entity = &intel.graph[node_idx];
        if entity.kind == dependency_graph::EntityKind::Function {
            println!("Function: {} in {}:{}", 
                     entity.name, entity.file_path, entity.line_number);
            count += 1;
            if count >= 10 { break; }
        }
    }
}
