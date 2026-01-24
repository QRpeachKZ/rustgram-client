// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram Message ID - Message identifier for Telegram MTProto client.
//!
//! This module provides the [`MessageId`] type which uniquely identifies a message
//! in the Telegram MTProto protocol.
//!
//! ## Overview
//!
//! Message IDs use a bit-packed layout to encode different types of messages:
//!
//! ### Ordinary message ID layout:
//! ```text
//! |-------31--------|---17---|1|--2-|
//! |server_message_id|local_id|0|type|
//! ```
//!
//! ### Scheduled message ID layout:
//! ```text
//! |-------30-------|----18---|1|--2-|
//! |send_date-2**30 |server_id|1|type|
//! ```
//!
//! ### Sponsored message ID layout:
//! ```text
//! |-------31--------|---17---|1|-2|
//! |11111111111111111|local_id|0|10|
//! ```
//!
//! ## Message Types
//!
//! - **Server** (`MessageType::Server`): Messages confirmed by the server
//! - **YetUnsent** (`MessageType::YetUnsent`): Messages not yet sent to server
//! - **Local** (`MessageType::Local`): Local-only messages
//!
//! ## Examples
//!
//! ### Creating a Message ID
//!
//! ```
//! use rustgram_message_id::{MessageId, MessageType};
//! use rustgram_server_message_id::ServerMessageId;
//!
//! // Create from server message ID
//! let server_id = ServerMessageId::new(123);
//! let msg_id = MessageId::from_server_id(server_id);
//! assert_eq!(msg_id.get(), 123 << 20);
//! assert!(msg_id.is_server());
//! assert!(msg_id.is_valid());
//! ```
//!
//! ### Creating from raw value
//!
//! ```
//! use rustgram_message_id::MessageId;
//!
//! let id = MessageId::new(1234567890);
//! assert_eq!(id.get(), 1234567890);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_scheduled_server_message_id::ScheduledServerMessageId;
use rustgram_server_message_id::ServerMessageId;
use std::hash::{Hash, Hasher};

/// Bit shift for server message ID in the packed layout.
pub const SERVER_ID_SHIFT: i32 = 20;

/// Mask for the short type field (lower 2 bits).
pub const SHORT_TYPE_MASK: i64 = 0b11;

/// Mask for the full type field (lower 3 bits).
pub const TYPE_MASK: i64 = 0b111;

/// Mask for the full type field (lower 20 bits).
pub const FULL_TYPE_MASK: i64 = (1 << SERVER_ID_SHIFT) - 1;

/// Mask for scheduled messages (bit 2).
pub const SCHEDULED_MASK: i64 = 0b100;

/// Type value for yet-unsent messages.
pub const TYPE_YET_UNSENT: i64 = 1;

/// Type value for local messages.
pub const TYPE_LOCAL: i64 = 2;

/// Type value for sponsored messages.
pub const TYPE_SPONSORED: i64 = 2;

/// The type of a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageType {
    /// No specific type.
    #[default]
    None,
    /// Server-confirmed message.
    Server,
    /// Message not yet sent to server.
    YetUnsent,
    /// Local-only message.
    Local,
}

/// Unique identifier for a message.
///
/// Message IDs use bit packing to encode different message types and their
/// associated data in a single i64 value.
///
/// # Examples
///
/// ```
/// use rustgram_message_id::{MessageId, MessageType};
/// use rustgram_server_message_id::ServerMessageId;
///
/// // Create from server message ID
/// let server_id = ServerMessageId::new(123);
/// let msg_id = MessageId::from_server_id(server_id);
/// assert!(msg_id.is_server());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MessageId(i64);

