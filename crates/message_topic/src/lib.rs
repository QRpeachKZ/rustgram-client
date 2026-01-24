// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Message Topic
//!
//! Represents a topic classification for messages in Telegram.
//!
//! ## Overview
//!
//! Messages in Telegram can belong to different organizational structures:
//! - **Thread**: Comment threads under messages in megagroups
//! - **Forum**: Forum topics in forum-enabled groups
//! - **Monoforum**: Direct message topics in administered channels
//! - **SavedMessages**: Topics in saved messages
//!
//! ## TDLib Reference
//!
//! - `td/telegram/MessageTopic.h`
//! - `td/telegram/MessageTopic.cpp`
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_topic::MessageTopic;
//! use rustgram_types::{DialogId, MessageId};
//!
//! // Create a thread topic
//! let dialog_id = DialogId::from_channel(123);
//! let msg_id = MessageId::from_server_id(456);
//! let topic = MessageTopic::thread(dialog_id, msg_id);
//! assert!(topic.is_thread());
//! ```

use rustgram_forum_topic_id::ForumTopicId;
use rustgram_saved_messages_manager::SavedMessagesTopicId;
use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Message topic type enumeration.
///
/// Represents the different organizational structures a message can belong to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    /// No topic (default state)
    None,
    /// Thread in a megagroup
    Thread,
    /// Forum topic
    Forum,
    /// Monoforum (administered channel direct messages)
    Monoforum,
    /// Saved messages topic
    SavedMessages,
}

impl Default for MessageType {
    fn default() -> Self {
        Self::None
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "no topic"),
            Self::Thread => write!(f, "thread"),
            Self::Forum => write!(f, "forum"),
            Self::Monoforum => write!(f, "monoforum"),
            Self::SavedMessages => write!(f, "saved messages"),
        }
    }
}

/// Message topic identifier.
///
/// Encapsulates the different ways messages can be organized in Telegram:
/// - Threads (replies in megagroups)
/// - Forum topics
/// - Monoforum direct messages
/// - Saved messages topics
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageTopic {
    /// Topic type
    #[serde(rename = "type")]
    topic_type: MessageType,
    /// Dialog ID containing this topic
    dialog_id: DialogId,
    /// Top thread message ID (for Thread type)
    top_thread_message_id: Option<MessageId>,
    /// Forum topic ID (for Forum type)
    forum_topic_id: Option<ForumTopicId>,
    /// Saved messages topic ID (for Monoforum and SavedMessages types)
    saved_messages_topic_id: Option<SavedMessagesTopicId>,
}

impl Default for MessageTopic {
    fn default() -> Self {
        Self::none()
    }
}

impl fmt::Display for MessageTopic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.topic_type {
            MessageType::None => write!(f, "not a topic"),
            MessageType::Thread => {
                if let Some(msg_id) = self.top_thread_message_id {
                    write!(f, "Thread[{}]", msg_id)
                } else {
                    write!(f, "Thread[?]")
                }
            }
            MessageType::Forum => {
                if let Some(topic_id) = self.forum_topic_id {
                    write!(f, "ForumTopic[{}]", topic_id)
                } else {
                    write!(f, "ForumTopic[?]")
                }
            }
            MessageType::Monoforum => {
                if let Some(topic_id) = self.saved_messages_topic_id {
                    write!(f, "DirectMessagesTopic[{:?}]", topic_id)
                } else {
                    write!(f, "DirectMessagesTopic[?]")
                }
            }
            MessageType::SavedMessages => {
                if let Some(topic_id) = self.saved_messages_topic_id {
                    write!(f, "SavedMessagesTopic[{:?}]", topic_id)
                } else {
                    write!(f, "SavedMessagesTopic[?]")
                }
            }
        }
    }
}

impl MessageTopic {
    /// Creates a message topic with no type (empty topic).
    #[must_use]
    pub fn none() -> Self {
        Self {
            topic_type: MessageType::None,
            dialog_id: DialogId::default(),
            top_thread_message_id: None,
            forum_topic_id: None,
            saved_messages_topic_id: None,
        }
    }

    /// Creates a thread topic.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog containing the thread
    /// * `top_thread_message_id` - The message ID at the top of the thread
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_topic::MessageTopic;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let dialog_id = DialogId::from_channel(123);
    /// let msg_id = MessageId::from_server_id(456);
    /// let topic = MessageTopic::thread(dialog_id, msg_id);
    /// assert!(topic.is_thread());
    /// ```
    #[must_use]
    pub fn thread(dialog_id: DialogId, top_thread_message_id: MessageId) -> Self {
        Self {
            topic_type: MessageType::Thread,
            dialog_id,
            top_thread_message_id: Some(top_thread_message_id),
            forum_topic_id: None,
            saved_messages_topic_id: None,
        }
    }

