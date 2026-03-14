//! Mộc agent - knowledge, embeddings, and learning logic.

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct MocAgent {
    db_path: PathBuf,
}

impl MocAgent {
    pub fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }
}
