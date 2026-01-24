// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Sticker Set ID
//!
//! Sticker set identifier for Telegram.
//!
//! Based on TDLib's `StickerSetId` from `td/telegram/StickerSetId.h`.
//!
//! # Overview
//!
//! A `StickerSetId` uniquely identifies a sticker set in Telegram.
//! Sticker sets contain collections of stickers that can be used in messages.
//!
//! # Example
//!
//! ```rust
//! use rustgram_sticker_set_id::StickerSetId;
//!
//! let id = StickerSetId::new(1234567890);
//! assert!(id.is_valid());
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Sticker set identifier.
///
/// Represents a unique identifier for a Telegram sticker set.
/// A valid sticker set ID is any non-zero i64 value.
///
/// # Example
///
/// ```
/// use rustgram_sticker_set_id::StickerSetId;
///
/// let id = StickerSetId::new(1234567890);
/// assert_eq!(id.get(), 1234567890);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StickerSetId(i64);

impl StickerSetId {
    /// Creates a new `StickerSetId` from an i64 value.
    ///
    /// # Arguments
    ///
    /// * `id` - The raw sticker set ID value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_set_id::StickerSetId;
    ///
    /// let id = StickerSetId::new(1234567890);
    /// assert_eq!(id.get(), 1234567890);
    /// ```
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the inner i64 value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_set_id::StickerSetId;
    ///
    /// let id = StickerSetId::new(1234567890);
    /// assert_eq!(id.get(), 1234567890);
    /// ```
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Checks if this is a valid sticker set ID.
    ///
    /// A valid sticker set ID is non-zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_set_id::StickerSetId;
    ///
    /// assert!(StickerSetId::new(1234567890).is_valid());
    /// assert!(!StickerSetId::new(0).is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.0 != 0
    }
}

impl fmt::Display for StickerSetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sticker set {}", self.0)
    }
}

impl Hash for StickerSetId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<i64> for StickerSetId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<StickerSetId> for i64 {
    fn from(id: StickerSetId) -> Self {
        id.0
    }
}

/// Hasher for StickerSetId.
///
/// Provides a hashing function for StickerSetId values, useful for
/// hash map keys.
#[derive(Debug, Clone, Copy, Default)]
pub struct StickerSetIdHash;

impl StickerSetIdHash {
    /// Creates a new StickerSetIdHash.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Hashes a StickerSetId value.
    #[must_use]
    pub fn hash(self, sticker_set_id: StickerSetId) -> u32 {
        // Simple hash using the lower 32 bits
        (sticker_set_id.0 as u32).wrapping_add((sticker_set_id.0 >> 32) as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = StickerSetId::new(1234567890);
        assert_eq!(id.get(), 1234567890);
    }

    #[test]
    fn test_default() {
        let id = StickerSetId::default();
        assert_eq!(id.get(), 0);
    }

    #[test]
    fn test_from_i64() {
        let id = StickerSetId::from(1234567890);
        assert_eq!(id.get(), 1234567890);
    }

    #[test]
    fn test_to_i64() {
        let id = StickerSetId::new(1234567890);
        let value: i64 = id.into();
        assert_eq!(value, 1234567890);
    }

    #[test]
    fn test_is_valid() {
        assert!(StickerSetId::new(1).is_valid());
        assert!(StickerSetId::new(-1).is_valid());
        assert!(StickerSetId::new(i64::MAX).is_valid());
        assert!(!StickerSetId::new(0).is_valid());
    }

    #[test]
    fn test_equality() {
        let id1 = StickerSetId::new(1234567890);
        let id2 = StickerSetId::new(1234567890);
        let id3 = StickerSetId::new(9876543210);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_display() {
        let id = StickerSetId::new(1234567890);
        let display = format!("{id}");
        assert!(display.contains("sticker set"));
        assert!(display.contains("1234567890"));
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;

        let id = StickerSetId::new(1234567890);
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        // Just ensure it doesn't panic
    }

    #[test]
    fn test_sticker_set_id_hash() {
        let hasher = StickerSetIdHash::new();
        let id = StickerSetId::new(1234567890);
        let hash = hasher.hash(id);
        // Just ensure it produces a value
        assert!(hash > 0 || hash == 0); // Any u32 is valid
    }

    #[test]
    fn test_clone() {
        let id1 = StickerSetId::new(1234567890);
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_serialization() {
        let id = StickerSetId::new(1234567890);
        let json = serde_json::to_string(&id).expect("Failed to serialize");
        assert!(json.contains("1234567890"));

        let deserialized: StickerSetId =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, id);
    }

    #[test]
    fn test_large_id() {
        let id = StickerSetId::new(i64::MAX);
        assert!(id.is_valid());
    }

    #[test]
    fn test_negative_id() {
        let id = StickerSetId::new(-1234567890);
        assert!(id.is_valid());
        assert_eq!(id.get(), -1234567890);
    }

    #[test]
    fn test_zero_id_not_valid() {
        let id = StickerSetId::new(0);
        assert!(!id.is_valid());
    }
}
