//! Synchronous chat database interface.

use bytes::Bytes;
use rusqlite::params;

use crate::connection::DbConnection;
use crate::error::{StorageError, StorageResult};

/// Synchronous chat database interface.
pub struct ChatDbSync {
    db: DbConnection,
}

impl ChatDbSync {
    /// Creates a new synchronous chat database interface.
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    /// Adds or updates a chat in the database.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Chat identifier
    /// * `data` - Serialized chat data
    /// * `photo` - Serialized chat photo data (optional)
    /// * `permissions` - Serialized chat permissions (optional)
    pub fn add_chat(
        &mut self,
        chat_id: i64,
        data: Bytes,
        photo: Option<Bytes>,
        permissions: Option<Bytes>,
    ) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute(
            "INSERT OR REPLACE INTO chats (chat_id, data, photo, permissions)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                chat_id,
                data.as_ref(),
                photo.as_ref().map(|b| b.as_ref()),
                permissions.as_ref().map(|b| b.as_ref()),
            ],
        )
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Gets a chat by its ID.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Chat identifier
    pub fn get_chat(&mut self, chat_id: i64) -> StorageResult<Bytes> {
        let conn = self.db.connect()?;

        conn.query_row(
            "SELECT data FROM chats WHERE chat_id = ?1",
            params![chat_id],
            |row| {
                let data: Vec<u8> = row.get(0)?;
                Ok(Bytes::from(data))
            },
        )
        .map_err(|e| {
            if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                StorageError::NotFound(format!("Chat {}", chat_id))
            } else {
                StorageError::QueryError(e.to_string())
            }
        })
    }

    /// Gets multiple chats by their IDs.
    ///
    /// # Arguments
    ///
    /// * `chat_ids` - List of chat identifiers
    pub fn get_chats(&mut self, chat_ids: Vec<i64>) -> StorageResult<Vec<Bytes>> {
        if chat_ids.is_empty() {
            return Ok(Vec::new());
        }

        let conn = self.db.connect()?;

        // Build IN clause
        let in_clause = chat_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");

        let sql = format!("SELECT data FROM chats WHERE chat_id IN ({})", in_clause);

        let mut stmt = conn.prepare(&sql)?;

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            chat_ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();

        let mut chats = Vec::new();
        let mut rows = stmt.query(&params_refs[..])?;

        while let Some(row) = rows.next()? {
            let data: Vec<u8> = row.get(0)?;
            chats.push(Bytes::from(data));
        }

        Ok(chats)
    }

    /// Gets a chat's photo.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Chat identifier
    pub fn get_chat_photo(&mut self, chat_id: i64) -> StorageResult<Option<Bytes>> {
        let conn = self.db.connect()?;

        conn.query_row(
            "SELECT photo FROM chats WHERE chat_id = ?1",
            params![chat_id],
            |row| {
                let photo: Option<Vec<u8>> = row.get(0)?;
                Ok(photo.map(Bytes::from))
            },
        )
        .map_err(|e| {
            if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                StorageError::NotFound(format!("Chat {}", chat_id))
            } else {
                StorageError::QueryError(e.to_string())
            }
        })
    }

    /// Gets a chat's permissions.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Chat identifier
    pub fn get_chat_permissions(&mut self, chat_id: i64) -> StorageResult<Option<Bytes>> {
        let conn = self.db.connect()?;

        conn.query_row(
            "SELECT permissions FROM chats WHERE chat_id = ?1",
            params![chat_id],
            |row| {
                let permissions: Option<Vec<u8>> = row.get(0)?;
                Ok(permissions.map(Bytes::from))
            },
        )
        .map_err(|e| {
            if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                StorageError::NotFound(format!("Chat {}", chat_id))
            } else {
                StorageError::QueryError(e.to_string())
            }
        })
    }

    /// Deletes a chat from the database.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Chat identifier
    pub fn delete_chat(&mut self, chat_id: i64) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute("DELETE FROM chats WHERE chat_id = ?1", params![chat_id])
            .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Adds or updates full chat info.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Chat identifier
    /// * `full_info` - Serialized full chat info
    pub fn add_chat_full(&mut self, chat_id: i64, full_info: Bytes) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute(
            "INSERT OR REPLACE INTO chats_full (chat_id, full_info)
             VALUES (?1, ?2)",
            params![chat_id, full_info.as_ref()],
        )
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Gets full chat info.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Chat identifier
    pub fn get_chat_full(&mut self, chat_id: i64) -> StorageResult<Bytes> {
        let conn = self.db.connect()?;

        conn.query_row(
            "SELECT full_info FROM chats_full WHERE chat_id = ?1",
            params![chat_id],
            |row| {
                let data: Vec<u8> = row.get(0)?;
                Ok(Bytes::from(data))
            },
        )
        .map_err(|e| {
            if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                StorageError::NotFound(format!("ChatFullInfo for chat {}", chat_id))
            } else {
                StorageError::QueryError(e.to_string())
            }
        })
    }

    /// Deletes full chat info.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Chat identifier
    pub fn delete_chat_full(&mut self, chat_id: i64) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute(
            "DELETE FROM chats_full WHERE chat_id = ?1",
            params![chat_id],
        )
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Gets the count of chats in the database.
    pub fn get_chat_count(&mut self) -> StorageResult<i32> {
        let conn = self.db.connect()?;

        conn.query_row("SELECT COUNT(*) FROM chats", [], |row| row.get(0))
            .map_err(|e| StorageError::QueryError(e.to_string()))
    }

    /// Returns a reference to the underlying connection.
    pub fn db(&self) -> &DbConnection {
        &self.db
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat::schema::get_chat_migrations;
    use crate::migrations::MigrationManager;
    use tempfile::tempdir;

    fn setup_test_db() -> DbConnection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();

        // Run migrations
        let mut manager = MigrationManager::new();
        for migration in get_chat_migrations() {
            manager = manager.add_migration(migration);
        }
        manager.run(&db).unwrap();

        // Keep dir alive by intentionally leaking it
        std::mem::forget(dir);
        db
    }

    #[test]
    fn test_add_and_get_chat() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        let chat_id = 12345i64;
        let data = Bytes::from("test chat data");

        chat_db.add_chat(chat_id, data.clone(), None, None).unwrap();

        let retrieved = chat_db.get_chat(chat_id).unwrap();
        assert_eq!(retrieved, data);
    }

    #[test]
    fn test_get_nonexistent_chat() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        let result = chat_db.get_chat(99999);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_not_found());
    }

    #[test]
    fn test_delete_chat() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        let chat_id = 12345i64;
        let data = Bytes::from("test chat data");

        chat_db.add_chat(chat_id, data, None, None).unwrap();

        // Verify exists
        assert!(chat_db.get_chat(chat_id).is_ok());

        // Delete
        chat_db.delete_chat(chat_id).unwrap();

        // Verify gone
        assert!(chat_db.get_chat(chat_id).is_err());
    }

    #[test]
    fn test_get_chats_batch() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        // Add multiple chats
        let chat_ids = vec![1, 2, 3, 4, 5];
        for &chat_id in &chat_ids {
            chat_db
                .add_chat(chat_id, Bytes::from(format!("chat {}", chat_id)), None, None)
                .unwrap();
        }

        // Get all chats
        let chats = chat_db.get_chats(chat_ids.clone()).unwrap();
        assert_eq!(chats.len(), 5);

        // Get subset
        let chats = chat_db.get_chats(vec![1, 3, 5]).unwrap();
        assert_eq!(chats.len(), 3);
    }

    #[test]
    fn test_get_chats_empty() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        let chats = chat_db.get_chats(vec![]).unwrap();
        assert_eq!(chats.len(), 0);
    }

    #[test]
    fn test_chat_photo() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        let chat_id = 12345i64;
        let photo = Bytes::from("chat photo data");

        chat_db
            .add_chat(chat_id, Bytes::from("chat data"), Some(photo.clone()), None)
            .unwrap();

        let retrieved = chat_db.get_chat_photo(chat_id).unwrap();
        assert_eq!(retrieved, Some(photo));
    }

    #[test]
    fn test_chat_permissions() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        let chat_id = 12345i64;
        let permissions = Bytes::from("chat permissions data");

        chat_db
            .add_chat(
                chat_id,
                Bytes::from("chat data"),
                None,
                Some(permissions.clone()),
            )
            .unwrap();

        let retrieved = chat_db.get_chat_permissions(chat_id).unwrap();
        assert_eq!(retrieved, Some(permissions));
    }

    #[test]
    fn test_chat_full_info() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        let chat_id = 12345i64;
        let full_info = Bytes::from("full chat info data");

        chat_db.add_chat_full(chat_id, full_info.clone()).unwrap();

        let retrieved = chat_db.get_chat_full(chat_id).unwrap();
        assert_eq!(retrieved, full_info);
    }

    #[test]
    fn test_delete_chat_full() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        let chat_id = 12345i64;

        chat_db
            .add_chat_full(chat_id, Bytes::from("full info"))
            .unwrap();

        // Verify exists
        assert!(chat_db.get_chat_full(chat_id).is_ok());

        // Delete
        chat_db.delete_chat_full(chat_id).unwrap();

        // Verify gone
        assert!(chat_db.get_chat_full(chat_id).is_err());
    }

    #[test]
    fn test_get_chat_count() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        // Add chats
        for i in 1..=5 {
            chat_db
                .add_chat(i, Bytes::from(format!("chat {}", i)), None, None)
                .unwrap();
        }

        assert_eq!(chat_db.get_chat_count().unwrap(), 5);
    }

    #[test]
    fn test_update_chat() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        let chat_id = 12345i64;

        // Add initial chat
        chat_db
            .add_chat(chat_id, Bytes::from("original"), None, None)
            .unwrap();

        // Update with new data
        chat_db
            .add_chat(
                chat_id,
                Bytes::from("updated"),
                Some(Bytes::from("photo")),
                Some(Bytes::from("permissions")),
            )
            .unwrap();

        let retrieved = chat_db.get_chat(chat_id).unwrap();
        assert_eq!(retrieved, Bytes::from("updated"));

        let photo = chat_db.get_chat_photo(chat_id).unwrap();
        assert_eq!(photo, Some(Bytes::from("photo")));

        let permissions = chat_db.get_chat_permissions(chat_id).unwrap();
        assert_eq!(permissions, Some(Bytes::from("permissions")));
    }

    #[test]
    fn test_chat_and_full_info_separate() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        let chat_id = 12345i64;

        // Add chat
        chat_db
            .add_chat(chat_id, Bytes::from("chat data"), None, None)
            .unwrap();

        // Add full info
        chat_db
            .add_chat_full(chat_id, Bytes::from("full info"))
            .unwrap();

        // Both should exist
        assert!(chat_db.get_chat(chat_id).is_ok());
        assert!(chat_db.get_chat_full(chat_id).is_ok());

        // Delete chat
        chat_db.delete_chat(chat_id).unwrap();

        // Chat should be gone, but full info should remain
        assert!(chat_db.get_chat(chat_id).is_err());
        assert!(chat_db.get_chat_full(chat_id).is_ok());
    }

    #[test]
    fn test_get_nonexistent_photo() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        let chat_id = 12345i64;

        // Add chat without photo
        chat_db
            .add_chat(chat_id, Bytes::from("chat data"), None, None)
            .unwrap();

        let photo = chat_db.get_chat_photo(chat_id).unwrap();
        assert_eq!(photo, None);
    }

    #[test]
    fn test_get_nonexistent_permissions() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        let chat_id = 12345i64;

        // Add chat without permissions
        chat_db
            .add_chat(chat_id, Bytes::from("chat data"), None, None)
            .unwrap();

        let permissions = chat_db.get_chat_permissions(chat_id).unwrap();
        assert_eq!(permissions, None);
    }

    #[test]
    fn test_get_chats_nonexistent_ids() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        // Add one chat
        chat_db.add_chat(1, Bytes::from("chat 1"), None, None).unwrap();

        // Request multiple chats, some that don't exist
        let chats = chat_db.get_chats(vec![1, 999, 1000]).unwrap();
        assert_eq!(chats.len(), 1);
    }

    #[test]
    fn test_negative_chat_id() {
        let db = setup_test_db();
        let mut chat_db = ChatDbSync::new(db);

        // Chat IDs can be negative (supergroups/channels)
        let chat_id = -10012345i64;

        chat_db
            .add_chat(chat_id, Bytes::from("supergroup"), None, None)
            .unwrap();

        let retrieved = chat_db.get_chat(chat_id).unwrap();
        assert_eq!(retrieved, Bytes::from("supergroup"));
    }
}
