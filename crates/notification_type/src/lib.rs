// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Notification type stub for Telegram MTProto client.
//!
//! This module implements a stub for TDLib's NotificationType.
//!
//! # TODO
//!
//! This is a simplified stub implementation. The full TDLib NotificationType
//! is a polymorphic base class with multiple subclasses for different
//! notification types (new message, new call, new secret chat, etc.).
//!
//! The full implementation should include:
//! - NewMessage (with message_id and show_preview)
//! - NewSecretChat
//! - NewCall (with call_id)
//! - NewPushMessage (with sender info, message_id, etc.)
//!
//! # Example
//!
//! ```rust
//! use rustgram_notification_type::NotificationType;
//!
//! let ty = NotificationType::default();
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::fmt::{self, Display, Formatter};

/// Notification type stub.
///
/// Based on TDLib's `NotificationType` class.
///
/// # TODO
///
/// This is a simplified stub. The full TDLib implementation includes:
/// - `notificationTypeNewMessage`: New message notification
/// - `notificationTypeNewSecretChat`: New secret chat notification
/// - `notificationTypeNewCall`: New call notification
/// - `notificationTypePushMessage`: Push message notification
///
/// Each type has specific fields and behavior. This stub provides a minimal
/// placeholder for development.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum NotificationType {
    /// New message notification.
    ///
    /// TODO: Add fields for message_id and show_preview.
    NewMessage,

    /// New secret chat notification.
    NewSecretChat,

    /// New call notification.
    ///
    /// TODO: Add field for call_id.
    NewCall,

    /// Push message notification.
    ///
    /// TODO: Add fields for sender_user_id, sender_dialog_id, sender_name,
    /// is_outgoing, message_id, key, arg, photo, document.
    NewPushMessage,

    /// Unknown notification type.
    #[default]
    Unknown,
}

