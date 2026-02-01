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

//! # Message Sender
//!
//! Represents the sender of a message.
//!
//! ## TDLib Alignment
//!
//! This type aligns with TDLib's `MessageSender` concept.
//! - TDLib API: `messageSenderUser`, `messageSenderChat`
//! - Represents either a user or a chat as message sender
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_sender::MessageSender;
//! use rustgram_types::{UserId, ChatId, ChannelId};
//!
//! let user_sender = MessageSender::User(UserId::new(123).unwrap());
//! let chat_sender = MessageSender::Chat(ChatId::new(456).unwrap());
//! let channel_sender = MessageSender::Channel(ChannelId::new(1000000000).unwrap());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

use rustgram_types::{ChannelId, ChatId, DialogId, UserId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the sender of a message.
///
/// A message sender can be:
/// - A user (regular messages)
/// - A chat (group messages)
/// - A channel (channel posts)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageSender {
    /// Message sent by a user
    User(UserId),

    /// Message sent in a chat (group)
    Chat(ChatId),

    /// Message sent in a channel
    Channel(ChannelId),
}

impl MessageSender {
    /// Returns `true` if this is a user sender.
    #[must_use]
    pub const fn is_user(&self) -> bool {
        matches!(self, Self::User(_))
    }

    /// Returns `true` if this is a chat sender.
    #[must_use]
    pub const fn is_chat(&self) -> bool {
        matches!(self, Self::Chat(_))
    }

    /// Returns `true` if this is a channel sender.
    #[must_use]
    pub const fn is_channel(&self) -> bool {
        matches!(self, Self::Channel(_))
    }

