//! Thổ agent - core storage, metadata, and stability.

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ThoAgent {
    db_path: PathBuf,
}

impl ThoAgent {
    pub fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }
}
