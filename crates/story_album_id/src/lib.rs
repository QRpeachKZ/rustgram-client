// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Story Album ID
//!
//! Story album identifier types for Telegram.
//!
//! ## Overview
//!
//! This module provides story album identifier types:
//!
//! - [`StoryAlbumId`] - Album identifier
//! - [`StoryAlbumFullId`] - Full album identifier (dialog + album)
//!
//! ## Example
//!
//! ```rust
//! use rustgram_story_album_id::{StoryAlbumId, StoryAlbumFullId};
//! use rustgram_dialog_id::DialogId;
//!
//! let album_id = StoryAlbumId::new(123);
//! assert_eq!(album_id.get(), 123);
//! assert!(album_id.is_valid());
//!
//! let full_id = StoryAlbumFullId::new(DialogId::new(456), album_id);
//! assert_eq!(full_id.dialog_id().get(), 456);
//! assert_eq!(full_id.album_id().get(), 123);
//! ```

use std::fmt;
use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use rustgram_dialog_id::DialogId;

/// Story album identifier.
///
/// Represents a unique identifier for a story album.
///
/// # Example
///
/// ```rust
/// use rustgram_story_album_id::StoryAlbumId;
///
/// let id = StoryAlbumId::new(123);
/// assert_eq!(id.get(), 123);
/// assert!(id.is_valid());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoryAlbumId(i64);

impl StoryAlbumId {
    /// Creates a new story album ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The raw album ID value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_album_id::StoryAlbumId;
    ///
    /// let id = StoryAlbumId::new(123);
    /// assert_eq!(id.get(), 123);
    /// ```
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Creates a story album ID from a raw value.
    ///
    /// This is an alias for [`new`][Self::new].
    ///
    /// # Arguments
    ///
    /// * `id` - The raw album ID value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_album_id::StoryAlbumId;
    ///
    /// let id = StoryAlbumId::from_raw(456);
    /// assert_eq!(id.get(), 456);
    /// ```
    #[must_use]
    pub const fn from_raw(id: i64) -> Self {
        Self::new(id)
    }

    /// Returns the raw album ID value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_album_id::StoryAlbumId;
    ///
    /// let id = StoryAlbumId::new(123);
    /// assert_eq!(id.get(), 123);
    /// ```
    #[must_use]
    pub const fn get(&self) -> i64 {
        self.0
    }

    /// Checks if this is a valid album ID.
    ///
    /// A valid album ID is non-zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_album_id::StoryAlbumId;
    ///
    /// assert!(StoryAlbumId::new(1).is_valid());
    /// assert!(StoryAlbumId::new(-1).is_valid());
    /// assert!(!StoryAlbumId::new(0).is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.0 != 0
    }
}

impl fmt::Display for StoryAlbumId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "album {}", self.0)
    }
}

impl From<i64> for StoryAlbumId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<StoryAlbumId> for i64 {
    fn from(id: StoryAlbumId) -> Self {
        id.0
    }
}

/// Full story album identifier.
///
/// Combines a dialog ID and an album ID to uniquely identify
/// a story album within a specific dialog.
///
/// # Example
///
/// ```rust
/// use rustgram_story_album_id::{StoryAlbumId, StoryAlbumFullId};
/// use rustgram_dialog_id::DialogId;
///
/// let album_id = StoryAlbumId::new(123);
/// let dialog_id = DialogId::new(456);
/// let full_id = StoryAlbumFullId::new(dialog_id, album_id);
/// assert_eq!(full_id.dialog_id().get(), 456);
/// assert_eq!(full_id.album_id().get(), 123);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoryAlbumFullId {
    /// The dialog containing this album.
    dialog_id: DialogId,

    /// The album identifier.
    album_id: StoryAlbumId,
}

impl StoryAlbumFullId {
    /// Creates a new full story album ID.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog containing this album
    /// * `album_id` - The album identifier
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_album_id::{StoryAlbumId, StoryAlbumFullId};
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog_id = DialogId::new(456);
    /// let album_id = StoryAlbumId::new(123);
    /// let full_id = StoryAlbumFullId::new(dialog_id, album_id);
    /// ```
    #[must_use]
    pub const fn new(dialog_id: DialogId, album_id: StoryAlbumId) -> Self {
        Self {
            dialog_id,
            album_id,
        }
    }

    /// Returns the dialog ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_album_id::{StoryAlbumId, StoryAlbumFullId};
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let full_id = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(123));
    /// assert_eq!(full_id.dialog_id().get(), 456);
    /// ```
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the album ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_album_id::{StoryAlbumId, StoryAlbumFullId};
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let full_id = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(123));
    /// assert_eq!(full_id.album_id().get(), 123);
    /// ```
    #[must_use]
    pub const fn album_id(&self) -> StoryAlbumId {
        self.album_id
    }

    /// Checks if this is a valid full album ID.
    ///
    /// Both the dialog ID and album ID must be valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_album_id::{StoryAlbumId, StoryAlbumFullId};
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let valid = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(123));
    /// assert!(valid.is_valid());
    ///
    /// let invalid = StoryAlbumFullId::new(DialogId::new(0), StoryAlbumId::new(123));
    /// assert!(!invalid.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.dialog_id.is_valid() && self.album_id.is_valid()
    }
}

impl fmt::Display for StoryAlbumFullId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "album {} in {}", self.album_id.get(), self.dialog_id.get())
    }
}

