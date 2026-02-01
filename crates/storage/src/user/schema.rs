//! User database schema.

use crate::error::StorageResult;
use crate::migrations::Migration;

/// Current user database schema version.
pub const USER_DB_VERSION: i32 = 2;

/// Migration 1: Create initial users table.
pub struct UserMigrationV1;

impl Migration for UserMigrationV1 {
    fn version(&self) -> i32 {
        1
    }

    fn description(&self) -> &str {
        "Create users table"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                user_id INTEGER PRIMARY KEY NOT NULL,
                data BLOB NOT NULL,
                profile_photo BLOB,
                bio TEXT
            )",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

/// Migration 2: Create users_full table.
pub struct UserMigrationV2;

impl Migration for UserMigrationV2 {
    fn version(&self) -> i32 {
        2
    }

    fn description(&self) -> &str {
        "Create users_full table"
    }

    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users_full (
                user_id INTEGER PRIMARY KEY NOT NULL,
                full_info BLOB NOT NULL
            )",
            [],
        )
        .map_err(|e| crate::error::StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

/// Returns all user database migrations.
pub fn get_user_migrations() -> Vec<Box<dyn Migration>> {
    vec![Box::new(UserMigrationV1), Box::new(UserMigrationV2)]
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

        UserMigrationV1.apply(&mut conn).unwrap();

        // Verify table exists
        let table_exists: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='users'",
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

        for migration in get_user_migrations() {
            migration.apply(&mut conn).unwrap();
        }

        // Verify all tables exist
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('users', 'users_full')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_user_db_version() {
        assert_eq!(USER_DB_VERSION, 2);
    }
}
