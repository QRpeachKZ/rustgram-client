// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Reaction Notification Settings
//!
//! Settings for reaction notifications.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_reaction_notifications_from::ReactionNotificationsFrom;
use serde::{Deserialize, Serialize};

/// Reaction notification settings.
///
/// Based on TDLib's `ReactionNotificationSettings` class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactionNotificationSettings {
    /// Message reactions notification source.
    message_reactions: ReactionNotificationsFrom,
    /// Story reactions notification source.
    story_reactions: ReactionNotificationsFrom,
    /// Whether to show preview.
    show_preview: bool,
}

impl Default for ReactionNotificationSettings {
    fn default() -> Self {
        Self {
            message_reactions: ReactionNotificationsFrom::Contacts,
            story_reactions: ReactionNotificationsFrom::Contacts,
            show_preview: true,
        }
    }
}

impl ReactionNotificationSettings {
    /// Creates new reaction notification settings.
    #[must_use]
    pub const fn new(
        message_reactions: ReactionNotificationsFrom,
        story_reactions: ReactionNotificationsFrom,
        show_preview: bool,
    ) -> Self {
        Self {
            message_reactions,
            story_reactions,
            show_preview,
        }
    }

    /// Returns the message reactions notification source.
    #[must_use]
    pub const fn message_reactions(&self) -> ReactionNotificationsFrom {
        self.message_reactions
    }

    /// Returns the story reactions notification source.
    #[must_use]
    pub const fn story_reactions(&self) -> ReactionNotificationsFrom {
        self.story_reactions
    }

    /// Returns whether to show preview.
    #[must_use]
    pub const fn show_preview(&self) -> bool {
        self.show_preview
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let settings = ReactionNotificationSettings::default();
        assert!(settings.show_preview());
    }

    #[test]
    fn test_new() {
        let settings = ReactionNotificationSettings::new(
            ReactionNotificationsFrom::All,
            ReactionNotificationsFrom::Contacts,
            false,
        );
        assert!(!settings.show_preview());
        assert_eq!(settings.message_reactions(), ReactionNotificationsFrom::All);
    }
}
