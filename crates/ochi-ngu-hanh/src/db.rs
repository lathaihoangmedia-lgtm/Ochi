//! SQLite database layout for Ngũ Hành.

use std::path::{Path, PathBuf};
use ochi_core::Result;

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
