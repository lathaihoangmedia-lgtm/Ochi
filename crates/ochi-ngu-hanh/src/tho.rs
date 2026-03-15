//! Thổ Agent - Full Stack Memory System + Cửu Cung 3 Số (2-5-8)
//!
//! Responsibilities:
//! - System configuration (ochi.toml parsing)
//! - Router metadata (bat_quai → DB mappings)
//! - Cửu Cung mappings (positions 2, 5, 8 - center column)
//! - Health checks & audit logs
//! - Schema migrations (learning track)
//! - **Memory System** (ZeroClaw-style):
//!   - Vector embeddings (cosine similarity search)
//!   - FTS5 full-text search (BM25 scoring)
//!   - Hybrid merge (vector + keyword)
//!   - Embedding cache with LRU eviction
//!   - Safe reindex support

use rusqlite::{Connection, params};
use std::path::PathBuf;
use ochi_core::{Result, Error};
use crate::db::open_wal_db;

/// Cửu Cung positions that Thổ controls (2-5-8: center column)
const THO_CUU_CUNG_POSITIONS: [i32; 3] = [2, 5, 8];

/// Thổ Agent - Full Stack with WAL
pub struct ThoAgent {
    db_path: PathBuf,
    conn: Connection,
}

impl ThoAgent {
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = open_wal_db(&db_path)
            .map_err(|e| Error::Custom(format!("Tho DB open failed: {}", e)))?;

