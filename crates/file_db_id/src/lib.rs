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

//! # File DB ID
//!
//! Database identifier for files in TDLib.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `FileDbId` class from `td/telegram/files/FileDbId.h`.
//!
//! ## Usage
//!
//! ```
//! use rustgram_file_db_id::FileDbId;
//!
//! let id = FileDbId::new(42);
//! assert!(!id.is_empty());
//! assert!(id.is_valid());
//! assert_eq!(id.get(), 42);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// File database identifier.
///
/// Used to identify files in the database.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct FileDbId {
    /// The inner ID value
    id: u64,
}

impl FileDbId {
    /// Creates a new file DB ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID value (0 means empty/invalid)
    #[must_use]
    pub const fn new(id: u64) -> Self {
        Self { id }
    }

    /// Returns `true` if this ID is empty (ID == 0).
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.id == 0
    }

    /// Returns `true` if this ID is valid (ID > 0).
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.id > 0
    }

    /// Returns the inner ID value.
    #[must_use]
    pub const fn get(&self) -> u64 {
        self.id
    }
}

impl fmt::Display for FileDbId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileDbId{{{}}}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // === Constructor tests ===

    #[test]
    fn test_new() {
        let id = FileDbId::new(42);
        assert_eq!(id.get(), 42);
    }

    #[test]
    fn test_default() {
        let id = FileDbId::default();
        assert_eq!(id.get(), 0);
        assert!(id.is_empty());
        assert!(!id.is_valid());
    }

    // === Empty/Validity tests ===

    #[rstest]
    #[case(0, true, false)]
    #[case(1, false, true)]
    #[case(42, false, true)]
    #[case(u64::MAX, false, true)]
    fn test_empty_valid(#[case] id: u64, #[case] is_empty: bool, #[case] is_valid: bool) {
        let db_id = FileDbId::new(id);
        assert_eq!(db_id.is_empty(), is_empty);
        assert_eq!(db_id.is_valid(), is_valid);
    }

    // === Getter tests ===

    #[test]
    fn test_get() {
        let id = FileDbId::new(123);
        assert_eq!(id.get(), 123);
    }

    // === Comparison tests ===

    #[test]
    fn test_equality() {
        let id1 = FileDbId::new(42);
        let id2 = FileDbId::new(42);
        let id3 = FileDbId::new(43);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_ordering() {
        let id1 = FileDbId::new(10);
        let id2 = FileDbId::new(20);
        let id3 = FileDbId::new(20);

        assert!(id1 < id2);
        assert!(id2 <= id3);
        assert!(id2 >= id3);
        assert!(id2 > id1);
    }

    // === Display tests ===

    #[test]
    fn test_display() {
        let id = FileDbId::new(42);
        assert_eq!(format!("{id}"), "FileDbId{42}");
    }

    // === Serialization tests ===

    #[test]
    fn test_serialize() {
        let id = FileDbId::new(42);
        let json = serde_json::to_string(&id).unwrap();
        assert!(json.contains("42"));
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{"id":42}"#;
        let id: FileDbId = serde_json::from_str(json).unwrap();
        assert_eq!(id.get(), 42);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let original = FileDbId::new(123);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: FileDbId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // === Hash tests ===

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let id1 = FileDbId::new(42);
        let id2 = FileDbId::new(42);
        let id3 = FileDbId::new(43);

        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);
        set.insert(id3);

        assert_eq!(set.len(), 2); // id1 and id2 are the same
    }
}
