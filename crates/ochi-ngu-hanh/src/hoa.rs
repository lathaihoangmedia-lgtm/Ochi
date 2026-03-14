//! Hỏa agent - hot cache and realtime alerts.

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct HoaAgent {
    db_path: PathBuf,
}

impl HoaAgent {
    pub fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }
}
