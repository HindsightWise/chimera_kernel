use chimera_kernel::architecture::CodeIntel;

fn main() {
    let mut intel = CodeIntel::new();
    println!("Building knowledge graph...");
    intel.build_knowledge_graph(".");
    
    println!("Graph has {} nodes", intel.graph.node_count());
    println!("Graph has {} edges", intel.graph.edge_count());
    
    // List some nodes
    println!("\nFirst 20 nodes:");
    for (i, node_idx) in intel.graph.node_indices().enumerate().take(20) {
        let entity = &intel.graph[node_idx];
        println!("  {}: {} ({:?}) in {}:{}", 
            i, entity.name, entity.kind, entity.file_path, entity.line_number);
    }
    
    // Test blast radius
    let report = intel.assess_blast_radius("run_kernel_loop");
    println!("\nBlast radius for 'run_kernel_loop':");
    println!("  Target: {}", report.target_entity);
    println!("  Impacted functions: {}", report.impacted_functions.len());
    for func in &report.impacted_functions {
        println!("    - {}", func);
    }
    println!("  Upstream dependents: {}", report.upstream_dependents.len());
    for dep in &report.upstream_dependents {
        println!("    - {}", dep);
    }
    println!("  Risk score: {}", report.overall_risk_score);
}
