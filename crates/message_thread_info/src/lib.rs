// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Message Thread Info
//!
//! Information about a message thread in Telegram.
//!
//! ## Overview
//!
//! A message thread represents a conversation thread within a dialog,
//! typically in the form of replies to a specific message in megagroups.
//!
//! ## TDLib Reference
//!
//! - `td/telegram/MessageThreadInfo.h`
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_thread_info::MessageThreadInfo;
//! use rustgram_types::{DialogId, MessageId};
//!
//! let dialog_id = DialogId::from_channel(123);
//! let message_ids = vec![
//!     MessageId::from_server_id(1),
//!     MessageId::from_server_id(2),
//! ];
//!
//! let info = MessageThreadInfo::new(dialog_id, message_ids, 5);
//! assert_eq!(info.dialog_id(), dialog_id);
//! assert_eq!(info.message_ids().len(), 2);
//! assert_eq!(info.unread_message_count(), 5);
//! ```

use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Information about a message thread.
///
/// Contains the dialog ID, list of message IDs in the thread,
/// and the count of unread messages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageThreadInfo {
    /// The dialog containing this thread
    dialog_id: DialogId,
    /// List of message IDs in this thread
    message_ids: Vec<MessageId>,
    /// Number of unread messages in the thread
    unread_message_count: i32,
}

impl Default for MessageThreadInfo {
    fn default() -> Self {
        Self::new(DialogId::default(), Vec::new(), 0)
    }
}

impl fmt::Display for MessageThreadInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Thread in {} with {} messages ({} unread)",
            self.dialog_id,
            self.message_ids.len(),
            self.unread_message_count
        )
    }
}

impl MessageThreadInfo {
    /// Creates a new message thread info.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog containing this thread
    /// * `message_ids` - List of message IDs in the thread
    /// * `unread_message_count` - Number of unread messages
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_thread_info::MessageThreadInfo;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let dialog_id = DialogId::from_channel(123);
    /// let message_ids = vec![MessageId::from_server_id(1)];
    ///
    /// let info = MessageThreadInfo::new(dialog_id, message_ids, 5);
    /// assert_eq!(info.unread_message_count(), 5);
    /// ```
    #[must_use]
    pub fn new(
        dialog_id: DialogId,
        message_ids: Vec<MessageId>,
        unread_message_count: i32,
    ) -> Self {
        Self {
            dialog_id,
            message_ids,
            unread_message_count,
        }
    }

    /// Returns the dialog ID.
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the list of message IDs in this thread.
    #[must_use]
    pub fn message_ids(&self) -> &[MessageId] {
        &self.message_ids
    }

    /// Returns the number of unread messages in the thread.
    #[must_use]
    pub const fn unread_message_count(&self) -> i32 {
        self.unread_message_count
    }

    /// Sets the dialog ID.
    pub fn set_dialog_id(&mut self, dialog_id: DialogId) {
        self.dialog_id = dialog_id;
    }

    /// Sets the list of message IDs.
    pub fn set_message_ids(&mut self, message_ids: Vec<MessageId>) {
        self.message_ids = message_ids;
    }

    /// Sets the unread message count.
    pub fn set_unread_message_count(&mut self, count: i32) {
        self.unread_message_count = count;
    }

    /// Adds a message ID to the thread.
    pub fn add_message_id(&mut self, message_id: MessageId) {
        self.message_ids.push(message_id);
    }

    /// Clears all message IDs.
    pub fn clear_message_ids(&mut self) {
        self.message_ids.clear();
    }

    /// Returns the number of messages in the thread.
    #[must_use]
    pub fn len(&self) -> usize {
        self.message_ids.len()
    }

