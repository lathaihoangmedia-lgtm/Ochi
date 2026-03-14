//! Hỏa Agent - Hot Cache, Alerts & Performance
//! 
//! Responsibilities:
//! - Ephemeral cache storage (TTL-based)
//! - Real-time alerts & triggers
//! - Performance metrics & statistics
//! - Transformation logs (Active state)

use rusqlite::{Connection, params};
use std::path::PathBuf;
use ochi_core::{Result, Error};
use crate::db::open_wal_db;

/// Hỏa Agent - Performance & Active State Agent
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
            -- Ephemeral Hot Cache
            CREATE TABLE IF NOT EXISTS cache (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                ttl_seconds INTEGER DEFAULT 3600,
                expires_at INTEGER NOT NULL, -- Unix timestamp
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- Real-time Alerts
            CREATE TABLE IF NOT EXISTS alerts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                alert_type TEXT NOT NULL,
                severity TEXT CHECK(severity IN ('info', 'warning', 'critical')),
                message TEXT NOT NULL,
                acknowledged INTEGER DEFAULT 0,
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- Active Performance Stats
            CREATE TABLE IF NOT EXISTS hot_stats (
                metric_name TEXT PRIMARY KEY,
                metric_value REAL NOT NULL,
                updated_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            CREATE INDEX IF NOT EXISTS idx_cache_expiry ON cache(expires_at);
            CREATE INDEX IF NOT EXISTS idx_alerts_ack ON alerts(acknowledged);
        ")?;
        Ok(())
    }

    // === Cache Operations ===

    pub fn set_cache(&self, key: &str, value: &str, ttl_secs: i64) -> Result<()> {
        let expires_at = chrono::Utc::now().timestamp() + ttl_secs;
        self.conn.execute(
            "INSERT OR REPLACE INTO cache (key, value, ttl_seconds, expires_at) VALUES (?1, ?2, ?3, ?4)",
            params![key, value, ttl_secs, expires_at],
        )?;
        Ok(())
    }

    pub fn get_cache(&self, key: &str) -> Result<Option<String>> {
        let now = chrono::Utc::now().timestamp();
        let mut stmt = self.conn.prepare(
            "SELECT value FROM cache WHERE key = ?1 AND expires_at > ?2"
        )?;
        let result = stmt.query_row(params![key, now], |row| row.get(0));
        match result {
            Ok(val) => Ok(Some(val)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Error::Custom(e.to_string())),
        }
    }

    pub fn cleanup_cache(&self) -> Result<usize> {
        let now = chrono::Utc::now().timestamp();
        let count = self.conn.execute(
            "DELETE FROM cache WHERE expires_at <= ?1",
            params![now],
        )?;
        Ok(count)
    }

    // === Alert Operations ===

    pub fn raise_alert(&self, alert_type: &str, severity: &str, message: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO alerts (alert_type, severity, message) VALUES (?1, ?2, ?3)",
            params![alert_type, severity, message],
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

