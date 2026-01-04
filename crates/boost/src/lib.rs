// Copyright 2025 rustgram-client
//
// Licensed under MIT License

//! Boost module for Telegram client.
//!
//! This module provides functionality for managing chat boosts in Telegram.
//!
//! # Example
//!
//! ```no_run
//! use rustgram_boost::BoostManager;
//!
//! let manager = BoostManager::new();
//!
//! // Get available boost slots (placeholder - returns NetworkError)
//! // let slots = manager.get_boost_slots();
//!
//! // Get boost level features
//! let features = manager.get_chat_boost_level_features(false, 5);
//! println!("Can set emoji status: {}", features.can_set_emoji_status);
//! ```

pub mod error;
pub mod private;
pub mod types;

// Internal imports
use crate::private::{format_boost_link, parse_boost_link, validate_limit, validate_slot_ids};
use rustgram_types::{DialogId, UserId};

/// Re-export commonly used types.
pub use crate::error::{BoostError, Result};
pub use crate::types::{
    ChatBoost, ChatBoostFeatures, ChatBoostLevelFeatures, ChatBoostSlot, ChatBoostSlots,
    ChatBoostSource, ChatBoostStatus, DialogBoostLinkInfo, FoundChatBoosts, GiveawayPrize,
    PrepaidGiveaway,
};

/// Manager for chat boosts.
///
/// This struct provides methods for managing chat boosts in Telegram.
/// Boosts allow premium users to enhance their favorite chats with additional features.
///
/// # Note
/// Currently, this is a placeholder implementation. Network operations
/// will be added when the MTProto transport layer is ready.
#[derive(Debug, Clone, Default)]
pub struct BoostManager {
    /// Maximum boost level supported
    max_boost_level: i32,
}

