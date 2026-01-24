// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Boost Manager
//!
//! Manager for Telegram chat boosts.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `BoostManager` class from `td/telegram/BoostManager.h`.
//!
//! ## Overview
//!
//! The BoostManager handles:
//! - Boost level features
//! - User boost slots
//! - Dialog boost status
//! - Boost link information
//! - Boost queries and updates
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_boost_manager::BoostManager;
//!
//! let manager = BoostManager::new();
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
// RwLock poisoning is rare and panic is acceptable for manager pattern
#![allow(clippy::unwrap_used)]

use rustgram_boost::{
    ChatBoost, ChatBoostFeatures, ChatBoostLevelFeatures, ChatBoostSlots, ChatBoostStatus,
    DialogBoostLinkInfo, FoundChatBoosts,
};
use rustgram_dialog_id::DialogId;
use rustgram_types::UserId;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};

/// Errors that can occur in BoostManager operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoostError {
    /// Dialog not found.
    DialogNotFound(DialogId),
    /// User not found.
    UserNotFound(UserId),
    /// Invalid boost slot.
    InvalidBoostSlot(i32),
    /// Invalid URL.
    InvalidUrl(String),
    /// Boost link not found.
    BoostLinkNotFound,
    /// Operation failed.
    OperationFailed(String),
}

impl fmt::Display for BoostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DialogNotFound(id) => write!(f, "Dialog not found: {}", id),
            Self::UserNotFound(id) => write!(f, "User not found: {}", id),
            Self::InvalidBoostSlot(slot) => write!(f, "Invalid boost slot: {}", slot),
            Self::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            Self::BoostLinkNotFound => write!(f, "Boost link not found"),
            Self::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
        }
    }
}

impl std::error::Error for BoostError {}

/// Result type for BoostManager operations.
pub type Result<T> = std::result::Result<T, BoostError>;

/// Default boost features for megagroups.
const DEFAULT_MEGAGROUP_FEATURES: ChatBoostLevelFeatures = ChatBoostLevelFeatures {
    level: 1,
    actual_level: 1,
    profile_accent_color_count: 0,
    title_color_count: 0,
    can_set_profile_background_custom_emoji: false,
    accent_color_count: 0,
    can_set_background_custom_emoji: false,
    can_set_emoji_status: false,
    chat_theme_count: 0,
    can_set_custom_background: false,
    can_set_custom_emoji_sticker_set: false,
    can_enable_autotranslation: false,
    can_recognize_speech: false,
    can_restrict_sponsored_messages: false,
};

/// Default boost features for broadcast channels.
const DEFAULT_BROADCAST_FEATURES: ChatBoostLevelFeatures = ChatBoostLevelFeatures {
    level: 1,
    actual_level: 1,
    profile_accent_color_count: 0,
    title_color_count: 0,
    can_set_profile_background_custom_emoji: false,
    accent_color_count: 0,
    can_set_background_custom_emoji: false,
    can_set_emoji_status: false,
    chat_theme_count: 0,
    can_set_custom_background: false,
    can_set_custom_emoji_sticker_set: false,
    can_enable_autotranslation: false,
    can_recognize_speech: false,
    can_restrict_sponsored_messages: false,
};

/// Manager for Telegram chat boosts.
///
/// Handles boost-related operations including status checks, slot management,
/// and boost link information.
///
/// # TDLib Alignment
///
/// Corresponds to TDLib `BoostManager` class.
///
/// # Example
///
/// ```rust
/// use rustgram_boost_manager::BoostManager;
///
/// let manager = BoostManager::new();
/// ```
#[derive(Debug)]
pub struct BoostManager {
    /// Stored boost slots for users.
    boost_slots: Arc<RwLock<HashMap<DialogId, ChatBoostSlots>>>,
    /// Dialog boost link info.
    boost_links: Arc<RwLock<HashMap<DialogId, DialogBoostLinkInfo>>>,
    /// URL to dialog mapping for boost links.
    url_to_dialog: Arc<RwLock<HashMap<String, DialogId>>>,
}

