// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Statistics Manager
//!
//! Manager for fetching and managing Telegram statistics data for channels,
//! messages, and stories.
//!
//! ## Overview
//!
//! The `StatisticsManager` provides async methods to fetch various statistics
//! from Telegram, including:
//!
//! - Channel/supergroup statistics (members, messages, views)
//! - Message statistics (views, reactions)
//! - Story statistics (views, reactions)
//! - Revenue statistics (TON revenue, transactions)
//! - Statistical graphs (async graph data loading)
//! - Public forwards (who forwarded a message/story)
//!
//! ## Usage
//!
//! ```rust,no_run
//! use rustgram_statistics_manager::{StatisticsManager, StoryFullId, MessageFullId};
//! use rustgram_types::{DialogId, MessageId, UserId};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let manager = StatisticsManager::new();
//!
//!     // Fetch channel statistics
//!     let dialog_id = DialogId::User(rustgram_types::UserId(123456));
//!     let stats = manager.get_channel_statistics(dialog_id, false).await?;
//!
//!     // Fetch message statistics
//!     let user_id = UserId::new(123)?;
//!     let msg_dialog_id = DialogId::from_user(user_id);
//!     let message_id = MessageId::from_server_id(789);
//!     let msg_full_id = MessageFullId::new(msg_dialog_id, message_id);
//!     let msg_stats = manager.get_channel_message_statistics(msg_full_id, false).await?;
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(unused_imports)] // MessageId is used in doc examples and tests

use error::Result;
use rustgram_types::{DialogId, DialogType, MessageId};
use std::collections::HashMap;
use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Type alias for the channel statistics cache.
type ChannelCache = Arc<RwLock<HashMap<i64, CacheEntry<ChatStatistics>>>>;

/// Type alias for the message statistics cache.
type MessageCache = Arc<RwLock<HashMap<(i64, i32), CacheEntry<MessageStatistics>>>>;

/// Type alias for the story statistics cache.
type StoryCache = Arc<RwLock<HashMap<(i64, i32), CacheEntry<StoryStatistics>>>>;

/// Type alias for the revenue statistics cache.
type RevenueCache = Arc<RwLock<HashMap<i64, CacheEntry<RevenueStatistics>>>>;

/// Type alias for the graph data cache.
type GraphCache = Arc<RwLock<HashMap<String, CacheEntry<StatisticalGraph>>>>;

/// Type alias for the public forwards cache.
type ForwardsCache = Arc<RwLock<HashMap<String, CacheEntry<PublicForwards>>>>;
use tracing::{debug, info, warn};

pub mod error;
pub mod types;

pub use types::{
    AdministratorActionsInfo, ChatInteractionInfo, ChatStatistics, ChatStatisticsChannel,
    ChatStatisticsSupergroup, DateRange, InviterInfo, MessageFullId, MessageSenderInfo,
    MessageStatistics, PublicForward, PublicForwards, RevenueStatistics, RevenueTransaction,
    RevenueTransactionType, RevenueTransactions, RevenueWithdrawalState, StatisticalGraph,
    StatisticalValue, StoryFullId, StoryStatistics,
};

/// Default TTL for statistics cache (5 minutes).
const DEFAULT_CACHE_TTL: Duration = Duration::from_secs(300);

/// Default TTL for graph cache (2 minutes).
const GRAPH_CACHE_TTL: Duration = Duration::from_secs(120);

/// Cache entry with expiration time.
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    /// Cached data.
    data: T,
    /// Expiration time.
    expires_at: Instant,
}

impl<T> CacheEntry<T> {
    /// Creates a new cache entry.
    fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
            expires_at: Instant::now().add(ttl),
        }
    }

    /// Checks if the entry is expired.
    fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }
}

