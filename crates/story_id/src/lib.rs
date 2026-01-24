// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Story ID
//!
//! Story identifier type for Telegram.
//!
//! ## Overview
//!
//! This module provides the [`StoryId`] type, which represents a unique identifier
//! for a story in Telegram.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_story_id::StoryId;
//!
//! let id = StoryId::new(123);
//! assert_eq!(id.get(), 123);
//! assert!(id.is_valid());
//! assert!(id.is_server());
//! ```

use std::fmt;
use std::hash::Hash;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Maximum server story ID value.
///
/// Story IDs with values greater than this are considered local story IDs.
pub const MAX_SERVER_STORY_ID: i32 = 1999999999;

/// Story identifier.
///
/// Represents a unique identifier for a story in Telegram.
/// Based on TDLib's `StoryId` class.
///
/// # TDLib Alignment
///
/// Aligns with TDLib's `StoryId` class in `StoryId.h`.
///
/// # Server vs Local Stories
///
/// - Server stories: IDs in range `1..=MAX_SERVER_STORY_ID`
/// - Local stories: IDs greater than `MAX_SERVER_STORY_ID`
/// - Invalid: ID <= 0
///
/// # Example
///
/// ```rust
/// use rustgram_story_id::{StoryId, MAX_SERVER_STORY_ID};
///
/// let server_id = StoryId::new(123);
/// assert!(server_id.is_server());
///
/// let local_id = StoryId::new(MAX_SERVER_STORY_ID + 1);
/// assert!(local_id.is_valid());
/// assert!(!local_id.is_server());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoryId(i32);

impl StoryId {
    /// Creates a new story ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The raw story ID value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_id::StoryId;
    ///
    /// let id = StoryId::new(123);
    /// assert_eq!(id.get(), 123);
    /// ```
    #[must_use]
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    /// Creates a story ID from a raw value.
    ///
    /// This is an alias for [`new`][Self::new].
    ///
    /// # Arguments
    ///
    /// * `id` - The raw story ID value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_id::StoryId;
    ///
    /// let id = StoryId::from_raw(456);
    /// assert_eq!(id.get(), 456);
    /// ```
    #[must_use]
    pub const fn from_raw(id: i32) -> Self {
        Self::new(id)
    }

    /// Returns the raw story ID value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_id::StoryId;
    ///
    /// let id = StoryId::new(123);
    /// assert_eq!(id.get(), 123);
    /// ```
    #[must_use]
    pub const fn get(&self) -> i32 {
        self.0
    }

    /// Checks if this is a valid story ID.
    ///
    /// A valid story ID is positive (greater than 0).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_id::StoryId;
    ///
    /// assert!(StoryId::new(1).is_valid());
    /// assert!(!StoryId::new(0).is_valid());
    /// assert!(!StoryId::new(-1).is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.0 > 0
    }

    /// Checks if this is a server story ID.
    ///
    /// Server story IDs are in the range `1..=MAX_SERVER_STORY_ID`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_id::{StoryId, MAX_SERVER_STORY_ID};
    ///
    /// assert!(StoryId::new(1).is_server());
    /// assert!(StoryId::new(MAX_SERVER_STORY_ID).is_server());
    /// assert!(!StoryId::new(MAX_SERVER_STORY_ID + 1).is_server());
    /// assert!(!StoryId::new(0).is_server());
    /// ```
    #[must_use]
    pub fn is_server(&self) -> bool {
        self.0 > 0 && self.0 <= MAX_SERVER_STORY_ID
    }

    /// Converts a vector of story IDs to a vector of raw i32 values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_id::StoryId;
    ///
    /// let ids = vec![StoryId::new(123), StoryId::new(456)];
    /// let raw_ids = StoryId::get_input_story_ids(&ids);
    /// assert_eq!(raw_ids, vec![123, 456]);
    /// ```
    #[must_use]
    pub fn get_input_story_ids(story_ids: &[Self]) -> Vec<i32> {
        story_ids.iter().map(|id| id.get()).collect()
    }

    /// Creates a vector of story IDs from a vector of raw i32 values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_id::StoryId;
    ///
    /// let raw_ids = vec![123, 456];
    /// let ids = StoryId::get_story_ids(&raw_ids);
    /// assert_eq!(ids.len(), 2);
    /// assert_eq!(ids[0].get(), 123);
    /// assert_eq!(ids[1].get(), 456);
    /// ```
    #[must_use]
    pub fn get_story_ids(input_story_ids: &[i32]) -> Vec<Self> {
        input_story_ids.iter().map(|&id| Self::new(id)).collect()
    }
}

impl fmt::Display for StoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "story {}", self.0)
    }
}