impl Default for BoostManager {
    fn default() -> Self {
        Self::new()
    }
}

impl BoostManager {
    /// Creates a new BoostManager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_boost_manager::BoostManager;
    ///
    /// let manager = BoostManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            boost_slots: Arc::new(RwLock::new(HashMap::new())),
            boost_links: Arc::new(RwLock::new(HashMap::new())),
            url_to_dialog: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Gets boost level features object.
    ///
    /// # Arguments
    ///
    /// * `for_megagroup` - Whether to get features for megagroups (vs channels)
    /// * `level` - Requested boost level
    #[must_use]
    pub fn get_chat_boost_level_features_object(
        &self,
        for_megagroup: bool,
        level: i32,
    ) -> ChatBoostLevelFeatures {
        let actual_level = level.clamp(1, 10);

        if for_megagroup {
            ChatBoostLevelFeatures {
                level,
                actual_level,
                ..DEFAULT_MEGAGROUP_FEATURES
            }
        } else {
            ChatBoostLevelFeatures {
                level,
                actual_level,
                ..DEFAULT_BROADCAST_FEATURES
            }
        }
    }

    /// Gets all chat boost features.
    ///
    /// # Arguments
    ///
    /// * `for_megagroup` - Whether to get features for megagroups (vs channels)
    #[must_use]
    pub fn get_chat_boost_features_object(&self, for_megagroup: bool) -> ChatBoostFeatures {
        let mut features = ChatBoostFeatures::new();

        // Generate features for levels 1-10
        for level in 1..=10 {
            features
                .features
                .push(self.get_chat_boost_level_features_object(for_megagroup, level));
        }

        features
    }

    /// Gets boost slots for the current user.
    ///
    /// This is a stub implementation that returns empty slots.
    #[must_use]
    pub fn get_boost_slots(&self) -> ChatBoostSlots {
        ChatBoostSlots::new()
    }

    /// Gets boost status for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Errors
    ///
    /// Returns `BoostError::DialogNotFound` if the dialog doesn't exist.
    ///
    /// # Note
    ///
    /// This is a stub implementation. Real implementation would query TDLib API.
    pub fn get_dialog_boost_status(&self, dialog_id: DialogId) -> Result<ChatBoostStatus> {
        // Stub: return default status
        let _ = dialog_id; // Use parameter to avoid warning
        Ok(ChatBoostStatus {
            boost_url: None,
            my_boost_slots: Vec::new(),
            level: 0,
            gift_code_boost_count: 0,
            boost_count: 0,
            current_level_boost_count: 0,
            next_level_boost_count: 0,
            premium_member_count: 0,
            premium_member_percentage: 0.0,
            prepaid_giveaways: Vec::new(),
        })
    }

    /// Boosts a dialog using specified slots.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog to boost
    /// * `slot_ids` - Boost slot IDs to use
    ///
    /// # Errors
    ///
    /// Returns `BoostError::InvalidBoostSlot` if a slot is invalid.
    ///
    /// # Note
    ///
    /// This is a stub implementation.
    pub fn boost_dialog(&self, _dialog_id: DialogId, slot_ids: Vec<i32>) -> Result<ChatBoostSlots> {
        // Validate slots
        for &slot_id in &slot_ids {
            if slot_id < 0 {
                return Err(BoostError::InvalidBoostSlot(slot_id));
            }
        }

        // Stub: return empty slots
        Ok(ChatBoostSlots::new())
    }

    /// Gets boost link for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Errors
    ///
    /// Returns `BoostError::DialogNotFound` if no link exists.
    pub fn get_dialog_boost_link(&self, dialog_id: DialogId) -> Result<(String, bool)> {
        let links = self.boost_links.read().unwrap();
        let info = links
            .get(&dialog_id)
            .ok_or(BoostError::DialogNotFound(dialog_id))?;

        let url = if let Some(username) = &info.username {
            format!("https://t.me/{}", username)
        } else {
            format!("https://t.me/joinchat/{}", generate_random_string())
        };

        let is_public = info.username.is_some();

        Ok((url, is_public))
    }

