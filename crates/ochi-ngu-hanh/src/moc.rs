//! Mộc Agent - Knowledge Graph, Growth & Entities
//! 
//! Responsibilities:
//! - Knowledge base storage (RAG-ready)
//! - Entity relationship mapping
//! - Embedding vectors (metadata track)
//! - Long-term associative memory

use rusqlite::{Connection, params};
use std::path::PathBuf;
use ochi_core::{Result, Error};
use crate::db::open_wal_db;

/// Mộc Agent - Knowledge & Growth Agent
pub struct MocAgent {
    db_path: PathBuf,
    conn: Connection,
}

impl MocAgent {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = open_wal_db(&db_path)
            .map_err(|e| Error::Custom(format!("Moc DB open failed: {}", e)))?;

        let agent = Self { db_path, conn };
        agent.init_schema()?;
        Ok(agent)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch("
            -- Knowledge nuggets
            CREATE TABLE IF NOT EXISTS knowledge (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                category TEXT NOT NULL,
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                context TEXT,
                tags TEXT, -- Comma-separated or JSON
                created_at TEXT DEFAULT (datetime('now')),
                UNIQUE(category, key)
            ) STRICT;

            -- Semantic embeddings (vector metadata)
            CREATE TABLE IF NOT EXISTS embeddings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                vector_json TEXT NOT NULL, -- Stored as JSON array for external vector search sync
                model TEXT NOT NULL,
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- Entity Graph (Growth track)
            CREATE TABLE IF NOT EXISTS entities (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL,
                entity_type TEXT NOT NULL,
                properties TEXT_JSON,
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            CREATE TABLE IF NOT EXISTS relationships (
                source_id INTEGER,
                target_id INTEGER,
                rel_type TEXT NOT NULL,
                weight REAL DEFAULT 1.0,
                PRIMARY KEY (source_id, target_id, rel_type),
                FOREIGN KEY(source_id) REFERENCES entities(id),
                FOREIGN KEY(target_id) REFERENCES entities(id)
            ) STRICT;

            CREATE INDEX IF NOT EXISTS idx_knowledge_cat ON knowledge(category);
            CREATE INDEX IF NOT EXISTS idx_ent_type ON entities(entity_type);
        ")?;
        Ok(())
    }

    // === Knowledge Operations ===

    pub fn learn(&self, category: &str, key: &str, value: &str, tags: Option<&str>) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO knowledge (category, key, value, tags) VALUES (?1, ?2, ?3, ?4)",
            params![category, key, value, tags.unwrap_or("")],
        )?;
        Ok(())
    }

    pub fn recall(&self, category: &str, key: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT value FROM knowledge WHERE category = ?1 AND key = ?2"
        )?;
        let result = stmt.query_row([category, key], |row| row.get(0));
        match result {
            Ok(val) => Ok(Some(val)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Error::Custom(e.to_string())),
        }
    }

    // === Entity Graph ===

    pub fn add_entity(&self, name: &str, entity_type: &str, properties: &str) -> Result<i64> {
        self.conn.execute(
            "INSERT OR IGNORE INTO entities (name, entity_type, properties) VALUES (?1, ?2, ?3)",
            params![name, entity_type, properties],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }

    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}

