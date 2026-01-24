// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Story Full ID
//!
//! Full story identifier for Telegram.
//!
//! ## Overview
//!
//! This module provides the [`StoryFullId`] type, which uniquely identifies
//! a story by combining a dialog ID and a story ID.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_story_full_id::StoryFullId;
//! use rustgram_dialog_id::DialogId;
//! use rustgram_story_id::StoryId;
//!
//! let story_id = StoryId::new(123);
//! let dialog_id = DialogId::new(456);
//! let full_id = StoryFullId::new(dialog_id, story_id);
//! assert_eq!(full_id.dialog_id().get(), 456);
//! assert_eq!(full_id.story_id().get(), 123);
//! ```

use std::fmt;
use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use rustgram_dialog_id::DialogId;
use rustgram_story_id::StoryId;

/// Full story identifier.
///
/// Combines a dialog ID and a story ID to uniquely identify a story
/// within a specific dialog.
///
/// Based on TDLib's `StoryFullId` struct.
///
/// # TDLib Alignment
///
/// Aligns with TDLib's `StoryFullId` struct in `StoryFullId.h`.
///
/// # Example
///
/// ```rust
/// use rustgram_story_full_id::StoryFullId;
/// use rustgram_dialog_id::DialogId;
/// use rustgram_story_id::StoryId;
///
/// let story_id = StoryId::new(123);
/// let dialog_id = DialogId::new(456);
/// let full_id = StoryFullId::new(dialog_id, story_id);
/// assert_eq!(full_id.dialog_id().get(), 456);
/// assert_eq!(full_id.story_id().get(), 123);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoryFullId {
    /// The dialog containing this story.
    dialog_id: DialogId,

    /// The story identifier.
    story_id: StoryId,
}

impl StoryFullId {
    /// Creates a new full story ID.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog containing this story
    /// * `story_id` - The story identifier
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_full_id::StoryFullId;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_story_id::StoryId;
    ///
    /// let dialog_id = DialogId::new(456);
    /// let story_id = StoryId::new(123);
    /// let full_id = StoryFullId::new(dialog_id, story_id);
    /// ```
    #[must_use]
    pub const fn new(dialog_id: DialogId, story_id: StoryId) -> Self {
        Self {
            dialog_id,
            story_id,
        }
    }

    /// Returns the dialog ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_full_id::StoryFullId;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_story_id::StoryId;
    ///
    /// let full_id = StoryFullId::new(DialogId::new(456), StoryId::new(123));
    /// assert_eq!(full_id.dialog_id().get(), 456);
    /// ```
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the story ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_full_id::StoryFullId;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_story_id::StoryId;
    ///
    /// let full_id = StoryFullId::new(DialogId::new(456), StoryId::new(123));
    /// assert_eq!(full_id.story_id().get(), 123);
    /// ```
    #[must_use]
    pub const fn story_id(&self) -> StoryId {
        self.story_id
    }

    /// Checks if this is a valid full story ID.
    ///
    /// Both the dialog ID and story ID must be valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_full_id::StoryFullId;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_story_id::StoryId;
    ///
    /// let valid = StoryFullId::new(DialogId::new(456), StoryId::new(123));
    /// assert!(valid.is_valid());
    ///
    /// let invalid = StoryFullId::new(DialogId::new(0), StoryId::new(123));
    /// assert!(!invalid.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.dialog_id.is_valid() && self.story_id.is_valid()
    }

    /// Checks if this is a server story.
    ///
    /// Both the dialog ID and story ID must be valid,
    /// and the story ID must be a server story ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_full_id::StoryFullId;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_story_id::StoryId;
    ///
    /// let server_story = StoryFullId::new(DialogId::new(456), StoryId::new(123));
    /// assert!(server_story.is_server());
    /// ```
    #[must_use]
    pub fn is_server(&self) -> bool {
        self.dialog_id.is_valid() && self.story_id.is_server()
    }
}

impl fmt::Display for StoryFullId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "story {} in {}",
            self.story_id.get(),
            self.dialog_id.get()
        )
    }
}