    /// Creates a forum topic.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog containing the forum
    /// * `forum_topic_id` - The forum topic identifier
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_topic::MessageTopic;
    /// use rustgram_forum_topic_id::ForumTopicId;
    /// use rustgram_types::DialogId;
    ///
    /// let dialog_id = DialogId::from_channel(123);
    /// let topic_id = ForumTopicId::new(456);
    /// let topic = MessageTopic::forum(dialog_id, topic_id);
    /// assert!(topic.is_forum());
    /// ```
    #[must_use]
    pub fn forum(dialog_id: DialogId, forum_topic_id: ForumTopicId) -> Self {
        Self {
            topic_type: MessageType::Forum,
            dialog_id,
            top_thread_message_id: None,
            forum_topic_id: Some(forum_topic_id),
            saved_messages_topic_id: None,
        }
    }

    /// Creates a monoforum topic (direct messages in administered channels).
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The administered channel
    /// * `saved_messages_topic_id` - The saved messages topic identifier
    #[must_use]
    pub fn monoforum(dialog_id: DialogId, saved_messages_topic_id: SavedMessagesTopicId) -> Self {
        Self {
            topic_type: MessageType::Monoforum,
            dialog_id,
            top_thread_message_id: None,
            forum_topic_id: None,
            saved_messages_topic_id: Some(saved_messages_topic_id),
        }
    }

    /// Creates a saved messages topic.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Should be the user's own dialog ID
    /// * `saved_messages_topic_id` - The saved messages topic identifier
    #[must_use]
    pub fn saved_messages(
        dialog_id: DialogId,
        saved_messages_topic_id: SavedMessagesTopicId,
    ) -> Self {
        Self {
            topic_type: MessageType::SavedMessages,
            dialog_id,
            top_thread_message_id: None,
            forum_topic_id: None,
            saved_messages_topic_id: Some(saved_messages_topic_id),
        }
    }

    /// Returns the topic type.
    #[must_use]
    pub const fn topic_type(&self) -> MessageType {
        self.topic_type
    }

