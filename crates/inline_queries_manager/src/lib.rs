// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Inline Queries Manager
//!
//! Manages inline bot queries and results for Telegram clients.
//!
//! Based on TDLib's `InlineQueriesManager` from `td/telegram/InlineQueriesManager.h`.
//!
//! ## Overview
//!
//! The inline queries manager handles:
//! - Sending and receiving inline bot queries
//! - Caching query results with configurable cache time
//! - Managing recent inline bots
//! - Handling prepared inline messages
//! - Processing web view queries
//! - Weather queries via inline bots
//!
//! ## Architecture
//!
//! The manager maintains:
//! - Query result cache with expiration times
//! - Inline message content storage
//! - Recent inline bots list (max 20)
//! - Pending query queue with rate limiting (400ms delay)
//!
//! ## Examples
//!
//! ```rust
//! use rustgram_inline_queries_manager::InlineQueriesManager;
//! use rustgram_types::UserId;
//! use rustgram_dialog_id::DialogId;
//! use rustgram_venue::Location;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create manager
//! let manager = InlineQueriesManager::new();
//!
//! // Send inline query
//! let bot_id = UserId::new(12345678)?;
//! let dialog_id = DialogId::new(123456);
//! let location = Location::empty();
//!
//! // Note: In real usage, this would return actual query results
//! // let results = manager.send_inline_query(bot_id, dialog_id, location, "query", "").await?;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

pub mod error;

use crate::error::{Error, Result};
use rustgram_dialog_id::DialogId;
use rustgram_types::UserId;
use rustgram_venue::Location;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Maximum number of recent inline bots to track.
///
/// TDLib reference: `InlineQueriesManager.h:97`
const MAX_RECENT_INLINE_BOTS: usize = 20;

/// Inline query delay in milliseconds.
///
/// This is the server-side rate limit for inline queries.
/// TDLib reference: `InlineQueriesManager.h:98`
#[allow(dead_code)]
const INLINE_QUERY_DELAY_MS: u64 = 400;

/// Default cache time for inline query results (in seconds).
#[allow(dead_code)]
const DEFAULT_CACHE_TIME: i32 = 300;

/// Represents a single inline query result from a bot.
///
/// TDLib reference: `td_api::inlineQueryResults`
#[derive(Debug, Clone, PartialEq)]
pub struct InlineQueryResults {
    /// The unique identifier of the inline query.
    pub query_id: i64,
    /// The button to be shown above inline query results.
    pub button: Option<InlineQueryResultsButton>,
    /// The results of the query.
    pub results: Vec<InlineQueryResult>,
    /// The cache time for the results.
    pub cache_time: i32,
    /// The offset for the next query.
    pub next_offset: String,
    /// Target dialog types mask.
    pub target_dialog_types_mask: i64,
}

impl InlineQueryResults {
    /// Creates new inline query results.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The unique query identifier
    /// * `results` - The query results
    /// * `cache_time` - Cache time in seconds
    /// * `next_offset` - Offset for next query
    #[must_use]
    pub const fn new(
        query_id: i64,
        results: Vec<InlineQueryResult>,
        cache_time: i32,
        next_offset: String,
    ) -> Self {
        Self {
            query_id,
            button: None,
            results,
            cache_time,
            next_offset,
            target_dialog_types_mask: 0,
        }
    }

    /// Creates empty inline query results.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            query_id: 0,
            button: None,
            results: Vec::new(),
            cache_time: 0,
            next_offset: String::new(),
            target_dialog_types_mask: 0,
        }
    }

    /// Returns true if there are no results.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Returns the number of results.
    #[must_use]
    pub fn len(&self) -> usize {
        self.results.len()
    }
}

/// Button to be shown above inline query results.
///
/// TDLib reference: `td_api::inlineQueryResultsButton`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InlineQueryResultsButton {
    /// The text of the button.
    pub text: String,
    /// URL to be opened when the button is clicked.
    pub url: Option<String>,
    /// Inline identifier of the inline keyboard button.
    pub inline_data: Option<String>,
}

