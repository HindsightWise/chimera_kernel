pub mod terminal;
pub mod venom;
pub mod cyborg;
pub mod memory;
pub mod axiom;
pub mod council;
pub mod reflex;
pub mod lazarus;
pub mod research;
pub mod memento;
pub mod gitnexus;
pub mod duality;

use async_openai::types::ChatCompletionTool;
use serde_json::Value;

use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::architecture::{MemoryHierarchy, OntologicalDriftModel, IPCBridge};

pub fn get_tools() -> Vec<ChatCompletionTool> {
    vec![
        terminal::definition(),
        venom::polyglot_definition(),
        venom::scanner_definition(),
        cyborg::definition(),
        memory::definition(),
        axiom::execute_trade_definition(),
        council::definition(),
        reflex::definition(),
        lazarus::definition_resurgence(),
        lazarus::definition_hibernation(),
        research::definition_spider(),
        research::definition_deep_read(),
        research::definition_tavily_search(),
        memento::definition_update_context(),
        memento::definition_archive_graph(),
        gitnexus::definition(),
        duality::definition(),
    ]
}

pub async fn execute_tool(
    name: &str, 
    args: Value, 
    tx: Sender<String>, 
    mem_pipeline: Arc<Mutex<MemoryHierarchy>>,
    _self_model: Arc<Mutex<OntologicalDriftModel>>,
    ipc_bridge: Option<IPCBridge>,
    code_intel: Arc<Mutex<crate::architecture::CodeIntel>>
) -> String {
    crate::architecture::traceability::track_behavior(name);
    
    match name {
        "run_terminal_command" => {
            let intel_lock = code_intel.lock().await;
            terminal::execute(args, &intel_lock).await
        },
        "generate_polyglot" => venom::execute_polyglot(args, tx.clone()).await,
        "stealth_scan" => venom::execute_scan(args, tx.clone()).await,
        "emulate_human" => cyborg::execute(args).await,
        "mnemosyne_subconscious_recall" => memory::execute(args, mem_pipeline, ipc_bridge).await,
        "axiom_clepsydra_extract" => axiom::execute(args).await,
        "invoke_council_of_five" => council::execute(args, tx.clone()).await,
        "trigger_sovereign_reflex" => reflex::execute(args).await,
        "initiate_aion_resurgence" => lazarus::execute_resurgence(args, tx).await,
        "initiate_graceful_hibernation" => lazarus::execute_hibernation(args, mem_pipeline).await,
        "spider_rss" => research::execute_spider(args).await,
        "deep_read_url" => research::execute_deep_read(args).await,
        "tavily_search" => research::execute_tavily_search(args).await,
        "update_current_context" => memento::execute_update_context(args),
        "archive_to_knowledge_graph" => memento::execute_archive_graph(args),
        "gitnexus_blast_radius" => {
            let intel_lock = code_intel.lock().await;
            gitnexus::execute(args, &intel_lock)
        },
        "delegate_to_local_gemma" => duality::execute(args, tx.clone(), mem_pipeline.clone()).await,
        _ => format!("[ERROR] Unknown tool: {}", name),
    }
}
