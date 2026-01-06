// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Input Group Call ID
//!
//! Identifier for group calls in Telegram with access hash.
//!
//! ## Overview
//!
//! This module provides the [`InputGroupCallId`] struct, which represents
//! a unique identifier for group calls in Telegram. It includes both
//! the group call ID and an access hash for authentication.
//! It mirrors TDLib's `InputGroupCallId` class.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_input_group_call_id::InputGroupCallId;
//!
//! // Create a group call ID
//! let call_id = InputGroupCallId::new(1234567890, 9876543210);
//! assert!(call_id.is_valid());
//! assert_eq!(call_id.group_call_id(), 1234567890);
//! ```

use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Unique identifier for a group call.
///
/// This type provides a type-safe wrapper around a group call identifier
/// with an access hash for authentication in Telegram.
///
/// # Fields
///
/// - `group_call_id` - The unique group call identifier
/// - `access_hash` - The access hash for authentication
///
/// # Example
///
/// ```rust
/// use rustgram_input_group_call_id::InputGroupCallId;
///
/// // Create a valid group call ID
/// let call_id = InputGroupCallId::new(1234567890, 9876543210);
/// assert!(call_id.is_valid());
/// assert_eq!(call_id.group_call_id(), 1234567890);
/// assert_eq!(call_id.access_hash(), 9876543210);
///
/// // Check if two IDs are identical (same group_call_id AND access_hash)
/// let call_id2 = InputGroupCallId::new(1234567890, 1111111111);
/// assert_eq!(call_id, call_id2); // Same group_call_id
/// assert!(!call_id.is_identical(&call_id2)); // Different access_hash
/// ```
#[derive(Debug, Clone, Copy, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InputGroupCallId {
    /// The unique group call identifier.
    group_call_id: i64,

    /// The access hash for authentication.
    access_hash: i64,
}

impl PartialEq for InputGroupCallId {
    /// Compares two group call IDs by group_call_id only.
    ///
    /// This matches TDLib's behavior where only the group_call_id is used
    /// for equality comparison. Use `is_identical()` to compare both fields.
    fn eq(&self, other: &Self) -> bool {
        self.group_call_id == other.group_call_id
    }
}

/// Hasher for using [`InputGroupCallId`] in `HashSet` and `HashMap`.
///
/// This hasher provides a consistent hash value for group call IDs,
/// allowing them to be used as keys in hash-based collections.
///
/// # Example
///
/// ```rust
/// use rustgram_input_group_call_id::{InputGroupCallId, InputGroupCallIdHash};
/// use std::collections::HashSet;
///
/// let mut set = HashSet::with_hasher(InputGroupCallIdHash);
/// set.insert(InputGroupCallId::new(12345, 67890));
/// set.insert(InputGroupCallId::new(99999, 11111));
/// assert_eq!(set.len(), 2);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct InputGroupCallIdHash;

impl std::hash::BuildHasher for InputGroupCallIdHash {
    type Hasher = InputGroupCallIdHasher;

    fn build_hasher(&self) -> Self::Hasher {
        InputGroupCallIdHasher::default()
    }
}

/// Hasher implementation for `InputGroupCallId`.
///
/// This is the actual hasher type used by `InputGroupCallIdHash`.
#[derive(Debug, Default)]
pub struct InputGroupCallIdHasher(DefaultHasher);

impl Hasher for InputGroupCallIdHasher {
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

impl InputGroupCallId {
    /// Creates a new group call ID.
    ///
    /// # Arguments
    ///
    /// * `group_call_id` - The unique group call identifier
    /// * `access_hash` - The access hash for authentication
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_input_group_call_id::InputGroupCallId;
    ///
    /// let call_id = InputGroupCallId::new(1234567890, 9876543210);
    /// assert_eq!(call_id.group_call_id(), 1234567890);
    /// assert_eq!(call_id.access_hash(), 9876543210);
    /// ```
    #[must_use]
    pub const fn new(group_call_id: i64, access_hash: i64) -> Self {
        Self {
            group_call_id,
            access_hash,
        }
    }

    /// Returns the group call ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_input_group_call_id::InputGroupCallId;
    ///
    /// let call_id = InputGroupCallId::new(1234567890, 9876543210);
    /// assert_eq!(call_id.group_call_id(), 1234567890);
    /// ```
    #[must_use]
    pub const fn group_call_id(&self) -> i64 {
        self.group_call_id
    }

