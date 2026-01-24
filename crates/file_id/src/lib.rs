// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # File ID
//!
//! File identifier with remote ID support.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `FileId` class from `td/telegram/files/FileId.h`.
//!
//! ## Structure
//!
//! - `id`: Local file identifier (int32)
//! - `remote_id`: Remote file identifier (int32)
//!
//! ## Usage
//!
//! ```
//! use rustgram_file_id::FileId;
//!
//! // Create a new FileId
//! let file_id = FileId::new(123, 456);
//! assert_eq!(file_id.get(), 123);
//! assert_eq!(file_id.get_remote(), 456);
//! assert!(file_id.is_valid());
//!
//! // Empty FileId
//! let empty = FileId::empty();
//! assert!(empty.is_empty());
//! assert!(!empty.is_valid());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

/// File identifier with local and remote ID components.
///
/// Corresponds to TDLib `FileId` class.
/// Note: Equality and ordering only consider the `id` field, matching TDLib behavior.
#[derive(Debug, Clone, Copy, Eq, Serialize, Deserialize)]
pub struct FileId {
    /// Local file identifier. Must be > 0 for valid files.
    id: i32,
    /// Remote file identifier.
    remote_id: i32,
}

impl PartialEq for FileId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl FileId {
    /// Creates a new FileId with the given local and remote IDs.
    ///
    /// # Arguments
    ///
    /// * `id` - Local file identifier (use positive value for valid files)
    /// * `remote_id` - Remote file identifier
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_id::FileId;
    ///
    /// let file_id = FileId::new(123, 456);
    /// assert_eq!(file_id.get(), 123);
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(id: i32, remote_id: i32) -> Self {
        Self { id, remote_id }
    }

    /// Creates an empty FileId (id = 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_id::FileId;
    ///
    /// let empty = FileId::empty();
    /// assert!(empty.is_empty());
    /// ```
    #[must_use]
    #[inline]
    pub const fn empty() -> Self {
        Self {
            id: 0,
            remote_id: 0,
        }
    }

    /// Returns `true` if the file ID is empty (id <= 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_id::FileId;
    ///
    /// assert!(FileId::empty().is_empty());
    /// assert!(FileId::new(0, 0).is_empty());
    /// assert!(FileId::new(-1, 0).is_empty());
    /// assert!(!FileId::new(1, 0).is_empty());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.id <= 0
    }

    /// Returns `true` if the file ID is valid (id > 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_id::FileId;
    ///
    /// assert!(!FileId::empty().is_valid());
    /// assert!(!FileId::new(0, 0).is_valid());
    /// assert!(FileId::new(1, 0).is_valid());
    /// assert!(FileId::new(100, 200).is_valid());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.id > 0
    }

    /// Returns the local file identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_id::FileId;
    ///
    /// let file_id = FileId::new(123, 456);
    /// assert_eq!(file_id.get(), 123);
    /// ```
    #[must_use]
    #[inline]
    pub const fn get(&self) -> i32 {
        self.id
    }

    /// Returns the remote file identifier.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_file_id::FileId;
    ///
    /// let file_id = FileId::new(123, 456);
    /// assert_eq!(file_id.get_remote(), 456);
    /// ```
    #[must_use]
    #[inline]
    pub const fn get_remote(&self) -> i32 {
        self.remote_id
    }
}

impl Default for FileId {
    fn default() -> Self {
        Self::empty()
    }
}

