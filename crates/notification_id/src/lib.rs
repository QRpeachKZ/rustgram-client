// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Notification ID type for Telegram MTProto client.
//!
//! This module implements TDLib's NotificationId.
//!
//! # Example
//!
//! ```rust
//! use rustgram_notification_id::NotificationId;
//!
//! let id = NotificationId::new(123);
//! assert!(id.is_valid());
//! assert_eq!(id.get(), 123);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::fmt::{self, Display, Formatter};
use std::hash::Hash;

/// Notification identifier.
///
/// Based on TDLib's `NotificationId` class.
///
/// Valid notification IDs are positive integers (greater than 0).
/// This is used to uniquely identify notifications within a dialog.
///
/// # Example
///
/// ```rust
/// use rustgram_notification_id::NotificationId;
///
/// let id = NotificationId::new(42);
/// assert!(id.is_valid());
/// assert_eq!(id.get(), 42);
///
/// let invalid = NotificationId::new(0);
/// assert!(!invalid.is_valid());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct NotificationId(i32);

impl NotificationId {
    /// Creates a new NotificationId.
    ///
    /// # Arguments
    ///
    /// * `id` - The notification ID value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_id::NotificationId;
    ///
    /// let id = NotificationId::new(123);
    /// ```
    #[inline]
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    /// Returns the inner i32 value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_id::NotificationId;
    ///
    /// let id = NotificationId::new(456);
    /// assert_eq!(id.get(), 456);
    /// ```
    #[inline]
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Checks if this is a valid notification ID.
    ///
    /// Valid notification IDs are positive (greater than 0).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_id::NotificationId;
    ///
    /// assert!(NotificationId::new(1).is_valid());
    /// assert!(NotificationId::new(100).is_valid());
    /// assert!(!NotificationId::new(0).is_valid());
    /// assert!(!NotificationId::new(-1).is_valid());
    /// ```
    #[inline]
    pub const fn is_valid(self) -> bool {
        self.0 > 0
    }

    /// Returns the maximum possible notification ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_id::NotificationId;
    ///
    /// let max = NotificationId::max();
    /// assert_eq!(max.get(), i32::MAX);
    /// ```
    #[inline]
    pub const fn max() -> Self {
        Self(i32::MAX)
    }
}

impl Hash for NotificationId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Display for NotificationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "notification {}", self.0)
    }
}

impl From<i32> for NotificationId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<NotificationId> for i32 {
    fn from(id: NotificationId) -> Self {
        id.0
    }
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
impl Serialize for NotificationId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for NotificationId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        i32::deserialize(deserializer)
            .map(Self)
            .map_err(|e| serde::de::Error::custom(format!("invalid NotificationId: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = NotificationId::new(123);
        assert_eq!(id.get(), 123);
    }

    #[test]
    fn test_get() {
        assert_eq!(NotificationId::new(1).get(), 1);
        assert_eq!(NotificationId::new(100).get(), 100);
        assert_eq!(NotificationId::new(-50).get(), -50);
        assert_eq!(NotificationId::new(0).get(), 0);
    }

    #[test]
    fn test_is_valid() {
        assert!(NotificationId::new(1).is_valid());
        assert!(NotificationId::new(100).is_valid());
        assert!(NotificationId::new(i32::MAX).is_valid());
        assert!(!NotificationId::new(0).is_valid());
        assert!(!NotificationId::new(-1).is_valid());
        assert!(!NotificationId::new(-100).is_valid());
        assert!(!NotificationId::new(i32::MIN).is_valid());
    }

    #[test]
    fn test_max() {
        let max = NotificationId::max();
        assert_eq!(max.get(), i32::MAX);
        assert!(max.is_valid());
    }

    #[test]
    fn test_default() {
        let id = NotificationId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_equality() {
        let id1 = NotificationId::new(123);
        let id2 = NotificationId::new(123);
        let id3 = NotificationId::new(456);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_ordering() {
        let id1 = NotificationId::new(100);
        let id2 = NotificationId::new(200);
        let id3 = NotificationId::new(150);

        assert!(id1 < id2);
        assert!(id1 < id3);
        assert!(id3 < id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_copy() {
        let id1 = NotificationId::new(789);
        let id2 = id1;
        assert_eq!(id1, id2);
        assert_eq!(id1.get(), 789);
        assert_eq!(id2.get(), 789);
    }

    #[test]
    fn test_clone() {
        let id1 = NotificationId::new(999);
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(NotificationId::new(1));
        set.insert(NotificationId::new(2));
        set.insert(NotificationId::new(3));
        assert_eq!(set.len(), 3);

        // Duplicate doesn't increase size
        set.insert(NotificationId::new(2));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", NotificationId::new(42)), "notification 42");
        assert_eq!(format!("{}", NotificationId::new(0)), "notification 0");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", NotificationId::new(123)), "NotificationId(123)");
    }

    #[test]
    fn test_from_i32() {
        let id: NotificationId = 456.into();
        assert_eq!(id.get(), 456);
    }

    #[test]
    fn test_into_i32() {
        let id = NotificationId::new(789);
        let value: i32 = id.into();
        assert_eq!(value, 789);
    }

    #[test]
    fn test_partial_ord() {
        let id1 = NotificationId::new(10);
        let id2 = NotificationId::new(20);

        assert!(id1.partial_cmp(&id2).unwrap().is_lt());
        assert!(id2.partial_cmp(&id1).unwrap().is_gt());
        assert!(id1.partial_cmp(&id1).unwrap().is_eq());
    }

    #[test]
    fn test_const_creation() {
        const ID: NotificationId = NotificationId::new(42);
        assert_eq!(ID.get(), 42);
    }

    #[test]
    fn test_zero_is_invalid() {
        assert!(!NotificationId::new(0).is_valid());
    }

    #[test]
    fn test_negative_is_invalid() {
        assert!(!NotificationId::new(-1).is_valid());
        assert!(!NotificationId::new(-100).is_valid());
        assert!(!NotificationId::new(i32::MIN).is_valid());
    }

    #[test]
    fn test_positive_is_valid() {
        assert!(NotificationId::new(1).is_valid());
        assert!(NotificationId::new(100).is_valid());
        assert!(NotificationId::new(i32::MAX).is_valid());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let id = NotificationId::new(12345);
        let serialized = serde_json::to_string(&id).unwrap();
        let deserialized: NotificationId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(id, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_json() {
        let id = NotificationId::new(999);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "999");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_json() {
        let json = "12345";
        let id: NotificationId = serde_json::from_str(json).unwrap();
        assert_eq!(id.get(), 12345);
    }

    #[test]
    fn test_notification_id_in_hashmap() {
        use std::collections::HashMap;

        let mut map: HashMap<NotificationId, String> = HashMap::new();

        map.insert(NotificationId::new(1), "first".to_string());
        map.insert(NotificationId::new(2), "second".to_string());

        assert_eq!(map.get(&NotificationId::new(1)), Some(&"first".to_string()));
        assert_eq!(map.get(&NotificationId::new(2)), Some(&"second".to_string()));
        assert_eq!(map.get(&NotificationId::new(3)), None);
    }

    #[test]
    fn test_ord_total_ordering() {
        let ids = vec![
            NotificationId::new(30),
            NotificationId::new(10),
            NotificationId::new(20),
        ];

        let mut sorted = ids.clone();
        sorted.sort();

        assert_eq!(sorted[0].get(), 10);
        assert_eq!(sorted[1].get(), 20);
        assert_eq!(sorted[2].get(), 30);
    }
}
