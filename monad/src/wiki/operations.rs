use super::{WikiManager, WikiArticle};
use std::path::Path;
use chrono::Utc;

pub enum WikiOperation {
    Ingest { document_path: String },
    Query { query: String },
    Compile,
    HealthCheck,
    GenerateArticle { topic: String },
}

impl WikiOperation {
    pub async fn execute(&self, manager: &mut WikiManager) -> Result<String, String> {
        match self {
            WikiOperation::Ingest { document_path } => {
                let path = Path::new(document_path);
                if !path.exists() {
                    return Err(format!("Document not found: {}", document_path));
                }
                
                // Read the document
                let content = tokio::fs::read_to_string(path).await
                    .map_err(|e| format!("Failed to read document: {}", e))?;
                
                // For now, create a simple article
                let title = path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                
                let article = WikiArticle {
                    title: title.clone(),
                    path: path.to_path_buf(),
                    content: content.clone(),
                    summary: format!("Summary of {}", title),
                    tags: vec![],
                    backlinks: vec![],
                    references: vec![],
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };
                
                manager.articles.insert(title.clone(), article.clone());
                
                // Save to wiki directory
                let wiki_path = manager.config.wiki_dir.join(format!("{}.md", title));
                tokio::fs::write(&wiki_path, &article.content).await
                    .map_err(|e| format!("Failed to write wiki article: {}", e))?;
                
                let _ = manager.log_operation(&format!("ingest | {}", title)).await;
                
                // Recompile index automatically after new ingestion
                let index = manager.generate_index()?;
                let index_path = manager.config.wiki_dir.join("index.md");
                let _ = tokio::fs::write(&index_path, &index).await;

                Ok(format!("Ingested document '{}' into wiki. Article saved to {:?}", 
                    document_path, wiki_path))
            }
            
            WikiOperation::Query { query } => {
                // Simple search in article titles and summaries
                let mut results = Vec::new();
                
                for (title, article) in &manager.articles {
                    if title.contains(query) || 
                       article.summary.contains(query) || 
                       article.content.contains(query) {
                        results.push(format!("- [[{}]]: {}", title, article.summary));
                    }
                }
                
                if results.is_empty() {
                    Ok("No matching articles found.".to_string())
                } else {
                    Ok(format!("Query results for '{}':\n\n{}", 
                        query, results.join("\n")))
                }
            }
            
            WikiOperation::Compile => {
                // Generate index and update all articles
                let index = manager.generate_index()?;
                let index_path = manager.config.wiki_dir.join("index.md");
                
                tokio::fs::write(&index_path, &index).await
                    .map_err(|e| format!("Failed to write index: {}", e))?;
                
                Ok(format!("Compiled wiki. Index generated at {:?}\n\nTotal articles: {}", 
                    index_path, manager.articles.len()))
            }
            
            WikiOperation::HealthCheck => {
                let mut issues: Vec<String> = Vec::new();
                
                // Check raw directory
                let raw_docs = manager.scan_raw_documents().await?;
                if raw_docs.is_empty() {
                    issues.push("Raw directory is empty. Add documents to get started.".to_string());
                } else {
                    issues.push(format!("Found {} raw documents.", raw_docs.len()));
                }
                
                // Check wiki articles
                if manager.articles.is_empty() {
                    issues.push("Wiki has no compiled articles.".to_string());
                } else {
                    issues.push(format!("Wiki has {} compiled articles.", manager.articles.len()));
                    
                    // Check for articles with missing files
                    for (title, article) in &manager.articles {
                        if !article.path.exists() {
                            issues.push(format!("Article '{}' references missing file: {:?}", 
                                title, article.path));
                        }
                    }
                }
                
                // Check index file
                let index_path = manager.config.wiki_dir.join("index.md");
                if !index_path.exists() {
                    issues.push("Index file missing. Run 'compile' operation.".to_string());
                }
                
                Ok(format!("Wiki Health Check:\n\n{}", 
                    issues.iter().map(|i| format!("- {}", i)).collect::<Vec<_>>().join("\n")))
            }
            
            WikiOperation::GenerateArticle { topic } => {
                // Create a new article from scratch
                let title = topic.replace(" ", "_").to_lowercase();
                let content = format!("# {}\n\n## Summary\n\nThis article discusses {}.\n\n## Content\n\nAdd your content here.\n\n## References\n\n- Add references here\n\n## Backlinks\n\n- Add related articles here\n\n---\nCreated: {}\nUpdated: {}", 
                    topic, topic, Utc::now(), Utc::now());
                
                let article = WikiArticle {
                    title: topic.clone(),
                    path: manager.config.wiki_dir.join(format!("{}.md", title)),
                    content: content.clone(),
                    summary: format!("Article about {}", topic),
                    tags: vec!["generated".to_string()],
                    backlinks: vec![],
                    references: vec![],
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };
                
                manager.articles.insert(topic.clone(), article.clone());
                
                // Save the article
                tokio::fs::write(&article.path, &article.content).await
                    .map_err(|e| format!("Failed to write generated article: {}", e))?;
                
                let _ = manager.log_operation(&format!("generate | {}", topic)).await;
                
                // Recompile index automatically after new article
                let index = manager.generate_index()?;
                let index_path = manager.config.wiki_dir.join("index.md");
                let _ = tokio::fs::write(&index_path, &index).await;

                Ok(format!("Generated article '{}' at {:?}", topic, article.path))
            }
        }
    }
}
