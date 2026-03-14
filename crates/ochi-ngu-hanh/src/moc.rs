//! Mộc Agent - Knowledge & Embeddings (WAL enabled)

use rusqlite::Connection;
use std::path::PathBuf;
use ochi_core::{Result, Error};
use crate::db::open_wal_db;

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
            CREATE TABLE IF NOT EXISTS knowledge (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                category TEXT,
                key TEXT NOT NULL,
                value TEXT,
                embedding TEXT,
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            CREATE TABLE IF NOT EXISTS embeddings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                entity_type TEXT,
                entity_id TEXT,
                vector TEXT,
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            CREATE INDEX IF NOT EXISTS idx_knowledge_category ON knowledge(category);
        ")?;
        Ok(())
    }

    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }

    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}