    /// Returns true if the thread has no messages.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.message_ids.is_empty()
    }

    /// Returns true if there are unread messages.
    #[must_use]
    pub const fn has_unread(&self) -> bool {
        self.unread_message_count > 0
    }

    /// Increments the unread message count.
    pub fn increment_unread(&mut self, amount: i32) {
        self.unread_message_count = self.unread_message_count.saturating_add(amount);
    }

    /// Decrements the unread message count.
    pub fn decrement_unread(&mut self, amount: i32) {
        self.unread_message_count = self.unread_message_count.saturating_sub(amount);
    }

    /// Marks all messages as read.
    pub fn mark_all_read(&mut self) {
        self.unread_message_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::ChannelId;

    #[test]
    fn test_default() {
        let info = MessageThreadInfo::default();
        assert!(info.is_empty());
        assert_eq!(info.unread_message_count(), 0);
        assert!(!info.has_unread());
    }

    #[test]
    fn test_new() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let message_ids = vec![MessageId::from_server_id(1), MessageId::from_server_id(2)];
        let info = MessageThreadInfo::new(dialog_id.clone(), message_ids, 5);

        assert_eq!(info.dialog_id(), dialog_id);
        assert_eq!(info.len(), 2);
        assert_eq!(info.unread_message_count(), 5);
        assert!(info.has_unread());
    }

    #[test]
    fn test_empty() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let info = MessageThreadInfo::new(dialog_id, vec![], 0);

        assert!(info.is_empty());
        assert!(!info.has_unread());
    }

    #[test]
    fn test_add_message_id() {
        let mut info = MessageThreadInfo::default();
        assert!(info.is_empty());

        info.add_message_id(MessageId::from_server_id(1));
        assert_eq!(info.len(), 1);

        info.add_message_id(MessageId::from_server_id(2));
        assert_eq!(info.len(), 2);
    }

    #[test]
    fn test_clear_message_ids() {
        let mut info =
            MessageThreadInfo::new(DialogId::default(), vec![MessageId::from_server_id(1)], 5);
        assert!(!info.is_empty());

        info.clear_message_ids();
        assert!(info.is_empty());
    }

    #[test]
    fn test_setters() {
        let mut info = MessageThreadInfo::default();
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);

        info.set_dialog_id(dialog_id);
        info.set_unread_message_count(10);
        info.set_message_ids(vec![MessageId::from_server_id(1)]);

        assert_eq!(info.dialog_id(), dialog_id);
        assert_eq!(info.unread_message_count(), 10);
        assert_eq!(info.len(), 1);
    }

    #[test]
    fn test_increment_unread() {
        let mut info = MessageThreadInfo::default();
        assert_eq!(info.unread_message_count(), 0);

        info.increment_unread(5);
        assert_eq!(info.unread_message_count(), 5);

        info.increment_unread(3);
        assert_eq!(info.unread_message_count(), 8);
    }

    #[test]
    fn test_decrement_unread() {
        let mut info = MessageThreadInfo::new(DialogId::default(), vec![], 10);

        info.decrement_unread(3);
        assert_eq!(info.unread_message_count(), 7);

        info.decrement_unread(10);
        assert_eq!(info.unread_message_count(), 0); // saturates at 0
    }

    #[test]
    fn test_mark_all_read() {
        let mut info = MessageThreadInfo::new(DialogId::default(), vec![], 10);
        assert!(info.has_unread());

        info.mark_all_read();
        assert!(!info.has_unread());
        assert_eq!(info.unread_message_count(), 0);
    }

    #[test]
    fn test_has_unread() {
        let mut info = MessageThreadInfo::default();
        assert!(!info.has_unread());

        info.increment_unread(1);
        assert!(info.has_unread());

        info.mark_all_read();
        assert!(!info.has_unread());
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut info = MessageThreadInfo::default();
        assert_eq!(info.len(), 0);
        assert!(info.is_empty());

        info.add_message_id(MessageId::from_server_id(1));
        assert_eq!(info.len(), 1);
        assert!(!info.is_empty());

        info.add_message_id(MessageId::from_server_id(2));
        info.add_message_id(MessageId::from_server_id(3));
        assert_eq!(info.len(), 3);
    }

    #[test]
    fn test_equality() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let messages = vec![MessageId::from_server_id(1)];

        let info1 = MessageThreadInfo::new(dialog_id, messages.clone(), 5);
        let info2 = MessageThreadInfo::new(dialog_id, messages, 5);
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_display() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let messages = vec![MessageId::from_server_id(1), MessageId::from_server_id(2)];
        let info = MessageThreadInfo::new(dialog_id, messages, 5);

        let s = format!("{}", info);
        assert!(s.contains("Thread"));
        assert!(s.contains("2 messages"));
        assert!(s.contains("5 unread"));
    }

    #[test]
    fn test_serialize() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let messages = vec![MessageId::from_server_id(1)];
        let info = MessageThreadInfo::new(dialog_id, messages, 5);

        let serialized = serde_json::to_string(&info).unwrap();
        let deserialized: MessageThreadInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(info, deserialized);
    }

    #[test]
    fn test_clone() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = DialogId::from_channel(channel_id);
        let messages = vec![MessageId::from_server_id(1), MessageId::from_server_id(2)];
        let info1 = MessageThreadInfo::new(dialog_id, messages, 5);
        let info2 = info1.clone();

        assert_eq!(info1, info2);
        assert_eq!(info2.len(), 2);
    }

    #[test]
    fn test_saturating_operations() {
        let mut info = MessageThreadInfo::new(DialogId::default(), vec![], 5);

        // Decrement more than available
        info.decrement_unread(100);
        assert_eq!(info.unread_message_count(), 0);

        // Increment to very large value
        info.increment_unread(i32::MAX);
        assert!(info.unread_message_count() > 0);

        // Saturate on overflow
        info.increment_unread(1);
        assert_eq!(info.unread_message_count(), i32::MAX);
    }
}
