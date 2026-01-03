//! Dialog database schema.

use crate::error::StorageResult;
use crate::migrations::Migration;

#[cfg(test)]
use crate::connection::DbConnection;

/// Current database schema version.
pub const DIALOG_DB_VERSION: i32 = 5;

/// Migration 1: Create initial dialogs table.
pub struct DialogMigrationV1;

impl Migration for DialogMigrationV1 {
    fn version(&self) -> i32 {
        1
    }

    fn description(&self) -> &str {
        "Create dialogs table"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS dialogs (
                dialog_id INTEGER PRIMARY KEY,
                dialog_order INTEGER NOT NULL,
                data BLOB NOT NULL,
                folder_id INTEGER
            )",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

/// Migration 2: Create notification_groups table.
pub struct DialogMigrationV2;

impl Migration for DialogMigrationV2 {
    fn version(&self) -> i32 {
        2
    }

    fn description(&self) -> &str {
        "Create notification_groups table"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS notification_groups (
                notification_group_id INTEGER PRIMARY KEY,
                dialog_id INTEGER NOT NULL,
                last_notification_date INTEGER
            )",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

/// Migration 3: Create index on notification_groups.
pub struct DialogMigrationV3;

impl Migration for DialogMigrationV3 {
    fn version(&self) -> i32 {
        3
    }

    fn description(&self) -> &str {
        "Create notification group index"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        conn.execute(
            "CREATE INDEX IF NOT EXISTS notification_group_by_last_notification_date
             ON notification_groups (last_notification_date, dialog_id, notification_group_id)
             WHERE last_notification_date IS NOT NULL",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

/// Migration 4: Add folder support to dialogs.
pub struct DialogMigrationV4;

impl Migration for DialogMigrationV4 {
    fn version(&self) -> i32 {
        4
    }

    fn description(&self) -> &str {
        "Add folder_id column to dialogs"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        // Check if column exists (SQLite doesn't have IF NOT EXISTS for columns)
        let has_column = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('dialogs') WHERE name = 'folder_id'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap_or(0)
            > 0;

        if !has_column {
            conn.execute("ALTER TABLE dialogs ADD COLUMN folder_id INTEGER", [])
                .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;
        }

        Ok(())
    }
}

/// Migration 5: Create index for folder queries.
pub struct DialogMigrationV5;

impl Migration for DialogMigrationV5 {
    fn version(&self) -> i32 {
        5
    }

    fn description(&self) -> &str {
        "Create folder dialog order index"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        conn.execute(
            "CREATE INDEX IF NOT EXISTS dialog_in_folder_by_dialog_order
             ON dialogs (folder_id, dialog_order, dialog_id)
             WHERE folder_id IS NOT NULL",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

/// Returns all dialog database migrations.
pub fn get_dialog_migrations() -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(DialogMigrationV1),
        Box::new(DialogMigrationV2),
        Box::new(DialogMigrationV3),
        Box::new(DialogMigrationV4),
        Box::new(DialogMigrationV5),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_migration_v1() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();
        let mut conn = db.connect().unwrap();

        DialogMigrationV1.apply(&mut conn).unwrap();

        // Verify table exists
        let table_exists: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='dialogs'",
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

        for migration in get_dialog_migrations() {
            migration.apply(&mut conn).unwrap();
        }

        // Verify all tables and indexes exist
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('dialogs', 'notification_groups')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 2);

        // Verify indexes
        let idx_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND tbl_name = 'dialogs'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(idx_count >= 1); // At least the folder index
    }
}