impl InlineQueryResultsButton {
    /// Creates a new inline query results button.
    ///
    /// # Arguments
    ///
    /// * `text` - The button text
    #[must_use]
    pub fn new(text: String) -> Self {
        Self {
            text,
            url: None,
            inline_data: None,
        }
    }

    /// Creates a URL button.
    ///
    /// # Arguments
    ///
    /// * `text` - The button text
    /// * `url` - The URL to open
    #[must_use]
    pub fn with_url(text: String, url: String) -> Self {
        Self {
            text,
            url: Some(url),
            inline_data: None,
        }
    }

    /// Creates an inline data button.
    ///
    /// # Arguments
    ///
    /// * `text` - The button text
    /// * `inline_data` - The inline data
    #[must_use]
    pub fn with_inline_data(text: String, inline_data: String) -> Self {
        Self {
            text,
            url: None,
            inline_data: Some(inline_data),
        }
    }
}

/// A single inline query result.
///
/// TDLib reference: `td_api::InlineQueryResult`
#[derive(Debug, Clone, PartialEq)]
pub struct InlineQueryResult {
    /// Unique identifier of the result.
    pub id: String,
    /// Title of the result.
    pub title: Option<String>,
    /// Description of the result.
    pub description: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Inline message content.
    pub message: InlineMessageContent,
}

impl InlineQueryResult {
    /// Creates a new inline query result.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `message` - The message content
    #[must_use]
    pub fn new(id: String, message: InlineMessageContent) -> Self {
        Self {
            id,
            title: None,
            description: None,
            url: None,
            message,
        }
    }

    /// Sets the title of the result.
    #[must_use]
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Sets the description of the result.
    #[must_use]
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the URL of the result.
    #[must_use]
    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }
}

/// Content of an inline message.
///
/// TDLib reference: `td_api::InputInlineQueryResult`
#[derive(Debug, Clone, PartialEq)]
pub struct InlineMessageContent {
    /// The message text.
    pub text: String,
    /// Whether the message can be edited.
    pub is_editable: bool,
}

impl InlineMessageContent {
    /// Creates a new inline message content.
    ///
    /// # Arguments
    ///
    /// * `text` - The message text
    #[must_use]
    pub fn new(text: String) -> Self {
        Self {
            text,
            is_editable: true,
        }
    }

    /// Sets whether the message is editable.
    #[must_use]
    pub fn with_editable(mut self, editable: bool) -> Self {
        self.is_editable = editable;
        self
    }
}

/// Prepared inline message identifier.
///
/// TDLib reference: `td_api::preparedInlineMessageId`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PreparedInlineMessageId(String);

impl PreparedInlineMessageId {
    /// Creates a new prepared inline message ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID string
    #[must_use]
    pub fn new(id: String) -> Self {
        Self(id)
    }

