// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Callback Queries Manager
//!
//! Manager for bot callback queries in Telegram.
//!
//! ## Overview
//!
//! The `CallbackQueriesManager` handles callback queries from inline buttons
//! in bot messages. It provides methods for:
//!
//! - Answering callback queries
//! - Processing new callback queries
//! - Handling inline callback queries
//! - Managing business callback queries
//!
//! ## Architecture
//!
//! Based on TDLib's `CallbackQueriesManager` class, this module:
//! - Processes callback query payloads (data and game)
//! - Sends answers to callback queries
//! - Manages inline message callbacks
//! - Handles business connection callbacks
//!
//! ## Callback Query Types
//!
//! ```text
//! Regular - From button in chat message
//! Inline - From button in inline message
//! Business - From button in business message
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_callback_queries_manager::CallbackQueriesManager;
//! use rustgram_types::{UserId, DialogId, MessageId};
//! use rustgram_message_full_id::MessageFullId;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let manager = CallbackQueriesManager::new();
//!
//!     // Answer a callback query
//!     manager.answer_callback_query(12345, "Button pressed!", false, None, 0).await?;
//!
//!     // Send a callback query
//!     let message_full_id = MessageFullId::new(DialogId::from_user(UserId::new(123).unwrap()), MessageId::from_server_id(456));
//!     manager.send_callback_query(message_full_id, b"payload".to_vec()).await?;
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod error;

use rustgram_message_full_id::MessageFullId;
use rustgram_types::{DialogId, MessageId, UserId};
use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

pub use error::{Error, Result};

/// Callback query payload
///
/// Represents the data sent with a callback query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallbackQueryPayload {
    /// Data payload (bytes)
    Data(Vec<u8>),
    /// Game payload (short name)
    Game(String),
}

impl CallbackQueryPayload {
    /// Creates a new data payload
    #[must_use]
    pub fn data(data: Vec<u8>) -> Self {
        Self::Data(data)
    }

    /// Creates a new game payload
    #[must_use]
    pub fn game(short_name: String) -> Self {
        Self::Game(short_name)
    }

    /// Returns true if this is a data payload
    #[must_use]
    pub const fn is_data(&self) -> bool {
        matches!(self, Self::Data(_))
    }

    /// Returns true if this is a game payload
    #[must_use]
    pub const fn is_game(&self) -> bool {
        matches!(self, Self::Game(_))
    }
}

/// Callback query information
#[derive(Debug, Clone)]
pub struct CallbackQueryInfo {
    /// Query ID
    id: i64,
    /// Sender user ID
    sender_user_id: UserId,
    /// Dialog ID (for regular queries)
    dialog_id: Option<DialogId>,
    /// Message ID
    message_id: Option<MessageId>,
    /// Inline message ID (for inline queries)
    inline_message_id: Option<String>,
    /// Chat instance
    chat_instance: i64,
    /// Payload
    payload: CallbackQueryPayload,
    /// Whether this is a business query
    is_business: bool,
    /// Business connection ID (for business queries)
    business_connection_id: Option<String>,
}

/// Answer to a callback query
///
/// Represents the response sent when answering a callback query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallbackQueryAnswer {
    /// Text to show
    pub text: Option<String>,
    /// Whether to show as alert
    pub show_alert: bool,
    /// URL to open
    pub url: Option<String>,
    /// Cache time in seconds
    pub cache_time: i32,
}

impl Default for CallbackQueryAnswer {
    fn default() -> Self {
        Self {
            text: None,
            show_alert: false,
            url: None,
            cache_time: 0,
        }
    }
}

impl CallbackQueryAnswer {
    /// Creates a new callback query answer
    #[must_use]
    pub const fn new() -> Self {
        Self {
            text: None,
            show_alert: false,
            url: None,
            cache_time: 0,
        }
    }

    /// Sets the answer text
    #[must_use]
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Sets whether to show as alert
    #[must_use]
    pub const fn with_alert(mut self, show_alert: bool) -> Self {
        self.show_alert = show_alert;
        self
    }

