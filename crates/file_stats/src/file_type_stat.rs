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

//! File type statistics.
//!
//! Contains `FileTypeStat` which tracks size and count statistics for a file type.

use serde::{Deserialize, Serialize};

/// Statistics for a single file type.
///
/// Tracks the total size and count of files of a particular type.
/// Corresponds to TDLib's `FileTypeStat` struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FileTypeStat {
    /// Total size in bytes of all files of this type.
    pub size: i64,
    /// Number of files of this type.
    pub cnt: i32,
}

impl FileTypeStat {
    /// Creates a new `FileTypeStat` with the given size and count.
    #[must_use]
    pub const fn new(size: i64, cnt: i32) -> Self {
        Self { size, cnt }
    }

    /// Creates a new empty `FileTypeStat`.
    #[must_use]
    pub const fn empty() -> Self {
        Self { size: 0, cnt: 0 }
    }

    /// Adds another `FileTypeStat` to this one using saturating arithmetic.
    ///
    /// # Arguments
    ///
    /// * `other` - The statistics to add
    ///
    /// # Returns
    ///
    /// A new `FileTypeStat` with the summed values.
    #[must_use]
    pub const fn add(self, other: FileTypeStat) -> Self {
        Self {
            size: self.size.saturating_add(other.size),
            cnt: self.cnt.saturating_add(other.cnt),
        }
    }

    /// Adds a file size and count to this stat using saturating arithmetic.
    ///
    /// # Arguments
    ///
    /// * `size` - The file size in bytes to add
    /// * `cnt` - The count to add (typically 1)
    ///
    /// # Returns
    ///
    /// A new `FileTypeStat` with the added values.
    #[must_use]
    pub const fn add_file(self, size: i64, cnt: i32) -> Self {
        Self {
            size: self.size.saturating_add(size),
            cnt: self.cnt.saturating_add(cnt),
        }
    }

