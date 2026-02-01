//! User database module.

pub mod schema;
pub mod sync;

pub use schema::{get_user_migrations, USER_DB_VERSION};
pub use sync::UserDbSync;

use crate::connection::DbConnection;
use crate::error::StorageResult;

/// User database with synchronous interface.
pub struct UserDb {
    db: DbConnection,
}

impl UserDb {
    /// Creates a new user database.
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    /// Returns the synchronous interface.
    pub fn sync(&self) -> UserDbSync {
        sync::UserDbSync::new(self.db.clone())
    }

    /// Initializes the database schema.
    pub fn init(&self) -> StorageResult<()> {
        let mut manager = crate::migrations::MigrationManager::new();
        for migration in get_user_migrations() {
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
    fn test_user_db_init() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();
        let user_db = UserDb::new(db);

        user_db.init().unwrap();

        // Verify tables exist
        let conn = user_db.db.connect().unwrap();
        let count = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('users', 'users_full')",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap();
        assert_eq!(count, 2);
    }
}
