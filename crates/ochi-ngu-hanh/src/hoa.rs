//! Hỏa Agent - Hot Cache & Alerts (WAL enabled)

use rusqlite::Connection;
use std::path::PathBuf;
use ochi_core::{Result, Error};
use crate::db::open_wal_db;

pub struct HoaAgent {
    db_path: PathBuf,
    conn: Connection,
}

impl HoaAgent {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = open_wal_db(&db_path)
            .map_err(|e| Error::Custom(format!("Hoa DB open failed: {}", e)))?;

        let agent = Self { db_path, conn };
        agent.init_schema()?;
        Ok(agent)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS cache (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                expires_at TEXT,
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            CREATE TABLE IF NOT EXISTS alerts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                alert_type TEXT NOT NULL,
                severity TEXT,
                message TEXT,
                acknowledged INTEGER DEFAULT 0,
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            CREATE INDEX IF NOT EXISTS idx_cache_expires ON cache(expires_at);
            CREATE INDEX IF NOT EXISTS idx_alerts_type ON alerts(alert_type);
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