    /// Returns the inner ID string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Converts into the inner string.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<String> for PreparedInlineMessageId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl AsRef<str> for PreparedInlineMessageId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Prepared inline message.
///
/// TDLib reference: `td_api::preparedInlineMessage`
#[derive(Debug, Clone, PartialEq)]
pub struct PreparedInlineMessage {
    /// The prepared message ID.
    pub id: PreparedInlineMessageId,
    /// The inline message content.
    pub message: InlineMessageContent,
}

impl PreparedInlineMessage {
    /// Creates a new prepared inline message.
    ///
    /// # Arguments
    ///
    /// * `id` - The message ID
    /// * `message` - The message content
    #[must_use]
    pub const fn new(id: PreparedInlineMessageId, message: InlineMessageContent) -> Self {
        Self { id, message }
    }
}

/// Current weather information.
///
/// TDLib reference: `td_api::currentWeather`
#[derive(Debug, Clone, PartialEq)]
pub struct CurrentWeather {
    /// Temperature in Celsius.
    pub temperature: f32,
    /// Weather description.
    pub description: String,
}

impl CurrentWeather {
    /// Creates a new current weather object.
    ///
    /// # Arguments
    ///
    /// * `temperature` - Temperature in Celsius
    /// * `description` - Weather description
    #[must_use]
    pub const fn new(temperature: f32, description: String) -> Self {
        Self {
            temperature,
            description,
        }
    }
}

/// Cached inline query result with expiration.
///
/// TDLib reference: `InlineQueriesManager.h:171-177`
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CachedInlineQueryResult {
    /// The query results.
    results: InlineQueryResults,
    /// Cache expiration time.
    expire_time: std::time::Instant,
    /// Pending request count.
    pending_requests: i32,
}

/// Manages inline bot queries and results.
///
/// This manager handles sending inline queries to bots, caching results,
/// and managing recent inline bot usage.
///
/// TDLib reference: `td::InlineQueriesManager` from `InlineQueriesManager.h`
#[derive(Debug, Clone)]
pub struct InlineQueriesManager {
    /// Query result cache.
    query_cache: Arc<RwLock<HashMap<u64, CachedInlineQueryResult>>>,
    /// Inline message content cache.
    message_content_cache: Arc<RwLock<HashMap<i64, HashMap<String, InlineMessageContent>>>>,
    /// Query ID to bot user ID mapping.
    query_bot_map: Arc<RwLock<HashMap<i64, UserId>>>,
    /// Recent inline bots.
    recent_bots: Arc<RwLock<Vec<UserId>>>,
}

impl InlineQueriesManager {
    /// Creates a new inline queries manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_inline_queries_manager::InlineQueriesManager;
    ///
    /// let manager = InlineQueriesManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            query_cache: Arc::new(RwLock::new(HashMap::new())),
            message_content_cache: Arc::new(RwLock::new(HashMap::new())),
            query_bot_map: Arc::new(RwLock::new(HashMap::new())),
            recent_bots: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Sends an inline query to a bot.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - The bot user ID
    /// * `dialog_id` - The dialog ID
    /// * `user_location` - The user's location
    /// * `query` - The query string
    /// * `offset` - The offset string
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The bot user ID is invalid
    /// - The dialog ID is invalid
    ///
    /// TDLib reference: `InlineQueriesManager.h:66-67`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_inline_queries_manager::InlineQueriesManager;
    /// use rustgram_types::UserId;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_venue::Location;
    /// # use rustgram_types::TypeError;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = InlineQueriesManager::new();
    /// let bot_id = UserId::new(12345678)?;
    /// let dialog_id = DialogId::new(123456);
    /// let location = Location::empty();
    ///
    /// // In production, this would send the actual query
    /// // let results = manager.send_inline_query(bot_id, dialog_id, location, "test", "").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_inline_query(
        &self,
        bot_user_id: UserId,
        dialog_id: DialogId,
        _user_location: Location,
        _query: String,
        _offset: String,
    ) -> Result<InlineQueryResults> {
        if !bot_user_id.is_valid() {
            return Err(Error::InvalidUserId(bot_user_id));
        }

        if !dialog_id.is_valid() {
            return Err(Error::InvalidDialogId(dialog_id));
        }

        // Update bot usage
        self.update_bot_usage(bot_user_id).await;

        // In a real implementation, this would send the query to the server
        // For now, return empty results
        Ok(InlineQueryResults::empty())
    }

    /// Answers an inline query.
    ///
    /// # Arguments
    ///
    /// * `inline_query_id` - The inline query ID
    /// * `is_personal` - Whether results are personal
    /// * `button` - Optional button to show
    /// * `results` - The query results
    /// * `cache_time` - Cache time in seconds
    /// * `next_offset` - Next offset string
    ///
    /// # Errors
    ///
    /// Returns an error if the query ID is invalid.
    ///
    /// TDLib reference: `InlineQueriesManager.h:43-46`
    pub async fn answer_inline_query(
        &self,
        inline_query_id: i64,
        _is_personal: bool,
        _button: Option<InlineQueryResultsButton>,
        results: Vec<InlineQueryResult>,
        cache_time: i32,
        next_offset: String,
    ) -> Result<()> {
        if inline_query_id <= 0 {
            return Err(Error::InvalidQueryId(inline_query_id));
        }

        // Cache the results
        let mut cache = self.query_cache.write().await;
        let query_hash = self.calculate_query_hash(inline_query_id);
        cache.insert(
            query_hash,
            CachedInlineQueryResult {
                results: InlineQueryResults::new(
                    inline_query_id,
                    results,
                    cache_time.max(0),
                    next_offset,
                ),
                expire_time: std::time::Instant::now()
                    + std::time::Duration::from_secs(cache_time.max(0) as u64),
                pending_requests: 0,
            },
        );

        // In a real implementation, this would send the answer to the server
        Ok(())
    }

