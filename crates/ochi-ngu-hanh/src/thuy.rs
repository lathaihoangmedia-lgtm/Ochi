//! Thủy Agent - Message Queues & Events (WAL enabled)

use rusqlite::Connection;
use std::path::PathBuf;
use ochi_core::{Result, Error};
use crate::db::open_wal_db;

pub struct ThuyAgent {
    db_path: PathBuf,
    conn: Connection,
}

impl ThuyAgent {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = open_wal_db(&db_path)
            .map_err(|e| Error::Custom(format!("Thuy DB open failed: {}", e)))?;

        let agent = Self { db_path, conn };
        agent.init_schema()?;
        Ok(agent)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                event_type TEXT NOT NULL,
                payload TEXT,
                processed INTEGER DEFAULT 0,
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            CREATE TABLE IF NOT EXISTS message_queue (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                queue_name TEXT NOT NULL,
                message TEXT NOT NULL,
                priority INTEGER DEFAULT 0,
                consumed INTEGER DEFAULT 0,
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            CREATE INDEX IF NOT EXISTS idx_events_type ON events(event_type);
            CREATE INDEX IF NOT EXISTS idx_queue_name ON message_queue(queue_name);
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