    /// Gets boost link info from URL.
    ///
    /// # Arguments
    ///
    /// * `url` - Boost link URL
    ///
    /// # Errors
    ///
    /// Returns `BoostError::InvalidUrl` if URL is invalid.
    /// Returns `BoostError::BoostLinkNotFound` if link not found.
    pub fn get_dialog_boost_link_info(&self, url: &str) -> Result<DialogBoostLinkInfo> {
        if !url.starts_with("https://t.me/") && !url.starts_with("http://t.me/") {
            return Err(BoostError::InvalidUrl(url.to_string()));
        }

        let url_map = self.url_to_dialog.read().unwrap();
        let dialog_id = url_map.get(url).ok_or(BoostError::BoostLinkNotFound)?;

        let links = self.boost_links.read().unwrap();
        links
            .get(dialog_id)
            .cloned()
            .ok_or(BoostError::BoostLinkNotFound)
    }

    /// Gets boost link info as object.
    ///
    /// # Arguments
    ///
    /// * `info` - Dialog boost link info
    #[must_use]
    pub fn get_chat_boost_link_info_object(
        &self,
        info: &DialogBoostLinkInfo,
    ) -> DialogBoostLinkInfo {
        info.clone()
    }

    /// Gets boosts for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `only_gift_codes` - Whether to only return gift code boosts
    /// * `offset` - Pagination offset
    /// * `limit` - Maximum results
    ///
    /// # Note
    ///
    /// This is a stub implementation.
    #[must_use]
    pub fn get_dialog_boosts(
        &self,
        _dialog_id: DialogId,
        _only_gift_codes: bool,
        offset: String,
        _limit: i32,
    ) -> FoundChatBoosts {
        FoundChatBoosts {
            total_count: 0,
            boosts: Vec::new(),
            next_offset: offset,
        }
    }

    /// Gets boosts from a specific user for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `user_id` - User ID
    ///
    /// # Note
    ///
    /// This is a stub implementation.
    #[must_use]
    pub fn get_user_dialog_boosts(
        &self,
        _dialog_id: DialogId,
        _user_id: UserId,
    ) -> FoundChatBoosts {
        FoundChatBoosts::new()
    }

    /// Updates dialog boost information.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `boost` - Boost data
    ///
    /// # Note
    ///
    /// This is a stub implementation.
    pub fn on_update_dialog_boost(&self, dialog_id: DialogId, _boost: ChatBoost) {
        // Stub: could update internal state
        let _ = dialog_id;
    }

    /// Sets boost link info for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `info` - Boost link info
    pub fn set_dialog_boost_link_info(&self, dialog_id: DialogId, info: DialogBoostLinkInfo) {
        let mut links = self.boost_links.write().unwrap();
        let mut url_map = self.url_to_dialog.write().unwrap();

        let url = if let Some(username) = &info.username {
            format!("https://t.me/{}", username)
        } else {
            format!("https://t.me/joinchat/{}", dialog_id)
        };

        links.insert(dialog_id, info);
        url_map.insert(url, dialog_id);
    }

    /// Removes boost link info for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    pub fn remove_dialog_boost_link_info(&self, dialog_id: DialogId) {
        let mut links = self.boost_links.write().unwrap();
        let mut url_map = self.url_to_dialog.write().unwrap();

