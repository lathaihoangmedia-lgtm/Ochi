//! Kim agent - rules, permissions, and security logic.

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct KimAgent {
    db_path: PathBuf,
}

impl KimAgent {
    pub fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }
}
