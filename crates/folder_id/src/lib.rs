//! Folder identifier.
//!
//! This module provides the `FolderId` type, which identifies a folder in Telegram.
//! Folders are used to organize chats into main and archive folders.
//!
//! # TDLib Alignment
//!
//! - TDLib type: `FolderId` (td/telegram/FolderId.h)
//! - Valid values: 0 (main) or 1 (archive)
//! - Any other value is normalized to 0 (main)
//!
//! # Example
//!
//! ```rust
//! use rustgram_folder_id::FolderId;
//!
//! // Main folder
//! let main = FolderId::main();
//! assert_eq!(main.get(), 0);
//!
//! // Archive folder
//! let archive = FolderId::archive();
//! assert_eq!(archive.get(), 1);
//!
//! // Invalid values are normalized to main
//! let invalid = FolderId::new(5);
//! assert_eq!(invalid.get(), 0);
//! ```

use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use std::hash::Hash;

/// Folder identifier.
///
/// Represents a folder in Telegram. Only two valid values exist:
/// - 0: Main folder
/// - 1: Archive folder
///
/// Any other value is automatically normalized to 0 (main folder).
///
/// # Example
///
/// ```rust
/// use rustgram_folder_id::FolderId;
///
/// let main = FolderId::main();
/// assert_eq!(main.get(), 0);
///
/// let archive = FolderId::archive();
/// assert_eq!(archive.get(), 1);
///
/// // Invalid values are normalized
/// let invalid = FolderId::new(99);
/// assert_eq!(invalid.get(), 0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FolderId(pub i32);

impl FolderId {
    /// Main folder ID.
    pub const MAIN: i32 = 0;

    /// Archive folder ID.
    pub const ARCHIVE: i32 = 1;

    /// Creates a new FolderId.
    ///
    /// Only values 0 (main) and 1 (archive) are valid.
    /// Any other value is normalized to 0 (main).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_folder_id::FolderId;
    ///
    /// let main = FolderId::new(0);
    /// assert_eq!(main.get(), 0);
    ///
    /// let archive = FolderId::new(1);
    /// assert_eq!(archive.get(), 1);
    ///
    /// let invalid = FolderId::new(5);
    /// assert_eq!(invalid.get(), 0); // normalized to main
    /// ```
    pub fn new(id: i32) -> Self {
        if id == Self::ARCHIVE {
            Self(id)
        } else {
            // Everything else (including MAIN) becomes MAIN
            // This matches TDLib behavior
            Self(Self::MAIN)
        }
    }

    /// Returns the inner i32 value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_folder_id::FolderId;
    ///
    /// let archive = FolderId::archive();
    /// assert_eq!(archive.get(), 1);
    /// ```
    #[inline]
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Checks if this is the main folder.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_folder_id::FolderId;
    ///
    /// let main = FolderId::main();
    /// assert!(main.is_main());
    /// assert!(!main.is_archive());
    /// ```
    #[inline]
    pub const fn is_main(self) -> bool {
        self.0 == Self::MAIN
    }

    /// Checks if this is the archive folder.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_folder_id::FolderId;
    ///
    /// let archive = FolderId::archive();
    /// assert!(archive.is_archive());
    /// assert!(!archive.is_main());
    /// ```
    #[inline]
    pub const fn is_archive(self) -> bool {
        self.0 == Self::ARCHIVE
    }

    /// Returns the main folder ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_folder_id::FolderId;
    ///
    /// let main = FolderId::main();
    /// assert_eq!(main.get(), 0);
    /// assert!(main.is_main());
    /// ```
    pub fn main() -> Self {
        Self(Self::MAIN)
    }

    /// Returns the archive folder ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_folder_id::FolderId;
    ///
    /// let archive = FolderId::archive();
    /// assert_eq!(archive.get(), 1);
    /// assert!(archive.is_archive());
    /// ```
    pub fn archive() -> Self {
        Self(Self::ARCHIVE)
    }
}

impl Default for FolderId {
    fn default() -> Self {
        Self::main()
    }
}

impl fmt::Display for FolderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "folder {}", self.0)
    }
}