impl From<i32> for StoryId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<StoryId> for i32 {
    fn from(id: StoryId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== new Tests ==========

    #[test]
    fn test_new() {
        let id = StoryId::new(123);
        assert_eq!(id.get(), 123);
    }

    #[test]
    fn test_from_raw() {
        let id = StoryId::from_raw(456);
        assert_eq!(id.get(), 456);
    }

    // ========== get Tests ==========

    #[test]
    fn test_get() {
        let id = StoryId::new(789);
        assert_eq!(id.get(), 789);
    }

    // ========== is_valid Tests ==========

    #[test]
    fn test_is_valid_true() {
        assert!(StoryId::new(1).is_valid());
        assert!(StoryId::new(123).is_valid());
        assert!(StoryId::new(i32::MAX).is_valid());
    }

    #[test]
    fn test_is_valid_false() {
        assert!(!StoryId::new(0).is_valid());
        assert!(!StoryId::new(-1).is_valid());
        assert!(!StoryId::new(i32::MIN).is_valid());
    }

    // ========== is_server Tests ==========

    #[test]
    fn test_is_server_true() {
        assert!(StoryId::new(1).is_server());
        assert!(StoryId::new(123).is_server());
        assert!(StoryId::new(MAX_SERVER_STORY_ID).is_server());
    }

    #[test]
    fn test_is_server_false() {
        assert!(!StoryId::new(0).is_server());
        assert!(!StoryId::new(-1).is_server());
        assert!(!StoryId::new(MAX_SERVER_STORY_ID + 1).is_server());
        assert!(!StoryId::new(i32::MAX).is_server());
    }

    // ========== get_input_story_ids Tests ==========

    #[test]
    fn test_get_input_story_ids_empty() {
        let ids = vec![];
        let raw_ids = StoryId::get_input_story_ids(&ids);
        assert!(raw_ids.is_empty());
    }

    #[test]
    fn test_get_input_story_ids_single() {
        let ids = vec![StoryId::new(123)];
        let raw_ids = StoryId::get_input_story_ids(&ids);
        assert_eq!(raw_ids, vec![123]);
    }

    #[test]
    fn test_get_input_story_ids_multiple() {
        let ids = vec![StoryId::new(123), StoryId::new(456), StoryId::new(789)];
        let raw_ids = StoryId::get_input_story_ids(&ids);
        assert_eq!(raw_ids, vec![123, 456, 789]);
    }

    // ========== get_story_ids Tests ==========

    #[test]
    fn test_get_story_ids_empty() {
        let raw_ids = vec![];
        let ids = StoryId::get_story_ids(&raw_ids);
        assert!(ids.is_empty());
    }

    #[test]
    fn test_get_story_ids_single() {
        let raw_ids = vec![123];
        let ids = StoryId::get_story_ids(&raw_ids);
        assert_eq!(ids.len(), 1);
        assert_eq!(ids[0].get(), 123);
    }

    #[test]
    fn test_get_story_ids_multiple() {
        let raw_ids = vec![123, 456, 789];
        let ids = StoryId::get_story_ids(&raw_ids);
        assert_eq!(ids.len(), 3);
        assert_eq!(ids[0].get(), 123);
        assert_eq!(ids[1].get(), 456);
        assert_eq!(ids[2].get(), 789);
    }

    // ========== default Tests ==========

    #[test]
    fn test_default() {
        let id = StoryId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
        assert!(!id.is_server());
    }

    // ========== equality Tests ==========

    #[test]
    fn test_equality_same() {
        let id1 = StoryId::new(123);
        let id2 = StoryId::new(123);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_equality_different() {
        let id1 = StoryId::new(123);
        let id2 = StoryId::new(456);
        assert_ne!(id1, id2);
    }

    // ========== clone Tests ==========

    #[test]
    fn test_clone() {
        let id1 = StoryId::new(123);
        let id2 = id1;
        assert_eq!(id1, id2);
    }

    // ========== hash Tests ==========

    #[test]
    fn test_hash_same() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;

        let id1 = StoryId::new(123);
        let id2 = StoryId::new(123);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        id1.hash(&mut hasher1);
        id2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_hash_different() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;

        let id1 = StoryId::new(123);
        let id2 = StoryId::new(456);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        id1.hash(&mut hasher1);
        id2.hash(&mut hasher2);

        assert_ne!(hasher1.finish(), hasher2.finish());
    }

    // ========== display Tests ==========

    #[test]
    fn test_display() {
        let id = StoryId::new(123);
        assert_eq!(format!("{}", id), "story 123");
    }

    #[test]
    fn test_debug() {
        let id = StoryId::new(123);
        let debug_str = format!("{:?}", id);
        assert!(debug_str.contains("StoryId"));
        assert!(debug_str.contains("123"));
    }

    // ========== from_i32 Tests ==========

    #[test]
    fn test_from_i32() {
        let id = StoryId::from(123);
        assert_eq!(id.get(), 123);
    }

    #[test]
    fn test_to_i32() {
        let id = StoryId::new(456);
        let value: i32 = id.into();
        assert_eq!(value, 456);
    }

    // ========== boundary Tests ==========

    #[test]
    fn test_max_server_story_id_boundary() {
        assert!(StoryId::new(MAX_SERVER_STORY_ID).is_server());
        assert!(!StoryId::new(MAX_SERVER_STORY_ID + 1).is_server());
    }

    #[test]
    fn test_i32_max() {
        let id = StoryId::new(i32::MAX);
        assert!(id.is_valid());
        assert!(!id.is_server());
    }

    #[test]
    fn test_i32_min() {
        let id = StoryId::new(i32::MIN);
        assert!(!id.is_valid());
        assert!(!id.is_server());
    }

    // ========== roundtrip Tests ==========

    #[test]
    fn test_roundtrip_input_story_ids() {
        let ids = vec![StoryId::new(123), StoryId::new(456)];
        let raw_ids = StoryId::get_input_story_ids(&ids);
        let restored_ids = StoryId::get_story_ids(&raw_ids);
        assert_eq!(ids, restored_ids);
    }

    // ========== Serialization Tests ==========

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let id = StoryId::new(123);
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: StoryId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_all_values() {
        let values = [0, 1, 123, MAX_SERVER_STORY_ID, MAX_SERVER_STORY_ID + 1];
        for &value in &values {
            let id = StoryId::new(value);
            let json = serde_json::to_string(&id).unwrap();
            let deserialized: StoryId = serde_json::from_str(&json).unwrap();
            assert_eq!(id, deserialized);
        }
    }
}
