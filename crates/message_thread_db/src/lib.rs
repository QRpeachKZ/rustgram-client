// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Message Thread Database
//!
//! Thread-safe database operations for message threads.
//!
//! ## Overview
//!
//! This module provides database operations for storing and retrieving
//! message thread information. In TDLib, this is backed by SQLite,
//! but for now we provide the type definitions and a simplified interface.
//!
//! ## TDLib Reference
//!
//! - `td/telegram/MessageThreadDb.h`
//! - `td/telegram/MessageThreadDb.cpp`
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_thread_db::{MessageThreadDb, MessageThreadsResult};
//! use rustgram_types::DialogId;
//!
//! // In a real implementation, you would open a database connection
//! // For now, this provides the type definitions
//! ```

use rustgram_forum_topic_id::ForumTopicId;
use rustgram_types::DialogId;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Errors that can occur in message thread database operations.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum MessageThreadDbError {
    /// Database not initialized
    #[error("database not initialized")]
    NotInitialized,

    /// Thread not found
    #[error("thread not found for dialog {0}, topic {1}")]
    NotFound(i64, i32),

    /// Invalid database state
    #[error("invalid database state: {0}")]
    InvalidState(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(String),
}

/// Result type for message thread database operations.
pub type Result<T> = std::result::Result<T, MessageThreadDbError>;

/// Raw data for a message thread (serialized bytes).
///
/// In TDLib, this contains the serialized thread data.
pub type ThreadData = Vec<u8>;

/// Result of a query for message threads in a dialog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageThreadsResult {
    /// List of thread data (serialized)
    pub message_threads: Vec<ThreadData>,
    /// Order for the next query (pagination)
    pub next_order: i64,
}

impl Default for MessageThreadsResult {
    fn default() -> Self {
        Self {
            message_threads: Vec::new(),
            next_order: 0,
        }
    }
}

impl MessageThreadsResult {
    /// Creates a new empty result.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a result with the specified threads and next order.
    #[must_use]
    pub fn with_fields(message_threads: Vec<ThreadData>, next_order: i64) -> Self {
        Self {
            message_threads,
            next_order,
        }
    }

    /// Returns true if there are no threads.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.message_threads.is_empty()
    }

    /// Returns the number of threads.
    #[must_use]
    pub fn len(&self) -> usize {
        self.message_threads.len()
    }

    /// Adds a thread to the result.
    pub fn add_thread(&mut self, data: ThreadData) {
        self.message_threads.push(data);
    }

    /// Clears all threads.
    pub fn clear(&mut self) {
        self.message_threads.clear();
        self.next_order = 0;
    }
}

impl fmt::Display for MessageThreadsResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MessageThreadsResult({} threads, next_order: {})",
            self.message_threads.len(),
            self.next_order
        )
    }
}

/// Synchronous interface for message thread database operations.
///
/// This trait defines the operations that can be performed on the
/// message thread database. In TDLib, this is backed by SQLite.
pub trait MessageThreadDbSync: Send + Sync {
    /// Adds or updates a message thread in the database.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog containing the thread
    /// * `forum_topic_id` - The forum topic identifier
    /// * `order` - Sorting order for the thread
    /// * `data` - Serialized thread data
    fn add_message_thread(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        order: i64,
        data: ThreadData,
    ) -> Result<()>;

    /// Deletes a message thread from the database.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog containing the thread
    /// * `forum_topic_id` - The forum topic identifier
    fn delete_message_thread(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
    ) -> Result<()>;

    /// Deletes all message threads for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog whose threads should be deleted
    fn delete_all_dialog_message_threads(&mut self, dialog_id: DialogId) -> Result<()>;

    /// Gets a single message thread from the database.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog containing the thread
    /// * `forum_topic_id` - The forum topic identifier
    ///
    /// # Returns
    ///
    /// The thread data, or empty Vec if not found.
    fn get_message_thread(
        &self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
    ) -> Result<ThreadData>;

    /// Gets message threads for a dialog with pagination.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog to query
    /// * `offset_order` - Order offset for pagination
    /// * `limit` - Maximum number of threads to return
    ///
    /// # Returns
    ///
    /// A result containing the threads and the next order offset.
    fn get_message_threads(
        &self,
        dialog_id: DialogId,
        offset_order: i64,
        limit: i32,
    ) -> Result<MessageThreadsResult>;

    /// Begins a write transaction.
    fn begin_write_transaction(&mut self) -> Result<()>;

    /// Commits the current transaction.
    fn commit_transaction(&mut self) -> Result<()>;
}

/// In-memory implementation of message thread database for testing.
///
/// This implementation stores all data in memory and is primarily
/// intended for testing and development. In production, a proper
/// SQLite-backed implementation should be used.
#[derive(Debug, Default)]
pub struct InMemoryMessageThreadDb {
    /// Map from (dialog_id, forum_topic_id) to thread data
    threads: std::collections::HashMap<(i64, i32), (i64, ThreadData)>,
    /// In-transaction flag
    in_transaction: bool,
}

