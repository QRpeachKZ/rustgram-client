//! Dialog database module.

pub mod schema;
pub mod sync;
pub mod async_;

pub use schema::{get_dialog_migrations, DIALOG_DB_VERSION};
pub use sync::{DialogDbSync, DialogsResult};
pub use async_::DialogDbAsync;

use crate::connection::DbConnection;
use crate::error::StorageResult;

/// Dialog database with both sync and async interfaces.
pub struct DialogDb {
    db: DbConnection,
}

impl DialogDb {
    /// Creates a new dialog database.
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    /// Returns the synchronous interface.
    pub fn sync(&self) -> DialogDbSync {
        sync::DialogDbSync::new(self.db.clone())
    }

    /// Returns the asynchronous interface.
    pub fn async_(&self) -> async_::DialogDbAsync {
        async_::DialogDbAsync::new(self.db.clone())
    }

    /// Initializes the database schema.
    pub fn init(&self) -> StorageResult<()> {
        let mut manager = crate::migrations::MigrationManager::new();
        for migration in get_dialog_migrations() {
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
    fn test_dialog_db_init() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();
        let dialog_db = DialogDb::new(db);

        dialog_db.init().unwrap();

        // Verify tables exist
        let mut conn = dialog_db.db.connect().unwrap();
        let count = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='dialogs'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }
}
