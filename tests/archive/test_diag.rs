use std::fs;
use tree_sitter::{Parser, Query, QueryCursor};

fn main() {
    println!("=== Tree-sitter Rust parsing diagnostic ===");
    
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_rust::language()).expect("Error loading Rust grammar");
    
    // Test with a simple Rust source
    let source_code = r#"
pub fn execute(args: Value) -> String {
    let target = match args.get("target_entity").and_then(|v| v.as_str()) {
        Some(t) => t,
        None => return "[ERROR] Missing target_entity string parameter.".to_string(),
    };
    println!("Testing: {}", target);
    format!("Result: {}", target)
}

fn test() {
    execute(Value::Null);
}
"#;
    
    println!("Source code to parse:");
    println!("{}", source_code);
    
    let tree = parser.parse(source_code, None).unwrap();
    println!("\nParse tree depth: {}", tree.root_node().depth());
    
    // Find function definitions
    let func_query = Query::new(tree_sitter_rust::language(), "(function_item name: (identifier) @func_name)").unwrap();
    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&func_query, tree.root_node(), source_code.as_bytes());
    
    println!("\nFunctions found:");
    for m in matches {
        for capture in m.captures {
            let func_name = capture.node.utf8_text(source_code.as_bytes()).unwrap().to_string();
            let line_num = capture.node.start_position().row + 1;
            println!("  - {} at line {}", func_name, line_num);
        }
    }
    
    // Find function calls
    let call_query = Query::new(tree_sitter_rust::language(), "(call_expression function: (identifier) @call_name)").unwrap();
    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&call_query, tree.root_node(), source_code.as_bytes());
    
    println!("\nFunction calls found:");
    for m in matches {
        for capture in m.captures {
            let call_name = capture.node.utf8_text(source_code.as_bytes()).unwrap().to_string();
            let line_num = capture.node.start_position().row + 1;
            println!("  - {} at line {}", call_name, line_num);
        }
    }
}