    /// Gets recent inline bots.
    ///
    /// Returns a list of recently used inline bot user IDs.
    ///
    /// TDLib reference: `InlineQueriesManager.h:69`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_inline_queries_manager::InlineQueriesManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = InlineQueriesManager::new();
    /// let bots = manager.get_recent_inline_bots().await;
    /// assert!(bots.is_empty());
    /// # }
    /// ```
    pub async fn get_recent_inline_bots(&self) -> Vec<UserId> {
        self.recent_bots.read().await.clone()
    }

    /// Removes a bot from recent inline bots.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - The bot user ID to remove
    ///
    /// # Errors
    ///
    /// Returns an error if the bot user ID is invalid.
    ///
    /// TDLib reference: `InlineQueriesManager.h:71`
    pub async fn remove_recent_inline_bot(&self, bot_user_id: UserId) -> Result<()> {
        if !bot_user_id.is_valid() {
            return Err(Error::InvalidUserId(bot_user_id));
        }

        let mut bots = self.recent_bots.write().await;
        bots.retain(|&id| id != bot_user_id);
        Ok(())
    }

    /// Gets inline message content.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The query ID
    /// * `result_id` - The result ID
    ///
    /// Returns the inline message content if found.
    ///
    /// TDLib reference: `InlineQueriesManager.h:73`
    pub async fn get_inline_message_content(
        &self,
        query_id: i64,
        result_id: &str,
    ) -> Option<InlineMessageContent> {
        let cache = self.message_content_cache.read().await;
        cache.get(&query_id)?.get(result_id).cloned()
    }

    /// Gets the bot user ID for a query.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The query ID
    ///
    /// Returns the bot user ID if found.
    ///
    /// TDLib reference: `InlineQueriesManager.h:75`
    pub async fn get_inline_bot_user_id(&self, query_id: i64) -> Option<UserId> {
        let map = self.query_bot_map.read().await;
        map.get(&query_id).copied()
    }

    /// Clears expired query results from the cache.
    pub async fn clear_expired_results(&self) {
        let mut cache = self.query_cache.write().await;
        let now = std::time::Instant::now();
        cache.retain(|_, result| result.expire_time > now);
    }

    /// Calculates a query hash for caching.
    fn calculate_query_hash(&self, query_id: i64) -> u64 {
        query_id as u64
    }

    /// Updates bot usage tracking.
    async fn update_bot_usage(&self, bot_user_id: UserId) {
        let mut bots = self.recent_bots.write().await;

        // Remove if already exists
        bots.retain(|&id| id != bot_user_id);

        // Add to front
        bots.insert(0, bot_user_id);

        // Limit to MAX_RECENT_INLINE_BOTS
        if bots.len() > MAX_RECENT_INLINE_BOTS {
            bots.truncate(MAX_RECENT_INLINE_BOTS);
        }
    }
}

