//! Chat database schema.

use crate::error::StorageResult;
use crate::migrations::Migration;

/// Current chat database schema version.
pub const CHAT_DB_VERSION: i32 = 2;

/// Migration 1: Create initial chats table.
pub struct ChatMigrationV1;

impl Migration for ChatMigrationV1 {
    fn version(&self) -> i32 {
        1
    }

    fn description(&self) -> &str {
        "Create chats table"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS chats (
                chat_id INTEGER PRIMARY KEY NOT NULL,
                data BLOB NOT NULL,
                photo BLOB,
                permissions BLOB
            )",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

/// Migration 2: Create chats_full table.
pub struct ChatMigrationV2;

impl Migration for ChatMigrationV2 {
    fn version(&self) -> i32 {
        2
    }

    fn description(&self) -> &str {
        "Create chats_full table"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS chats_full (
                chat_id INTEGER PRIMARY KEY NOT NULL,
                full_info BLOB NOT NULL
            )",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

/// Returns all chat database migrations.
pub fn get_chat_migrations() -> Vec<Box<dyn Migration>> {
    vec![Box::new(ChatMigrationV1), Box::new(ChatMigrationV2)]
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

        ChatMigrationV1.apply(&mut conn).unwrap();

        // Verify table exists
        let table_exists: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='chats'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(table_exists, 1);
    }

    #[test]
    fn test_all_migrations() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();
        let mut conn = db.connect().unwrap();

        for migration in get_chat_migrations() {
            migration.apply(&mut conn).unwrap();
        }

        // Verify all tables exist
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('chats', 'chats_full')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_chat_db_version() {
        assert_eq!(CHAT_DB_VERSION, 2);
    }
}
