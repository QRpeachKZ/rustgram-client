// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Notification type for Telegram MTProto client.
//!
//! This module implements TDLib's Notification class.
//!
//! # Example
//!
//! ```rust
//! use rustgram_notification::{Notification, NotificationSettings};
//! use rustgram_notification_id::NotificationId;
//! use rustgram_notification_type::NotificationType;
//!
//! let notification = Notification::new(
//!     NotificationId::new(1),
//!     1234567890,
//!     false,
//!     NotificationType::new_message(),
//!     NotificationSettings::default()
//! );
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_notification_id::NotificationId;
use rustgram_notification_type::NotificationType;
use std::fmt::{self, Display, Formatter};

/// Notification settings.
///
/// Additional settings for notification behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotificationSettings {
    /// Whether the notification should be disabled.
    pub disable_notification: bool,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            disable_notification: false,
        }
    }
}

/// Notification.
///
/// Based on TDLib's `Notification` class.
///
/// Represents a single notification with its ID, date, type, and settings.
///
/// # Example
///
/// ```rust
/// use rustgram_notification::{Notification, NotificationSettings};
/// use rustgram_notification_id::NotificationId;
/// use rustgram_notification_type::NotificationType;
///
/// let notification = Notification::new(
///     NotificationId::new(1),
///     1234567890,
///     false,
///     NotificationType::new_message(),
///     NotificationSettings::default()
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    /// Notification ID.
    notification_id: NotificationId,

    /// Notification date (Unix timestamp).
    date: i32,

    /// Notification settings.
    settings: NotificationSettings,

    /// Notification type.
    ty: NotificationType,
}

impl Notification {
    /// Creates a new Notification.
    ///
    /// # Arguments
    ///
    /// * `notification_id` - Notification ID
    /// * `date` - Notification date (Unix timestamp)
    /// * `disable_notification` - Whether the notification is disabled
    /// * `ty` - Notification type
    /// * `settings` - Notification settings
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification::{Notification, NotificationSettings};
    /// use rustgram_notification_id::NotificationId;
    /// use rustgram_notification_type::NotificationType;
    ///
    /// let notification = Notification::new(
    ///     NotificationId::new(1),
    ///     1234567890,
    ///     false,
    ///     NotificationType::new_message(),
    ///     NotificationSettings::default()
    /// );
    /// ```
    pub fn new(
        notification_id: NotificationId,
        date: i32,
        disable_notification: bool,
        ty: NotificationType,
        settings: NotificationSettings,
    ) -> Self {
        Self {
            notification_id,
            date,
            settings: NotificationSettings { disable_notification },
            ty,
        }
    }

    /// Returns the notification ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification::Notification;
    /// use rustgram_notification_id::NotificationId;
    /// use rustgram_notification_type::NotificationType;
    ///
    /// let notification = Notification::new(
    ///     NotificationId::new(42),
    ///     0,
    ///     false,
    ///     NotificationType::new_message(),
    ///     Default::default()
    /// );
    /// assert_eq!(notification.notification_id().get(), 42);
    /// ```
    pub fn notification_id(&self) -> NotificationId {
        self.notification_id
    }

    /// Returns the notification date.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification::Notification;
    ///
    /// let notification = Notification::new(
    ///     Default::default(),
    ///     1234567890,
    ///     false,
    ///     Default::default(),
    ///     Default::default()
    /// );
    /// assert_eq!(notification.date(), 1234567890);
    /// ```
    pub fn date(&self) -> i32 {
        self.date
    }

    /// Checks if the notification is disabled.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification::Notification;
    ///
    /// let notification = Notification::new(
    ///     Default::default(),
    ///     0,
    ///     true,
    ///     Default::default(),
    ///     Default::default()
    /// );
    /// assert!(notification.is_disabled());
    /// ```
    pub fn is_disabled(&self) -> bool {
        self.settings.disable_notification
    }

    /// Returns the notification type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification::Notification;
    /// use rustgram_notification_type::NotificationType;
    ///
    /// let notification = Notification::new(
    ///     Default::default(),
    ///     0,
    ///     false,
    ///     NotificationType::new_message(),
    ///     Default::default()
    /// );
    /// assert!(notification.ty().is_new_message());
    /// ```
    pub fn ty(&self) -> &NotificationType {
        &self.ty
    }

    /// Returns the notification settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification::Notification;
    ///
    /// let notification = Notification::default();
    /// let settings = notification.settings();
    /// ```
    pub fn settings(&self) -> NotificationSettings {
        self.settings
    }

    /// Sets the notification date.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification::Notification;
    ///
    /// let mut notification = Notification::default();
    /// notification.set_date(987654321);
    /// assert_eq!(notification.date(), 987654321);
    /// ```
    pub fn set_date(&mut self, date: i32) {
        self.date = date;
    }

    /// Sets whether the notification is disabled.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification::Notification;
    ///
    /// let mut notification = Notification::default();
    /// notification.set_disabled(true);
    /// assert!(notification.is_disabled());
    /// ```
    pub fn set_disabled(&mut self, disable_notification: bool) {
        self.settings.disable_notification = disable_notification;
    }

    /// Sets the notification type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification::Notification;
    /// use rustgram_notification_type::NotificationType;
    ///
    /// let mut notification = Notification::default();
    /// notification.set_type(NotificationType::new_call());
    /// assert!(notification.ty().is_new_call());
    /// ```
    pub fn set_type(&mut self, ty: NotificationType) {
        self.ty = ty;
    }
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            notification_id: NotificationId::default(),
            date: 0,
            settings: NotificationSettings::default(),
            ty: NotificationType::default(),
        }
    }
}