impl Hash for StoryFullId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dialog_id.hash(state);
        self.story_id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== new Tests ==========

    #[test]
    fn test_new() {
        let dialog_id = DialogId::new(456);
        let story_id = StoryId::new(123);
        let full_id = StoryFullId::new(dialog_id, story_id);
        assert_eq!(full_id.dialog_id().get(), 456);
        assert_eq!(full_id.story_id().get(), 123);
    }

    // ========== dialog_id Tests ==========

    #[test]
    fn test_dialog_id() {
        let full_id = StoryFullId::new(DialogId::new(456), StoryId::new(123));
        assert_eq!(full_id.dialog_id().get(), 456);
    }

    // ========== story_id Tests ==========

    #[test]
    fn test_story_id() {
        let full_id = StoryFullId::new(DialogId::new(456), StoryId::new(123));
        assert_eq!(full_id.story_id().get(), 123);
    }

    // ========== is_valid Tests ==========

    #[test]
    fn test_is_valid_true() {
        let full_id = StoryFullId::new(DialogId::new(456), StoryId::new(123));
        assert!(full_id.is_valid());
    }

    #[test]
    fn test_is_valid_invalid_dialog() {
        let full_id = StoryFullId::new(DialogId::new(0), StoryId::new(123));
        assert!(!full_id.is_valid());
    }

    #[test]
    fn test_is_valid_invalid_story() {
        let full_id = StoryFullId::new(DialogId::new(456), StoryId::new(0));
        assert!(!full_id.is_valid());
    }

    #[test]
    fn test_is_valid_both_invalid() {
        let full_id = StoryFullId::new(DialogId::new(0), StoryId::new(0));
        assert!(!full_id.is_valid());
    }

    // ========== is_server Tests ==========

    #[test]
    fn test_is_server_true() {
        let full_id = StoryFullId::new(DialogId::new(456), StoryId::new(123));
        assert!(full_id.is_server());
    }

    #[test]
    fn test_is_server_invalid_dialog() {
        let full_id = StoryFullId::new(DialogId::new(0), StoryId::new(123));
        assert!(!full_id.is_server());
    }

    #[test]
    fn test_is_server_local_story() {
        use rustgram_story_id::MAX_SERVER_STORY_ID;
        let full_id = StoryFullId::new(DialogId::new(456), StoryId::new(MAX_SERVER_STORY_ID + 1));
        assert!(!full_id.is_server());
    }

    // ========== default Tests ==========

    #[test]
    fn test_default() {
        let full_id = StoryFullId::default();
        assert!(!full_id.is_valid());
        assert!(!full_id.is_server());
    }

    // ========== equality Tests ==========

    #[test]
    fn test_equality_same() {
        let full_id1 = StoryFullId::new(DialogId::new(456), StoryId::new(123));
        let full_id2 = StoryFullId::new(DialogId::new(456), StoryId::new(123));
        assert_eq!(full_id1, full_id2);
    }

    #[test]
    fn test_equality_different_dialog() {
        let full_id1 = StoryFullId::new(DialogId::new(456), StoryId::new(123));
        let full_id2 = StoryFullId::new(DialogId::new(789), StoryId::new(123));
        assert_ne!(full_id1, full_id2);
    }

    #[test]
    fn test_equality_different_story() {
        let full_id1 = StoryFullId::new(DialogId::new(456), StoryId::new(123));
        let full_id2 = StoryFullId::new(DialogId::new(456), StoryId::new(789));
        assert_ne!(full_id1, full_id2);
    }

    // ========== clone Tests ==========

    #[test]
    fn test_clone() {
        let full_id1 = StoryFullId::new(DialogId::new(456), StoryId::new(123));
        let full_id2 = full_id1;
        assert_eq!(full_id1, full_id2);
    }

    // ========== hash Tests ==========

    #[test]
    fn test_hash_same() {
        use std::collections::hash_map::DefaultHasher;

        let full_id1 = StoryFullId::new(DialogId::new(456), StoryId::new(123));
        let full_id2 = StoryFullId::new(DialogId::new(456), StoryId::new(123));

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        full_id1.hash(&mut hasher1);
        full_id2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_hash_different() {
        use std::collections::hash_map::DefaultHasher;

        let full_id1 = StoryFullId::new(DialogId::new(456), StoryId::new(123));
        let full_id2 = StoryFullId::new(DialogId::new(789), StoryId::new(123));

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        full_id1.hash(&mut hasher1);
        full_id2.hash(&mut hasher2);

        assert_ne!(hasher1.finish(), hasher2.finish());
    }

    // ========== display Tests ==========

    #[test]
    fn test_display() {
        let full_id = StoryFullId::new(DialogId::new(456), StoryId::new(123));
        let display = format!("{}", full_id);
        assert!(display.contains("123"));
        assert!(display.contains("456"));
    }

    #[test]
    fn test_debug() {
        let full_id = StoryFullId::new(DialogId::new(456), StoryId::new(123));
        let debug_str = format!("{:?}", full_id);
        assert!(debug_str.contains("StoryFullId"));
    }

    // ========== Serialization Tests ==========

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let full_id = StoryFullId::new(DialogId::new(456), StoryId::new(123));
        let json = serde_json::to_string(&full_id).unwrap();
        let deserialized: StoryFullId = serde_json::from_str(&json).unwrap();
        assert_eq!(full_id, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_all_values() {
        let values = [
            (DialogId::new(0), StoryId::new(0)),
            (DialogId::new(1), StoryId::new(1)),
            (DialogId::new(456), StoryId::new(123)),
        ];
        for (dialog_id, story_id) in &values {
            let full_id = StoryFullId::new(*dialog_id, *story_id);
            let json = serde_json::to_string(&full_id).unwrap();
            let deserialized: StoryFullId = serde_json::from_str(&json).unwrap();
            assert_eq!(full_id, deserialized);
        }
    }
}
