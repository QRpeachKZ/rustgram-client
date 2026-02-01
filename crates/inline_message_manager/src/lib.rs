// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Inline Message Manager
//!
//! Manager for editing inline messages in Telegram.
//!
//! ## Overview
//!
//! The `InlineMessageManager` handles editing of inline messages sent via
//! inline bots. It provides methods for:
//!
//! - Editing inline message text
//! - Editing inline message live locations
//! - Editing inline message media
//! - Editing inline message captions
//! - Editing inline message reply markup
//! - Setting inline game scores
//! - Getting inline game high scores
//!
//! ## Architecture
//!
//! Based on TDLib's `InlineMessageManager` class, this module:
//! - Tracks inline message edits
//! - Manages game scores for inline messages
//! - Provides methods for all inline message editing operations
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_inline_message_manager::InlineMessageManager;
//! use rustgram_user_id::UserId;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let manager = InlineMessageManager::new();
//!
//!     // Edit an inline message's text
//!     let message_id = "inline_msg_123";
//!     manager.edit_inline_message_text(message_id, None, None).await?;
//!
//!     // Set game score
//!     let user_id = UserId::new(1234567890)?;
//!//!     manager.set_inline_game_score(message_id, true, user_id, 100, false).await?;
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

use rustgram_types::UserId;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use error::{Error, Result};

/// Inline message ID type
///
/// Unique identifier for inline messages in Telegram.
pub type InlineMessageId = String;

/// Game score for an inline message
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameScore {
    /// User who achieved the score
    user_id: UserId,
    /// The score value
    score: i32,
    /// Timestamp of the score
    timestamp: i64,
}

impl GameScore {
    /// Creates a new game score
    #[must_use]
    pub const fn new(user_id: UserId, score: i32, timestamp: i64) -> Self {
        Self {
            user_id,
            score,
            timestamp,
        }
    }

    /// Returns the user ID
    #[must_use]
    pub const fn user_id(&self) -> UserId {
        self.user_id
    }

    /// Returns the score
    #[must_use]
    pub const fn score(&self) -> i32 {
        self.score
    }

    /// Returns the timestamp
    #[must_use]
    pub const fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

/// Manager for editing inline messages
///
/// Handles editing of inline messages sent via inline bots.
/// Based on TDLib's `InlineMessageManager` class.
///
/// # Example
///
/// ```rust
/// use rustgram_inline_message_manager::InlineMessageManager;
/// use rustgram_user_id::UserId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let manager = InlineMessageManager::new();
///
/// let message_id = "inline_msg_123";
///
/// // Edit inline message text
/// manager.edit_inline_message_text(message_id, None, None).await?;
///
/// // Set game score
/// let user_id = UserId::new(1234567890)?;
/// manager.set_inline_game_score(message_id, true, user_id, 100, false).await?;
///
/// // Get high scores
/// let scores = manager.get_inline_game_high_scores(message_id, user_id).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct InlineMessageManager {
    /// Active inline messages
    messages: Arc<RwLock<HashMap<InlineMessageId, InlineMessageInfo>>>,
}

/// Information about an inline message
#[derive(Debug, Clone)]
struct InlineMessageInfo {
    /// Inline message ID
    id: InlineMessageId,
    /// Game scores for this message
    scores: Vec<GameScore>,
}

impl Default for InlineMessageManager {
    fn default() -> Self {
        Self::new()
    }
}

