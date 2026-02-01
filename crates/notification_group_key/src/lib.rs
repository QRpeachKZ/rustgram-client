// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Notification group key type for Telegram MTProto client.
//!
//! This module implements TDLib's NotificationGroupKey struct.
//!
//! # Example
//!
//! ```rust
//! use rustgram_notification_group_key::NotificationGroupKey;
//! use rustgram_notification_group_id::NotificationGroupId;
//! use rustgram_dialog_id::DialogId;
//!
//! let key = NotificationGroupKey::new(
//!     NotificationGroupId::new(1),
//!     DialogId::new(100),
//!     1234567890
//! );
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_dialog_id::DialogId;
use rustgram_notification_group_id::NotificationGroupId;
use std::cmp::{Ord, Ordering};
use std::fmt::{self, Display, Formatter};

/// Notification group key.
///
/// Based on TDLib's `NotificationGroupKey` struct.
///
/// Used as a key for sorting notification groups. The ordering is
/// primarily by last_notification_date (descending), then by dialog_id
/// (descending), then by group_id (descending).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationGroupKey {
    /// Notification group ID.
    pub group_id: NotificationGroupId,

    /// Dialog ID.
    pub dialog_id: DialogId,

    /// Date of the last notification in the group.
    pub last_notification_date: i32,
}

impl NotificationGroupKey {
    /// Creates a new NotificationGroupKey.
    ///
    /// # Arguments
    ///
    /// * `group_id` - Notification group ID
    /// * `dialog_id` - Dialog ID
    /// * `last_notification_date` - Date of the last notification
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_key::NotificationGroupKey;
    /// use rustgram_notification_group_id::NotificationGroupId;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let key = NotificationGroupKey::new(
    ///     NotificationGroupId::new(1),
    ///     DialogId::new(100),
    ///     1234567890
    /// );
    /// ```
    pub fn new(group_id: NotificationGroupId, dialog_id: DialogId, last_notification_date: i32) -> Self {
        Self {
            group_id,
            dialog_id,
            last_notification_date,
        }
    }
}

impl Default for NotificationGroupKey {
    fn default() -> Self {
        Self {
            group_id: NotificationGroupId::default(),
            dialog_id: DialogId::default(),
            last_notification_date: 0,
        }
    }
}

impl Ord for NotificationGroupKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // TDLib ordering: descending by date, then by dialog_id, then by group_id
        match other.last_notification_date.cmp(&self.last_notification_date) {
            Ordering::Equal => {
                // Compare by dialog_id (descending)
                // DialogId.get() returns the inner i64 value
                let self_value = self.dialog_id.get();
                let other_value = other.dialog_id.get();
                match other_value.cmp(&self_value) {
                    Ordering::Equal => {
                        // Compare by group_id (descending)
                        other.group_id.cmp(&self.group_id)
                    }
                    ord => ord,
                }
            }
            ord => ord,
        }
    }
}

impl PartialOrd for NotificationGroupKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for NotificationGroupKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[group={}, dialog={}, date={}]",
            self.group_id.get(),
            self.dialog_id,
            self.last_notification_date
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let key = NotificationGroupKey::new(
            NotificationGroupId::new(1),
            DialogId::new(100),
            1234567890,
        );
        assert_eq!(key.group_id.get(), 1);
        assert_eq!(key.last_notification_date, 1234567890);
    }

    #[test]
    fn test_default() {
        let key = NotificationGroupKey::default();
        assert_eq!(key.group_id.get(), 0);
        assert_eq!(key.last_notification_date, 0);
    }

    #[test]
    fn test_ordering_by_date() {
        let key1 = NotificationGroupKey::new(
            NotificationGroupId::new(1),
            DialogId::new(100),
            100,
        );
        let key2 = NotificationGroupKey::new(
            NotificationGroupId::new(1),
            DialogId::new(100),
            200,
        );

        // Higher date comes first (descending order)
        assert!(key2 < key1);
    }

    #[test]
    fn test_ordering_by_dialog_id() {
        let key1 = NotificationGroupKey::new(
            NotificationGroupId::new(1),
            DialogId::new(100),
            100,
        );
        let key2 = NotificationGroupKey::new(
            NotificationGroupId::new(1),
            DialogId::new(200),
            100,
        );

        // Higher dialog_id comes first (descending order)
        assert!(key2 < key1);
    }

    #[test]
    fn test_ordering_by_group_id() {
        let key1 = NotificationGroupKey::new(
            NotificationGroupId::new(1),
            DialogId::new(100),
            100,
        );
        let key2 = NotificationGroupKey::new(
            NotificationGroupId::new(2),
            DialogId::new(100),
            100,
        );

        // Higher group_id comes first (descending order)
        assert!(key2 < key1);
    }

    #[test]
    fn test_equality() {
        let key1 = NotificationGroupKey::new(
            NotificationGroupId::new(1),
            DialogId::new(100),
            123,
        );
        let key2 = NotificationGroupKey::new(
            NotificationGroupId::new(1),
            DialogId::new(100),
            123,
        );
        let key3 = NotificationGroupKey::new(
            NotificationGroupId::new(2),
            DialogId::new(100),
            123,
        );

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_clone() {
        let key1 = NotificationGroupKey::new(
            NotificationGroupId::new(1),
            DialogId::new(100),
            123,
        );
        let key2 = key1.clone();
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_display() {
        let key = NotificationGroupKey::new(
            NotificationGroupId::new(42),
            DialogId::new(100),
            1234567890,
        );
        let display = format!("{}", key);
        assert!(display.contains("42"));
        assert!(display.contains("1234567890"));
    }

    #[test]
    fn test_sort_ordering() {
        let mut keys = vec![
            NotificationGroupKey::new(
                NotificationGroupId::new(1),
                DialogId::new(100),
                100,
            ),
            NotificationGroupKey::new(
                NotificationGroupId::new(2),
                DialogId::new(100),
                200,
            ),
            NotificationGroupKey::new(
                NotificationGroupId::new(3),
                DialogId::new(100),
                150,
            ),
        ];

        keys.sort();

        // Should be sorted by date descending: 200, 150, 100
        assert_eq!(keys[0].last_notification_date, 200);
        assert_eq!(keys[1].last_notification_date, 150);
        assert_eq!(keys[2].last_notification_date, 100);
    }
}
