//! Synchronous message database interface.

use bytes::Bytes;
use rusqlite::params;

use crate::connection::DbConnection;
use crate::error::{StorageError, StorageResult};

/// Parameters for adding a message.
#[derive(Debug, Clone, Default)]
pub struct AddMessageParams {
    /// Dialog identifier (encoded DialogId)
    pub dialog_id: i64,
    /// Message identifier
    pub message_id: i32,
    /// Sender dialog identifier (encoded DialogId)
    pub sender_id: i64,
    /// Message timestamp (Unix timestamp)
    pub date: i32,
    /// TTL expiration timestamp (0 = no TTL)
    pub ttl_expires_at: Option<i32>,
    /// Serialized message content
    pub content: Bytes,
    /// Extracted text for search (optional)
    pub text: Option<String>,
    /// Random message ID for deduplication (optional)
    pub random_id: Option<i64>,
    /// Server-unique message ID (optional)
    pub unique_message_id: Option<i32>,
    /// Search index ID (optional)
    pub search_id: Option<i32>,
    /// Top thread message ID (optional)
    pub top_thread_message_id: Option<i32>,
}

impl AddMessageParams {
    /// Creates new add message parameters.
    #[must_use]
    pub const fn new(dialog_id: i64, message_id: i32, sender_id: i64, date: i32, content: Bytes) -> Self {
        Self {
            dialog_id,
            message_id,
            sender_id,
            date,
            ttl_expires_at: None,
            content,
            text: None,
            random_id: None,
            unique_message_id: None,
            search_id: None,
            top_thread_message_id: None,
        }
    }

    /// Sets TTL expiration.
    #[must_use]
    pub const fn with_ttl(mut self, ttl_expires_at: Option<i32>) -> Self {
        self.ttl_expires_at = ttl_expires_at;
        self
    }

    /// Sets text.
    #[must_use]
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Sets random ID.
    #[must_use]
    pub const fn with_random_id(mut self, random_id: i64) -> Self {
        self.random_id = Some(random_id);
        self
    }

    /// Sets unique message ID.
    #[must_use]
    pub const fn with_unique_message_id(mut self, unique_message_id: i32) -> Self {
        self.unique_message_id = Some(unique_message_id);
        self
    }
}

/// Message search filter for querying messages.
#[derive(Debug, Clone, Default)]
pub struct MessageSearchFilter {
    /// Filter by sender ID (optional)
    pub sender_id: Option<i64>,
    /// Minimum date (optional)
    pub min_date: Option<i32>,
    /// Maximum date (optional)
    pub max_date: Option<i32>,
    /// Text search query (optional, uses LIKE)
    pub text_query: Option<String>,
}

/// Message result from database queries.
#[derive(Debug, Clone)]
pub struct MessageDbDialogMessage {
    /// Dialog ID
    pub dialog_id: i64,
    /// Message ID
    pub message_id: i32,
    /// Sender ID
    pub sender_id: i64,
    /// Message date (Unix timestamp)
    pub date: i32,
    /// TTL expiration timestamp (0 = no TTL)
    pub ttl_expires_at: Option<i32>,
    /// Serialized message content
    pub content: Bytes,
    /// Extracted text for search
    pub text: Option<String>,
    /// Random message ID for deduplication
    pub random_id: Option<i64>,
    /// Server-unique message ID
    pub unique_message_id: Option<i32>,
    /// Search index ID
    pub search_id: Option<i32>,
    /// Top thread message ID for replies
    pub top_thread_message_id: Option<i32>,
}

/// Synchronous message database interface.
pub struct MessageDbSync {
    db: DbConnection,
}

impl MessageDbSync {
    /// Creates a new synchronous message database interface.
    pub fn new(db: DbConnection) -> Self {
        Self { db }
    }