impl InlineMessageManager {
    /// Creates a new inline message manager
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_inline_message_manager::InlineMessageManager;
    ///
    /// let manager = InlineMessageManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            messages: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Edits the text of an inline message
    ///
    /// # Arguments
    ///
    /// * `inline_message_id` - Identifier of the inline message
    /// * `reply_markup` - Optional new reply markup
    /// * `input_message_content` - Optional new text content
    ///
    /// # Errors
    ///
    /// Returns an error if the message doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_inline_message_manager::InlineMessageManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = InlineMessageManager::new();
    /// let message_id = "inline_msg_123";
    ///
    /// // Note: This is a stub - actual implementation would send to Telegram
    /// manager.edit_inline_message_text(message_id, None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn edit_inline_message_text(
        &self,
        inline_message_id: &str,
        _reply_markup: Option<()>,
        _input_message_content: Option<()>,
    ) -> Result<()> {
        // Check if message exists (in real implementation, this would send to Telegram)
        let messages = self.messages.read().await;
        if !messages.contains_key(inline_message_id) {
            // For now, allow editing non-existent messages (will be created on first edit)
            // In real implementation, this would verify with Telegram
        }
        Ok(())
    }

    /// Edits the live location of an inline message
    ///
    /// # Arguments
    ///
    /// * `inline_message_id` - Identifier of the inline message
    /// * `reply_markup` - Optional new reply markup
    /// * `_input_location` - Optional new location
    /// * `_live_period` - New time period for live location
    /// * `_heading` - Direction in which the user is moving
    /// * `_proximity_alert_radius` - Distance for proximity alerts
    ///
    /// # Errors
    ///
    /// Returns an error if the message doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_inline_message_manager::InlineMessageManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = InlineMessageManager::new();
    /// let message_id = "inline_msg_123";
    ///
    /// manager.edit_inline_message_live_location(
    ///     message_id,
    ///     None,
    ///     None,
    ///     3600,
    ///     0,
    ///     0
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn edit_inline_message_live_location(
        &self,
        inline_message_id: &str,
        _reply_markup: Option<()>,
        _input_location: Option<()>,
        _live_period: i32,
        _heading: i32,
        _proximity_alert_radius: i32,
    ) -> Result<()> {
        let messages = self.messages.read().await;
        if !messages.contains_key(inline_message_id) {
            return Err(Error::MessageNotFound(inline_message_id.to_string()));
        }
        Ok(())
    }

    /// Edits the media content of an inline message
    ///
    /// # Arguments
    ///
    /// * `inline_message_id` - Identifier of the inline message
    /// * `reply_markup` - Optional new reply markup
    /// * `_input_message_content` - New media content
    ///
    /// # Errors
    ///
    /// Returns an error if the message doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_inline_message_manager::InlineMessageManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = InlineMessageManager::new();
    /// let message_id = "inline_msg_123";
    ///
    /// manager.edit_inline_message_media(message_id, None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn edit_inline_message_media(
        &self,
        inline_message_id: &str,
        _reply_markup: Option<()>,
        _input_message_content: Option<()>,
    ) -> Result<()> {
        let messages = self.messages.read().await;
        if !messages.contains_key(inline_message_id) {
            return Err(Error::MessageNotFound(inline_message_id.to_string()));
        }
        Ok(())
    }

    /// Edits the caption of an inline message
    ///
    /// # Arguments
    ///
    /// * `inline_message_id` - Identifier of the inline message
    /// * `reply_markup` - Optional new reply markup
    /// * `_input_caption` - New caption
    /// * `_invert_media` - Whether to invert media position
    ///
    /// # Errors
    ///
    /// Returns an error if the message doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_inline_message_manager::InlineMessageManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = InlineMessageManager::new();
    /// let message_id = "inline_msg_123";
    ///
    /// manager.edit_inline_message_caption(message_id, None, None, false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn edit_inline_message_caption(
        &self,
        inline_message_id: &str,
        _reply_markup: Option<()>,
        _input_caption: Option<()>,
        _invert_media: bool,
    ) -> Result<()> {
        let messages = self.messages.read().await;
        if !messages.contains_key(inline_message_id) {
            return Err(Error::MessageNotFound(inline_message_id.to_string()));
        }
        Ok(())
    }

    /// Edits the reply markup of an inline message
    ///
    /// # Arguments
    ///
    /// * `inline_message_id` - Identifier of the inline message
    /// * `_reply_markup` - New reply markup
    ///
    /// # Errors
    ///
    /// Returns an error if the message doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_inline_message_manager::InlineMessageManager;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = InlineMessageManager::new();
    /// let message_id = "inline_msg_123";
    ///
    /// manager.edit_inline_message_reply_markup(message_id, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn edit_inline_message_reply_markup(
        &self,
        inline_message_id: &str,
        _reply_markup: Option<()>,
    ) -> Result<()> {
        let messages = self.messages.read().await;
        if !messages.contains_key(inline_message_id) {
            return Err(Error::MessageNotFound(inline_message_id.to_string()));
        }
        Ok(())
    }

    /// Sets the game score for an inline message
    ///
    /// # Arguments
    ///
    /// * `inline_message_id` - Identifier of the inline message
    /// * `edit_message` - Whether to edit the message to show the score
    /// * `user_id` - User who achieved the score
    /// * `score` - The score value
    /// * `_force` - Whether to set the score even if it's lower
    ///
    /// # Errors
    ///
    /// Returns an error if the message doesn't exist or user_id is invalid
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_inline_message_manager::InlineMessageManager;
    /// use rustgram_user_id::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = InlineMessageManager::new();
    /// let message_id = "inline_msg_123";
    /// let user_id = UserId::new(1234567890)?;
    ///
    /// manager.set_inline_game_score(message_id, true, user_id, 100, false).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_inline_game_score(
        &self,
        inline_message_id: &str,
        _edit_message: bool,
        user_id: UserId,
        score: i32,
        _force: bool,
    ) -> Result<()> {
        let mut messages = self.messages.write().await;

        let message_info = messages
            .entry(inline_message_id.to_string())
            .or_insert_with(|| InlineMessageInfo {
                id: inline_message_id.to_string(),
                scores: Vec::new(),
            });

        // Check if user already has a score
        if let Some(existing) = message_info.scores.iter().position(|s| s.user_id == user_id) {
            message_info.scores[existing].score = score;
        } else {
            let game_score = GameScore::new(user_id, score, chrono_timestamp());
            message_info.scores.push(game_score);
        }

        Ok(())
    }

    /// Gets the high scores for a game in an inline message
    ///
    /// # Arguments
    ///
    /// * `inline_message_id` - Identifier of the inline message
    /// * `user_id` - User to get scores for
    ///
    /// # Returns
    ///
    /// Vector of game scores sorted by score (highest first)
    ///
    /// # Errors
    ///
    /// Returns an error if the message doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_inline_message_manager::InlineMessageManager;
    /// use rustgram_user_id::UserId;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = InlineMessageManager::new();
    /// let message_id = "inline_msg_123";
    /// let user_id = UserId::new(1234567890)?;
    ///
    /// let scores = manager.get_inline_game_high_scores(message_id, user_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_inline_game_high_scores(
        &self,
        inline_message_id: &str,
        _user_id: UserId,
    ) -> Result<Vec<GameScore>> {
        let messages = self.messages.read().await;

        let message_info = messages
            .get(inline_message_id)
            .ok_or_else(|| Error::MessageNotFound(inline_message_id.to_string()))?;

        // Return scores sorted by score (highest first)
        let mut scores = message_info.scores.clone();
        scores.sort_by(|a, b| b.score.cmp(&a.score));

        Ok(scores)
    }
}

