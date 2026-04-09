pub mod schema;
pub mod operations;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiConfig {
    pub name: String,
    pub raw_dir: PathBuf,
    pub wiki_dir: PathBuf,
    pub schema_file: PathBuf,
    pub conventions: HashMap<String, String>,
    pub max_articles: usize,
    pub auto_ingest: bool,
}

impl Default for WikiConfig {
    fn default() -> Self {
        Self {
            name: "ChimeraWiki".to_string(),
            raw_dir: PathBuf::from("raw"),
            wiki_dir: PathBuf::from("wiki"),
            schema_file: PathBuf::from("SCHEMA.md"),
            conventions: HashMap::from([
                ("summary_format".to_string(), "## Summary\n\n".to_string()),
                ("backlink_format".to_string(), "[[{title}]]".to_string()),
                ("index_format".to_string(), "- [[{title}]]: {summary}".to_string()),
            ]),
            max_articles: 100,
            auto_ingest: true,
        }
    }
}

pub struct WikiManager {
    config: WikiConfig,
    articles: HashMap<String, WikiArticle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiArticle {
    pub title: String,
    pub path: PathBuf,
    pub content: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub backlinks: Vec<String>,
    pub references: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl WikiManager {
    pub async fn new(config: WikiConfig) -> Result<Self, String> {
        // Ensure directories exist
        tokio::fs::create_dir_all(&config.raw_dir)
            .await
            .map_err(|e| format!("Failed to create raw directory: {}", e))?;
        tokio::fs::create_dir_all(&config.wiki_dir)
            .await
            .map_err(|e| format!("Failed to create wiki directory: {}", e))?;
        
        Ok(Self {
            config,
            articles: HashMap::new(),
        })
    }
    
    pub async fn scan_raw_documents(&self) -> Result<Vec<PathBuf>, String> {
        let mut documents = Vec::new();
        
        let mut dir = tokio::fs::read_dir(&self.config.raw_dir)
            .await
            .map_err(|e| format!("Failed to read raw directory: {}", e))?;
            
        while let Some(entry) = dir.next_entry().await.map_err(|e| format!("Failed to read entry: {}", e))? {
            let path = entry.path();
            
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" || ext == "txt" || ext == "pdf" || ext == "html" {
                        documents.push(path);
                    }
                }
            }
        }
        
        Ok(documents)
    }
    
    pub fn generate_index(&self) -> Result<String, String> {
        let mut index = String::new();
        index.push_str("# Wiki Index\n\n");
        
        for (title, article) in &self.articles {
            index.push_str(&format!("- [[{}]]: {}\n", title, article.summary));
        }
        
        Ok(index)
    }
}

pub fn get_wiki_tool() -> async_openai::types::ChatCompletionTool {
    use async_openai::types::{ChatCompletionTool, FunctionObject};
    use serde_json::json;
    
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "compile_wiki".to_string(),
            description: Some("Compile raw documents into a structured wiki. This implements the Karpathy LLM-wiki pattern: the LLM acts as a compiler that reads raw sources and produces a structured, interlinked wiki. Use this to build knowledge bases that compound over time.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "operation": {
                        "type": "string",
                        "enum": ["ingest", "query", "compile", "health_check", "generate_article"],
                        "description": "The wiki operation to perform"
                    },
                    "document_path": {
                        "type": "string",
                        "description": "Path to raw document for ingestion"
                    },
                    "query": {
                        "type": "string", 
                        "description": "Query to search in wiki"
                    },
                    "topic": {
                        "type": "string",
                        "description": "Topic for article generation"
                    }
                },
                "required": ["operation"]
            })),
        },
    }
}
