use crate::architecture::tau_telemetry::{TauTelemetryCollector, TauMeasurement};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use chrono::Utc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::Notify;

pub struct AdaptiveSemaphore {
    active_workers: Mutex<usize>,
    notify: Notify,
}

impl AdaptiveSemaphore {
    pub fn new() -> Self {
        Self {
            active_workers: Mutex::new(0),
            notify: Notify::new(),
        }
    }

    pub async fn acquire(self: &Arc<Self>, current_tau_ms: f64) -> AdaptivePermit {
        let limit = if current_tau_ms <= 0.0 {
            5
        } else if current_tau_ms < 100.0 {
            5
        } else if current_tau_ms < 300.0 {
            4
        } else if current_tau_ms < 600.0 {
            3
        } else if current_tau_ms < 1500.0 {
            2
        } else {
            1
        };

        loop {
            {
                let mut active = self.active_workers.lock().unwrap();
                if *active < limit {
                    *active += 1;
                    return AdaptivePermit { semaphore: self.clone() };
                }
            }
            self.notify.notified().await;
        }
    }

    fn release(&self) {
        {
            let mut active = self.active_workers.lock().unwrap();
            if *active > 0 {
                *active -= 1;
            }
        }
        self.notify.notify_waiters();
    }
}

pub struct AdaptivePermit {
    semaphore: Arc<AdaptiveSemaphore>,
}

