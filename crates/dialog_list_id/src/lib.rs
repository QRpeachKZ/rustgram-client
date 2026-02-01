//! Dialog list identifier.
//!
//! This module provides the `DialogListId` type, which identifies a dialog list
//! (chat list) in Telegram. A dialog list can be either a folder or a filter.
//!
//! # TDLib Alignment
//!
//! - TDLib type: `DialogListId` (td/telegram/DialogListId.h)
//! - Encodes both folders and filters in a single i64 value
//! - Filters use bit shift: filter_id + FILTER_ID_SHIFT
//! - Folders use direct i32 values
//!
//! # Encoding
//!
//! - Folders: direct i32 values (0 = main, 1 = archive)
//! - Filters: filter_id + FILTER_ID_SHIFT where FILTER_ID_SHIFT = 1 << 32
//!
//! # Example
//!
//! ```rust
//! use rustgram_dialog_list_id::DialogListId;
//!
//! // Create from folder ID (main folder)
//! let list_id = DialogListId::main();
//! assert!(list_id.is_folder());
//!
//! // Create from filter ID
//! let filter_id = rustgram_dialog_filter_id::DialogFilterId::new(5).unwrap();
//! let list_id = DialogListId::from_filter(filter_id);
//! assert!(list_id.is_filter());
//! # Ok::<(), rustgram_dialog_filter_id::Error>(())
//! ```

use rustgram_dialog_filter_id::DialogFilterId;
use rustgram_folder_id::FolderId;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Bit shift for encoding filter IDs in the dialog list ID.
pub const FILTER_ID_SHIFT: i64 = 1 << 32;

/// Dialog list identifier.
///
/// Represents either a folder or a filter, encoded in a single i64 value.
/// This follows the TDLib convention for efficient storage and comparison.
///
/// # Example
///
/// ```rust
/// use rustgram_dialog_list_id::DialogListId;
///
/// // Main folder
/// let main = DialogListId::main();
/// assert!(main.is_folder());
/// assert_eq!(main.get(), 0);
///
/// // Archive folder
/// let archive = DialogListId::archive();
/// assert!(archive.is_folder());
///
/// // From filter ID
/// let filter_id = rustgram_dialog_filter_id::DialogFilterId::new(10).unwrap();
/// let list_id = DialogListId::from_filter(filter_id);
/// assert!(list_id.is_filter());
/// # Ok::<(), rustgram_dialog_filter_id::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DialogListId(pub i64);

impl DialogListId {
    /// Creates a DialogListId from an i64 value.
    ///
    /// Validates folder IDs but allows filter IDs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_list_id::DialogListId;
    ///
    /// // Valid folder ID
    /// let list_id = DialogListId::new(0);
    /// assert!(list_id.is_folder());
    ///
    /// // Invalid folder ID becomes main
    /// let list_id = DialogListId::new(5);
    /// assert!(list_id.is_folder());
    /// assert_eq!(list_id.get(), 0);
    /// ```
    pub fn new(id: i64) -> Self {
        let result = Self(id);
        if result.is_folder() {
            let folder_id = FolderId::new(id as i32);
            Self(folder_id.get() as i64)
        } else {
            result
        }
    }

    /// Creates a DialogListId from a filter ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_list_id::DialogListId;
    /// use rustgram_dialog_filter_id::DialogFilterId;
    ///
    /// let filter_id = DialogFilterId::new(15).unwrap();
    /// let list_id = DialogListId::from_filter(filter_id);
    /// assert!(list_id.is_filter());
    /// assert_eq!(list_id.get_filter_id().get(), 15);
    /// # Ok::<(), rustgram_dialog_filter_id::Error>(())
    /// ```
    pub fn from_filter(filter_id: DialogFilterId) -> Self {
        Self(filter_id.get() as i64 + FILTER_ID_SHIFT)
    }

