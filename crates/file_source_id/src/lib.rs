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

//! # File Source ID
//!
//! Identifier for file download sources in TDLib.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `FileSourceId` class from `td/telegram/files/FileSourceId.h`.
//!
//! ## Usage
//!
//! ```
//! use rustgram_file_source_id::FileSourceId;
//!
//! let id = FileSourceId::new(42);
//! assert!(id.is_valid());
//! assert_eq!(id.get(), 42);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// File source identifier.
///
/// Used to track the source of a file download.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct FileSourceId {
    /// The inner ID value
    id: i32,
}

impl FileSourceId {
    /// Creates a new file source ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID value (must be > 0 to be valid)
    #[must_use]
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

    /// Returns `true` if this ID is valid (ID > 0).
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.id > 0
    }

    /// Returns the inner ID value.
    #[must_use]
    pub const fn get(&self) -> i32 {
        self.id
    }
}

impl fmt::Display for FileSourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileSourceId({})", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // === Constructor tests ===

    #[test]
    fn test_new() {
        let id = FileSourceId::new(42);
        assert_eq!(id.get(), 42);
    }

    #[test]
    fn test_default() {
        let id = FileSourceId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    // === Validity tests ===

    #[rstest]
    #[case(0, false)]
    #[case(-1, false)]
    #[case(1, true)]
    #[case(42, true)]
    #[case(i32::MAX, true)]
    fn test_is_valid(#[case] id: i32, #[case] expected: bool) {
        let source_id = FileSourceId::new(id);
        assert_eq!(source_id.is_valid(), expected);
    }

    // === Getter tests ===

    #[test]
    fn test_get() {
        let id = FileSourceId::new(123);
        assert_eq!(id.get(), 123);
    }

    // === Comparison tests ===

    #[test]
    fn test_equality() {
        let id1 = FileSourceId::new(42);
        let id2 = FileSourceId::new(42);
        let id3 = FileSourceId::new(43);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_ordering() {
        let id1 = FileSourceId::new(10);
        let id2 = FileSourceId::new(20);
        let id3 = FileSourceId::new(20);

        assert!(id1 < id2);
        assert!(id2 <= id3);
        assert!(id2 >= id3);
        assert!(id2 > id1);
    }

    // === Display tests ===

    #[test]
    fn test_display() {
        let id = FileSourceId::new(42);
        assert_eq!(format!("{id}"), "FileSourceId(42)");
    }

    // === Serialization tests ===

    #[test]
    fn test_serialize() {
        let id = FileSourceId::new(42);
        let json = serde_json::to_string(&id).unwrap();
        assert!(json.contains("42"));
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{"id":42}"#;
        let id: FileSourceId = serde_json::from_str(json).unwrap();
        assert_eq!(id.get(), 42);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let original = FileSourceId::new(123);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: FileSourceId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    // === Hash tests ===

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let id1 = FileSourceId::new(42);
        let id2 = FileSourceId::new(42);
        let id3 = FileSourceId::new(43);

        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);
        set.insert(id3);

        assert_eq!(set.len(), 2); // id1 and id2 are the same
    }
}
