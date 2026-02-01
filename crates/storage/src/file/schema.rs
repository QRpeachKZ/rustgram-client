//! File database schema.

use crate::error::StorageResult;
use crate::migrations::Migration;

/// Current file database schema version.
pub const FILE_DB_VERSION: i32 = 1;

/// Migration 1: Create file tables.
pub struct FileMigrationV1;

impl Migration for FileMigrationV1 {
    fn version(&self) -> i32 {
        1
    }

    fn description(&self) -> &str {
        "Create file tables"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS files (
                file_db_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                key BLOB NOT NULL UNIQUE,
                file_data BLOB NOT NULL
            )",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        // Create sequence table for file_db_id
        conn.execute(
            "CREATE TABLE IF NOT EXISTS file_db_id_seq (seq INTEGER)",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        // Initialize sequence if not exists
        let has_seq: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM file_db_id_seq",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        if has_seq == 0 {
            conn.execute(
                "INSERT INTO file_db_id_seq (seq) SELECT COALESCE(MAX(file_db_id), 0) FROM files",
                [],
            )
            .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;
        }

        // Create index on key
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_files_key ON files(key)",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

/// Returns all file database migrations.
pub fn get_file_migrations() -> Vec<Box<dyn Migration>> {
    vec![Box::new(FileMigrationV1)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::DbConnection;
    use tempfile::tempdir;

    #[test]
    fn test_migration_v1() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();
        let mut conn = db.connect().unwrap();

        FileMigrationV1.apply(&mut conn).unwrap();

        // Verify table exists
        let table_exists: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='files'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(table_exists, 1);

        // Verify sequence table exists
        let seq_exists: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='file_db_id_seq'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(seq_exists, 1);

        // Verify index exists
        let idx_exists: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name='idx_files_key'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(idx_exists, 1);
    }

    #[test]
    fn test_file_db_version() {
        assert_eq!(FILE_DB_VERSION, 1);
    }

    #[test]
    fn test_sequence_initialized() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();
        let mut conn = db.connect().unwrap();

        FileMigrationV1.apply(&mut conn).unwrap();

        // Verify sequence is initialized
        let seq: i32 = conn
            .query_row("SELECT seq FROM file_db_id_seq", [], |row| row.get(0))
            .unwrap();
        assert_eq!(seq, 0); // Starts at 0
    }
}
