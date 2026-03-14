//! Thủy agent - message queues and data flow.

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ThuyAgent {
    db_path: PathBuf,
}

impl ThuyAgent {
    pub fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }
}