impl InMemoryMessageThreadDb {
    /// Creates a new in-memory database.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of threads stored.
    #[must_use]
    pub fn len(&self) -> usize {
        self.threads.len()
    }

    /// Returns true if the database is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.threads.is_empty()
    }

    /// Clears all stored threads.
    pub fn clear(&mut self) {
        self.threads.clear();
    }
}

impl MessageThreadDbSync for InMemoryMessageThreadDb {
    fn add_message_thread(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        order: i64,
        data: ThreadData,
    ) -> Result<()> {
        let key = (dialog_id.to_encoded(), forum_topic_id.get());
        self.threads.insert(key, (order, data));
        Ok(())
    }

    fn delete_message_thread(
        &mut self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
    ) -> Result<()> {
        let key = (dialog_id.to_encoded(), forum_topic_id.get());
        self.threads.remove(&key).map(|_| ()).ok_or_else(|| {
            MessageThreadDbError::NotFound(dialog_id.to_encoded(), forum_topic_id.get())
        })
    }

    fn delete_all_dialog_message_threads(&mut self, dialog_id: DialogId) -> Result<()> {
        self.threads
            .retain(|(d, _), _| *d != dialog_id.to_encoded());
        Ok(())
    }

    fn get_message_thread(
        &self,
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
    ) -> Result<ThreadData> {
        let key = (dialog_id.to_encoded(), forum_topic_id.get());
        self.threads
            .get(&key)
            .map(|(_, data)| data.clone())
            .ok_or_else(|| {
                MessageThreadDbError::NotFound(dialog_id.to_encoded(), forum_topic_id.get())
            })
    }

    fn get_message_threads(
        &self,
        dialog_id: DialogId,
        offset_order: i64,
        limit: i32,
    ) -> Result<MessageThreadsResult> {
        let mut threads: Vec<_> = self
            .threads
            .iter()
            .filter(|((d, _), _)| *d == dialog_id.to_encoded())
            .filter(|(_, (order, _))| *order < offset_order)
            .map(|(_, (order, data))| (*order, data.clone()))
            .collect();

        // Sort by order descending
        threads.sort_by(|a, b| b.0.cmp(&a.0));

        // Apply limit
        threads.truncate(limit as usize);

        let next_order = threads
            .last()
            .map(|(order, _)| *order)
            .unwrap_or(offset_order);

        let message_threads = threads.into_iter().map(|(_, data)| data).collect();

        Ok(MessageThreadsResult {
            message_threads,
            next_order,
        })
    }

    fn begin_write_transaction(&mut self) -> Result<()> {
        if self.in_transaction {
            return Err(MessageThreadDbError::InvalidState(
                "already in transaction".to_string(),
            ));
        }
        self.in_transaction = true;
        Ok(())
    }

    fn commit_transaction(&mut self) -> Result<()> {
        if !self.in_transaction {
            return Err(MessageThreadDbError::InvalidState(
                "not in transaction".to_string(),
            ));
        }
        self.in_transaction = false;
        Ok(())
    }
}

