// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Star Gift Collection ID
//!
//! Star gift collection identifier for Telegram.
//!
//! Based on TDLib's `StarGiftCollectionId` from `td/telegram/StarGiftCollectionId.h`.
//!
//! # Overview
//!
//! A `StarGiftCollectionId` uniquely identifies a star gift collection in Telegram.
//! Star gift collections are groups of star gifts that can be purchased or won in auctions.
//!
//! # Example
//!
//! ```rust
//! use rustgram_star_gift_collection_id::StarGiftCollectionId;
//!
//! let id = StarGiftCollectionId::new(1234567890);
//! assert!(id.is_valid());
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Star gift collection identifier.
///
/// Represents a unique identifier for a Telegram star gift collection.
/// A valid star gift collection ID is any positive i32 value.
///
/// # TDLib Mapping
///
/// - `StarGiftCollectionId::new(0)` → TDLib: default/invalid ID
/// - `StarGiftCollectionId::new(n)` where n > 0 → TDLib: valid collection ID
///
/// # Example
///
/// ```rust
/// use rustgram_star_gift_collection_id::StarGiftCollectionId;
///
/// let id = StarGiftCollectionId::new(1234567890);
/// assert_eq!(id.get(), 1234567890);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StarGiftCollectionId(i32);

impl StarGiftCollectionId {
    /// Creates a new `StarGiftCollectionId` from an i32 value.
    ///
    /// # Arguments
    ///
    /// * `id` - The raw star gift collection ID value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_collection_id::StarGiftCollectionId;
    ///
    /// let id = StarGiftCollectionId::new(1234567890);
    /// assert_eq!(id.get(), 1234567890);
    /// ```
    #[must_use]
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    /// Returns the inner i32 value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_collection_id::StarGiftCollectionId;
    ///
    /// let id = StarGiftCollectionId::new(1234567890);
    /// assert_eq!(id.get(), 1234567890);
    /// ```
    #[must_use]
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Checks if this is a valid star gift collection ID.
    ///
    /// A valid star gift collection ID is positive (> 0).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_collection_id::StarGiftCollectionId;
    ///
    /// assert!(StarGiftCollectionId::new(1234567890).is_valid());
    /// assert!(!StarGiftCollectionId::new(0).is_valid());
    /// assert!(!StarGiftCollectionId::new(-1).is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.0 > 0
    }
}

impl fmt::Display for StarGiftCollectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "gift collection {}", self.0)
    }
}

impl Hash for StarGiftCollectionId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<i32> for StarGiftCollectionId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<StarGiftCollectionId> for i32 {
    fn from(id: StarGiftCollectionId) -> Self {
        id.0
    }
}

/// Hasher for StarGiftCollectionId.
///
/// Provides a hashing function for StarGiftCollectionId values, useful for
/// hash map keys.
#[derive(Debug, Clone, Copy, Default)]
pub struct StarGiftCollectionIdHash;

impl StarGiftCollectionIdHash {
    /// Creates a new StarGiftCollectionIdHash.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Hashes a StarGiftCollectionId value.
    #[must_use]
    pub fn hash(self, star_gift_collection_id: StarGiftCollectionId) -> u32 {
        star_gift_collection_id.0 as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = StarGiftCollectionId::new(1234567890);
        assert_eq!(id.get(), 1234567890);
    }

    #[test]
    fn test_default() {
        let id = StarGiftCollectionId::default();
        assert_eq!(id.get(), 0);
    }

    #[test]
    fn test_from_i32() {
        let id = StarGiftCollectionId::from(1234567890);
        assert_eq!(id.get(), 1234567890);
    }

    #[test]
    fn test_to_i32() {
        let id = StarGiftCollectionId::new(1234567890);
        let value: i32 = id.into();
        assert_eq!(value, 1234567890);
    }

    #[test]
    fn test_is_valid() {
        assert!(StarGiftCollectionId::new(1).is_valid());
        assert!(StarGiftCollectionId::new(i32::MAX).is_valid());
        assert!(!StarGiftCollectionId::new(0).is_valid());
        assert!(!StarGiftCollectionId::new(-1).is_valid());
        assert!(!StarGiftCollectionId::new(i32::MIN).is_valid());
    }

    #[test]
    fn test_equality() {
        let id1 = StarGiftCollectionId::new(1234567890);
        let id2 = StarGiftCollectionId::new(1234567890);
        let id3 = StarGiftCollectionId::new(987654321);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_display() {
        let id = StarGiftCollectionId::new(1234567890);
        let display = format!("{id}");
        assert!(display.contains("gift collection"));
        assert!(display.contains("1234567890"));
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;

        let id = StarGiftCollectionId::new(1234567890);
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        // Just ensure it doesn't panic
    }

    #[test]
    fn test_star_gift_collection_id_hash() {
        let hasher = StarGiftCollectionIdHash::new();
        let id = StarGiftCollectionId::new(1234567890);
        let hash = hasher.hash(id);
        assert_eq!(hash, 1234567890u32);
    }

    #[test]
    fn test_clone() {
        let id1 = StarGiftCollectionId::new(1234567890);
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_serialization() {
        let id = StarGiftCollectionId::new(1234567890);
        let json = serde_json::to_string(&id).expect("Failed to serialize");
        assert!(json.contains("1234567890"));

        let deserialized: StarGiftCollectionId =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, id);
    }

    #[test]
    fn test_max_id() {
        let id = StarGiftCollectionId::new(i32::MAX);
        assert!(id.is_valid());
        assert_eq!(id.get(), i32::MAX);
    }

    #[test]
    fn test_min_id() {
        let id = StarGiftCollectionId::new(i32::MIN);
        assert!(!id.is_valid());
        assert_eq!(id.get(), i32::MIN);
    }

    #[test]
    fn test_zero_id_not_valid() {
        let id = StarGiftCollectionId::new(0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_negative_id_not_valid() {
        let id = StarGiftCollectionId::new(-100);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_positive_id_valid() {
        for id in [1, 100, 1000, 1_000_000, i32::MAX] {
            assert!(StarGiftCollectionId::new(id).is_valid());
        }
    }
}
