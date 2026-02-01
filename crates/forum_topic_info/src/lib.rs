// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0;

//! # Forum Topic Info
//!
//! Information about a forum topic.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `ForumTopicInfo` class from
//! `td/telegram/ForumTopicInfo.h`.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use rustgram_dialog_id::DialogId;
use rustgram_forum_topic_icon::ForumTopicIcon;
use rustgram_forum_topic_id::ForumTopicId;
use std::fmt;

/// Information about a forum topic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForumTopicInfo {
    dialog_id: DialogId,
    forum_topic_id: ForumTopicId,
    title: String,
    icon: ForumTopicIcon,
    creation_date: i32,
    creator_dialog_id: DialogId,
    is_outgoing: bool,
    is_closed: bool,
    is_hidden: bool,
    is_title_missing: bool,
}

impl ForumTopicInfo {
    /// Creates a new ForumTopicInfo.
    #[must_use]
    pub fn new(
        dialog_id: DialogId,
        forum_topic_id: ForumTopicId,
        title: String,
        icon: ForumTopicIcon,
        creation_date: i32,
        creator_dialog_id: DialogId,
        is_outgoing: bool,
        is_closed: bool,
        is_hidden: bool,
        is_title_missing: bool,
    ) -> Self {
        Self {
            dialog_id,
            forum_topic_id,
            title,
            icon,
            creation_date,
            creator_dialog_id,
            is_outgoing,
            is_closed,
            is_hidden,
            is_title_missing,
        }
    }

    /// Returns `true` if the topic info is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.forum_topic_id.is_valid()
    }

    /// Returns the dialog ID.
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the forum topic ID.
    #[must_use]
    pub const fn forum_topic_id(&self) -> ForumTopicId {
        self.forum_topic_id
    }

    /// Returns the creator dialog ID.
    #[must_use]
    pub const fn creator_dialog_id(&self) -> DialogId {
        self.creator_dialog_id
    }

    /// Returns `true` if this is an outgoing topic.
    #[must_use]
    pub const fn is_outgoing(&self) -> bool {
        self.is_outgoing
    }

    /// Returns `true` if the topic is closed.
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        self.is_closed
    }

    /// Returns `true` if the topic is hidden.
    #[must_use]
    pub const fn is_hidden(&self) -> bool {
        self.is_hidden
    }
}

impl Default for ForumTopicInfo {
    fn default() -> Self {
        Self {
            dialog_id: DialogId::default(),
            forum_topic_id: ForumTopicId::default(),
            title: String::new(),
            icon: ForumTopicIcon::default(),
            creation_date: 0,
            creator_dialog_id: DialogId::default(),
            is_outgoing: false,
            is_closed: false,
            is_hidden: false,
            is_title_missing: false,
        }
    }
}

impl fmt::Display for ForumTopicInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ForumTopicInfo({})", self.title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let info = ForumTopicInfo::new(
            DialogId::default(),
            ForumTopicId::new(100),
            "Test Topic".to_string(),
            ForumTopicIcon::new(),
            1234567890,
            DialogId::default(),
            false,
            false,
            false,
            false,
        );
        assert_eq!(info.forum_topic_id().get(), 100);
        assert_eq!(info.title, "Test Topic");
    }

    #[test]
    fn test_default() {
        let info = ForumTopicInfo::default();
        assert!(info.is_empty());
    }

    #[test]
    fn test_is_empty_true() {
        let info = ForumTopicInfo::default();
        assert!(info.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let info = ForumTopicInfo::new(
            DialogId::default(),
            ForumTopicId::new(100),
            "Test".to_string(),
            ForumTopicIcon::new(),
            0,
            DialogId::default(),
            false,
            false,
            false,
            false,
        );
        assert!(!info.is_empty());
    }

    #[test]
    fn test_is_closed() {
        let mut info = ForumTopicInfo::new(
            DialogId::default(),
            ForumTopicId::new(100),
            "Test".to_string(),
            ForumTopicIcon::new(),
            0,
            DialogId::default(),
            false,
            true,
            false,
            false,
        );
        assert!(info.is_closed());
    }

    #[test]
    fn test_is_outgoing() {
        let info = ForumTopicInfo::new(
            DialogId::default(),
            ForumTopicId::new(100),
            "Test".to_string(),
            ForumTopicIcon::new(),
            0,
            DialogId::default(),
            true,
            false,
            false,
            false,
        );
        assert!(info.is_outgoing());
    }

    #[test]
    fn test_is_hidden() {
        let info = ForumTopicInfo::new(
            DialogId::default(),
            ForumTopicId::new(100),
            "Test".to_string(),
            ForumTopicIcon::new(),
            0,
            DialogId::default(),
            false,
            false,
            true,
            false,
        );
        assert!(info.is_hidden());
    }
}