/// Convenience type for using the in-memory implementation.
pub type MessageThreadDb = InMemoryMessageThreadDb;

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::ChannelId;

    fn make_db() -> MessageThreadDb {
        MessageThreadDb::new()
    }

    fn make_dialog_id() -> DialogId {
        let channel_id = ChannelId::new(123).unwrap();
        DialogId::from_channel(channel_id)
    }

    fn make_forum_topic_id(id: i32) -> ForumTopicId {
        ForumTopicId::new(id)
    }

    #[test]
    fn test_in_memory_db_new() {
        let db = make_db();
        assert!(db.is_empty());
        assert_eq!(db.len(), 0);
    }

    #[test]
    fn test_add_and_get_thread() {
        let mut db = make_db();
        let dialog_id = make_dialog_id();
        let topic_id = make_forum_topic_id(1);
        let data = vec![1, 2, 3, 4];

        db.add_message_thread(dialog_id, topic_id, 100, data.clone())
            .unwrap();

        let retrieved = db.get_message_thread(dialog_id, topic_id).unwrap();
        assert_eq!(retrieved, data);
    }

    #[test]
    fn test_add_overwrites() {
        let mut db = make_db();
        let dialog_id = make_dialog_id();
        let topic_id = make_forum_topic_id(1);

        db.add_message_thread(dialog_id, topic_id, 100, vec![1, 2])
            .unwrap();
        db.add_message_thread(dialog_id, topic_id, 200, vec![3, 4])
            .unwrap();

        let retrieved = db.get_message_thread(dialog_id, topic_id).unwrap();
        assert_eq!(retrieved, vec![3, 4]);
        assert_eq!(db.len(), 1);
    }

    #[test]
    fn test_delete_thread() {
        let mut db = make_db();
        let dialog_id = make_dialog_id();
        let topic_id = make_forum_topic_id(1);

        db.add_message_thread(dialog_id, topic_id, 100, vec![1, 2])
            .unwrap();
        assert_eq!(db.len(), 1);

        db.delete_message_thread(dialog_id, topic_id).unwrap();
        assert!(db.is_empty());
    }

    #[test]
    fn test_delete_thread_not_found() {
        let mut db = make_db();
        let dialog_id = make_dialog_id();
        let topic_id = make_forum_topic_id(1);

        let result = db.delete_message_thread(dialog_id, topic_id);
        assert!(matches!(result, Err(MessageThreadDbError::NotFound(_, _))));
    }

    #[test]
    fn test_delete_all_dialog_threads() {
        let mut db = make_db();
        let dialog1 = make_dialog_id();
        let dialog2 = DialogId::from_channel(ChannelId::new(456).unwrap());

        db.add_message_thread(dialog1, make_forum_topic_id(1), 100, vec![1])
            .unwrap();
        db.add_message_thread(dialog1, make_forum_topic_id(2), 200, vec![2])
            .unwrap();
        db.add_message_thread(dialog2, make_forum_topic_id(1), 300, vec![3])
            .unwrap();

        assert_eq!(db.len(), 3);

        db.delete_all_dialog_message_threads(dialog1).unwrap();
        assert_eq!(db.len(), 1);

        // Verify dialog2's thread still exists
        let result = db.get_message_thread(dialog2, make_forum_topic_id(1));
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_message_threads_pagination() {
        let mut db = make_db();
        let dialog_id = make_dialog_id();

        // Add threads with different orders
        for i in 1..=5u8 {
            db.add_message_thread(
                dialog_id,
                make_forum_topic_id(i as i32),
                (i as i64) * 100,
                vec![i],
            )
            .unwrap();
        }

        // Get threads with order < 500, limit 2
        let result = db.get_message_threads(dialog_id, 500, 2).unwrap();
        assert_eq!(result.len(), 2);
        // Should return order 400 and 300 (descending)
        assert_eq!(result.next_order, 300);

        // Get next page
        let result2 = db
            .get_message_threads(dialog_id, result.next_order, 2)
            .unwrap();
        assert_eq!(result2.len(), 2);
        // Should return order 200 and 100
        assert_eq!(result2.next_order, 100);
    }

    #[test]
    fn test_transaction() {
        let mut db = make_db();
        let dialog_id = make_dialog_id();
        let topic_id = make_forum_topic_id(1);

        db.begin_write_transaction().unwrap();
        db.add_message_thread(dialog_id, topic_id, 100, vec![1, 2])
            .unwrap();
        db.commit_transaction().unwrap();

        assert_eq!(db.len(), 1);
    }

    #[test]
    fn test_transaction_nested_error() {
        let mut db = make_db();

        db.begin_write_transaction().unwrap();
        let result = db.begin_write_transaction();
        assert!(matches!(result, Err(MessageThreadDbError::InvalidState(_))));
    }

    #[test]
    fn test_commit_without_begin() {
        let mut db = make_db();
        let result = db.commit_transaction();
        assert!(matches!(result, Err(MessageThreadDbError::InvalidState(_))));
    }

    #[test]
    fn test_message_threads_result_default() {
        let result = MessageThreadsResult::default();
        assert!(result.is_empty());
        assert_eq!(result.len(), 0);
        assert_eq!(result.next_order, 0);
    }

    #[test]
    fn test_message_threads_result_with_fields() {
        let threads = vec![vec![1, 2], vec![3, 4]];
        let result = MessageThreadsResult::with_fields(threads.clone(), 123);
        assert_eq!(result.len(), 2);
        assert_eq!(result.next_order, 123);
    }

    #[test]
    fn test_message_threads_result_add_clear() {
        let mut result = MessageThreadsResult::new();
        assert!(result.is_empty());

        result.add_thread(vec![1, 2]);
        result.add_thread(vec![3, 4]);
        assert_eq!(result.len(), 2);

        result.clear();
        assert!(result.is_empty());
        assert_eq!(result.next_order, 0);
    }

    #[test]
    fn test_error_display() {
        let err = MessageThreadDbError::NotFound(123, 456);
        let s = format!("{}", err);
        assert!(s.contains("123"));
        assert!(s.contains("456"));
    }

    #[test]
    fn test_clear_db() {
        let mut db = make_db();
        let dialog_id = make_dialog_id();

        for i in 1..=5u8 {
            db.add_message_thread(
                dialog_id,
                make_forum_topic_id(i as i32),
                i as i64 * 100,
                vec![i],
            )
            .unwrap();
        }

        assert_eq!(db.len(), 5);
        db.clear();
        assert!(db.is_empty());
    }
}