impl Display for Notification {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Notification[id={}, date={}, type={}, disabled={}]",
            self.notification_id.get(),
            self.date,
            self.ty,
            self.settings.disable_notification
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let notification = Notification::new(
            NotificationId::new(1),
            1234567890,
            true,
            NotificationType::new_message(),
            NotificationSettings::default(),
        );
        assert_eq!(notification.notification_id().get(), 1);
        assert_eq!(notification.date(), 1234567890);
        assert!(notification.is_disabled());
        assert!(notification.ty().is_new_message());
    }

    #[test]
    fn test_default() {
        let notification = Notification::default();
        assert_eq!(notification.notification_id().get(), 0);
        assert_eq!(notification.date(), 0);
        assert!(!notification.is_disabled());
    }

    #[test]
    fn test_notification_id() {
        let notification = Notification::new(
            NotificationId::new(42),
            0,
            false,
            NotificationType::default(),
            NotificationSettings::default(),
        );
        assert_eq!(notification.notification_id().get(), 42);
    }

    #[test]
    fn test_date() {
        let notification = Notification::new(
            NotificationId::default(),
            999999,
            false,
            NotificationType::default(),
            NotificationSettings::default(),
        );
        assert_eq!(notification.date(), 999999);
    }

    #[test]
    fn test_is_disabled() {
        let notification1 = Notification::new(
            NotificationId::default(),
            0,
            true,
            NotificationType::default(),
            NotificationSettings::default(),
        );
        assert!(notification1.is_disabled());

        let notification2 = Notification::new(
            NotificationId::default(),
            0,
            false,
            NotificationType::default(),
            NotificationSettings::default(),
        );
        assert!(!notification2.is_disabled());
    }

    #[test]
    fn test_ty() {
        let notification = Notification::new(
            NotificationId::default(),
            0,
            false,
            NotificationType::new_call(),
            NotificationSettings::default(),
        );
        assert!(notification.ty().is_new_call());
    }

    #[test]
    fn test_set_date() {
        let mut notification = Notification::default();
        notification.set_date(111111);
        assert_eq!(notification.date(), 111111);
    }

    #[test]
    fn test_set_disabled() {
        let mut notification = Notification::default();
        assert!(!notification.is_disabled());
        notification.set_disabled(true);
        assert!(notification.is_disabled());
        notification.set_disabled(false);
        assert!(!notification.is_disabled());
    }

    #[test]
    fn test_set_type() {
        let mut notification = Notification::default();
        notification.set_type(NotificationType::new_secret_chat());
        assert!(notification.ty().is_new_secret_chat());
        notification.set_type(NotificationType::new_call());
        assert!(notification.ty().is_new_call());
    }

    #[test]
    fn test_equality() {
        let notification1 = Notification::new(
            NotificationId::new(1),
            100,
            false,
            NotificationType::new_message(),
            NotificationSettings::default(),
        );
        let notification2 = Notification::new(
            NotificationId::new(1),
            100,
            false,
            NotificationType::new_message(),
            NotificationSettings::default(),
        );
        let notification3 = Notification::new(
            NotificationId::new(2),
            100,
            false,
            NotificationType::new_message(),
            NotificationSettings::default(),
        );

        assert_eq!(notification1, notification2);
        assert_ne!(notification1, notification3);
    }

    #[test]
    fn test_clone() {
        let notification1 = Notification::new(
            NotificationId::new(1),
            12345,
            true,
            NotificationType::new_call(),
            NotificationSettings::default(),
        );
        let notification2 = notification1.clone();
        assert_eq!(notification1, notification2);
    }

    #[test]
    fn test_display() {
        let notification = Notification::new(
            NotificationId::new(42),
            1234567890,
            true,
            NotificationType::new_message(),
            NotificationSettings::default(),
        );
        let display = format!("{}", notification);
        assert!(display.contains("42"));
        assert!(display.contains("1234567890"));
        assert!(display.contains("disabled=true"));
    }

    #[test]
    fn test_notification_settings_default() {
        let settings = NotificationSettings::default();
        assert!(!settings.disable_notification);
    }

    #[test]
    fn test_notification_settings_fields() {
        let settings = NotificationSettings {
            disable_notification: true,
        };
        assert!(settings.disable_notification);
    }

    #[test]
    fn test_all_notification_types() {
        let types = [
            NotificationType::new_message(),
            NotificationType::new_secret_chat(),
            NotificationType::new_call(),
            NotificationType::new_push_message(),
        ];

        for ty in types {
            let notification = Notification::new(
                NotificationId::new(1),
                0,
                false,
                ty.clone(),
                NotificationSettings::default(),
            );
            assert_eq!(notification.ty(), &ty);
        }
    }

    #[test]
    fn test_notification_with_zero_date() {
        let notification = Notification::new(
            NotificationId::new(1),
            0,
            false,
            NotificationType::default(),
            NotificationSettings::default(),
        );
        assert_eq!(notification.date(), 0);
    }

    #[test]
    fn test_notification_with_negative_date() {
        let notification = Notification::new(
            NotificationId::new(1),
            -100,
            false,
            NotificationType::default(),
            NotificationSettings::default(),
        );
        assert_eq!(notification.date(), -100);
    }
}
