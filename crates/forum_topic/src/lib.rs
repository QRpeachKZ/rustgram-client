// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0;

//! # Forum Topic
//!
//! Represents a forum topic with state tracking.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `ForumTopic` class from
//! `td/telegram/ForumTopic.h`.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use rustgram_dialog_id::DialogId;
use rustgram_dialog_notification_settings::DialogNotificationSettings;
use rustgram_draft_message::DraftMessage;
use rustgram_forum_topic_info::ForumTopicInfo;
use rustgram_types::MessageId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Forum topic with read state and draft tracking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForumTopic {
    is_short: bool,
    is_pinned: bool,
    unread_count: i32,
    last_message_id: MessageId,
    last_read_inbox_message_id: MessageId,
    last_read_outbox_message_id: MessageId,
    unread_mention_count: i32,
    unread_reaction_count: i32,
    notification_settings: DialogNotificationSettings,
    draft_message: Option<DraftMessage>,
}

impl ForumTopic {
    /// Creates a new ForumTopic.
    #[must_use]
    pub fn new() -> Self {
        Self {
            is_short: false,
            is_pinned: false,
            unread_count: 0,
            last_message_id: MessageId::default(),
            last_read_inbox_message_id: MessageId::default(),
            last_read_outbox_message_id: MessageId::default(),
            unread_mention_count: 0,
            unread_reaction_count: 0,
            notification_settings: DialogNotificationSettings::new(),
            draft_message: None,
        }
    }

    /// Returns `true` if this is a short topic info.
    #[must_use]
    pub const fn is_short(&self) -> bool {
        self.is_short
    }

    /// Returns `true` if the topic is pinned.
    #[must_use]
    pub const fn is_pinned(&self) -> bool {
        self.is_pinned
    }

    /// Returns the unread message count.
    #[must_use]
    pub const fn unread_count(&self) -> i32 {
        self.unread_count
    }

    /// Returns the last message ID.
    #[must_use]
    pub const fn last_message_id(&self) -> MessageId {
        self.last_message_id
    }

    /// Returns the last read inbox message ID.
    #[must_use]
    pub const fn last_read_inbox_message_id(&self) -> MessageId {
        self.last_read_inbox_message_id
    }

    /// Returns the last read outbox message ID.
    #[must_use]
    pub const fn last_read_outbox_message_id(&self) -> MessageId {
        self.last_read_outbox_message_id
    }

    /// Returns the notification settings.
    #[must_use]
    pub const fn notification_settings(&self) -> &DialogNotificationSettings {
        &self.notification_settings
    }

    /// Returns the draft message if present.
    #[must_use]
    pub const fn draft_message(&self) -> Option<&DraftMessage> {
        self.draft_message.as_ref()
    }
}

impl Default for ForumTopic {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ForumTopic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ForumTopic(unread={}, pinned={})",
            self.unread_count, self.is_pinned
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let topic = ForumTopic::new();
        assert!(!topic.is_short());
        assert!(!topic.is_pinned());
        assert_eq!(topic.unread_count(), 0);
    }

    #[test]
    fn test_default() {
        let topic = ForumTopic::default();
        assert_eq!(topic.unread_count(), 0);
    }

    #[test]
    fn test_is_short() {
        let topic = ForumTopic::new();
        assert!(!topic.is_short());
    }

    #[test]
    fn test_is_pinned() {
        let topic = ForumTopic::new();
        assert!(!topic.is_pinned());
    }

    #[test]
    fn test_unread_count() {
        let topic = ForumTopic::new();
        assert_eq!(topic.unread_count(), 0);
    }

    #[test]
    fn test_last_message_id() {
        let topic = ForumTopic::new();
        assert_eq!(topic.last_message_id(), MessageId::default());
    }

    #[test]
    fn test_notification_settings() {
        let topic = ForumTopic::new();
        let settings = topic.notification_settings();
        assert_eq!(settings.mute_for(), 0);
    }

    #[test]
    fn test_draft_message_none() {
        let topic = ForumTopic::new();
        assert!(topic.draft_message().is_none());
    }

    #[test]
    fn test_display() {
        let topic = ForumTopic::new();
        let display = format!("{}", topic);
        assert!(display.contains("ForumTopic"));
    }
}
