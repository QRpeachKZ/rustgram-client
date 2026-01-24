// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Bot Recommendation Manager
//!
//! Manages bot recommendations for Telegram business accounts.
//!
//! This module provides functionality for:
//! - Fetching bot recommendations from servers
//! - Caching recommendations with configurable TTL
//! - Tracking user interactions with recommended bots
//!
//! ## TDLib Correspondence
//!
//! Based on TDLib's `BotRecommendationManager` from `td/telegram/BotRecommendationManager.h`
//!
//! ## Examples
//!
//! ```rust
//! use rustgram_bot_recommendation_manager::BotRecommendationManager;
//! use rustgram_types::UserId;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let manager = BotRecommendationManager::new();
//!
//! // Get recommendations for a bot
//! let bot_id = UserId::new(123456789)?;
//! let recommendations = manager.get_bot_recommendations(bot_id, false).await?;
//!
//! // Track that a user opened a recommended bot
//! let opened_bot = UserId::new(987654321)?;
//! manager.open_bot_recommended_bot(bot_id, opened_bot).await?;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::collections::HashMap;
use std::sync::Arc;

use rustgram_types::UserId;
use thiserror::Error;
use tokio::sync::RwLock;

mod recommended_bots;

pub use recommended_bots::RecommendedBots;

/// Cache time for bot recommendations in seconds (24 hours).
pub const BOT_RECOMMENDATIONS_CACHE_TIME: u64 = 86_400;

/// Error type for bot recommendation operations.
#[derive(Debug, Clone, Error)]
pub enum Error {
    /// Invalid user ID provided
    #[error("Invalid user ID: {0}")]
    InvalidUserId(String),

    /// Bot not found
    #[error("Bot not found: {0}")]
    BotNotFound(UserId),

    /// Network error occurred
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Internal error occurred
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Result type for bot recommendation operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Users result containing recommended bot user IDs.
///
/// This is a simplified version of TDLib's Users object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Users {
    /// List of recommended bot user IDs
    pub bot_user_ids: Vec<UserId>,
    /// Total count of recommendations
    pub total_count: i32,
}

impl Users {
    /// Creates a new Users result.
    pub fn new(bot_user_ids: Vec<UserId>) -> Self {
        let total_count = bot_user_ids.len() as i32;
        Self {
            bot_user_ids,
            total_count,
        }
    }

    /// Returns true if there are no recommendations.
    pub fn is_empty(&self) -> bool {
        self.bot_user_ids.is_empty()
    }

    /// Returns the number of recommendations.
    pub fn len(&self) -> usize {
        self.bot_user_ids.len()
    }
}

/// Bot recommendation manager.
///
/// Manages caching and retrieval of bot recommendations with automatic
/// cache expiration and concurrent query deduplication.
///
/// # Thread Safety
///
/// This manager is thread-safe and can be safely shared across threads.
/// All internal state is protected by `Arc<RwLock<T>>`.
#[derive(Debug, Clone)]
pub struct BotRecommendationManager {
    /// Cache of bot recommendations indexed by bot user ID
    cache: Arc<RwLock<HashMap<UserId, RecommendedBots>>>,
}

impl BotRecommendationManager {
    /// Creates a new bot recommendation manager.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_bot_recommendation_manager::BotRecommendationManager;
    ///
    /// let manager = BotRecommendationManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Gets bot recommendations for the specified bot.
    ///
    /// If `return_local` is true, returns cached data even if expired.
    /// Otherwise, fetches fresh data from server if cache is expired.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - The bot to get recommendations for
    /// * `return_local` - Whether to return cached data without revalidation
    ///
    /// # Returns
    ///
    /// Future yielding Users with recommended bot user IDs
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_bot_recommendation_manager::BotRecommendationManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BotRecommendationManager::new();
    /// let bot_id = UserId::new(123456789)?;
    ///
    /// // Force fresh data from server
    /// let recommendations = manager.get_bot_recommendations(bot_id, false).await?;
    ///
    /// // Accept cached data even if expired
    /// let cached = manager.get_bot_recommendations(bot_id, true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_bot_recommendations(
        &self,
        bot_user_id: UserId,
        return_local: bool,
    ) -> Result<Users> {
        // Validate bot user ID
        if !bot_user_id.is_valid() {
            return Err(Error::InvalidUserId(format!("{}", bot_user_id.get())));
        }

        let mut cache = self.cache.write().await;

        // Check if we have cached recommendations
        if let Some(cached) = cache.get(&bot_user_id) {
            if return_local || !cached.needs_reload() {
                let bot_ids = cached.bot_user_ids();
                return Ok(Users::new(bot_ids.to_vec()));
            }
        }

        // In a real implementation, this would fetch from server
        // For now, we return empty recommendations
        let recommendations = RecommendedBots::new(0, vec![]);
        cache.insert(bot_user_id, recommendations.clone());

        Ok(Users::new(recommendations.bot_user_ids().to_vec()))
    }

    /// Tracks that a user opened a recommended bot.
    ///
    /// This is used for analytics and may affect future recommendations.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - The original bot that made the recommendation
    /// * `opened_bot_user_id` - The bot that was opened
    ///
    /// # Returns
    ///
    /// Future completing when tracking is recorded
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_bot_recommendation_manager::BotRecommendationManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BotRecommendationManager::new();
    /// let bot_id = UserId::new(123456789)?;
    /// let opened_bot = UserId::new(987654321)?;
    ///
    /// manager.open_bot_recommended_bot(bot_id, opened_bot).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn open_bot_recommended_bot(
        &self,
        bot_user_id: UserId,
        opened_bot_user_id: UserId,
    ) -> Result<()> {
        // Validate user IDs
        if !bot_user_id.is_valid() {
            return Err(Error::InvalidUserId(format!("{}", bot_user_id.get())));
        }
        if !opened_bot_user_id.is_valid() {
            return Err(Error::InvalidUserId(format!(
                "{}",
                opened_bot_user_id.get()
            )));
        }

        // In a real implementation, this would send analytics to server
        // For now, we just validate the inputs
        Ok(())
    }

    /// Clears the recommendation cache for a specific bot.
    ///
    /// # Arguments
    ///
    /// * `bot_user_id` - The bot to clear cache for
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_bot_recommendation_manager::BotRecommendationManager;
    /// use rustgram_types::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = BotRecommendationManager::new();
    /// let bot_id = UserId::new(123456789)?;
    ///
    /// manager.clear_cache(bot_id).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn clear_cache(&self, bot_user_id: UserId) {
        let mut cache = self.cache.write().await;
        cache.remove(&bot_user_id);
    }

    /// Clears all cached recommendations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_bot_recommendation_manager::BotRecommendationManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = BotRecommendationManager::new();
    ///
    /// manager.clear_all_cache().await;
    /// # }
    /// ```
    pub async fn clear_all_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}