impl Serialize for FolderId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FolderId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = i32::deserialize(deserializer)?;
        Ok(FolderId::new(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (10)
    #[test]
    fn test_debug() {
        let folder_id = FolderId::main();
        assert_eq!(format!("{:?}", folder_id), "FolderId(0)");
    }

    #[test]
    fn test_clone() {
        let folder_id = FolderId::archive();
        let cloned = folder_id;
        assert_eq!(folder_id, cloned);
    }

    #[test]
    fn test_copy() {
        let folder_id = FolderId::main();
        let copied = folder_id;
        assert_eq!(folder_id, copied);
    }

    #[test]
    fn test_partial_eq() {
        let id1 = FolderId::main();
        let id2 = FolderId::main();
        let id3 = FolderId::archive();
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        let id1 = FolderId::main();
        let id2 = FolderId::main();
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        id1.hash(&mut h1);
        id2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_display_main() {
        let folder_id = FolderId::main();
        assert_eq!(format!("{}", folder_id), "folder 0");
    }

    #[test]
    fn test_display_archive() {
        let folder_id = FolderId::archive();
        assert_eq!(format!("{}", folder_id), "folder 1");
    }

    #[test]
    fn test_default() {
        let folder_id = FolderId::default();
        assert_eq!(folder_id, FolderId::main());
    }

    #[test]
    fn test_const_main() {
        assert_eq!(FolderId::MAIN, 0);
    }

    #[test]
    fn test_const_archive() {
        assert_eq!(FolderId::ARCHIVE, 1);
    }

    // Constructor tests (3 constructors * 3 tests = 9)
    #[test]
    fn test_new_main() {
        let folder_id = FolderId::new(0);
        assert_eq!(folder_id.get(), 0);
    }

    #[test]
    fn test_new_archive() {
        let folder_id = FolderId::new(1);
        assert_eq!(folder_id.get(), 1);
    }

    #[test]
    fn test_new_normalizes_invalid() {
        let folder_id = FolderId::new(5);
        assert_eq!(folder_id.get(), 0);
    }

    #[test]
    fn test_new_normalizes_negative() {
        let folder_id = FolderId::new(-1);
        assert_eq!(folder_id.get(), 0);
    }

    #[test]
    fn test_new_normalizes_large() {
        let folder_id = FolderId::new(999);
        assert_eq!(folder_id.get(), 0);
    }

    #[test]
    fn test_main_static() {
        let folder_id = FolderId::main();
        assert_eq!(folder_id.get(), 0);
        assert!(folder_id.is_main());
    }

    #[test]
    fn test_archive_static() {
        let folder_id = FolderId::archive();
        assert_eq!(folder_id.get(), 1);
        assert!(folder_id.is_archive());
    }

    #[test]
    fn test_main_vs_archive() {
        let main = FolderId::main();
        let archive = FolderId::archive();
        assert_ne!(main, archive);
    }

    // Method tests (5 methods * 3 tests = 15)
    #[test]
    fn test_get_main() {
        let folder_id = FolderId::main();
        assert_eq!(folder_id.get(), 0);
    }

    #[test]
    fn test_get_archive() {
        let folder_id = FolderId::archive();
        assert_eq!(folder_id.get(), 1);
    }

    #[test]
    fn test_is_main_true() {
        let folder_id = FolderId::main();
        assert!(folder_id.is_main());
    }

    #[test]
    fn test_is_main_false() {
        let folder_id = FolderId::archive();
        assert!(!folder_id.is_main());
    }

    #[test]
    fn test_is_archive_true() {
        let folder_id = FolderId::archive();
        assert!(folder_id.is_archive());
    }

    #[test]
    fn test_is_archive_false() {
        let folder_id = FolderId::main();
        assert!(!folder_id.is_archive());
    }

    #[test]
    fn test_main_static_returns_main() {
        let folder_id = FolderId::main();
        assert!(folder_id.is_main());
        assert!(!folder_id.is_archive());
    }

    #[test]
    fn test_archive_static_returns_archive() {
        let folder_id = FolderId::archive();
        assert!(folder_id.is_archive());
        assert!(!folder_id.is_main());
    }

    #[test]
    fn test_only_two_valid_values() {
        let main = FolderId::new(0);
        let archive = FolderId::new(1);
        assert_eq!(main, FolderId::main());
        assert_eq!(archive, FolderId::archive());
    }

    #[test]
    fn test_normalization_behavior() {
        // All non-1 values should become main
        for i in [-10, -1, 0, 2, 5, 100] {
            let folder_id = FolderId::new(i);
            assert_eq!(folder_id.get(), 0, "Value {} should normalize to 0", i);
        }
    }

    // Serialization tests (2)
    #[test]
    fn test_serialize_deserialize_main() {
        let folder_id = FolderId::main();
        let serialized = serde_json::to_string(&folder_id).unwrap();
        let deserialized: FolderId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(folder_id, deserialized);
    }

    #[test]
    fn test_serialize_deserialize_archive() {
        let folder_id = FolderId::archive();
        let serialized = serde_json::to_string(&folder_id).unwrap();
        let deserialized: FolderId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(folder_id, deserialized);
    }

    #[test]
    fn test_deserialize_normalizes_invalid() {
        let serialized = serde_json::to_string(&5i32).unwrap();
        let deserialized: FolderId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, FolderId::main());
    }

    // Doc test example (1)
    #[test]
    fn test_doc_example() {
        let main = FolderId::main();
        assert_eq!(main.get(), 0);

        let archive = FolderId::archive();
        assert_eq!(archive.get(), 1);

        let invalid = FolderId::new(99);
        assert_eq!(invalid.get(), 0);
    }
}