impl BoostManager {
    /// Create a new `BoostManager`.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_boost::BoostManager;
    ///
    /// let manager = BoostManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            max_boost_level: 10,
        }
    }

    /// Create a new `BoostManager` with custom max boost level.
    pub fn with_max_level(max_boost_level: i32) -> Self {
        Self { max_boost_level }
    }

    /// Get the user's available boost slots.
    ///
    /// # Note
    /// This is a placeholder implementation. Network operations
    /// will be added when the MTProto transport layer is ready.
    ///
    /// # Returns
    /// A `ChatBoostSlots` containing the user's boost slots.
    ///
    /// # Errors
    /// Returns a `BoostError::NetworkError` if network operations fail.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustgram_boost::{BoostManager, ChatBoostSlots};
    /// # fn example() -> Result<ChatBoostSlots, rustgram_boost::BoostError> {
    /// let manager = BoostManager::new();
    /// let slots = manager.get_boost_slots()?;
    /// # Ok(slots)
    /// # }
    /// ```
    pub fn get_boost_slots(&self) -> Result<ChatBoostSlots> {
        // TODO: Implement network call when MTProto is ready
        Err(BoostError::network(
            "Network operations not yet implemented",
        ))
    }

    /// Get the boost status for a dialog.
    ///
    /// # Arguments
    /// * `dialog_id` - The dialog to get boost status for
    ///
    /// # Note
    /// This is a placeholder implementation. Network operations
    /// will be added when the MTProto transport layer is ready.
    ///
    /// # Returns
    /// A `ChatBoostStatus` containing the dialog's boost information.
    ///
    /// # Errors
    /// * `BoostError::InvalidDialog` - if the dialog ID is invalid
    /// * `BoostError::NetworkError` - if network operations fail
    ///
    /// # Example
    ///
    /// ```
    /// # use rustgram_boost::{BoostManager, ChatBoostStatus};
    /// # use rustgram_types::{DialogId, ChannelId};
    /// # fn example() -> Result<ChatBoostStatus, rustgram_boost::BoostError> {
    /// let manager = BoostManager::new();
    /// let dialog_id = DialogId::from_channel(ChannelId::new(123).unwrap());
    /// let status = manager.get_dialog_boost_status(dialog_id)?;
    /// # Ok(status)
    /// # }
    /// ```
    pub fn get_dialog_boost_status(&self, dialog_id: DialogId) -> Result<ChatBoostStatus> {
        if !dialog_id.is_valid() {
            return Err(BoostError::invalid_dialog("Dialog ID is not valid"));
        }
        // TODO: Implement network call when MTProto is ready
        Err(BoostError::network(
            "Network operations not yet implemented",
        ))
    }

    /// Apply boosts to a dialog.
    ///
    /// # Arguments
    /// * `dialog_id` - The dialog to boost
    /// * `slot_ids` - List of boost slot IDs to use (empty = get all slots)
    ///
    /// # Note
    /// This is a placeholder implementation. Network operations
    /// will be added when the MTProto transport layer is ready.
    ///
    /// # Returns
    /// A `ChatBoostSlots` containing updated slot information.
    ///
    /// # Errors
    /// * `BoostError::InvalidDialog` - if the dialog ID is invalid
    /// * `BoostError::InvalidSlotId` - if any slot ID is invalid
    /// * `BoostError::NetworkError` - if network operations fail
    ///
    /// # Example
    ///
    /// ```
    /// # use rustgram_boost::{BoostManager, ChatBoostSlots};
    /// # use rustgram_types::{DialogId, ChannelId};
    /// # fn example() -> Result<ChatBoostSlots, rustgram_boost::BoostError> {
    /// let manager = BoostManager::new();
    /// let dialog_id = DialogId::from_channel(ChannelId::new(123).unwrap());
    /// let result = manager.boost_dialog(dialog_id, vec![0, 1])?;
    /// # Ok(result)
    /// # }
    /// ```
    pub fn boost_dialog(&self, dialog_id: DialogId, slot_ids: Vec<i32>) -> Result<ChatBoostSlots> {
        if !dialog_id.is_valid() {
            return Err(BoostError::invalid_dialog("Dialog ID is not valid"));
        }
        validate_slot_ids(&slot_ids)?;
        // TODO: Implement network call when MTProto is ready
        Err(BoostError::network(
            "Network operations not yet implemented",
        ))
    }

    /// Get the boost link for a dialog.
    ///
    /// # Arguments
    /// * `dialog_id` - The dialog to get boost link for
    ///
    /// # Returns
    /// A tuple of (boost link URL, is_public).
    ///
    /// # Errors
    /// * `BoostError::InvalidDialog` - if the dialog ID is invalid
    /// * `BoostError::CannotBoostChat` - if the dialog is not a channel
    ///
    /// # Example
    ///
    /// ```
    /// # use rustgram_boost::BoostManager;
    /// # use rustgram_types::{DialogId, ChannelId};
    /// # fn example() -> Result<(String, bool), rustgram_boost::BoostError> {
    /// let manager = BoostManager::new();
    /// let dialog_id = DialogId::from_channel(ChannelId::new(123).unwrap());
    /// let (link, is_public) = manager.get_dialog_boost_link(dialog_id)?;
    /// # Ok((link, is_public))
    /// # }
    /// ```
    pub fn get_dialog_boost_link(&self, dialog_id: DialogId) -> Result<(String, bool)> {
        if !dialog_id.is_valid() {
            return Err(BoostError::invalid_dialog("Dialog ID is not valid"));
        }

        // Check if it's a channel
        if dialog_id.get_type() != rustgram_types::DialogType::Channel {
            return Err(BoostError::CannotBoostChat);
        }

        let channel_id = dialog_id
            .get_channel_id()
            .ok_or_else(|| BoostError::invalid_dialog("Not a channel"))?;

        // TODO: Get username from ChatManager when available
        // For now, return private link
        let (url, is_public) = format_boost_link(None, channel_id);
        Ok((url, is_public))
    }

    /// Parse a boost link to get information about the dialog.
    ///
    /// # Arguments
    /// * `url` - The boost link URL to parse
    ///
    /// # Returns
    /// A `DialogBoostLinkInfo` containing parsed information.
    ///
    /// # Errors
    /// * `BoostError::InvalidBoostLink` - if the URL is not a valid boost link
    ///
    /// # Example
    ///
    /// ```
    /// # use rustgram_boost::{BoostManager, DialogBoostLinkInfo};
    /// # fn example() -> Result<DialogBoostLinkInfo, rustgram_boost::BoostError> {
    /// let manager = BoostManager::new();
    /// let info = manager.get_dialog_boost_link_info("https://t.me/boost/mychannel")?;
    /// # Ok(info)
    /// # }
    /// ```
    pub fn get_dialog_boost_link_info(&self, url: &str) -> Result<DialogBoostLinkInfo> {
        parse_boost_link(url)
    }

    /// Get boosts for a dialog.
    ///
    /// # Arguments
    /// * `dialog_id` - The dialog to get boosts for
    /// * `only_gift_codes` - Whether to only return gift code boosts
    /// * `offset` - Offset for pagination
    /// * `limit` - Maximum number of boosts to return
    ///
    /// # Note
    /// This is a placeholder implementation. Network operations
    /// will be added when the MTProto transport layer is ready.
    ///
    /// # Returns
    /// A `FoundChatBoosts` containing the boosts.
    ///
    /// # Errors
    /// * `BoostError::InvalidDialog` - if the dialog ID is invalid
    /// * `BoostError::InvalidLimit` - if the limit is not positive
    /// * `BoostError::NetworkError` - if network operations fail
    ///
    /// # Example
    ///
    /// ```
    /// # use rustgram_boost::{BoostManager, FoundChatBoosts};
    /// # use rustgram_types::{DialogId, ChannelId};
    /// # fn example() -> Result<FoundChatBoosts, rustgram_boost::BoostError> {
    /// let manager = BoostManager::new();
    /// let dialog_id = DialogId::from_channel(ChannelId::new(123).unwrap());
    /// let result = manager.get_dialog_boosts(dialog_id, false, "".to_string(), 50)?;
    /// # Ok(result)
    /// # }
    /// ```
    pub fn get_dialog_boosts(
        &self,
        dialog_id: DialogId,
        _only_gift_codes: bool,
        _offset: String,
        limit: i32,
    ) -> Result<FoundChatBoosts> {
        if !dialog_id.is_valid() {
            return Err(BoostError::invalid_dialog("Dialog ID is not valid"));
        }
        validate_limit(limit)?;
        // TODO: Implement network call when MTProto is ready
        Err(BoostError::network(
            "Network operations not yet implemented",
        ))
    }

    /// Get boosts from a specific user for a dialog.
    ///
    /// # Arguments
    /// * `dialog_id` - The dialog to get boosts for
    /// * `user_id` - The user whose boosts to get
    ///
    /// # Note
    /// This is a placeholder implementation. Network operations
    /// will be added when the MTProto transport layer is ready.
    ///
    /// # Returns
    /// A `FoundChatBoosts` containing the user's boosts.
    ///
    /// # Errors
    /// * `BoostError::InvalidDialog` - if the dialog ID is invalid
    /// * `BoostError::InvalidUserId` - if the user ID is invalid
    /// * `BoostError::NetworkError` - if network operations fail
    ///
    /// # Example
    ///
    /// ```
    /// # use rustgram_boost::{BoostManager, FoundChatBoosts};
    /// # use rustgram_types::{DialogId, UserId, ChannelId};
    /// # fn example() -> Result<FoundChatBoosts, rustgram_boost::BoostError> {
    /// let manager = BoostManager::new();
    /// let dialog_id = DialogId::from_channel(ChannelId::new(123).unwrap());
    /// let user_id = UserId::from_i32(456);
    /// let result = manager.get_user_dialog_boosts(dialog_id, user_id)?;
    /// # Ok(result)
    /// # }
    /// ```
    pub fn get_user_dialog_boosts(
        &self,
        dialog_id: DialogId,
        user_id: UserId,
    ) -> Result<FoundChatBoosts> {
        if !dialog_id.is_valid() {
            return Err(BoostError::invalid_dialog("Dialog ID is not valid"));
        }
        if !user_id.is_valid() {
            return Err(BoostError::invalid_user_id("User ID is not valid"));
        }
        // TODO: Implement network call when MTProto is ready
        Err(BoostError::network(
            "Network operations not yet implemented",
        ))
    }

    /// Get boost level features for a specific level.
    ///
    /// # Arguments
    /// * `for_megagroup` - Whether this is for a megagroup (vs channel)
    /// * `level` - The boost level to get features for
    ///
    /// # Returns
    /// A `ChatBoostLevelFeatures` describing available features at this level.
    ///
    /// # Example
    ///
    /// ```
    /// # use rustgram_boost::BoostManager;
    /// # fn example() -> rustgram_boost::ChatBoostLevelFeatures {
    /// let manager = BoostManager::new();
    /// let features = manager.get_chat_boost_level_features(false, 5);
    /// # features
    /// # }
    /// ```
    pub fn get_chat_boost_level_features(
        &self,
        _for_megagroup: bool,
        level: i32,
    ) -> ChatBoostLevelFeatures {
        let actual_level = if level < 0 {
            0
        } else {
            level.min(self.max_boost_level)
        };

        ChatBoostLevelFeatures {
            level,
            actual_level,
            profile_accent_color_count: if actual_level >= 3 { 2 } else { 0 },
            title_color_count: if actual_level >= 3 { 2 } else { 0 },
            can_set_profile_background_custom_emoji: actual_level >= 5,
            accent_color_count: if actual_level >= 3 { 1 } else { 0 },
            can_set_background_custom_emoji: actual_level >= 4,
            can_set_emoji_status: actual_level >= 2,
            chat_theme_count: if actual_level >= 5 { 1 } else { 0 },
            can_set_custom_background: actual_level >= 7,
            can_set_custom_emoji_sticker_set: actual_level >= 8,
            can_enable_autotranslation: actual_level >= 9,
            can_recognize_speech: actual_level >= 9,
            can_restrict_sponsored_messages: actual_level >= 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::ChannelId;

    #[test]
    fn test_boost_manager_new() {
        let manager = BoostManager::new();
        assert_eq!(manager.max_boost_level, 10);
    }

    #[test]
    fn test_get_dialog_boost_link_public_channel() {
        let manager = BoostManager::new();
        // Public channel would require ChatManager - for now just test private
        let channel_id = ChannelId::try_from(12345).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let (link, is_public) = manager.get_dialog_boost_link(dialog_id).unwrap();
        assert_eq!(link, "https://t.me/boost?c=12345");
        assert!(!is_public);
    }

    #[test]
    fn test_get_dialog_boost_link_invalid_dialog() {
        let manager = BoostManager::new();
        // UserId(0) is invalid, so DialogId::User(UserId(0)) is invalid
        let dialog_id = DialogId::User(UserId(0));
        let result = manager.get_dialog_boost_link(dialog_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_dialog_boost_link_non_channel() {
        let manager = BoostManager::new();
        // User dialog - not boostable
        let user_id = UserId::try_from(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let result = manager.get_dialog_boost_link(dialog_id);
        assert!(matches!(result, Err(BoostError::CannotBoostChat)));
    }

    #[test]
    fn test_get_dialog_boost_link_info_public() {
        let manager = BoostManager::new();
        let info = manager
            .get_dialog_boost_link_info("https://t.me/boost/mychannel")
            .unwrap();
        assert!(info.is_public());
        assert_eq!(info.username, Some("mychannel".to_string()));
    }

    #[test]
    fn test_get_dialog_boost_link_info_private() {
        let manager = BoostManager::new();
        let info = manager
            .get_dialog_boost_link_info("https://t.me/boost?c=12345")
            .unwrap();
        assert!(!info.is_public());
        assert_eq!(info.channel_id, ChannelId::try_from(12345).ok());
    }

    #[test]
    fn test_get_dialog_boost_link_info_invalid() {
        let manager = BoostManager::new();
        let result = manager.get_dialog_boost_link_info("https://example.com/boost");
        assert!(matches!(result, Err(BoostError::InvalidBoostLink(_))));
    }

    #[test]
    fn test_boost_dialog_invalid_slot_id() {
        let manager = BoostManager::new();
        let channel_id = ChannelId::try_from(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let result = manager.boost_dialog(dialog_id, vec![-1]);
        assert!(matches!(result, Err(BoostError::InvalidSlotId(_))));
    }

    #[test]
    fn test_get_dialog_boosts_invalid_limit() {
        let manager = BoostManager::new();
        let channel_id = ChannelId::try_from(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let result = manager.get_dialog_boosts(dialog_id, false, "".to_string(), 0);
        assert!(matches!(result, Err(BoostError::InvalidLimit(_))));
    }

    #[test]
    fn test_get_user_dialog_boosts_invalid_user() {
        let manager = BoostManager::new();
        let channel_id = ChannelId::try_from(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        // UserId(0) is invalid
        let user_id = UserId::try_from(0).unwrap();
        let result = manager.get_user_dialog_boosts(dialog_id, user_id);
        assert!(result.is_err()); // UserId(0) is invalid
    }

    #[test]
    fn test_get_chat_boost_level_features() {
        let manager = BoostManager::new();

        // Level 0
        let features = manager.get_chat_boost_level_features(false, 0);
        assert_eq!(features.level, 0);
        assert_eq!(features.actual_level, 0);
        assert!(!features.can_set_emoji_status);

        // Level 5
        let features = manager.get_chat_boost_level_features(false, 5);
        assert_eq!(features.level, 5);
        assert_eq!(features.actual_level, 5);
        assert!(features.can_set_emoji_status);
        assert!(features.can_set_profile_background_custom_emoji);

        // Level > max
        let features = manager.get_chat_boost_level_features(false, 100);
        assert_eq!(features.level, 100);
        assert_eq!(features.actual_level, 10);

        // Negative level
        let features = manager.get_chat_boost_level_features(false, -1);
        assert_eq!(features.level, -1);
        assert_eq!(features.actual_level, 0);
    }
}
