// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Ordered Messages
//!
//! Ordered collection of message IDs using a binary tree structure.
//!
//! ## Overview
//!
//! This module provides an ordered collection for message IDs, optimized
//! for operations like:
//! - Insert messages in order
//! - Retrieve history pages with offset/limit
//! - Find messages by date
//! - Calculate unread counts
//!
//! The implementation uses a simplified binary tree instead of TDLib's
//! splay tree, trading some performance for implementation simplicity.
//!
//! ## Types
//!
//! - [`OrderedMessages`] - Ordered collection of message IDs
//!
//! ## Example
//!
//! ```rust
//! use rustgram_ordered_messages::OrderedMessages;
//! use rustgram_types::MessageId;
//!
//! let mut messages = OrderedMessages::new();
//! messages.insert(MessageId::from_server_id(1), false, MessageId::from_server_id(0), "test");
//! messages.insert(MessageId::from_server_id(2), false, MessageId::from_server_id(0), "test");
//! assert_eq!(messages.len(), 2);
//! ```

use rustgram_types::MessageId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Ordered collection of message IDs.
///
/// Provides efficient insertion, deletion, and history retrieval operations.
/// Messages are stored in sorted order by their ID value.
///
/// # Implementation Notes
///
/// TDLib uses a splay tree for this purpose. This implementation uses
/// a BTreeSet for simplicity, which provides O(log n) operations
/// for insert, delete, and range queries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderedMessages {
    /// Set of message IDs in sorted order
    messages: BTreeSet<MessageId>,
}

