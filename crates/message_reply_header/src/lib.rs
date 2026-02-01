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

//! # Message Reply Header
//!
//! Information about a message that is being replied to.
//!
//! ## TDLib Alignment
//!
//! This type aligns with TDLib's `MessageReplyHeader` struct.
//! - TDLib header: `td/telegram/MessageReplyHeader.h`
//! - TDLib type: Struct with RepliedMessageInfo, top_thread_message_id, StoryFullId
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_reply_header::MessageReplyHeader;
//! use rustgram_types::MessageId;
//!
//! let header = MessageReplyHeader::with_thread_message(
//!     MessageId::from_server_id(123),
//!     false
//! );
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stub for RepliedMessageInfo.
///
/// TODO: Full implementation when message-reply module is available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepliedMessageInfo {
    /// Dialog that sent the original message
    pub dialog_id: DialogId,
    /// Message ID of the original message
    pub message_id: MessageId,
    /// Date when the original message was sent
    pub date: i32,
}

impl RepliedMessageInfo {
    /// Creates a new RepliedMessageInfo.
    pub fn new(dialog_id: DialogId, message_id: MessageId, date: i32) -> Self {
        Self {
            dialog_id,
            message_id,
            date,
        }
    }
}

/// Stub for StoryFullId.
///
/// TODO: Full implementation when story module is available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryFullId {
    /// Dialog that owns the story
    pub dialog_id: DialogId,
    /// Story ID
    pub story_id: i32,
}

impl StoryFullId {
    /// Creates a new StoryFullId.
    pub fn new(dialog_id: DialogId, story_id: i32) -> Self {
        Self {
            dialog_id,
            story_id,
        }
    }

    /// Returns `true` if this is a valid story ID.
    pub fn is_valid(&self) -> bool {
        self.story_id > 0 && self.dialog_id.is_valid()
    }
}

/// Information about a message that is being replied to.
///
/// This can be either:
/// - A reply to a message (with RepliedMessageInfo)
/// - A reply to a thread (with top_thread_message_id)
/// - A reply to a story (with StoryFullId)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageReplyHeader {
    /// Reply to a message
    Message(RepliedMessageInfo),

    /// Reply to a forum thread
    Thread {
        /// Top thread message ID
        top_thread_message_id: MessageId,
        /// Whether this is a topic message
        is_topic_message: bool,
    },

    /// Reply to a story
    Story(StoryFullId),
}

impl MessageReplyHeader {
    /// Creates a reply header for a message reply.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_reply_header::MessageReplyHeader;
    /// use rustgram_types::{DialogId, MessageId, UserId};
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let message_id = MessageId::from_server_id(456);
    /// let reply_info = RepliedMessageInfo::new(dialog_id, message_id, 12345);
    ///
    /// let header = MessageReplyHeader::Message(reply_info);
    /// ```
    #[must_use]
    pub fn message(reply_info: RepliedMessageInfo) -> Self {
        Self::Message(reply_info)
    }

    /// Creates a reply header for a thread reply.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_reply_header::MessageReplyHeader;
    /// use rustgram_types::MessageId;
    ///
    /// let header = MessageReplyHeader::with_thread_message(
    ///     MessageId::from_server_id(123),
    ///     false
    /// );
    /// ```
    #[must_use]
    pub fn with_thread_message(top_thread_message_id: MessageId, is_topic_message: bool) -> Self {
        Self::Thread {
            top_thread_message_id,
            is_topic_message,
        }
    }

    /// Creates a reply header for a story reply.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_reply_header::MessageReplyHeader;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let story_id = StoryFullId::new(dialog_id, 456);
    ///
    /// let header = MessageReplyHeader::Story(story_id);
    /// ```
    #[must_use]
    pub fn story(story_full_id: StoryFullId) -> Self {
        Self::Story(story_full_id)
    }

    /// Returns `true` if this is a message reply.
    #[must_use]
    pub const fn is_message(&self) -> bool {
        matches!(self, Self::Message(_))
    }

    /// Returns `true` if this is a thread reply.
    #[must_use]
    pub const fn is_thread(&self) -> bool {
        matches!(self, Self::Thread { .. })
    }

    /// Returns `true` if this is a story reply.
    #[must_use]
    pub const fn is_story(&self) -> bool {
        matches!(self, Self::Story(_))
    }

    /// Returns the top thread message ID if this is a thread reply.
    #[must_use]
    pub const fn top_thread_message_id(&self) -> Option<MessageId> {
        match self {
            Self::Thread {
                top_thread_message_id,
                ..
            } => Some(*top_thread_message_id),
            _ => None,
        }
    }

    /// Returns `true` if this is a topic message.
    #[must_use]
    pub const fn is_topic_message(&self) -> Option<bool> {
        match self {
            Self::Thread {
                is_topic_message, ..
            } => Some(*is_topic_message),
            _ => None,
        }
    }

    /// Returns the StoryFullId if this is a story reply.
    #[must_use]
    pub fn story_full_id(&self) -> Option<StoryFullId> {
        match self {
            Self::Story(story_full_id) => Some(story_full_id.clone()),
            _ => None,
        }
    }