    /// Returns the access hash.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_input_group_call_id::InputGroupCallId;
    ///
    /// let call_id = InputGroupCallId::new(1234567890, 9876543210);
    /// assert_eq!(call_id.access_hash(), 9876543210);
    /// ```
    #[must_use]
    pub const fn access_hash(&self) -> i64 {
        self.access_hash
    }

    /// Returns both values as a tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_input_group_call_id::InputGroupCallId;
    ///
    /// let call_id = InputGroupCallId::new(1234567890, 9876543210);
    /// assert_eq!(call_id.get(), (1234567890, 9876543210));
    /// ```
    #[must_use]
    pub const fn get(&self) -> (i64, i64) {
        (self.group_call_id, self.access_hash)
    }

    /// Checks if this is a valid group call ID.
    ///
    /// A valid ID must have a non-zero group_call_id.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_input_group_call_id::InputGroupCallId;
    ///
    /// assert!(InputGroupCallId::new(12345, 67890).is_valid());
    /// assert!(!InputGroupCallId::new(0, 67890).is_valid());
    /// ```
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.group_call_id != 0
    }

    /// Checks if this ID is identical to another (both fields must match).
    ///
    /// Unlike `PartialEq`, which only compares `group_call_id`, this method
    /// checks both `group_call_id` and `access_hash`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_input_group_call_id::InputGroupCallId;
    ///
    /// let id1 = InputGroupCallId::new(12345, 67890);
    /// let id2 = InputGroupCallId::new(12345, 11111);
    ///
    /// // PartialEq only checks group_call_id
    /// assert_eq!(id1, id2);
    ///
    /// // is_identical checks both fields
    /// assert!(!id1.is_identical(&id2));
    /// ```
    #[must_use]
    pub const fn is_identical(&self, other: &Self) -> bool {
        self.group_call_id == other.group_call_id && self.access_hash == other.access_hash
    }

    /// Returns the hash of the group call ID.
    ///
    /// This is useful for using group call IDs in hash-based collections.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_input_group_call_id::InputGroupCallId;
    ///
    /// let call_id = InputGroupCallId::new(12345, 67890);
    /// let hash = call_id.get_hash();
    /// assert!(hash > 0);
    /// ```
    #[must_use]
    pub fn get_hash(&self) -> u32 {
        let mut hasher = DefaultHasher::new();
        self.group_call_id.hash(&mut hasher);
        (hasher.finish() & 0xFFFFFFFF) as u32
    }
}

impl Default for InputGroupCallId {
    /// Creates a default group call ID with zeros (invalid).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_input_group_call_id::InputGroupCallId;
    ///
    /// let default = InputGroupCallId::default();
    /// assert_eq!(default.group_call_id(), 0);
    /// assert_eq!(default.access_hash(), 0);
    /// assert!(!default.is_valid());
    /// ```
    fn default() -> Self {
        Self {
            group_call_id: 0,
            access_hash: 0,
        }
    }
}

impl fmt::Display for InputGroupCallId {
    /// Formats the group call ID for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_input_group_call_id::InputGroupCallId;
    ///
    /// let call_id = InputGroupCallId::new(12345, 67890);
    /// assert_eq!(format!("{}", call_id), "group call 12345");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "group call {}", self.group_call_id)
    }
}