        if let Some(info) = links.remove(&dialog_id) {
            let url = if let Some(username) = &info.username {
                format!("https://t.me/{}", username)
            } else {
                format!("https://t.me/joinchat/{}", dialog_id)
            };
            url_map.remove(&url);
        }
    }

    /// Sets boost slots for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `slots` - Boost slots
    pub fn set_boost_slots(&self, dialog_id: DialogId, slots: ChatBoostSlots) {
        let mut boost_slots = self.boost_slots.write().unwrap();
        boost_slots.insert(dialog_id, slots);
    }

    /// Gets boost slots for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    #[must_use]
    pub fn get_dialog_boost_slots(&self, dialog_id: DialogId) -> Option<ChatBoostSlots> {
        let boost_slots = self.boost_slots.read().unwrap();
        boost_slots.get(&dialog_id).cloned()
    }
}

/// Generates a random string for chat join links.
fn generate_random_string() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    format!("{:x}", timestamp)[0..16].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_boost::{ChatBoostSlot, ChatBoostSource};

    fn create_test_manager() -> BoostManager {
        BoostManager::new()
    }

    // Constructor tests (2)
    #[test]
    fn test_manager_new() {
        let manager = BoostManager::new();
        let features = manager.get_chat_boost_level_features_object(true, 5);
        assert_eq!(features.level, 5);
    }

    #[test]
    fn test_manager_default() {
        let manager = BoostManager::default();
        assert_eq!(manager.get_boost_slots().slots.len(), 0);
    }

    // Feature tests (6)
    #[test]
    fn test_get_chat_boost_level_features_megagroup() {
        let manager = create_test_manager();
        let features = manager.get_chat_boost_level_features_object(true, 5);

        assert_eq!(features.level, 5);
        assert_eq!(features.actual_level, 5);
    }

    #[test]
    fn test_get_chat_boost_level_features_broadcast() {
        let manager = create_test_manager();
        let features = manager.get_chat_boost_level_features_object(false, 3);

        assert_eq!(features.level, 3);
        assert_eq!(features.actual_level, 3);
    }

    #[test]
    fn test_get_chat_boost_level_features_clamps_high() {
        let manager = create_test_manager();
        let features = manager.get_chat_boost_level_features_object(true, 100);

        assert_eq!(features.level, 100);
        assert_eq!(features.actual_level, 10);
    }

    #[test]
    fn test_get_chat_boost_level_features_clamps_low() {
        let manager = create_test_manager();
        let features = manager.get_chat_boost_level_features_object(true, 0);

        assert_eq!(features.level, 0);
        assert_eq!(features.actual_level, 1);
    }

    #[test]
    fn test_get_chat_boost_features_megagroup() {
        let manager = create_test_manager();
        let features = manager.get_chat_boost_features_object(true);

        assert_eq!(features.features.len(), 10);
    }

    #[test]
    fn test_get_chat_boost_features_broadcast() {
        let manager = create_test_manager();
        let features = manager.get_chat_boost_features_object(false);

        assert_eq!(features.features.len(), 10);
    }

    // Boost slots tests (5)
    #[test]
    fn test_get_boost_slots() {
        let manager = create_test_manager();
        let slots = manager.get_boost_slots();

        assert_eq!(slots.slots.len(), 0);
    }

    #[test]
    fn test_set_boost_slots() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);
        let slots = ChatBoostSlots::from_slots(vec![ChatBoostSlot::unused(1, 1000)]);

        manager.set_boost_slots(dialog_id, slots.clone());

        let retrieved = manager.get_dialog_boost_slots(dialog_id).unwrap();
        assert_eq!(retrieved.slots.len(), 1);
    }

    #[test]
    fn test_get_dialog_boost_slots_some() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);
        let slots = ChatBoostSlots::from_slots(vec![
            ChatBoostSlot::unused(1, 1000),
            ChatBoostSlot::unused(2, 2000),
        ]);

        manager.set_boost_slots(dialog_id, slots);

        let retrieved = manager.get_dialog_boost_slots(dialog_id).unwrap();
        assert_eq!(retrieved.slots.len(), 2);
    }

    #[test]
    fn test_get_dialog_boost_slots_none() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);

        assert!(manager.get_dialog_boost_slots(dialog_id).is_none());
    }

    #[test]
    fn test_boost_slots_available_count() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);
        // Use only unused slots to avoid type mismatch
        let slots = ChatBoostSlots::from_slots(vec![
            ChatBoostSlot::unused(1, 1000),
            ChatBoostSlot::unused(2, 2000),
        ]);

        manager.set_boost_slots(dialog_id, slots);

        let retrieved = manager.get_dialog_boost_slots(dialog_id).unwrap();
        assert_eq!(retrieved.available_count(150), 2);
    }

    // Boost status tests (2)
    #[test]
    fn test_get_dialog_boost_status() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);

        let status = manager.get_dialog_boost_status(dialog_id).unwrap();
        assert_eq!(status.level, 0);
    }

    #[test]
    fn test_get_dialog_boost_status_fields() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);

        let status = manager.get_dialog_boost_status(dialog_id).unwrap();
        assert_eq!(status.boost_count, 0);
        assert_eq!(status.my_boost_slots.len(), 0);
    }

    // Boost dialog tests (3)
    #[test]
    fn test_boost_dialog_success() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);

        let result = manager.boost_dialog(dialog_id, vec![1, 2, 3]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_boost_dialog_invalid_slot() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);

        let result = manager.boost_dialog(dialog_id, vec![-1]);
        assert!(matches!(result, Err(BoostError::InvalidBoostSlot(-1))));
    }

    #[test]
    fn test_boost_dialog_empty_slots() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);

        let result = manager.boost_dialog(dialog_id, vec![]);
        assert!(result.is_ok());
    }

    // Boost link tests (10)
    #[test]
    fn test_get_dialog_boost_link_not_found() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);

        let result = manager.get_dialog_boost_link(dialog_id);
        assert_eq!(result, Err(BoostError::DialogNotFound(dialog_id)));
    }

    #[test]
    fn test_get_dialog_boost_link_public() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);
        let info = DialogBoostLinkInfo::public("testchannel".to_string());

        manager.set_dialog_boost_link_info(dialog_id, info);

        let (url, is_public) = manager.get_dialog_boost_link(dialog_id).unwrap();
        assert!(url.contains("testchannel"));
        assert!(is_public);
    }

    #[test]
    fn test_get_dialog_boost_link_private() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);
        let info = DialogBoostLinkInfo::private(rustgram_types::ChannelId::new(123).unwrap());

        manager.set_dialog_boost_link_info(dialog_id, info);

        let (_url, is_public) = manager.get_dialog_boost_link(dialog_id).unwrap();
        assert!(!is_public);
    }

    #[test]
    fn test_get_dialog_boost_link_info_invalid_url() {
        let manager = create_test_manager();

        let result = manager.get_dialog_boost_link_info("not a url");
        assert!(matches!(result, Err(BoostError::InvalidUrl(_))));
    }

    #[test]
    fn test_get_dialog_boost_link_info_not_found() {
        let manager = create_test_manager();

        let result = manager.get_dialog_boost_link_info("https://t.me/nonexistent");
        assert_eq!(result, Err(BoostError::BoostLinkNotFound));
    }

    #[test]
    fn test_get_dialog_boost_link_info_found() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);
        let info = DialogBoostLinkInfo::public("testchannel".to_string());

        manager.set_dialog_boost_link_info(dialog_id, info.clone());

        let result = manager
            .get_dialog_boost_link_info("https://t.me/testchannel")
            .unwrap();
        assert_eq!(result.username, info.username);
    }

    #[test]
    fn test_get_chat_boost_link_info_object() {
        let manager = create_test_manager();
        let info = DialogBoostLinkInfo::public("testchannel".to_string());

        let obj = manager.get_chat_boost_link_info_object(&info);
        assert_eq!(obj.username, Some("testchannel".to_string()));
    }

    #[test]
    fn test_get_chat_boost_link_info_object_private() {
        let manager = create_test_manager();
        let info = DialogBoostLinkInfo::private(rustgram_types::ChannelId::new(123).unwrap());

        let obj = manager.get_chat_boost_link_info_object(&info);
        assert!(obj.username.is_none());
    }

    #[test]
    fn test_remove_dialog_boost_link_info() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);
        let info = DialogBoostLinkInfo::public("testchannel".to_string());

        manager.set_dialog_boost_link_info(dialog_id, info);
        manager.remove_dialog_boost_link_info(dialog_id);

        let result = manager.get_dialog_boost_link(dialog_id);
        assert_eq!(result, Err(BoostError::DialogNotFound(dialog_id)));
    }

    // Get boosts tests (3)
    #[test]
    fn test_get_dialog_boosts() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);

        let result = manager.get_dialog_boosts(dialog_id, false, "offset".to_string(), 10);
        assert_eq!(result.total_count, 0);
        assert_eq!(result.boosts.len(), 0);
    }

    #[test]
    fn test_get_dialog_boosts_with_offset() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);

        let result = manager.get_dialog_boosts(dialog_id, true, "abc123".to_string(), 50);
        assert_eq!(result.next_offset, "abc123");
    }

    #[test]
    fn test_get_user_dialog_boosts() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);
        let user_id = UserId::new(123).unwrap();

        let result = manager.get_user_dialog_boosts(dialog_id, user_id);
        assert_eq!(result.total_count, 0);
    }

    // Update tests (2)
    #[test]
    fn test_on_update_dialog_boost() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);
        let user_id = UserId::new(123).unwrap();

        let boost = ChatBoost {
            id: "test".to_string(),
            multiplier: 1,
            source: ChatBoostSource::Premium { user_id },
            date: 1000,
            expiration_date: 2000,
        };

        // Should not panic
        manager.on_update_dialog_boost(dialog_id, boost);
    }

    #[test]
    fn test_on_update_dialog_boost_gift_code() {
        let manager = create_test_manager();
        let dialog_id = DialogId::new(1234567890);
        let user_id = UserId::new(123).unwrap();

        let boost = ChatBoost {
            id: "test".to_string(),
            multiplier: 1,
            source: ChatBoostSource::GiftCode {
                user_id,
                gift_slug: Some("test".to_string()),
            },
            date: 1000,
            expiration_date: 2000,
        };

        // Should not panic
        manager.on_update_dialog_boost(dialog_id, boost);
    }

    // Error display tests (7)
    #[test]
    fn test_error_display_dialog_not_found() {
        let dialog_id = DialogId::new(1234567890);
        let err = BoostError::DialogNotFound(dialog_id);
        let display = format!("{}", err);
        assert!(display.contains("1234567890"));
    }

    #[test]
    fn test_error_display_user_not_found() {
        let user_id = UserId::new(123).unwrap();
        let err = BoostError::UserNotFound(user_id);
        let display = format!("{}", err);
        assert!(display.contains("123"));
    }

    #[test]
    fn test_error_display_invalid_boost_slot() {
        let err = BoostError::InvalidBoostSlot(-5);
        let display = format!("{}", err);
        assert!(display.contains("-5"));
    }

    #[test]
    fn test_error_display_invalid_url() {
        let err = BoostError::InvalidUrl("bad url".to_string());
        let display = format!("{}", err);
        assert!(display.contains("bad url"));
    }

    #[test]
    fn test_error_display_boost_link_not_found() {
        let err = BoostError::BoostLinkNotFound;
        let display = format!("{}", err);
        assert!(display.contains("not found"));
    }

    #[test]
    fn test_error_display_operation_failed() {
        let err = BoostError::OperationFailed("test error".to_string());
        let display = format!("{}", err);
        assert!(display.contains("test error"));
    }

    #[test]
    fn test_error_eq() {
        let err1 = BoostError::InvalidBoostSlot(5);
        let err2 = BoostError::InvalidBoostSlot(5);
        assert_eq!(err1, err2);
    }
}