        let agent = Self { db_path, conn };
        agent.init_schema()?;
        agent.seed_cuu_cung()?;
        Ok(agent)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch("
            -- System configuration
            CREATE TABLE IF NOT EXISTS system_config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- Router metadata (bat_quai mappings)
            CREATE TABLE IF NOT EXISTS router_metadata (
                bat_quai TEXT PRIMARY KEY,
                db_mapping TEXT NOT NULL,
                checkpoint_number INTEGER NOT NULL,
                ngu_hanh TEXT NOT NULL,
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- Cửu Cung mappings (Thổ controls 2-5-8)
            CREATE TABLE IF NOT EXISTS cuu_cung (
                position INTEGER PRIMARY KEY,
                bat_quai TEXT NOT NULL,
                ngu_hanh TEXT NOT NULL,
                description TEXT,
                active INTEGER DEFAULT 1,
                updated_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- Health checks
            CREATE TABLE IF NOT EXISTS health_checks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                component TEXT NOT NULL,
                status TEXT NOT NULL,
                message TEXT,
                checked_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- Audit logs (learning track)
            CREATE TABLE IF NOT EXISTS audit_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                action TEXT NOT NULL,
                entity TEXT,
                old_value TEXT,
                new_value TEXT,
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- Schema migrations (learning track)
            CREATE TABLE IF NOT EXISTS schema_migrations (
                version TEXT PRIMARY KEY,
                description TEXT,
                applied_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- === MEMORY SYSTEM (ZeroClaw-style) ===
            
            -- Memory chunks (documents)
            CREATE TABLE IF NOT EXISTS memory_chunks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content TEXT NOT NULL,
                source TEXT,
                chunk_index INTEGER,
                metadata TEXT, -- JSON metadata
                created_at TEXT DEFAULT (datetime('now')),
                updated_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- Vector embeddings (stored as JSON array for portability)
            CREATE TABLE IF NOT EXISTS embeddings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                chunk_id INTEGER NOT NULL,
                embedding_type TEXT NOT NULL, -- 'openai', 'custom', 'noop'
                vector_json TEXT NOT NULL, -- JSON array [0.1, 0.2, ...]
                dimensions INTEGER NOT NULL,
                created_at TEXT DEFAULT (datetime('now')),
                FOREIGN KEY (chunk_id) REFERENCES memory_chunks(id) ON DELETE CASCADE
            ) STRICT;

            -- FTS5 virtual table for full-text search (BM25)
            CREATE VIRTUAL TABLE IF NOT EXISTS memory_fts USING fts5(
                content,
                source,
                metadata,
                content='memory_chunks',
                content_rowid='id'
            );

            -- Triggers to keep FTS5 in sync
            CREATE TRIGGER IF NOT EXISTS memory_chunks_ai AFTER INSERT ON memory_chunks BEGIN
                INSERT INTO memory_fts(rowid, content, source, metadata)
                VALUES (new.id, new.content, new.source, new.metadata);
            END;

            CREATE TRIGGER IF NOT EXISTS memory_chunks_ad AFTER DELETE ON memory_chunks BEGIN
                INSERT INTO memory_fts(memory_fts, rowid, content, source, metadata)
                VALUES ('delete', old.id, old.content, old.source, old.metadata);
            END;

            CREATE TRIGGER IF NOT EXISTS memory_chunks_au AFTER UPDATE ON memory_chunks BEGIN
                INSERT INTO memory_fts(memory_fts, rowid, content, source, metadata)
                VALUES ('delete', old.id, old.content, old.source, old.metadata);
                INSERT INTO memory_fts(rowid, content, source, metadata)
                VALUES (new.id, new.content, new.source, new.metadata);
            END;

            -- Embedding cache with LRU eviction
            CREATE TABLE IF NOT EXISTS embedding_cache (
                key TEXT PRIMARY KEY, -- hash of input text
                embedding_json TEXT NOT NULL,
                dimensions INTEGER NOT NULL,
                hits INTEGER DEFAULT 0,
                last_accessed TEXT DEFAULT (datetime('now')),
                created_at TEXT DEFAULT (datetime('now'))
            ) STRICT;

            -- Indexes for performance
            CREATE INDEX IF NOT EXISTS idx_health_component
                ON health_checks(component);
            CREATE INDEX IF NOT EXISTS idx_audit_action
                ON audit_logs(action);
            CREATE INDEX IF NOT EXISTS idx_cuu_cung_active
                ON cuu_cung(active);
            CREATE INDEX IF NOT EXISTS idx_embeddings_chunk
                ON embeddings(chunk_id);
            CREATE INDEX IF NOT EXISTS idx_embeddings_type
                ON embeddings(embedding_type);
            CREATE INDEX IF NOT EXISTS idx_cache_accessed
                ON embedding_cache(last_accessed);
        ")?;

        self.seed_bat_quai()?;
        Ok(())
    }

    fn seed_cuu_cung(&self) -> Result<()> {
        // Thổ controls positions 2, 5, 8 (center column)
        let cuu_cung_data = [
            (1, "Can", "Moc", "Khởi đầu, sinh trưởng"),
            (2, "Khon", "Tho", "Đất, tiếp nhận - THỔ"),
            (3, "Chan", "Moc", "Sấm, chấn động"),
            (4, "Ton", "Moc", "Gió, xâm nhập"),
            (5, "Trung Cung", "Tho", "Trung tâm - THỔ"),
            (6, "Can", "Kim", "Trời, sáng tạo"),
            (7, "Doai", "Kim", "Hồ, vui vẻ"),
            (8, "Con", "Tho", "Núi, dừng lại - THỔ"),
            (9, "Ly", "Hoa", "Lửa, tỏa sáng"),
        ];

        for (pos, bat_quai, ngu_hanh, desc) in cuu_cung_data {
            self.conn.execute(
                "INSERT OR IGNORE INTO cuu_cung 
                 (position, bat_quai, ngu_hanh, description) 
                 VALUES (?1, ?2, ?3, ?4)",
                params![pos, bat_quai, ngu_hanh, desc],
            )?;
        }

        Ok(())
    }

    fn seed_bat_quai(&self) -> Result<()> {
        let bat_quai_mappings = [
            ("Can", "moc.db", 1, "Moc"),
            ("Can Gua", "thuy.db", 2, "Thuy"),
            ("Chan", "hoa.db", 3, "Hoa"),
            ("Ton", "kim.db", 4, "Kim"),
            ("Kham", "thuy.db", 5, "Thuy"),
            ("Ly", "hoa.db", 6, "Hoa"),
            ("Con", "tho.db", 7, "Tho"),
            ("Doai", "moc.db", 8, "Moc"),
        ];

        for (bat_quai, db, checkpoint, ngu_hanh) in bat_quai_mappings {
            self.conn.execute(
                "INSERT OR IGNORE INTO router_metadata 
                 (bat_quai, db_mapping, checkpoint_number, ngu_hanh) 
                 VALUES (?1, ?2, ?3, ?4)",
                params![bat_quai, db, checkpoint as i32, ngu_hanh],
            )?;
        }

        self.conn.execute(
            "INSERT OR IGNORE INTO system_config (key, value) 
             VALUES ('version', '0.1.0'), ('initialized', 'true')",
            params![],
        )?;

        Ok(())
    }

    // === Cửu Cung Operations (Thổ Special) ===

    pub fn get_cuu_cung_position(&self, position: i32) -> Result<Option<(String, String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT bat_quai, ngu_hanh, description FROM cuu_cung 
             WHERE position = ?1 AND active = 1"
        )?;
        
        let result = stmt.query_row([position], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        });

        match result {
            Ok(data) => Ok(Some(data)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Error::Custom(e.to_string())),
        }
    }

