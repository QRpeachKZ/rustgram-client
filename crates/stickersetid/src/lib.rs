// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Sticker Set ID
//!
//! Unique identifier for sticker sets in Telegram.
//!
//! ## Overview
//!
//! This module provides the [`StickerSetId`] type, which represents a unique
//! identifier for sticker sets in the Telegram MTProto protocol. It mirrors
//! TDLib's `StickerSetId` class, providing a type-safe wrapper around `i64`
//! identifiers.
//!
//! ## Types
//!
//! - [`StickerSetId`] - Sticker set identifier with validation and hashing support
//! - [`StickerSetIdHash`] - Hasher for using `StickerSetId` in `HashSet`/`HashMap`
//!
//! ## Example
//!
//! ```rust
//! use rustgram_stickersetid::StickerSetId;
//!
//! // Create a sticker set ID
//! let set_id = StickerSetId::new(1234567890);
//! assert!(set_id.is_valid());
//!
//! // Create from i64
//! let set_id2: StickerSetId = 9876543210.into();
//! assert_eq!(set_id2.get(), 9876543210);
//!
//! // Default (invalid) ID
//! let default_id = StickerSetId::default();
//! assert!(!default_id.is_valid());
//! ```

use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Unique identifier for a sticker set.
///
/// This type provides a type-safe wrapper around an `i64` identifier for
/// sticker sets in Telegram. An ID is considered valid if it is non-zero.
///
/// # Example
///
/// ```rust
/// use rustgram_stickersetid::StickerSetId;
///
/// // Create a valid sticker set ID
/// let set_id = StickerSetId::new(1234567890);
/// assert!(set_id.is_valid());
/// assert_eq!(set_id.get(), 1234567890);
///
/// // Create from i64
/// let set_id2: StickerSetId = 9876543210.into();
/// assert_eq!(set_id2.get(), 9876543210);
///
/// // Default (invalid) ID
/// let default_id = StickerSetId::default();
/// assert!(!default_id.is_valid());
/// assert_eq!(default_id.get(), 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StickerSetId {
    /// The unique identifier value
    id: i64,
}

impl StickerSetId {
    /// Creates a new sticker set ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_stickersetid::StickerSetId;
    ///
    /// let set_id = StickerSetId::new(1234567890);
    /// assert_eq!(set_id.get(), 1234567890);
    /// ```
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self { id }
    }

    /// Returns the inner identifier value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_stickersetid::StickerSetId;
    ///
    /// let set_id = StickerSetId::new(1234567890);
    /// assert_eq!(set_id.get(), 1234567890);
    /// ```
    #[must_use]
    pub const fn get(self) -> i64 {
        self.id
    }

    /// Checks if this is a valid sticker set ID.
    ///
    /// A valid ID must be non-zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_stickersetid::StickerSetId;
    ///
    /// assert!(StickerSetId::new(1234567890).is_valid());
    /// assert!(StickerSetId::new(-1).is_valid());
    /// assert!(!StickerSetId::new(0).is_valid());
    /// ```
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.id != 0
    }
}

impl Default for StickerSetId {
    /// Creates a default sticker set ID with value 0 (invalid).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_stickersetid::StickerSetId;
    ///
    /// let default_id = StickerSetId::default();
    /// assert_eq!(default_id.get(), 0);
    /// assert!(!default_id.is_valid());
    /// ```
    fn default() -> Self {
        Self { id: 0 }
    }
}

impl From<i64> for StickerSetId {
    /// Creates a sticker set ID from an i64 value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_stickersetid::StickerSetId;
    ///
    /// let set_id: StickerSetId = 12345.into();
    /// assert_eq!(set_id.get(), 12345);
    /// ```
    fn from(id: i64) -> Self {
        Self::new(id)
    }
}

impl fmt::Display for StickerSetId {
    /// Formats the sticker set ID for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_stickersetid::StickerSetId;
    ///
    /// let set_id = StickerSetId::new(12345);
    /// assert_eq!(format!("{}", set_id), "sticker set 12345");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "sticker set {}", self.id)
    }
}

/// Hasher for [`StickerSetId`] to use in `HashSet` and `HashMap`.
///
/// This hasher provides a consistent hash value for sticker set IDs,
/// allowing them to be used as keys in hash-based collections.
///
/// # Example
///
/// ```rust
/// use rustgram_stickersetid::{StickerSetId, StickerSetIdHash};
/// use std::collections::HashSet;
///
/// let mut set = HashSet::with_hasher(StickerSetIdHash);
/// set.insert(StickerSetId::new(12345));
/// set.insert(StickerSetId::new(67890));
/// assert_eq!(set.len(), 2);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct StickerSetIdHash;

