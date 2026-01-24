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

//! Fast file statistics.
//!
//! Contains `FileStatsFast` for quick statistics without per-dialog breakdown.

use serde::{Deserialize, Serialize};

/// Quick file statistics without per-dialog breakdown.
///
/// Provides aggregate statistics for storage usage without the overhead
/// of tracking individual dialog ownership. Corresponds to TDLib's `FileStatsFast`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FileStatsFast {
    /// Total size of all files in bytes.
    pub size: i64,
    /// Total count of all files.
    pub count: i32,
    /// Database size in bytes.
    pub database_size: i64,
    /// Language pack database size in bytes.
    pub language_pack_database_size: i64,
    /// Log size in bytes.
    pub log_size: i64,
}

impl FileStatsFast {
    /// Creates a new `FileStatsFast` with the specified values.
    ///
    /// # Arguments
    ///
    /// * `size` - Total size of all files in bytes
    /// * `count` - Total count of all files
    /// * `database_size` - Database size in bytes
    /// * `language_pack_database_size` - Language pack database size in bytes
    /// * `log_size` - Log size in bytes
    #[must_use]
    pub const fn new(
        size: i64,
        count: i32,
        database_size: i64,
        language_pack_database_size: i64,
        log_size: i64,
    ) -> Self {
        Self {
            size,
            count,
            database_size,
            language_pack_database_size,
            log_size,
        }
    }