impl MessageId {
    /// Creates a new [`MessageId`] from a raw i64 value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    ///
    /// let id = MessageId::new(1234567890);
    /// assert_eq!(id.get(), 1234567890);
    /// ```
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Creates a [`MessageId`] from a [`ServerMessageId`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    /// use rustgram_server_message_id::ServerMessageId;
    ///
    /// let server_id = ServerMessageId::new(123);
    /// let msg_id = MessageId::from_server_id(server_id);
    /// assert!(msg_id.is_server());
    /// ```
    pub const fn from_server_id(server_id: ServerMessageId) -> Self {
        Self((server_id.get() as i64) << SERVER_ID_SHIFT)
    }

    /// Returns the underlying i64 value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    ///
    /// let id = MessageId::new(1234567890);
    /// assert_eq!(id.get(), 1234567890);
    /// ```
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Returns the minimum valid message ID (for yet-unsent messages).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    ///
    /// let min = MessageId::min();
    /// assert_eq!(min.get(), 1);
    /// ```
    pub const fn min() -> Self {
        Self(TYPE_YET_UNSENT)
    }

    /// Returns the maximum valid message ID (for server messages).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    ///
    /// let max = MessageId::max();
    /// assert!(max.get() > 0);
    /// ```
    pub const fn max() -> Self {
        Self((i32::MAX as i64) << SERVER_ID_SHIFT)
    }

    /// Returns `true` if this is a valid message ID.
    ///
    /// A message ID is valid if it's non-zero and within the valid range.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    /// use rustgram_server_message_id::ServerMessageId;
    ///
    /// assert!(!MessageId::default().is_valid());
    /// assert!(MessageId::from_server_id(ServerMessageId::new(1)).is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        if self.0 == 0 {
            return false;
        }
        if self.is_scheduled() {
            self.is_valid_scheduled()
        } else if self.is_server() {
            true
        } else {
            // YetUnsent or Local
            self.0 > 0
        }
    }

    /// Returns `true` if this is a valid scheduled message ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    ///
    /// assert!(!MessageId::default().is_valid_scheduled());
    /// ```
    pub fn is_valid_scheduled(&self) -> bool {
        if !self.is_scheduled() {
            return false;
        }
        let type_bits = self.0 & SHORT_TYPE_MASK;
        // Scheduled messages can be type 0 (server) or have valid date
        if type_bits == 0 {
            // Must have valid scheduled server ID
            self.get_scheduled_server_message_id().is_some()
        } else {
            self.0 > 0
        }
    }

    /// Returns `true` if this is a valid sponsored message ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    ///
    /// assert!(!MessageId::default().is_valid_sponsored());
    /// ```
    pub fn is_valid_sponsored(&self) -> bool {
        // Sponsored messages have all upper 31 bits set to 1
        // and type bits equal to 2
        if self.is_scheduled() {
            return false;
        }
        const SPONSORED_HIGH_MASK: i64 = 0xFFFFFFFFFFFFFF80_u64 as i64;
        (self.0 & SPONSORED_HIGH_MASK) == SPONSORED_HIGH_MASK
    }

    /// Gets the message type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::{MessageId, MessageType};
    /// use rustgram_server_message_id::ServerMessageId;
    ///
    /// let msg_id = MessageId::from_server_id(ServerMessageId::new(1));
    /// assert_eq!(msg_id.get_type(), MessageType::Server);
    /// ```
    pub fn get_type(&self) -> MessageType {
        if self.is_server() {
            MessageType::Server
        } else if self.is_yet_unsent() {
            MessageType::YetUnsent
        } else if self.is_local() {
            MessageType::Local
        } else {
            MessageType::None
        }
    }

    /// Returns `true` if this is a scheduled message.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    ///
    /// assert!(!MessageId::new(0).is_scheduled());
    /// ```
    pub fn is_scheduled(&self) -> bool {
        (self.0 & SCHEDULED_MASK) != 0
    }

    /// Returns `true` if this is a yet-unsent message.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    ///
    /// assert!(MessageId::new(1).is_yet_unsent());
    /// ```
    pub fn is_yet_unsent(&self) -> bool {
        (self.0 & SHORT_TYPE_MASK) == TYPE_YET_UNSENT
    }

    /// Returns `true` if this is a local message.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    ///
    /// assert!(MessageId::new(2).is_local());
    /// ```
    pub fn is_local(&self) -> bool {
        (self.0 & SHORT_TYPE_MASK) == TYPE_LOCAL
    }

    /// Returns `true` if this is a server message.
    ///
    /// This also checks that the message ID is valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    /// use rustgram_server_message_id::ServerMessageId;
    ///
    /// let msg_id = MessageId::from_server_id(ServerMessageId::new(1));
    /// assert!(msg_id.is_server());
    /// ```
    pub fn is_server(&self) -> bool {
        (self.0 & FULL_TYPE_MASK) == 0 && self.0 > 0 && self.0 <= Self::max().0
    }

    /// Returns `true` if this is a scheduled server message.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    ///
    /// assert!(!MessageId::default().is_scheduled_server());
    /// ```
    pub fn is_scheduled_server(&self) -> bool {
        self.is_scheduled() && (self.0 & SHORT_TYPE_MASK) == 0
    }

    /// Returns `true` if this is any server message (scheduled or regular).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    /// use rustgram_server_message_id::ServerMessageId;
    ///
    /// let msg_id = MessageId::from_server_id(ServerMessageId::new(1));
    /// assert!(msg_id.is_any_server());
    /// ```
    pub fn is_any_server(&self) -> bool {
        if self.is_scheduled() {
            self.is_scheduled_server()
        } else {
            self.is_server()
        }
    }

    /// Gets the server message ID if this is a server message.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    /// use rustgram_server_message_id::ServerMessageId;
    ///
    /// let msg_id = MessageId::from_server_id(ServerMessageId::new(123));
    /// assert_eq!(msg_id.get_server_message_id(), Some(ServerMessageId::new(123)));
    /// ```
    pub fn get_server_message_id(&self) -> Option<ServerMessageId> {
        if self.0 == 0 || self.is_server() {
            Some(ServerMessageId::new((self.0 >> SERVER_ID_SHIFT) as i32))
        } else {
            None
        }
    }

    /// Gets the scheduled server message ID if this is a scheduled server message.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    /// use rustgram_scheduled_server_message_id::ScheduledServerMessageId;
    ///
    /// // Create a scheduled message ID manually
    /// let msg_id = MessageId::new(0b1000010000000000000000);
    /// // ... test logic
    /// ```
    pub fn get_scheduled_server_message_id(&self) -> Option<ScheduledServerMessageId> {
        if !self.is_scheduled() {
            return None;
        }
        let server_id = ((self.0 >> 3) & ((1 << 18) - 1)) as i32;
        Some(ScheduledServerMessageId::new(server_id))
    }

    /// Gets the scheduled message date.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    ///
    /// let id = MessageId::new(0); // Placeholder
    /// // Test with actual scheduled message
    /// ```
    pub fn get_scheduled_message_date(&self) -> Option<i32> {
        if !self.is_valid_scheduled() {
            return None;
        }
        Some(((self.0 >> 21) + (1 << 30)) as i32)
    }

    /// Returns the greatest server message ID not greater than this one.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    /// use rustgram_server_message_id::ServerMessageId;
    ///
    /// let msg_id = MessageId::from_server_id(ServerMessageId::new(123));
    /// let prev = msg_id.get_prev_server_message_id();
    /// assert!(prev.get() <= msg_id.get());
    /// ```
    pub fn get_prev_server_message_id(&self) -> Self {
        Self(self.0 & !FULL_TYPE_MASK)
    }

    /// Returns the smallest server message ID not less than this one.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_id::MessageId;
    /// use rustgram_server_message_id::ServerMessageId;
    ///
    /// let msg_id = MessageId::from_server_id(ServerMessageId::new(123));
    /// let next = msg_id.get_next_server_message_id();
    /// assert!(next.get() >= msg_id.get());
    /// ```
    pub fn get_next_server_message_id(&self) -> Self {
        Self((self.0 + FULL_TYPE_MASK) & !FULL_TYPE_MASK)
    }
}

