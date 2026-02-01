// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Notification group type enumeration for Telegram MTProto client.
//!
//! This module implements TDLib's NotificationGroupType.
//!
//! # Example
//!
//! ```rust
//! use rustgram_notification_group_type::NotificationGroupType;
//!
//! let group_type = NotificationGroupType::Messages;
//! assert!(group_type.is_messages());
//! assert_eq!(group_type.name(), "messages");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::fmt::{self, Display, Formatter};

/// Notification group type.
///
/// Based on TDLib's `NotificationGroupType` enum.
///
/// Represents the type of notification group in Telegram.
///
/// # Example
///
/// ```rust
/// use rustgram_notification_group_type::NotificationGroupType;
///
/// let messages = NotificationGroupType::Messages;
/// assert!(messages.is_messages());
/// assert!(!messages.is_mentions());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum NotificationGroupType {
    /// Messages notification group
    Messages = 0,

    /// Mentions notification group
    Mentions = 1,

    /// Secret chat notification group
    SecretChat = 2,

    /// Calls notification group
    Calls = 3,

    /// Unknown notification group type
    #[default]
    Unknown = 4,
}

impl NotificationGroupType {
    /// Creates a new NotificationGroupType from an i32 value.
    ///
    /// # Arguments
    ///
    /// * `value` - The integer value to convert
    ///
    /// # Returns
    ///
    /// Returns `Some(NotificationGroupType)` if the value is valid, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_type::NotificationGroupType;
    ///
    /// assert_eq!(NotificationGroupType::from_i32(0), Some(NotificationGroupType::Messages));
    /// assert_eq!(NotificationGroupType::from_i32(1), Some(NotificationGroupType::Mentions));
    /// assert_eq!(NotificationGroupType::from_i32(99), None);
    /// ```
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Messages),
            1 => Some(Self::Mentions),
            2 => Some(Self::SecretChat),
            3 => Some(Self::Calls),
            4 => Some(Self::Unknown),
            _ => None,
        }
    }

    /// Returns the i32 representation of this notification group type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_type::NotificationGroupType;
    ///
    /// assert_eq!(NotificationGroupType::Messages.to_i32(), 0);
    /// assert_eq!(NotificationGroupType::Mentions.to_i32(), 1);
    /// ```
    pub fn to_i32(self) -> i32 {
        self as i32
    }

    /// Returns the name of this notification group type.
    ///
    /// # Returns
    ///
    /// * `"messages"` for messages
    /// * `"mentions"` for mentions
    /// * `"secret_chat"` for secret chats
    /// * `"calls"` for calls
    /// * `"unknown"` for unknown types
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_type::NotificationGroupType;
    ///
    /// assert_eq!(NotificationGroupType::Messages.name(), "messages");
    /// assert_eq!(NotificationGroupType::Mentions.name(), "mentions");
    /// assert_eq!(NotificationGroupType::Unknown.name(), "unknown");
    /// ```
    pub fn name(self) -> &'static str {
        match self {
            Self::Messages => "messages",
            Self::Mentions => "mentions",
            Self::SecretChat => "secret_chat",
            Self::Calls => "calls",
            Self::Unknown => "unknown",
        }
    }

    /// Checks if this is a messages notification group.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is a messages group, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_type::NotificationGroupType;
    ///
    /// assert!(NotificationGroupType::Messages.is_messages());
    /// assert!(!NotificationGroupType::Mentions.is_messages());
    /// ```
    pub fn is_messages(self) -> bool {
        matches!(self, Self::Messages)
    }

    /// Checks if this is a mentions notification group.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is a mentions group, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_type::NotificationGroupType;
    ///
    /// assert!(NotificationGroupType::Mentions.is_mentions());
    /// assert!(!NotificationGroupType::Messages.is_mentions());
    /// ```
    pub fn is_mentions(self) -> bool {
        matches!(self, Self::Mentions)
    }

    /// Checks if this is a secret chat notification group.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is a secret chat group, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_type::NotificationGroupType;
    ///
    /// assert!(NotificationGroupType::SecretChat.is_secret_chat());
    /// assert!(!NotificationGroupType::Messages.is_secret_chat());
    /// ```
    pub fn is_secret_chat(self) -> bool {
        matches!(self, Self::SecretChat)
    }

    /// Checks if this is a calls notification group.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is a calls group, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_type::NotificationGroupType;
    ///
    /// assert!(NotificationGroupType::Calls.is_calls());
    /// assert!(!NotificationGroupType::Messages.is_calls());
    /// ```
    pub fn is_calls(self) -> bool {
        matches!(self, Self::Calls)
    }

    /// Checks if this notification group type is unknown.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is an unknown type, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_type::NotificationGroupType;
    ///
    /// assert!(NotificationGroupType::Unknown.is_unknown());
    /// assert!(!NotificationGroupType::Messages.is_unknown());
    /// ```
    pub fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Checks if this notification group type is supported.
    ///
    /// # Returns
    ///
    /// Returns `true` if the type is supported, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_type::NotificationGroupType;
    ///
    /// assert!(NotificationGroupType::Messages.is_supported());
    /// assert!(!NotificationGroupType::Unknown.is_supported());
    /// ```
    pub fn is_supported(self) -> bool {
        !matches!(self, Self::Unknown)
    }

    /// Checks if this notification group type can have sound.
    ///
    /// # Returns
    ///
    /// Returns `true` if the type can have sound, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_type::NotificationGroupType;
    ///
    /// assert!(NotificationGroupType::Messages.can_have_sound());
    /// assert!(NotificationGroupType::Calls.can_have_sound());
    /// ```
    pub fn can_have_sound(self) -> bool {
        matches!(self, Self::Messages | Self::Mentions | Self::Calls)
    }

    /// Checks if this notification group type can be muted.
    ///
    /// # Returns
    ///
    /// Returns `true` if the type can be muted, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_type::NotificationGroupType;
    ///
    /// assert!(NotificationGroupType::Messages.can_be_muted());
    /// assert!(NotificationGroupType::Mentions.can_be_muted());
    /// ```
    pub fn can_be_muted(self) -> bool {
        matches!(self, Self::Messages | Self::Mentions | Self::SecretChat)
    }
}