impl Display for NotificationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NewMessage => write!(f, "new message"),
            Self::NewSecretChat => write!(f, "new secret chat"),
            Self::NewCall => write!(f, "new call"),
            Self::NewPushMessage => write!(f, "new push message"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl NotificationType {
    /// Creates a new message notification type.
    ///
    /// # TODO
    ///
    /// This should accept message_id and show_preview parameters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_type::NotificationType;
    ///
    /// let ty = NotificationType::new_message();
    /// assert!(matches!(ty, NotificationType::NewMessage));
    /// ```
    pub fn new_message() -> Self {
        Self::NewMessage
    }

    /// Creates a new secret chat notification type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_type::NotificationType;
    ///
    /// let ty = NotificationType::new_secret_chat();
    /// assert!(matches!(ty, NotificationType::NewSecretChat));
    /// ```
    pub fn new_secret_chat() -> Self {
        Self::NewSecretChat
    }

    /// Creates a new call notification type.
    ///
    /// # TODO
    ///
    /// This should accept a call_id parameter.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_type::NotificationType;
    ///
    /// let ty = NotificationType::new_call();
    /// assert!(matches!(ty, NotificationType::NewCall));
    /// ```
    pub fn new_call() -> Self {
        Self::NewCall
    }

    /// Creates a new push message notification type.
    ///
    /// # TODO
    ///
    /// This should accept sender_user_id, sender_dialog_id, sender_name,
    /// is_outgoing, message_id, key, arg, photo, and document parameters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_type::NotificationType;
    ///
    /// let ty = NotificationType::new_push_message();
    /// assert!(matches!(ty, NotificationType::NewPushMessage));
    /// ```
    pub fn new_push_message() -> Self {
        Self::NewPushMessage
    }

    /// Checks if this notification can be delayed.
    ///
    /// # TODO
    ///
    /// Implement proper logic based on notification type.
    /// In TDLib, most notifications can be delayed except some specific types.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_type::NotificationType;
    ///
    /// let ty = NotificationType::new_message();
    /// assert!(ty.can_be_delayed());
    /// ```
    pub fn can_be_delayed(&self) -> bool {
        // TODO: Implement proper logic
        match self {
            Self::Unknown => false,
            _ => true,
        }
    }

    /// Checks if this is a temporary notification.
    ///
    /// # TODO
    ///
    /// Implement proper logic based on notification type.
    /// In TDLib, some notifications are temporary (e.g., uploading files).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_type::NotificationType;
    ///
    /// let ty = NotificationType::new_message();
    /// assert!(!ty.is_temporary());
    /// ```
    pub fn is_temporary(&self) -> bool {
        // TODO: Implement proper logic
        false
    }

    /// Checks if this is a new message notification.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_type::NotificationType;
    ///
    /// let ty = NotificationType::new_message();
    /// assert!(ty.is_new_message());
    /// ```
    pub fn is_new_message(&self) -> bool {
        matches!(self, Self::NewMessage)
    }

    /// Checks if this is a new secret chat notification.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_type::NotificationType;
    ///
    /// let ty = NotificationType::new_secret_chat();
    /// assert!(ty.is_new_secret_chat());
    /// ```
    pub fn is_new_secret_chat(&self) -> bool {
        matches!(self, Self::NewSecretChat)
    }

    /// Checks if this is a new call notification.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_type::NotificationType;
    ///
    /// let ty = NotificationType::new_call();
    /// assert!(ty.is_new_call());
    /// ```
    pub fn is_new_call(&self) -> bool {
        matches!(self, Self::NewCall)
    }

    /// Checks if this is a new push message notification.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_type::NotificationType;
    ///
    /// let ty = NotificationType::new_push_message();
    /// assert!(ty.is_new_push_message());
    /// ```
    pub fn is_new_push_message(&self) -> bool {
        matches!(self, Self::NewPushMessage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_message() {
        let ty = NotificationType::new_message();
        assert!(matches!(ty, NotificationType::NewMessage));
        assert!(ty.is_new_message());
        assert!(!ty.is_new_secret_chat());
        assert!(!ty.is_new_call());
        assert!(!ty.is_new_push_message());
    }

    #[test]
    fn test_new_secret_chat() {
        let ty = NotificationType::new_secret_chat();
        assert!(matches!(ty, NotificationType::NewSecretChat));
        assert!(!ty.is_new_message());
        assert!(ty.is_new_secret_chat());
        assert!(!ty.is_new_call());
        assert!(!ty.is_new_push_message());
    }

    #[test]
    fn test_new_call() {
        let ty = NotificationType::new_call();
        assert!(matches!(ty, NotificationType::NewCall));
        assert!(!ty.is_new_message());
        assert!(!ty.is_new_secret_chat());
        assert!(ty.is_new_call());
        assert!(!ty.is_new_push_message());
    }

    #[test]
    fn test_new_push_message() {
        let ty = NotificationType::new_push_message();
        assert!(matches!(ty, NotificationType::NewPushMessage));
        assert!(!ty.is_new_message());
        assert!(!ty.is_new_secret_chat());
        assert!(!ty.is_new_call());
        assert!(ty.is_new_push_message());
    }

    #[test]
    fn test_default() {
        let ty = NotificationType::default();
        assert!(matches!(ty, NotificationType::Unknown));
    }

    #[test]
    fn test_can_be_delayed() {
        assert!(NotificationType::new_message().can_be_delayed());
        assert!(NotificationType::new_secret_chat().can_be_delayed());
        assert!(NotificationType::new_call().can_be_delayed());
        assert!(NotificationType::new_push_message().can_be_delayed());
        assert!(!NotificationType::Unknown.can_be_delayed());
    }

    #[test]
    fn test_is_temporary() {
        assert!(!NotificationType::new_message().is_temporary());
        assert!(!NotificationType::new_secret_chat().is_temporary());
        assert!(!NotificationType::new_call().is_temporary());
        assert!(!NotificationType::new_push_message().is_temporary());
        assert!(!NotificationType::Unknown.is_temporary());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", NotificationType::NewMessage), "new message");
        assert_eq!(
            format!("{}", NotificationType::NewSecretChat),
            "new secret chat"
        );
        assert_eq!(format!("{}", NotificationType::NewCall), "new call");
        assert_eq!(
            format!("{}", NotificationType::NewPushMessage),
            "new push message"
        );
        assert_eq!(format!("{}", NotificationType::Unknown), "unknown");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", NotificationType::NewMessage), "NewMessage");
        assert_eq!(
            format!("{:?}", NotificationType::NewSecretChat),
            "NewSecretChat"
        );
        assert_eq!(format!("{:?}", NotificationType::NewCall), "NewCall");
        assert_eq!(
            format!("{:?}", NotificationType::NewPushMessage),
            "NewPushMessage"
        );
        assert_eq!(format!("{:?}", NotificationType::Unknown), "Unknown");
    }

    #[test]
    fn test_equality() {
        assert_eq!(NotificationType::NewMessage, NotificationType::NewMessage);
        assert_eq!(
            NotificationType::NewSecretChat,
            NotificationType::NewSecretChat
        );
        assert_eq!(NotificationType::NewCall, NotificationType::NewCall);
        assert_eq!(
            NotificationType::NewPushMessage,
            NotificationType::NewPushMessage
        );
        assert_eq!(NotificationType::Unknown, NotificationType::Unknown);
    }

    #[test]
    fn test_inequality() {
        assert_ne!(NotificationType::NewMessage, NotificationType::NewSecretChat);
        assert_ne!(NotificationType::NewSecretChat, NotificationType::NewCall);
        assert_ne!(NotificationType::NewCall, NotificationType::NewPushMessage);
        assert_ne!(NotificationType::NewPushMessage, NotificationType::Unknown);
    }

    #[test]
    fn test_copy() {
        let ty1 = NotificationType::NewMessage;
        let ty2 = ty1;
        assert_eq!(ty1, ty2);
    }

    #[test]
    fn test_clone() {
        let ty1 = NotificationType::NewCall;
        let ty2 = ty1.clone();
        assert_eq!(ty1, ty2);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(NotificationType::NewMessage);
        set.insert(NotificationType::NewSecretChat);
        set.insert(NotificationType::NewCall);
        set.insert(NotificationType::NewPushMessage);
        set.insert(NotificationType::Unknown);
        assert_eq!(set.len(), 5);
    }

    #[test]
    fn test_all_types_distinct() {
        let types = [
            NotificationType::NewMessage,
            NotificationType::NewSecretChat,
            NotificationType::NewCall,
            NotificationType::NewPushMessage,
            NotificationType::Unknown,
        ];

        for i in 0..types.len() {
            for j in (i + 1)..types.len() {
                assert_ne!(types[i], types[j]);
            }
        }
    }
}
