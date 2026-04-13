use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock, mpsc, oneshot};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use async_openai::types::{ChatCompletionTool, ChatCompletionToolType, FunctionObject};

#[derive(Serialize, Deserialize, Debug)]
struct McpRegistry {
    servers: HashMap<String, McpServerConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
struct McpServerConfig {
    command: String,
    args: Vec<String>,
    #[serde(default)]
    env: HashMap<String, String>,
}

#[derive(Clone)]
pub struct McpGateway {
    pub tools_cache: Arc<RwLock<HashMap<String, String>>>, // tool_name -> server_name
    pub schemas: Arc<RwLock<Vec<ChatCompletionTool>>>,
    dispatchers: Arc<RwLock<HashMap<String, mpsc::Sender<(Value, oneshot::Sender<Value>)>>>>,
}

impl McpGateway {
    pub fn new() -> Self {
        Self {
            tools_cache: Arc::new(RwLock::new(HashMap::new())),
            schemas: Arc::new(RwLock::new(Vec::new())),
            dispatchers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn load_servers(&self) {
        let registry_content = match tokio::fs::read_to_string("mcp_registry.json").await {
            Ok(c) => c,
            Err(_) => return,
        };
        let registry: McpRegistry = match serde_json::from_str(&registry_content) {
            Ok(r) => r,
            Err(_) => return,
        };

        for (server_name, config) in registry.servers {
            self.spawn_server(server_name, config).await;
        }
    }

    pub async fn spawn_server_from_value(&self, server_name: String, config_val: Value) {
        if let Ok(config) = serde_json::from_value::<McpServerConfig>(config_val) {
            self.spawn_server(server_name, config).await;
        }
    }

    async fn spawn_server(&self, server_name: String, config: McpServerConfig) {
        crate::log_ui!("[MCP GATEWAY] Booting server: {}", server_name);
        
        let mut child_cmd = Command::new(&config.command);
        child_cmd.args(&config.args)
                 .envs(&config.env)
                 .stdin(std::process::Stdio::piped())
                 .stdout(std::process::Stdio::piped())
                 .stderr(std::process::Stdio::piped());

        let mut child = match child_cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                crate::log_ui!("[MCP GATEWAY] Failed to spawn {}: {}", server_name, e);
                return;
            }
        };

        let mut stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        let (req_tx, mut req_rx) = mpsc::channel::<(Value, oneshot::Sender<Value>)>(10);
        self.dispatchers.write().await.insert(server_name.clone(), req_tx.clone());

        let pending_responses: Arc<Mutex<HashMap<u64, oneshot::Sender<Value>>>> = Arc::new(Mutex::new(HashMap::new()));
        let pr_clone = pending_responses.clone();

        // Stdout reader
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                if let Ok(value) = serde_json::from_str::<Value>(&line) {
                    if let Some(id) = value.get("id").and_then(|id| id.as_u64()) {
                        let mut pr = pr_clone.lock().await;
                        if let Some(sender) = pr.remove(&id) {
                            let _ = sender.send(value);
                        }
                    }
                }
            }
        });

        // Request sender
        let pr_clone2 = pending_responses.clone();
        tokio::spawn(async move {
            let mut msg_id = 1;
            while let Some((mut req, cb)) = req_rx.recv().await {
                req["jsonrpc"] = serde_json::json!("2.0");
                
                let is_notification = req.get("method").and_then(|m| m.as_str()).map(|m| m.starts_with("notifications/")).unwrap_or(false);
                
                if !is_notification {
                    let id = msg_id;
                    msg_id += 1;
                    req["id"] = serde_json::json!(id);
                    pr_clone2.lock().await.insert(id, cb);
                }
                
                let mut str_val = serde_json::to_string(&req).unwrap();
                str_val.push('\n');
                if stdin.write_all(str_val.as_bytes()).await.is_err() {
                    break;
                }
            }
        });

        // Initialize handshake
        let init_req = serde_json::json!({
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "chimera",
                    "version": "1.0.0"
                }
            }
        });
        
        let (cb_tx, cb_rx) = oneshot::channel();
        let _ = req_tx.send((init_req, cb_tx)).await;
        if let Ok(Ok(_)) = tokio::time::timeout(std::time::Duration::from_secs(30), cb_rx).await {
            // send initialized notification
            let initialized_req = serde_json::json!({
                "method": "notifications/initialized",
                "params": {}
            });
            let (cb_tx2, _) = oneshot::channel();
            let _ = req_tx.send((initialized_req, cb_tx2)).await;
        }

        // Send tools/list
        let tools_list_req = serde_json::json!({
            "method": "tools/list",
            "params": {}
        });
        let (cb_tx, cb_rx) = oneshot::channel();
        let _ = req_tx.send((tools_list_req, cb_tx)).await;
        if let Ok(Ok(res)) = tokio::time::timeout(std::time::Duration::from_secs(30), cb_rx).await {
            if let Some(tools) = res.get("result").and_then(|r| r.get("tools")).and_then(|t| t.as_array()) {
                let mut schemas = self.schemas.write().await;
                let mut tools_cache = self.tools_cache.write().await;

                for tool in tools {
                    if let (Some(name), Some(desc), Some(schema)) = (
                        tool.get("name").and_then(|n| n.as_str()),
                        tool.get("description").and_then(|d| d.as_str()),
                        tool.get("inputSchema")
                    ) {
                        tools_cache.insert(name.to_string(), server_name.clone());

                        let open_ai_tool = ChatCompletionTool {
                            r#type: ChatCompletionToolType::Function,
                            function: FunctionObject {
                                name: name.to_string(),
                                description: Some(desc.to_string()),
                                parameters: Some(schema.clone()),
                            },
                        };
                        schemas.push(open_ai_tool);
                        crate::log_ui!("[MCP GATEWAY] Ingested Native Tool: {} [From: {}]", name, server_name);
                    }
                }
            }
        }
    }

    pub async fn call_tool(&self, tool_name: &str, arguments: Value) -> String {
        let server_name = {
            let tc = self.tools_cache.read().await;
            match tc.get(tool_name) {
                Some(s) => s.clone(),
                None => return format!("[ERROR] MCP Server for tool {} not found.", tool_name),
            }
        };

        let req_tx = {
            let dp = self.dispatchers.read().await;
            match dp.get(&server_name) {
                Some(tx) => tx.clone(),
                None => return format!("[ERROR] MCP Server {} is dead.", server_name),
            }
        };

        let req = serde_json::json!({
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": arguments
            }
        });

        let (cb_tx, cb_rx) = oneshot::channel();
        if req_tx.send((req, cb_tx)).await.is_ok() {
            if let Ok(response) = cb_rx.await {
                if let Some(error) = response.get("error") {
                    return format!("[MCP ERROR] {}", error);
                }
                if let Some(content) = response.get("result").and_then(|r| r.get("content")).and_then(|c| c.as_array()) {
                    let text_parts: Vec<String> = content.iter().filter_map(|c| c.get("text").and_then(|t| t.as_str())).map(String::from).collect();
                    return text_parts.join("\n");
                }
            }
        }
        "[ERROR] MCP Tool call timed out or failed to execute.".to_string()
    }
}