    /// Creates a DialogListId from a folder ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_list_id::DialogListId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let folder_id = FolderId::archive();
    /// let list_id = DialogListId::from_folder(folder_id);
    /// assert!(list_id.is_folder());
    /// assert_eq!(list_id.get_folder_id().get(), 1);
    /// ```
    pub fn from_folder(folder_id: FolderId) -> Self {
        Self(folder_id.get() as i64)
    }

    /// Returns the main dialog list (folder 0).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_list_id::DialogListId;
    ///
    /// let main = DialogListId::main();
    /// assert!(main.is_folder());
    /// assert_eq!(main.get(), 0);
    /// ```
    pub fn main() -> Self {
        Self::from_folder(FolderId::main())
    }

    /// Returns the archive dialog list (folder 1).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_list_id::DialogListId;
    ///
    /// let archive = DialogListId::archive();
    /// assert!(archive.is_folder());
    /// assert_eq!(archive.get_folder_id().get(), 1);
    /// ```
    pub fn archive() -> Self {
        Self::from_folder(FolderId::archive())
    }

    /// Returns the inner i64 value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_list_id::DialogListId;
    /// use rustgram_dialog_filter_id::DialogFilterId;
    ///
    /// let filter_id = DialogFilterId::new(20).unwrap();
    /// let list_id = DialogListId::from_filter(filter_id);
    /// assert!(list_id.get() > FILTER_ID_SHIFT);
    /// # Ok::<(), rustgram_dialog_filter_id::Error>(())
    /// ```
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Checks if this is a folder-based dialog list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_list_id::DialogListId;
    ///
    /// let main = DialogListId::main();
    /// assert!(main.is_folder());
    /// ```
    pub fn is_folder(&self) -> bool {
        (i32::MIN as i64) <= self.0 && self.0 <= (i32::MAX as i64)
    }

    /// Checks if this is a filter-based dialog list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_list_id::DialogListId;
    /// use rustgram_dialog_filter_id::DialogFilterId;
    ///
    /// let filter_id = DialogFilterId::new(5).unwrap();
    /// let list_id = DialogListId::from_filter(filter_id);
    /// assert!(list_id.is_filter());
    /// # Ok::<(), rustgram_dialog_filter_id::Error>(())
    /// ```
    pub fn is_filter(&self) -> bool {
        (i32::MIN as i64) + FILTER_ID_SHIFT <= self.0
            && self.0 <= (i32::MAX as i64) + FILTER_ID_SHIFT
    }

    /// Gets the folder ID if this is a folder-based list.
    ///
    /// Returns None if this is a filter-based list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_list_id::DialogListId;
    ///
    /// let archive = DialogListId::archive();
    /// let folder_id = archive.get_folder_id();
    /// assert_eq!(folder_id.get(), 1);
    /// ```
    pub fn get_folder_id(self) -> FolderId {
        debug_assert!(self.is_folder(), "Not a folder ID");
        FolderId::new(self.0 as i32)
    }

    /// Gets the filter ID if this is a filter-based list.
    ///
    /// Returns None if this is a folder-based list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_list_id::DialogListId;
    /// use rustgram_dialog_filter_id::DialogFilterId;
    ///
    /// let filter_id = DialogFilterId::new(10).unwrap();
    /// let list_id = DialogListId::from_filter(filter_id);
    /// let retrieved = list_id.get_filter_id();
    /// assert_eq!(retrieved.get(), 10);
    /// # Ok::<(), rustgram_dialog_filter_id::Error>(())
    /// ```
    pub fn get_filter_id(self) -> DialogFilterId {
        debug_assert!(self.is_filter(), "Not a filter ID");
        DialogFilterId::new((self.0 - FILTER_ID_SHIFT) as i32)
            .unwrap_or_else(|_| DialogFilterId::new(DialogFilterId::MIN).unwrap())
    }
}

impl Default for DialogListId {
    fn default() -> Self {
        Self::main()
    }
}