    pub fn get_tho_positions(&self) -> Result<Vec<(i32, String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT position, bat_quai, description FROM cuu_cung 
             WHERE ngu_hanh = 'Tho' AND active = 1 
             ORDER BY position"
        )?;
        
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
        })?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::Custom(e.to_string()))
    }

    pub fn is_tho_position(&self, position: i32) -> bool {
        THO_CUU_CUNG_POSITIONS.contains(&position)
    }

    // === Config Operations ===

    pub fn store_config(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO system_config (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        self.log_audit("store_config", Some(key), None, Some(value))?;
        Ok(())
    }

    pub fn get_config(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT value FROM system_config WHERE key = ?1"
        )?;
        
        let result = stmt.query_row([key], |row| row.get(0));
        match result {
            Ok(val) => Ok(Some(val)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Error::Custom(e.to_string())),
        }
    }

    // === Router Metadata ===

    pub fn get_bat_quai_mapping(&self, bat_quai: &str) -> Result<Option<(String, i32, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT db_mapping, checkpoint_number, ngu_hanh 
             FROM router_metadata WHERE bat_quai = ?1"
        )?;
        
        let result = stmt.query_row([bat_quai], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        });

        match result {
            Ok(mapping) => Ok(Some(mapping)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Error::Custom(e.to_string())),
        }
    }

    // === Health Checks ===

    pub fn log_health_check(&self, component: &str, status: &str, message: Option<&str>) -> Result<()> {
        self.conn.execute(
            "INSERT INTO health_checks (component, status, message) 
             VALUES (?1, ?2, ?3)",
            params![component, status, message.unwrap_or("")],
        )?;
        Ok(())
    }

    // === Audit Logging ===

    pub fn log_audit(&self, action: &str, entity: Option<&str>,
                 old_value: Option<&str>, new_value: Option<&str>) -> Result<()> {
        self.conn.execute(
            "INSERT INTO audit_logs (action, entity, old_value, new_value)
             VALUES (?1, ?2, ?3, ?4)",
            params![action, entity.unwrap_or(""), old_value.unwrap_or(""), new_value.unwrap_or("")],
        )?;
        Ok(())
    }

    // === MEMORY SYSTEM OPERATIONS (ZeroClaw-style) ===

    /// Store a memory chunk
    pub fn store_chunk(&self, content: &str, source: Option<&str>, metadata: Option<&str>) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO memory_chunks (content, source, metadata)
             VALUES (?1, ?2, ?3)",
            params![content, source.unwrap_or(""), metadata.unwrap_or("")],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Store embedding for a chunk
    pub fn store_embedding(&self, chunk_id: i64, embedding_type: &str, vector: &[f32]) -> Result<()> {
        let vector_json = serde_json::to_string(vector)
            .map_err(|e| Error::Custom(format!("JSON serialize failed: {}", e)))?;

        self.conn.execute(
            "INSERT INTO embeddings (chunk_id, embedding_type, vector_json, dimensions)
             VALUES (?1, ?2, ?3, ?4)",
            params![chunk_id, embedding_type, vector_json, vector.len() as i64],
        )?;
        Ok(())
    }

    /// Search with hybrid (vector + keyword)
    pub fn hybrid_search(&self, query: &str, vector: Option<&[f32]>, vector_weight: f32, limit: i64) -> Result<Vec<MemorySearchResult>> {
        // FTS5 keyword search with BM25
        let fts_sql = "
            SELECT m.id, m.content, m.source, bm25(memory_fts) as score
            FROM memory_chunks m
            JOIN memory_fts ON m.id = memory_fts.rowid
            WHERE memory_fts MATCH ?1
            ORDER BY score
            LIMIT ?2
        ";

        let mut stmt = self.conn.prepare(fts_sql)?;
        let rows = stmt.query_map(params![query, limit], |row| {
            Ok(MemorySearchResult {
                id: row.get(0)?,
                content: row.get(1)?,
                source: row.get(2)?,
                keyword_score: row.get(3)?,
                vector_score: 0.0,
                combined_score: 0.0,
            })
        })?;

        let mut results: Vec<MemorySearchResult> = rows.filter_map(|r| r.ok()).collect();

        // If vector provided, calculate cosine similarity and merge
        if let Some(query_vector) = vector {
            self.merge_vector_scores(&mut results, query_vector, vector_weight)?;
        }

        // Sort by combined score
        results.sort_by(|a, b| b.combined_score.partial_cmp(&a.combined_score).unwrap_or(std::cmp::Ordering::Equal));

        Ok(results)
    }

    /// Merge vector scores with keyword scores
    fn merge_vector_scores(&self, results: &mut [MemorySearchResult], query_vector: &[f32], vector_weight: f32) -> Result<()> {
        for result in results.iter_mut() {
            // Get embedding for this chunk
            let embedding = self.get_embedding_for_chunk(result.id)?;
            
            if let Some(doc_vector) = embedding {
                // Calculate cosine similarity
                let similarity = cosine_similarity(query_vector, &doc_vector);
                result.vector_score = similarity;
                
                // Normalize keyword score (BM25 is negative, lower is better)
                let keyword_norm = 1.0 / (1.0 - result.keyword_score.min(0.0));
                
                // Hybrid merge (cast to f64 for consistency)
                result.combined_score = (vector_weight as f64) * similarity + (1.0 - (vector_weight as f64)) * keyword_norm;
            }
        }
        Ok(())
    }

    /// Get embedding for a chunk
    fn get_embedding_for_chunk(&self, chunk_id: i64) -> Result<Option<Vec<f32>>> {
        let mut stmt = self.conn.prepare(
            "SELECT vector_json FROM embeddings WHERE chunk_id = ?1 LIMIT 1"
        )?;

        let result = stmt.query_row([chunk_id], |row| row.get::<_, String>(0));
        
        match result {
            Ok(json_str) => {
                let vector: Vec<f32> = serde_json::from_str(&json_str)
                    .map_err(|e| Error::Custom(format!("JSON parse failed: {}", e)))?;
                Ok(Some(vector))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Error::Custom(e.to_string())),
        }
    }

    /// Cache embedding with LRU
    pub fn cache_embedding(&self, key: &str, embedding: &[f32]) -> Result<()> {
        let embedding_json = serde_json::to_string(embedding)
            .map_err(|e| Error::Custom(format!("JSON serialize failed: {}", e)))?;

        self.conn.execute(
            "INSERT OR REPLACE INTO embedding_cache (key, embedding_json, dimensions, last_accessed)
             VALUES (?1, ?2, ?3, datetime('now'))",
            params![key, embedding_json, embedding.len() as i64],
        )?;

        // LRU eviction: keep only 1000 most recent
        self.conn.execute(
            "DELETE FROM embedding_cache
             WHERE key NOT IN (
                 SELECT key FROM embedding_cache
                 ORDER BY last_accessed DESC
                 LIMIT 1000
             )",
            params![],
        )?;

        Ok(())
    }

    /// Get cached embedding
    pub fn get_cached_embedding(&self, key: &str) -> Result<Option<Vec<f32>>> {
        // Update access time for LRU
        self.conn.execute(
            "UPDATE embedding_cache SET last_accessed = datetime('now'), hits = hits + 1
             WHERE key = ?1",
            params![key],
        )?;

        let mut stmt = self.conn.prepare(
            "SELECT embedding_json FROM embedding_cache WHERE key = ?1"
        )?;

        let result = stmt.query_row([key], |row| row.get::<_, String>(0));
        
        match result {
            Ok(json_str) => {
                let vector: Vec<f32> = serde_json::from_str(&json_str)
                    .map_err(|e| Error::Custom(format!("JSON parse failed: {}", e)))?;
                Ok(Some(vector))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(Error::Custom(e.to_string())),
        }
    }

    /// Rebuild FTS5 index (safe reindex)
    pub fn rebuild_fts_index(&self) -> Result<()> {
        self.conn.execute_batch("
            INSERT INTO memory_fts(memory_fts) VALUES('rebuild');
        ")?;
        Ok(())
    }

    /// Get memory stats
    pub fn get_memory_stats(&self) -> Result<MemoryStats> {
        let mut stmt = self.conn.prepare(
            "SELECT 
                (SELECT COUNT(*) FROM memory_chunks) as chunks,
                (SELECT COUNT(*) FROM embeddings) as embeddings,
                (SELECT COUNT(*) FROM embedding_cache) as cache_size
            "
        )?;

        stmt.query_row([], |row| {
            Ok(MemoryStats {
                chunk_count: row.get(0)?,
                embedding_count: row.get(1)?,
                cache_size: row.get(2)?,
            })
        }).map_err(|e| Error::Custom(e.to_string()))
    }

    // === Learning: Raw SQL Execution ===

    pub fn execute_raw(&self, sql: &str) -> Result<usize> {
        self.conn.execute(sql, params![])
            .map_err(|e| Error::Custom(e.to_string()))
    }

    pub fn query_debug(&self, sql: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(sql)
            .map_err(|e| Error::Custom(e.to_string()))?;
        
        let results = stmt.query_map(params![], |row| {
            let mut row_str = String::new();
            for i in 0.. {
                match row.get::<_, String>(i) {
                    Ok(val) => row_str.push_str(&format!("{}|", val)),
                    Err(_) => break,
                }
            }
            Ok(row_str.trim_end_matches('|').to_string())
        })?;

        results.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| Error::Custom(e.to_string()))
    }

    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }

    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_agent() -> ThoAgent {
        let tmp = NamedTempFile::new().unwrap();
        ThoAgent::new(tmp.path().to_path_buf()).unwrap()
    }

    #[test]
    fn test_wal_mode_enabled() {
        let agent = create_test_agent();
        let mut stmt = agent.conn.prepare("PRAGMA journal_mode").unwrap();
        let mode: String = stmt.query_row([], |row| row.get(0)).unwrap();
        assert_eq!(mode, "wal");
    }

    #[test]
    fn test_tho_cuu_cung_positions() {
        let agent = create_test_agent();
        assert!(agent.is_tho_position(2));
        assert!(agent.is_tho_position(5));
        assert!(agent.is_tho_position(8));
        assert!(!agent.is_tho_position(1));
    }

    #[test]
    fn test_get_tho_positions() {
        let agent = create_test_agent();
        let positions = agent.get_tho_positions().unwrap();
        assert_eq!(positions.len(), 3);
        assert_eq!(positions[0].0, 2);
        assert_eq!(positions[1].0, 5);
        assert_eq!(positions[2].0, 8);
    }

    #[test]
    fn test_config_store_get() {
        let agent = create_test_agent();
        agent.store_config("test_key", "test_value").unwrap();
        let value = agent.get_config("test_key").unwrap();
        assert_eq!(value, Some("test_value".to_string()));
    }
}

// ============== Memory System Types ==============

/// Search result with hybrid scoring
#[derive(Debug, Clone)]
pub struct MemorySearchResult {
    pub id: i64,
    pub content: String,
    pub source: String,
    pub keyword_score: f64,  // BM25 score (negative, lower is better)
    pub vector_score: f64,   // Cosine similarity (0-1)
    pub combined_score: f64, // Weighted merge
}

/// Memory statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub chunk_count: i64,
    pub embedding_count: i64,
    pub cache_size: i64,
}

/// Cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for (x, y) in a.iter().zip(b.iter()) {
        dot_product += (*x as f64) * (*y as f64);
        norm_a += (*x as f64).powi(2);
        norm_b += (*y as f64).powi(2);
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a.sqrt() * norm_b.sqrt())
}
