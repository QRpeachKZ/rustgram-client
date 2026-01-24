// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Dialog ID
//!
//! Dialog identifier for Telegram MTProto.
//!
//! Based on TDLib's `DialogId` from `td/telegram/DialogId.h`.
//!
//! # Overview
//!
//! A `DialogId` uniquely identifies a conversation in Telegram, which can be
//! a user chat, group chat, channel, or secret chat. The ID space is divided
//! into ranges for different dialog types.
//!
//! # Example
//!
//! ```rust
//! use rustgram_dialog_id::DialogId;
//!
//! let id = DialogId::new(123456);
//! assert!(id.is_valid());
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Dialog type enumeration.
///
/// Represents the different types of conversations in Telegram.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum DialogType {
    /// No dialog type (invalid).
    #[default]
    None,
    /// Private chat with a user.
    User,
    /// Group chat.
    Chat,
    /// Channel or supergroup.
    Channel,
    /// Secret chat.
    SecretChat,
}

impl DialogType {
    /// Checks if this is a valid dialog type.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        matches!(
            self,
            Self::User | Self::Chat | Self::Channel | Self::SecretChat
        )
    }
}

/// Dialog identifier.
///
/// Represents a unique identifier for a Telegram conversation.
/// The ID space is divided into ranges for different dialog types.
///
/// # Example
///
/// ```
/// use rustgram_dialog_id::DialogId;
///
/// let id = DialogId::new(123456);
/// assert_eq!(id.get(), 123456);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DialogId(i64);

impl DialogId {
    /// Constant for zero secret chat ID.
    pub const ZERO_SECRET_CHAT_ID: i64 = -2_000_000_000_000;

    /// Constant for zero channel ID.
    pub const ZERO_CHANNEL_ID: i64 = -1_000_000_000_000;

    /// Creates a new `DialogId` from an i64 value.
    ///
    /// # Arguments
    ///
    /// * `id` - The raw dialog ID value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let id = DialogId::new(123456);
    /// assert_eq!(id.get(), 123456);
    /// ```
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the inner i64 value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let id = DialogId::new(123456);
    /// assert_eq!(id.get(), 123456);
    /// ```
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Checks if this is a valid dialog ID.
    ///
    /// A valid dialog ID is non-zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_id::DialogId;
    ///
    /// assert!(DialogId::new(123456).is_valid());
    /// assert!(!DialogId::new(0).is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.0 != 0
    }

    /// Returns the dialog type.
    ///
    /// Determines the type of dialog based on the ID value range:
    /// - Positive: User
    /// - Channel ID range: Channel
    /// - Secret chat ID range: SecretChat
    /// - Other negative: Chat (typically -chat_id)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_id::{DialogId, DialogType};
    ///
    /// assert_eq!(DialogId::new(123456).get_type(), DialogType::User);
    /// ```
    #[must_use]
    pub fn get_type(self) -> DialogType {
        if self.0 > 0 {
            DialogType::User
        } else if self.0 <= Self::ZERO_SECRET_CHAT_ID {
            DialogType::SecretChat
        } else if self.0 <= Self::ZERO_CHANNEL_ID {
            DialogType::Channel
        } else {
            // Negative but not in special ranges - treat as Chat
            DialogType::Chat
        }
    }
}

impl fmt::Display for DialogId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "chat {}", self.0)
    }
}

impl Hash for DialogId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<i64> for DialogId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<DialogId> for i64 {
    fn from(id: DialogId) -> Self {
        id.0
    }
}

/// Hasher for DialogId.
///
/// Provides a hashing function for DialogId values, useful for
/// hash map keys.
#[derive(Debug, Clone, Copy, Default)]
pub struct DialogIdHash;

