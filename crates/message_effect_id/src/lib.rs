// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram Message Effect ID - Message effect identifier for Telegram MTProto client.
//!
//! This module provides the [`MessageEffectId`] type which uniquely identifies a
//! message effect in the Telegram MTProto protocol.
//!
//! ## Overview
//!
//! Message effect IDs are used to uniquely identify special effects that can be
//! applied to messages (such as emoji reactions, animations, etc.). A valid
//! message effect ID is any non-zero i64 value. The value 0 is reserved and
//! represents an invalid or empty message effect ID.
//!
//! ## Examples
//!
//! ### Creating a Message Effect ID
//!
//! ```
//! use rustgram_message_effect_id::MessageEffectId;
//!
//! // Create from i64
//! let id = MessageEffectId::new(12345);
//! assert!(id.is_valid());
//! assert_eq!(id.get(), 12345);
//!
//! // Default is invalid
//! let default = MessageEffectId::default();
//! assert!(!default.is_valid());
//! assert_eq!(default.get(), 0);
//! ```
//!
//! ### Using with HashMap
//!
//! ```
//! use rustgram_message_effect_id::MessageEffectId;
//! use std::collections::HashMap;
//!
//! let mut map = HashMap::new();
//! let id = MessageEffectId::new(123);
//! map.insert(id, "Effect data");
//! assert_eq!(map.get(&MessageEffectId::new(123)), Some(&"Effect data"));
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::hash::{Hash, Hasher};

/// Unique identifier for a message effect.
///
/// Represents a message effect ID in the Telegram MTProto protocol. Valid message
/// effect IDs are non-zero integers, while 0 represents an invalid or empty message
/// effect ID.
///
/// # Examples
///
/// ```
/// use rustgram_message_effect_id::MessageEffectId;
///
/// let id = MessageEffectId::new(12345);
/// assert!(id.is_valid());
/// assert_eq!(id.get(), 12345);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MessageEffectId(i64);

impl MessageEffectId {
    /// Creates a new [`MessageEffectId`] from an i64 value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_effect_id::MessageEffectId;
    ///
    /// let id = MessageEffectId::new(12345);
    /// assert_eq!(id.get(), 12345);
    /// ```
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the underlying i64 value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_effect_id::MessageEffectId;
    ///
    /// let id = MessageEffectId::new(12345);
    /// assert_eq!(id.get(), 12345);
    /// ```
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Returns `true` if this is a valid message effect ID.
    ///
    /// A message effect ID is considered valid if it is non-zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_message_effect_id::MessageEffectId;
    ///
    /// assert!(!MessageEffectId::default().is_valid());
    /// assert!(MessageEffectId::new(123).is_valid());
    /// assert!(!MessageEffectId::new(0).is_valid());
    /// ```
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

impl Hash for MessageEffectId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl std::fmt::Display for MessageEffectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "message effect {}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let id = MessageEffectId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_new_valid() {
        let id = MessageEffectId::new(12345);
        assert_eq!(id.get(), 12345);
        assert!(id.is_valid());
    }

    #[test]
    fn test_new_zero() {
        let id = MessageEffectId::new(0);
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_new_negative() {
        let id = MessageEffectId::new(-1);
        assert_eq!(id.get(), -1);
        assert!(id.is_valid());
    }

    #[test]
    fn test_new_large() {
        let id = MessageEffectId::new(i64::MAX);
        assert_eq!(id.get(), i64::MAX);
        assert!(id.is_valid());
    }

    #[test]
    fn test_equality() {
        let id1 = MessageEffectId::new(123);
        let id2 = MessageEffectId::new(123);
        assert_eq!(id1, id2);

        let id3 = MessageEffectId::new(456);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_ordering() {
        let id1 = MessageEffectId::new(100);
        let id2 = MessageEffectId::new(200);
        assert!(id1 < id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_copy() {
        let id1 = MessageEffectId::new(123);
        let id2 = id1;
        assert_eq!(id1, id2);
        assert_eq!(id1.get(), 123);
        assert_eq!(id2.get(), 123);
    }

    #[test]
    fn test_clone() {
        let id = MessageEffectId::new(123);
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_display() {
        let id = MessageEffectId::new(12345);
        assert_eq!(format!("{}", id), "message effect 12345");
    }

    #[test]
    fn test_debug() {
        let id = MessageEffectId::new(12345);
        let debug_str = format!("{:?}", id);
        assert!(debug_str.contains("12345"));
    }

    #[test]
    fn test_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        let id1 = MessageEffectId::new(123);
        let id2 = MessageEffectId::new(456);

        map.insert(id1, "first");
        map.insert(id2, "second");

        assert_eq!(map.get(&MessageEffectId::new(123)), Some(&"first"));
        assert_eq!(map.get(&MessageEffectId::new(456)), Some(&"second"));
    }

    #[test]
    fn test_const_context() {
        const ID: MessageEffectId = MessageEffectId::new(123);
        assert!(ID.is_valid());
        assert_eq!(ID.get(), 123);
    }

    #[test]
    fn test_array_usage() {
        let ids = [
            MessageEffectId::new(1),
            MessageEffectId::new(2),
            MessageEffectId::new(3),
        ];
        assert_eq!(ids[0].get(), 1);
        assert_eq!(ids[1].get(), 2);
        assert_eq!(ids[2].get(), 3);
    }

    #[test]
    fn test_option_usage() {
        let some_id: Option<MessageEffectId> = Some(MessageEffectId::new(123));
        assert!(some_id.is_some());
        assert!(some_id.unwrap().is_valid());

        let none_id: Option<MessageEffectId> = None;
        assert!(none_id.is_none());
    }

    #[test]
    fn test_vec_usage() {
        let mut ids = Vec::new();
        ids.push(MessageEffectId::new(10));
        ids.push(MessageEffectId::new(20));
        assert_eq!(ids.len(), 2);
        assert_eq!(ids[0].get(), 10);
    }

    #[test]
    fn test_btree_set_usage() {
        use std::collections::BTreeSet;
        let mut set = BTreeSet::new();
        set.insert(MessageEffectId::new(5));
        set.insert(MessageEffectId::new(10));
        assert_eq!(set.len(), 2);
        assert!(set.contains(&MessageEffectId::new(5)));
    }

    #[test]
    fn test_min_max_values() {
        let min = MessageEffectId::new(i64::MIN);
        assert!(min.is_valid());
        assert_eq!(min.get(), i64::MIN);

        let max = MessageEffectId::new(i64::MAX);
        assert!(max.is_valid());
        assert_eq!(max.get(), i64::MAX);
    }

    #[test]
    fn test_zero_is_invalid() {
        let zero = MessageEffectId::new(0);
        assert!(!zero.is_valid());
        assert_eq!(zero, MessageEffectId::default());
    }
}
