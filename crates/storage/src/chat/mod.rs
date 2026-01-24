//! Chat database module.

pub mod schema;
pub mod sync;

pub use schema::{get_chat_migrations, CHAT_DB_VERSION};
pub use sync::ChatDbSync;

use crate::connection::DbConnection;
use crate::error::StorageResult;

/// Chat database with synchronous interface.
pub struct ChatDb {
    db: DbConnection,
}

impl ChatDb {
    /// Creates a new chat database.
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    /// Returns the synchronous interface.
    pub fn sync(&self) -> ChatDbSync {
        sync::ChatDbSync::new(self.db.clone())
    }

    /// Initializes the database schema.
    pub fn init(&self) -> StorageResult<()> {
        let mut manager = crate::migrations::MigrationManager::new();
        for migration in get_chat_migrations() {
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
    fn test_chat_db_init() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();
        let chat_db = ChatDb::new(db);

        chat_db.init().unwrap();

        // Verify tables exist
        let conn = chat_db.db.connect().unwrap();
        let count = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('chats', 'chats_full')",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap();
        assert_eq!(count, 2);
    }
}
