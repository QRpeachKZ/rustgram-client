// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Messages Info
//!
//! Container for message query results from Telegram MTProto.
//!
//! ## Overview
//!
//! This module provides the `MessagesInfo` struct which encapsulates
//! the results of message queries, including the message list, total count,
//! and rate limiting information.
//!
//! ## Types
//!
//! - [`MessagesInfo`] - Container for message query results
//!
//! ## Example
//!
//! ```rust
//! use rustgram_messages_info::MessagesInfo;
//!
//! let info = MessagesInfo::new();
//! assert_eq!(info.total_count(), 0);
//! assert_eq!(info.next_rate(), -1);
//! ```

use rustgram_types::MessageId;
use serde::{Deserialize, Serialize};

/// Container for message query results from Telegram MTProto.
///
/// Encapsulates the results of message history queries, including
/// the list of messages (represented as MessageIds for now),
/// total count for pagination, and rate limiting info.
///
/// # Fields
///
/// - `messages` - Vector of message IDs (placeholder for full Message type)
/// - `total_count` - Total number of messages available server-side
/// - `next_rate` - Rate limit for next query (-1 if no limit)
/// - `is_channel_messages` - Whether messages are from a channel
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessagesInfo {
    /// Vector of message IDs
    /// Note: TDLib uses full Message objects, we use IDs as placeholder
    messages: Vec<MessageId>,
    /// Total count of messages available server-side
    total_count: i32,
    /// Rate limit for next query (-1 means no limit)
    next_rate: i32,
    /// Whether these messages are from a channel
    is_channel_messages: bool,
}

impl MessagesInfo {
    /// Creates a new empty MessagesInfo.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_messages_info::MessagesInfo;
    ///
    /// let info = MessagesInfo::new();
    /// assert_eq!(info.total_count(), 0);
    /// assert!(info.messages().is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            messages: Vec::new(),
            total_count: 0,
            next_rate: -1,
            is_channel_messages: false,
        }
    }

    /// Creates a new MessagesInfo with all fields.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_messages_info::MessagesInfo;
    /// use rustgram_types::MessageId;
    ///
    /// let messages = vec![MessageId::from_server_id(1), MessageId::from_server_id(2)];
    /// let info = MessagesInfo::with_fields(messages, 10, 100, true);
    /// assert_eq!(info.total_count(), 10);
    /// assert_eq!(info.next_rate(), 100);
    /// ```
    #[must_use]
    pub const fn with_fields(
        messages: Vec<MessageId>,
        total_count: i32,
        next_rate: i32,
        is_channel_messages: bool,
    ) -> Self {
        Self {
            messages,
            total_count,
            next_rate,
            is_channel_messages,
        }
    }

    /// Returns the list of message IDs.
    #[must_use]
    pub const fn messages(&self) -> &Vec<MessageId> {
        &self.messages
    }

    /// Returns the total count of messages available server-side.
    #[must_use]
    pub const fn total_count(&self) -> i32 {
        self.total_count
    }

    /// Returns the rate limit for the next query (-1 means no limit).
    #[must_use]
    pub const fn next_rate(&self) -> i32 {
        self.next_rate
    }

    /// Returns whether these messages are from a channel.
    #[must_use]
    pub const fn is_channel_messages(&self) -> bool {
        self.is_channel_messages
    }

    /// Sets the list of messages.
    pub fn set_messages(&mut self, messages: Vec<MessageId>) {
        self.messages = messages;
    }

    /// Sets the total count.
    pub fn set_total_count(&mut self, total_count: i32) {
        self.total_count = total_count;
    }

    /// Sets the next rate.
    pub fn set_next_rate(&mut self, next_rate: i32) {
        self.next_rate = next_rate;
    }

    /// Sets whether these are channel messages.
    pub fn set_is_channel_messages(&mut self, is_channel_messages: bool) {
        self.is_channel_messages = is_channel_messages;
    }

    /// Adds a message ID to the list.
    pub fn add_message(&mut self, message_id: MessageId) {
        self.messages.push(message_id);
    }

    /// Clears all messages.
    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    /// Returns the number of messages in this result.
    #[must_use]
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Returns true if there are no messages.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

impl Default for MessagesInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messages_info_new() {
        let info = MessagesInfo::new();
        assert!(info.messages().is_empty());
        assert_eq!(info.total_count(), 0);
        assert_eq!(info.next_rate(), -1);
        assert!(!info.is_channel_messages());
    }

    #[test]
    fn test_messages_info_with_fields() {
        let messages = vec![MessageId::from_server_id(1), MessageId::from_server_id(2)];
        let info = MessagesInfo::with_fields(messages.clone(), 10, 100, true);
        assert_eq!(info.messages(), &messages);
        assert_eq!(info.total_count(), 10);
        assert_eq!(info.next_rate(), 100);
        assert!(info.is_channel_messages());
    }

    #[test]
    fn test_messages_info_default() {
        let info = MessagesInfo::default();
        assert!(info.messages().is_empty());
        assert_eq!(info.total_count(), 0);
    }

    #[test]
    fn test_setters() {
        let mut info = MessagesInfo::new();
        let messages = vec![MessageId::from_server_id(5)];
        info.set_messages(messages);
        info.set_total_count(20);
        info.set_next_rate(50);
        info.set_is_channel_messages(true);

        assert_eq!(info.messages().len(), 1);
        assert_eq!(info.total_count(), 20);
        assert_eq!(info.next_rate(), 50);
        assert!(info.is_channel_messages());
    }

    #[test]
    fn test_add_message() {
        let mut info = MessagesInfo::new();
        info.add_message(MessageId::from_server_id(1));
        info.add_message(MessageId::from_server_id(2));
        assert_eq!(info.len(), 2);
    }

    #[test]
    fn test_clear_messages() {
        let mut info = MessagesInfo::with_fields(vec![MessageId::from_server_id(1)], 5, 10, false);
        info.clear_messages();
        assert!(info.is_empty());
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut info = MessagesInfo::new();
        assert!(info.is_empty());
        assert_eq!(info.len(), 0);

        info.add_message(MessageId::from_server_id(1));
        assert!(!info.is_empty());
        assert_eq!(info.len(), 1);
    }

    #[test]
    fn test_equality() {
        let info1 = MessagesInfo::with_fields(vec![MessageId::from_server_id(1)], 10, 100, false);
        let info2 = MessagesInfo::with_fields(vec![MessageId::from_server_id(1)], 10, 100, false);
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_clone() {
        let info1 = MessagesInfo::with_fields(vec![MessageId::from_server_id(1)], 10, 100, true);
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_serialize() {
        let info = MessagesInfo::with_fields(vec![MessageId::from_server_id(1)], 10, 100, false);
        let serialized = bincode::serialize(&info).unwrap();
        let deserialized: MessagesInfo = bincode::deserialize(&serialized).unwrap();
        assert_eq!(info, deserialized);
    }
}