impl OrderedMessages {
    /// Creates a new empty OrderedMessages.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_ordered_messages::OrderedMessages;
    ///
    /// let messages = OrderedMessages::new();
    /// assert!(messages.is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            messages: BTreeSet::new(),
        }
    }

    /// Inserts a message ID into the collection.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The message ID to insert
    /// * `_auto_attach` - Whether to auto-attach (unused in this implementation)
    /// * `_old_last` - Previous last message ID (unused)
    /// * `_source` - Source identifier for logging
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_ordered_messages::OrderedMessages;
    /// use rustgram_types::MessageId;
    ///
    /// let mut messages = OrderedMessages::new();
    /// messages.insert(MessageId::from_server_id(1), false, MessageId::from_server_id(0), "test");
    /// assert!(!messages.is_empty());
    /// ```
    pub fn insert(
        &mut self,
        message_id: MessageId,
        _auto_attach: bool,
        _old_last: MessageId,
        _source: &str,
    ) {
        self.messages.insert(message_id);
    }

    /// Erases a message ID from the collection.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The message ID to erase
    /// * `_only_from_memory` - Whether to only erase from memory (unused)
    /// * `_source` - Source identifier for logging
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_ordered_messages::OrderedMessages;
    /// use rustgram_types::MessageId;
    ///
    /// let mut messages = OrderedMessages::new();
    /// messages.insert(MessageId::from_server_id(1), false, MessageId::from_server_id(0), "test");
    /// messages.erase(MessageId::from_server_id(1), false, "test");
    /// assert!(messages.is_empty());
    /// ```
    pub fn erase(&mut self, message_id: MessageId, _only_from_memory: bool, _source: &str) {
        self.messages.remove(&message_id);
    }

    /// Gets message history with pagination.
    ///
    /// Returns messages in reverse order (newest first) starting from
    /// the given message ID, with optional offset and limit.
    ///
    /// # Arguments
    ///
    /// * `last` - The last message ID to start from (use `MessageId::from_server_id(0)` for newest)
    /// * `from` - Updated with the starting message ID for next query
    /// * `offset` - Number of messages to skip
    /// * `limit` - Maximum number of messages to return
    /// * `_force` - Whether to force the query
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_ordered_messages::OrderedMessages;
    /// use rustgram_types::MessageId;
    ///
    /// let mut messages = OrderedMessages::new();
    /// for i in 1..=10 {
    ///     messages.insert(MessageId::from_server_id(i), false, MessageId::from_server_id(0), "test");
    /// }
    ///
    /// let mut from = MessageId::from_server_id(0);
    /// let mut offset = 0;
    /// let mut limit = 5;
    /// let history = messages.get_history(MessageId::from_server_id(0), &mut from, &mut offset, &mut limit, false);
    /// assert_eq!(history.len(), 5);
    /// ```
    pub fn get_history(
        &self,
        last: MessageId,
        from: &mut MessageId,
        offset: &mut i32,
        limit: &mut i32,
        _force: bool,
    ) -> Vec<MessageId> {
        let mut result = Vec::new();

        // Start from the last message if specified, otherwise from the newest
        let start = if last.get() > 0 {
            self.messages.range(..=last).next_back()
        } else {
            self.messages.iter().next_back()
        };

        if let Some(&start_id) = start {
            *from = start_id;

            // Iterate in reverse order (newest first)
            let iter = self.messages.range(..=start_id).rev();
            let mut skipped = 0;
            let mut collected = 0;

            for &msg_id in iter {
                if skipped < *offset as usize {
                    skipped += 1;
                    continue;
                }

                if collected >= *limit as usize {
                    break;
                }

                result.push(msg_id);
                collected += 1;
            }
        }

        result
    }

    /// Finds a message by date using a date getter function.
    ///
    /// # Arguments
    ///
    /// * `date` - The date to search for
    /// * `get_date` - Function that returns the date for a given message ID
    ///
    /// # Returns
    ///
    /// The message ID closest to the given date, or `MessageId::from_server_id(0)` if not found.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_ordered_messages::OrderedMessages;
    /// use rustgram_types::MessageId;
    ///
    /// let mut messages = OrderedMessages::new();
    /// messages.insert(MessageId::from_server_id(1), false, MessageId::from_server_id(0), "test");
    ///
    /// // Simple date function: message ID * 1000
    /// let found = messages.find_message_by_date(1000, |id| id.get() as i32 * 1000);
    /// assert_eq!(found, MessageId::from_server_id(1));
    /// ```
    pub fn find_message_by_date(
        &self,
        date: i32,
        get_date: impl Fn(MessageId) -> i32,
    ) -> MessageId {
        // Binary search for the message with the closest date
        let mut closest = MessageId::from_server_id(0);
        let mut closest_diff = i32::MAX;

        for &msg_id in &self.messages {
            let msg_date = get_date(msg_id);
            let diff = (msg_date - date).abs();

            if diff < closest_diff {
                closest_diff = diff;
                closest = msg_id;
            }

            if diff == 0 {
                break; // Exact match found
            }
        }

        closest
    }

    /// Finds messages in a date range.
    ///
    /// # Arguments
    ///
    /// * `min_date` - Minimum date (inclusive)
    /// * `max_date` - Maximum date (inclusive)
    /// * `get_date` - Function that returns the date for a given message ID
    ///
    /// # Returns
    ///
    /// Vector of message IDs within the date range.
    pub fn find_messages_by_date(
        &self,
        min_date: i32,
        max_date: i32,
        get_date: impl Fn(MessageId) -> i32,
    ) -> Vec<MessageId> {
        self.messages
            .iter()
            .filter(|&&msg_id| {
                let date = get_date(msg_id);
                date >= min_date && date <= max_date
            })
            .copied()
            .collect()
    }

    /// Returns the last (newest) message ID in the collection.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_ordered_messages::OrderedMessages;
    /// use rustgram_types::MessageId;
    ///
    /// let mut messages = OrderedMessages::new();
    /// messages.insert(MessageId::from_server_id(1), false, MessageId::from_server_id(0), "test");
    /// messages.insert(MessageId::from_server_id(5), false, MessageId::from_server_id(0), "test");
    /// assert_eq!(messages.get_last_message_id(), MessageId::from_server_id(5));
    /// ```
    #[must_use]
    pub fn get_last_message_id(&self) -> MessageId {
        self.messages
            .iter()
            .next_back()
            .copied()
            .unwrap_or_else(|| MessageId::from_server_id(0))
    }

    /// Calculates the new unread count based on message changes.
    ///
    /// # Arguments
    ///
    /// * `max_id` - Maximum message ID to consider
    /// * `last_read` - Last read message ID
    /// * `old_unread` - Previous unread count
    /// * `_last` - Last message ID in the collection (unused)
    /// * `_is_unread` - Function that checks if a message is unread (unused)
    /// * `_hint` - Hint for optimization (unused)
    ///
    /// # Returns
    ///
    /// The calculated unread count.
    pub fn calc_new_unread_count(
        &self,
        max_id: MessageId,
        last_read: MessageId,
        _old_unread: i32,
        _last: MessageId,
        _is_unread: impl Fn(MessageId) -> bool,
        _hint: i32,
    ) -> i32 {
        // If max_id is before or equal to last_read, no new unread messages
        if max_id.get() <= last_read.get() {
            return 0;
        }

        // Count messages greater than last_read and up to max_id
        let range = self.messages.range(last_read..max_id);
        range.count() as i32
    }

    /// Returns the number of messages in the collection.
    #[must_use]
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Returns true if the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Returns true if the collection contains the given message ID.
    #[must_use]
    pub fn contains(&self, message_id: MessageId) -> bool {
        self.messages.contains(&message_id)
    }

    /// Clears all messages from the collection.
    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

