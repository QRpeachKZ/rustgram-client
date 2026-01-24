// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Notification group ID type for Telegram MTProto client.
//!
//! This module implements TDLib's NotificationGroupId class.
//!
//! # Example
//!
//! ```rust
//! use rustgram_notification_group_id::NotificationGroupId;
//!
//! let id = NotificationGroupId::new(42);
//! assert!(id.is_valid());
//! assert_eq!(id.get(), 42);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::fmt::{self, Display, Formatter};
use std::hash::{Hash, Hasher};

/// Notification group identifier.
///
/// Based on TDLib's `NotificationGroupId` class.
///
/// Valid notification group IDs are positive integers (greater than 0).
/// This is used to uniquely identify notification groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NotificationGroupId(i32);

impl NotificationGroupId {
    /// Creates a new NotificationGroupId.
    ///
    /// # Arguments
    ///
    /// * `id` - The group ID value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_id::NotificationGroupId;
    ///
    /// let id = NotificationGroupId::new(123);
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
    /// use rustgram_notification_group_id::NotificationGroupId;
    ///
    /// let id = NotificationGroupId::new(456);
    /// assert_eq!(id.get(), 456);
    /// ```
    #[inline]
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Checks if this is a valid notification group ID.
    ///
    /// Valid notification group IDs are positive (greater than 0).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_id::NotificationGroupId;
    ///
    /// assert!(NotificationGroupId::new(1).is_valid());
    /// assert!(NotificationGroupId::new(100).is_valid());
    /// assert!(!NotificationGroupId::new(0).is_valid());
    /// assert!(!NotificationGroupId::new(-1).is_valid());
    /// ```
    #[inline]
    pub const fn is_valid(self) -> bool {
        self.0 > 0
    }
}

impl Default for NotificationGroupId {
    fn default() -> Self {
        Self(0)
    }
}

impl Hash for NotificationGroupId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Display for NotificationGroupId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "notification group {}", self.0)
    }
}

impl From<i32> for NotificationGroupId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<NotificationGroupId> for i32 {
    fn from(id: NotificationGroupId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = NotificationGroupId::new(123);
        assert_eq!(id.get(), 123);
    }

    #[test]
    fn test_get() {
        assert_eq!(NotificationGroupId::new(1).get(), 1);
        assert_eq!(NotificationGroupId::new(100).get(), 100);
        assert_eq!(NotificationGroupId::new(0).get(), 0);
        assert_eq!(NotificationGroupId::new(-50).get(), -50);
    }

    #[test]
    fn test_is_valid() {
        assert!(NotificationGroupId::new(1).is_valid());
        assert!(NotificationGroupId::new(100).is_valid());
        assert!(NotificationGroupId::new(i32::MAX).is_valid());
        assert!(!NotificationGroupId::new(0).is_valid());
        assert!(!NotificationGroupId::new(-1).is_valid());
        assert!(!NotificationGroupId::new(-100).is_valid());
    }

    #[test]
    fn test_default() {
        let id = NotificationGroupId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_equality() {
        let id1 = NotificationGroupId::new(123);
        let id2 = NotificationGroupId::new(123);
        let id3 = NotificationGroupId::new(456);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_ordering() {
        let id1 = NotificationGroupId::new(100);
        let id2 = NotificationGroupId::new(200);
        let id3 = NotificationGroupId::new(150);

        assert!(id1 < id2);
        assert!(id1 < id3);
        assert!(id3 < id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_copy() {
        let id1 = NotificationGroupId::new(789);
        let id2 = id1;
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_clone() {
        let id1 = NotificationGroupId::new(999);
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(NotificationGroupId::new(1));
        set.insert(NotificationGroupId::new(2));
        set.insert(NotificationGroupId::new(3));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", NotificationGroupId::new(42)), "notification group 42");
    }

    #[test]
    fn test_from_i32() {
        let id: NotificationGroupId = 456.into();
        assert_eq!(id.get(), 456);
    }

    #[test]
    fn test_into_i32() {
        let id = NotificationGroupId::new(789);
        let value: i32 = id.into();
        assert_eq!(value, 789);
    }

    #[test]
    fn test_zero_is_invalid() {
        assert!(!NotificationGroupId::new(0).is_valid());
    }

    #[test]
    fn test_negative_is_invalid() {
        assert!(!NotificationGroupId::new(-1).is_valid());
        assert!(!NotificationGroupId::new(-100).is_valid());
    }

    #[test]
    fn test_positive_is_valid() {
        assert!(NotificationGroupId::new(1).is_valid());
        assert!(NotificationGroupId::new(100).is_valid());
    }
}
