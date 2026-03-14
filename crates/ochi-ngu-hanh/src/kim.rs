//! Kim Agent - Rules & Permissions (WAL enabled)

use rusqlite::Connection;
use std::path::PathBuf;
use ochi_core::{Result, Error};
use crate::db::open_wal_db;

pub struct KimAgent {
    db_path: PathBuf,
    conn: Connection,
}

impl KimAgent {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = open_wal_db(&db_path)
            .map_err(|e| Error::Custom(format!("Kim DB open failed: {}", e)))?;

        let agent = Self { db_path, conn };
        agent.init_schema()?;
        Ok(agent)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS rules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL,
                pattern TEXT NOT NULL,
                action TEXT NOT NULL,
                priority INTEGER DEFAULT 0,
                active INTEGER DEFAULT 1
            ) STRICT;

            CREATE TABLE IF NOT EXISTS permissions (
                role TEXT,
                resource TEXT,
                action TEXT,
                granted INTEGER DEFAULT 1,
                PRIMARY KEY (role, resource, action)
            ) STRICT;

            CREATE INDEX IF NOT EXISTS idx_rules_active ON rules(active);
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