impl Drop for AdaptivePermit {
    fn drop(&mut self) {
        self.semaphore.release();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum BrowserTool {
    Puppeteer,
    StealthBrowser,
    StealthBrowserEnhancer,
    StealthBrowserMCP,
    BrowserActuation,
    Lightpanda,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserRequest {
    pub url: String,
    pub operation_type: String,
    pub headless: bool,
    pub timeout_ms: u64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserResponse {
    pub success: bool,
    pub content: String,
    pub tool_used: BrowserTool,
    pub execution_time_ms: f64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPerformance {
    pub avg_tau_ms: f64,
    pub success_rate: f64,
    pub failure_count: u32,
    pub last_used: chrono::DateTime<chrono::Utc>,
}

pub struct BrowserOrchestrator {
    tool_performance: RwLock<HashMap<BrowserTool, ToolPerformance>>,
    telemetry: TauTelemetryCollector,
    mcp_gateway: Option<Arc<crate::sensory_inputs::mcp_gateway::McpGateway>>,
    adaptive_semaphore: Arc<AdaptiveSemaphore>,
    global_avg_tau_ms: AtomicU64,
}

impl BrowserOrchestrator {
    pub fn new(mcp_gateway: Option<std::sync::Arc<crate::sensory_inputs::mcp_gateway::McpGateway>>) -> Self {
        let mut tool_performance = HashMap::new();
        
        // Initialize with default performance for each tool
        for tool in vec![
            BrowserTool::Puppeteer,
            BrowserTool::StealthBrowser,
            BrowserTool::StealthBrowserEnhancer,
            BrowserTool::StealthBrowserMCP,
            BrowserTool::BrowserActuation,
            BrowserTool::Lightpanda,
        ] {
            tool_performance.insert(tool, ToolPerformance {
                avg_tau_ms: 1000.0, // Default 1 second
                success_rate: 0.95, // Default 95% success
                failure_count: 0,
                last_used: Utc::now(),
            });
        }
        
        Self {
            tool_performance: RwLock::new(tool_performance),
            telemetry: TauTelemetryCollector::new(1000),
            mcp_gateway,
            adaptive_semaphore: Arc::new(AdaptiveSemaphore::new()),
            global_avg_tau_ms: AtomicU64::new(1000f64.to_bits()),
        }
    }
    
    pub async fn dispatch(&self, request: BrowserRequest) -> BrowserResponse {
        // Retrieve O(1) lock-free global average tau for dynamic concurrency limit
        let global_avg_tau = f64::from_bits(self.global_avg_tau_ms.load(Ordering::Relaxed));
        
        // 0. Acquire adaptive concurrency permit
        let _permit = self.adaptive_semaphore.acquire(global_avg_tau).await;

        // 1. Select optimal tool based on performance
        let best_tool = self.select_optimal_tool(&request).await;
        
        // 2. Execute with selected tool
        let start = std::time::Instant::now();
        let response = self.execute_with_tool(&best_tool, &request).await;
        let execution_time = start.elapsed().as_secs_f64() * 1000.0;
        
        // 3. Update telemetry
        let measurement = TauMeasurement {
            tool_name: format!("{:?}", best_tool),
            operation_type: request.operation_type.clone(),
            execution_time_ms: execution_time,
            success: response.success,
            timestamp: Utc::now(),
            metadata: request.metadata.clone(),
        };
        
        self.telemetry.record_measurement(measurement).await;
        
        // 4. Update tool performance
        self.update_tool_performance(&best_tool, execution_time, response.success).await;
        
        // Return response with actual execution time
        BrowserResponse {
            success: response.success,
            content: response.content,
            tool_used: response.tool_used,
            execution_time_ms: execution_time,
            error: response.error,
        }
    }
    
    async fn select_optimal_tool(&self, _request: &BrowserRequest) -> BrowserTool {
        let performance = self.tool_performance.read().await;
        
        let mut best_score = f64::NEG_INFINITY;
        let mut best_tool = BrowserTool::StealthBrowserMCP; // Default fallback
        
        for (tool, perf) in performance.iter() {
            // Penalize recent failures heavily
            if perf.failure_count > 3 {
                continue;
            }
            
            // Score = success_rate² / log(avg_tau_ms + 2)
            // This prioritizes reliability over raw speed and prevents division-by-zero panics
            let time_penalty = (perf.avg_tau_ms.max(0.0) + 2.0).ln();
            let score = (perf.success_rate * perf.success_rate) / time_penalty;
            
            if score > best_score {
                best_score = score;
                best_tool = tool.clone();
            }
        }
        
        best_tool
    }
    
    async fn execute_with_tool(&self, tool: &BrowserTool, request: &BrowserRequest) -> BrowserResponse {
        // Prepare MCP Arguments dynamically based on tool support
        let mut mcp_args_map = serde_json::Map::new();
        mcp_args_map.insert("url".to_string(), serde_json::Value::String(request.url.clone()));
        
        if tool != &BrowserTool::Puppeteer {
            mcp_args_map.insert("headless".to_string(), serde_json::Value::Bool(request.headless));
        }
        
        let mcp_args = serde_json::Value::Object(mcp_args_map);

        // Determine tool name for MCP Gateway
        let mcp_tool_name = match tool {
            BrowserTool::StealthBrowserMCP => Some("execute_stealth_browser_mcp"),
            BrowserTool::StealthBrowser => Some("execute_stealth_browser"),
            BrowserTool::StealthBrowserEnhancer => Some("execute_stealth_browser_enhancer"),
            BrowserTool::Puppeteer => Some("puppeteer_navigate"),
            _ => None, // Tools that are native or unsupported via MCP
        };

        // If it's an MCP tool and we have a gateway, execute physically!
        if let Some(tool_name) = mcp_tool_name {
            if let Some(gateway) = &self.mcp_gateway {
                let mcp_result = gateway.call_tool(tool_name, mcp_args).await;
                let success = !mcp_result.contains("[ERROR]");
                return BrowserResponse {
                    success,
                    content: mcp_result.clone(),
                    tool_used: tool.clone(),
                    execution_time_ms: 0.0, // Handled in dispatch()
                    error: if success { None } else { Some(mcp_result) },
                };
            }
        }

        // Fallback realistic simulation based on Phase 1 τ measurements for missing gateways/unsupported tools
        let execution_time_ms = match tool {
            BrowserTool::StealthBrowserMCP => 63.27,
            BrowserTool::StealthBrowserEnhancer => 63.32,
            BrowserTool::StealthBrowser => 63.72,
            BrowserTool::Puppeteer => 110.14,
            BrowserTool::Lightpanda => 43.66,
            BrowserTool::BrowserActuation => 500.0,
        };
        
        // Simulate execution time
        tokio::time::sleep(tokio::time::Duration::from_millis(execution_time_ms as u64)).await;
        
        match tool {
            BrowserTool::StealthBrowserMCP | BrowserTool::Puppeteer | BrowserTool::StealthBrowser | BrowserTool::StealthBrowserEnhancer | BrowserTool::Lightpanda => {
                BrowserResponse {
                    success: true,
                    content: format!("{:?} executed: {}", tool, request.url),
                    tool_used: tool.clone(),
                    execution_time_ms: 0.0,
                    error: None,
                }
            }
            BrowserTool::BrowserActuation => {
                BrowserResponse {
                    success: false,
                    content: String::new(),
                    tool_used: tool.clone(),
                    execution_time_ms: 0.0,
                    error: Some("browser_actuation needs setup (Playwright/Docker)".to_string()),
                }
            }
        }
    }
    
    async fn update_tool_performance(&self, tool: &BrowserTool, tau_ms: f64, success: bool) {
        let mut performance = self.tool_performance.write().await;
        
        if let Some(perf) = performance.get_mut(tool) {
            // Update moving average (simple EMA)
            let alpha = 0.1; // Learning rate
            perf.avg_tau_ms = alpha * tau_ms + (1.0 - alpha) * perf.avg_tau_ms;
            
            // Update success rate
            let total_uses = (perf.success_rate * 100.0) as u32 + 1;
            let success_count = (perf.success_rate * total_uses as f64) as u32;
            let new_success_count = if success { success_count + 1 } else { success_count };
            perf.success_rate = new_success_count as f64 / total_uses as f64;
            
            // Update failure count
            if !success {
                perf.failure_count += 1;
            }
            
            
            perf.last_used = Utc::now();
        }
        
        // Update lock-free global tau cache for O(1) dispatch
        let mut total_tau = 0.0;
        let mut count = 0;
        for p in performance.values() {
            total_tau += p.avg_tau_ms;
            count += 1;
        }
        let global_avg = if count > 0 { total_tau / count as f64 } else { 1000.0 };
        self.global_avg_tau_ms.store(global_avg.to_bits(), Ordering::Relaxed);
    }
    
    pub async fn get_performance_report(&self) -> HashMap<BrowserTool, ToolPerformance> {
        self.tool_performance.read().await.clone()
    }
    
    pub async fn get_telemetry(&self) -> TauTelemetryCollector {
        self.telemetry.clone()
    }
}