    /// Adds or updates a message in the database.
    ///
    /// # Arguments
    ///
    /// * `params` - Message parameters
    pub fn add_message(&mut self, params: AddMessageParams) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute(
            "INSERT OR REPLACE INTO messages (
                dialog_id, message_id, sender_id, date, ttl_expires_at,
                content, text, random_id, unique_message_id, search_id, top_thread_message_id
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                params.dialog_id,
                params.message_id,
                params.sender_id,
                params.date,
                params.ttl_expires_at,
                params.content.as_ref(),
                params.text,
                params.random_id,
                params.unique_message_id,
                params.search_id,
                params.top_thread_message_id,
            ],
        )
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Gets a message by its full ID.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog identifier (encoded DialogId)
    /// * `message_id` - Message identifier
    pub fn get_message(&mut self, dialog_id: i64, message_id: i32) -> StorageResult<MessageDbDialogMessage> {
        let conn = self.db.connect()?;

        conn.query_row(
            "SELECT dialog_id, message_id, sender_id, date, ttl_expires_at,
                    content, text, random_id, unique_message_id, search_id, top_thread_message_id
             FROM messages WHERE dialog_id = ?1 AND message_id = ?2",
            params![dialog_id, message_id],
            |row| {
                Ok(MessageDbDialogMessage {
                    dialog_id: row.get(0)?,
                    message_id: row.get(1)?,
                    sender_id: row.get(2)?,
                    date: row.get(3)?,
                    ttl_expires_at: row.get(4)?,
                    content: Bytes::from(row.get::<_, Vec<u8>>(5)?),
                    text: row.get(6)?,
                    random_id: row.get(7)?,
                    unique_message_id: row.get(8)?,
                    search_id: row.get(9)?,
                    top_thread_message_id: row.get(10)?,
                })
            },
        )
        .map_err(|e| {
            if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                StorageError::NotFound(format!("Message {} in dialog {}", message_id, dialog_id))
            } else {
                StorageError::QueryError(e.to_string())
            }
        })
    }

    /// Deletes a message from the database.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog identifier (encoded DialogId)
    /// * `message_id` - Message identifier
    pub fn delete_message(&mut self, dialog_id: i64, message_id: i32) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute(
            "DELETE FROM messages WHERE dialog_id = ?1 AND message_id = ?2",
            params![dialog_id, message_id],
        )
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Gets messages from a dialog with pagination.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog identifier (encoded DialogId)
    /// * `from_message_id` - Message ID to start from (0 for latest)
    /// * `offset` - Number of messages to skip
    /// * `limit` - Maximum number of messages to return
    /// * `filter` - Optional search filter
    pub fn get_dialog_messages(
        &mut self,
        dialog_id: i64,
        from_message_id: i32,
        offset: i32,
        limit: i32,
        filter: Option<MessageSearchFilter>,
    ) -> StorageResult<Vec<MessageDbDialogMessage>> {
        let conn = self.db.connect()?;

        let (mut sql, mut params) = (
            "SELECT dialog_id, message_id, sender_id, date, ttl_expires_at,
                    content, text, random_id, unique_message_id, search_id, top_thread_message_id
             FROM messages WHERE dialog_id = ?1".to_string(),
            vec![Box::new(dialog_id) as Box<dyn rusqlite::ToSql>],
        );

        // Apply from_message_id filter
        if from_message_id > 0 {
            sql.push_str(" AND message_id < ?2");
            params.push(Box::new(from_message_id));
        }

        // Apply search filter
        if let Some(f) = filter {
            if let Some(sender_id) = f.sender_id {
                sql.push_str(" AND sender_id = ?");
                params.push(Box::new(sender_id));
            }
            if let Some(min_date) = f.min_date {
                sql.push_str(" AND date >= ?");
                params.push(Box::new(min_date));
            }
            if let Some(max_date) = f.max_date {
                sql.push_str(" AND date <= ?");
                params.push(Box::new(max_date));
            }
            if let Some(text_query) = f.text_query {
                sql.push_str(" AND text LIKE ?");
                params.push(Box::new(format!("%{}%", text_query)));
            }
        }

        sql.push_str(" ORDER BY message_id DESC LIMIT ?");
        params.push(Box::new(limit));

        if offset > 0 {
            sql.push_str(" OFFSET ?");
            params.push(Box::new(offset));
        }

        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        let mut stmt = conn.prepare(&sql)?;

        let mut messages = Vec::new();
        let mut rows = stmt.query(&params_refs[..])?;

        while let Some(row) = rows.next()? {
            messages.push(MessageDbDialogMessage {
                dialog_id: row.get(0)?,
                message_id: row.get(1)?,
                sender_id: row.get(2)?,
                date: row.get(3)?,
                ttl_expires_at: row.get(4)?,
                content: Bytes::from(row.get::<_, Vec<u8>>(5)?),
                text: row.get(6)?,
                random_id: row.get(7)?,
                unique_message_id: row.get(8)?,
                search_id: row.get(9)?,
                top_thread_message_id: row.get(10)?,
            });
        }

        Ok(messages)
    }

    /// Gets messages from a dialog by date range.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog identifier (encoded DialogId)
    /// * `min_date` - Minimum date (Unix timestamp)
    /// * `max_date` - Maximum date (Unix timestamp)
    /// * `limit` - Maximum number of messages to return
    pub fn get_messages_by_date(
        &mut self,
        dialog_id: i64,
        min_date: i32,
        max_date: i32,
        limit: i32,
    ) -> StorageResult<Vec<MessageDbDialogMessage>> {
        let conn = self.db.connect()?;

        let mut stmt = conn.prepare(
            "SELECT dialog_id, message_id, sender_id, date, ttl_expires_at,
                    content, text, random_id, unique_message_id, search_id, top_thread_message_id
             FROM messages
             WHERE dialog_id = ?1 AND date >= ?2 AND date <= ?3
             ORDER BY date DESC
             LIMIT ?4",
        )?;

        let mut messages = Vec::new();
        let mut rows = stmt.query(params![dialog_id, min_date, max_date, limit])?;

        while let Some(row) = rows.next()? {
            messages.push(MessageDbDialogMessage {
                dialog_id: row.get(0)?,
                message_id: row.get(1)?,
                sender_id: row.get(2)?,
                date: row.get(3)?,
                ttl_expires_at: row.get(4)?,
                content: Bytes::from(row.get::<_, Vec<u8>>(5)?),
                text: row.get(6)?,
                random_id: row.get(7)?,
                unique_message_id: row.get(8)?,
                search_id: row.get(9)?,
                top_thread_message_id: row.get(10)?,
            });
        }

        Ok(messages)
    }

    /// Adds or updates a scheduled message.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog identifier (encoded DialogId)
    /// * `message_id` - Scheduled message identifier
    /// * `date` - Scheduled date (Unix timestamp)
    /// * `content` - Serialized message content
    pub fn add_scheduled_message(
        &mut self,
        dialog_id: i64,
        message_id: i32,
        date: i32,
        content: Bytes,
    ) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute(
            "INSERT OR REPLACE INTO scheduled_messages (dialog_id, message_id, date, content)
             VALUES (?1, ?2, ?3, ?4)",
            params![dialog_id, message_id, date, content.as_ref()],
        )
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Gets scheduled messages from a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog identifier (encoded DialogId)
    /// * `limit` - Maximum number of messages to return
    pub fn get_scheduled_messages(
        &mut self,
        dialog_id: i64,
        limit: i32,
    ) -> StorageResult<Vec<MessageDbDialogMessage>> {
        let conn = self.db.connect()?;

        let mut stmt = conn.prepare(
            "SELECT dialog_id, message_id, 0 as sender_id, date, NULL as ttl_expires_at,
                    content, NULL as text, NULL as random_id, NULL as unique_message_id,
                    NULL as search_id, NULL as top_thread_message_id
             FROM scheduled_messages
             WHERE dialog_id = ?1
             ORDER BY date DESC
             LIMIT ?2",
        )?;

        let mut messages = Vec::new();
        let mut rows = stmt.query(params![dialog_id, limit])?;

        while let Some(row) = rows.next()? {
            messages.push(MessageDbDialogMessage {
                dialog_id: row.get(0)?,
                message_id: row.get(1)?,
                sender_id: row.get(2)?,
                date: row.get(3)?,
                ttl_expires_at: row.get(4)?,
                content: Bytes::from(row.get::<_, Vec<u8>>(5)?),
                text: row.get(6)?,
                random_id: row.get(7)?,
                unique_message_id: row.get(8)?,
                search_id: row.get(9)?,
                top_thread_message_id: row.get(10)?,
            });
        }

        Ok(messages)
    }

    /// Deletes a scheduled message.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog identifier (encoded DialogId)
    /// * `message_id` - Scheduled message identifier
    pub fn delete_scheduled_message(
        &mut self,
        dialog_id: i64,
        message_id: i32,
    ) -> StorageResult<()> {
        let conn = self.db.connect()?;

        conn.execute(
            "DELETE FROM scheduled_messages WHERE dialog_id = ?1 AND message_id = ?2",
            params![dialog_id, message_id],
        )
        .map_err(|e| StorageError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Gets the count of messages in a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog identifier (encoded DialogId)
    pub fn get_message_count(&mut self, dialog_id: i64) -> StorageResult<i32> {
        let conn = self.db.connect()?;

        conn.query_row(
            "SELECT COUNT(*) FROM messages WHERE dialog_id = ?1",
            params![dialog_id],
            |row| row.get(0),
        )
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
    use crate::message::schema::get_message_migrations;
    use crate::migrations::MigrationManager;
    use tempfile::tempdir;

    fn setup_test_db() -> DbConnection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let db = DbConnection::new(&db_path).unwrap();

        // Run migrations
        let mut manager = MigrationManager::new();
        for migration in get_message_migrations() {
            manager = manager.add_migration(migration);
        }
        manager.run(&db).unwrap();

        // Keep dir alive by intentionally leaking it
        std::mem::forget(dir);
        db
    }

    #[test]
    fn test_add_and_get_message() {
        let db = setup_test_db();
        let mut message_db = MessageDbSync::new(db);

        let content = Bytes::from("test message content");
        let dialog_id = 12345i64;
        let message_id = 1i32;
        let sender_id = 67890i64;
        let date = 1704067200i32; // 2024-01-01 00:00:00 UTC

        message_db
            .add_message(
                AddMessageParams::new(dialog_id, message_id, sender_id, date, content.clone())
                    .with_text("test message".to_string()),
            )
            .unwrap();

        let retrieved = message_db.get_message(dialog_id, message_id).unwrap();
        assert_eq!(retrieved.dialog_id, dialog_id);
        assert_eq!(retrieved.message_id, message_id);
        assert_eq!(retrieved.sender_id, sender_id);
        assert_eq!(retrieved.content, content);
        assert_eq!(retrieved.text, Some("test message".to_string()));
    }

    #[test]
    fn test_get_nonexistent_message() {
        let db = setup_test_db();
        let mut message_db = MessageDbSync::new(db);

        let result = message_db.get_message(99999, 99999);
        assert!(result.is_err());
        assert!(result.unwrap_err().is_not_found());
    }

    #[test]
    fn test_delete_message() {
        let db = setup_test_db();
        let mut message_db = MessageDbSync::new(db);

        let dialog_id = 12345i64;
        let message_id = 1i32;
        let content = Bytes::from("test");

        message_db
            .add_message(
                AddMessageParams::new(dialog_id, message_id, 67890, 1704067200, content.clone()),
            )
            .unwrap();

        // Verify exists
        assert!(message_db.get_message(dialog_id, message_id).is_ok());

        // Delete
        message_db.delete_message(dialog_id, message_id).unwrap();

        // Verify gone
        assert!(message_db.get_message(dialog_id, message_id).is_err());
    }

    #[test]
    fn test_get_dialog_messages() {
        let db = setup_test_db();
        let mut message_db = MessageDbSync::new(db);

        let dialog_id = 12345i64;

        // Add multiple messages
        for i in 1..=10 {
            message_db
                .add_message(
                    AddMessageParams::new(
                        dialog_id,
                        i,
                        67890,
                        1704067200 + i,
                        Bytes::from(format!("message {}", i)),
                    )
                    .with_text(format!("text {}", i)),
                )
                .unwrap();
        }

        // Get first page
        let messages = message_db
            .get_dialog_messages(dialog_id, 0, 0, 5, None)
            .unwrap();
        assert_eq!(messages.len(), 5);
        assert_eq!(messages[0].message_id, 10); // Descending order

        // Get next page (message_id < 5 means IDs 1-4)
        let messages = message_db
            .get_dialog_messages(dialog_id, 5, 0, 5, None)
            .unwrap();
        assert_eq!(messages.len(), 4); // Only 4 messages have ID < 5
        assert_eq!(messages[0].message_id, 4);
    }

    #[test]
    fn test_get_message_count() {
        let db = setup_test_db();
        let mut message_db = MessageDbSync::new(db);

        let dialog_id = 12345i64;

        // Add messages
        for i in 1..=5 {
            message_db
                .add_message(
                    AddMessageParams::new(dialog_id, i, 67890, 1704067200 + i, Bytes::new()),
                )
                .unwrap();
        }

        assert_eq!(message_db.get_message_count(dialog_id).unwrap(), 5);
    }

    #[test]
    fn test_scheduled_messages() {
        let db = setup_test_db();
        let mut message_db = MessageDbSync::new(db);

        let dialog_id = 12345i64;

        // Add scheduled messages
        for i in 1..=3 {
            message_db
                .add_scheduled_message(
                    dialog_id,
                    i * 100, // Different message IDs
                    1704067200 + i * 3600, // 1 hour apart
                    Bytes::from(format!("scheduled {}", i)),
                )
                .unwrap();
        }

        // Get scheduled messages
        let messages = message_db
            .get_scheduled_messages(dialog_id, 10)
            .unwrap();
        assert_eq!(messages.len(), 3);

        // Delete one
        message_db
            .delete_scheduled_message(dialog_id, 200)
            .unwrap();

        let messages = message_db
            .get_scheduled_messages(dialog_id, 10)
            .unwrap();
        assert_eq!(messages.len(), 2);
    }

    #[test]
    fn test_message_search_filter() {
        let db = setup_test_db();
        let mut message_db = MessageDbSync::new(db);

        let dialog_id = 12345i64;
        let sender_id = 67890i64;

        // Add messages with different text
        for i in 1..=5 {
            let text = if i % 2 == 0 {
                "important".to_string()
            } else {
                "normal".to_string()
            };
            message_db
                .add_message(
                    AddMessageParams::new(
                        dialog_id,
                        i,
                        sender_id,
                        1704067200 + i,
                        Bytes::from(format!("content {}", i)),
                    )
                    .with_text(text),
                )
                .unwrap();
        }

        // Search by text
        let filter = MessageSearchFilter {
            text_query: Some("important".to_string()),
            ..Default::default()
        };
        let messages = message_db
            .get_dialog_messages(dialog_id, 0, 0, 10, Some(filter))
            .unwrap();
        assert_eq!(messages.len(), 2);

        // Search by sender
        let filter = MessageSearchFilter {
            sender_id: Some(sender_id),
            ..Default::default()
        };
        let messages = message_db
            .get_dialog_messages(dialog_id, 0, 0, 10, Some(filter))
            .unwrap();
        assert_eq!(messages.len(), 5);

        // Search by date range
        let filter = MessageSearchFilter {
            min_date: Some(1704067202),
            max_date: Some(1704067204),
            ..Default::default()
        };
        let messages = message_db
            .get_dialog_messages(dialog_id, 0, 0, 10, Some(filter))
            .unwrap();
        assert_eq!(messages.len(), 3);
    }

    #[test]
    fn test_get_messages_by_date() {
        let db = setup_test_db();
        let mut message_db = MessageDbSync::new(db);

        let dialog_id = 12345i64;

        // Add messages
        for i in 1..=10 {
            message_db
                .add_message(AddMessageParams::new(
                    dialog_id,
                    i,
                    67890,
                    1704067200 + i * 100, // 100 seconds apart
                    Bytes::from(format!("message {}", i)),
                ))
                .unwrap();
        }

        // Get messages in date range
        let messages = message_db
            .get_messages_by_date(dialog_id, 1704067200, 1704067300, 10)
            .unwrap();
        assert!(messages.len() > 0);
    }

    #[test]
    fn test_ttl_support() {
        let db = setup_test_db();
        let mut message_db = MessageDbSync::new(db);

        let dialog_id = 12345i64;
        let message_id = 1i32;
        let ttl_expires_at = Some(1704153600i32); // 24 hours later

        message_db
            .add_message(
                AddMessageParams::new(
                    dialog_id,
                    message_id,
                    67890,
                    1704067200,
                    Bytes::from("expiring message"),
                )
                .with_ttl(ttl_expires_at),
            )
            .unwrap();

        let retrieved = message_db.get_message(dialog_id, message_id).unwrap();
        assert_eq!(retrieved.ttl_expires_at, ttl_expires_at);
    }

    #[test]
    fn test_update_message() {
        let db = setup_test_db();
        let mut message_db = MessageDbSync::new(db);

        let dialog_id = 12345i64;
        let message_id = 1i32;

        // Add initial message
        message_db
            .add_message(AddMessageParams::new(
                dialog_id,
                message_id,
                67890,
                1704067200,
                Bytes::from("original"),
            ))
            .unwrap();

        // Update with new content
        message_db
            .add_message(
                AddMessageParams::new(
                    dialog_id,
                    message_id,
                    67890,
                    1704067200,
                    Bytes::from("updated"),
                )
                .with_text("updated text".to_string()),
            )
            .unwrap();

        let retrieved = message_db.get_message(dialog_id, message_id).unwrap();
        assert_eq!(retrieved.content, Bytes::from("updated"));
        assert_eq!(retrieved.text, Some("updated text".to_string()));
    }

    #[test]
    fn test_multiple_dialogs() {
        let db = setup_test_db();
        let mut message_db = MessageDbSync::new(db);

        // Add messages to different dialogs
        message_db
            .add_message(AddMessageParams::new(
                1,
                1,
                100,
                1704067200,
                Bytes::from("dialog 1 message"),
            ))
            .unwrap();

        message_db
            .add_message(AddMessageParams::new(
                2,
                1,
                200,
                1704067200,
                Bytes::from("dialog 2 message"),
            ))
            .unwrap();

        // Verify separate counts
        assert_eq!(message_db.get_message_count(1).unwrap(), 1);
        assert_eq!(message_db.get_message_count(2).unwrap(), 1);

        // Verify can get from each dialog
        assert!(message_db.get_message(1, 1).is_ok());
        assert!(message_db.get_message(2, 1).is_ok());
    }
}