    /// Returns the dialog ID.
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns true if this topic is empty (no type).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.topic_type == MessageType::None
    }

    /// Returns true if this is a thread topic.
    #[must_use]
    pub fn is_thread(&self) -> bool {
        self.topic_type == MessageType::Thread
    }

    /// Returns true if this is a forum topic.
    #[must_use]
    pub fn is_forum(&self) -> bool {
        self.topic_type == MessageType::Forum
    }

    /// Returns true if this is a monoforum topic.
    #[must_use]
    pub fn is_monoforum(&self) -> bool {
        self.topic_type == MessageType::Monoforum
    }

    /// Returns true if this is a saved messages topic.
    #[must_use]
    pub fn is_saved_messages(&self) -> bool {
        self.topic_type == MessageType::SavedMessages
    }

    /// Returns true if this is the general forum topic.
    #[must_use]
    pub fn is_general_forum(&self) -> bool {
        if self.topic_type != MessageType::Forum {
            return false;
        }
        self.forum_topic_id.map_or(false, |id| id.is_general())
    }

    /// Returns the top thread message ID.
    ///
    /// # Panics
    ///
    /// Panics if this is not a thread topic.
    #[must_use]
    pub fn get_top_thread_message_id(&self) -> MessageId {
        if self.topic_type != MessageType::Thread {
            panic!("MessageTopic is not a thread");
        }
        self.top_thread_message_id.unwrap_or(MessageId::default())
    }

    /// Returns the forum topic ID.
    ///
    /// # Panics
    ///
    /// Panics if this is not a forum topic.
    #[must_use]
    pub fn get_forum_topic_id(&self) -> ForumTopicId {
        if self.topic_type != MessageType::Forum {
            panic!("MessageTopic is not a forum");
        }
        self.forum_topic_id.unwrap_or(ForumTopicId::EMPTY)
    }

    /// Returns the monoforum saved messages topic ID.
    ///
    /// # Panics
    ///
    /// Panics if this is not a monoforum topic.
    #[must_use]
    pub fn get_monoforum_saved_messages_topic_id(&self) -> SavedMessagesTopicId {
        if self.topic_type != MessageType::Monoforum {
            panic!("MessageTopic is not a monoforum");
        }
        self.saved_messages_topic_id.unwrap()
    }

    /// Returns the implicit reply-to message ID for this topic.
    ///
    /// For threads, this returns the top thread message ID.
    /// For forums (except general), this returns the topic's top message ID.
    /// For other types, returns empty.
    #[must_use]
    pub fn get_implicit_reply_to_message_id(&self) -> MessageId {
        match self.topic_type {
            MessageType::Thread => self.top_thread_message_id.unwrap_or(MessageId::default()),
            MessageType::Forum => {
                // For general forum topic, no implicit reply
                if self.is_general_forum() {
                    MessageId::default()
                } else {
                    self.forum_topic_id
                        .map(|id| id.to_top_thread_message_id())
                        .unwrap_or(MessageId::default())
                }
            }
            MessageType::Monoforum | MessageType::SavedMessages | MessageType::None => {
                MessageId::default()
            }
        }
    }

    /// Returns the input top message ID for MTProto API calls.
    ///
    /// For threads, returns the server message ID.
    /// For forums, returns the forum topic ID value.
    /// For other types, returns 0.
    #[must_use]
    pub fn get_input_top_msg_id(&self) -> i32 {
        match self.topic_type {
            MessageType::Thread => {
                if let Some(msg_id) = self.top_thread_message_id {
                    msg_id.get_server_id() as i32
                } else {
                    0
                }
            }
            MessageType::Forum => {
                if let Some(topic_id) = self.forum_topic_id {
                    topic_id.get()
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::ChannelId;

    #[test]
    fn test_default() {
        let topic = MessageTopic::default();
        assert!(topic.is_empty());
        assert!(!topic.is_thread());
        assert!(!topic.is_forum());
    }

    #[test]
    fn test_none() {
        let topic = MessageTopic::none();
        assert!(topic.is_empty());
        assert_eq!(topic.topic_type(), MessageType::None);
    }

    #[test]
    fn test_thread() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let msg_id = MessageId::from_server_id(456);
        let topic = MessageTopic::thread(dialog_id, msg_id);

        assert!(topic.is_thread());
        assert!(!topic.is_empty());
        assert_eq!(topic.get_top_thread_message_id(), msg_id);
    }

    #[test]
    fn test_forum() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let forum_id = ForumTopicId::new(789);
        let topic = MessageTopic::forum(dialog_id, forum_id);

        assert!(topic.is_forum());
        assert!(!topic.is_empty());
        assert_eq!(topic.get_forum_topic_id(), forum_id);
    }

    #[test]
    fn test_general_forum() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let general = ForumTopicId::general();
        let topic = MessageTopic::forum(dialog_id, general);

        assert!(topic.is_general_forum());
    }

    #[test]
    fn test_non_general_forum() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let forum_id = ForumTopicId::new(12345);
        let topic = MessageTopic::forum(dialog_id, forum_id);

        assert!(!topic.is_general_forum());
        assert!(topic.is_forum());
    }

    #[test]
    fn test_get_implicit_reply_to_message_id_thread() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let msg_id = MessageId::from_server_id(456);
        let topic = MessageTopic::thread(dialog_id, msg_id);

        assert_eq!(topic.get_implicit_reply_to_message_id(), msg_id);
    }

    #[test]
    fn test_get_implicit_reply_to_message_id_forum() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let forum_id = ForumTopicId::new(789);
        let topic = MessageTopic::forum(dialog_id, forum_id);

        let reply_id = topic.get_implicit_reply_to_message_id();
        assert_eq!(reply_id.get_server_id(), 789);
    }

    #[test]
    fn test_get_implicit_reply_to_message_id_general_forum() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let general = ForumTopicId::general();
        let topic = MessageTopic::forum(dialog_id, general);

        assert!(topic.get_implicit_reply_to_message_id().get() == 0);
    }

    #[test]
    fn test_get_implicit_reply_to_message_id_none() {
        let topic = MessageTopic::none();
        assert!(topic.get_implicit_reply_to_message_id().get() == 0);
    }

    #[test]
    fn test_get_input_top_msg_id_thread() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let msg_id = MessageId::from_server_id(456);
        let topic = MessageTopic::thread(dialog_id, msg_id);

        assert_eq!(topic.get_input_top_msg_id(), 456);
    }

    #[test]
    fn test_get_input_top_msg_id_forum() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let forum_id = ForumTopicId::new(789);
        let topic = MessageTopic::forum(dialog_id, forum_id);

        assert_eq!(topic.get_input_top_msg_id(), 789);
    }

    #[test]
    fn test_get_input_top_msg_id_none() {
        let topic = MessageTopic::none();
        assert_eq!(topic.get_input_top_msg_id(), 0);
    }

    #[test]
    fn test_display_none() {
        let topic = MessageTopic::none();
        let s = format!("{}", topic);
        assert!(s.contains("not a topic"));
    }

    #[test]
    fn test_display_thread() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let msg_id = MessageId::from_server_id(456);
        let topic = MessageTopic::thread(dialog_id, msg_id);
        let s = format!("{}", topic);
        assert!(s.contains("Thread"));
    }

    #[test]
    fn test_display_forum() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let forum_id = ForumTopicId::new(789);
        let topic = MessageTopic::forum(dialog_id, forum_id);
        let s = format!("{}", topic);
        assert!(s.contains("ForumTopic"));
    }

    #[test]
    fn test_equality() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let msg_id = MessageId::from_server_id(456);

        let topic1 = MessageTopic::thread(dialog_id, msg_id);
        let topic2 = MessageTopic::thread(dialog_id, msg_id);
        assert_eq!(topic1, topic2);
    }

    #[test]
    fn test_serialize() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let msg_id = MessageId::from_server_id(456);
        let topic = MessageTopic::thread(dialog_id, msg_id);

        let serialized = serde_json::to_string(&topic).unwrap();
        assert!(serialized.contains("thread"));

        let deserialized: MessageTopic = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.topic_type, MessageType::Thread);
    }
}
