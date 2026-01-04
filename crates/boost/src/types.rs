// Copyright 2025 rustgram-client
//
// Licensed under MIT License

//! Boost module types for Telegram client.
//!
//! This module provides types for managing chat boosts in Telegram.

use rustgram_types::{ChannelId, DialogId, UserId};
use serde::{Deserialize, Serialize};

/// Information about a boost link for a dialog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DialogBoostLinkInfo {
    /// Username of the chat (if public)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Channel ID (if private)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,
}

impl DialogBoostLinkInfo {
    /// Create a new public boost link info.
    pub fn public(username: String) -> Self {
        Self {
            username: Some(username),
            channel_id: None,
        }
    }

    /// Create a new private boost link info.
    pub fn private(channel_id: ChannelId) -> Self {
        Self {
            username: None,
            channel_id: Some(channel_id),
        }
    }

    /// Check if this is a public link.
    pub fn is_public(&self) -> bool {
        self.username.is_some()
    }
}

/// Source of a chat boost.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ChatBoostSource {
    /// Boost from a premium subscription.
    Premium {
        /// User who gifted the boost
        user_id: UserId,
    },
    /// Boost from a gift code.
    GiftCode {
        /// User who gifted the code
        user_id: UserId,
        /// Gift slug used
        #[serde(skip_serializing_if = "Option::is_none")]
        gift_slug: Option<String>,
    },
    /// Boost from a giveaway.
    Giveaway {
        /// User who started the giveaway
        #[serde(skip_serializing_if = "Option::is_none")]
        user_id: Option<UserId>,
        /// Gift slug used
        #[serde(skip_serializing_if = "Option::is_none")]
        gift_slug: Option<String>,
        /// Stars awarded
        #[serde(skip_serializing_if = "Option::is_none")]
        stars: Option<i64>,
        /// Giveaway message ID
        giveaway_message_id: i32,
        /// Whether the boost was unclaimed
        unclaimed: bool,
    },
}

/// A chat boost.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoost {
    /// Unique boost identifier
    pub id: String,
    /// Multiplier for this boost
    pub multiplier: i32,
    /// Source of the boost
    pub source: ChatBoostSource,
    /// When the boost was started (Unix timestamp)
    pub date: i32,
    /// When the boost expires (Unix timestamp)
    pub expiration_date: i32,
}

impl ChatBoost {
    /// Check if this boost is expired.
    pub fn is_expired(&self, current_time: i32) -> bool {
        self.expiration_date <= current_time
    }
}

/// A single boost slot owned by the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostSlot {
    /// Slot identifier
    pub slot_id: i32,
    /// Dialog where the boost is applied (None if unused)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_id: Option<DialogId>,
    /// When the boost started (0 if unused)
    #[serde(default)]
    pub start_date: i32,
    /// When the boost will expire
    pub expiration_date: i32,
    /// When another boost can be applied from this slot
    #[serde(default)]
    pub cooldown_until_date: i32,
}

impl ChatBoostSlot {
    /// Create an unused boost slot.
    pub fn unused(slot_id: i32, expiration_date: i32) -> Self {
        Self {
            slot_id,
            dialog_id: None,
            start_date: 0,
            expiration_date,
            cooldown_until_date: 0,
        }
    }

    /// Create an active boost slot.
    pub fn active(
        slot_id: i32,
        dialog_id: DialogId,
        start_date: i32,
        expiration_date: i32,
        cooldown_until_date: i32,
    ) -> Self {
        Self {
            slot_id,
            dialog_id: Some(dialog_id),
            start_date,
            expiration_date,
            cooldown_until_date,
        }
    }

    /// Check if this slot is currently in use.
    pub fn is_used(&self) -> bool {
        self.dialog_id.is_some()
    }

    /// Check if this slot is on cooldown.
    pub fn is_on_cooldown(&self, current_time: i32) -> bool {
        self.cooldown_until_date > current_time
    }
}

/// Collection of boost slots owned by the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostSlots {
    /// List of boost slots
    pub slots: Vec<ChatBoostSlot>,
}

impl ChatBoostSlots {
    /// Create empty boost slots.
    pub fn new() -> Self {
        Self { slots: Vec::new() }
    }

    /// Create boost slots from a list.
    pub fn from_slots(slots: Vec<ChatBoostSlot>) -> Self {
        Self { slots }
    }

    /// Get the number of available (unused) slots.
    pub fn available_count(&self, current_time: i32) -> usize {
        self.slots
            .iter()
            .filter(|slot| !slot.is_used() && !slot.is_on_cooldown(current_time))
            .count()
    }
}

impl Default for ChatBoostSlots {
    fn default() -> Self {
        Self::new()
    }
}

/// Prize type for a prepaid giveaway.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GiveawayPrize {
    /// Premium subscription prize.
    Premium {
        /// Number of months
        months: i32,
    },
    /// Stars prize.
    Stars {
        /// Number of stars
        stars: i64,
    },
}

/// A prepaid giveaway.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrepaidGiveaway {
    /// Unique giveaway identifier
    pub id: i64,
    /// Quantity of prizes
    pub quantity: i32,
    /// Prize type
    pub prize: GiveawayPrize,
    /// Number of boosts this giveaway provides
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boosts: Option<i32>,
    /// When the giveaway was created (Unix timestamp)
    pub date: i32,
}

