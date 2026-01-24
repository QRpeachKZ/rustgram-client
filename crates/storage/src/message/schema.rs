//! Message database schema.

use crate::error::StorageResult;
use crate::migrations::Migration;

/// Current message database schema version.
pub const MESSAGE_DB_VERSION: i32 = 5;

/// Migration 1: Create initial messages table.
pub struct MessageMigrationV1;

impl Migration for MessageMigrationV1 {
    fn version(&self) -> i32 {
        1
    }

    fn description(&self) -> &str {
        "Create messages table"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                dialog_id INTEGER NOT NULL,
                message_id INTEGER NOT NULL,
                sender_id INTEGER NOT NULL,
                date INTEGER NOT NULL,
                content BLOB NOT NULL,
                PRIMARY KEY (dialog_id, message_id)
            )",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

/// Migration 2: Add TTL support.
pub struct MessageMigrationV2;

impl Migration for MessageMigrationV2 {
    fn version(&self) -> i32 {
        2
    }

    fn description(&self) -> &str {
        "Add TTL support"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        // Check if column exists
        let has_column = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('messages') WHERE name = 'ttl_expires_at'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap_or(0)
            > 0;

        if !has_column {
            conn.execute(
                "ALTER TABLE messages ADD COLUMN ttl_expires_at INTEGER",
                [],
            )
            .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;
        }

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_messages_ttl ON messages(ttl_expires_at) WHERE ttl_expires_at IS NOT NULL",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

/// Migration 3: Add text column for basic search.
pub struct MessageMigrationV3;

impl Migration for MessageMigrationV3 {
    fn version(&self) -> i32 {
        3
    }

    fn description(&self) -> &str {
        "Add text column for basic search"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        // Check if column exists
        let has_column = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('messages') WHERE name = 'text'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap_or(0)
            > 0;

        if !has_column {
            conn.execute("ALTER TABLE messages ADD COLUMN text TEXT", [])
                .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;
        }

        Ok(())
    }
}

/// Migration 4: Create scheduled_messages table.
pub struct MessageMigrationV4;

impl Migration for MessageMigrationV4 {
    fn version(&self) -> i32 {
        4
    }

    fn description(&self) -> &str {
        "Create scheduled_messages table"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS scheduled_messages (
                dialog_id INTEGER NOT NULL,
                message_id INTEGER NOT NULL,
                date INTEGER NOT NULL,
                content BLOB NOT NULL,
                PRIMARY KEY (dialog_id, message_id)
            )",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_scheduled_messages_date ON scheduled_messages(dialog_id, date DESC)",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

/// Migration 5: Add deduplication and search columns.
pub struct MessageMigrationV5;

impl Migration for MessageMigrationV5 {
    fn version(&self) -> i32 {
        5
    }

    fn description(&self) -> &str {
        "Add deduplication and search columns"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        // Add columns if they don't exist
        let columns_to_add = vec![
            "random_id",
            "unique_message_id",
            "search_id",
            "top_thread_message_id",
        ];

        for column in columns_to_add {
            let has_column = conn
                .query_row(
                    &format!(
                        "SELECT COUNT(*) FROM pragma_table_info('messages') WHERE name = '{column}'"
                    ),
                    [],
                    |row| row.get::<_, i32>(0),
                )
                .unwrap_or(0)
                > 0;

            if !has_column {
                conn.execute(
                    &format!("ALTER TABLE messages ADD COLUMN {column} INTEGER"),
                    [],
                )
                .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;
            }
        }

        // Create indexes
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_messages_date ON messages(dialog_id, date DESC)",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_messages_sender ON messages(dialog_id, sender_id, date DESC)",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

/// Returns all message database migrations.
pub fn get_message_migrations() -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(MessageMigrationV1),
        Box::new(MessageMigrationV2),
        Box::new(MessageMigrationV3),
        Box::new(MessageMigrationV4),
        Box::new(MessageMigrationV5),
    ]
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

        MessageMigrationV1.apply(&mut conn).unwrap();

        // Verify table exists
        let table_exists: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='messages'",
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

        for migration in get_message_migrations() {
            migration.apply(&mut conn).unwrap();
        }

        // Verify all tables exist
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('messages', 'scheduled_messages')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 2);

        // Verify indexes
        let idx_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND tbl_name = 'messages'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(idx_count >= 3); // At least the date, sender, and TTL indexes
    }

    #[test]
    fn test_message_db_version() {
        assert_eq!(MESSAGE_DB_VERSION, 5);
    }
}
