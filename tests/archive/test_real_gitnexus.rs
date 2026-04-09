use std::path::Path;
use std::fs;
use tree_sitter::{Parser, Query, QueryCursor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing tree-sitter parsing...");
    
    // Test parsing src/tools/gitnexus.rs
    let test_file = "src/tools/gitnexus.rs";
    if !Path::new(test_file).exists() {
        println!("File not found: {}", test_file);
        return Ok(());
    }
    
    let source_code = fs::read_to_string(test_file)?;
    println!("File size: {} bytes", source_code.len());
    
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_rust::language()).expect("Error loading Rust grammar");
    
    let tree = match parser.parse(&source_code, None) {
        Some(t) => t,
        None => {
            println!("Failed to parse");
            return Ok(());
        }
    };
    
    // Look for the execute function
    println!("\nLooking for functions in {}:", test_file);
    
    let query_source = "(function_item name: (identifier) @func_name)";
    let query = Query::new(tree_sitter_rust::language(), query_source).unwrap();
    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());
    
    let mut found_execute = false;
    for m in matches {
        for capture in m.captures {
            let func_name = capture.node.utf8_text(source_code.as_bytes()).unwrap().to_string();
            let line_num = capture.node.start_position().row + 1;
            println!("  Found function: {} at line {}", func_name, line_num);
            if func_name == "execute" || func_name == "definition" {
                found_execute = true;
            }
        }
    }
    
    if !found_execute {
        println!("WARNING: Could not find execute or definition functions!");
    }
    
    // Now test the dependency_graph.rs file
    println!("\n=== Checking dependency_graph.rs ===");
    let dep_file = "src/architecture/dependency_graph.rs";
    let dep_content = fs::read_to_string(dep_file)?;
    
    // Check if it has hardcoded values
    if dep_content.contains("vec![\"main\".to_string(), \"agent_loop\".to_string()]") {
        println!("⚠️  WARNING: dependency_graph.rs contains HARDCODED values!");
    } else if dep_content.contains("graph.neighbors_directed") {
        println!("✓ dependency_graph.rs appears to have REAL graph traversal");
    }
    
    // Look for the assess_blast_radius function
    if let Some(start) = dep_content.find("pub fn assess_blast_radius") {
        let snippet = &dep_content[start..];
        let end = snippet.find('}').unwrap_or(snippet.len());
        let func_text = &snippet[..end];
        
        if func_text.contains("0.85") || func_text.contains("\"main\"") {
            println!("⚠️  HARDCODED assess_blast_radius detected!");
        } else {
            println!("✓ Real assess_blast_radius implementation found");
        }
    }
    
    Ok(())
}
