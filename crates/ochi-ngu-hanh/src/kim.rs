//! Kim Agent - Rules, Analysis & Predictions
//! 
//! Responsibilities:
//! - Rule engine storage (validation patterns)
//! - Permissions & Access Control
//! - Analytical predictions (inference history)
//! - Static structure & standard definitions

use rusqlite::{Connection, params};
use std::path::PathBuf;
use ochi_core::{Result, Error};
use crate::db::open_wal_db;

/// Kim Agent - Analytical & Structural Agent
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
            -- Rules for data validation or agent behavior
            CREATE TABLE IF NOT EXISTS rules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL,
                pattern TEXT NOT NULL,
                action TEXT NOT NULL,
                priority INTEGER DEFAULT 0,
                active INTEGER DEFAULT 1,
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- Granular permissions
            CREATE TABLE IF NOT EXISTS permissions (
                role TEXT,
                resource TEXT,
                action TEXT,
                granted INTEGER DEFAULT 1,
                PRIMARY KEY (role, resource, action)
            ) STRICT;

            -- Predictions and analytical results
            CREATE TABLE IF NOT EXISTS predictions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                input_hash TEXT NOT NULL,
                model_name TEXT NOT NULL,
                result TEXT_JSON NOT NULL,
                confidence REAL,
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            CREATE INDEX IF NOT EXISTS idx_rules_active ON rules(active);
            CREATE INDEX IF NOT EXISTS idx_pred_hash ON predictions(input_hash);
        ")?;
        Ok(())
    }

    // === Rule Operations ===

    pub fn add_rule(&self, name: &str, pattern: &str, action: &str, priority: i32) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO rules (name, pattern, action, priority) VALUES (?1, ?2, ?3, ?4)",
            params![name, pattern, action, priority],
        )?;
        Ok(())
    }

    pub fn get_active_rules(&self) -> Result<Vec<(String, String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, pattern, action FROM rules WHERE active = 1 ORDER BY priority DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        rows.collect::<std::result::Result<Vec<_>, _>>().map_err(|e| Error::Custom(e.to_string()))
    }

    // === Prediction Storage ===

    pub fn store_prediction(&self, hash: &str, model: &str, result: &str, confidence: f64) -> Result<()> {
        self.conn.execute(
            "INSERT INTO predictions (input_hash, model_name, result, confidence) VALUES (?1, ?2, ?3, ?4)",
            params![hash, model, result, confidence],
        )?;
        Ok(())
    }

    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }

    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}

