//! Thủy Agent - Events, Message Flow & Sessions
//! 
//! Responsibilities:
//! - Event stream logging (short-term history)
//! - Message queue management (async tasks)
//! - User session & interaction context
//! - Flow control & communication state

use rusqlite::{Connection, params};
use std::path::PathBuf;
use ochi_core::{Result, Error};
use crate::db::open_wal_db;

/// Thủy Agent - Event & Communication Agent
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
            -- Event logs (recent activity stream)
            CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                event_type TEXT NOT NULL,
                payload TEXT_JSON,
                source TEXT,
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- Async Task Queue
            CREATE TABLE IF NOT EXISTS message_queue (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                queue_name TEXT NOT NULL,
                message TEXT NOT NULL,
                priority INTEGER DEFAULT 0,
                status TEXT DEFAULT 'pending', -- pending, processing, completed, failed
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- Conversation Sessions
            CREATE TABLE IF NOT EXISTS sessions (
                session_id TEXT PRIMARY KEY,
                user_id TEXT,
                context_json TEXT, -- Active context window metadata
                last_active TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- Message history (Session level)
            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL,
                role TEXT NOT NULL, -- user, assistant, system
                content TEXT NOT NULL,
                tokens INTEGER,
                created_at TEXT DEFAULT (datetime('now')),
                FOREIGN KEY(session_id) REFERENCES sessions(session_id)
            ) STRICT;

            CREATE INDEX IF NOT EXISTS idx_events_type ON events(event_type);
            CREATE INDEX IF NOT EXISTS idx_sess_msg ON messages(session_id);
            CREATE INDEX IF NOT EXISTS idx_queue_status ON message_queue(status);
        ")?;
        Ok(())
    }

    // === Event Operations ===

    pub fn log_event(&self, event_type: &str, payload: &str, source: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO events (event_type, payload, source) VALUES (?1, ?2, ?3)",
            params![event_type, payload, source],
        )?;
        Ok(())
    }

    // === Session Operations ===

    pub fn save_message(&self, session_id: &str, role: &str, content: &str, tokens: i32) -> Result<()> {
        self.conn.execute(
            "INSERT INTO messages (session_id, role, content, tokens) VALUES (?1, ?2, ?3, ?4)",
            params![session_id, role, content, tokens],
        )?;
        Ok(())
    }

    pub fn get_session_history(&self, session_id: &str, limit: i32) -> Result<Vec<(String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT role, content FROM messages WHERE session_id = ?1 ORDER BY created_at DESC LIMIT ?2"
        )?;
        let rows = stmt.query_map([session_id, &limit.to_string()], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;
        let mut history: Vec<_> = rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::Custom(e.to_string()))?;
        history.reverse();
        Ok(history)
    }

    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }

    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}

