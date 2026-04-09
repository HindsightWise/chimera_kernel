use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiSchema {
    pub name: String,
    pub version: String,
    pub conventions: Conventions,
    pub workflows: Vec<Workflow>,
    pub entity_types: HashMap<String, EntityDefinition>,
    pub relationship_types: Vec<RelationshipType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conventions {
    pub file_naming: String,
    pub header_levels: HashMap<String, String>,
    pub summary_format: String,
    pub backlink_format: String,
    pub index_format: String,
    pub logging_format: String,
    pub visual_outputs: Vec<String>, // e.g., "markdown", "marp_slides", "matplotlib"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    pub triggers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub action: String,
    pub target: String,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDefinition {
    pub name: String,
    pub template: String,
    pub required_fields: Vec<String>,
    pub optional_fields: Vec<String>,
    pub example: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipType {
    pub name: String,
    pub description: String,
    pub notation: String,
}

impl Default for WikiSchema {
    fn default() -> Self {
        Self {
            name: "ChimeraWikiSchema".to_string(),
            version: "1.0.0".to_string(),
            conventions: Conventions {
                file_naming: "{title}.md".to_string(),
                header_levels: HashMap::from([
                    ("article".to_string(), "#".to_string()),
                    ("section".to_string(), "##".to_string()),
                    ("subsection".to_string(), "###".to_string()),
                ]),
                summary_format: "## Summary\n\n{content}\n".to_string(),
                backlink_format: "[[{title}]]".to_string(),
                index_format: "- [[{title}]]: {summary}".to_string(),
                logging_format: "[{timestamp}] {operation}: {details}".to_string(),
                visual_outputs: vec!["markdown".to_string(), "marp_slides".to_string()],
            },
            workflows: vec![
                Workflow {
                    name: "ingest_source".to_string(),
                    description: "Process a raw source document and integrate it into the wiki".to_string(),
                    steps: vec![
                        WorkflowStep {
                            action: "read".to_string(),
                            target: "raw_document".to_string(),
                            parameters: HashMap::new(),
                        },
                        WorkflowStep {
                            action: "extract_key_takeaways".to_string(),
                            target: "content".to_string(),
                            parameters: HashMap::new(),
                        },
                        WorkflowStep {
                            action: "write_summary_page".to_string(),
                            target: "wiki".to_string(),
                            parameters: HashMap::from([("format".to_string(), "article".to_string())]),
                        },
                        WorkflowStep {
                            action: "update_index".to_string(),
                            target: "index".to_string(),
                            parameters: HashMap::new(),
                        },
                        WorkflowStep {
                            action: "update_entities".to_string(),
                            target: "related_articles".to_string(),
                            parameters: HashMap::new(),
                        },
                        WorkflowStep {
                            action: "append_log".to_string(),
                            target: "log".to_string(),
                            parameters: HashMap::new(),
                        },
                    ],
                    triggers: vec!["new_raw_file".to_string()],
                },
                Workflow {
                    name: "answer_query".to_string(),
                    description: "Answer a query using the compiled wiki knowledge".to_string(),
                    steps: vec![
                        WorkflowStep {
                            action: "parse_query".to_string(),
                            target: "query".to_string(),
                            parameters: HashMap::new(),
                        },
                        WorkflowStep {
                            action: "search_wiki".to_string(),
                            target: "wiki".to_string(),
                            parameters: HashMap::new(),
                        },
                        WorkflowStep {
                            action: "synthesize_answer".to_string(),
                            target: "answer".to_string(),
                            parameters: HashMap::new(),
                        },
                        WorkflowStep {
                            action: "generate_output".to_string(),
                            target: "output".to_string(),
                            parameters: HashMap::from([("format".to_string(), "markdown".to_string())]),
                        },
                    ],
                    triggers: vec!["user_query".to_string()],
                },
            ],
            entity_types: HashMap::from([
                ("article".to_string(), EntityDefinition {
                    name: "article".to_string(),
                    template: "# {title}\n\n## Summary\n\n{summary}\n\n## Content\n\n{content}\n\n## References\n\n{references}\n\n## Backlinks\n\n{backlinks}\n\n---\nCreated: {created_at}\nUpdated: {updated_at}".to_string(),
                    required_fields: vec!["title".to_string(), "summary".to_string(), "content".to_string()],
                    optional_fields: vec!["references".to_string(), "backlinks".to_string(), "tags".to_string()],
                    example: "# Neural Networks\n\n## Summary\n\nNeural networks are computational models inspired by biological neural networks...".to_string(),
                }),
                ("concept".to_string(), EntityDefinition {
                    name: "concept".to_string(),
                    template: "# {name}\n\n## Definition\n\n{definition}\n\n## Related Concepts\n\n{related_concepts}\n\n## Examples\n\n{examples}\n\n## Applications\n\n{applications}".to_string(),
                    required_fields: vec!["name".to_string(), "definition".to_string()],
                    optional_fields: vec!["related_concepts".to_string(), "examples".to_string(), "applications".to_string()],
                    example: "# Backpropagation\n\n## Definition\n\nBackpropagation is an algorithm for training neural networks...".to_string(),
                }),
            ]),
            relationship_types: vec![
                RelationshipType {
                    name: "references".to_string(),
                    description: "One article references another".to_string(),
                    notation: "A -> B".to_string(),
                },
                RelationshipType {
                    name: "builds_on".to_string(),
                    description: "One concept builds on another".to_string(),
                    notation: "A ⮕ B".to_string(),
                },
                RelationshipType {
                    name: "contradicts".to_string(),
                    description: "One statement contradicts another".to_string(),
                    notation: "A ⮔ B".to_string(),
                },
            ],
        }
    }
}
