// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Notification group from database type for Telegram MTProto client.
//!
//! This module implements TDLib's NotificationGroupFromDatabase struct.
//!
//! # Example
//!
//! ```rust
//! use rustgram_notification_group_from_database::NotificationGroupFromDatabase;
//! use rustgram_dialog_id::DialogId;
//! use rustgram_notification_group_type::NotificationGroupType;
//!
//! let group = NotificationGroupFromDatabase::new(
//!     DialogId::from_user(rustgram_types::UserId(100)),
//!     NotificationGroupType::Messages
//! );
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_dialog_id::DialogId;
use rustgram_notification::Notification;
use rustgram_notification_group_type::NotificationGroupType;
use std::fmt::{self, Display, Formatter};

/// Notification group from database.
///
/// Based on TDLib's `NotificationGroupFromDatabase` struct.
///
/// Contains information about a notification group loaded from the database,
/// including the dialog ID, group type, total count, and notifications.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationGroupFromDatabase {
    /// Dialog ID.
    pub dialog_id: DialogId,

    /// Notification group type.
    pub ty: NotificationGroupType,

    /// Total count of notifications in the group.
    pub total_count: i32,

    /// Notifications in the group.
    pub notifications: Vec<Notification>,
}

impl NotificationGroupFromDatabase {
    /// Creates a new NotificationGroupFromDatabase.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `ty` - Notification group type
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_from_database::NotificationGroupFromDatabase;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_notification_group_type::NotificationGroupType;
    /// 
    ///
    /// let group = NotificationGroupFromDatabase::new(
    ///     DialogId::new(100),
    ///     NotificationGroupType::Messages
    /// );
    /// ```
    pub fn new(dialog_id: DialogId, ty: NotificationGroupType) -> Self {
        Self {
            dialog_id,
            ty,
            total_count: 0,
            notifications: Vec::new(),
        }
    }

    /// Returns the total count of notifications.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_from_database::NotificationGroupFromDatabase;
    ///
    /// let group = NotificationGroupFromDatabase::new(Default::default(), Default::default());
    /// assert_eq!(group.total_count(), 0);
    /// ```
    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    /// Returns the notifications in the group.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_from_database::NotificationGroupFromDatabase;
    ///
    /// let group = NotificationGroupFromDatabase::new(Default::default(), Default::default());
    /// assert!(group.notifications().is_empty());
    /// ```
    pub fn notifications(&self) -> &[Notification] {
        &self.notifications
    }

    /// Sets the total count of notifications.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_from_database::NotificationGroupFromDatabase;
    ///
    /// let mut group = NotificationGroupFromDatabase::new(Default::default(), Default::default());
    /// group.set_total_count(42);
    /// assert_eq!(group.total_count(), 42);
    /// ```
    pub fn set_total_count(&mut self, total_count: i32) {
        self.total_count = total_count;
    }

    /// Adds a notification to the group.
    ///
    /// # Arguments
    ///
    /// * `notification` - Notification to add
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_from_database::NotificationGroupFromDatabase;
    /// use rustgram_notification::Notification;
    ///
    /// let mut group = NotificationGroupFromDatabase::new(Default::default(), Default::default());
    /// group.add_notification(Notification::default());
    /// assert_eq!(group.notifications().len(), 1);
    /// ```
    pub fn add_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }

    /// Sets the notifications for the group.
    ///
    /// # Arguments
    ///
    /// * `notifications` - Vector of notifications
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_from_database::NotificationGroupFromDatabase;
    /// use rustgram_notification::Notification;
    ///
    /// let mut group = NotificationGroupFromDatabase::new(Default::default(), Default::default());
    /// let notifications = vec![Notification::default(), Notification::default()];
    /// group.set_notifications(notifications);
    /// assert_eq!(group.notifications().len(), 2);
    /// ```
    pub fn set_notifications(&mut self, notifications: Vec<Notification>) {
        self.notifications = notifications;
    }

    /// Clears all notifications from the group.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_from_database::NotificationGroupFromDatabase;
    /// use rustgram_notification::Notification;
    ///
    /// let mut group = NotificationGroupFromDatabase::new(Default::default(), Default::default());
    /// group.add_notification(Notification::default());
    /// group.clear_notifications();
    /// assert!(group.notifications().is_empty());
    /// ```
    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    /// Returns the actual count of notifications in the group.
    ///
    /// This is the length of the notifications vector.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_group_from_database::NotificationGroupFromDatabase;
    /// use rustgram_notification::Notification;
    ///
    /// let mut group = NotificationGroupFromDatabase::new(Default::default(), Default::default());
    /// group.add_notification(Notification::default());
    /// group.add_notification(Notification::default());
    /// assert_eq!(group.actual_count(), 2);
    /// ```
    pub fn actual_count(&self) -> usize {
        self.notifications.len()
    }
}

impl Default for NotificationGroupFromDatabase {
    fn default() -> Self {
        Self {
            dialog_id: DialogId::default(),
            ty: NotificationGroupType::default(),
            total_count: 0,
            notifications: Vec::new(),
        }
    }
}

impl Display for NotificationGroupFromDatabase {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NotificationGroupFromDatabase[dialog={}, type={}, total={}, actual={}]",
            self.dialog_id,
            self.ty,
            self.total_count,
            self.actual_count()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_notification::{Notification, NotificationSettings};
    use rustgram_notification_id::NotificationId;
    use rustgram_notification_type::NotificationType;
    use rustgram_types::UserId;