impl std::hash::BuildHasher for StickerSetIdHash {
    type Hasher = StickerSetIdHasher;

    fn build_hasher(&self) -> Self::Hasher {
        StickerSetIdHasher::default()
    }
}

/// Hasher implementation for `StickerSetId`.
///
/// This is the actual hasher type used by `StickerSetIdHash`.
#[derive(Debug, Default)]
pub struct StickerSetIdHasher(DefaultHasher);

impl Hasher for StickerSetIdHasher {
    fn finish(&self) -> u64 {
        self.0.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }

    fn write_i64(&mut self, i: i64) {
        self.0.write_i64(i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid_id() {
        let set_id = StickerSetId::new(1234567890);
        assert_eq!(set_id.get(), 1234567890);
    }

    #[test]
    fn test_new_negative_id() {
        let set_id = StickerSetId::new(-1234567890);
        assert_eq!(set_id.get(), -1234567890);
        assert!(set_id.is_valid());
    }

    #[test]
    fn test_new_zero_id() {
        let set_id = StickerSetId::new(0);
        assert_eq!(set_id.get(), 0);
    }

    #[test]
    fn test_get_returns_value() {
        let set_id = StickerSetId::new(9876543210);
        assert_eq!(set_id.get(), 9876543210);
    }

    #[test]
    fn test_is_valid_true() {
        assert!(StickerSetId::new(1).is_valid());
        assert!(StickerSetId::new(1234567890).is_valid());
        assert!(StickerSetId::new(-1).is_valid());
        assert!(StickerSetId::new(-1234567890).is_valid());
    }

    #[test]
    fn test_is_valid_false_zero() {
        assert!(!StickerSetId::new(0).is_valid());
    }

    #[test]
    fn test_equality() {
        let id1 = StickerSetId::new(12345);
        let id2 = StickerSetId::new(12345);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_inequality() {
        let id1 = StickerSetId::new(12345);
        let id2 = StickerSetId::new(67890);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_copy_semantics() {
        let id1 = StickerSetId::new(12345);
        let id2 = id1;
        assert_eq!(id1, id2);
        assert_eq!(id1.get(), 12345);
        assert_eq!(id2.get(), 12345);
    }

    #[test]
    fn test_display_format() {
        let set_id = StickerSetId::new(12345);
        assert_eq!(format!("{}", set_id), "sticker set 12345");

        let set_id = StickerSetId::new(0);
        assert_eq!(format!("{}", set_id), "sticker set 0");

        let set_id = StickerSetId::new(-9999);
        assert_eq!(format!("{}", set_id), "sticker set -9999");
    }

    #[test]
    fn test_from_i64() {
        let set_id: StickerSetId = 12345.into();
        assert_eq!(set_id.get(), 12345);

        let set_id: StickerSetId = 0.into();
        assert_eq!(set_id.get(), 0);

        let set_id: StickerSetId = (-99999).into();
        assert_eq!(set_id.get(), -99999);
    }

    #[test]
    fn test_default() {
        let default_id = StickerSetId::default();
        assert_eq!(default_id.get(), 0);
        assert!(!default_id.is_valid());
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashSet;
        let mut set = HashSet::with_hasher(StickerSetIdHash);

        let id1 = StickerSetId::new(12345);
        let id2 = StickerSetId::new(12345);
        let id3 = StickerSetId::new(67890);

        set.insert(id1);
        set.insert(id2);
        set.insert(id3);

        assert_eq!(set.len(), 2);
        assert!(set.contains(&id1));
        assert!(set.contains(&id3));
    }

    #[test]
    fn test_ord_partial_ord() {
        // Verify that we can compare IDs
        let id1 = StickerSetId::new(12345);
        let id2 = StickerSetId::new(12345);
        let id3 = StickerSetId::new(67890);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let original = StickerSetId::new(1234567890);

        // Test JSON serialization
        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(json, r#"{"id":1234567890}"#);

        let deserialized: StickerSetId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);

        // Test binary serialization
        let encoded = bincode::serialize(&original).unwrap();
        let decoded: StickerSetId = bincode::deserialize(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_zero() {
        let original = StickerSetId::new(0);

        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(json, r#"{"id":0}"#);

        let deserialized: StickerSetId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_negative() {
        let original = StickerSetId::new(-1234567890);

        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(json, r#"{"id":-1234567890}"#);

        let deserialized: StickerSetId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
