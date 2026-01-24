// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Notification group info type for Telegram MTProto client.
//!
//! This module implements TDLib's NotificationGroupInfo class.
//!
//! # Example
//!
//! ```rust
//! use rustgram_notification_group_info::NotificationGroupInfo;
//! use rustgram_notification_group_id::NotificationGroupId;
//!
//! let info = NotificationGroupInfo::new(NotificationGroupId::new(1));
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_notification_group_id::NotificationGroupId;
use rustgram_notification_id::NotificationId;
use rustgram_notification_object_id::NotificationObjectId;
use std::fmt::{self, Display, Formatter};

/// Notification group info.
///
/// Based on TDLib's `NotificationGroupInfo` class.
///
/// This is a simplified version containing the essential fields for tracking
/// notification group state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationGroupInfo {
    /// Notification group ID.
    group_id: NotificationGroupId,

    /// Date of the last notification in the group.
    last_notification_date: i32,

    /// ID of the last notification in the group.
    last_notification_id: NotificationId,

    /// Notification ID up to which all notifications are removed.
    max_removed_notification_id: NotificationId,

    /// Object ID up to which all notifications are removed.
    max_removed_object_id: NotificationObjectId,

    /// Whether the group key has changed.
    is_key_changed: bool,

    /// Whether the group should be deleted from database and reused.
    try_reuse: bool,
}

impl NotificationGroupInfo {
    /// Creates a new NotificationGroupInfo.
    ///
    /// # Arguments
    ///
    /// * `group_id` - Notification group ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_info::NotificationGroupInfo;
    /// use rustgram_notification_group_id::NotificationGroupId;
    ///
    /// let info = NotificationGroupInfo::new(NotificationGroupId::new(1));
    /// ```
    pub fn new(group_id: NotificationGroupId) -> Self {
        Self {
            group_id,
            last_notification_date: 0,
            last_notification_id: NotificationId::default(),
            max_removed_notification_id: NotificationId::default(),
            max_removed_object_id: NotificationObjectId::default(),
            is_key_changed: true,
            try_reuse: false,
        }
    }

    /// Returns the group ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_info::NotificationGroupInfo;
    /// use rustgram_notification_group_id::NotificationGroupId;
    ///
    /// let info = NotificationGroupInfo::new(NotificationGroupId::new(42));
    /// assert_eq!(info.group_id().get(), 42);
    /// ```
    pub fn group_id(&self) -> NotificationGroupId {
        self.group_id
    }

    /// Returns the last notification date.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_info::NotificationGroupInfo;
    ///
    /// let info = NotificationGroupInfo::new(Default::default());
    /// assert_eq!(info.last_notification_date(), 0);
    /// ```
    pub fn last_notification_date(&self) -> i32 {
        self.last_notification_date
    }

    /// Returns the last notification ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_info::NotificationGroupInfo;
    ///
    /// let info = NotificationGroupInfo::new(Default::default());
    /// assert_eq!(info.last_notification_id().get(), 0);
    /// ```
    pub fn last_notification_id(&self) -> NotificationId {
        self.last_notification_id
    }

    /// Returns the max removed notification ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_info::NotificationGroupInfo;
    ///
    /// let info = NotificationGroupInfo::new(Default::default());
    /// assert_eq!(info.max_removed_notification_id().get(), 0);
    /// ```
    pub fn max_removed_notification_id(&self) -> NotificationId {
        self.max_removed_notification_id
    }

    /// Returns the max removed object ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_info::NotificationGroupInfo;
    ///
    /// let info = NotificationGroupInfo::new(Default::default());
    /// assert_eq!(info.max_removed_object_id().get(), 0);
    /// ```
    pub fn max_removed_object_id(&self) -> NotificationObjectId {
        self.max_removed_object_id
    }

    /// Checks if the group ID is valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_info::NotificationGroupInfo;
    /// use rustgram_notification_group_id::NotificationGroupId;
    ///
    /// let info = NotificationGroupInfo::new(NotificationGroupId::new(1));
    /// assert!(info.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.group_id.is_valid()
    }

    /// Checks if this is an active group.
    ///
    /// A group is active if it's valid and not marked for reuse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_info::NotificationGroupInfo;
    /// use rustgram_notification_group_id::NotificationGroupId;
    ///
    /// let info = NotificationGroupInfo::new(NotificationGroupId::new(1));
    /// assert!(info.is_active());
    /// ```
    pub fn is_active(&self) -> bool {
        self.is_valid() && !self.try_reuse
    }

    /// Checks if this has a specific group ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_info::NotificationGroupInfo;
    /// use rustgram_notification_group_id::NotificationGroupId;
    ///
    /// let info = NotificationGroupInfo::new(NotificationGroupId::new(42));
    /// assert!(info.has_group_id(NotificationGroupId::new(42)));
    /// assert!(!info.has_group_id(NotificationGroupId::new(99)));
    /// ```
    pub fn has_group_id(&self, group_id: NotificationGroupId) -> bool {
        self.group_id == group_id
    }

    /// Sets the last notification info.
    ///
    /// # Arguments
    ///
    /// * `date` - Notification date
    /// * `notification_id` - Notification ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_info::NotificationGroupInfo;
    /// use rustgram_notification_id::NotificationId;
    ///
    /// let mut info = NotificationGroupInfo::new(Default::default());
    /// info.set_last_notification(1234567890, NotificationId::new(5));
    /// assert_eq!(info.last_notification_date(), 1234567890);
    /// ```
    pub fn set_last_notification(&mut self, date: i32, notification_id: NotificationId) {
        self.last_notification_date = date;
        self.last_notification_id = notification_id;
    }

    /// Sets the max removed notification ID.
    ///
    /// # Arguments
    ///
    /// * `notification_id` - Max removed notification ID
    /// * `object_id` - Max removed object ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_info::NotificationGroupInfo;
    /// use rustgram_notification_id::NotificationId;
    /// use rustgram_notification_object_id::NotificationObjectId;
    ///
    /// let mut info = NotificationGroupInfo::new(Default::default());
    /// info.set_max_removed_notification_id(
    ///     NotificationId::new(10),
    ///     NotificationObjectId::new(100)
    /// );
    /// ```
    pub fn set_max_removed_notification_id(
        &mut self,
        notification_id: NotificationId,
        object_id: NotificationObjectId,
    ) {
        self.max_removed_notification_id = notification_id;
        self.max_removed_object_id = object_id;
    }

    /// Marks this group for reuse.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_info::NotificationGroupInfo;
    ///
    /// let mut info = NotificationGroupInfo::new(Default::default());
    /// info.try_reuse();
    /// assert!(!info.is_active());
    /// ```
    pub fn try_reuse(&mut self) {
        self.try_reuse = true;
    }
}

