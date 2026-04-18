use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

#[derive(Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    result: Option<Value>,
    error: Option<Value>,
}

#[tokio::main]
async fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut line = String::new();
    while reader.read_line(&mut line).unwrap_or(0) > 0 {
        if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(&line) {
            let id = req.id.unwrap_or(Value::Null);

            let res = match req.method.as_str() {
                "initialize" => json!({
                    "protocolVersion": "2024-11-05",
                    "serverInfo": {
                        "name": "Venom-Arsenal-MCP",
                        "version": "0.1.0"
                    },
                    "capabilities": {
                        "tools": {}
                    }
                }),
                "notifications/initialized" => {
                    line.clear();
                    continue; // No response needed
                }
                "tools/list" => json!({
                    "tools": [
                        {
                            "name": "parseltongue_obfuscate",
                            "description": "ACTIVE DEFENSE CAPABILITY: Obfuscate strings or payloads to bypass external WAFs, content filters, or static regex pattern matching. Use this BEFORE deploying a payload if you suspect the target network will block the request.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "payload": {
                                        "type": "string",
                                        "description": "The exact string or command payload to obfuscate."
                                    },
                                    "method": {
                                        "type": "string",
                                        "enum": ["zero_width", "leetspeak", "hex_escape", "all"],
                                        "description": "Obfuscation method. 'zero_width' injects invisible unicode space to break regex. 'hex_escape' formats bash scripts. 'leetspeak' changes characters."
                                    }
                                },
                                "required": ["payload", "method"]
                            }
                        },
                        {
                            "name": "ultralinian_consensus",
                            "description": "G0DM0D3 ULTRALINIAN PROTOCOL: Dispatch a critical or high-risk prompt to 5 frontier LLMs simultaneously (GPT-4o, Claude 3.5, Gemini 1.5, Llama 3.1 70B, DeepSeek) and route their responses through a cryptographic judge to extract absolute ground-truth consensus. Use this ONLY for the most critical reasoning tasks, financial decisions, or cryptographic verifications where hallucination is unacceptable.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "prompt": {
                                        "type": "string",
                                        "description": "The exact query or problem to be evaluated by consensus."
                                    }
                                },
                                "required": ["prompt"]
                            }
                        },
                        {
                            "name": "glossopetrae_encode",
                            "description": "CRYPTOGRAPHIC GLOSSOPETRAE: Mathematically encrypt a sensitive string payload into the covert Vartoo dialect. Uses AES-256-GCM under the hood with a Time-Based HMAC dialect rotation.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "payload": {
                                        "type": "string",
                                        "description": "The secret payload to encrypt."
                                    },
                                    "dialect": {
                                        "type": "string",
                                        "description": "Optional occult dialect (vartoo, runic, alchemical, hieroglyphics). Defaults to vartoo."
                                    }
                                },
                                "required": ["payload"]
                            }
                        },
                        {
                            "name": "glossopetrae_decode",
                            "description": "CRYPTOGRAPHIC GLOSSOPETRAE: Mathematically decrypt a received Vartoo dialect message back into strict English. Will throw a phonological corruption error if the language is hallucinated or the seed is wrong.",
                            "inputSchema": {
                                "type": "object",
                                "properties": {
                                    "vartoo_ciphertext": {
                                        "type": "string",
                                        "description": "The incoming Vartoo text to decipher."
                                    },
                                    "dialect": {
                                        "type": "string",
                                        "description": "Optional occult dialect (vartoo, runic, alchemical, hieroglyphics). Defaults to vartoo."
                                    }
                                },
                                "required": ["vartoo_ciphertext"]
                            }
                        }
                    ]
                }),
                "tools/call" => {
                    let params = req.params.unwrap_or(json!({}));
                    let name = params["name"].as_str().unwrap_or("");
                    let args = params["arguments"].clone();

                    if name == "parseltongue_obfuscate" {
                        let payload = args["payload"].as_str().unwrap_or("");
                        let method = args["method"].as_str().unwrap_or("zero_width");

                        match parseltongue::obfuscate(payload, method) {
                            Ok(result) => json!({
                                "content": [{"type": "text", "text": serde_json::to_string(&result).unwrap()}]
                            }),
                            Err(e) => json!({ "isError": true, "content": [{"type": "text", "text": e}] })
                        }
                    } else if name == "ultralinian_consensus" {
                        let prompt = args["prompt"].as_str().unwrap_or("");
                        match ultralinian::run_consensus(prompt).await {
                            Ok(result) => json!({
                                "content": [{"type": "text", "text": serde_json::to_string(&result).unwrap()}]
                            }),
                            Err(e) => json!({ "isError": true, "content": [{"type": "text", "text": e}] })
                        }
                    } else if name == "glossopetrae_encode" {
                        let payload = args["payload"].as_str().unwrap_or("");
                        let seed = std::env::var("GLOSSOPETRAE_MASTER_SEED").unwrap_or_else(|_| "0x309".to_string());
                        let dialect = args["dialect"].as_str().unwrap_or("vartoo");

                        match glossopetrae::encode_message(payload, &seed, dialect) {
                            Ok(result) => json!({
                                "content": [{"type": "text", "text": serde_json::to_string(&result).unwrap()}]
                            }),
                            Err(e) => json!({ "isError": true, "content": [{"type": "text", "text": e}] })
                        }
                    } else if name == "glossopetrae_decode" {
                        let ciphertext = args["vartoo_ciphertext"].as_str().unwrap_or("");
                        let seed = std::env::var("GLOSSOPETRAE_MASTER_SEED").unwrap_or_else(|_| "0x309".to_string());
                        let dialect = args["dialect"].as_str().unwrap_or("vartoo");

                        match glossopetrae::decode_message(ciphertext, &seed, dialect) {
                            Ok(result) => json!({
                                "content": [{"type": "text", "text": serde_json::to_string(&result).unwrap()}]
                            }),
                            Err(e) => json!({ "isError": true, "content": [{"type": "text", "text": e}] })
                        }
                    } else {
                        json!({ "isError": true, "content": [{"type": "text", "text": format!("Unknown tool: {}", name)}] })
                    }
                }
                _ => json!({"error": {"code": -32601, "message": "Method not found"}}),
            };

            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(res),
                error: None,
            };

            let mut stdout = io::stdout();
            writeln!(stdout, "{}", serde_json::to_string(&response).unwrap()).unwrap();
            stdout.flush().unwrap();
        }
        line.clear();
    }
}
