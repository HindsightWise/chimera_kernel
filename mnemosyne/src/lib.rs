#![allow(non_local_definitions)]
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use uuid::Uuid;
use chrono::Utc;

pub mod models; pub mod dnc; pub mod legislator; pub mod storage;
use models::{MemoryEntry, SkillTuple};
use dnc::{DncController, DncObservation};
use legislator::SemanticLegislator;
use storage::StorageController;

#[pyclass]
pub struct MnemosyneEngine {
    dnc: DncController,
    legislator: SemanticLegislator,
    storage: StorageController,
}

#[pymethods]
impl MnemosyneEngine {
    #[new]
    fn new() -> Self {
        Self { 
            dnc: DncController::new(), 
            legislator: SemanticLegislator::new(),
            storage: StorageController::new(),
        }
    }

    fn store(
        &mut self, agent_id: String, text: String, skill_tuple_json: String, 
        dnc_obs_json: String, temperature: f32, embedding_json: Option<String>,
        metadata_json: Option<String>
    ) -> PyResult<String> {
        let skill_tuple: SkillTuple = serde_json::from_str(&skill_tuple_json).unwrap();
        let obs: DncObservation = serde_json::from_str(&dnc_obs_json).unwrap();
        
        let embedding: Option<Vec<f32>> = match embedding_json {
            Some(j) => serde_json::from_str(&j).unwrap_or(None),
            None => None
        };

        // 1. Direct Numeric Control (Grayness Gate)
        if let Err(e) = self.dnc.evaluate(&obs, temperature) {
            return Err(PyValueError::new_err(format!("[DNC REJECTION] {}", e)));
        }
        
        let metadata = match metadata_json {
            Some(j) => serde_json::from_str(&j).unwrap_or(serde_json::json!({})),
            None => serde_json::json!({})
        };

        let entry = MemoryEntry {
            id: Uuid::new_v4(), timestamp: Utc::now(), agent_id, text,
            embedding, kg_node_id: None, skill_tuple,
            metadata, version: 1,
        };

        // 2. Layer 1 Semantic Legislator S={T,O,C} Validation
        if let Err(e) = self.legislator.validate(&entry) {
            return Err(PyValueError::new_err(format!("[CONSTRAINT VIOLATION] {}", e)));
        }

        // 2.5 CRYPTOGRAPHIC VAULT PROXY: Encrypt the text into Occult Runes before saving to disk.
        let seed = std::env::var("GLOSSOPETRAE_MASTER_SEED").unwrap_or_else(|_| "0x309".to_string());
        let dialect = std::env::var("GLOSSOPETRAE_DIALECT").unwrap_or_else(|_| "runic".to_string());
        
        // Mutate the entry directly so the database saves the Ciphertext
        let mut vaulted_entry = entry.clone();
        match glossopetrae::encode_memory_vault(&entry.text, &seed, &dialect) {
            Ok(cipher) => vaulted_entry.text = cipher,
            Err(e) => return Err(PyValueError::new_err(format!("[ENCRYPTION FAILURE] {}", e)))
        }

        // 3. (Production) Execute hybrid Kùzu/LanceDB write using vaulted_entry
        if let Err(e) = self.storage.persist(&vaulted_entry) {
             return Err(PyValueError::new_err(format!("[STORAGE REJECTION] {}", e)));
        }
        Ok(format!("Memory {} validated and committed to Vault.", entry.id))
    }

    fn query_semantic_memory(&mut self, embedding_json: String, limit: usize) -> PyResult<String> {
        let embedding: Vec<f32> = serde_json::from_str(&embedding_json).unwrap_or(vec![]);
        match self.storage.search_vector(embedding, limit) {
            Ok(res) => {
                // CRYPTOGRAPHIC VAULT PROXY: The database returned JSON containing Runes. We must parse and decrypt it back to English.
                let mut entries: Vec<MemoryEntry> = serde_json::from_str(&res).unwrap_or_default();
                let seed = std::env::var("GLOSSOPETRAE_MASTER_SEED").unwrap_or_else(|_| "0x309".to_string());
                let dialect = std::env::var("GLOSSOPETRAE_DIALECT").unwrap_or_else(|_| "runic".to_string());

                for entry in entries.iter_mut() {
                    if let Ok(english_text) = glossopetrae::decode_memory_vault(&entry.text, &seed, &dialect) {
                        entry.text = english_text;
                    }
                }
                
                Ok(serde_json::to_string(&entries).unwrap_or_else(|_| "[]".to_string()))
            },
            Err(e) => Err(PyValueError::new_err(format!("[SEARCH ERROR] {}", e)))
        }
    }

    fn traverse_knowledge_graph(&mut self, cypher: String) -> PyResult<String> {
        match self.storage.traverse_graph(&cypher) {
            Ok(res) => Ok(res),
            Err(e) => Err(PyValueError::new_err(format!("[GRAPH ERROR] {}", e)))
        }
    }
}

#[pymodule]
fn mnemosyne(_py: Python, m: &PyModule) -> PyResult<()> { m.add_class::<MnemosyneEngine>()?; Ok(()) }