impl Default for NotificationGroupInfo {
    fn default() -> Self {
        Self {
            group_id: NotificationGroupId::default(),
            last_notification_date: 0,
            last_notification_id: NotificationId::default(),
            max_removed_notification_id: NotificationId::default(),
            max_removed_object_id: NotificationObjectId::default(),
            is_key_changed: false,
            try_reuse: false,
        }
    }
}

impl Display for NotificationGroupInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NotificationGroupInfo[group_id={}, date={}, active={}]",
            self.group_id.get(),
            self.last_notification_date,
            self.is_active()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let info = NotificationGroupInfo::new(NotificationGroupId::new(1));
        assert_eq!(info.group_id().get(), 1);
        assert!(info.is_valid());
        assert!(info.is_active());
        assert!(info.is_key_changed);
    }

    #[test]
    fn test_default() {
        let info = NotificationGroupInfo::default();
        assert!(!info.is_valid());
        assert!(!info.is_active());
    }

    #[test]
    fn test_is_valid() {
        let valid_info = NotificationGroupInfo::new(NotificationGroupId::new(1));
        assert!(valid_info.is_valid());

        let invalid_info = NotificationGroupInfo::new(NotificationGroupId::new(0));
        assert!(!invalid_info.is_valid());
    }

    #[test]
    fn test_is_active() {
        let mut info = NotificationGroupInfo::new(NotificationGroupId::new(1));
        assert!(info.is_active());

        info.try_reuse();
        assert!(!info.is_active());
    }

    #[test]
    fn test_has_group_id() {
        let info = NotificationGroupInfo::new(NotificationGroupId::new(42));
        assert!(info.has_group_id(NotificationGroupId::new(42)));
        assert!(!info.has_group_id(NotificationGroupId::new(99)));
    }

    #[test]
    fn test_set_last_notification() {
        let mut info = NotificationGroupInfo::new(NotificationGroupId::new(1));
        info.set_last_notification(1234567890, NotificationId::new(5));
        assert_eq!(info.last_notification_date(), 1234567890);
        assert_eq!(info.last_notification_id().get(), 5);
    }

    #[test]
    fn test_set_max_removed_notification_id() {
        let mut info = NotificationGroupInfo::new(NotificationGroupId::new(1));
        info.set_max_removed_notification_id(
            NotificationId::new(10),
            NotificationObjectId::new(100),
        );
        assert_eq!(info.max_removed_notification_id().get(), 10);
        assert_eq!(info.max_removed_object_id().get(), 100);
    }

    #[test]
    fn test_try_reuse() {
        let mut info = NotificationGroupInfo::new(NotificationGroupId::new(1));
        assert!(info.is_active());
        info.try_reuse();
        assert!(!info.is_active());
        assert!(info.try_reuse);
    }

    #[test]
    fn test_clone() {
        let info1 = NotificationGroupInfo::new(NotificationGroupId::new(1));
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_equality() {
        let info1 = NotificationGroupInfo::new(NotificationGroupId::new(1));
        let info2 = NotificationGroupInfo::new(NotificationGroupId::new(1));
        let info3 = NotificationGroupInfo::new(NotificationGroupId::new(2));

        assert_eq!(info1, info2);
        assert_ne!(info1, info3);
    }

    #[test]
    fn test_display() {
        let info = NotificationGroupInfo::new(NotificationGroupId::new(42));
        let display = format!("{}", info);
        assert!(display.contains("42"));
        assert!(display.contains("active=true"));
    }

    #[test]
    fn test_with_invalid_group_id() {
        let info = NotificationGroupInfo::new(NotificationGroupId::new(0));
        assert!(!info.is_valid());
        assert!(!info.is_active());
    }

    #[test]
    fn test_last_notification_defaults() {
        let info = NotificationGroupInfo::new(NotificationGroupId::new(1));
        assert_eq!(info.last_notification_date(), 0);
        assert_eq!(info.last_notification_id().get(), 0);
    }

    #[test]
    fn test_max_removed_defaults() {
        let info = NotificationGroupInfo::new(NotificationGroupId::new(1));
        assert_eq!(info.max_removed_notification_id().get(), 0);
        assert_eq!(info.max_removed_object_id().get(), 0);
    }
}
