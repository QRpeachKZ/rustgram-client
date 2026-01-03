//! Database schema migrations.

use crate::connection::DbConnection;
use crate::error::{StorageError, StorageResult};

/// A migration that updates the database schema.
pub trait Migration: Send + Sync {
    /// Returns the version number of this migration.
    fn version(&self) -> i32;

    /// Returns a description of this migration.
    fn description(&self) -> &str;

    /// Applies the migration to the database.
    fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()>;
}

/// Manages database schema migrations.
pub struct MigrationManager {
    migrations: Vec<Box<dyn Migration>>,
}

impl MigrationManager {
    /// Creates a new migration manager with no migrations.
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }

    /// Adds a migration to the manager.
    pub fn add_migration(mut self, migration: Box<dyn Migration>) -> Self {
        self.migrations.push(migration);
        self
    }

    /// Runs all pending migrations.
    pub fn run(&self, db: &DbConnection) -> StorageResult<()> {
        let mut conn = db.connect()?;

        // Ensure migrations table exists
        self.create_migrations_table(&mut conn)?;

        // Get current version
        let current_version = self.get_current_version(&mut conn)?;

        // Apply migrations in order
        for migration in &self.migrations {
            if migration.version() > current_version {
                tracing::info!(
                    version = migration.version(),
                    description = migration.description(),
                    "Applying migration"
                );
                migration.apply(&mut conn)?;
                self.set_version(&mut conn, migration.version())?;
            }
        }

        Ok(())
    }

    /// Returns the current database schema version.
    fn get_current_version(&self, conn: &mut rusqlite::Connection) -> StorageResult<i32> {
        match conn.query_row(
            "SELECT version FROM schema_migrations ORDER BY version DESC LIMIT 1",
            [],
            |row| row.get(0),
        ) {
            Ok(version) => Ok(version),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
            Err(e) => Err(StorageError::from(e)),
        }
    }

    /// Sets the current schema version.
    fn set_version(&self, conn: &mut rusqlite::Connection, version: i32) -> StorageResult<()> {
        conn.execute(
            "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, ?2)",
            [&version as &dyn rusqlite::ToSql, &chrono_timestamp()],
        )
        .map_err(|e| StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }

    /// Creates the migrations tracking table if it doesn't exist.
    fn create_migrations_table(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| StorageError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

impl Default for MigrationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns the current timestamp as an ISO 8601 string.
fn chrono_timestamp() -> String {
    // Simple timestamp without chrono dependency
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));
    format!("{}", duration.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    struct TestMigrationV1;

    impl Migration for TestMigrationV1 {
        fn version(&self) -> i32 {
            1
        }

        fn description(&self) -> &str {
            "Create test table"
        }

        fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY, value TEXT)",
                [],
            )
            .map_err(|e| StorageError::MigrationError(e.to_string()))?;

            Ok(())
        }
    }

    struct TestMigrationV2;

    impl Migration for TestMigrationV2 {
        fn version(&self) -> i32 {
            2
        }

        fn description(&self) -> &str {
            "Add column to test table"
        }

        fn apply(&self, conn: &mut rusqlite::Connection) -> StorageResult<()> {
            conn.execute("ALTER TABLE test_table ADD COLUMN extra TEXT", [])
                .map_err(|e| StorageError::MigrationError(e.to_string()))?;

            Ok(())
        }
    }

    #[test]
    fn test_migration_manager() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();

        let manager = MigrationManager::new()
            .add_migration(Box::new(TestMigrationV1))
            .add_migration(Box::new(TestMigrationV2));

        // Run migrations
        manager.run(&db).unwrap();

        // Verify table exists
        let conn = db.connect().unwrap();
        let table_exists: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='test_table'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(table_exists, 1);

        // Verify version is set
        let version: i32 = conn
            .query_row(
                "SELECT version FROM schema_migrations ORDER BY version DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(version, 2);
    }

    #[test]
    fn test_migration_idempotent() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();

        let manager = MigrationManager::new()
            .add_migration(Box::new(TestMigrationV1))
            .add_migration(Box::new(TestMigrationV2));

        // Run migrations twice
        manager.run(&db).unwrap();
        manager.run(&db).unwrap();

        // Verify migrations only ran once
        let conn = db.connect().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(count, 2);
    }
}