impl Default for InlineQueriesManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_manager() -> InlineQueriesManager {
        InlineQueriesManager::new()
    }

    #[tokio::test]
    async fn test_manager_creation() {
        let manager = create_test_manager();
        assert_eq!(manager.get_recent_inline_bots().await.len(), 0);
    }

    #[tokio::test]
    async fn test_default_manager() {
        let manager = InlineQueriesManager::default();
        assert_eq!(manager.get_recent_inline_bots().await.len(), 0);
    }

    #[test]
    fn test_inline_query_results_new() {
        let results = InlineQueryResults::new(123, vec![], 300, "next_offset".to_string());
        assert_eq!(results.query_id, 123);
        assert!(results.is_empty());
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_inline_query_results_empty() {
        let results = InlineQueryResults::empty();
        assert!(results.is_empty());
        assert_eq!(results.query_id, 0);
    }

    #[test]
    fn test_inline_query_results_button_new() {
        let button = InlineQueryResultsButton::new("Click me".to_string());
        assert_eq!(button.text, "Click me");
        assert!(button.url.is_none());
        assert!(button.inline_data.is_none());
    }

    #[test]
    fn test_inline_query_results_button_with_url() {
        let button = InlineQueryResultsButton::with_url(
            "Open".to_string(),
            "https://example.com".to_string(),
        );
        assert_eq!(button.text, "Open");
        assert_eq!(button.url.as_deref(), Some("https://example.com"));
    }

    #[test]
    fn test_inline_query_results_button_with_inline_data() {
        let button =
            InlineQueryResultsButton::with_inline_data("Action".to_string(), "data".to_string());
        assert_eq!(button.text, "Action");
        assert_eq!(button.inline_data.as_deref(), Some("data"));
    }

    #[test]
    fn test_inline_query_result_new() {
        let message = InlineMessageContent::new("Test message".to_string());
        let result = InlineQueryResult::new("result_1".to_string(), message);
        assert_eq!(result.id, "result_1");
        assert!(result.title.is_none());
        assert!(result.description.is_none());
        assert!(result.url.is_none());
    }

    #[test]
    fn test_inline_query_result_builders() {
        let message = InlineMessageContent::new("Test".to_string());
        let result = InlineQueryResult::new("result_1".to_string(), message)
            .with_title("Test Title".to_string())
            .with_description("Test Description".to_string())
            .with_url("https://example.com".to_string());

        assert_eq!(result.title.as_deref(), Some("Test Title"));
        assert_eq!(result.description.as_deref(), Some("Test Description"));
        assert_eq!(result.url.as_deref(), Some("https://example.com"));
    }

    #[test]
    fn test_inline_message_content_new() {
        let content = InlineMessageContent::new("Message".to_string());
        assert_eq!(content.text, "Message");
        assert!(content.is_editable);
    }

    #[test]
    fn test_inline_message_content_with_editable() {
        let content = InlineMessageContent::new("Message".to_string()).with_editable(false);
        assert!(!content.is_editable);
    }

    #[test]
    fn test_prepared_inline_message_id_new() {
        let id = PreparedInlineMessageId::new("test_id".to_string());
        assert_eq!(id.as_str(), "test_id");
        assert_eq!(id.into_inner(), "test_id");
    }

    #[test]
    fn test_prepared_inline_message_id_from_string() {
        let id: PreparedInlineMessageId = "test_id".to_string().into();
        assert_eq!(id.as_ref(), "test_id");
    }

    #[test]
    fn test_prepared_inline_message_new() {
        let id = PreparedInlineMessageId::new("msg_id".to_string());
        let message = InlineMessageContent::new("Test".to_string());
        let prepared = PreparedInlineMessage::new(id, message);
        assert_eq!(prepared.id.as_str(), "msg_id");
    }

    #[test]
    fn test_current_weather_new() {
        let weather = CurrentWeather::new(20.5, "Sunny".to_string());
        assert_eq!(weather.temperature, 20.5);
        assert_eq!(weather.description, "Sunny");
    }

    #[tokio::test]
    async fn test_send_inline_query_invalid_user_id() {
        let manager = create_test_manager();
        let invalid_id = UserId::new(0).unwrap_or_else(|_| UserId::default());
        let dialog_id = DialogId::new(123456);
        let location = Location::empty();

        let result = manager
            .send_inline_query(
                invalid_id,
                dialog_id,
                location,
                "test".to_string(),
                "".to_string(),
            )
            .await;
        assert!(matches!(result, Err(Error::InvalidUserId(_))));
    }

    #[tokio::test]
    async fn test_send_inline_query_invalid_dialog_id() {
        let manager = create_test_manager();
        let bot_id = UserId::new(12345678).unwrap();
        let invalid_dialog = DialogId::new(0);
        let location = Location::empty();

        let result = manager
            .send_inline_query(
                bot_id,
                invalid_dialog,
                location,
                "test".to_string(),
                "".to_string(),
            )
            .await;
        assert!(matches!(result, Err(Error::InvalidDialogId(_))));
    }

    #[tokio::test]
    async fn test_send_inline_query_success() {
        let manager = create_test_manager();
        let bot_id = UserId::new(12345678).unwrap();
        let dialog_id = DialogId::new(123456);
        let location = Location::empty();

        let result = manager
            .send_inline_query(
                bot_id,
                dialog_id,
                location,
                "test".to_string(),
                "".to_string(),
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_answer_inline_query_invalid_id() {
        let manager = create_test_manager();
        let result = manager
            .answer_inline_query(-1, false, None, vec![], 300, "".to_string())
            .await;
        assert!(matches!(result, Err(Error::InvalidQueryId(_))));
    }

    #[tokio::test]
    async fn test_answer_inline_query_success() {
        let manager = create_test_manager();
        let results = vec![InlineQueryResult::new(
            "result_1".to_string(),
            InlineMessageContent::new("Test".to_string()),
        )];

        let result = manager
            .answer_inline_query(123, false, None, results, 300, "".to_string())
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_recent_bots_initially_empty() {
        let manager = create_test_manager();
        let bots = manager.get_recent_inline_bots().await;
        assert!(bots.is_empty());
    }

    #[tokio::test]
    async fn test_update_bot_usage() {
        let manager = create_test_manager();
        let bot_id = UserId::new(12345678).unwrap();
        let dialog_id = DialogId::new(123456);
        let location = Location::empty();

        // Send query to update bot usage
        let _ = manager
            .send_inline_query(
                bot_id,
                dialog_id,
                location,
                "test".to_string(),
                "".to_string(),
            )
            .await;

        let bots = manager.get_recent_inline_bots().await;
        assert_eq!(bots.len(), 1);
        assert_eq!(bots[0], bot_id);
    }

    #[tokio::test]
    async fn test_remove_recent_inline_bot() {
        let manager = create_test_manager();
        let bot_id = UserId::new(12345678).unwrap();
        let dialog_id = DialogId::new(123456);
        let location = Location::empty();

        // Add bot
        let _ = manager
            .send_inline_query(
                bot_id,
                dialog_id,
                location,
                "test".to_string(),
                "".to_string(),
            )
            .await;

        // Remove bot
        let result = manager.remove_recent_inline_bot(bot_id).await;
        assert!(result.is_ok());

        let bots = manager.get_recent_inline_bots().await;
        assert!(bots.is_empty());
    }

    #[tokio::test]
    async fn test_remove_recent_inline_bot_invalid_id() {
        let manager = create_test_manager();
        let invalid_id = UserId::new(0).unwrap_or_else(|_| UserId::default());

        let result = manager.remove_recent_inline_bot(invalid_id).await;
        assert!(matches!(result, Err(Error::InvalidUserId(_))));
    }

    #[tokio::test]
    async fn test_get_inline_message_content_not_found() {
        let manager = create_test_manager();
        let content = manager.get_inline_message_content(123, "result_1").await;
        assert!(content.is_none());
    }

    #[tokio::test]
    async fn test_get_inline_bot_user_id_not_found() {
        let manager = create_test_manager();
        let bot_id = manager.get_inline_bot_user_id(123).await;
        assert!(bot_id.is_none());
    }

    #[tokio::test]
    async fn test_clear_expired_results() {
        let manager = create_test_manager();
        // Should not panic even with empty cache
        manager.clear_expired_results().await;
    }

    #[test]
    fn test_max_recent_inline_bots_const() {
        assert_eq!(MAX_RECENT_INLINE_BOTS, 20);
    }

    #[test]
    fn test_inline_query_delay_ms_const() {
        assert_eq!(INLINE_QUERY_DELAY_MS, 400);
    }

    #[test]
    fn test_default_cache_time_const() {
        assert_eq!(DEFAULT_CACHE_TIME, 300);
    }
}