impl fmt::Display for DialogListId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_folder() {
            let folder_id = self.get_folder_id();
            if folder_id == FolderId::archive() {
                return write!(f, "Archive chat list");
            }
            if folder_id == FolderId::main() {
                return write!(f, "Main chat list");
            }
            write!(f, "chat list folder {}", folder_id.get())
        } else if self.is_filter() {
            write!(f, "chat list filter {}", self.get_filter_id().get())
        } else {
            write!(f, "unknown chat list {}", self.0)
        }
    }
}

impl Serialize for DialogListId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DialogListId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = i64::deserialize(deserializer)?;
        Ok(DialogListId::new(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (10)
    #[test]
    fn test_debug() {
        let list_id = DialogListId::main();
        assert_eq!(format!("{:?}", list_id), "DialogListId(0)");
    }

    #[test]
    fn test_clone() {
        let list_id = DialogListId::archive();
        let cloned = list_id;
        assert_eq!(list_id, cloned);
    }

    #[test]
    fn test_copy() {
        let list_id = DialogListId::main();
        let copied = list_id;
        assert_eq!(list_id, copied);
    }

    #[test]
    fn test_partial_eq() {
        let id1 = DialogListId::main();
        let id2 = DialogListId::main();
        let id3 = DialogListId::archive();
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        let id1 = DialogListId::main();
        let id2 = DialogListId::main();
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        id1.hash(&mut h1);
        id2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_display_main() {
        let list_id = DialogListId::main();
        assert_eq!(format!("{}", list_id), "Main chat list");
    }

    #[test]
    fn test_display_archive() {
        let list_id = DialogListId::archive();
        assert_eq!(format!("{}", list_id), "Archive chat list");
    }

    #[test]
    fn test_display_filter() {
        let filter_id = DialogFilterId::new(5).unwrap();
        let list_id = DialogListId::from_filter(filter_id);
        assert!(format!("{}", list_id).contains("filter 5"));
    }

    #[test]
    fn test_default() {
        let list_id = DialogListId::default();
        assert_eq!(list_id, DialogListId::main());
    }

    #[test]
    fn test_const_filter_shift() {
        assert_eq!(FILTER_ID_SHIFT, 1i64 << 32);
    }

    // Constructor tests (5 constructors * 2 tests = 10)
    #[test]
    fn test_new_folder_main() {
        let list_id = DialogListId::new(0);
        assert!(list_id.is_folder());
        assert_eq!(list_id.get(), 0);
    }

    #[test]
    fn test_new_folder_archive() {
        let list_id = DialogListId::new(1);
        assert!(list_id.is_folder());
        assert_eq!(list_id.get_folder_id().get(), 1);
    }

    #[test]
    fn test_new_invalid_folder() {
        let list_id = DialogListId::new(5);
        // Invalid folder ID becomes main
        assert!(list_id.is_folder());
        assert_eq!(list_id.get(), 0);
    }

    #[test]
    fn test_from_filter() {
        let filter_id = DialogFilterId::new(10).unwrap();
        let list_id = DialogListId::from_filter(filter_id);
        assert!(list_id.is_filter());
        assert_eq!(list_id.get_filter_id().get(), 10);
    }

    #[test]
    fn test_from_folder_main() {
        let list_id = DialogListId::from_folder(FolderId::main());
        assert!(list_id.is_folder());
        assert_eq!(list_id.get(), 0);
    }

    #[test]
    fn test_from_folder_archive() {
        let list_id = DialogListId::from_folder(FolderId::archive());
        assert!(list_id.is_folder());
        assert_eq!(list_id.get_folder_id().get(), 1);
    }

    #[test]
    fn test_main_static() {
        let list_id = DialogListId::main();
        assert!(list_id.is_folder());
        assert_eq!(list_id.get(), 0);
    }

    #[test]
    fn test_archive_static() {
        let list_id = DialogListId::archive();
        assert!(list_id.is_folder());
        assert_eq!(list_id.get_folder_id().get(), 1);
    }

    #[test]
    fn test_filter_encoding() {
        let filter_id = DialogFilterId::new(5).unwrap();
        let list_id = DialogListId::from_filter(filter_id);
        assert_eq!(list_id.get(), 5i64 + FILTER_ID_SHIFT);
    }

    #[test]
    fn test_filter_roundtrip() {
        let filter_id = DialogFilterId::new(20).unwrap();
        let list_id = DialogListId::from_filter(filter_id);
        let retrieved = list_id.get_filter_id();
        assert_eq!(filter_id, retrieved);
    }

    // Method tests (7 methods * 3 tests = 21)
    #[test]
    fn test_get_main() {
        let list_id = DialogListId::main();
        assert_eq!(list_id.get(), 0);
    }

    #[test]
    fn test_get_archive() {
        let list_id = DialogListId::archive();
        assert_eq!(list_id.get(), 1);
    }

    #[test]
    fn test_get_filter() {
        let filter_id = DialogFilterId::new(15).unwrap();
        let list_id = DialogListId::from_filter(filter_id);
        assert_eq!(list_id.get(), 15i64 + FILTER_ID_SHIFT);
    }

    #[test]
    fn test_is_folder_true() {
        let list_id = DialogListId::main();
        assert!(list_id.is_folder());
    }

    #[test]
    fn test_is_folder_false() {
        let filter_id = DialogFilterId::new(5).unwrap();
        let list_id = DialogListId::from_filter(filter_id);
        assert!(!list_id.is_folder());
    }

    #[test]
    fn test_is_filter_true() {
        let filter_id = DialogFilterId::new(10).unwrap();
        let list_id = DialogListId::from_filter(filter_id);
        assert!(list_id.is_filter());
    }

    #[test]
    fn test_is_filter_false() {
        let list_id = DialogListId::archive();
        assert!(!list_id.is_filter());
    }

    #[test]
    fn test_get_folder_id_main() {
        let list_id = DialogListId::main();
        let folder_id = list_id.get_folder_id();
        assert_eq!(folder_id.get(), 0);
    }

    #[test]
    fn test_get_folder_id_archive() {
        let list_id = DialogListId::archive();
        let folder_id = list_id.get_folder_id();
        assert_eq!(folder_id.get(), 1);
    }

    #[test]
    fn test_get_filter_id_retrieved() {
        let filter_id = DialogFilterId::new(25).unwrap();
        let list_id = DialogListId::from_filter(filter_id);
        let retrieved = list_id.get_filter_id();
        assert_eq!(retrieved.get(), 25);
    }

    #[test]
    fn test_folder_vs_filter_encoding() {
        let folder = DialogListId::archive();
        let filter_id = DialogFilterId::new(5).unwrap();
        let filter = DialogListId::from_filter(filter_id);

        assert!(folder.is_folder() && !folder.is_filter());
        assert!(filter.is_filter() && !filter.is_folder());
        assert_ne!(folder.get(), filter.get());
    }

    #[test]
    fn test_multiple_filters() {
        for i in [2, 10, 50, 100, 255] {
            let filter_id = DialogFilterId::new(i).unwrap();
            let list_id = DialogListId::from_filter(filter_id);
            assert!(list_id.is_filter());
            assert_eq!(list_id.get_filter_id().get(), i);
        }
    }

    // Serialization tests (2)
    #[test]
    fn test_serialize_deserialize_folder() {
        let list_id = DialogListId::archive();
        let serialized = serde_json::to_string(&list_id).unwrap();
        let deserialized: DialogListId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(list_id, deserialized);
    }

    #[test]
    fn test_serialize_deserialize_filter() {
        let filter_id = DialogFilterId::new(30).unwrap();
        let list_id = DialogListId::from_filter(filter_id);
        let serialized = serde_json::to_string(&list_id).unwrap();
        let deserialized: DialogListId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(list_id, deserialized);
    }
}
