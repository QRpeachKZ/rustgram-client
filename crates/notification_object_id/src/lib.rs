// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Notification object ID type for Telegram MTProto client.
//!
//! This module implements TDLib's NotificationObjectId.
//!
//! # Example
//!
//! ```rust
//! use rustgram_types::MessageId;
//! use rustgram_notification_object_id::NotificationObjectId;
//!
//! let msg_id = MessageId::from_server_id(123);
//! let obj_id = NotificationObjectId::from_message_id(msg_id);
//! assert!(obj_id.is_valid());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_types::MessageId;
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;

/// Notification object identifier.
///
/// Based on TDLib's `NotificationObjectId` class.
///
/// This wraps a MessageId to provide a unique identifier for notification objects.
/// Valid notification object IDs are positive integers (greater than 0).
///
/// # Example
///
/// ```rust
/// use rustgram_types::MessageId;
/// use rustgram_notification_object_id::NotificationObjectId;
///
/// let msg_id = MessageId::from_server_id(100);
/// let obj_id = NotificationObjectId::from_message_id(msg_id);
/// assert!(obj_id.is_valid());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NotificationObjectId(i64);

impl NotificationObjectId {
    /// Creates a new NotificationObjectId from an i64 value.
    ///
    /// # Arguments
    ///
    /// * `id` - The object ID value
    ///
    /// # Example
    ///
    /// ```rust
/// use rustgram_notification_object_id::NotificationObjectId;
    ///
    /// let id = NotificationObjectId::new(1234567890);
    /// ```
    #[inline]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Creates a NotificationObjectId from a MessageId.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The message ID to wrap
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_types::MessageId;
    /// use rustgram_notification_object_id::NotificationObjectId;
    ///
    /// let msg_id = MessageId::from_server_id(42);
    /// let obj_id = NotificationObjectId::from_message_id(msg_id);
    /// ```
    #[inline]
    pub const fn from_message_id(message_id: MessageId) -> Self {
        Self(message_id.get())
    }

    /// Returns the inner i64 value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_object_id::NotificationObjectId;
    ///
    /// let id = NotificationObjectId::new(9876543210);
    /// assert_eq!(id.get(), 9876543210);
    /// ```
    #[inline]
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Checks if this is a valid notification object ID.
    ///
    /// Valid notification object IDs are positive (greater than 0).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_object_id::NotificationObjectId;
    ///
    /// assert!(NotificationObjectId::new(1).is_valid());
    /// assert!(NotificationObjectId::new(100).is_valid());
    /// assert!(!NotificationObjectId::new(0).is_valid());
    /// assert!(!NotificationObjectId::new(-1).is_valid());
    /// ```
    #[inline]
    pub const fn is_valid(self) -> bool {
        self.0 > 0
    }

    /// Returns the maximum possible notification object ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_object_id::NotificationObjectId;
    ///
    /// let max = NotificationObjectId::max();
    /// assert_eq!(max.get(), i64::MAX);
    /// ```
    #[inline]
    pub const fn max() -> Self {
        Self(i64::MAX)
    }
}

impl Default for NotificationObjectId {
    fn default() -> Self {
        Self(0)
    }
}

impl Display for NotificationObjectId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "notification object {}", self.0)
    }
}

impl From<i64> for NotificationObjectId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<NotificationObjectId> for i64 {
    fn from(id: NotificationObjectId) -> Self {
        id.0
    }
}

impl From<MessageId> for NotificationObjectId {
    fn from(message_id: MessageId) -> Self {
        Self::from_message_id(message_id)
    }
}