    /// Returns the RepliedMessageInfo if this is a message reply.
    #[must_use]
    pub fn replied_message_info(&self) -> Option<RepliedMessageInfo> {
        match self {
            Self::Message(info) => Some(info.clone()),
            _ => None,
        }
    }
}

impl Default for MessageReplyHeader {
    fn default() -> Self {
        Self::Message(RepliedMessageInfo {
            dialog_id: DialogId::default(),
            message_id: MessageId::default(),
            date: 0,
        })
    }
}

impl fmt::Display for MessageReplyHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message(info) => write!(f, "reply to {}", info.message_id.get()),
            Self::Thread {
                top_thread_message_id,
                ..
            } => write!(f, "thread reply to {}", top_thread_message_id.get()),
            Self::Story(story) => write!(f, "story reply {}", story.story_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    // Constructor tests (3)
    #[test]
    fn test_message() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(456);
        let reply_info = RepliedMessageInfo::new(dialog_id, message_id, 12345);
        let header = MessageReplyHeader::message(reply_info);

        assert!(header.is_message());
        assert!(!header.is_thread());
        assert!(!header.is_story());
    }

    #[test]
    fn test_thread() {
        let header = MessageReplyHeader::with_thread_message(MessageId::from_server_id(123), false);

        assert!(!header.is_message());
        assert!(header.is_thread());
        assert!(!header.is_story());
    }

    #[test]
    fn test_story() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let story_id = StoryFullId::new(dialog_id, 456);
        let header = MessageReplyHeader::story(story_id);

        assert!(!header.is_message());
        assert!(!header.is_thread());
        assert!(header.is_story());
    }

    // Property tests (6)
    #[test]
    fn test_top_thread_message_id() {
        let header = MessageReplyHeader::with_thread_message(MessageId::from_server_id(123), true);
        assert_eq!(
            header.top_thread_message_id(),
            Some(MessageId::from_server_id(123))
        );
    }

    #[test]
    fn test_top_thread_message_id_none() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(456);
        let reply_info = RepliedMessageInfo::new(dialog_id, message_id, 12345);
        let header = MessageReplyHeader::message(reply_info);

        assert!(header.top_thread_message_id().is_none());
    }

    #[test]
    fn test_is_topic_message() {
        let header = MessageReplyHeader::with_thread_message(MessageId::from_server_id(123), true);
        assert_eq!(header.is_topic_message(), Some(true));
    }

    #[test]
    fn test_is_not_topic_message() {
        let header = MessageReplyHeader::with_thread_message(MessageId::from_server_id(123), false);
        assert_eq!(header.is_topic_message(), Some(false));
    }

    #[test]
    fn test_is_topic_message_none() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let story_id = StoryFullId::new(dialog_id, 456);
        let header = MessageReplyHeader::story(story_id);

        assert!(header.is_topic_message().is_none());
    }

    #[test]
    fn test_story_full_id() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let story_id = StoryFullId::new(dialog_id, 456);
        let header = MessageReplyHeader::story(story_id.clone());

        assert_eq!(header.story_full_id(), Some(story_id));
    }

    // Display tests (3)
    #[test]
    fn test_display_message() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(456);
        let reply_info = RepliedMessageInfo::new(dialog_id, message_id, 12345);
        let header = MessageReplyHeader::message(reply_info);

        let display = format!("{}", header);
        assert!(display.contains("reply to"));
    }

    #[test]
    fn test_display_thread() {
        let header = MessageReplyHeader::with_thread_message(MessageId::from_server_id(123), false);
        let display = format!("{}", header);
        assert!(display.contains("thread reply"));
    }

    #[test]
    fn test_display_story() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let story_id = StoryFullId::new(dialog_id, 456);
        let header = MessageReplyHeader::story(story_id);

        let display = format!("{}", header);
        assert!(display.contains("story reply"));
    }

    // Clone tests (2)
    #[test]
    fn test_clone() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let story_id = StoryFullId::new(dialog_id, 456);
        let header1 = MessageReplyHeader::story(story_id);
        let header2 = header1.clone();
        assert_eq!(header1, header2);
    }

    #[test]
    fn test_default() {
        let header = MessageReplyHeader::default();
        assert!(header.is_message());
    }

    // Serialization tests (3)
    #[test]
    fn test_serialize_message() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let message_id = MessageId::from_server_id(456);
        let reply_info = RepliedMessageInfo::new(dialog_id, message_id, 12345);
        let header = MessageReplyHeader::message(reply_info);

        let json = serde_json::to_string(&header).unwrap();
        let parsed: MessageReplyHeader = serde_json::from_str(&json).unwrap();
        assert_eq!(header, parsed);
    }

    #[test]
    fn test_serialize_thread() {
        let header = MessageReplyHeader::with_thread_message(MessageId::from_server_id(123), true);

        let json = serde_json::to_string(&header).unwrap();
        let parsed: MessageReplyHeader = serde_json::from_str(&json).unwrap();
        assert_eq!(header, parsed);
    }

    #[test]
    fn test_serialize_story() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let story_id = StoryFullId::new(dialog_id, 456);
        let header = MessageReplyHeader::story(story_id);

        let json = serde_json::to_string(&header).unwrap();
        let parsed: MessageReplyHeader = serde_json::from_str(&json).unwrap();
        assert_eq!(header, parsed);
    }
}