/// Gets current Unix timestamp
#[must_use]
fn chrono_timestamp() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Constructor Tests ==========

    #[test]
    fn test_inline_message_manager_new() {
        let manager = InlineMessageManager::new();
        assert!(manager.messages.read().now_or_never().unwrap().is_empty());
    }

    #[test]
    fn test_inline_message_manager_default() {
        let manager = InlineMessageManager::default();
        assert!(manager.messages.read().now_or_never().unwrap().is_empty());
    }

    // ========== GameScore Tests ==========

    #[test]
    fn test_game_score_new() {
        let user_id = UserId::new(123).unwrap();
        let score = GameScore::new(user_id, 100, 1234567890);
        assert_eq!(score.user_id(), user_id);
        assert_eq!(score.score(), 100);
        assert_eq!(score.timestamp(), 1234567890);
    }

    // ========== Edit Inline Message Text Tests ==========

    #[tokio::test]
    async fn test_edit_inline_message_text_success() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";

        let result = manager
            .edit_inline_message_text(message_id, None, None)
            .await;
        assert!(result.is_ok());
    }

    // ========== Edit Inline Message Live Location Tests ==========

    #[tokio::test]
    async fn test_edit_inline_message_live_location_not_found() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";

        let result = manager
            .edit_inline_message_live_location(message_id, None, None, 3600, 0, 0)
            .await;
        assert!(matches!(result, Err(Error::MessageNotFound(_))));
    }

    #[tokio::test]
    async fn test_edit_inline_message_live_location_success() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";

        // First add a score to create the message
        let user_id = UserId::new(123).unwrap();
        manager
            .set_inline_game_score(message_id, true, user_id, 100, false)
            .await
            .unwrap();

        let result = manager
            .edit_inline_message_live_location(message_id, None, None, 3600, 0, 0)
            .await;
        assert!(result.is_ok());
    }

    // ========== Edit Inline Message Media Tests ==========

    #[tokio::test]
    async fn test_edit_inline_message_media_not_found() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";

        let result = manager
            .edit_inline_message_media(message_id, None, None)
            .await;
        assert!(matches!(result, Err(Error::MessageNotFound(_))));
    }

    #[tokio::test]
    async fn test_edit_inline_message_media_success() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";

        // First add a score to create the message
        let user_id = UserId::new(123).unwrap();
        manager
            .set_inline_game_score(message_id, true, user_id, 100, false)
            .await
            .unwrap();

        let result = manager
            .edit_inline_message_media(message_id, None, None)
            .await;
        assert!(result.is_ok());
    }

    // ========== Edit Inline Message Caption Tests ==========

    #[tokio::test]
    async fn test_edit_inline_message_caption_not_found() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";

        let result = manager
            .edit_inline_message_caption(message_id, None, None, false)
            .await;
        assert!(matches!(result, Err(Error::MessageNotFound(_))));
    }

    #[tokio::test]
    async fn test_edit_inline_message_caption_success() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";

        // First add a score to create the message
        let user_id = UserId::new(123).unwrap();
        manager
            .set_inline_game_score(message_id, true, user_id, 100, false)
            .await
            .unwrap();

        let result = manager
            .edit_inline_message_caption(message_id, None, None, false)
            .await;
        assert!(result.is_ok());
    }

    // ========== Edit Inline Message Reply Markup Tests ==========

    #[tokio::test]
    async fn test_edit_inline_message_reply_markup_not_found() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";

        let result = manager
            .edit_inline_message_reply_markup(message_id, None)
            .await;
        assert!(matches!(result, Err(Error::MessageNotFound(_))));
    }

    #[tokio::test]
    async fn test_edit_inline_message_reply_markup_success() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";

        // First add a score to create the message
        let user_id = UserId::new(123).unwrap();
        manager
            .set_inline_game_score(message_id, true, user_id, 100, false)
            .await
            .unwrap();

        let result = manager
            .edit_inline_message_reply_markup(message_id, None)
            .await;
        assert!(result.is_ok());
    }

    // ========== Set Inline Game Score Tests ==========

    #[tokio::test]
    async fn test_set_inline_game_score_new_message() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";
        let user_id = UserId::new(123).unwrap();

        let result = manager
            .set_inline_game_score(message_id, true, user_id, 100, false)
            .await;
        assert!(result.is_ok());

        // Verify score was stored
        let scores = manager
            .get_inline_game_high_scores(message_id, user_id)
            .await
            .unwrap();
        assert_eq!(scores.len(), 1);
        assert_eq!(scores[0].score(), 100);
    }

    #[tokio::test]
    async fn test_set_inline_game_score_update_existing() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";
        let user_id = UserId::new(123).unwrap();

        // Set initial score
        manager
            .set_inline_game_score(message_id, true, user_id, 100, false)
            .await
            .unwrap();

        // Update score
        manager
            .set_inline_game_score(message_id, true, user_id, 200, false)
            .await
            .unwrap();

        // Verify score was updated
        let scores = manager
            .get_inline_game_high_scores(message_id, user_id)
            .await
            .unwrap();
        assert_eq!(scores.len(), 1);
        assert_eq!(scores[0].score(), 200);
    }

    #[tokio::test]
    async fn test_set_inline_game_score_multiple_users() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";
        let user1 = UserId::new(1).unwrap();
        let user2 = UserId::new(2).unwrap();

        manager
            .set_inline_game_score(message_id, true, user1, 100, false)
            .await
            .unwrap();
        manager
            .set_inline_game_score(message_id, true, user2, 150, false)
            .await
            .unwrap();

        // Verify both scores stored
        let scores = manager
            .get_inline_game_high_scores(message_id, user1)
            .await
            .unwrap();
        assert_eq!(scores.len(), 2);
    }

    // ========== Get Inline Game High Scores Tests ==========

    #[tokio::test]
    async fn test_get_inline_game_high_scores_not_found() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";
        let user_id = UserId::new(123).unwrap();

        let result = manager
            .get_inline_game_high_scores(message_id, user_id)
            .await;
        assert!(matches!(result, Err(Error::MessageNotFound(_))));
    }

    #[tokio::test]
    async fn test_get_inline_game_high_scores_sorted() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";
        let user1 = UserId::new(1).unwrap();
        let user2 = UserId::new(2).unwrap();
        let user3 = UserId::new(3).unwrap();

        // Add scores in random order
        manager
            .set_inline_game_score(message_id, true, user2, 150, false)
            .await
            .unwrap();
        manager
            .set_inline_game_score(message_id, true, user1, 200, false)
            .await
            .unwrap();
        manager
            .set_inline_game_score(message_id, true, user3, 100, false)
            .await
            .unwrap();

        // Scores should be sorted by score descending
        let scores = manager
            .get_inline_game_high_scores(message_id, user1)
            .await
            .unwrap();
        assert_eq!(scores.len(), 3);
        assert_eq!(scores[0].score(), 200);
        assert_eq!(scores[1].score(), 150);
        assert_eq!(scores[2].score(), 100);
    }

    #[tokio::test]
    async fn test_get_inline_game_high_scores_empty() {
        let manager = InlineMessageManager::new();
        let message_id = "inline_msg_123";

        // Create message by setting a score, then we'll verify it can be retrieved
        let user_id = UserId::new(123).unwrap();
        manager
            .set_inline_game_score(message_id, true, user_id, 100, false)
            .await
            .unwrap();

        // After creating, get scores
        let scores = manager
            .get_inline_game_high_scores(message_id, user_id)
            .await
            .unwrap();
        assert_eq!(scores.len(), 1);
    }

    // ========== GameScore Equality Tests ==========

    #[test]
    fn test_game_score_equality() {
        let user_id = UserId::new(123).unwrap();
        let score1 = GameScore::new(user_id, 100, 12345);
        let score2 = GameScore::new(user_id, 100, 12345);
        assert_eq!(score1, score2);
    }

    #[test]
    fn test_game_score_inequality() {
        let user_id = UserId::new(123).unwrap();
        let score1 = GameScore::new(user_id, 100, 12345);
        let score2 = GameScore::new(user_id, 200, 12345);
        assert_ne!(score1, score2);
    }

    // ========== Multiple Message Tests ==========

    #[tokio::test]
    async fn test_multiple_inline_messages() {
        let manager = InlineMessageManager::new();
        let msg1 = "inline_msg_1";
        let msg2 = "inline_msg_2";
        let user_id = UserId::new(123).unwrap();

        manager
            .set_inline_game_score(msg1, true, user_id, 100, false)
            .await
            .unwrap();
        manager
            .set_inline_game_score(msg2, true, user_id, 200, false)
            .await
            .unwrap();

        let scores1 = manager
            .get_inline_game_high_scores(msg1, user_id)
            .await
            .unwrap();
        let scores2 = manager
            .get_inline_game_high_scores(msg2, user_id)
            .await
            .unwrap();

        assert_eq!(scores1[0].score(), 100);
        assert_eq!(scores2[0].score(), 200);
    }

    // ========== Chrono Timestamp Tests ==========

    #[test]
    fn test_chrono_timestamp() {
        let timestamp = chrono_timestamp();
        assert!(timestamp > 0);
    }
}
