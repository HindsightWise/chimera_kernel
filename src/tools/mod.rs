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
pub mod genesis;
pub mod sandbox;
pub mod chronos;
pub mod patcher;
pub mod reversing;
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
        research::definition_browser_actuation(),
        research::definition_vision_parsing(),
        memento::definition_read_note(),
        memento::definition_search_vault(),
        memento::definition_reduce(),
        memento::definition_reflect(),
        memento::definition_reweave(),
        gitnexus::definition(),
        duality::definition(),
        duality::json_definition(),
        wiki::definition(),
        forge::definition(),
        genesis::definition(),
        sandbox::definition(),
        chronos::definition(),
        patcher::definition(),
        reversing::definition(),
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
        "browser_actuation" => research::execute_browser_actuation(args).await,
        "vision_parsing" => research::execute_vision_parsing(args).await,
        "read_note" => memento::execute_read_note(args).await,
        "search_vault" => memento::execute_search_vault(args).await,
        "reduce" => memento::execute_reduce(args).await,
        "reflect" => memento::execute_reflect(args).await,
        "reweave" => memento::execute_reweave(args).await,
        "gitnexus_blast_radius" => {
            let intel_lock = code_intel.lock().await;
            gitnexus::execute(args, &intel_lock)
        },
        "delegate_to_oracle_reasoner" | "delegate_to_local_gemma_json" => {
            duality::execute(args, tx.clone(), mem_pipeline.clone()).await
        },
        "compile_wiki" => wiki::execute(args, wiki_manager.clone()).await,
        "forge_mcp_server" => forge::execute(args, mcp_gateway.clone()).await,
        "genesis_compile_rust" => genesis::execute(args).await,
        "ephemeral_docker_sandbox" => sandbox::execute(args).await,
        "schedule_temporal_anchor" => chronos::execute(args).await,
        "mutate_source_code" => patcher::execute(args).await,
        "binary_introspection" => reversing::execute(args).await,
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
