// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0;

//! # Forum Topic Full ID
//!
//! Composite identifier for forum topics combining DialogId and ForumTopicId.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `ForumTopicFullId` struct from
//! `td/telegram/ForumTopicFullId.h`.
//!
//! ## Structure
//!
//! - `dialog_id`: The dialog (channel) ID
//! - `forum_topic_id`: The forum topic ID

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use rustgram_dialog_id::DialogId;
use rustgram_forum_topic_id::ForumTopicId;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Full identifier for a forum topic.
///
/// Combines a dialog ID with a forum topic ID to uniquely identify
/// a topic within a specific forum (channel).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForumTopicFullId {
    dialog_id: DialogId,
    forum_topic_id: ForumTopicId,
}

impl ForumTopicFullId {
    /// Creates a new ForumTopicFullId.
    #[must_use]
    pub fn new(dialog_id: DialogId, forum_topic_id: ForumTopicId) -> Self {
        Self {
            dialog_id,
            forum_topic_id,
        }
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
}

impl Default for ForumTopicFullId {
    fn default() -> Self {
        Self {
            dialog_id: DialogId::default(),
            forum_topic_id: ForumTopicId::default(),
        }
    }
}

impl Hash for ForumTopicFullId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dialog_id.hash(state);
        self.forum_topic_id.hash(state);
    }
}

impl fmt::Display for ForumTopicFullId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} in {}", self.forum_topic_id, self.dialog_id)
    }
}

/// Hasher for ForumTopicFullId.
#[derive(Debug, Clone, Copy, Default)]
pub struct ForumTopicFullIdHash;

impl ForumTopicFullIdHash {
    /// Hashes a ForumTopicFullId value.
    #[must_use]
    pub fn hash(&self, id: ForumTopicFullId) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        id.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dialog_id = DialogId::default();
        let topic_id = ForumTopicId::new(100);
        let full_id = ForumTopicFullId::new(dialog_id, topic_id);
        assert_eq!(full_id.forum_topic_id().get(), 100);
    }

    #[test]
    fn test_default() {
        let full_id = ForumTopicFullId::default();
        assert!(!full_id.forum_topic_id().is_valid());
    }

    #[test]
    fn test_dialog_id() {
        let dialog_id = DialogId::default();
        let topic_id = ForumTopicId::new(100);
        let full_id = ForumTopicFullId::new(dialog_id, topic_id);
        assert_eq!(full_id.dialog_id(), dialog_id);
    }

    #[test]
    fn test_forum_topic_id() {
        let dialog_id = DialogId::default();
        let topic_id = ForumTopicId::new(100);
        let full_id = ForumTopicFullId::new(dialog_id, topic_id);
        assert_eq!(full_id.forum_topic_id(), topic_id);
    }

    #[test]
    fn test_equality() {
        let dialog_id = DialogId::default();
        let topic_id = ForumTopicId::new(100);
        let f1 = ForumTopicFullId::new(dialog_id, topic_id);
        let f2 = ForumTopicFullId::new(dialog_id, topic_id);
        assert_eq!(f1, f2);
    }

    #[test]
    fn test_inequality() {
        let dialog_id = DialogId::default();
        let t1 = ForumTopicId::new(100);
        let t2 = ForumTopicId::new(200);
        let f1 = ForumTopicFullId::new(dialog_id, t1);
        let f2 = ForumTopicFullId::new(dialog_id, t2);
        assert_ne!(f1, f2);
    }

    #[test]
    fn test_display() {
        let dialog_id = DialogId::default();
        let topic_id = ForumTopicId::new(100);
        let full_id = ForumTopicFullId::new(dialog_id, topic_id);
        let display = format!("{}", full_id);
        assert!(display.contains("topic"));
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        let dialog_id = DialogId::default();
        let topic_id = ForumTopicId::new(100);
        let f1 = ForumTopicFullId::new(dialog_id, topic_id);
        let f2 = ForumTopicFullId::new(dialog_id, topic_id);
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        f1.hash(&mut h1);
        f2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }
}
