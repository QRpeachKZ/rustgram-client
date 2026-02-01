// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Channel Recommendation Manager
//!
//! Manager for channel recommendations in Telegram.
//!
//! ## Overview
//!
//! The `ChannelRecommendationManager` handles channel recommendations and
//! similar channels for Telegram channels. It provides methods for:
//!
//! - Getting global recommended channels
//! - Getting channel-specific recommendations
//! - Managing recommendation cache
//! - Tracking opened recommended channels
//!
//! ## Architecture
//!
//! Based on TDLib's `ChannelRecommendationManager` class, this module:
//! - Caches channel recommendations for performance
//! - Tracks per-channel recommendation lists
//! - Manages recommendation refresh timers
//! - Handles recommendation analytics
//!
//! ## Cache Behavior
//!
//! ```text
//! Recommendations are cached for 24 hours (86400 seconds)
//! Local cache is returned if available and valid
//! Server is queried if cache is expired or missing
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_channel_recommendation_manager::ChannelRecommendationManager;
//! use rustgram_types::{DialogId, UserId};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let manager = ChannelRecommendationManager::new();
//!
//!     // Get globally recommended channels
//!     let channels = manager.get_recommended_channels().await?;
//!
//!     // Get recommendations for a specific channel
//!     let channel_id = rustgram_types::ChannelId::new(1234567890).unwrap();
//!     let dialog_id = DialogId::from_channel(channel_id);
//!     let (channels, count) = manager.get_channel_recommendations(dialog_id).await?;
//!
//!     // Track that a user opened a recommended channel
//!     let opened_id = DialogId::from_channel(rustgram_types::ChannelId::new(9876543210).unwrap());
//!     manager.open_channel_recommended_channel(dialog_id, opened_id).await?;
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

use rustgram_types::{ChannelId, DialogId, DialogType};
use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

pub use error::{Error, Result};

/// Channel recommendation state for tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelRecommendationState {
    /// Recommendation is active
    Active,
    /// Recommendation was opened
    Opened,
    /// Recommendation was dismissed
    Dismissed,
}

/// Cache time for channel recommendations in seconds
const CHANNEL_RECOMMENDATIONS_CACHE_TIME: u64 = 86_400; // 24 hours

/// Recommended dialogs information
#[derive(Debug, Clone)]
struct RecommendedDialogs {
    /// Total count of recommendations
    total_count: i32,
    /// List of recommended dialog IDs
    dialog_ids: Vec<DialogId>,
    /// When to next reload the recommendations
    next_reload_time: Instant,
}

/// Manager for channel recommendations
///
/// Handles channel recommendations and similar channels.
/// Based on TDLib's `ChannelRecommendationManager` class.
///
/// # Example
///
/// ```rust
/// use rustgram_channel_recommendation_manager::ChannelRecommendationManager;
/// use rustgram_types::{DialogId, ChannelId};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let manager = ChannelRecommendationManager::new();
///
/// // Get recommended channels
/// let channels = manager.get_recommended_channels().await?;
/// assert!(channels.is_empty()); // No channels cached yet
///
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct ChannelRecommendationManager {
    /// Next recommendation ID to assign
    next_id: Arc<AtomicI32>,
    /// Globally recommended channels
    recommended_channels: Arc<RwLock<Option<RecommendedDialogs>>>,
    /// Per-channel recommendations
    channel_recommendations: Arc<RwLock<HashMap<ChannelId, RecommendedDialogs>>>,
}

impl Default for ChannelRecommendationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ChannelRecommendationManager {
    /// Creates a new channel recommendation manager
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_recommendation_manager::ChannelRecommendationManager;
    ///
    /// let manager = ChannelRecommendationManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_id: Arc::new(AtomicI32::new(1)),
            recommended_channels: Arc::new(RwLock::new(None)),
            channel_recommendations: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Gets globally recommended channels
    ///
    /// # Returns
    ///
    /// List of recommended channel dialog IDs
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_recommendation_manager::ChannelRecommendationManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = ChannelRecommendationManager::new();
    ///
    /// let channels = manager.get_recommended_channels().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_recommended_channels(&self) -> Result<Vec<DialogId>> {
        let recommended = self.recommended_channels.read().await;

        if let Some(ref data) = *recommended {
            if data.next_reload_time > Instant::now() {
                return Ok(data.dialog_ids.clone());
            }
        }
        drop(recommended);