    /// Returns `true` if both size and count are zero.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.size == 0 && self.cnt == 0
    }

    /// Subtracts another `FileTypeStat` from this one using saturating arithmetic.
    ///
    /// # Arguments
    ///
    /// * `other` - The statistics to subtract
    ///
    /// # Returns
    ///
    /// A new `FileTypeStat` with the subtracted values (floored at zero).
    #[must_use]
    pub const fn sub(self, other: FileTypeStat) -> Self {
        Self {
            size: self.size.saturating_sub(other.size),
            cnt: self.cnt.saturating_sub(other.cnt),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Construction tests ===

    #[test]
    fn test_new() {
        let stat = FileTypeStat::new(1024, 5);
        assert_eq!(stat.size, 1024);
        assert_eq!(stat.cnt, 5);
    }

    #[test]
    fn test_default() {
        let stat = FileTypeStat::default();
        assert_eq!(stat.size, 0);
        assert_eq!(stat.cnt, 0);
    }

    #[test]
    fn test_empty() {
        let stat = FileTypeStat::empty();
        assert_eq!(stat.size, 0);
        assert_eq!(stat.cnt, 0);
        assert!(stat.is_empty());
    }

    #[test]
    fn test_is_empty_true() {
        let stat = FileTypeStat::new(0, 0);
        assert!(stat.is_empty());
    }

    #[test]
    fn test_is_empty_false_by_size() {
        let stat = FileTypeStat::new(100, 0);
        assert!(!stat.is_empty());
    }

    #[test]
    fn test_is_empty_false_by_count() {
        let stat = FileTypeStat::new(0, 1);
        assert!(!stat.is_empty());
    }

    // === Addition tests ===

    #[test]
    fn test_add() {
        let stat1 = FileTypeStat::new(1000, 10);
        let stat2 = FileTypeStat::new(500, 5);
        let result = stat1.add(stat2);
        assert_eq!(result.size, 1500);
        assert_eq!(result.cnt, 15);
    }

    #[test]
    fn test_add_empty() {
        let stat1 = FileTypeStat::new(1000, 10);
        let stat2 = FileTypeStat::empty();
        let result = stat1.add(stat2);
        assert_eq!(result.size, 1000);
        assert_eq!(result.cnt, 10);
    }

    #[test]
    fn test_add_file() {
        let stat = FileTypeStat::new(1000, 10);
        let result = stat.add_file(500, 1);
        assert_eq!(result.size, 1500);
        assert_eq!(result.cnt, 11);
    }

    #[test]
    fn test_add_multiple_files() {
        let mut stat = FileTypeStat::empty();
        stat = stat.add_file(100, 1);
        stat = stat.add_file(200, 1);
        stat = stat.add_file(300, 1);
        assert_eq!(stat.size, 600);
        assert_eq!(stat.cnt, 3);
    }

    // === Saturating arithmetic tests ===

    #[test]
    fn test_add_size_overflow() {
        let stat1 = FileTypeStat::new(i64::MAX, 0);
        let stat2 = FileTypeStat::new(1, 0);
        let result = stat1.add(stat2);
        assert_eq!(result.size, i64::MAX);
    }

    #[test]
    fn test_add_count_overflow() {
        let stat1 = FileTypeStat::new(0, i32::MAX);
        let stat2 = FileTypeStat::new(0, 1);
        let result = stat1.add(stat2);
        assert_eq!(result.cnt, i32::MAX);
    }

    // === Subtraction tests ===

    #[test]
    fn test_sub() {
        let stat1 = FileTypeStat::new(1500, 15);
        let stat2 = FileTypeStat::new(500, 5);
        let result = stat1.sub(stat2);
        assert_eq!(result.size, 1000);
        assert_eq!(result.cnt, 10);
    }

    #[test]
    fn test_sub_produces_negative_for_signed() {
        // For signed integers, saturating_sub produces negative values
        // (saturates at i64::MIN/i32::MIN, not at 0)
        let stat1 = FileTypeStat::new(100, 5);
        let stat2 = FileTypeStat::new(200, 10);
        let result = stat1.sub(stat2);
        assert_eq!(result.size, -100);
        assert_eq!(result.cnt, -5);
    }

    #[test]
    fn test_sub_partial_size() {
        let stat1 = FileTypeStat::new(1500, 10);
        let stat2 = FileTypeStat::new(500, 10);
        let result = stat1.sub(stat2);
        assert_eq!(result.size, 1000);
        assert_eq!(result.cnt, 0);
    }

    // === Serialization tests ===

    #[test]
    fn test_serialize() {
        let stat = FileTypeStat::new(1024, 5);
        let json = serde_json::to_string(&stat).expect("Failed to serialize");
        assert!(json.contains("1024"));
        assert!(json.contains("5"));
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{"size":1024,"cnt":5}"#;
        let stat: FileTypeStat = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(stat.size, 1024);
        assert_eq!(stat.cnt, 5);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let original = FileTypeStat::new(999999, 42);
        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: FileTypeStat =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }

    // === Clone tests ===

    #[test]
    fn test_clone() {
        let stat1 = FileTypeStat::new(1024, 5);
        let stat2 = stat1;
        assert_eq!(stat1, stat2);
    }

    // === PartialEq tests ===

    #[test]
    fn test_equality_true() {
        let stat1 = FileTypeStat::new(1024, 5);
        let stat2 = FileTypeStat::new(1024, 5);
        assert_eq!(stat1, stat2);
    }

    #[test]
    fn test_equality_false_size() {
        let stat1 = FileTypeStat::new(1024, 5);
        let stat2 = FileTypeStat::new(2048, 5);
        assert_ne!(stat1, stat2);
    }

    #[test]
    fn test_equality_false_count() {
        let stat1 = FileTypeStat::new(1024, 5);
        let stat2 = FileTypeStat::new(1024, 10);
        assert_ne!(stat1, stat2);
    }

    // === Debug tests ===

    #[test]
    fn test_debug_format() {
        let stat = FileTypeStat::new(1024, 5);
        let debug = format!("{stat:?}");
        assert!(debug.contains("1024"));
        assert!(debug.contains("5"));
    }

    // === Chain operations tests ===

    #[test]
    fn test_chained_operations() {
        let stat = FileTypeStat::empty()
            .add_file(100, 1)
            .add_file(200, 1)
            .add_file(300, 1)
            .sub(FileTypeStat::new(150, 0));
        assert_eq!(stat.size, 450);
        assert_eq!(stat.cnt, 3);
    }

    #[test]
    fn test_accumulate_multiple_stats() {
        let stats = [
            FileTypeStat::new(100, 1),
            FileTypeStat::new(200, 1),
            FileTypeStat::new(300, 1),
        ];
        let mut result = FileTypeStat::empty();
        for stat in stats {
            result = result.add(stat);
        }
        assert_eq!(result.size, 600);
        assert_eq!(result.cnt, 3);
    }
}