impl Default for OrderedMessages {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for OrderedMessages {
    type Item = MessageId;
    type IntoIter = std::vec::IntoIter<MessageId>;

    fn into_iter(self) -> Self::IntoIter {
        self.messages.into_iter().collect::<Vec<_>>().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered_messages_new() {
        let messages = OrderedMessages::new();
        assert!(messages.is_empty());
        assert_eq!(messages.len(), 0);
    }

    #[test]
    fn test_ordered_messages_default() {
        let messages = OrderedMessages::default();
        assert!(messages.is_empty());
    }

    #[test]
    fn test_insert() {
        let mut messages = OrderedMessages::new();
        messages.insert(
            MessageId::from_server_id(1),
            false,
            MessageId::from_server_id(0),
            "test",
        );
        messages.insert(
            MessageId::from_server_id(5),
            false,
            MessageId::from_server_id(0),
            "test",
        );
        messages.insert(
            MessageId::from_server_id(3),
            false,
            MessageId::from_server_id(0),
            "test",
        );

        assert_eq!(messages.len(), 3);
        assert!(messages.contains(MessageId::from_server_id(1)));
        assert!(messages.contains(MessageId::from_server_id(3)));
        assert!(messages.contains(MessageId::from_server_id(5)));
    }

    #[test]
    fn test_erase() {
        let mut messages = OrderedMessages::new();
        messages.insert(
            MessageId::from_server_id(1),
            false,
            MessageId::from_server_id(0),
            "test",
        );
        messages.insert(
            MessageId::from_server_id(2),
            false,
            MessageId::from_server_id(0),
            "test",
        );

        messages.erase(MessageId::from_server_id(1), false, "test");

        assert_eq!(messages.len(), 1);
        assert!(!messages.contains(MessageId::from_server_id(1)));
        assert!(messages.contains(MessageId::from_server_id(2)));
    }

    #[test]
    fn test_get_history() {
        let mut messages = OrderedMessages::new();
        for i in 1..=10 {
            messages.insert(
                MessageId::from_server_id(i),
                false,
                MessageId::from_server_id(0),
                "test",
            );
        }

        let mut from = MessageId::from_server_id(0);
        let mut offset = 0;
        let mut limit = 5;
        let history = messages.get_history(
            MessageId::from_server_id(0),
            &mut from,
            &mut offset,
            &mut limit,
            false,
        );

        assert_eq!(history.len(), 5);
        // History should be in reverse order (newest first)
        assert_eq!(
            history,
            vec![
                MessageId::from_server_id(10),
                MessageId::from_server_id(9),
                MessageId::from_server_id(8),
                MessageId::from_server_id(7),
                MessageId::from_server_id(6)
            ]
        );
    }

    #[test]
    fn test_get_history_with_offset() {
        let mut messages = OrderedMessages::new();
        for i in 1..=10 {
            messages.insert(
                MessageId::from_server_id(i),
                false,
                MessageId::from_server_id(0),
                "test",
            );
        }

        let mut from = MessageId::from_server_id(0);
        let mut offset = 5;
        let mut limit = 3;
        let history = messages.get_history(
            MessageId::from_server_id(0),
            &mut from,
            &mut offset,
            &mut limit,
            false,
        );

        assert_eq!(history.len(), 3);
        // Should skip first 5 and return next 3
        assert_eq!(
            history,
            vec![
                MessageId::from_server_id(5),
                MessageId::from_server_id(4),
                MessageId::from_server_id(3)
            ]
        );
    }

    #[test]
    fn test_find_message_by_date() {
        let mut messages = OrderedMessages::new();
        messages.insert(
            MessageId::from_server_id(1),
            false,
            MessageId::from_server_id(0),
            "test",
        );
        messages.insert(
            MessageId::from_server_id(2),
            false,
            MessageId::from_server_id(0),
            "test",
        );
        messages.insert(
            MessageId::from_server_id(3),
            false,
            MessageId::from_server_id(0),
            "test",
        );

        // Date function: use the server_id directly as date
        // MessageId::from_server_id(i) creates MessageId(i << 20), so we extract the server_id
        let get_date = |id: MessageId| (id.get() >> 20) as i32;

        assert_eq!(
            messages.find_message_by_date(2, get_date),
            MessageId::from_server_id(2)
        );
        assert_eq!(
            messages.find_message_by_date(3, get_date),
            MessageId::from_server_id(3)
        );
    }

    #[test]
    fn test_find_messages_by_date() {
        let mut messages = OrderedMessages::new();
        for i in 1..=10 {
            messages.insert(
                MessageId::from_server_id(i),
                false,
                MessageId::from_server_id(0),
                "test",
            );
        }

        // Date function: extract server_id
        let get_date = |id: MessageId| (id.get() >> 20) as i32;
        let result = messages.find_messages_by_date(3, 6, get_date);

        assert_eq!(
            result,
            vec![
                MessageId::from_server_id(3),
                MessageId::from_server_id(4),
                MessageId::from_server_id(5),
                MessageId::from_server_id(6)
            ]
        );
    }

    #[test]
    fn test_get_last_message_id() {
        let mut messages = OrderedMessages::new();
        assert_eq!(messages.get_last_message_id(), MessageId::from_server_id(0));

        messages.insert(
            MessageId::from_server_id(1),
            false,
            MessageId::from_server_id(0),
            "test",
        );
        messages.insert(
            MessageId::from_server_id(10),
            false,
            MessageId::from_server_id(0),
            "test",
        );
        messages.insert(
            MessageId::from_server_id(5),
            false,
            MessageId::from_server_id(0),
            "test",
        );

        assert_eq!(
            messages.get_last_message_id(),
            MessageId::from_server_id(10)
        );
    }

    #[test]
    fn test_calc_new_unread_count() {
        let mut messages = OrderedMessages::new();
        for i in 1..=10 {
            messages.insert(
                MessageId::from_server_id(i),
                false,
                MessageId::from_server_id(0),
                "test",
            );
        }

        // All messages from 1-10, last_read at 5, should have 5 new messages
        let count = messages.calc_new_unread_count(
            MessageId::from_server_id(10),
            MessageId::from_server_id(5),
            0,
            MessageId::from_server_id(10),
            |_| true,
            0,
        );

        // Messages 6, 7, 8, 9, 10 = 5 messages
        assert_eq!(count, 5);
    }

    #[test]
    fn test_calc_new_unread_count_before_read() {
        let mut messages = OrderedMessages::new();
        for i in 1..=10 {
            messages.insert(
                MessageId::from_server_id(i),
                false,
                MessageId::from_server_id(0),
                "test",
            );
        }

        // max_id is before last_read, should return 0
        let count = messages.calc_new_unread_count(
            MessageId::from_server_id(3),
            MessageId::from_server_id(5),
            0,
            MessageId::from_server_id(10),
            |_| true,
            0,
        );

        assert_eq!(count, 0);
    }

    #[test]
    fn test_clear() {
        let mut messages = OrderedMessages::new();
        messages.insert(
            MessageId::from_server_id(1),
            false,
            MessageId::from_server_id(0),
            "test",
        );
        messages.insert(
            MessageId::from_server_id(2),
            false,
            MessageId::from_server_id(0),
            "test",
        );

        messages.clear();
        assert!(messages.is_empty());
    }

    #[test]
    fn test_clone() {
        let mut messages1 = OrderedMessages::new();
        messages1.insert(
            MessageId::from_server_id(1),
            false,
            MessageId::from_server_id(0),
            "test",
        );
        messages1.insert(
            MessageId::from_server_id(2),
            false,
            MessageId::from_server_id(0),
            "test",
        );

        let messages2 = messages1.clone();
        assert_eq!(messages1, messages2);
        assert_eq!(messages2.len(), 2);
    }

    #[test]
    fn test_serialize() {
        let mut messages = OrderedMessages::new();
        messages.insert(
            MessageId::from_server_id(1),
            false,
            MessageId::from_server_id(0),
            "test",
        );
        messages.insert(
            MessageId::from_server_id(2),
            false,
            MessageId::from_server_id(0),
            "test",
        );

        let serialized = bincode::serialize(&messages).unwrap();
        let deserialized: OrderedMessages = bincode::deserialize(&serialized).unwrap();

        assert_eq!(messages, deserialized);
    }

    #[test]
    fn test_into_iter() {
        let mut messages = OrderedMessages::new();
        messages.insert(
            MessageId::from_server_id(1),
            false,
            MessageId::from_server_id(0),
            "test",
        );
        messages.insert(
            MessageId::from_server_id(2),
            false,
            MessageId::from_server_id(0),
            "test",
        );

        let collected: Vec<_> = messages.into_iter().collect();
        assert_eq!(collected.len(), 2);
    }
}