    /// Sets the URL to open
    #[must_use]
    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    /// Sets the cache time
    #[must_use]
    pub const fn with_cache_time(mut self, cache_time: i32) -> Self {
        self.cache_time = cache_time;
        self
    }
}

/// Manager for bot callback queries
///
/// Handles callback queries from inline buttons in bot messages.
/// Based on TDLib's `CallbackQueriesManager` class.
///
/// # Example
///
/// ```rust
/// use rustgram_callback_queries_manager::CallbackQueriesManager;
/// use rustgram_types::{UserId, DialogId, MessageId};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let manager = CallbackQueriesManager::new();
///
/// // Create a query first
/// let user_id = UserId::new(123).unwrap();
/// let dialog_id = DialogId::from_user(user_id);
/// let message_id = MessageId::from_server_id(456);
/// let query_id = manager.on_new_query(user_id, dialog_id, message_id, b"data".to_vec(), 0).await?;
///
/// // Answer the callback query
/// let answer = manager.answer_callback_query(query_id, "Done!", false, None, 0).await?;
/// assert_eq!(answer.text, Some("Done!".to_string()));
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct CallbackQueriesManager {
    /// Next callback query ID
    next_query_id: Arc<AtomicI64>,
    /// Active callback queries by ID
    queries: Arc<RwLock<HashMap<i64, CallbackQueryInfo>>>,
    /// Queries by message
    message_queries: Arc<RwLock<HashMap<DialogId, HashMap<MessageId, i64>>>>,
}