/// Features available at a specific boost level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostLevelFeatures {
    /// Requested boost level
    pub level: i32,
    /// Actual boost level (clamped to max)
    pub actual_level: i32,
    /// Number of custom emoji colors for profile background
    #[serde(default)]
    pub profile_accent_color_count: i32,
    /// Number of title colors
    #[serde(default)]
    pub title_color_count: i32,
    /// Can set custom emoji on profile background
    #[serde(default)]
    pub can_set_profile_background_custom_emoji: bool,
    /// Number of accent colors
    #[serde(default)]
    pub accent_color_count: i32,
    /// Can set custom emoji on chat background
    #[serde(default)]
    pub can_set_background_custom_emoji: bool,
    /// Can set emoji status
    #[serde(default)]
    pub can_set_emoji_status: bool,
    /// Number of available chat themes
    #[serde(default)]
    pub chat_theme_count: i32,
    /// Can set custom background
    #[serde(default)]
    pub can_set_custom_background: bool,
    /// Can set custom emoji sticker set
    #[serde(default)]
    pub can_set_custom_emoji_sticker_set: bool,
    /// Can enable auto-translation
    #[serde(default)]
    pub can_enable_autotranslation: bool,
    /// Can enable speech recognition
    #[serde(default)]
    pub can_recognize_speech: bool,
    /// Can restrict sponsored messages
    #[serde(default)]
    pub can_restrict_sponsored_messages: bool,
}

/// Features available for chat boosts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostFeatures {
    /// Features for each boost level (1-10 and special levels)
    #[serde(default)]
    pub features: Vec<ChatBoostLevelFeatures>,
}

impl ChatBoostFeatures {
    /// Create empty boost features.
    pub fn new() -> Self {
        Self {
            features: Vec::new(),
        }
    }
}

impl Default for ChatBoostFeatures {
    fn default() -> Self {
        Self::new()
    }
}

/// Status of boosts for a chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostStatus {
    /// URL to boost the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_url: Option<String>,
    /// Slots occupied by the current user
    #[serde(default)]
    pub my_boost_slots: Vec<i32>,
    /// Current boost level
    pub level: i32,
    /// Number of boosts from gift codes
    #[serde(default)]
    pub gift_code_boost_count: i32,
    /// Total number of boosts
    pub boost_count: i32,
    /// Number of boosts for the current level
    pub current_level_boost_count: i32,
    /// Number of boosts needed for the next level (0 if max level)
    pub next_level_boost_count: i32,
    /// Number of premium members
    #[serde(default)]
    pub premium_member_count: i32,
    /// Percentage of premium members (0-100)
    #[serde(default)]
    pub premium_member_percentage: f64,
    /// Prepaid giveaways
    #[serde(default)]
    pub prepaid_giveaways: Vec<PrepaidGiveaway>,
}

/// Result of searching for boosts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundChatBoosts {
    /// Total number of boosts
    pub total_count: i32,
    /// List of boosts
    pub boosts: Vec<ChatBoost>,
    /// Offset for pagination
    #[serde(default)]
    pub next_offset: String,
}

impl FoundChatBoosts {
    /// Create empty result.
    pub fn new() -> Self {
        Self {
            total_count: 0,
            boosts: Vec::new(),
            next_offset: String::new(),
        }
    }

    /// Check if there are more results.
    pub fn has_more(&self) -> bool {
        !self.next_offset.is_empty()
    }
}

impl Default for FoundChatBoosts {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_boost_link_info_public() {
        let info = DialogBoostLinkInfo::public("mychannel".to_string());
        assert!(info.is_public());
        assert!(info.username.is_some());
        assert!(info.channel_id.is_none());
    }

    #[test]
    fn test_dialog_boost_link_info_private() {
        let channel_id = ChannelId::try_from(12345).unwrap();
        let info = DialogBoostLinkInfo::private(channel_id);
        assert!(!info.is_public());
        assert!(info.username.is_none());
        assert!(info.channel_id.is_some());
    }

    #[test]
    fn test_chat_boost_is_expired() {
        let user_id = UserId::try_from(123).unwrap();
        let boost = ChatBoost {
            id: "test".to_string(),
            multiplier: 1,
            source: ChatBoostSource::Premium { user_id },
            date: 1000,
            expiration_date: 2000,
        };
        assert!(!boost.is_expired(1500));
        assert!(boost.is_expired(2000));
        assert!(boost.is_expired(2500));
    }

    #[test]
    fn test_boost_slot_unused() {
        let slot = ChatBoostSlot::unused(1, 2000);
        assert!(!slot.is_used());
        assert!(!slot.is_on_cooldown(1500));
    }

    #[test]
    fn test_boost_slot_active() {
        let channel_id = ChannelId::try_from(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let slot = ChatBoostSlot::active(1, dialog_id, 1000, 2000, 1500);
        assert!(slot.is_used());
        assert!(slot.is_on_cooldown(1400));
        assert!(!slot.is_on_cooldown(1600));
    }

    #[test]
    fn test_boost_slots_available_count() {
        let channel_id = ChannelId::try_from(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let slots = ChatBoostSlots::from_slots(vec![
            ChatBoostSlot::unused(1, 2000),
            ChatBoostSlot::active(2, dialog_id, 1000, 2000, 1500),
            ChatBoostSlot::unused(3, 2000),
        ]);
        assert_eq!(slots.available_count(1500), 2); // 2 unused slots
    }
}
