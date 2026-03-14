//! DuckDB storage for Trung Cung core metadata.

use std::path::{Path, PathBuf};
use duckdb::Connection;
use ochi_core::{Error, Result};

/// DuckDB store for router metadata and logs.
pub struct DuckDbStore {
    path: PathBuf,
    conn: Connection,
}

impl DuckDbStore {
    /// Open (or create) the DuckDB file at `path`.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_buf = path.as_ref().to_path_buf();
        let conn = Connection::open(&path_buf)
            .map_err(|e| Error::Custom(format!("DuckDB open failed: {e}")))?;

        let store = Self { path: path_buf, conn };
        store.init_schema()?;
        Ok(store)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| Error::Custom(format!("DuckDB open failed: {e}")))?;
        let store = Self {
            path: PathBuf::from(":memory:"),
            conn,
        };
        store.init_schema()?;
        Ok(store)
    }

    fn init_schema(&self) -> Result<()> {
        let ddl = r#"
            CREATE TABLE IF NOT EXISTS router_logs (
                id VARCHAR,
                created_at TIMESTAMP,
                intent VARCHAR,
                bat_quai VARCHAR,
                notes VARCHAR
            );

            CREATE TABLE IF NOT EXISTS agent_registry (
                agent_id VARCHAR,
                agent_group VARCHAR,
                agent_type VARCHAR,
                status VARCHAR,
                updated_at TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS task_queue (
                task_id VARCHAR,
                agent_id VARCHAR,
                task_type VARCHAR,
                payload VARCHAR,
                status VARCHAR,
                created_at TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS cuu_cung_nodes (
                cung_number INTEGER,
                direction VARCHAR,
                bat_quai VARCHAR,
                ngu_hanh VARCHAR,
                is_trung_cung BOOLEAN,
                PRIMARY KEY (cung_number)
            );

            CREATE TABLE IF NOT EXISTS bat_quai_db_map (
                bat_quai VARCHAR,
                db_name VARCHAR,
                PRIMARY KEY (bat_quai)
            );

            CREATE TABLE IF NOT EXISTS cuu_cung_schema (
                cung_number INTEGER,
                table_name VARCHAR,
                description VARCHAR,
                PRIMARY KEY (cung_number)
            );

            CREATE TABLE IF NOT EXISTS ngu_hanh_rules (
                rule_type VARCHAR,
                from_hanh VARCHAR,
                to_hanh VARCHAR,
                action VARCHAR
            );

            CREATE TABLE IF NOT EXISTS checkpoint_rules (
                checkpoint_number INTEGER,
                ngu_hanh VARCHAR,
                purpose VARCHAR,
                notes VARCHAR,
                PRIMARY KEY (checkpoint_number)
            );

            CREATE TABLE IF NOT EXISTS rules_5w1h (
                rule_id VARCHAR,
                who_hint VARCHAR,
                what_hint VARCHAR,
                where_hint VARCHAR,
                when_hint VARCHAR,
                why_hint VARCHAR,
                how_hint VARCHAR,
                bat_quai VARCHAR
            );

            CREATE TABLE IF NOT EXISTS logic_constraints (
                constraint_id VARCHAR,
                description VARCHAR,
                predicate VARCHAR
            );

            CREATE TABLE IF NOT EXISTS task_flow_logs (
                flow_id VARCHAR,
                phase VARCHAR,
                checkpoint_number INTEGER,
                bat_quai VARCHAR,
                status VARCHAR,
                created_at TIMESTAMP
            );
        "#;

        self.conn
            .execute_batch(ddl)
            .map_err(|e| Error::Custom(format!("DuckDB init failed: {e}")))?;

        let seed = r#"
            INSERT OR IGNORE INTO cuu_cung_nodes
                (cung_number, direction, bat_quai, ngu_hanh, is_trung_cung)
            VALUES
                (1, 'Bac',  'Kham', 'Thuy', false),
                (2, 'Dong Bac', 'Khon', 'Tho', false),
                (3, 'Dong', 'Chan', 'Moc', false),
                (4, 'Dong Nam', 'Ton', 'Moc', false),
                (5, 'Trung Cung', 'Trung', 'Tho', true),
                (6, 'Tay Bac', 'Can', 'Kim', false),
                (7, 'Tay', 'Doai', 'Kim', false),
                (8, 'Tay Nam', 'Can Gua', 'Tho', false),
                (9, 'Nam', 'Ly', 'Hoa', false);

            INSERT OR IGNORE INTO bat_quai_db_map (bat_quai, db_name)
            VALUES
                ('Kham', 'thuy.db'),
                ('Khon', 'tho.db'),
                ('Chan', 'moc.db'),
                ('Ton', 'moc.db'),
                ('Trung', 'tho.db'),
                ('Can', 'kim.db'),
                ('Doai', 'kim.db'),
                ('Can Gua', 'tho.db'),
                ('Ly', 'hoa.db');

            INSERT OR IGNORE INTO cuu_cung_schema (cung_number, table_name, description)
            VALUES
                (1, 'cung_1', 'Bac / Thuy / Luu tru dong'),
                (2, 'cung_2', 'Dong Bac / Tho / Dia ly'),
                (3, 'cung_3', 'Dong / Moc / Sinh truong'),
                (4, 'cung_4', 'Dong Nam / Moc / Phan luong'),
                (5, 'cung_5', 'Trung Cung / Tho / Dieu hanh'),
                (6, 'cung_6', 'Tay Bac / Kim / Ky luat'),
                (7, 'cung_7', 'Tay / Kim / Tuong tac'),
                (8, 'cung_8', 'Tay Nam / Tho / On dinh'),
                (9, 'cung_9', 'Nam / Hoa / Toc do');

            INSERT OR IGNORE INTO ngu_hanh_rules (rule_type, from_hanh, to_hanh, action)
            VALUES
                ('sinh', 'Moc', 'Hoa', 'create_related'),
                ('sinh', 'Hoa', 'Tho', 'create_related'),
                ('sinh', 'Tho', 'Kim', 'create_related'),
                ('sinh', 'Kim', 'Thuy', 'create_related'),
                ('sinh', 'Thuy', 'Moc', 'create_related'),
                ('khac', 'Moc', 'Tho', 'invalidate'),
                ('khac', 'Tho', 'Thuy', 'invalidate'),
                ('khac', 'Thuy', 'Hoa', 'invalidate'),
                ('khac', 'Hoa', 'Kim', 'invalidate'),
                ('khac', 'Kim', 'Moc', 'invalidate');

            INSERT OR IGNORE INTO checkpoint_rules
                (checkpoint_number, ngu_hanh, purpose, notes)
            VALUES
                (1, 'Thuy', 'Khoi nguon / luong du lieu', 'Dau vao dong chay'),
                (2, 'Tho', 'On dinh / nuoi duong', 'Thổ phụ'),
                (3, 'Moc', 'Sang tao / tri thuc nen', 'Checkpoint Mộc 1'),
                (4, 'Moc', 'Mo rong / dieu chinh', 'Checkpoint Mộc 2'),
                (5, 'Tho', 'Trung Cung / dieu phoi', 'Checkpoint trung tam'),
                (6, 'Kim', 'Ky luat / policy / auth', 'Checkpoint Kim 1'),
                (7, 'Kim', 'Quyet dinh / validation', 'Checkpoint Kim 2'),
                (8, 'Tho', 'Vung chac / neo', 'Thổ phụ'),
                (9, 'Hoa', 'Nang luong / phan hoi', 'Checkpoint Hỏa');

            INSERT OR IGNORE INTO rules_5w1h
                (rule_id, who_hint, what_hint, where_hint, when_hint, why_hint, how_hint, bat_quai)
            VALUES
                ('r1', 'user', 'chat', '', '', '', '', 'Doai'),
                ('r2', '', 'stream', '', '', '', '', 'Kham'),
                ('r3', '', 'cache', '', '', '', '', 'Khon'),
                ('r4', '', 'webhook', '', '', '', '', 'Chan'),
                ('r5', '', 'balance', '', '', '', '', 'Ton'),
                ('r6', '', 'session', '', '', '', '', 'Can Gua'),
                ('r7', '', 'search', '', '', '', '', 'Ly'),
                ('r8', 'admin', 'config', '', '', '', '', 'Can');
        "#;

        self.conn
            .execute_batch(seed)
            .map_err(|e| Error::Custom(format!("DuckDB seed failed: {e}")))?;

        let cuu_cung_tables = r#"
            CREATE TABLE IF NOT EXISTS cung_1 (
                id VARCHAR,
                title VARCHAR,
                payload VARCHAR,
                tags VARCHAR,
                who VARCHAR,
                what VARCHAR,
                where_ VARCHAR,
                when_ VARCHAR,
                why_ VARCHAR,
                how_ VARCHAR,
                created_at TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS cung_2 (
                id VARCHAR,
                title VARCHAR,
                payload VARCHAR,
                tags VARCHAR,
                who VARCHAR,
                what VARCHAR,
                where_ VARCHAR,
                when_ VARCHAR,
                why_ VARCHAR,
                how_ VARCHAR,
                created_at TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS cung_3 (
                id VARCHAR,
                title VARCHAR,
                payload VARCHAR,
                tags VARCHAR,
                who VARCHAR,
                what VARCHAR,
                where_ VARCHAR,
                when_ VARCHAR,
                why_ VARCHAR,
                how_ VARCHAR,
                created_at TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS cung_4 (
                id VARCHAR,
                title VARCHAR,
                payload VARCHAR,
                tags VARCHAR,
                who VARCHAR,
                what VARCHAR,
                where_ VARCHAR,
                when_ VARCHAR,
                why_ VARCHAR,
                how_ VARCHAR,
                created_at TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS cung_5 (
                id VARCHAR,
                title VARCHAR,
                payload VARCHAR,
                tags VARCHAR,
                who VARCHAR,
                what VARCHAR,
                where_ VARCHAR,
                when_ VARCHAR,
                why_ VARCHAR,
                how_ VARCHAR,
                created_at TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS cung_6 (
                id VARCHAR,
                title VARCHAR,
                payload VARCHAR,
                tags VARCHAR,
                who VARCHAR,
                what VARCHAR,
                where_ VARCHAR,
                when_ VARCHAR,
                why_ VARCHAR,
                how_ VARCHAR,
                created_at TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS cung_7 (
                id VARCHAR,
                title VARCHAR,
                payload VARCHAR,
                tags VARCHAR,
                who VARCHAR,
                what VARCHAR,
                where_ VARCHAR,
                when_ VARCHAR,
                why_ VARCHAR,
                how_ VARCHAR,
                created_at TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS cung_8 (
                id VARCHAR,
                title VARCHAR,
                payload VARCHAR,
                tags VARCHAR,
                who VARCHAR,
                what VARCHAR,
                where_ VARCHAR,
                when_ VARCHAR,
                why_ VARCHAR,
                how_ VARCHAR,
                created_at TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS cung_9 (
                id VARCHAR,
                title VARCHAR,
                payload VARCHAR,
                tags VARCHAR,
                who VARCHAR,
                what VARCHAR,
                where_ VARCHAR,
                when_ VARCHAR,
                why_ VARCHAR,
                how_ VARCHAR,
                created_at TIMESTAMP
            );
        "#;

        self.conn
            .execute_batch(cuu_cung_tables)
            .map_err(|e| Error::Custom(format!("DuckDB cuu_cung init failed: {e}")))?;
        Ok(())
    }

    pub fn record_route(&self, id: &str, intent: &str, bat_quai: &str, notes: &str) -> Result<()> {
        let sql = r#"
            INSERT INTO router_logs (id, created_at, intent, bat_quai, notes)
            VALUES (?1, now(), ?2, ?3, ?4)
        "#;

        self.conn
            .execute(sql, duckdb::params![id, intent, bat_quai, notes])
            .map_err(|e| Error::Custom(format!("DuckDB insert failed: {e}")))?;
        Ok(())
    }

    pub fn db_for_bat_quai(&self, bat_quai: &str) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT db_name FROM bat_quai_db_map WHERE bat_quai = ?1")
            .map_err(|e| Error::Custom(format!("DuckDB query failed: {e}")))?;
        let mut rows = stmt
            .query(duckdb::params![bat_quai])
            .map_err(|e| Error::Custom(format!("DuckDB query failed: {e}")))?;

        if let Some(row) = rows
            .next()
            .map_err(|e| Error::Custom(format!("DuckDB query failed: {e}")))? {
            let name: String = row
                .get(0)
                .map_err(|e| Error::Custom(format!("DuckDB get failed: {e}")))?;
            Ok(Some(name))
        } else {
            Ok(None)
        }
    }

    pub fn log_task_flow(&self, entry: TaskFlowLog) -> Result<()> {
        let sql = r#"
            INSERT INTO task_flow_logs (flow_id, phase, checkpoint_number, bat_quai, status, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, now())
        "#;
        self.conn
            .execute(
                sql,
                duckdb::params![
                    entry.flow_id,
                    entry.phase,
                    entry.checkpoint_number,
                    entry.bat_quai,
                    entry.status
                ],
            )
            .map_err(|e| Error::Custom(format!("DuckDB insert failed: {e}")))?;
        Ok(())
    }

    pub fn checkpoint_for_bat_quai(&self, bat_quai: &str) -> Result<Option<CheckpointRule>> {
        let sql = r#"
            SELECT n.cung_number, c.ngu_hanh, c.purpose, c.notes
            FROM cuu_cung_nodes n
            JOIN checkpoint_rules c ON c.checkpoint_number = n.cung_number
            WHERE n.bat_quai = ?1
        "#;
        let mut stmt = self
            .conn
            .prepare(sql)
            .map_err(|e| Error::Custom(format!("DuckDB query failed: {e}")))?;
        let mut rows = stmt
            .query(duckdb::params![bat_quai])
            .map_err(|e| Error::Custom(format!("DuckDB query failed: {e}")))?;

        if let Some(row) = rows
            .next()
            .map_err(|e| Error::Custom(format!("DuckDB query failed: {e}")))? {
            let checkpoint_number: i32 = row
                .get(0)
                .map_err(|e| Error::Custom(format!("DuckDB get failed: {e}")))?;
            let ngu_hanh: String = row
                .get(1)
                .map_err(|e| Error::Custom(format!("DuckDB get failed: {e}")))?;
            let purpose: String = row
                .get(2)
                .map_err(|e| Error::Custom(format!("DuckDB get failed: {e}")))?;
            let notes: String = row
                .get(3)
                .map_err(|e| Error::Custom(format!("DuckDB get failed: {e}")))?;

            Ok(Some(CheckpointRule {
                checkpoint_number,
                ngu_hanh,
                purpose,
                notes,
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckpointRule {
    pub checkpoint_number: i32,
    pub ngu_hanh: String,
    pub purpose: String,
    pub notes: String,
}

#[derive(Debug, Clone)]
pub struct TaskFlowLog {
    pub flow_id: String,
    pub phase: String,
    pub checkpoint_number: i32,
    pub bat_quai: String,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bat_quai_db_map() {
        let store = DuckDbStore::open_in_memory().expect("store");
        let db = store.db_for_bat_quai("Kham").expect("query");
        assert_eq!(db.as_deref(), Some("thuy.db"));
    }

    #[test]
    fn test_checkpoint_lookup() {
        let store = DuckDbStore::open_in_memory().expect("store");
        let rule = store
            .checkpoint_for_bat_quai("Can")
            .expect("query")
            .expect("rule");
        assert_eq!(rule.checkpoint_number, 6);
        assert_eq!(rule.ngu_hanh, "Kim");
    }
}