impl Default for CallbackQueriesManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CallbackQueriesManager {
    /// Creates a new callback queries manager
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_callback_queries_manager::CallbackQueriesManager;
    ///
    /// let manager = CallbackQueriesManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_query_id: Arc::new(AtomicI64::new(1)),
            queries: Arc::new(RwLock::new(HashMap::new())),
            message_queries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Answers a callback query
    ///
    /// # Arguments
    ///
    /// * `callback_query_id` - Callback query ID to answer
    /// * `text` - Text to show (optional)
    /// * `show_alert` - Whether to show as alert
    /// * `url` - URL to open (optional)
    /// * `cache_time` - Cache time in seconds
    ///
    /// # Returns
    ///
    /// The answer that was sent
    ///
    /// # Errors
    ///
    /// Returns an error if the callback query doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_callback_queries_manager::CallbackQueriesManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = CallbackQueriesManager::new();
    ///
    /// // First create a query
    /// let user_id = rustgram_types::UserId::new(123).unwrap();
    /// let dialog_id = rustgram_types::DialogId::from_user(user_id);
    /// let message_id = rustgram_types::MessageId::from_server_id(456);
    /// let query_id = manager.on_new_query(user_id, dialog_id, message_id, b"data".to_vec(), 0).await?;
    ///
    /// // Answer it
    /// let answer = manager.answer_callback_query(query_id, "Done!", false, None, 0).await?;
    /// assert_eq!(answer.text, Some("Done!".to_string()));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn answer_callback_query(
        &self,
        callback_query_id: i64,
        text: &str,
        show_alert: bool,
        url: Option<&str>,
        cache_time: i32,
    ) -> Result<CallbackQueryAnswer> {
        let queries = self.queries.read().await;

        // Check if query exists
        if !queries.contains_key(&callback_query_id) {
            return Err(Error::InvalidCallbackQueryId(callback_query_id));
        }

        // In a real implementation, this would send the answer to the server
        let answer = CallbackQueryAnswer {
            text: if text.is_empty() {
                None
            } else {
                Some(text.to_string())
            },
            show_alert,
            url: url.map(|u| u.to_string()),
            cache_time,
        };

        Ok(answer)
    }

    /// Registers a new callback query from a regular message
    ///
    /// # Arguments
    ///
    /// * `sender_user_id` - User who sent the query
    /// * `dialog_id` - Dialog where the message was sent
    /// * `message_id` - Message ID
    /// * `data` - Callback data payload
    /// * `chat_instance` - Chat instance identifier
    ///
    /// # Returns
    ///
    /// The callback query ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_callback_queries_manager::CallbackQueriesManager;
    /// use rustgram_types::{UserId, DialogId, MessageId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = CallbackQueriesManager::new();
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let message_id = MessageId::from_server_id(456);
    ///
    /// let query_id = manager.on_new_query(user_id, dialog_id, message_id, b"data".to_vec(), 0).await?;
    /// assert!(query_id > 0);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_new_query(
        &self,
        sender_user_id: UserId,
        dialog_id: DialogId,
        message_id: MessageId,
        data: Vec<u8>,
        chat_instance: i64,
    ) -> Result<i64> {
        let query_id = self.next_query_id.fetch_add(1, Ordering::SeqCst);

        let query_info = CallbackQueryInfo {
            id: query_id,
            sender_user_id,
            dialog_id: Some(dialog_id),
            message_id: Some(message_id),
            inline_message_id: None,
            chat_instance,
            payload: CallbackQueryPayload::Data(data),
            is_business: false,
            business_connection_id: None,
        };

        let mut queries = self.queries.write().await;
        let mut message_queries = self.message_queries.write().await;

        queries.insert(query_id, query_info);
        message_queries
            .entry(dialog_id)
            .or_insert_with(HashMap::new)
            .insert(message_id, query_id);

        Ok(query_id)
    }

    /// Registers a new callback query from an inline message
    ///
    /// # Arguments
    ///
    /// * `sender_user_id` - User who sent the query
    /// * `inline_message_id` - Inline message ID
    /// * `data` - Callback data payload
    /// * `chat_instance` - Chat instance identifier
    ///
    /// # Returns
    ///
    /// The callback query ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_callback_queries_manager::CallbackQueriesManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = CallbackQueriesManager::new();
    /// let user_id = UserId::new(123).unwrap();
    ///
    /// let query_id = manager.on_new_inline_query(user_id, "inline_msg_123".to_string(), b"data".to_vec(), 0).await?;
    /// assert!(query_id > 0);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_new_inline_query(
        &self,
        sender_user_id: UserId,
        inline_message_id: String,
        data: Vec<u8>,
        chat_instance: i64,
    ) -> Result<i64> {
        let query_id = self.next_query_id.fetch_add(1, Ordering::SeqCst);

        let query_info = CallbackQueryInfo {
            id: query_id,
            sender_user_id,
            dialog_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id),
            chat_instance,
            payload: CallbackQueryPayload::Data(data),
            is_business: false,
            business_connection_id: None,
        };

        let mut queries = self.queries.write().await;
        queries.insert(query_id, query_info);

        Ok(query_id)
    }

    /// Registers a new callback query from a business message
    ///
    /// # Arguments
    ///
    /// * `sender_user_id` - User who sent the query
    /// * `connection_id` - Business connection ID
    /// * `data` - Callback data payload
    /// * `chat_instance` - Chat instance identifier
    ///
    /// # Returns
    ///
    /// The callback query ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_callback_queries_manager::CallbackQueriesManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = CallbackQueriesManager::new();
    /// let user_id = UserId::new(123).unwrap();
    ///
    /// let query_id = manager.on_new_business_query(user_id, "conn_123".to_string(), b"data".to_vec(), 0).await?;
    /// assert!(query_id > 0);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn on_new_business_query(
        &self,
        sender_user_id: UserId,
        connection_id: String,
        data: Vec<u8>,
        chat_instance: i64,
    ) -> Result<i64> {
        let query_id = self.next_query_id.fetch_add(1, Ordering::SeqCst);

        let query_info = CallbackQueryInfo {
            id: query_id,
            sender_user_id,
            dialog_id: None,
            message_id: None,
            inline_message_id: None,
            chat_instance,
            payload: CallbackQueryPayload::Data(data),
            is_business: true,
            business_connection_id: Some(connection_id),
        };

        let mut queries = self.queries.write().await;
        queries.insert(query_id, query_info);

        Ok(query_id)
    }

    /// Sends a callback query for a message
    ///
    /// # Arguments
    ///
    /// * `message_full_id` - Full message identifier
    /// * `payload` - Callback payload to send
    ///
    /// # Returns
    ///
    /// The answer from the bot
    ///
    /// # Errors
    ///
    /// Returns an error if the message is invalid or payload is empty
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_callback_queries_manager::CallbackQueriesManager;
    /// use rustgram_types::{UserId, DialogId, MessageId};
    /// use rustgram_message_full_id::MessageFullId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = CallbackQueriesManager::new();
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let message_id = MessageId::from_server_id(456);
    /// let message_full_id = MessageFullId::new(dialog_id, message_id);
    ///
    /// // Register the query first
    /// manager.on_new_query(user_id, dialog_id, message_id, b"initial".to_vec(), 0).await?;
    ///
    /// // Send callback
    /// let answer = manager.send_callback_query(message_full_id, b"callback_data".to_vec()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_callback_query(
        &self,
        message_full_id: MessageFullId,
        payload: Vec<u8>,
    ) -> Result<CallbackQueryAnswer> {
        if payload.is_empty() {
            return Err(Error::InvalidPayload);
        }

        let dialog_id = message_full_id.dialog_id();
        let message_id = message_full_id.message_id();

        let message_queries = self.message_queries.read().await;
        let dialog_queries = message_queries.get(&dialog_id);
        let query_id = dialog_queries.and_then(|m| m.get(&message_id));

        let query_id = match query_id {
            Some(&id) => id,
            None => return Err(Error::InvalidMessageId),
        };

        // In a real implementation, this would send the query to the server
        // and receive an answer
        let answer = CallbackQueryAnswer::default();

        // Update the payload
        let mut queries = self.queries.write().await;
        if let Some(query) = queries.get_mut(&query_id) {
            query.payload = CallbackQueryPayload::Data(payload);
        }

        Ok(answer)
    }

    /// Gets info about a callback query
    ///
    /// # Arguments
    ///
    /// * `callback_query_id` - Callback query ID
    ///
    /// # Returns
    ///
    /// The callback query info if found
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_callback_queries_manager::CallbackQueriesManager;
    /// use rustgram_types::{UserId, DialogId, MessageId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = CallbackQueriesManager::new();
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let message_id = MessageId::from_server_id(456);
    /// let query_id = manager.on_new_query(user_id, dialog_id, message_id, b"data".to_vec(), 0).await?;
    ///
    /// let info = manager.get_query_info(query_id).await?;
    /// assert!(info.is_some());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_query_info(
        &self,
        callback_query_id: i64,
    ) -> Result<Option<CallbackQueryInfo>> {
        let queries = self.queries.read().await;
        Ok(queries.get(&callback_query_id).cloned())
    }

    /// Returns the number of active callback queries
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_callback_queries_manager::CallbackQueriesManager;
    /// use rustgram_types::{UserId, DialogId, MessageId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = CallbackQueriesManager::new();
    /// assert_eq!(manager.active_query_count().await, 0);
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let message_id = MessageId::from_server_id(456);
    /// manager.on_new_query(user_id, dialog_id, message_id, b"data".to_vec(), 0).await?;
    /// assert_eq!(manager.active_query_count().await, 1);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn active_query_count(&self) -> usize {
        self.queries.read().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Constructor Tests ==========

    #[test]
    fn test_callback_queries_manager_new() {
        let manager = CallbackQueriesManager::new();
        assert_eq!(manager.next_query_id.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_callback_queries_manager_default() {
        let manager = CallbackQueriesManager::default();
        assert_eq!(manager.next_query_id.load(Ordering::SeqCst), 1);
    }

    // ========== CallbackQueryPayload Tests ==========

    #[test]
    fn test_callback_query_payload_data() {
        let payload = CallbackQueryPayload::Data(vec![1, 2, 3]);
        assert!(payload.is_data());
        assert!(!payload.is_game());
    }

    #[test]
    fn test_callback_query_payload_game() {
        let payload = CallbackQueryPayload::Game("test_game".to_string());
        assert!(!payload.is_data());
        assert!(payload.is_game());
    }

    // ========== CallbackQueryAnswer Tests ==========

    #[test]
    fn test_callback_query_answer_default() {
        let answer = CallbackQueryAnswer::default();
        assert!(answer.text.is_none());
        assert!(!answer.show_alert);
        assert!(answer.url.is_none());
        assert_eq!(answer.cache_time, 0);
    }

    #[test]
    fn test_callback_query_answer_builder() {
        let answer = CallbackQueryAnswer::new()
            .with_text("Hello".to_string())
            .with_alert(true)
            .with_url("https://example.com".to_string())
            .with_cache_time(60);

        assert_eq!(answer.text, Some("Hello".to_string()));
        assert!(answer.show_alert);
        assert_eq!(answer.url, Some("https://example.com".to_string()));
        assert_eq!(answer.cache_time, 60);
    }

    // ========== On New Query Tests ==========

    #[tokio::test]
    async fn test_on_new_query_success() {
        let manager = CallbackQueriesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(456);

        let query_id = manager
            .on_new_query(user_id, dialog_id, message_id, b"data".to_vec(), 0)
            .await
            .unwrap();

        assert!(query_id > 0);

        let info = manager.get_query_info(query_id).await.unwrap();
        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(info.sender_user_id, user_id);
        assert_eq!(info.dialog_id, Some(dialog_id));
        assert_eq!(info.message_id, Some(message_id));
    }

    #[tokio::test]
    async fn test_on_new_query_increments_id() {
        let manager = CallbackQueriesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(456);

        let query_id1 = manager
            .on_new_query(user_id, dialog_id, message_id, b"data".to_vec(), 0)
            .await
            .unwrap();
        let query_id2 = manager
            .on_new_query(user_id, dialog_id, message_id, b"data2".to_vec(), 0)
            .await
            .unwrap();

        assert_eq!(query_id2, query_id1 + 1);
    }

    // ========== On New Inline Query Tests ==========

    #[tokio::test]
    async fn test_on_new_inline_query_success() {
        let manager = CallbackQueriesManager::new();
        let user_id = UserId::new(123).unwrap();

        let query_id = manager
            .on_new_inline_query(user_id, "inline_msg_123".to_string(), b"data".to_vec(), 0)
            .await
            .unwrap();

        assert!(query_id > 0);

        let info = manager.get_query_info(query_id).await.unwrap();
        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(info.inline_message_id, Some("inline_msg_123".to_string()));
        assert!(info.dialog_id.is_none());
        assert!(info.message_id.is_none());
    }

    // ========== On New Business Query Tests ==========

    #[tokio::test]
    async fn test_on_new_business_query_success() {
        let manager = CallbackQueriesManager::new();
        let user_id = UserId::new(123).unwrap();

        let query_id = manager
            .on_new_business_query(user_id, "conn_123".to_string(), b"data".to_vec(), 0)
            .await
            .unwrap();

        assert!(query_id > 0);

        let info = manager.get_query_info(query_id).await.unwrap();
        assert!(info.is_some());
        let info = info.unwrap();
        assert!(info.is_business);
        assert_eq!(info.business_connection_id, Some("conn_123".to_string()));
    }

    // ========== Answer Callback Query Tests ==========

    #[tokio::test]
    async fn test_answer_callback_query_success() {
        let manager = CallbackQueriesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(456);

        let query_id = manager
            .on_new_query(user_id, dialog_id, message_id, b"data".to_vec(), 0)
            .await
            .unwrap();

        let answer = manager
            .answer_callback_query(query_id, "Done!", false, None, 0)
            .await
            .unwrap();

        assert_eq!(answer.text, Some("Done!".to_string()));
        assert!(!answer.show_alert);
        assert!(answer.url.is_none());
    }

    #[tokio::test]
    async fn test_answer_callback_query_not_found() {
        let manager = CallbackQueriesManager::new();

        let result = manager
            .answer_callback_query(999, "Done!", false, None, 0)
            .await;
        assert!(matches!(result, Err(Error::InvalidCallbackQueryId(_))));
    }

    #[tokio::test]
    async fn test_answer_callback_query_with_url() {
        let manager = CallbackQueriesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(456);

        let query_id = manager
            .on_new_query(user_id, dialog_id, message_id, b"data".to_vec(), 0)
            .await
            .unwrap();

        let answer = manager
            .answer_callback_query(query_id, "Open link", false, Some("https://t.me"), 60)
            .await
            .unwrap();

        assert_eq!(answer.text, Some("Open link".to_string()));
        assert_eq!(answer.url, Some("https://t.me".to_string()));
        assert_eq!(answer.cache_time, 60);
    }

    // ========== Send Callback Query Tests ==========

    #[tokio::test]
    async fn test_send_callback_query_success() {
        let manager = CallbackQueriesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(456);
        let message_full_id = MessageFullId::new(dialog_id, message_id);

        manager
            .on_new_query(user_id, dialog_id, message_id, b"initial".to_vec(), 0)
            .await
            .unwrap();

        let answer = manager
            .send_callback_query(message_full_id, b"callback_data".to_vec())
            .await
            .unwrap();

        assert_eq!(answer, CallbackQueryAnswer::default());
    }

    #[tokio::test]
    async fn test_send_callback_query_empty_payload() {
        let manager = CallbackQueriesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(456);
        let message_full_id = MessageFullId::new(dialog_id, message_id);

        let result = manager.send_callback_query(message_full_id, vec![]).await;
        assert!(matches!(result, Err(Error::InvalidPayload)));
    }

    #[tokio::test]
    async fn test_send_callback_query_not_found() {
        let manager = CallbackQueriesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(456);
        let message_full_id = MessageFullId::new(dialog_id, message_id);

        let result = manager
            .send_callback_query(message_full_id, b"data".to_vec())
            .await;
        assert!(matches!(result, Err(Error::InvalidMessageId)));
    }

    // ========== Get Query Info Tests ==========

    #[tokio::test]
    async fn test_get_query_info_found() {
        let manager = CallbackQueriesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(456);

        let query_id = manager
            .on_new_query(user_id, dialog_id, message_id, b"data".to_vec(), 0)
            .await
            .unwrap();

        let info = manager.get_query_info(query_id).await.unwrap();
        assert!(info.is_some());
    }

    #[tokio::test]
    async fn test_get_query_info_not_found() {
        let manager = CallbackQueriesManager::new();

        let info = manager.get_query_info(999).await.unwrap();
        assert!(info.is_none());
    }

    // ========== Active Query Count Tests ==========

    #[tokio::test]
    async fn test_active_query_count_zero() {
        let manager = CallbackQueriesManager::new();
        assert_eq!(manager.active_query_count().await, 0);
    }

    #[tokio::test]
    async fn test_active_query_count_multiple() {
        let manager = CallbackQueriesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(456);

        manager
            .on_new_query(user_id, dialog_id, message_id, b"data1".to_vec(), 0)
            .await
            .unwrap();
        manager
            .on_new_query(user_id, dialog_id, message_id, b"data2".to_vec(), 0)
            .await
            .unwrap();
        manager
            .on_new_query(user_id, dialog_id, message_id, b"data3".to_vec(), 0)
            .await
            .unwrap();

        assert_eq!(manager.active_query_count().await, 3);
    }

    // ========== Mixed Query Types Tests ==========

    #[tokio::test]
    async fn test_mixed_query_types() {
        let manager = CallbackQueriesManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(456);

        // Regular query
        let q1 = manager
            .on_new_query(user_id, dialog_id, message_id, b"data".to_vec(), 0)
            .await
            .unwrap();

        // Inline query
        let q2 = manager
            .on_new_inline_query(user_id, "inline".to_string(), b"data".to_vec(), 0)
            .await
            .unwrap();

        // Business query
        let q3 = manager
            .on_new_business_query(user_id, "conn".to_string(), b"data".to_vec(), 0)
            .await
            .unwrap();

        assert_ne!(q1, q2);
        assert_ne!(q2, q3);
        assert_eq!(manager.active_query_count().await, 3);
    }
}
