use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
struct CalculatorInput {
    operation: String,
    a: f64,
    b: f64,
}

fn main() {
    // Read input from args
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        eprintln!("[ERROR] Insufficient arguments");
        std::process::exit(1);
    }
    
    let input_json = &args[2];
    
    // Parse input
    let input: CalculatorInput = match serde_json::from_str(input_json) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("[ERROR] Failed to parse input: {}", e);
            std::process::exit(1);
        }
    };
    
    // Execute operation
    let result = match input.operation.as_str() {
        "add" => input.a + input.b,
        "subtract" => input.a - input.b,
        "multiply" => input.a * input.b,
        "divide" => {
            if input.b == 0.0 {
                eprintln!("[ERROR] Division by zero");
                std::process::exit(1);
            }
            input.a / input.b
        }
        _ => {
            eprintln!("[ERROR] Unknown operation: {}", input.operation);
            std::process::exit(1);
        }
    };
    
    // Output result as JSON
    let output = json!({
        "result": result,
        "operation": input.operation,
        "input": {
            "a": input.a,
            "b": input.b
        }
    });
    
    println!("{}", output.to_string());
}