impl Hash for StoryAlbumFullId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dialog_id.hash(state);
        self.album_id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== StoryAlbumId Tests ==========

    #[test]
    fn test_album_id_new() {
        let id = StoryAlbumId::new(123);
        assert_eq!(id.get(), 123);
    }

    #[test]
    fn test_album_id_from_raw() {
        let id = StoryAlbumId::from_raw(456);
        assert_eq!(id.get(), 456);
    }

    #[test]
    fn test_album_id_get() {
        let id = StoryAlbumId::new(789);
        assert_eq!(id.get(), 789);
    }

    #[test]
    fn test_album_id_is_valid_true() {
        assert!(StoryAlbumId::new(1).is_valid());
        assert!(StoryAlbumId::new(-1).is_valid());
        assert!(StoryAlbumId::new(i64::MAX).is_valid());
        assert!(StoryAlbumId::new(i64::MIN).is_valid());
    }

    #[test]
    fn test_album_id_is_valid_false() {
        assert!(!StoryAlbumId::new(0).is_valid());
    }

    #[test]
    fn test_album_id_default() {
        let id = StoryAlbumId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_album_id_from_i64() {
        let id = StoryAlbumId::from(123);
        assert_eq!(id.get(), 123);
    }

    #[test]
    fn test_album_id_to_i64() {
        let id = StoryAlbumId::new(456);
        let value: i64 = id.into();
        assert_eq!(value, 456);
    }

    #[test]
    fn test_album_id_equality() {
        let id1 = StoryAlbumId::new(123);
        let id2 = StoryAlbumId::new(123);
        let id3 = StoryAlbumId::new(456);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_album_id_clone() {
        let id1 = StoryAlbumId::new(123);
        let id2 = id1;
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_album_id_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;

        let id1 = StoryAlbumId::new(123);
        let id2 = StoryAlbumId::new(123);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        id1.hash(&mut hasher1);
        id2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_album_id_display() {
        let id = StoryAlbumId::new(123);
        assert_eq!(format!("{}", id), "album 123");
    }

    #[test]
    fn test_album_id_debug() {
        let id = StoryAlbumId::new(123);
        let debug_str = format!("{:?}", id);
        assert!(debug_str.contains("StoryAlbumId"));
        assert!(debug_str.contains("123"));
    }

    // ========== StoryAlbumFullId Tests ==========

    #[test]
    fn test_full_id_new() {
        let dialog_id = DialogId::new(456);
        let album_id = StoryAlbumId::new(123);
        let full_id = StoryAlbumFullId::new(dialog_id, album_id);
        assert_eq!(full_id.dialog_id().get(), 456);
        assert_eq!(full_id.album_id().get(), 123);
    }

    #[test]
    fn test_full_id_dialog_id() {
        let full_id = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(123));
        assert_eq!(full_id.dialog_id().get(), 456);
    }

    #[test]
    fn test_full_id_album_id() {
        let full_id = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(123));
        assert_eq!(full_id.album_id().get(), 123);
    }

    #[test]
    fn test_full_id_is_valid_true() {
        let full_id = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(123));
        assert!(full_id.is_valid());
    }

    #[test]
    fn test_full_id_is_valid_invalid_dialog() {
        let full_id = StoryAlbumFullId::new(DialogId::new(0), StoryAlbumId::new(123));
        assert!(!full_id.is_valid());
    }

    #[test]
    fn test_full_id_is_valid_invalid_album() {
        let full_id = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(0));
        assert!(!full_id.is_valid());
    }

    #[test]
    fn test_full_id_is_valid_both_invalid() {
        let full_id = StoryAlbumFullId::new(DialogId::new(0), StoryAlbumId::new(0));
        assert!(!full_id.is_valid());
    }

    #[test]
    fn test_full_id_equality() {
        let full_id1 = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(123));
        let full_id2 = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(123));
        let full_id3 = StoryAlbumFullId::new(DialogId::new(789), StoryAlbumId::new(123));
        assert_eq!(full_id1, full_id2);
        assert_ne!(full_id1, full_id3);
    }

    #[test]
    fn test_full_id_clone() {
        let full_id1 = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(123));
        let full_id2 = full_id1;
        assert_eq!(full_id1, full_id2);
    }

    #[test]
    fn test_full_id_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;

        let full_id1 = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(123));
        let full_id2 = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(123));

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        full_id1.hash(&mut hasher1);
        full_id2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_full_id_display() {
        let full_id = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(123));
        let display = format!("{}", full_id);
        assert!(display.contains("123"));
        assert!(display.contains("456"));
    }

    #[test]
    fn test_full_id_debug() {
        let full_id = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(123));
        let debug_str = format!("{:?}", full_id);
        assert!(debug_str.contains("StoryAlbumFullId"));
    }

    // ========== Serialization Tests ==========

    #[cfg(feature = "serde")]
    #[test]
    fn test_album_id_serialize() {
        let id = StoryAlbumId::new(123);
        let json = serde_json::to_string(&id).unwrap();
        assert!(json.contains("123"));
        let deserialized: StoryAlbumId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_full_id_serialize() {
        let full_id = StoryAlbumFullId::new(DialogId::new(456), StoryAlbumId::new(123));
        let json = serde_json::to_string(&full_id).unwrap();
        let deserialized: StoryAlbumFullId = serde_json::from_str(&json).unwrap();
        assert_eq!(full_id, deserialized);
    }
}