impl PartialOrd for FileId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FileId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for FileId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.id, self.remote_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use rstest::rstest;

    // === Basic trait tests ===

    #[test]
    fn test_clone() {
        let file_id = FileId::new(123, 456);
        let cloned = file_id;
        assert_eq!(file_id, cloned);
    }

    #[test]
    fn test_copy() {
        let file_id = FileId::new(123, 456);
        let copied = file_id;
        assert_eq!(file_id, copied);
    }

    #[test]
    fn test_default() {
        let file_id = FileId::default();
        assert!(file_id.is_empty());
        assert_eq!(file_id.get(), 0);
        assert_eq!(file_id.get_remote(), 0);
    }

    // === Constructor tests ===

    #[rstest]
    #[case(1, 0)]
    #[case(100, 200)]
    #[case(i32::MAX, i32::MAX)]
    fn test_new_valid(#[case] id: i32, #[case] remote_id: i32) {
        let file_id = FileId::new(id, remote_id);
        assert_eq!(file_id.get(), id);
        assert_eq!(file_id.get_remote(), remote_id);
        assert!(file_id.is_valid());
        assert!(!file_id.is_empty());
    }

    #[rstest]
    #[case(0, 0)]
    #[case(-1, 0)]
    #[case(-100, 200)]
    #[case(i32::MIN, 0)]
    fn test_new_invalid(#[case] id: i32, #[case] remote_id: i32) {
        let file_id = FileId::new(id, remote_id);
        assert_eq!(file_id.get(), id);
        assert_eq!(file_id.get_remote(), remote_id);
        assert!(!file_id.is_valid());
        assert!(file_id.is_empty());
    }

    #[test]
    fn test_empty() {
        let empty = FileId::empty();
        assert!(empty.is_empty());
        assert!(!empty.is_valid());
        assert_eq!(empty.get(), 0);
        assert_eq!(empty.get_remote(), 0);
    }

    // === Validation tests ===

    #[rstest]
    #[case(0, false)]
    #[case(-1, false)]
    #[case(1, true)]
    #[case(100, true)]
    #[case(i32::MAX, true)]
    fn test_is_valid(#[case] id: i32, #[case] expected: bool) {
        let file_id = FileId::new(id, 0);
        assert_eq!(file_id.is_valid(), expected);
    }

    #[rstest]
    #[case(0, true)]
    #[case(-1, true)]
    #[case(1, false)]
    #[case(100, false)]
    #[case(i32::MAX, false)]
    fn test_is_empty(#[case] id: i32, #[case] expected: bool) {
        let file_id = FileId::new(id, 0);
        assert_eq!(file_id.is_empty(), expected);
    }

    // === Getter tests ===

    #[rstest]
    #[case(123, 456)]
    #[case(0, 0)]
    #[case(-1, -2)]
    #[case(i32::MAX, i32::MIN)]
    fn test_getters(#[case] id: i32, #[case] remote_id: i32) {
        let file_id = FileId::new(id, remote_id);
        assert_eq!(file_id.get(), id);
        assert_eq!(file_id.get_remote(), remote_id);
    }

    // === Comparison tests ===

    #[test]
    fn test_equality() {
        let a = FileId::new(123, 456);
        let b = FileId::new(123, 456);
        let c = FileId::new(124, 456);
        let d = FileId::new(123, 457);

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a, d); // Equality only checks id
    }

    #[test]
    fn test_ordering() {
        let a = FileId::new(100, 0);
        let b = FileId::new(200, 0);
        let c = FileId::new(150, 0);

        assert!(a < b);
        assert!(b > a);
        assert!(a < c);
        assert!(c < b);
    }

    // === Display tests ===

    #[rstest]
    #[case(123, 456, "123(456)")]
    #[case(0, 0, "0(0)")]
    #[case(-1, 0, "-1(0)")]
    fn test_display(#[case] id: i32, #[case] remote_id: i32, #[case] expected: &str) {
        let file_id = FileId::new(id, remote_id);
        assert_eq!(format!("{file_id}"), expected);
    }

    #[test]
    fn test_debug() {
        let file_id = FileId::new(123, 456);
        assert!(format!("{file_id:?}").contains("FileId"));
        assert!(format!("{file_id:?}").contains("123"));
        assert!(format!("{file_id:?}").contains("456"));
    }

    // === Hash tests ===

    #[test]
    fn test_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;

        let a = FileId::new(123, 456);
        let b = FileId::new(123, 789);

        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();

        a.hash(&mut hasher_a);
        b.hash(&mut hasher_b);

        // Hash only depends on id, not remote_id
        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    // === Serialization tests ===

    #[test]
    fn test_serialize_json() {
        let file_id = FileId::new(123, 456);
        let json = serde_json::to_string(&file_id).unwrap();
        assert!(json.contains("\"id\":123") || json.contains("\"id\": 123"));
        assert!(json.contains("\"remote_id\":456") || json.contains("\"remote_id\": 456"));
    }

    #[test]
    fn test_deserialize_json() {
        let json = r#"{"id":123,"remote_id":456}"#;
        let file_id: FileId = serde_json::from_str(json).unwrap();
        assert_eq!(file_id.get(), 123);
        assert_eq!(file_id.get_remote(), 456);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let original = FileId::new(123, 456);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: FileId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // === Property-based tests ===

    proptest! {
        #[test]
        fn test_valid_id_is_not_empty(id in 1i32.., remote_id in i32::MIN..) {
            let file_id = FileId::new(id, remote_id);
            prop_assert!(file_id.is_valid());
            prop_assert!(!file_id.is_empty());
        }

        #[test]
        fn test_invalid_or_zero_id_is_empty(id in -100i32..=0, remote_id in i32::MIN..) {
            let file_id = FileId::new(id, remote_id);
            prop_assert!(file_id.is_empty());
            prop_assert!(!file_id.is_valid());
        }

        #[test]
        fn test_getters_return_input(id in i32::MIN.., remote_id in i32::MIN..) {
            let file_id = FileId::new(id, remote_id);
            prop_assert_eq!(file_id.get(), id);
            prop_assert_eq!(file_id.get_remote(), remote_id);
        }

        #[test]
        fn test_ordering_is_consistent(a in 1i32.., b in 1i32..) {
            let file_a = FileId::new(a, 0);
            let file_b = FileId::new(b, 0);
            prop_assert_eq!(file_a < file_b, a < b);
        }
    }
}