impl PartialOrd for NotificationObjectId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NotificationObjectId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl Serialize for NotificationObjectId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for NotificationObjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        i64::deserialize(deserializer)
            .map(Self)
            .map_err(|e| serde::de::Error::custom(format!("invalid NotificationObjectId: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = NotificationObjectId::new(1234567890);
        assert_eq!(id.get(), 1234567890);
    }

    #[test]
    fn test_from_message_id() {
        let msg_id = MessageId::from_server_id(100);
        let obj_id = NotificationObjectId::from_message_id(msg_id);
        assert!(obj_id.is_valid());
        assert_eq!(obj_id.get(), msg_id.get());
    }

    #[test]
    fn test_get() {
        assert_eq!(NotificationObjectId::new(1).get(), 1);
        assert_eq!(NotificationObjectId::new(100).get(), 100);
        assert_eq!(NotificationObjectId::new(0).get(), 0);
        assert_eq!(NotificationObjectId::new(-50).get(), -50);
    }

    #[test]
    fn test_is_valid() {
        assert!(NotificationObjectId::new(1).is_valid());
        assert!(NotificationObjectId::new(100).is_valid());
        assert!(NotificationObjectId::new(i64::MAX).is_valid());
        assert!(!NotificationObjectId::new(0).is_valid());
        assert!(!NotificationObjectId::new(-1).is_valid());
        assert!(!NotificationObjectId::new(-100).is_valid());
        assert!(!NotificationObjectId::new(i64::MIN).is_valid());
    }

    #[test]
    fn test_max() {
        let max = NotificationObjectId::max();
        assert_eq!(max.get(), i64::MAX);
        assert!(max.is_valid());
    }

    #[test]
    fn test_default() {
        let id = NotificationObjectId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_equality() {
        let id1 = NotificationObjectId::new(12345);
        let id2 = NotificationObjectId::new(12345);
        let id3 = NotificationObjectId::new(67890);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_ordering() {
        let id1 = NotificationObjectId::new(100);
        let id2 = NotificationObjectId::new(200);
        let id3 = NotificationObjectId::new(150);

        assert!(id1 < id2);
        assert!(id1 < id3);
        assert!(id3 < id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_partial_ordering() {
        let id1 = NotificationObjectId::new(10);
        let id2 = NotificationObjectId::new(20);

        assert!(id1.partial_cmp(&id2).unwrap().is_lt());
        assert!(id2.partial_cmp(&id1).unwrap().is_gt());
        assert!(id1.partial_cmp(&id1).unwrap().is_eq());
    }

    #[test]
    fn test_copy() {
        let id1 = NotificationObjectId::new(7890123);
        let id2 = id1;
        assert_eq!(id1, id2);
        assert_eq!(id1.get(), 7890123);
        assert_eq!(id2.get(), 7890123);
    }

    #[test]
    fn test_clone() {
        let id1 = NotificationObjectId::new(9876543);
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(NotificationObjectId::new(1));
        set.insert(NotificationObjectId::new(2));
        set.insert(NotificationObjectId::new(3));
        assert_eq!(set.len(), 3);

        // Duplicate doesn't increase size
        set.insert(NotificationObjectId::new(2));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", NotificationObjectId::new(42)),
            "notification object 42"
        );
        assert_eq!(
            format!("{}", NotificationObjectId::new(0)),
            "notification object 0"
        );
    }

    #[test]
    fn test_debug() {
        assert_eq!(
            format!("{:?}", NotificationObjectId::new(123)),
            "NotificationObjectId(123)"
        );
    }

    #[test]
    fn test_from_i64() {
        let id: NotificationObjectId = 456789.into();
        assert_eq!(id.get(), 456789);
    }

    #[test]
    fn test_into_i64() {
        let id = NotificationObjectId::new(789012);
        let value: i64 = id.into();
        assert_eq!(value, 789012);
    }

    #[test]
    fn test_from_message_id_trait() {
        let msg_id = MessageId::from_server_id(200);
        let obj_id: NotificationObjectId = msg_id.into();
        assert_eq!(obj_id.get(), msg_id.get());
    }

    #[test]
    fn test_const_creation() {
        const ID: NotificationObjectId = NotificationObjectId::new(42);
        assert_eq!(ID.get(), 42);
    }

    #[test]
    fn test_zero_is_invalid() {
        assert!(!NotificationObjectId::new(0).is_valid());
    }

    #[test]
    fn test_negative_is_invalid() {
        assert!(!NotificationObjectId::new(-1).is_valid());
        assert!(!NotificationObjectId::new(-100).is_valid());
        assert!(!NotificationObjectId::new(i64::MIN).is_valid());
    }

    #[test]
    fn test_positive_is_valid() {
        assert!(NotificationObjectId::new(1).is_valid());
        assert!(NotificationObjectId::new(100).is_valid());
        assert!(NotificationObjectId::new(i64::MAX).is_valid());
    }

    #[test]
    fn test_ord_total_ordering() {
        let ids = vec![
            NotificationObjectId::new(30),
            NotificationObjectId::new(10),
            NotificationObjectId::new(20),
        ];

        let mut sorted = ids.clone();
        sorted.sort();

        assert_eq!(sorted[0].get(), 10);
        assert_eq!(sorted[1].get(), 20);
        assert_eq!(sorted[2].get(), 30);
    }

    #[test]
    fn test_notification_object_id_in_hashmap() {
        use std::collections::HashMap;

        let mut map: HashMap<NotificationObjectId, String> = HashMap::new();

        map.insert(NotificationObjectId::new(1), "first".to_string());
        map.insert(NotificationObjectId::new(2), "second".to_string());

        assert_eq!(
            map.get(&NotificationObjectId::new(1)),
            Some(&"first".to_string())
        );
        assert_eq!(
            map.get(&NotificationObjectId::new(2)),
            Some(&"second".to_string())
        );
        assert_eq!(map.get(&NotificationObjectId::new(3)), None);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let id = NotificationObjectId::new(1234567890);
        let serialized = serde_json::to_string(&id).unwrap();
        let deserialized: NotificationObjectId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_json() {
        let id = NotificationObjectId::new(999999);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "999999");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_json() {
        let json = "1234567890";
        let id: NotificationObjectId = serde_json::from_str(json).unwrap();
        assert_eq!(id.get(), 1234567890);
    }

    #[test]
    fn test_from_message_id_valid() {
        let msg_id = MessageId::from_server_id(999);
        let obj_id = NotificationObjectId::from_message_id(msg_id);
        assert!(obj_id.is_valid());
    }
}