impl Display for NotificationGroupType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_i32() {
        assert_eq!(
            NotificationGroupType::from_i32(0),
            Some(NotificationGroupType::Messages)
        );
        assert_eq!(
            NotificationGroupType::from_i32(1),
            Some(NotificationGroupType::Mentions)
        );
        assert_eq!(
            NotificationGroupType::from_i32(2),
            Some(NotificationGroupType::SecretChat)
        );
        assert_eq!(
            NotificationGroupType::from_i32(3),
            Some(NotificationGroupType::Calls)
        );
        assert_eq!(
            NotificationGroupType::from_i32(4),
            Some(NotificationGroupType::Unknown)
        );
        assert_eq!(NotificationGroupType::from_i32(-1), None);
        assert_eq!(NotificationGroupType::from_i32(5), None);
        assert_eq!(NotificationGroupType::from_i32(99), None);
    }

    #[test]
    fn test_to_i32() {
        assert_eq!(NotificationGroupType::Messages.to_i32(), 0);
        assert_eq!(NotificationGroupType::Mentions.to_i32(), 1);
        assert_eq!(NotificationGroupType::SecretChat.to_i32(), 2);
        assert_eq!(NotificationGroupType::Calls.to_i32(), 3);
        assert_eq!(NotificationGroupType::Unknown.to_i32(), 4);
    }

    #[test]
    fn test_roundtrip_conversion() {
        for value in 0i32..=4 {
            let group_type = NotificationGroupType::from_i32(value);
            assert_eq!(group_type.map(|gt| gt.to_i32()), Some(value));
        }
    }

    #[test]
    fn test_name() {
        assert_eq!(NotificationGroupType::Messages.name(), "messages");
        assert_eq!(NotificationGroupType::Mentions.name(), "mentions");
        assert_eq!(NotificationGroupType::SecretChat.name(), "secret_chat");
        assert_eq!(NotificationGroupType::Calls.name(), "calls");
        assert_eq!(NotificationGroupType::Unknown.name(), "unknown");
    }

    #[test]
    fn test_is_messages() {
        assert!(NotificationGroupType::Messages.is_messages());
        assert!(!NotificationGroupType::Mentions.is_messages());
        assert!(!NotificationGroupType::SecretChat.is_messages());
        assert!(!NotificationGroupType::Calls.is_messages());
        assert!(!NotificationGroupType::Unknown.is_messages());
    }

    #[test]
    fn test_is_mentions() {
        assert!(!NotificationGroupType::Messages.is_mentions());
        assert!(NotificationGroupType::Mentions.is_mentions());
        assert!(!NotificationGroupType::SecretChat.is_mentions());
        assert!(!NotificationGroupType::Calls.is_mentions());
        assert!(!NotificationGroupType::Unknown.is_mentions());
    }

    #[test]
    fn test_is_secret_chat() {
        assert!(!NotificationGroupType::Messages.is_secret_chat());
        assert!(!NotificationGroupType::Mentions.is_secret_chat());
        assert!(NotificationGroupType::SecretChat.is_secret_chat());
        assert!(!NotificationGroupType::Calls.is_secret_chat());
        assert!(!NotificationGroupType::Unknown.is_secret_chat());
    }

    #[test]
    fn test_is_calls() {
        assert!(!NotificationGroupType::Messages.is_calls());
        assert!(!NotificationGroupType::Mentions.is_calls());
        assert!(!NotificationGroupType::SecretChat.is_calls());
        assert!(NotificationGroupType::Calls.is_calls());
        assert!(!NotificationGroupType::Unknown.is_calls());
    }

    #[test]
    fn test_is_unknown() {
        assert!(!NotificationGroupType::Messages.is_unknown());
        assert!(!NotificationGroupType::Mentions.is_unknown());
        assert!(!NotificationGroupType::SecretChat.is_unknown());
        assert!(!NotificationGroupType::Calls.is_unknown());
        assert!(NotificationGroupType::Unknown.is_unknown());
    }

    #[test]
    fn test_is_supported() {
        assert!(NotificationGroupType::Messages.is_supported());
        assert!(NotificationGroupType::Mentions.is_supported());
        assert!(NotificationGroupType::SecretChat.is_supported());
        assert!(NotificationGroupType::Calls.is_supported());
        assert!(!NotificationGroupType::Unknown.is_supported());
    }

    #[test]
    fn test_can_have_sound() {
        assert!(NotificationGroupType::Messages.can_have_sound());
        assert!(NotificationGroupType::Mentions.can_have_sound());
        assert!(!NotificationGroupType::SecretChat.can_have_sound());
        assert!(NotificationGroupType::Calls.can_have_sound());
        assert!(!NotificationGroupType::Unknown.can_have_sound());
    }

    #[test]
    fn test_can_be_muted() {
        assert!(NotificationGroupType::Messages.can_be_muted());
        assert!(NotificationGroupType::Mentions.can_be_muted());
        assert!(NotificationGroupType::SecretChat.can_be_muted());
        assert!(!NotificationGroupType::Calls.can_be_muted());
        assert!(!NotificationGroupType::Unknown.can_be_muted());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", NotificationGroupType::Messages), "messages");
        assert_eq!(format!("{}", NotificationGroupType::Mentions), "mentions");
        assert_eq!(
            format!("{}", NotificationGroupType::SecretChat),
            "secret_chat"
        );
        assert_eq!(format!("{}", NotificationGroupType::Calls), "calls");
        assert_eq!(format!("{}", NotificationGroupType::Unknown), "unknown");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", NotificationGroupType::Messages), "Messages");
        assert_eq!(format!("{:?}", NotificationGroupType::Mentions), "Mentions");
        assert_eq!(
            format!("{:?}", NotificationGroupType::SecretChat),
            "SecretChat"
        );
        assert_eq!(format!("{:?}", NotificationGroupType::Calls), "Calls");
        assert_eq!(format!("{:?}", NotificationGroupType::Unknown), "Unknown");
    }

    #[test]
    fn test_default() {
        assert_eq!(
            NotificationGroupType::default(),
            NotificationGroupType::Unknown
        );
    }

    #[test]
    fn test_equality() {
        assert_eq!(
            NotificationGroupType::Messages,
            NotificationGroupType::Messages
        );
        assert_eq!(
            NotificationGroupType::Mentions,
            NotificationGroupType::Mentions
        );
        assert_eq!(
            NotificationGroupType::SecretChat,
            NotificationGroupType::SecretChat
        );
        assert_eq!(NotificationGroupType::Calls, NotificationGroupType::Calls);
        assert_eq!(
            NotificationGroupType::Unknown,
            NotificationGroupType::Unknown
        );
    }

    #[test]
    fn test_inequality() {
        assert_ne!(
            NotificationGroupType::Messages,
            NotificationGroupType::Mentions
        );
        assert_ne!(
            NotificationGroupType::Mentions,
            NotificationGroupType::SecretChat
        );
        assert_ne!(
            NotificationGroupType::SecretChat,
            NotificationGroupType::Calls
        );
        assert_ne!(NotificationGroupType::Calls, NotificationGroupType::Unknown);
    }

    #[test]
    fn test_copy() {
        let a = NotificationGroupType::Messages;
        let b = a;
        assert_eq!(a, NotificationGroupType::Messages);
        assert_eq!(b, NotificationGroupType::Messages);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(NotificationGroupType::Messages);
        set.insert(NotificationGroupType::Mentions);
        set.insert(NotificationGroupType::SecretChat);
        set.insert(NotificationGroupType::Calls);
        set.insert(NotificationGroupType::Unknown);
        assert_eq!(set.len(), 5);
    }

    #[test]
    fn test_all_types_distinct() {
        let types = [
            NotificationGroupType::Messages,
            NotificationGroupType::Mentions,
            NotificationGroupType::SecretChat,
            NotificationGroupType::Calls,
            NotificationGroupType::Unknown,
        ];

        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j]);
            }
        }
    }

    #[test]
    fn test_messages_properties() {
        let messages = NotificationGroupType::Messages;
        assert!(messages.is_messages());
        assert!(messages.is_supported());
        assert!(messages.can_have_sound());
        assert!(messages.can_be_muted());
    }

    #[test]
    fn test_mentions_properties() {
        let mentions = NotificationGroupType::Mentions;
        assert!(mentions.is_mentions());
        assert!(mentions.is_supported());
        assert!(mentions.can_have_sound());
        assert!(mentions.can_be_muted());
    }

    #[test]
    fn test_secret_chat_properties() {
        let secret_chat = NotificationGroupType::SecretChat;
        assert!(secret_chat.is_secret_chat());
        assert!(secret_chat.is_supported());
        assert!(!secret_chat.can_have_sound());
        assert!(secret_chat.can_be_muted());
    }

    #[test]
    fn test_calls_properties() {
        let calls = NotificationGroupType::Calls;
        assert!(calls.is_calls());
        assert!(calls.is_supported());
        assert!(calls.can_have_sound());
        assert!(!calls.can_be_muted());
    }

    #[test]
    fn test_unknown_properties() {
        let unknown = NotificationGroupType::Unknown;
        assert!(unknown.is_unknown());
        assert!(!unknown.is_supported());
        assert!(!unknown.can_have_sound());
        assert!(!unknown.can_be_muted());
    }

    #[test]
    fn test_i32_values() {
        assert_eq!(NotificationGroupType::Messages.to_i32(), 0);
        assert_eq!(NotificationGroupType::Mentions.to_i32(), 1);
        assert_eq!(NotificationGroupType::SecretChat.to_i32(), 2);
        assert_eq!(NotificationGroupType::Calls.to_i32(), 3);
        assert_eq!(NotificationGroupType::Unknown.to_i32(), 4);
    }

    #[test]
    fn test_from_invalid_i32() {
        assert_eq!(NotificationGroupType::from_i32(-100), None);
        assert_eq!(NotificationGroupType::from_i32(100), None);
        assert_eq!(NotificationGroupType::from_i32(i32::MAX), None);
    }

    #[test]
    fn test_type_count() {
        let types = [
            NotificationGroupType::Messages,
            NotificationGroupType::Mentions,
            NotificationGroupType::SecretChat,
            NotificationGroupType::Calls,
            NotificationGroupType::Unknown,
        ];
        assert_eq!(types.len(), 5);
    }
}