    /// Creates a new empty `FileStatsFast`.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            size: 0,
            count: 0,
            database_size: 0,
            language_pack_database_size: 0,
            log_size: 0,
        }
    }

    /// Returns `true` if all values are zero.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
            && self.count == 0
            && self.database_size == 0
            && self.language_pack_database_size == 0
            && self.log_size == 0
    }

    /// Returns the total storage size including database and logs.
    ///
    /// This is the sum of file size, database size, language pack size, and log size.
    #[must_use]
    pub const fn total_storage_size(&self) -> i64 {
        self.size
            .saturating_add(self.database_size)
            .saturating_add(self.language_pack_database_size)
            .saturating_add(self.log_size)
    }

    /// Adds another `FileStatsFast` to this one using saturating arithmetic.
    #[must_use]
    pub const fn add(self, other: FileStatsFast) -> Self {
        Self {
            size: self.size.saturating_add(other.size),
            count: self.count.saturating_add(other.count),
            database_size: self.database_size.saturating_add(other.database_size),
            language_pack_database_size: self
                .language_pack_database_size
                .saturating_add(other.language_pack_database_size),
            log_size: self.log_size.saturating_add(other.log_size),
        }
    }

    /// Returns the database-related size (database + language pack).
    #[must_use]
    pub const fn database_total_size(&self) -> i64 {
        self.database_size
            .saturating_add(self.language_pack_database_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Construction tests ===

    #[test]
    fn test_new() {
        let stats = FileStatsFast::new(1_000_000, 100, 50_000, 10_000, 5_000);
        assert_eq!(stats.size, 1_000_000);
        assert_eq!(stats.count, 100);
        assert_eq!(stats.database_size, 50_000);
        assert_eq!(stats.language_pack_database_size, 10_000);
        assert_eq!(stats.log_size, 5_000);
    }

    #[test]
    fn test_default() {
        let stats = FileStatsFast::default();
        assert_eq!(stats.size, 0);
        assert_eq!(stats.count, 0);
        assert_eq!(stats.database_size, 0);
        assert_eq!(stats.language_pack_database_size, 0);
        assert_eq!(stats.log_size, 0);
    }

    #[test]
    fn test_empty() {
        let stats = FileStatsFast::empty();
        assert!(stats.is_empty());
        assert_eq!(stats.size, 0);
        assert_eq!(stats.count, 0);
    }

    #[test]
    fn test_is_empty_true() {
        let stats = FileStatsFast::new(0, 0, 0, 0, 0);
        assert!(stats.is_empty());
    }

    #[test]
    fn test_is_empty_false_by_size() {
        let stats = FileStatsFast::new(1, 0, 0, 0, 0);
        assert!(!stats.is_empty());
    }

    #[test]
    fn test_is_empty_false_by_count() {
        let stats = FileStatsFast::new(0, 1, 0, 0, 0);
        assert!(!stats.is_empty());
    }

    #[test]
    fn test_is_empty_false_by_database() {
        let stats = FileStatsFast::new(0, 0, 1, 0, 0);
        assert!(!stats.is_empty());
    }

    #[test]
    fn test_is_empty_false_by_language_pack() {
        let stats = FileStatsFast::new(0, 0, 0, 1, 0);
        assert!(!stats.is_empty());
    }

    #[test]
    fn test_is_empty_false_by_log() {
        let stats = FileStatsFast::new(0, 0, 0, 0, 1);
        assert!(!stats.is_empty());
    }

    // === Calculation tests ===

    #[test]
    fn test_total_storage_size() {
        let stats = FileStatsFast::new(1_000_000, 100, 50_000, 10_000, 5_000);
        let total = stats.total_storage_size();
        assert_eq!(total, 1_065_000);
    }

    #[test]
    fn test_total_storage_size_empty() {
        let stats = FileStatsFast::empty();
        assert_eq!(stats.total_storage_size(), 0);
    }

    #[test]
    fn test_total_storage_size_files_only() {
        let stats = FileStatsFast::new(1_000_000, 100, 0, 0, 0);
        assert_eq!(stats.total_storage_size(), 1_000_000);
    }

    #[test]
    fn test_database_total_size() {
        let stats = FileStatsFast::new(1_000_000, 100, 50_000, 10_000, 5_000);
        assert_eq!(stats.database_total_size(), 60_000);
    }

    #[test]
    fn test_database_total_size_no_language_pack() {
        let stats = FileStatsFast::new(1_000_000, 100, 50_000, 0, 5_000);
        assert_eq!(stats.database_total_size(), 50_000);
    }

    #[test]
    fn test_database_total_size_empty() {
        let stats = FileStatsFast::empty();
        assert_eq!(stats.database_total_size(), 0);
    }

    // === Addition tests ===

    #[test]
    fn test_add() {
        let stats1 = FileStatsFast::new(1_000_000, 100, 50_000, 10_000, 5_000);
        let stats2 = FileStatsFast::new(500_000, 50, 25_000, 5_000, 2_500);
        let result = stats1.add(stats2);
        assert_eq!(result.size, 1_500_000);
        assert_eq!(result.count, 150);
        assert_eq!(result.database_size, 75_000);
        assert_eq!(result.language_pack_database_size, 15_000);
        assert_eq!(result.log_size, 7_500);
    }

    #[test]
    fn test_add_empty() {
        let stats1 = FileStatsFast::new(1_000_000, 100, 50_000, 10_000, 5_000);
        let stats2 = FileStatsFast::empty();
        let result = stats1.add(stats2);
        assert_eq!(result.size, 1_000_000);
        assert_eq!(result.count, 100);
    }

    #[test]
    fn test_add_multiple() {
        let mut stats = FileStatsFast::empty();
        stats = stats.add(FileStatsFast::new(100, 1, 10, 1, 1));
        stats = stats.add(FileStatsFast::new(200, 2, 20, 2, 2));
        stats = stats.add(FileStatsFast::new(300, 3, 30, 3, 3));
        assert_eq!(stats.size, 600);
        assert_eq!(stats.count, 6);
        assert_eq!(stats.database_size, 60);
        assert_eq!(stats.language_pack_database_size, 6);
        assert_eq!(stats.log_size, 6);
    }

    // === Saturating arithmetic tests ===

    #[test]
    fn test_add_size_overflow() {
        let stats1 = FileStatsFast::new(i64::MAX, 0, 0, 0, 0);
        let stats2 = FileStatsFast::new(1, 0, 0, 0, 0);
        let result = stats1.add(stats2);
        assert_eq!(result.size, i64::MAX);
    }

    #[test]
    fn test_add_count_overflow() {
        let stats1 = FileStatsFast::new(0, i32::MAX, 0, 0, 0);
        let stats2 = FileStatsFast::new(0, 1, 0, 0, 0);
        let result = stats1.add(stats2);
        assert_eq!(result.count, i32::MAX);
    }

    #[test]
    fn test_total_storage_size_overflow() {
        let stats = FileStatsFast::new(i64::MAX, 0, 1, 0, 0);
        let total = stats.total_storage_size();
        assert_eq!(total, i64::MAX);
    }

    // === Serialization tests ===

    #[test]
    fn test_serialize() {
        let stats = FileStatsFast::new(1_000_000, 100, 50_000, 10_000, 5_000);
        let json = serde_json::to_string(&stats).expect("Failed to serialize");
        assert!(json.contains("1000000"));
        assert!(json.contains("100"));
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{"size":1000000,"count":100,"database_size":50000,"language_pack_database_size":10000,"log_size":5000}"#;
        let stats: FileStatsFast = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(stats.size, 1_000_000);
        assert_eq!(stats.count, 100);
    }

    #[test]
    fn test_serialize_roundtrip() {
        let original = FileStatsFast::new(999999, 42, 12345, 6789, 555);
        let json = serde_json::to_string(&original).expect("Failed to serialize");
        let deserialized: FileStatsFast =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(original, deserialized);
    }

    // === Equality tests ===

    #[test]
    fn test_equality_true() {
        let stats1 = FileStatsFast::new(1_000_000, 100, 50_000, 10_000, 5_000);
        let stats2 = FileStatsFast::new(1_000_000, 100, 50_000, 10_000, 5_000);
        assert_eq!(stats1, stats2);
    }

    #[test]
    fn test_equality_false_size() {
        let stats1 = FileStatsFast::new(1_000_000, 100, 50_000, 10_000, 5_000);
        let stats2 = FileStatsFast::new(2_000_000, 100, 50_000, 10_000, 5_000);
        assert_ne!(stats1, stats2);
    }

    #[test]
    fn test_equality_false_count() {
        let stats1 = FileStatsFast::new(1_000_000, 100, 50_000, 10_000, 5_000);
        let stats2 = FileStatsFast::new(1_000_000, 200, 50_000, 10_000, 5_000);
        assert_ne!(stats1, stats2);
    }

    #[test]
    fn test_equality_false_database() {
        let stats1 = FileStatsFast::new(1_000_000, 100, 50_000, 10_000, 5_000);
        let stats2 = FileStatsFast::new(1_000_000, 100, 60_000, 10_000, 5_000);
        assert_ne!(stats1, stats2);
    }

    // === Clone tests ===

    #[test]
    fn test_clone() {
        let stats1 = FileStatsFast::new(1_000_000, 100, 50_000, 10_000, 5_000);
        let stats2 = stats1;
        assert_eq!(stats1, stats2);
    }

    // === Debug tests ===

    #[test]
    fn test_debug_format() {
        let stats = FileStatsFast::new(1_000_000, 100, 50_000, 10_000, 5_000);
        let debug = format!("{stats:?}");
        assert!(debug.contains("1000000"));
        assert!(debug.contains("100"));
    }

    // === Real-world values tests ===

    #[test]
    fn test_realistic_storage_values() {
        // Typical storage: 500MB files, 10MB database, 5MB language pack, 2MB logs
        let stats = FileStatsFast::new(500_000_000, 500, 10_000_000, 5_000_000, 2_000_000);
        assert_eq!(stats.total_storage_size(), 517_000_000);
        assert_eq!(stats.database_total_size(), 15_000_000);
    }

    #[test]
    fn test_large_storage_values() {
        // Large storage: 10GB files, 500MB database, 100MB language pack, 50MB logs
        let stats = FileStatsFast::new(10_000_000_000, 10000, 500_000_000, 100_000_000, 50_000_000);
        assert_eq!(stats.total_storage_size(), 10_650_000_000);
    }
}
