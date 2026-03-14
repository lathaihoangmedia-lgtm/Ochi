//! SQLite database utilities for Ngũ Hành.
//! 
//! All databases use WAL (Write-Ahead Logging) mode for concurrent access.

use rusqlite::{Connection, OpenFlags, Result as SqliteResult};
use std::path::{Path, PathBuf};
use ochi_core::Result;

/// Common WAL database operations for all agents
pub trait WalDatabase {
    fn enable_wal(&self) -> SqliteResult<()>;
    fn set_busy_timeout(&self, ms: u32) -> SqliteResult<()>;
    fn checkpoint(&self) -> SqliteResult<()>;
}

impl WalDatabase for Connection {
    fn enable_wal(&self) -> SqliteResult<()> {
        self.execute_batch("PRAGMA journal_mode=WAL;")
    }

    fn set_busy_timeout(&self, ms: u32) -> SqliteResult<()> {
        self.execute_batch(&format!("PRAGMA busy_timeout={};", ms))
    }

    fn checkpoint(&self) -> SqliteResult<()> {
        self.execute_batch("PRAGMA wal_checkpoint(PASSIVE);")
    }
}

/// Open SQLite with WAL mode enabled for concurrent read/write
pub fn open_wal_db<P: AsRef<Path>>(path: P) -> SqliteResult<Connection> {
    let conn = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    )?;
    
    // WAL configuration for concurrency
    conn.enable_wal()?;
    conn.set_busy_timeout(5000)?;
    conn.execute_batch("
        PRAGMA synchronous=NORMAL;
        PRAGMA wal_autocheckpoint=1000;
        PRAGMA cache_size=-64000;
        PRAGMA temp_store=MEMORY;
    ")?;
    
    Ok(conn)
}

/// Database paths for the five elements.
#[derive(Debug, Clone)]
pub struct DatabaseManager {
    pub kim: PathBuf,
    pub moc: PathBuf,
    pub thuy: PathBuf,
    pub hoa: PathBuf,
    pub tho: PathBuf,
}

impl DatabaseManager {
    /// Create DB paths under the provided data directory.
    pub fn new<P: AsRef<Path>>(data_dir: P) -> Self {
        let base = data_dir.as_ref();
        Self {
            kim: base.join("kim.db"),
            moc: base.join("moc.db"),
            thuy: base.join("thuy.db"),
            hoa: base.join("hoa.db"),
            tho: base.join("tho.db"),
        }
    }

    /// Ensure the data directory exists.
    pub fn ensure_data_dir<P: AsRef<Path>>(data_dir: P) -> Result<()> {
        std::fs::create_dir_all(data_dir)?;
        Ok(())
    }
}