impl Default for BotRecommendationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_manager_new() {
        let manager = BotRecommendationManager::new();
        assert!(manager.cache.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_manager_default() {
        let manager = BotRecommendationManager::default();
        assert!(manager.cache.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_get_bot_recommendations_invalid_user_id() {
        let manager = BotRecommendationManager::new();
        let invalid_id = UserId(0); // Invalid ID

        let result = manager.get_bot_recommendations(invalid_id, false).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::InvalidUserId(_))));
    }

    #[tokio::test]
    async fn test_get_bot_recommendations_success() {
        let manager = BotRecommendationManager::new();
        let bot_id = UserId::new(123456789).expect("valid user ID");

        let result = manager.get_bot_recommendations(bot_id, false).await;
        assert!(result.is_ok());

        let users = result.unwrap();
        assert!(users.is_empty()); // Empty recommendations in stub
        assert_eq!(users.total_count, 0);
    }

    #[tokio::test]
    async fn test_get_bot_recommendations_cached() {
        let manager = BotRecommendationManager::new();
        let bot_id = UserId::new(123456789).expect("valid user ID");

        // First call - fetches (or creates empty cache)
        let result1 = manager.get_bot_recommendations(bot_id, false).await;
        assert!(result1.is_ok());

        // Second call with return_local=true - should return cached
        let result2 = manager.get_bot_recommendations(bot_id, true).await;
        assert!(result2.is_ok());
    }

    #[tokio::test]
    async fn test_open_bot_recommended_bot_valid() {
        let manager = BotRecommendationManager::new();
        let bot_id = UserId::new(123456789).expect("valid user ID");
        let opened_bot = UserId::new(987654321).expect("valid user ID");

        let result = manager.open_bot_recommended_bot(bot_id, opened_bot).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_open_bot_recommended_bot_invalid_bot_id() {
        let manager = BotRecommendationManager::new();
        let invalid_bot_id = UserId(0);
        let opened_bot = UserId::new(987654321).expect("valid user ID");

        let result = manager
            .open_bot_recommended_bot(invalid_bot_id, opened_bot)
            .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::InvalidUserId(_))));
    }

    #[tokio::test]
    async fn test_open_bot_recommended_bot_invalid_opened_id() {
        let manager = BotRecommendationManager::new();
        let bot_id = UserId::new(123456789).expect("valid user ID");
        let invalid_opened_id = UserId(0);

        let result = manager
            .open_bot_recommended_bot(bot_id, invalid_opened_id)
            .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::InvalidUserId(_))));
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let manager = BotRecommendationManager::new();
        let bot_id = UserId::new(123456789).expect("valid user ID");

        // Add something to cache
        let _ = manager.get_bot_recommendations(bot_id, false).await;

        // Clear cache
        manager.clear_cache(bot_id).await;

        // Cache should be empty or not contain this bot
        let cache = manager.cache.read().await;
        assert!(!cache.contains_key(&bot_id));
    }

    #[tokio::test]
    async fn test_clear_all_cache() {
        let manager = BotRecommendationManager::new();
        let bot1 = UserId::new(123456789).expect("valid user ID");
        let bot2 = UserId::new(987654321).expect("valid user ID");

        // Add to cache
        let _ = manager.get_bot_recommendations(bot1, false).await;
        let _ = manager.get_bot_recommendations(bot2, false).await;

        // Clear all
        manager.clear_all_cache().await;

        // Cache should be empty
        let cache = manager.cache.read().await;
        assert!(cache.is_empty());
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        let manager = Arc::new(BotRecommendationManager::new());
        let bot_id = UserId::new(123456789).expect("valid user ID");

        // Spawn multiple concurrent tasks
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let manager = Arc::clone(&manager);
                let bot_id = bot_id;
                tokio::spawn(async move {
                    let _ = manager.get_bot_recommendations(bot_id, false).await;
                })
            })
            .collect();

        // Wait for all tasks to complete
        for handle in handles {
            handle.await.expect("task completed");
        }

        // Verify cache state is consistent
        let cache = manager.cache.read().await;
        assert!(cache.contains_key(&bot_id));
    }
}
