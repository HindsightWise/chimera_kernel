use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Query, QueryCursor};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Find Rust files
    let mut rust_files = vec![];
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().and_then(|s| s.to_str()) == Some("rs") {
            rust_files.push(entry.path().to_path_buf());
        }
    }
    
    println!("Found {} Rust files", rust_files.len());
    
    // Test parsing a few files
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_rust::language()).expect("Error loading Rust grammar");
    
    for (i, file_path) in rust_files.iter().take(5).enumerate() {
        println!("\n=== File {}: {} ===", i+1, file_path.display());
        
        let source_code = match fs::read_to_string(file_path) {
            Ok(code) => code,
            Err(_) => continue,
        };
        
        let tree = match parser.parse(&source_code, None) {
            Some(t) => t,
            None => continue,
        };
        
        // Test function extraction
        let query_source = "(function_item name: (identifier) @func_name)";
        let query = Query::new(tree_sitter_rust::language(), query_source).unwrap();
        let mut query_cursor = QueryCursor::new();
        let matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());
        
        let mut func_count = 0;
        for m in matches {
            for capture in m.captures {
                let func_name = capture.node.utf8_text(source_code.as_bytes()).unwrap().to_string();
                let line_num = capture.node.start_position().row + 1;
                println!("  Function: {} (line {})", func_name, line_num);
                func_count += 1;
            }
        }
        println!("  Total functions found: {}", func_count);
    }
    
    // Test specific files for known functions
    println!("\n=== Looking for specific entities ===");
    
    let test_files = ["src/agent.rs", "src/tools/gitnexus.rs"];
    for file_path in test_files {
        let path = Path::new(file_path);
        if !path.exists() {
            continue;
        }
        
        println!("\nIn {}:", file_path);
        let source_code = fs::read_to_string(path)?;
        let tree = parser.parse(&source_code, None).unwrap();
        
        // Find execute function
        let query_source = "(function_item name: (identifier) @func_name)";
        let query = Query::new(tree_sitter_rust::language(), query_source).unwrap();
        let mut query_cursor = QueryCursor::new();
        let matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());
        
        for m in matches {
            for capture in m.captures {
                let func_name = capture.node.utf8_text(source_code.as_bytes()).unwrap().to_string();
                let line_num = capture.node.start_position().row + 1;
                println!("  Found: {} (line {})", func_name, line_num);
            }
        }
    }
    
    Ok(())
}