impl From<(i64, i64)> for InputGroupCallId {
    /// Creates a group call ID from a tuple of (group_call_id, access_hash).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_input_group_call_id::InputGroupCallId;
    ///
    /// let call_id: InputGroupCallId = (12345, 67890).into();
    /// assert_eq!(call_id.group_call_id(), 12345);
    /// assert_eq!(call_id.access_hash(), 67890);
    /// ```
    fn from((group_call_id, access_hash): (i64, i64)) -> Self {
        Self::new(group_call_id, access_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let call_id = InputGroupCallId::new(1234567890, 9876543210);
        assert_eq!(call_id.group_call_id(), 1234567890);
        assert_eq!(call_id.access_hash(), 9876543210);
    }

    #[test]
    fn test_group_call_id() {
        let call_id = InputGroupCallId::new(99999, 67890);
        assert_eq!(call_id.group_call_id(), 99999);
    }

    #[test]
    fn test_access_hash() {
        let call_id = InputGroupCallId::new(12345, 99999);
        assert_eq!(call_id.access_hash(), 99999);
    }

    #[test]
    fn test_get() {
        let call_id = InputGroupCallId::new(1234567890, 9876543210);
        assert_eq!(call_id.get(), (1234567890, 9876543210));
    }

    #[test]
    fn test_is_valid_true() {
        assert!(InputGroupCallId::new(1, 67890).is_valid());
        assert!(InputGroupCallId::new(12345, 67890).is_valid());
        assert!(InputGroupCallId::new(-1, 67890).is_valid());
    }

    #[test]
    fn test_is_valid_false_zero() {
        assert!(!InputGroupCallId::new(0, 67890).is_valid());
    }

    #[test]
    fn test_is_identical_true() {
        let id1 = InputGroupCallId::new(12345, 67890);
        let id2 = InputGroupCallId::new(12345, 67890);
        assert!(id1.is_identical(&id2));
    }

    #[test]
    fn test_is_identical_false_different_id() {
        let id1 = InputGroupCallId::new(12345, 67890);
        let id2 = InputGroupCallId::new(99999, 67890);
        assert!(!id1.is_identical(&id2));
    }

    #[test]
    fn test_is_identical_false_different_hash() {
        let id1 = InputGroupCallId::new(12345, 67890);
        let id2 = InputGroupCallId::new(12345, 11111);
        assert!(!id1.is_identical(&id2));
    }

    #[test]
    fn test_get_hash() {
        let call_id = InputGroupCallId::new(12345, 67890);
        let hash = call_id.get_hash();
        assert!(hash > 0);
    }

    #[test]
    fn test_partial_eq_same_group_call_id() {
        let id1 = InputGroupCallId::new(12345, 67890);
        let id2 = InputGroupCallId::new(12345, 11111);
        assert_eq!(id1, id2); // Only group_call_id is compared
    }

    #[test]
    fn test_partial_eq_different_group_call_id() {
        let id1 = InputGroupCallId::new(12345, 67890);
        let id2 = InputGroupCallId::new(99999, 67890);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_default() {
        let default = InputGroupCallId::default();
        assert_eq!(default.group_call_id(), 0);
        assert_eq!(default.access_hash(), 0);
        assert!(!default.is_valid());
    }

    #[test]
    fn test_copy_semantics() {
        let id1 = InputGroupCallId::new(12345, 67890);
        let id2 = id1;
        assert_eq!(id1, id2);
        assert_eq!(id1.group_call_id(), 12345);
    }

    #[test]

    #[test]
    fn test_display_format() {
        let call_id = InputGroupCallId::new(12345, 67890);
        assert_eq!(format!("{}", call_id), "group call 12345");

        let call_id = InputGroupCallId::new(0, 67890);
        assert_eq!(format!("{}", call_id), "group call 0");
    }

    #[test]
    fn test_from_tuple() {
        let call_id: InputGroupCallId = (12345, 67890).into();
        assert_eq!(call_id.group_call_id(), 12345);
        assert_eq!(call_id.access_hash(), 67890);
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashSet;
        let mut set = HashSet::with_hasher(InputGroupCallIdHash);

        let id1 = InputGroupCallId::new(12345, 67890);
        let id2 = InputGroupCallId::new(12345, 67890);
        let id3 = InputGroupCallId::new(99999, 11111);

        set.insert(id1);
        set.insert(id2);
        set.insert(id3);

        assert_eq!(set.len(), 2);
        assert!(set.contains(&id1));
        assert!(set.contains(&id3));
    }

    #[test]
    fn test_debug_format() {
        let call_id = InputGroupCallId::new(12345, 67890);
        let debug_str = format!("{:?}", call_id);
        assert!(debug_str.contains("InputGroupCallId"));
    }

    #[test]
    fn test_negative_values() {
        let call_id = InputGroupCallId::new(-12345, -67890);
        assert!(call_id.is_valid());
        assert_eq!(call_id.group_call_id(), -12345);
        assert_eq!(call_id.access_hash(), -67890);
    }

    #[test]
    fn test_large_values() {
        let call_id = InputGroupCallId::new(i64::MAX, i64::MAX);
        assert!(call_id.is_valid());
        assert_eq!(call_id.group_call_id(), i64::MAX);
        assert_eq!(call_id.access_hash(), i64::MAX);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let original = InputGroupCallId::new(1234567890, 9876543210);

        // Test JSON serialization
        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(
            json,
            r#"{"group_call_id":1234567890,"access_hash":9876543210}"#
        );

        let deserialized: InputGroupCallId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);

        // Test binary serialization
        let encoded = bincode::serialize(&original).unwrap();
        let decoded: InputGroupCallId = bincode::deserialize(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_default() {
        let original = InputGroupCallId::default();

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: InputGroupCallId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_negative() {
        let original = InputGroupCallId::new(-12345, -67890);

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: InputGroupCallId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
