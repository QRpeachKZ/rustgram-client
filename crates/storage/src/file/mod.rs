//! File database module.

pub mod schema;
pub mod sync;

pub use schema::{get_file_migrations, FILE_DB_VERSION};
pub use sync::FileDbSync;

use crate::connection::DbConnection;
use crate::error::StorageResult;

/// File database with synchronous interface.
pub struct FileDb {
    db: DbConnection,
}

impl FileDb {
    /// Creates a new file database.
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    /// Returns the synchronous interface.
    pub fn sync(&self) -> FileDbSync {
        sync::FileDbSync::new(self.db.clone())
    }

    /// Initializes the database schema.
    pub fn init(&self) -> StorageResult<()> {
        let mut manager = crate::migrations::MigrationManager::new();
        for migration in get_file_migrations() {
            manager = manager.add_migration(migration);
        }
        manager.run(&self.db)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_file_db_init() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();
        let file_db = FileDb::new(db);

        file_db.init().unwrap();

        // Verify tables exist
        let conn = file_db.db.connect().unwrap();
        let count = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('files', 'file_db_id_seq')",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap();
        assert_eq!(count, 2);
    }
}
