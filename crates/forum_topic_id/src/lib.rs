// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Forum Topic ID
//!
//! Forum topic identifier for Telegram forum discussions.
//!
//! ## Overview
//!
//! Forum topics allow organizing group discussions into separate threads.
//! Each topic has a unique identifier derived from the top message ID.
//!
//! ## TDLib Reference
//!
//! - `td/telegram/ForumTopicId.h`
//!
//! ## Example
//!
//! ```rust
//! use rustgram_forum_topic_id::ForumTopicId;
//! use rustgram_types::MessageId;
//!
//! // Create from message ID
//! let msg_id = MessageId::from_server_id(12345);
//! let topic_id = ForumTopicId::from_top_thread_message_id(msg_id);
//! assert_eq!(topic_id.get(), 12345);
//!
//! // Create general topic
//! let general = ForumTopicId::general();
//! assert!(general.is_general());
//! ```

use rustgram_types::MessageId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Forum topic identifier.
///
/// Forum topic IDs are 32-bit signed integers. The value represents
/// the message ID of the first message in the topic thread.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ForumTopicId(i32);

impl Default for ForumTopicId {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl fmt::Display for ForumTopicId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "topic_{}", self.0)
    }
}

impl ForumTopicId {
    /// Empty/invalid forum topic ID (zero value).
    pub const EMPTY: Self = Self(0);

    /// General forum topic ID (value of 1).
    ///
    /// The general topic is a special topic that exists in all forums
    /// and contains messages not belonging to any specific topic.
    pub const GENERAL: Self = Self(1);

    /// Creates a new forum topic ID from an i32 value.
    #[must_use]
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    /// Returns the underlying i32 value.
    #[must_use]
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Creates a forum topic ID from a top thread message ID.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The message ID at the top of the thread
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_forum_topic_id::ForumTopicId;
    /// use rustgram_types::MessageId;
    ///
    /// let msg_id = MessageId::from_server_id(12345);
    /// let topic_id = ForumTopicId::from_top_thread_message_id(msg_id);
    /// assert_eq!(topic_id.get(), 12345);
    /// ```
    #[must_use]
    pub fn from_top_thread_message_id(message_id: MessageId) -> Self {
        Self(message_id.get_server_id() as i32)
    }

    /// Returns true if this is the general forum topic.
    #[must_use]
    pub const fn is_general(self) -> bool {
        self.0 == Self::GENERAL.0
    }

    /// Returns true if this topic ID is valid (non-zero).
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 > 0
    }

    /// Converts this topic ID to the top thread message ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_forum_topic_id::ForumTopicId;
    ///
    /// let topic_id = ForumTopicId::new(12345);
    /// let msg_id = topic_id.to_top_thread_message_id();
    /// assert_eq!(msg_id.get_server_id(), 12345);
    /// ```
    #[must_use]
    pub fn to_top_thread_message_id(self) -> MessageId {
        MessageId::from_server_id(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let topic_id = ForumTopicId::default();
        assert_eq!(topic_id, ForumTopicId::EMPTY);
        assert!(!topic_id.is_valid());
    }

    #[test]
    fn test_new() {
        let topic_id = ForumTopicId::new(12345);
        assert_eq!(topic_id.get(), 12345);
        assert!(topic_id.is_valid());
    }

    #[test]
    fn test_empty() {
        let topic_id = ForumTopicId::EMPTY;
        assert_eq!(topic_id.get(), 0);
        assert!(!topic_id.is_valid());
    }

    #[test]
    fn test_general() {
        let general = ForumTopicId::general();
        assert_eq!(general.get(), 1);
        assert!(general.is_general());
        assert!(general.is_valid());
    }

    #[test]
    fn test_is_general() {
        let general = ForumTopicId::general();
        assert!(general.is_general());

        let other = ForumTopicId::new(12345);
        assert!(!other.is_general());
    }

    #[test]
    fn test_is_valid() {
        assert!(!ForumTopicId::default().is_valid());
        assert!(ForumTopicId::general().is_valid());
        assert!(ForumTopicId::new(12345).is_valid());
        assert!(!ForumTopicId::new(-1).is_valid());
    }

    #[test]
    fn test_from_top_thread_message_id() {
        let msg_id = MessageId::from_server_id(12345);
        let topic_id = ForumTopicId::from_top_thread_message_id(msg_id);
        assert_eq!(topic_id.get(), 12345);
    }

    #[test]
    fn test_to_top_thread_message_id() {
        let topic_id = ForumTopicId::new(12345);
        let msg_id = topic_id.to_top_thread_message_id();
        assert_eq!(msg_id.get_server_id(), 12345);
    }

    #[test]
    fn test_roundtrip_message_id() {
        let msg_id = MessageId::from_server_id(99999);
        let topic_id = ForumTopicId::from_top_thread_message_id(msg_id);
        let back = topic_id.to_top_thread_message_id();
        assert_eq!(msg_id, back);
    }

    #[test]
    fn test_equality() {
        let id1 = ForumTopicId::new(12345);
        let id2 = ForumTopicId::new(12345);
        let id3 = ForumTopicId::new(67890);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_ordering() {
        let id1 = ForumTopicId::new(100);
        let id2 = ForumTopicId::new(200);
        let id3 = ForumTopicId::new(150);

        assert!(id1 < id2);
        assert!(id2 > id1);
        assert!(id1 < id3);
        assert!(id3 < id2);
    }

    #[test]
    fn test_display() {
        let topic_id = ForumTopicId::new(12345);
        let s = format!("{}", topic_id);
        assert!(s.contains("12345"));
        assert!(s.contains("topic"));
    }

    #[test]
    fn test_serialize() {
        let topic_id = ForumTopicId::new(12345);
        let serialized = serde_json::to_string(&topic_id).unwrap();
        assert_eq!(serialized, "12345");

        let deserialized: ForumTopicId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, topic_id);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;

        let id1 = ForumTopicId::new(100);
        let id2 = ForumTopicId::new(200);
        let id3 = ForumTopicId::new(100);

        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);
        set.insert(id3);

        assert_eq!(set.len(), 2);
        assert!(set.contains(&id1));
        assert!(set.contains(&id2));
    }

    // Property-based tests
    #[cfg(feature = "proptest")]
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn prop_roundtrip(id in 1i32..1000i32) {
                let topic_id = ForumTopicId::new(id);
                let msg_id = topic_id.to_top_thread_message_id();
                let back = ForumTopicId::from_top_thread_message_id(msg_id);
                assert_eq!(topic_id, back);
            }

            #[test]
            fn prop_valid_positive(id in 1i32..1000i32) {
                let topic_id = ForumTopicId::new(id);
                assert!(topic_id.is_valid());
            }

            #[test]
            fn prop_serialize_roundtrip(id in 1i32..1000i32) {
                let topic_id = ForumTopicId::new(id);
                let serialized = serde_json::to_string(&topic_id).unwrap();
                let deserialized: ForumTopicId = serde_json::from_str(&serialized).unwrap();
                assert_eq!(topic_id, deserialized);
            }
        }
    }
}
