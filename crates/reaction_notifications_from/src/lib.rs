// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Reaction Notifications From
//!
//! Source of reaction notifications.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use serde::{Deserialize, Serialize};

/// Source of reaction notifications.
///
/// Based on TDLib's `ReactionNotificationsFrom` class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReactionNotificationsFrom {
    /// No notifications.
    None,
    /// Notifications from contacts only.
    Contacts,
    /// Notifications from all users.
    All,
}

impl Default for ReactionNotificationsFrom {
    fn default() -> Self {
        Self::Contacts
    }
}

impl ReactionNotificationsFrom {
    /// Checks if this is None (no notifications).
    #[must_use]
    pub const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }

    /// Checks if this is Contacts.
    #[must_use]
    pub const fn is_contacts(self) -> bool {
        matches!(self, Self::Contacts)
    }

    /// Checks if this is All.
    #[must_use]
    pub const fn is_all(self) -> bool {
        matches!(self, Self::All)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let from = ReactionNotificationsFrom::default();
        assert!(from.is_contacts());
    }

    #[test]
    fn test_is_none() {
        assert!(ReactionNotificationsFrom::None.is_none());
    }

    #[test]
    fn test_is_contacts() {
        assert!(ReactionNotificationsFrom::Contacts.is_contacts());
    }

    #[test]
    fn test_is_all() {
        assert!(ReactionNotificationsFrom::All.is_all());
    }
}
