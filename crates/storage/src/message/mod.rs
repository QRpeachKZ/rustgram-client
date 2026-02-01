//! Message database module.

pub mod schema;
pub mod sync;

pub use schema::{get_message_migrations, MESSAGE_DB_VERSION};
pub use sync::{MessageDbDialogMessage, MessageDbSync, MessageSearchFilter};

use crate::connection::DbConnection;
use crate::error::StorageResult;

/// Message database with synchronous interface.
pub struct MessageDb {
    db: DbConnection,
}

impl MessageDb {
    /// Creates a new message database.
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    /// Returns the synchronous interface.
    pub fn sync(&self) -> MessageDbSync {
        sync::MessageDbSync::new(self.db.clone())
    }

    /// Initializes the database schema.
    pub fn init(&self) -> StorageResult<()> {
        let mut manager = crate::migrations::MigrationManager::new();
        for migration in get_message_migrations() {
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
    fn test_message_db_init() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();
        let message_db = MessageDb::new(db);

        message_db.init().unwrap();

        // Verify tables exist
        let conn = message_db.db.connect().unwrap();
        let count = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('messages', 'scheduled_messages')",
                [],
                |row| row.get::<_, i32>(0),
            )
            .unwrap();
        assert_eq!(count, 2);
    }
}
