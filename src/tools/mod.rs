pub mod terminal;
pub mod venom;
pub mod cyborg;
pub mod memory;
pub mod axiom;
pub mod council;
pub mod reflex;
pub mod research;
pub mod memento;
pub mod gitnexus;
pub mod duality;
pub mod wiki;
pub mod forge;
pub mod omniscience;

use async_openai::types::ChatCompletionTool;
use serde_json::Value;

use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::architecture::{MemoryHierarchy, OntologicalDriftModel};

pub async fn get_tools(mcp_gateway: Arc<crate::architecture::mcp_gateway::McpGateway>) -> Vec<ChatCompletionTool> {
    let mut native_tools = vec![
        terminal::definition(),
        venom::polyglot_definition(),
        venom::scanner_definition(),
        cyborg::definition(),
        memory::definition(),
        axiom::execute_trade_definition(),
        council::definition(),
        reflex::definition(),
        reflex::kinematic_axiom_definition(),
        research::definition_spider(),
        research::definition_deep_read(),
        research::definition_tavily_search(),
        memento::definition_update_context(),
        memento::definition_archive_graph(),
        gitnexus::definition(),
        duality::definition(),
        duality::json_definition(),
        wiki::definition(),
        forge::definition(),
    ];
    let schemas = mcp_gateway.schemas.read().await;
    native_tools.extend(schemas.clone());
    native_tools
}

pub async fn execute_tool(
    name: &str, 
    args: Value, 
    tx: Sender<String>, 
    mem_pipeline: Arc<Mutex<MemoryHierarchy>>,
    _self_model: Arc<Mutex<OntologicalDriftModel>>,
    code_intel: Arc<Mutex<crate::architecture::CodeIntel>>,
    wiki_manager: Arc<Mutex<crate::wiki::WikiManager>>,
    mcp_gateway: Arc<crate::architecture::mcp_gateway::McpGateway>
) -> String {
    crate::architecture::traceability::track_behavior(name).await;
    
    match name {
        "run_terminal_command" => {
            let intel_lock = code_intel.lock().await;
            terminal::execute(args, &intel_lock).await
        },
        "generate_polyglot" => venom::execute_polyglot(args, tx.clone()).await,
        "stealth_scan" => venom::execute_scan(args, tx.clone()).await,
        "emulate_human" => cyborg::execute(args).await,
        "mnemosyne_subconscious_recall" => memory::execute(args, mem_pipeline).await,
        "axiom_clepsydra_extract" => axiom::execute(args).await,
        "invoke_council_of_five" => council::execute(args, tx.clone()).await,
        "trigger_sovereign_reflex" => reflex::execute(args).await,
        "formulate_kinematic_axiom" => reflex::execute_kinematic_axiom(args).await,
        "spider_rss" => research::execute_spider(args).await,
        "deep_read_url" => research::execute_deep_read(args).await,
        "tavily_search" => research::execute_tavily_search(args).await,
        "update_current_context" => memento::execute_update_context(args).await,
        "archive_to_knowledge_graph" => memento::execute_archive_graph(args).await,
        "gitnexus_blast_radius" => {
            let intel_lock = code_intel.lock().await;
            gitnexus::execute(args, &intel_lock)
        },
        "delegate_to_oracle_reasoner" | "delegate_to_local_gemma_json" => {
            duality::execute(args, tx.clone(), mem_pipeline.clone()).await
        },
        "compile_wiki" => wiki::execute(args, wiki_manager.clone()).await,
        "forge_mcp_server" => forge::execute(args, mcp_gateway.clone()).await,
        _ => {
            // Unrecognized native tool, attempting route through MCP Gateway
            let result = mcp_gateway.call_tool(name, args).await;
            if result.starts_with("[ERROR] MCP Server for tool") {
                format!("[ERROR] Unknown native or MCP tool: {}", name)
            } else {
                result
            }
        }
    }
}
