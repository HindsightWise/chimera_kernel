use rusqlite::{params, Connection, Result};
use std::sync::Arc;
use std::sync::Mutex;
use chrono::Utc;

pub struct GraphMemoryManager {
    db: Arc<Mutex<Connection>>,
}

impl GraphMemoryManager {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Initialize SQLite Graph tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS entities (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                type TEXT NOT NULL,
                last_seen TEXT NOT NULL
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS relationships (
                source_id TEXT NOT NULL,
                target_id TEXT NOT NULL,
                relation_type TEXT NOT NULL,
                weight REAL DEFAULT 1.0,
                last_reinforced TEXT NOT NULL,
                PRIMARY KEY (source_id, target_id, relation_type),
                FOREIGN KEY (source_id) REFERENCES entities(id),
                FOREIGN KEY (target_id) REFERENCES entities(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS chronos_queue (
                id TEXT PRIMARY KEY,
                execute_at INTEGER NOT NULL,
                payload TEXT NOT NULL,
                topic TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
        })
    }

    pub async fn upsert_entity(&self, name: &str, entity_type: &str) -> Result<String> {
        let name = name.to_string();
        let entity_type = entity_type.to_string();
        let db = self.db.clone();
        
        tokio::task::spawn_blocking(move || {
            let conn = db.lock().unwrap();
            let id_uuid = uuid::Uuid::new_v4().to_string();
            let now = Utc::now().to_rfc3339();
            
            conn.execute(
                "INSERT INTO entities (id, name, type, last_seen)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(name) DO UPDATE SET last_seen=excluded.last_seen",
                params![id_uuid, name, entity_type, now],
            )?;
            
            // Fetch the ID (either newly inserted or existing)
            let mut stmt = conn.prepare("SELECT id FROM entities WHERE name = ?1")?;
            let actual_id: String = stmt.query_row(params![name], |row| row.get(0))?;
            
            Ok(actual_id)
        }).await.unwrap()
    }

    pub async fn upsert_relationship(&self, source_id: &str, target_id: &str, relation_type: &str) -> Result<()> {
        let source_id = source_id.to_string();
        let target_id = target_id.to_string();
        let relation_type = relation_type.to_string();
        let db = self.db.clone();
        
        tokio::task::spawn_blocking(move || {
            let conn = db.lock().unwrap();
            let now = Utc::now().to_rfc3339();
            
            conn.execute(
                "INSERT INTO relationships (source_id, target_id, relation_type, weight, last_reinforced)
                 VALUES (?1, ?2, ?3, 1.0, ?4)
                 ON CONFLICT(source_id, target_id, relation_type) 
                 DO UPDATE SET weight = weight + 0.1, last_reinforced=excluded.last_reinforced",
                params![source_id, target_id, relation_type, now],
            )?;
            
            Ok(())
        }).await.unwrap()
    }

    pub async fn prune_synapses(&self, weight_threshold: f64) -> Result<usize> {
        let db = self.db.clone();
        
        tokio::task::spawn_blocking(move || {
            let conn = db.lock().unwrap();
            let pruned = conn.execute(
                "DELETE FROM relationships WHERE weight < ?1 AND date(last_reinforced) < date('now', '-30 days')",
                params![weight_threshold],
            )?;
            
            // Remove orphan entities
            conn.execute(
                "DELETE FROM entities WHERE id NOT IN (SELECT source_id FROM relationships) AND id NOT IN (SELECT target_id FROM relationships)",
                [],
            )?;
            
            Ok(pruned)
        }).await.unwrap()
    }

    pub async fn insert_chronos_task(&self, execute_at: i64, payload: &str, topic: &str) -> Result<()> {
        let payload = payload.to_string();
        let topic = topic.to_string();
        let db = self.db.clone();
        
        tokio::task::spawn_blocking(move || {
            let conn = db.lock().unwrap();
            let id_uuid = uuid::Uuid::new_v4().to_string();
            conn.execute(
                "INSERT INTO chronos_queue (id, execute_at, payload, topic) VALUES (?1, ?2, ?3, ?4)",
                params![id_uuid, execute_at, payload, topic],
            )?;
            Ok(())
        }).await.unwrap()
    }

    pub async fn poll_chronos_tasks(&self, current_unix: i64) -> Result<Vec<(String, String, String)>> {
        let db = self.db.clone();
        
        tokio::task::spawn_blocking(move || {
            let conn = db.lock().unwrap();
            let mut stmt = conn.prepare("SELECT id, payload, topic FROM chronos_queue WHERE execute_at <= ?1")?;
            
            let pending = stmt.query_map(params![current_unix], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?
                ))
            })?;
            
            let mut tasks = Vec::new();
            for task in pending {
                if let Ok(t) = task {
                    tasks.push(t);
                }
            }
            
            for (ref id, _, _) in &tasks {
                conn.execute("DELETE FROM chronos_queue WHERE id = ?1", params![id])?;
            }
            
            Ok(tasks)
        }).await.unwrap()
    }
}