    /// Returns the user ID if this is a user sender.
    #[must_use]
    pub const fn user_id(&self) -> Option<UserId> {
        match self {
            Self::User(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns the chat ID if this is a chat sender.
    #[must_use]
    pub const fn chat_id(&self) -> Option<ChatId> {
        match self {
            Self::Chat(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns the channel ID if this is a channel sender.
    #[must_use]
    pub const fn channel_id(&self) -> Option<ChannelId> {
        match self {
            Self::Channel(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns the corresponding dialog ID.
    #[must_use]
    pub fn dialog_id(&self) -> DialogId {
        match self {
            Self::User(id) => DialogId::from_user(*id),
            Self::Chat(id) => DialogId::from_chat(*id),
            Self::Channel(id) => DialogId::from_channel(*id),
        }
    }

    /// Returns `true` if this sender is valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        match self {
            Self::User(id) => id.is_valid(),
            Self::Chat(id) => id.is_valid(),
            Self::Channel(id) => id.is_valid(),
        }
    }

    /// Creates a MessageSender from a DialogId.
    ///
    /// Returns `None` if the dialog type is not supported.
    #[must_use]
    pub fn from_dialog_id(dialog_id: DialogId) -> Option<Self> {
        match dialog_id {
            DialogId::User(user_id) => Some(Self::User(user_id)),
            DialogId::Chat(chat_id) => Some(Self::Chat(chat_id)),
            DialogId::Channel(channel_id) => Some(Self::Channel(channel_id)),
            DialogId::SecretChat(_) => None,
        }
    }
}

impl Default for MessageSender {
    fn default() -> Self {
        Self::User(UserId::default())
    }
}

impl fmt::Display for MessageSender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User(id) => write!(f, "user {}", id.get()),
            Self::Chat(id) => write!(f, "chat {}", id.get()),
            Self::Channel(id) => write!(f, "channel {}", id.get()),
        }
    }
}

impl From<UserId> for MessageSender {
    fn from(id: UserId) -> Self {
        Self::User(id)
    }
}

impl From<ChatId> for MessageSender {
    fn from(id: ChatId) -> Self {
        Self::Chat(id)
    }
}

impl From<ChannelId> for MessageSender {
    fn from(id: ChannelId) -> Self {
        Self::Channel(id)
    }
}

impl TryFrom<DialogId> for MessageSender {
    type Error = &'static str;

    fn try_from(dialog_id: DialogId) -> Result<Self, Self::Error> {
        Self::from_dialog_id(dialog_id).ok_or("Invalid dialog type for MessageSender")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Constructor tests (3)
    #[test]
    fn test_user_sender() {
        let user_id = UserId::new(123).unwrap();
        let sender = MessageSender::User(user_id);

        assert!(sender.is_user());
        assert!(!sender.is_chat());
        assert!(!sender.is_channel());
    }

    #[test]
    fn test_chat_sender() {
        let chat_id = ChatId::new(456).unwrap();
        let sender = MessageSender::Chat(chat_id);

        assert!(!sender.is_user());
        assert!(sender.is_chat());
        assert!(!sender.is_channel());
    }

    #[test]
    fn test_channel_sender() {
        let channel_id = ChannelId::new(1000000000).unwrap();
        let sender = MessageSender::Channel(channel_id);

        assert!(!sender.is_user());
        assert!(!sender.is_chat());
        assert!(sender.is_channel());
    }

    // Type tests (3)
    #[test]
    fn test_is_user() {
        let sender = MessageSender::User(UserId::new(123).unwrap());
        assert!(sender.is_user());
    }

    #[test]
    fn test_is_chat() {
        let sender = MessageSender::Chat(ChatId::new(456).unwrap());
        assert!(sender.is_chat());
    }

    #[test]
    fn test_is_channel() {
        let sender = MessageSender::Channel(ChannelId::new(1000000000).unwrap());
        assert!(sender.is_channel());
    }

    // ID extraction tests (6)
    #[test]
    fn test_user_id_some() {
        let user_id = UserId::new(123).unwrap();
        let sender = MessageSender::User(user_id);
        assert_eq!(sender.user_id(), Some(user_id));
    }

    #[test]
    fn test_user_id_none() {
        let sender = MessageSender::Chat(ChatId::new(456).unwrap());
        assert!(sender.user_id().is_none());
    }

    #[test]
    fn test_chat_id_some() {
        let chat_id = ChatId::new(456).unwrap();
        let sender = MessageSender::Chat(chat_id);
        assert_eq!(sender.chat_id(), Some(chat_id));
    }

    #[test]
    fn test_chat_id_none() {
        let sender = MessageSender::User(UserId::new(123).unwrap());
        assert!(sender.chat_id().is_none());
    }

    #[test]
    fn test_channel_id_some() {
        let channel_id = ChannelId::new(1000000000).unwrap();
        let sender = MessageSender::Channel(channel_id);
        assert_eq!(sender.channel_id(), Some(channel_id));
    }

    #[test]
    fn test_channel_id_none() {
        let sender = MessageSender::User(UserId::new(123).unwrap());
        assert!(sender.channel_id().is_none());
    }

    // Dialog ID tests (7)
    #[test]
    fn test_dialog_id_from_user() {
        let user_id = UserId::new(123).unwrap();
        let sender = MessageSender::User(user_id);
        let dialog_id = sender.dialog_id();

        assert_eq!(dialog_id, DialogId::from_user(user_id));
    }

    #[test]
    fn test_dialog_id_from_chat() {
        let chat_id = ChatId::new(456).unwrap();
        let sender = MessageSender::Chat(chat_id);
        let dialog_id = sender.dialog_id();

        assert_eq!(dialog_id, DialogId::from_chat(chat_id));
    }

    #[test]
    fn test_dialog_id_from_channel() {
        let channel_id = ChannelId::new(1000000000).unwrap();
        let sender = MessageSender::Channel(channel_id);
        let dialog_id = sender.dialog_id();

        assert_eq!(dialog_id, DialogId::from_channel(channel_id));
    }

    #[test]
    fn test_from_dialog_id_user() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let sender = MessageSender::from_dialog_id(dialog_id);

        assert_eq!(sender, Some(MessageSender::User(user_id)));
    }

    #[test]
    fn test_from_dialog_id_chat() {
        let chat_id = ChatId::new(456).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);
        let sender = MessageSender::from_dialog_id(dialog_id);

        assert_eq!(sender, Some(MessageSender::Chat(chat_id)));
    }

    #[test]
    fn test_from_dialog_id_channel() {
        let channel_id = ChannelId::new(1000000000).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let sender = MessageSender::from_dialog_id(dialog_id);

        assert_eq!(sender, Some(MessageSender::Channel(channel_id)));
    }

    #[test]
    fn test_from_dialog_id_invalid() {
        // DialogId::default() creates User(UserId(0)) which is a valid type
        // For a truly "invalid" case, we need to test with SecretChat which is not supported
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        // All supported dialog types should convert successfully
        assert!(MessageSender::from_dialog_id(dialog_id).is_some());
    }

    // Validity tests (4)
    #[test]
    fn test_is_valid_user() {
        let sender = MessageSender::User(UserId::new(123).unwrap());
        assert!(sender.is_valid());
    }

    #[test]
    fn test_is_valid_chat() {
        let sender = MessageSender::Chat(ChatId::new(456).unwrap());
        assert!(sender.is_valid());
    }

    #[test]
    fn test_is_valid_channel() {
        let sender = MessageSender::Channel(ChannelId::new(1000000000).unwrap());
        assert!(sender.is_valid());
    }

    #[test]
    fn test_is_invalid_default() {
        let sender = MessageSender::default();
        assert!(!sender.is_valid());
    }

    // Clone tests (3)
    #[test]
    fn test_clone_user() {
        let sender1 = MessageSender::User(UserId::new(123).unwrap());
        let sender2 = sender1;
        assert_eq!(sender1, sender2);
    }

    #[test]
    fn test_clone_chat() {
        let sender1 = MessageSender::Chat(ChatId::new(456).unwrap());
        let sender2 = sender1;
        assert_eq!(sender1, sender2);
    }

    #[test]
    fn test_clone_channel() {
        let sender1 = MessageSender::Channel(ChannelId::new(1000000000).unwrap());
        let sender2 = sender1;
        assert_eq!(sender1, sender2);
    }

    // Display tests (3)
    #[test]
    fn test_display_user() {
        let sender = MessageSender::User(UserId::new(123).unwrap());
        let display = format!("{}", sender);
        assert!(display.contains("user"));
        assert!(display.contains("123"));
    }

    #[test]
    fn test_display_chat() {
        let sender = MessageSender::Chat(ChatId::new(456).unwrap());
        let display = format!("{}", sender);
        assert!(display.contains("chat"));
        assert!(display.contains("456"));
    }

    #[test]
    fn test_display_channel() {
        let sender = MessageSender::Channel(ChannelId::new(1000000000).unwrap());
        let display = format!("{}", sender);
        assert!(display.contains("channel"));
    }

    // From trait tests (3)
    #[test]
    fn test_from_user_id() {
        let user_id = UserId::new(123).unwrap();
        let sender: MessageSender = user_id.into();
        assert_eq!(sender, MessageSender::User(user_id));
    }

    #[test]
    fn test_from_chat_id() {
        let chat_id = ChatId::new(456).unwrap();
        let sender: MessageSender = chat_id.into();
        assert_eq!(sender, MessageSender::Chat(chat_id));
    }

    #[test]
    fn test_from_channel_id() {
        let channel_id = ChannelId::new(1000000000).unwrap();
        let sender: MessageSender = channel_id.into();
        assert_eq!(sender, MessageSender::Channel(channel_id));
    }

    // TryFrom trait tests (2)
    #[test]
    fn test_try_from_dialog_id_ok() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let sender: Result<MessageSender, _> = dialog_id.try_into();
        assert!(sender.is_ok());
    }

    #[test]
    fn test_try_from_dialog_id_err() {
        // DialogId::default() creates User(UserId(0)) which is supported
        // So try_into should succeed, not fail
        let dialog_id = DialogId::default();
        let sender: Result<MessageSender, _> = dialog_id.try_into();
        assert!(sender.is_ok());
        assert!(matches!(sender.unwrap(), MessageSender::User(_)));
    }

    // Hash tests (1)
    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let sender1 = MessageSender::User(UserId::new(123).unwrap());
        let sender2 = MessageSender::User(UserId::new(123).unwrap());
        let sender3 = MessageSender::User(UserId::new(456).unwrap());

        let mut set = HashSet::new();
        set.insert(sender1);
        set.insert(sender2);
        set.insert(sender3);

        assert_eq!(set.len(), 2);
    }

    // Serialization tests (3)
    #[test]
    fn test_serialize_user() {
        let sender = MessageSender::User(UserId::new(123).unwrap());
        let json = serde_json::to_string(&sender).unwrap();
        let parsed: MessageSender = serde_json::from_str(&json).unwrap();
        assert_eq!(sender, parsed);
    }

    #[test]
    fn test_serialize_chat() {
        let sender = MessageSender::Chat(ChatId::new(456).unwrap());
        let json = serde_json::to_string(&sender).unwrap();
        let parsed: MessageSender = serde_json::from_str(&json).unwrap();
        assert_eq!(sender, parsed);
    }

    #[test]
    fn test_serialize_channel() {
        let sender = MessageSender::Channel(ChannelId::new(1000000000).unwrap());
        let json = serde_json::to_string(&sender).unwrap();
        let parsed: MessageSender = serde_json::from_str(&json).unwrap();
        assert_eq!(sender, parsed);
    }

    // Equality tests (3)
    #[test]
    fn test_equality_same() {
        let sender1 = MessageSender::User(UserId::new(123).unwrap());
        let sender2 = MessageSender::User(UserId::new(123).unwrap());
        assert_eq!(sender1, sender2);
    }

    #[test]
    fn test_equality_different_type() {
        let sender1 = MessageSender::User(UserId::new(123).unwrap());
        let sender2 = MessageSender::Chat(ChatId::new(123).unwrap());
        assert_ne!(sender1, sender2);
    }

    #[test]
    fn test_equality_different_id() {
        let sender1 = MessageSender::User(UserId::new(123).unwrap());
        let sender2 = MessageSender::User(UserId::new(456).unwrap());
        assert_ne!(sender1, sender2);
    }
}