    #[test]
    fn test_new() {
        let group = NotificationGroupFromDatabase::new(
            DialogId::new(100),
            NotificationGroupType::Messages,
        );
        assert_eq!(group.total_count(), 0);
        assert!(group.notifications().is_empty());
    }

    #[test]
    fn test_default() {
        let group = NotificationGroupFromDatabase::default();
        assert_eq!(group.total_count(), 0);
        assert!(group.notifications().is_empty());
    }

    #[test]
    fn test_total_count() {
        let group = NotificationGroupFromDatabase::new(
            DialogId::default(),
            NotificationGroupType::Messages,
        );
        assert_eq!(group.total_count(), 0);
    }

    #[test]
    fn test_set_total_count() {
        let mut group = NotificationGroupFromDatabase::new(
            DialogId::default(),
            NotificationGroupType::Messages,
        );
        group.set_total_count(42);
        assert_eq!(group.total_count(), 42);
    }

    #[test]
    fn test_add_notification() {
        let mut group = NotificationGroupFromDatabase::new(
            DialogId::default(),
            NotificationGroupType::Messages,
        );
        assert_eq!(group.actual_count(), 0);

        group.add_notification(Notification::default());
        assert_eq!(group.actual_count(), 1);

        group.add_notification(Notification::default());
        assert_eq!(group.actual_count(), 2);
    }

    #[test]
    fn test_set_notifications() {
        let mut group = NotificationGroupFromDatabase::new(
            DialogId::default(),
            NotificationGroupType::Messages,
        );
        let notifications = vec![
            Notification::default(),
            Notification::default(),
            Notification::default(),
        ];
        group.set_notifications(notifications);
        assert_eq!(group.actual_count(), 3);
    }

    #[test]
    fn test_clear_notifications() {
        let mut group = NotificationGroupFromDatabase::new(
            DialogId::default(),
            NotificationGroupType::Messages,
        );
        group.add_notification(Notification::default());
        group.add_notification(Notification::default());
        assert_eq!(group.actual_count(), 2);

        group.clear_notifications();
        assert_eq!(group.actual_count(), 0);
    }

    #[test]
    fn test_actual_count() {
        let mut group = NotificationGroupFromDatabase::new(
            DialogId::default(),
            NotificationGroupType::Messages,
        );
        assert_eq!(group.actual_count(), 0);

        group.add_notification(Notification::default());
        assert_eq!(group.actual_count(), 1);

        group.set_notifications(vec![
            Notification::default(),
            Notification::default(),
            Notification::default(),
        ]);
        assert_eq!(group.actual_count(), 3);
    }

    #[test]
    fn test_total_count_vs_actual_count() {
        let mut group = NotificationGroupFromDatabase::new(
            DialogId::default(),
            NotificationGroupType::Messages,
        );
        group.set_total_count(10);
        group.add_notification(Notification::default());

        assert_eq!(group.total_count(), 10);
        assert_eq!(group.actual_count(), 1);
    }

    #[test]
    fn test_equality() {
        let group1 = NotificationGroupFromDatabase::new(
            DialogId::new(100),
            NotificationGroupType::Messages,
        );
        let group2 = NotificationGroupFromDatabase::new(
            DialogId::new(100),
            NotificationGroupType::Messages,
        );
        let group3 = NotificationGroupFromDatabase::new(
            DialogId::new(200),
            NotificationGroupType::Messages,
        );

        assert_eq!(group1, group2);
        assert_ne!(group1, group3);
    }

    #[test]
    fn test_clone() {
        let mut group1 = NotificationGroupFromDatabase::new(
            DialogId::new(100),
            NotificationGroupType::Calls,
        );
        group1.add_notification(Notification::default());

        let group2 = group1.clone();
        assert_eq!(group1, group2);
        assert_eq!(group2.actual_count(), 1);
    }

    #[test]
    fn test_display() {
        let mut group = NotificationGroupFromDatabase::new(
            DialogId::new(100),
            NotificationGroupType::Mentions,
        );
        group.set_total_count(5);
        group.add_notification(Notification::default());

        let display = format!("{}", group);
        assert!(display.contains("total=5"));
        assert!(display.contains("actual=1"));
    }

    #[test]
    fn test_all_group_types() {
        let types = [
            NotificationGroupType::Messages,
            NotificationGroupType::Mentions,
            NotificationGroupType::SecretChat,
            NotificationGroupType::Calls,
        ];

        for ty in types {
            let group = NotificationGroupFromDatabase::new(DialogId::default(), ty);
            assert_eq!(group.ty, ty);
        }
    }

    #[test]
    fn test_with_full_notification() {
        let mut group = NotificationGroupFromDatabase::new(
            DialogId::new(100),
            NotificationGroupType::Messages,
        );

        let notification = Notification::new(
            NotificationId::new(1),
            1234567890,
            false,
            NotificationType::new_message(),
            NotificationSettings::default(),
        );

        group.add_notification(notification);
        assert_eq!(group.actual_count(), 1);
        assert_eq!(group.notifications()[0].notification_id().get(), 1);
    }

    #[test]
    fn test_notifications_immutability() {
        let mut group = NotificationGroupFromDatabase::new(
            DialogId::default(),
            NotificationGroupType::Messages,
        );
        group.add_notification(Notification::default());

        // Getting notifications as a slice should allow reading but not modifying directly
        let notifications = group.notifications();
        assert_eq!(notifications.len(), 1);
    }
}
