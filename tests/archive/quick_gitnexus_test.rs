use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the dependency_graph.rs file to see the actual implementation
    let content = fs::read_to_string("src/architecture/dependency_graph.rs")?;
    
    // Find the assess_blast_radius function
    let start = content.find("pub fn assess_blast_radius");
    if let Some(start_idx) = start {
        let snippet = &content[start_idx..];
        let end = snippet.find('}').unwrap_or(snippet.len());
        println!("=== assess_blast_radius function ===");
        println!("{}", &snippet[..end]);
        
        // Check if it has hardcoded values
        if snippet.contains("\"main\"") || snippet.contains("\"agent_loop\"") || snippet.contains("0.85") {
            println!("\n⚠️  WARNING: Function appears to have hardcoded values!");
        } else {
            println!("\n✓ Function appears to do real graph analysis");
        }
    }
    
    // Check for graph construction
    println!("\n=== Graph construction metrics ===");
    let graph_lines: Vec<&str> = content.lines()
        .filter(|l| l.contains("graph.add_edge") || l.contains("graph.add_node") || l.contains("node_map"))
        .collect();
    
    println!("Found {} graph operations", graph_lines.len());
    for line in graph_lines.iter().take(10) {
        println!("  {}", line.trim());
    }
    
    // Check query patterns
    println!("\n=== Tree-sitter queries ===");
    let query_lines: Vec<&str> = content.lines()
        .filter(|l| l.contains("Query::new") || l.contains("query_source") || l.contains("apply_query"))
        .collect();
    
    for line in query_lines.iter().take(10) {
        println!("  {}", line.trim());
    }
    
    Ok(())
}