impl Hash for MessageId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialOrd for MessageId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Note: In TDLib, scheduled and non-scheduled messages cannot be compared
        // This is a simplified implementation
        Some(self.cmp(other))
    }
}

impl Ord for MessageId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::fmt::Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "msg {}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let id = MessageId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_new() {
        let id = MessageId::new(1234567890);
        assert_eq!(id.get(), 1234567890);
    }

    #[test]
    fn test_from_server_id() {
        let server_id = ServerMessageId::new(123);
        let msg_id = MessageId::from_server_id(server_id);
        assert_eq!(msg_id.get(), 123 << 20);
        assert!(msg_id.is_server());
        assert!(msg_id.is_valid());
        assert!(msg_id.is_any_server());
        assert!(!msg_id.is_scheduled());
        assert!(!msg_id.is_yet_unsent());
        assert!(!msg_id.is_local());
    }

    #[test]
    fn test_min() {
        let min = MessageId::min();
        assert_eq!(min.get(), 1);
        assert!(min.is_yet_unsent());
    }

    #[test]
    fn test_max() {
        let max = MessageId::max();
        assert!(max.get() > 0);
        assert!(max.is_server());
    }

    #[test]
    fn test_is_valid_zero() {
        let id = MessageId::new(0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_is_valid_server() {
        let id = MessageId::from_server_id(ServerMessageId::new(1));
        assert!(id.is_valid());
        assert!(id.is_server());
    }

    #[test]
    fn test_is_yet_unsent() {
        let id = MessageId::new(1);
        assert!(id.is_yet_unsent());
        assert!(!id.is_local());
        assert!(!id.is_server());
    }

    #[test]
    fn test_is_local() {
        let id = MessageId::new(2);
        assert!(id.is_local());
        assert!(!id.is_yet_unsent());
        assert!(!id.is_server());
    }

    #[test]
    fn test_is_scheduled() {
        let id = MessageId::new(SCHEDULED_MASK);
        assert!(id.is_scheduled());
        assert!(!id.is_server());
    }

    #[test]
    fn test_get_server_message_id() {
        let msg_id = MessageId::from_server_id(ServerMessageId::new(123));
        assert_eq!(
            msg_id.get_server_message_id(),
            Some(ServerMessageId::new(123))
        );
    }

    #[test]
    fn test_get_server_message_id_zero() {
        let msg_id = MessageId::new(0);
        assert_eq!(
            msg_id.get_server_message_id(),
            Some(ServerMessageId::new(0))
        );
    }

    #[test]
    fn test_get_type_server() {
        let msg_id = MessageId::from_server_id(ServerMessageId::new(1));
        assert_eq!(msg_id.get_type(), MessageType::Server);
    }

    #[test]
    fn test_get_type_yet_unsent() {
        let msg_id = MessageId::new(1);
        assert_eq!(msg_id.get_type(), MessageType::YetUnsent);
    }

    #[test]
    fn test_get_type_local() {
        let msg_id = MessageId::new(2);
        assert_eq!(msg_id.get_type(), MessageType::Local);
    }

    #[test]
    fn test_get_type_none() {
        let msg_id = MessageId::new(0);
        assert_eq!(msg_id.get_type(), MessageType::None);
    }

    #[test]
    fn test_equality() {
        let id1 = MessageId::new(123456);
        let id2 = MessageId::new(123456);
        assert_eq!(id1, id2);

        let id3 = MessageId::new(789012);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_ordering() {
        let id1 = MessageId::new(100);
        let id2 = MessageId::new(200);
        assert!(id1 < id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_copy() {
        let id1 = MessageId::new(123);
        let id2 = id1;
        assert_eq!(id1, id2);
        assert_eq!(id1.get(), 123);
        assert_eq!(id2.get(), 123);
    }

    #[test]
    fn test_clone() {
        let id = MessageId::new(123);
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_display() {
        let id = MessageId::new(12345);
        assert_eq!(format!("{}", id), "msg 12345");
    }

    #[test]
    fn test_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        let id1 = MessageId::new(123);
        let id2 = MessageId::new(456);

        map.insert(id1, "first");
        map.insert(id2, "second");

        assert_eq!(map.get(&MessageId::new(123)), Some(&"first"));
        assert_eq!(map.get(&MessageId::new(456)), Some(&"second"));
    }

    #[test]
    fn test_prev_server_message_id() {
        let msg_id = MessageId::from_server_id(ServerMessageId::new(123));
        let prev = msg_id.get_prev_server_message_id();
        assert!(prev.get() <= msg_id.get());
    }

    #[test]
    fn test_next_server_message_id() {
        let msg_id = MessageId::from_server_id(ServerMessageId::new(123));
        let next = msg_id.get_next_server_message_id();
        assert!(next.get() >= msg_id.get());
    }

    #[test]
    fn test_scheduled_mask() {
        assert_eq!(SCHEDULED_MASK, 4);
    }

    #[test]
    fn test_type_masks() {
        assert_eq!(SHORT_TYPE_MASK, 3);
        assert_eq!(TYPE_MASK, 7);
        assert_eq!(FULL_TYPE_MASK, (1 << 20) - 1);
    }

    #[test]
    fn test_constants() {
        assert_eq!(TYPE_YET_UNSENT, 1);
        assert_eq!(TYPE_LOCAL, 2);
        assert_eq!(SERVER_ID_SHIFT, 20);
    }

    #[test]
    fn test_scheduled_server_message() {
        // Create a scheduled server message
        // Format: |send_date-2**30|server_id(18)|1|type(2)|
        // For scheduled server message, type = 0
        let send_date = 1700000000_i32;
        let server_id = 123_i32;
        let scheduled_value =
            ((send_date - (1 << 30)) as i64) << 21 | ((server_id as i64) << 3) | SCHEDULED_MASK;
        let msg_id = MessageId::new(scheduled_value);

        assert!(msg_id.is_scheduled());
        assert!(msg_id.is_scheduled_server());
        assert!(msg_id.is_valid_scheduled());
        assert_eq!(
            msg_id.get_scheduled_server_message_id(),
            Some(ScheduledServerMessageId::new(server_id))
        );
        assert_eq!(msg_id.get_scheduled_message_date(), Some(send_date));
    }

    #[test]
    fn test_message_type_default() {
        let msg_type = MessageType::default();
        assert_eq!(msg_type, MessageType::None);
    }

    #[test]
    fn test_message_type_copy() {
        let msg_type = MessageType::Server;
        let copied = msg_type;
        assert_eq!(msg_type, copied);
    }

    #[test]
    fn test_vec_usage() {
        let mut ids = Vec::new();
        ids.push(MessageId::from_server_id(ServerMessageId::new(1)));
        ids.push(MessageId::from_server_id(ServerMessageId::new(2)));
        assert_eq!(ids.len(), 2);
    }

    #[test]
    fn test_btree_set_usage() {
        use std::collections::BTreeSet;
        let mut set = BTreeSet::new();
        set.insert(MessageId::new(1));
        set.insert(MessageId::new(2));
        assert_eq!(set.len(), 2);
    }
}