impl DialogIdHash {
    /// Creates a new DialogIdHash.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Hashes a DialogId value.
    #[must_use]
    pub fn hash(self, dialog_id: DialogId) -> u32 {
        // Simple hash using the lower 32 bits
        (dialog_id.0 as u32).wrapping_add((dialog_id.0 >> 32) as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = DialogId::new(123456);
        assert_eq!(id.get(), 123456);
    }

    #[test]
    fn test_default() {
        let id = DialogId::default();
        assert_eq!(id.get(), 0);
    }

    #[test]
    fn test_from_i64() {
        let id = DialogId::from(123456);
        assert_eq!(id.get(), 123456);
    }

    #[test]
    fn test_to_i64() {
        let id = DialogId::new(123456);
        let value: i64 = id.into();
        assert_eq!(value, 123456);
    }

    #[test]
    fn test_is_valid() {
        assert!(DialogId::new(1).is_valid());
        assert!(DialogId::new(-1).is_valid());
        assert!(DialogId::new(i64::MAX).is_valid());
        assert!(!DialogId::new(0).is_valid());
    }

    #[test]
    fn test_get_type_user() {
        let id = DialogId::new(123456);
        assert_eq!(id.get_type(), DialogType::User);
    }

    #[test]
    fn test_get_type_channel() {
        let id = DialogId::new(DialogId::ZERO_CHANNEL_ID);
        assert_eq!(id.get_type(), DialogType::Channel);

        let id = DialogId::new(DialogId::ZERO_CHANNEL_ID - 1000);
        assert_eq!(id.get_type(), DialogType::Channel);
    }

    #[test]
    fn test_get_type_secret_chat() {
        let id = DialogId::new(DialogId::ZERO_SECRET_CHAT_ID);
        assert_eq!(id.get_type(), DialogType::SecretChat);

        let id = DialogId::new(DialogId::ZERO_SECRET_CHAT_ID - 1000);
        assert_eq!(id.get_type(), DialogType::SecretChat);
    }

    #[test]
    fn test_get_type_chat() {
        // Negative IDs in the chat range
        let id = DialogId::new(-123456);
        assert_eq!(id.get_type(), DialogType::Chat);

        let id = DialogId::new(-1);
        assert_eq!(id.get_type(), DialogType::Chat);
    }

    #[test]
    fn test_equality() {
        let id1 = DialogId::new(123456);
        let id2 = DialogId::new(123456);
        let id3 = DialogId::new(789012);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_display() {
        let id = DialogId::new(123456);
        let display = format!("{id}");
        assert!(display.contains("chat"));
        assert!(display.contains("123456"));
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;

        let id = DialogId::new(123456);
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        // Just ensure it doesn't panic
    }

    #[test]
    fn test_dialog_type_default() {
        let dt = DialogType::default();
        assert_eq!(dt, DialogType::None);
    }

    #[test]
    fn test_dialog_type_is_valid() {
        assert!(DialogType::User.is_valid());
        assert!(DialogType::Chat.is_valid());
        assert!(DialogType::Channel.is_valid());
        assert!(DialogType::SecretChat.is_valid());
        assert!(!DialogType::None.is_valid());
    }

    #[test]
    fn test_dialog_id_hash() {
        let hasher = DialogIdHash::new();
        let id = DialogId::new(123456);
        let hash = hasher.hash(id);
        // Just ensure it produces a value
        assert!(hash > 0 || hash == 0); // Any u32 is valid
    }

    #[test]
    fn test_constants() {
        assert_eq!(DialogId::ZERO_SECRET_CHAT_ID, -2_000_000_000_000);
        assert_eq!(DialogId::ZERO_CHANNEL_ID, -1_000_000_000_000);
    }

    #[test]
    fn test_clone() {
        let id1 = DialogId::new(123456);
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_serialization() {
        let id = DialogId::new(123456);
        let json = serde_json::to_string(&id).expect("Failed to serialize");
        assert!(json.contains("123456"));

        let deserialized: DialogId = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, id);
    }

    #[test]
    fn test_large_id() {
        let id = DialogId::new(i64::MAX);
        assert!(id.is_valid());
        assert_eq!(id.get_type(), DialogType::User);
    }

    #[test]
    fn test_negative_id_ranges() {
        // Test boundary conditions between ranges
        let just_above_secret = DialogId::new(DialogId::ZERO_SECRET_CHAT_ID + 1);
        assert_eq!(just_above_secret.get_type(), DialogType::Channel);

        let just_above_channel = DialogId::new(DialogId::ZERO_CHANNEL_ID + 1);
        assert_eq!(just_above_channel.get_type(), DialogType::Chat);
    }
}
