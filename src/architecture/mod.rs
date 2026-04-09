pub mod memory_hierarchy;
pub mod self_model;
pub mod xenoactualization;
pub mod kinematics;
pub mod plugins;
pub mod dependency_graph;
pub mod trap_in;
pub mod fear_automation;
pub mod traceability;
pub mod duality;
pub mod agent_trait;
pub mod agent_registry;
pub mod message_bus;
pub mod task_manager;
pub mod specialized_agents;
pub mod multi_agent_kernel;
pub mod sensory_drift;pub mod task_decomposer;
pub mod agent_coordinator;


pub static GLOBAL_TX: tokio::sync::OnceCell<tokio::sync::mpsc::Sender<String>> = tokio::sync::OnceCell::const_new();
pub static GLOBAL_CODE_INTEL: tokio::sync::OnceCell<std::sync::Arc<tokio::sync::Mutex<CodeIntel>>> = tokio::sync::OnceCell::const_new();
pub static GLOBAL_MEM_PIPELINE: tokio::sync::OnceCell<std::sync::Arc<tokio::sync::Mutex<MemoryHierarchy>>> = tokio::sync::OnceCell::const_new();
pub static GLOBAL_WIKI_MANAGER: tokio::sync::OnceCell<std::sync::Arc<tokio::sync::Mutex<crate::wiki::WikiManager>>> = tokio::sync::OnceCell::const_new();

pub use memory_hierarchy::{MemoryHierarchy, MemoryChunk};
pub use self_model::{OntologicalDriftModel, OntologicalState, Projection};
// Pub use embedding_bridge::IPCBridge; <- REMOVED FOR SPRINT 3 NATIVE RUST MIGRATION
pub use xenoactualization::{TranslationLayer, DriftMonitor};
pub use kinematics::KinematicCortex;
pub use plugins::{PluginManager, PluginManifest};
pub use dependency_graph::{CodeIntel, CodeEntity, EdgeType, BlastRadiusReport, EntityKind};
pub use duality::Oracle;
pub use agent_trait::{Agent, AgentCapability, Task, TaskResult, AgentConfig, BaseAgent};
pub use agent_registry::AgentRegistry;
pub use message_bus::{MessageBus, Message, Subscription};
pub use task_manager::{TaskManager, TaskStatus, TaskManagerStats};
pub use specialized_agents::SpecializedAgentFactory;
pub use multi_agent_kernel::MultiAgentKernel;
pub use task_decomposer::{TaskDecomposer, DecompositionPattern, CapabilityGraph};
pub use agent_coordinator::{AgentCoordinator, SubtaskStatus};