/// Manager for Telegram statistics data.
///
/// This manager provides async methods to fetch various statistics from
/// Telegram, including channel statistics, message statistics, story
/// statistics, revenue statistics, and public forwards.
///
/// # Thread Safety
///
/// The manager uses `Arc<RwLock<T>>` for internal state, making it safe
/// to share across threads.
///
/// # Caching
///
/// Statistics data is cached with a configurable TTL (default: 5 minutes).
/// Graph data has a shorter cache TTL (default: 2 minutes).
#[derive(Debug, Clone)]
pub struct StatisticsManager {
    /// Channel statistics cache.
    channel_cache: ChannelCache,
    /// Message statistics cache.
    message_cache: MessageCache,
    /// Story statistics cache.
    story_cache: StoryCache,
    /// Revenue statistics cache.
    revenue_cache: RevenueCache,
    /// Graph data cache.
    graph_cache: GraphCache,
    /// Public forwards cache.
    forwards_cache: ForwardsCache,
    /// Cache TTL for statistics.
    cache_ttl: Duration,
    /// Cache TTL for graphs.
    graph_cache_ttl: Duration,
}

impl StatisticsManager {
    /// Creates a new statistics manager.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_statistics_manager::StatisticsManager;
    ///
    /// let manager = StatisticsManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::with_cache_ttl(DEFAULT_CACHE_TTL, GRAPH_CACHE_TTL)
    }

    /// Creates a new statistics manager with custom cache TTL.
    ///
    /// # Arguments
    ///
    /// * `cache_ttl` - TTL for statistics cache
    /// * `graph_cache_ttl` - TTL for graph cache
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_statistics_manager::StatisticsManager;
    /// use std::time::Duration;
    ///
    /// let manager = StatisticsManager::with_cache_ttl(
    ///     Duration::from_secs(600),
    ///     Duration::from_secs(180),
    /// );
    /// ```
    #[must_use]
    pub fn with_cache_ttl(cache_ttl: Duration, graph_cache_ttl: Duration) -> Self {
        Self {
            channel_cache: Arc::new(RwLock::new(HashMap::new())),
            message_cache: Arc::new(RwLock::new(HashMap::new())),
            story_cache: Arc::new(RwLock::new(HashMap::new())),
            revenue_cache: Arc::new(RwLock::new(HashMap::new())),
            graph_cache: Arc::new(RwLock::new(HashMap::new())),
            forwards_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_ttl,
            graph_cache_ttl,
        }
    }

    /// Clears all caches.
    ///
    /// This method removes all cached statistics data, forcing the next
    /// request to fetch fresh data from the server.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustgram_statistics_manager::StatisticsManager;
    /// # let manager = StatisticsManager::new();
    /// manager.clear_cache();
    /// ```
    pub async fn clear_cache(&self) {
        self.channel_cache.write().await.clear();
        self.message_cache.write().await.clear();
        self.story_cache.write().await.clear();
        self.revenue_cache.write().await.clear();
        self.graph_cache.write().await.clear();
        self.forwards_cache.write().await.clear();
        info!("All statistics caches cleared");
    }

    /// Clears expired cache entries.
    ///
    /// This method removes expired entries from all caches without affecting
    /// valid entries.
    pub async fn clear_expired_cache(&self) {
        let mut channel_cache = self.channel_cache.write().await;
        channel_cache.retain(|_, entry| !entry.is_expired());

        let mut message_cache = self.message_cache.write().await;
        message_cache.retain(|_, entry| !entry.is_expired());

        let mut story_cache = self.story_cache.write().await;
        story_cache.retain(|_, entry| !entry.is_expired());

        let mut revenue_cache = self.revenue_cache.write().await;
        revenue_cache.retain(|_, entry| !entry.is_expired());

        let mut graph_cache = self.graph_cache.write().await;
        graph_cache.retain(|_, entry| !entry.is_expired());

        let mut forwards_cache = self.forwards_cache.write().await;
        forwards_cache.retain(|_, entry| !entry.is_expired());

        debug!("Expired cache entries cleared");
    }

    /// Fetches channel statistics.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID of the channel
    /// * `is_dark` - Whether to use dark color scheme for graphs
    ///
    /// # Returns
    ///
    /// Returns `ChatStatistics` containing either supergroup or channel
    /// statistics.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidDialog` if the dialog is not a channel or
    /// doesn't exist.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use rustgram_statistics_manager::StatisticsManager;
    /// # use rustgram_types::DialogId;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = StatisticsManager::new();
    /// let dialog_id = DialogId::User(rustgram_types::UserId(123456));
    /// let stats = manager.get_channel_statistics(dialog_id, false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_channel_statistics(
        &self,
        dialog_id: DialogId,
        _is_dark: bool,
    ) -> Result<ChatStatistics> {
        info!(
            "Fetching channel statistics for dialog_id={}",
            dialog_id.to_encoded()
        );

        // Check if this is a valid channel
        if dialog_id.get_type() != DialogType::Channel {
            warn!(
                "Invalid dialog type for channel statistics: {:?}",
                dialog_id.get_type()
            );
            return Err(error::Error::InvalidDialog);
        }

        let id = dialog_id.to_encoded();

        // Check cache
        {
            let cache = self.channel_cache.read().await;
            if let Some(entry) = cache.get(&id) {
                if !entry.is_expired() {
                    debug!("Using cached channel statistics");
                    return Ok(entry.data.clone());
                }
            }
        }

        // In a real implementation, this would make a network request
        // For now, return a stub response
        let period = DateRange::new(1609459200, 1609545600);
        let stats = ChatStatistics::Channel(ChatStatisticsChannel::new(
            period.clone(),
            StatisticalValue::new(10000.0, 9500.0, 5.26),
            StatisticalValue::new(5000.0, 4500.0, 11.11),
            StatisticalValue::new(200.0, 180.0, 11.11),
            StatisticalValue::new(100.0, 90.0, 11.11),
        ));

        // Cache the result
        let mut cache = self.channel_cache.write().await;
        cache.insert(id, CacheEntry::new(stats.clone(), self.cache_ttl));

        Ok(stats)
    }

    /// Fetches message statistics for a channel message.
    ///
    /// # Arguments
    ///
    /// * `message_full_id` - The full message ID
    /// * `is_dark` - Whether to use dark color scheme for graphs
    ///
    /// # Returns
    ///
    /// Returns `MessageStatistics` containing views and reactions graphs.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidMessage` if the message doesn't exist or
    /// is not in a channel.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use rustgram_statistics_manager::{StatisticsManager, MessageFullId};
    /// # use rustgram_types::{DialogId, MessageId, UserId};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = StatisticsManager::new();
    /// let dialog_id = DialogId::User(rustgram_types::UserId(123456));
    /// let message_id = MessageId::from_server_id(456);
    /// let msg_full_id = MessageFullId::new(dialog_id, message_id);
    /// let stats = manager.get_channel_message_statistics(msg_full_id, false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_channel_message_statistics(
        &self,
        message_full_id: MessageFullId,
        _is_dark: bool,
    ) -> Result<MessageStatistics> {
        info!(
            "Fetching message statistics for message_full_id={:?}",
            message_full_id
        );

        let dialog_id = message_full_id.dialog_id().to_encoded();
        let message_id = message_full_id.message_id();

        // Check if message ID is valid
        if !message_id.is_valid() {
            return Err(error::Error::InvalidMessage);
        }

        // Check cache
        let cache_key = (dialog_id, message_id.get_server_id());
        {
            let cache = self.message_cache.read().await;
            if let Some(entry) = cache.get(&cache_key) {
                if !entry.is_expired() {
                    debug!("Using cached message statistics");
                    return Ok(entry.data.clone());
                }
            }
        }

        // In a real implementation, this would make a network request
        let stats = MessageStatistics::new(
            StatisticalGraph::async_graph("views_token".to_string()),
            StatisticalGraph::async_graph("reactions_token".to_string()),
        );

        // Cache the result
        let mut cache = self.message_cache.write().await;
        cache.insert(cache_key, CacheEntry::new(stats.clone(), self.cache_ttl));

        Ok(stats)
    }

    /// Fetches story statistics for a channel story.
    ///
    /// # Arguments
    ///
    /// * `story_full_id` - The full story ID
    /// * `is_dark` - Whether to use dark color scheme for graphs
    ///
    /// # Returns
    ///
    /// Returns `StoryStatistics` containing views and reactions graphs.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidStory` if the story doesn't exist.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use rustgram_statistics_manager::{StatisticsManager, StoryFullId};
    /// # use rustgram_types::DialogId;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = StatisticsManager::new();
    /// let dialog_id = DialogId::User(rustgram_types::UserId(123456));
    /// let story_full_id = StoryFullId::new(dialog_id.to_encoded(), 789);
    /// let stats = manager.get_channel_story_statistics(story_full_id, false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_channel_story_statistics(
        &self,
        story_full_id: StoryFullId,
        _is_dark: bool,
    ) -> Result<StoryStatistics> {
        info!(
            "Fetching story statistics for story_full_id={}",
            story_full_id
        );

        // Check if story ID is valid
        if !story_full_id.is_valid() {
            return Err(error::Error::InvalidStory);
        }

        let cache_key = (story_full_id.dialog_id, story_full_id.story_id);

        // Check cache
        {
            let cache = self.story_cache.read().await;
            if let Some(entry) = cache.get(&cache_key) {
                if !entry.is_expired() {
                    debug!("Using cached story statistics");
                    return Ok(entry.data.clone());
                }
            }
        }

        // In a real implementation, this would make a network request
        let stats = StoryStatistics::new(
            StatisticalGraph::async_graph("views_token".to_string()),
            StatisticalGraph::async_graph("reactions_token".to_string()),
        );

        // Cache the result
        let mut cache = self.story_cache.write().await;
        cache.insert(cache_key, CacheEntry::new(stats.clone(), self.cache_ttl));

        Ok(stats)
    }

    /// Fetches revenue statistics for a channel.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID of the channel
    /// * `is_dark` - Whether to use dark color scheme for graphs
    ///
    /// # Returns
    ///
    /// Returns `RevenueStatistics` containing revenue and transaction data.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidDialog` if the dialog is not a channel.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use rustgram_statistics_manager::StatisticsManager;
    /// # use rustgram_types::DialogId;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = StatisticsManager::new();
    /// let dialog_id = DialogId::User(rustgram_types::UserId(123456));
    /// let stats = manager.get_dialog_revenue_statistics(dialog_id, false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_dialog_revenue_statistics(
        &self,
        dialog_id: DialogId,
        _is_dark: bool,
    ) -> Result<RevenueStatistics> {
        info!(
            "Fetching revenue statistics for dialog_id={}",
            dialog_id.to_encoded()
        );

        let id = dialog_id.to_encoded();

        // Check cache
        {
            let cache = self.revenue_cache.read().await;
            if let Some(entry) = cache.get(&id) {
                if !entry.is_expired() {
                    debug!("Using cached revenue statistics");
                    return Ok(entry.data.clone());
                }
            }
        }

        // In a real implementation, this would make a network request
        let stats = RevenueStatistics::new(10000.0, 5000.0, 3000.0, true, 1.0);

        // Cache the result
        let mut cache = self.revenue_cache.write().await;
        cache.insert(id, CacheEntry::new(stats.clone(), self.cache_ttl));

        Ok(stats)
    }

    /// Gets the withdrawal URL for channel revenue.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID of the channel
    /// * `password` - The user's password for authentication
    ///
    /// # Returns
    ///
    /// Returns the withdrawal URL as a string.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidPassword` if the password is invalid.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use rustgram_statistics_manager::StatisticsManager;
    /// # use rustgram_types::DialogId;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = StatisticsManager::new();
    /// let dialog_id = DialogId::User(rustgram_types::UserId(123456));
    /// let url = manager.get_dialog_revenue_withdrawal_url(dialog_id, "password").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_dialog_revenue_withdrawal_url(
        &self,
        dialog_id: DialogId,
        password: &str,
    ) -> Result<String> {
        info!(
            "Fetching revenue withdrawal URL for dialog_id={}",
            dialog_id.to_encoded()
        );

        if password.is_empty() {
            return Err(error::Error::InvalidPassword);
        }

        // In a real implementation, this would make a network request
        Ok(format!(
            "https://withdrawal.example.com/{}",
            dialog_id.to_encoded()
        ))
    }

    /// Fetches revenue transactions for a channel.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID of the channel
    /// * `offset` - Offset for pagination
    /// * `limit` - Maximum number of transactions to return
    ///
    /// # Returns
    ///
    /// Returns `RevenueTransactions` containing transaction list and pagination info.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidDialog` if the dialog is not a channel.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use rustgram_statistics_manager::StatisticsManager;
    /// # use rustgram_types::DialogId;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = StatisticsManager::new();
    /// let dialog_id = DialogId::User(rustgram_types::UserId(123456));
    /// let txns = manager.get_dialog_revenue_transactions(dialog_id, "", 50).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_dialog_revenue_transactions(
        &self,
        dialog_id: DialogId,
        offset: &str,
        limit: i32,
    ) -> Result<RevenueTransactions> {
        info!(
            "Fetching revenue transactions for dialog_id={}, offset={}, limit={}",
            dialog_id.to_encoded(),
            offset,
            limit
        );

        if limit <= 0 {
            return Err(error::Error::InvalidParameter(
                "limit must be positive".to_string(),
            ));
        }

        // In a real implementation, this would make a network request
        let txns = RevenueTransactions::new(1000.0, Vec::new(), String::new());

        Ok(txns)
    }

    /// Loads detailed statistical graph data using a token.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    /// * `token` - The token from an async graph
    /// * `x` - X coordinate for zooming (0 for no zoom)
    ///
    /// # Returns
    ///
    /// Returns `StatisticalGraph` with the loaded data.
    ///
    /// # Errors
    ///
    /// Returns `Error::GraphNotFound` if the token is invalid.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use rustgram_statistics_manager::StatisticsManager;
    /// # use rustgram_types::DialogId;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = StatisticsManager::new();
    /// let dialog_id = DialogId::User(rustgram_types::UserId(123456));
    /// let graph = manager.load_statistics_graph(dialog_id, "token123".to_string(), 0).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load_statistics_graph(
        &self,
        dialog_id: DialogId,
        token: String,
        _x: i64,
    ) -> Result<StatisticalGraph> {
        info!(
            "Loading graph with token for dialog_id={}",
            dialog_id.to_encoded()
        );

        // Check cache
        {
            let cache = self.graph_cache.read().await;
            if let Some(entry) = cache.get(&token) {
                if !entry.is_expired() {
                    debug!("Using cached graph data");
                    return Ok(entry.data.clone());
                }
            }
        }

        // In a real implementation, this would make a network request
        let graph = StatisticalGraph::data("json_data".to_string(), "zoom_token".to_string());

        // Cache the result
        let mut cache = self.graph_cache.write().await;
        cache.insert(token, CacheEntry::new(graph.clone(), self.graph_cache_ttl));

        Ok(graph)
    }

    /// Fetches public forwards of a message.
    ///
    /// # Arguments
    ///
    /// * `message_full_id` - The full message ID
    /// * `offset` - Offset for pagination
    /// * `limit` - Maximum number of forwards to return
    ///
    /// # Returns
    ///
    /// Returns `PublicForwards` containing the list of forwards.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidMessage` if the message doesn't exist.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use rustgram_statistics_manager::{StatisticsManager, MessageFullId};
    /// # use rustgram_types::{DialogId, MessageId};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = StatisticsManager::new();
    /// let dialog_id = DialogId::User(rustgram_types::UserId(123456));
    /// let message_id = MessageId::from_server_id(456);
    /// let msg_full_id = MessageFullId::new(dialog_id, message_id);
    /// let forwards = manager.get_message_public_forwards(msg_full_id, "".to_string(), 50).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_message_public_forwards(
        &self,
        message_full_id: MessageFullId,
        offset: String,
        limit: i32,
    ) -> Result<PublicForwards> {
        info!(
            "Fetching message public forwards for message_full_id={:?}",
            message_full_id
        );

        if limit <= 0 {
            return Err(error::Error::InvalidParameter(
                "limit must be positive".to_string(),
            ));
        }

        const MAX_MESSAGE_FORWARDS: i32 = 100;
        let _actual_limit = limit.min(MAX_MESSAGE_FORWARDS);

        let cache_key = format!(
            "msg:{}:{}:{}",
            message_full_id.dialog_id().to_encoded(),
            message_full_id.message_id().get_server_id(),
            offset
        );

        // Check cache
        {
            let cache = self.forwards_cache.read().await;
            if let Some(entry) = cache.get(&cache_key) {
                if !entry.is_expired() {
                    debug!("Using cached message forwards");
                    return Ok(entry.data.clone());
                }
            }
        }

        // In a real implementation, this would make a network request
        let forwards = PublicForwards::new(0, Vec::new(), String::new());

        // Cache the result
        let mut cache = self.forwards_cache.write().await;
        cache.insert(cache_key, CacheEntry::new(forwards.clone(), self.cache_ttl));

        Ok(forwards)
    }

    /// Fetches public forwards of a story.
    ///
    /// # Arguments
    ///
    /// * `story_full_id` - The full story ID
    /// * `offset` - Offset for pagination
    /// * `limit` - Maximum number of forwards to return
    ///
    /// # Returns
    ///
    /// Returns `PublicForwards` containing the list of forwards.
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidStory` if the story doesn't exist.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use rustgram_statistics_manager::{StatisticsManager, StoryFullId};
    /// # use rustgram_types::DialogId;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = StatisticsManager::new();
    /// let dialog_id = DialogId::User(rustgram_types::UserId(123456));
    /// let story_full_id = StoryFullId::new(dialog_id.to_encoded(), 789);
    /// let forwards = manager.get_story_public_forwards(story_full_id, "".to_string(), 50).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_story_public_forwards(
        &self,
        story_full_id: StoryFullId,
        offset: String,
        limit: i32,
    ) -> Result<PublicForwards> {
        info!(
            "Fetching story public forwards for story_full_id={}",
            story_full_id
        );

        if limit <= 0 {
            return Err(error::Error::InvalidParameter(
                "limit must be positive".to_string(),
            ));
        }

        const MAX_STORY_FORWARDS: i32 = 100;
        let _actual_limit = limit.min(MAX_STORY_FORWARDS);

        let cache_key = format!(
            "story:{}:{}:{}",
            story_full_id.dialog_id, story_full_id.story_id, offset
        );

        // Check cache
        {
            let cache = self.forwards_cache.read().await;
            if let Some(entry) = cache.get(&cache_key) {
                if !entry.is_expired() {
                    debug!("Using cached story forwards");
                    return Ok(entry.data.clone());
                }
            }
        }

        // In a real implementation, this would make a network request
        let forwards = PublicForwards::new(0, Vec::new(), String::new());

        // Cache the result
        let mut cache = self.forwards_cache.write().await;
        cache.insert(cache_key, CacheEntry::new(forwards.clone(), self.cache_ttl));

        Ok(forwards)
    }

    /// Gets the current cache TTL for statistics.
    #[must_use]
    pub const fn cache_ttl(&self) -> Duration {
        self.cache_ttl
    }

    /// Gets the current cache TTL for graphs.
    #[must_use]
    pub const fn graph_cache_ttl(&self) -> Duration {
        self.graph_cache_ttl
    }
}

impl Default for StatisticsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    fn create_test_dialog_id() -> DialogId {
        DialogId::from_user(rustgram_types::UserId(123))
    }

    fn create_test_message_full_id() -> MessageFullId {
        let dialog_id = DialogId::Channel(rustgram_types::ChannelId(-1000000000));
        let message_id = MessageId::from_server_id(456);
        MessageFullId::new(dialog_id, message_id)
    }

    fn create_test_story_full_id() -> StoryFullId {
        StoryFullId::new(123456, 789)
    }

    #[tokio::test]
    async fn test_manager_creation() {
        let manager = StatisticsManager::new();
        assert_eq!(manager.cache_ttl(), DEFAULT_CACHE_TTL);
        assert_eq!(manager.graph_cache_ttl(), GRAPH_CACHE_TTL);
    }

    #[tokio::test]
    async fn test_manager_with_custom_ttl() {
        let manager =
            StatisticsManager::with_cache_ttl(Duration::from_secs(600), Duration::from_secs(180));
        assert_eq!(manager.cache_ttl(), Duration::from_secs(600));
        assert_eq!(manager.graph_cache_ttl(), Duration::from_secs(180));
    }

    #[tokio::test]
    async fn test_manager_default() {
        let manager = StatisticsManager::default();
        assert_eq!(manager.cache_ttl(), DEFAULT_CACHE_TTL);
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let manager = StatisticsManager::new();

        // Add some data to cache
        let _dialog_id = DialogId::User(rustgram_types::UserId(123456));
        let cache_entry = CacheEntry::new(
            ChatStatistics::Channel(ChatStatisticsChannel::new(
                DateRange::new(1609459200, 1609545600),
                StatisticalValue::new(10000.0, 9500.0, 5.26),
                StatisticalValue::new(5000.0, 4500.0, 11.11),
                StatisticalValue::new(200.0, 180.0, 11.11),
                StatisticalValue::new(100.0, 90.0, 11.11),
            )),
            Duration::from_secs(300),
        );

        {
            let mut cache = manager.channel_cache.write().await;
            cache.insert(123456, cache_entry);
        }

        // Clear cache
        manager.clear_cache().await;

        // Verify cache is empty
        let cache = manager.channel_cache.read().await;
        assert!(cache.is_empty());
    }

    #[tokio::test]
    async fn test_get_channel_statistics_invalid_type() {
        let manager = StatisticsManager::new();
        let dialog_id = create_test_dialog_id(); // User dialog, not channel

        let result = manager.get_channel_statistics(dialog_id, false).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error::Error::InvalidDialog);
    }

    #[tokio::test]
    async fn test_get_channel_statistics_channel() {
        let manager = StatisticsManager::new();
        let dialog_id = DialogId::Channel(rustgram_types::ChannelId(-1000000000)); // Channel ID

        let result = manager.get_channel_statistics(dialog_id, false).await;

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert!(stats.is_channel());
        assert!(!stats.is_supergroup());
    }

    #[tokio::test]
    async fn test_get_channel_statistics_caching() {
        let manager = StatisticsManager::new();
        let dialog_id = DialogId::Channel(rustgram_types::ChannelId(-1000000000)); // Channel ID

        // First call
        let result1 = manager.get_channel_statistics(dialog_id, false).await;
        assert!(result1.is_ok());

        // Second call should use cache
        let result2 = manager.get_channel_statistics(dialog_id, false).await;
        assert!(result2.is_ok());
    }

    #[tokio::test]
    async fn test_get_channel_message_statistics() {
        let manager = StatisticsManager::new();
        let msg_full_id = create_test_message_full_id();

        let result = manager
            .get_channel_message_statistics(msg_full_id, false)
            .await;

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert!(!stats.is_views_loaded());
        assert!(!stats.is_reactions_loaded());
    }

    #[tokio::test]
    async fn test_get_channel_story_statistics() {
        let manager = StatisticsManager::new();
        let story_full_id = create_test_story_full_id();

        let result = manager
            .get_channel_story_statistics(story_full_id, false)
            .await;

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert!(!stats.is_views_loaded());
        assert!(!stats.is_reactions_loaded());
    }

    #[tokio::test]
    async fn test_get_channel_story_statistics_invalid() {
        let manager = StatisticsManager::new();
        let story_full_id = StoryFullId::new(0, 0); // Invalid

        let result = manager
            .get_channel_story_statistics(story_full_id, false)
            .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error::Error::InvalidStory);
    }

    #[tokio::test]
    async fn test_get_dialog_revenue_statistics() {
        let manager = StatisticsManager::new();
        let dialog_id = DialogId::Channel(rustgram_types::ChannelId(-1000000000));

        let result = manager
            .get_dialog_revenue_statistics(dialog_id, false)
            .await;

        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.overall_revenue, 10000.0);
        assert!(stats.can_withdraw());
    }

    #[tokio::test]
    async fn test_get_dialog_revenue_withdrawal_url() {
        let manager = StatisticsManager::new();
        let dialog_id = DialogId::Channel(rustgram_types::ChannelId(-1000000000));

        let result = manager
            .get_dialog_revenue_withdrawal_url(dialog_id, "password")
            .await;

        assert!(result.is_ok());
        let url = result.unwrap();
        assert!(url.contains("withdrawal"));
    }

    #[tokio::test]
    async fn test_get_dialog_revenue_withdrawal_url_empty_password() {
        let manager = StatisticsManager::new();
        let dialog_id = DialogId::Channel(rustgram_types::ChannelId(-1000000000));

        let result = manager
            .get_dialog_revenue_withdrawal_url(dialog_id, "")
            .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error::Error::InvalidPassword);
    }

    #[tokio::test]
    async fn test_get_dialog_revenue_transactions() {
        let manager = StatisticsManager::new();
        let dialog_id = DialogId::Channel(rustgram_types::ChannelId(-1000000000));

        let result = manager
            .get_dialog_revenue_transactions(dialog_id, "", 50)
            .await;

        assert!(result.is_ok());
        let txns = result.unwrap();
        assert_eq!(txns.count(), 0);
        assert!(!txns.has_more());
    }

    #[tokio::test]
    async fn test_get_dialog_revenue_transactions_invalid_limit() {
        let manager = StatisticsManager::new();
        let dialog_id = DialogId::Channel(rustgram_types::ChannelId(-1000000000));

        let result = manager
            .get_dialog_revenue_transactions(dialog_id, "", 0)
            .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            error::Error::InvalidParameter("limit must be positive".to_string())
        );
    }

    #[tokio::test]
    async fn test_load_statistics_graph() {
        let manager = StatisticsManager::new();
        let dialog_id = DialogId::Channel(rustgram_types::ChannelId(-1000000000));

        let result = manager
            .load_statistics_graph(dialog_id, "token123".to_string(), 0)
            .await;

        assert!(result.is_ok());
        let graph = result.unwrap();
        assert!(graph.is_loaded());
        assert!(!graph.is_error());
    }

    #[tokio::test]
    async fn test_get_message_public_forwards() {
        let manager = StatisticsManager::new();
        let msg_full_id = create_test_message_full_id();

        let result = manager
            .get_message_public_forwards(msg_full_id, "".to_string(), 50)
            .await;

        assert!(result.is_ok());
        let forwards = result.unwrap();
        assert_eq!(forwards.count(), 0);
    }

    #[tokio::test]
    async fn test_get_message_public_forwards_invalid_limit() {
        let manager = StatisticsManager::new();
        let msg_full_id = create_test_message_full_id();

        let result = manager
            .get_message_public_forwards(msg_full_id, "".to_string(), 0)
            .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            error::Error::InvalidParameter("limit must be positive".to_string())
        );
    }

    #[tokio::test]
    async fn test_get_story_public_forwards() {
        let manager = StatisticsManager::new();
        let story_full_id = create_test_story_full_id();

        let result = manager
            .get_story_public_forwards(story_full_id, "".to_string(), 50)
            .await;

        assert!(result.is_ok());
        let forwards = result.unwrap();
        assert_eq!(forwards.count(), 0);
    }

    #[tokio::test]
    async fn test_get_story_public_forwards_invalid_limit() {
        let manager = StatisticsManager::new();
        let story_full_id = create_test_story_full_id();

        let result = manager
            .get_story_public_forwards(story_full_id, "".to_string(), 0)
            .await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            error::Error::InvalidParameter("limit must be positive".to_string())
        );
    }

    #[tokio::test]
    async fn test_cache_entry_expiration() {
        let entry = CacheEntry::new("test_data", Duration::from_millis(10));
        assert!(!entry.is_expired());
        tokio::time::sleep(Duration::from_millis(20)).await;
        assert!(entry.is_expired());
    }

    #[tokio::test]
    async fn test_clear_expired_cache() {
        let manager = StatisticsManager::new();

        // Add some data to cache with short TTL
        let _dialog_id = DialogId::User(rustgram_types::UserId(123456));
        let cache_entry = CacheEntry::new(
            ChatStatistics::Channel(ChatStatisticsChannel::new(
                DateRange::new(1609459200, 1609545600),
                StatisticalValue::new(10000.0, 9500.0, 5.26),
                StatisticalValue::new(5000.0, 4500.0, 11.11),
                StatisticalValue::new(200.0, 180.0, 11.11),
                StatisticalValue::new(100.0, 90.0, 11.11),
            )),
            Duration::from_millis(10),
        );

        {
            let mut cache = manager.channel_cache.write().await;
            cache.insert(123456, cache_entry);
        }

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(20)).await;

        // Clear expired
        manager.clear_expired_cache().await;

        // Verify cache is empty
        let cache = manager.channel_cache.read().await;
        assert!(cache.is_empty());
    }

    #[tokio::test]
    async fn test_dialog_id_type_validation() {
        let manager = StatisticsManager::new();

        // Test user dialog (should fail)
        let user_dialog = create_test_dialog_id();
        assert_eq!(user_dialog.get_type(), DialogType::User);
        let result = manager.get_channel_statistics(user_dialog, false).await;
        assert!(matches!(result.unwrap_err(), error::Error::InvalidDialog));

        // Test channel dialog (should succeed)
        let channel_dialog = DialogId::Channel(rustgram_types::ChannelId(-1000000000));
        assert_eq!(channel_dialog.get_type(), DialogType::Channel);
        let result = manager.get_channel_statistics(channel_dialog, false).await;
        assert!(result.is_ok());
    }
}