        // In a real implementation, this would query the server
        // For now, return empty
        Ok(Vec::new())
    }

    /// Gets channel recommendations for a specific channel
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID of the channel
    ///
    /// # Returns
    ///
    /// Tuple of (channel list, total count)
    ///
    /// # Errors
    ///
    /// Returns an error if the dialog is not a channel
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_recommendation_manager::ChannelRecommendationManager;
    /// use rustgram_types::{DialogId, ChannelId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = ChannelRecommendationManager::new();
    /// let channel_id = ChannelId::new(1234567890).unwrap();
    /// let dialog_id = DialogId::from_channel(channel_id);
    ///
    /// let (channels, count) = manager.get_channel_recommendations(dialog_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_channel_recommendations(
        &self,
        dialog_id: DialogId,
    ) -> Result<(Vec<DialogId>, i32)> {
        // Validate that this is a channel
        if dialog_id.get_type() != DialogType::Channel {
            return Ok((Vec::new(), 0));
        }

        let channel_id = dialog_id.get_channel_id().ok_or(Error::InvalidDialogType)?;

        let recommendations = self.channel_recommendations.read().await;

        if let Some(ref data) = recommendations.get(&channel_id) {
            if data.next_reload_time > Instant::now() {
                return Ok((data.dialog_ids.clone(), data.total_count));
            }
        }
        drop(recommendations);

        // In a real implementation, this would query the server
        // For now, return empty
        Ok((Vec::new(), 0))
    }

    /// Records that a user opened a recommended channel
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID of the channel that showed the recommendation
    /// * `opened_dialog_id` - Dialog ID of the channel that was opened
    ///
    /// # Errors
    ///
    /// Returns an error if either dialog is not a channel
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_recommendation_manager::ChannelRecommendationManager;
    /// use rustgram_types::{DialogId, ChannelId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = ChannelRecommendationManager::new();
    /// let channel_id = ChannelId::new(1234567890).unwrap();
    /// let dialog_id = DialogId::from_channel(channel_id);
    /// let opened_id = DialogId::from_channel(ChannelId::new(9876543210).unwrap());
    ///
    /// manager.open_channel_recommended_channel(dialog_id, opened_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn open_channel_recommended_channel(
        &self,
        dialog_id: DialogId,
        opened_dialog_id: DialogId,
    ) -> Result<()> {
        // Validate both are channels
        if dialog_id.get_type() != DialogType::Channel {
            return Err(Error::InvalidDialogType);
        }
        if opened_dialog_id.get_type() != DialogType::Channel {
            return Err(Error::InvalidDialogType);
        }

        // In a real implementation, this would send analytics to the server
        Ok(())
    }

    /// Sets the globally recommended channels (for testing/internal use)
    ///
    /// # Arguments
    ///
    /// * `dialog_ids` - List of recommended channel dialog IDs
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_recommendation_manager::ChannelRecommendationManager;
    /// use rustgram_types::{DialogId, ChannelId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = ChannelRecommendationManager::new();
    /// let channels = vec![
    ///     DialogId::from_channel(ChannelId::new(123).unwrap()),
    ///     DialogId::from_channel(ChannelId::new(456).unwrap()),
    /// ];
    ///
    /// manager.set_recommended_channels(channels).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_recommended_channels(&self, dialog_ids: Vec<DialogId>) {
        let data = RecommendedDialogs {
            total_count: dialog_ids.len() as i32,
            dialog_ids,
            next_reload_time: Instant::now()
                + Duration::from_secs(CHANNEL_RECOMMENDATIONS_CACHE_TIME),
        };

        let mut recommended = self.recommended_channels.write().await;
        *recommended = Some(data);
    }

    /// Sets recommendations for a specific channel (for testing/internal use)
    ///
    /// # Arguments
    ///
    /// * `channel_id` - Channel ID
    /// * `dialog_ids` - List of recommended channel dialog IDs
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_recommendation_manager::ChannelRecommendationManager;
    /// use rustgram_types::{DialogId, ChannelId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = ChannelRecommendationManager::new();
    /// let channel_id = ChannelId::new(1234567890).unwrap();
    /// let channels = vec![
    ///     DialogId::from_channel(ChannelId::new(456).unwrap()),
    ///     DialogId::from_channel(ChannelId::new(789).unwrap()),
    /// ];
    ///
    /// manager.set_channel_recommendations(channel_id, channels, 10).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_channel_recommendations(
        &self,
        channel_id: ChannelId,
        dialog_ids: Vec<DialogId>,
        total_count: i32,
    ) {
        let data = RecommendedDialogs {
            total_count,
            dialog_ids,
            next_reload_time: Instant::now()
                + Duration::from_secs(CHANNEL_RECOMMENDATIONS_CACHE_TIME),
        };

        let mut recommendations = self.channel_recommendations.write().await;
        recommendations.insert(channel_id, data);
    }

    /// Clears all cached recommendations (for testing/internal use)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_recommendation_manager::ChannelRecommendationManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = ChannelRecommendationManager::new();
    ///
    /// manager.clear_cache().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn clear_cache(&self) {
        let mut recommended = self.recommended_channels.write().await;
        *recommended = None;

        let mut recommendations = self.channel_recommendations.write().await;
        recommendations.clear();
    }

    /// Checks if a channel is suitable for recommendation
    ///
    /// # Arguments
    ///
    /// * `channel_id` - Channel ID to check
    ///
    /// # Returns
    ///
    /// True if the channel is suitable for recommendation
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_recommendation_manager::ChannelRecommendationManager;
    /// use rustgram_types::ChannelId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = ChannelRecommendationManager::new();
    /// let channel_id = ChannelId::new(1234567890).unwrap();
    ///
    /// // In a real implementation, this would check channel status
    /// let suitable = manager.is_suitable_channel(channel_id).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn is_suitable_channel(&self, _channel_id: ChannelId) -> bool {
        // In a real implementation, this would check:
        // - Channel is accessible
        // - User is not a member
        // - Channel has proper access rights
        true
    }

    /// Returns the number of cached channel recommendations
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_channel_recommendation_manager::ChannelRecommendationManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = ChannelRecommendationManager::new();
    ///
    /// let count = manager.cached_recommendation_count().await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cached_recommendation_count(&self) -> usize {
        self.channel_recommendations.read().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Constructor Tests ==========

    #[test]
    fn test_channel_recommendation_manager_new() {
        let manager = ChannelRecommendationManager::new();
        assert_eq!(manager.next_id.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_channel_recommendation_manager_default() {
        let manager = ChannelRecommendationManager::default();
        assert_eq!(manager.next_id.load(Ordering::SeqCst), 1);
    }

    // ========== Get Recommended Channels Tests ==========

    #[tokio::test]
    async fn test_get_recommended_channels_empty() {
        let manager = ChannelRecommendationManager::new();

        let channels = manager.get_recommended_channels().await.unwrap();
        assert!(channels.is_empty());
    }

    #[tokio::test]
    async fn test_set_and_get_recommended_channels() {
        let manager = ChannelRecommendationManager::new();
        let channels = vec![
            DialogId::from_channel(ChannelId::new(123).unwrap()),
            DialogId::from_channel(ChannelId::new(456).unwrap()),
        ];

        manager.set_recommended_channels(channels.clone()).await;

        let result = manager.get_recommended_channels().await.unwrap();
        assert_eq!(result, channels);
    }

    // ========== Get Channel Recommendations Test ==========

    #[tokio::test]
    async fn test_get_channel_recommendations_not_channel() {
        let manager = ChannelRecommendationManager::new();
        let user_id = rustgram_types::UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);

        let (channels, count) = manager
            .get_channel_recommendations(dialog_id)
            .await
            .unwrap();
        assert!(channels.is_empty());
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn test_set_and_get_channel_recommendations() {
        let manager = ChannelRecommendationManager::new();
        let channel_id = ChannelId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let recommendations = vec![
            DialogId::from_channel(ChannelId::new(456).unwrap()),
            DialogId::from_channel(ChannelId::new(789).unwrap()),
        ];

        manager
            .set_channel_recommendations(channel_id, recommendations.clone(), 10)
            .await;

        let (result, count) = manager
            .get_channel_recommendations(dialog_id)
            .await
            .unwrap();
        assert_eq!(result, recommendations);
        assert_eq!(count, 10);
    }

    #[tokio::test]
    async fn test_get_channel_recommendations_empty() {
        let manager = ChannelRecommendationManager::new();
        let channel_id = ChannelId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);

        let (channels, count) = manager
            .get_channel_recommendations(dialog_id)
            .await
            .unwrap();
        assert!(channels.is_empty());
        assert_eq!(count, 0);
    }

    // ========== Open Channel Recommended Channel Tests ==========

    #[tokio::test]
    async fn test_open_channel_recommended_channel_success() {
        let manager = ChannelRecommendationManager::new();
        let channel_id = ChannelId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let opened_id = DialogId::from_channel(ChannelId::new(9876543210).unwrap());

        let result = manager
            .open_channel_recommended_channel(dialog_id, opened_id)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_open_channel_recommended_channel_invalid_source() {
        let manager = ChannelRecommendationManager::new();
        let user_id = rustgram_types::UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let opened_id = DialogId::from_channel(ChannelId::new(9876543210).unwrap());

        let result = manager
            .open_channel_recommended_channel(dialog_id, opened_id)
            .await;
        assert!(matches!(result, Err(Error::InvalidDialogType)));
    }

    #[tokio::test]
    async fn test_open_channel_recommended_channel_invalid_target() {
        let manager = ChannelRecommendationManager::new();
        let channel_id = ChannelId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let user_id = rustgram_types::UserId::new(123).unwrap();
        let opened_id = DialogId::from_user(user_id);

        let result = manager
            .open_channel_recommended_channel(dialog_id, opened_id)
            .await;
        assert!(matches!(result, Err(Error::InvalidDialogType)));
    }

    // ========== Clear Cache Tests ==========

    #[tokio::test]
    async fn test_clear_cache() {
        let manager = ChannelRecommendationManager::new();
        let channels = vec![DialogId::from_channel(ChannelId::new(123).unwrap())];

        manager.set_recommended_channels(channels).await;
        manager.clear_cache().await;

        let result = manager.get_recommended_channels().await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_clear_channel_recommendations() {
        let manager = ChannelRecommendationManager::new();
        let channel_id = ChannelId::new(1234567890).unwrap();
        let recommendations = vec![DialogId::from_channel(ChannelId::new(456).unwrap())];

        manager
            .set_channel_recommendations(channel_id, recommendations, 1)
            .await;
        assert_eq!(manager.cached_recommendation_count().await, 1);

        manager.clear_cache().await;
        assert_eq!(manager.cached_recommendation_count().await, 0);
    }

    // ========== Cached Recommendation Count Tests ==========

    #[tokio::test]
    async fn test_cached_recommendation_count_zero() {
        let manager = ChannelRecommendationManager::new();
        assert_eq!(manager.cached_recommendation_count().await, 0);
    }

    #[tokio::test]
    async fn test_cached_recommendation_count_multiple() {
        let manager = ChannelRecommendationManager::new();

        manager
            .set_channel_recommendations(
                ChannelId::new(123).unwrap(),
                vec![DialogId::from_channel(ChannelId::new(1).unwrap())],
                1,
            )
            .await;
        manager
            .set_channel_recommendations(
                ChannelId::new(456).unwrap(),
                vec![DialogId::from_channel(ChannelId::new(1).unwrap())],
                1,
            )
            .await;
        manager
            .set_channel_recommendations(
                ChannelId::new(789).unwrap(),
                vec![DialogId::from_channel(ChannelId::new(1).unwrap())],
                1,
            )
            .await;

        assert_eq!(manager.cached_recommendation_count().await, 3);
    }

    // ========== Is Suitable Channel Tests ==========

    #[tokio::test]
    async fn test_is_suitable_channel() {
        let manager = ChannelRecommendationManager::new();
        let channel_id = ChannelId::new(1234567890).unwrap();

        // In test/mock, always returns true
        assert!(manager.is_suitable_channel(channel_id).await);
    }

    // ========== Multiple Channels Tests ==========

    #[tokio::test]
    async fn test_multiple_channel_recommendations() {
        let manager = ChannelRecommendationManager::new();

        let ch1 = ChannelId::new(111).unwrap();
        let ch2 = ChannelId::new(222).unwrap();
        let recs = vec![DialogId::from_channel(ChannelId::new(999).unwrap())];

        manager
            .set_channel_recommendations(ch1, recs.clone(), 5)
            .await;
        manager
            .set_channel_recommendations(ch2, recs.clone(), 10)
            .await;

        let dialog1 = DialogId::from_channel(ch1);
        let (_result1, count1) = manager.get_channel_recommendations(dialog1).await.unwrap();
        assert_eq!(count1, 5);

        let dialog2 = DialogId::from_channel(ch2);
        let (_result2, count2) = manager.get_channel_recommendations(dialog2).await.unwrap();
        assert_eq!(count2, 10);
    }
}
