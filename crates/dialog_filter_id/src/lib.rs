//! Dialog filter identifier.
//!
//! This module provides the `DialogFilterId` type, which identifies a dialog filter
//! (also known as a chat folder) in Telegram.
//!
//! # TDLib Alignment
//!
//! - TDLib type: `DialogFilterId` (td/telegram/DialogFilterId.h)
//! - Valid range: 2-255 (min=2, max=255)
//! - Used for: Chat folder/filter identification
//!
//! # Example
//!
//! ```rust
//! use rustgram_dialog_filter_id::DialogFilterId;
//!
//! let filter_id = DialogFilterId::new(5)?;
//! assert!(filter_id.is_valid());
//! assert_eq!(filter_id.get(), 5);
//! # Ok::<(), rustgram_dialog_filter_id::Error>(())
//! ```

use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Error type for DialogFilterId operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Invalid filter ID (must be in range 2-255)
    InvalidFilterId(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFilterId(msg) => write!(f, "invalid filter ID: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

/// Result type for DialogFilterId operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Dialog filter identifier.
///
/// Valid filter IDs are in the range 2-255, following TDLib conventions.
/// Filter ID 0 is reserved for "no filter" and ID 1 is not used.
///
/// # Example
///
/// ```rust
/// use rustgram_dialog_filter_id::DialogFilterId;
///
/// // Create a valid filter ID
/// let filter_id = DialogFilterId::new(10)?;
/// assert_eq!(filter_id.get(), 10);
/// assert!(filter_id.is_valid());
///
/// // Invalid filter ID (too small)
/// assert!(DialogFilterId::new(1).is_err());
///
/// // Invalid filter ID (too large)
/// assert!(DialogFilterId::new(256).is_err());
/// # Ok::<(), rustgram_dialog_filter_id::Error>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DialogFilterId(pub i32);

impl DialogFilterId {
    /// Minimum valid filter ID (TDLib convention).
    pub const MIN: i32 = 2;

    /// Maximum valid filter ID (TDLib convention).
    pub const MAX: i32 = 255;

    /// Creates a new DialogFilterId.
    ///
    /// Returns an error if the ID is not in the valid range (2-255).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_filter_id::DialogFilterId;
    ///
    /// let filter_id = DialogFilterId::new(5)?;
    /// assert_eq!(filter_id.get(), 5);
    /// # Ok::<(), rustgram_dialog_filter_id::Error>(())
    /// ```
    #[inline]
    pub fn new(id: i32) -> Result<Self> {
        if id >= Self::MIN && id <= Self::MAX {
            Ok(Self(id))
        } else {
            Err(Error::InvalidFilterId(format!(
                "must be in range {}..={}, got {id}",
                Self::MIN,
                Self::MAX
            )))
        }
    }

    /// Returns the inner ID value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_filter_id::DialogFilterId;
    ///
    /// let filter_id = DialogFilterId::new(42).unwrap();
    /// assert_eq!(filter_id.get(), 42);
    /// # Ok::<(), rustgram_dialog_filter_id::Error>(())
    /// ```
    #[inline]
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Checks if this is a valid filter ID.
    ///
    /// Always returns true for instances created via `new()`, but useful
    /// for instances created via other means (e.g., deserialization).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_filter_id::DialogFilterId;
    ///
    /// let filter_id = DialogFilterId::new(10).unwrap();
    /// assert!(filter_id.is_valid());
    /// # Ok::<(), rustgram_dialog_filter_id::Error>(())
    /// ```
    #[inline]
    pub const fn is_valid(self) -> bool {
        self.0 >= Self::MIN && self.0 <= Self::MAX
    }

    /// Returns the minimum valid filter ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_filter_id::DialogFilterId;
    ///
    /// assert_eq!(DialogFilterId::min(), DialogFilterId(2));
    /// ```
    #[inline]
    pub const fn min() -> Self {
        Self(Self::MIN)
    }

    /// Returns the maximum valid filter ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_filter_id::DialogFilterId;
    ///
    /// assert_eq!(DialogFilterId::max(), DialogFilterId(255));
    /// ```
    #[inline]
    pub const fn max() -> Self {
        Self(Self::MAX)
    }
}

impl Default for DialogFilterId {
    fn default() -> Self {
        Self(0)
    }
}

impl Hash for DialogFilterId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for DialogFilterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "folder {}", self.0)
    }
}

impl Serialize for DialogFilterId {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DialogFilterId {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = i32::deserialize(deserializer)?;
        Ok(DialogFilterId(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (10)
    #[test]
    fn test_debug() {
        let filter_id = DialogFilterId::new(5).unwrap();
        assert_eq!(format!("{:?}", filter_id), "DialogFilterId(5)");
    }

    #[test]
    fn test_clone() {
        let filter_id = DialogFilterId::new(10).unwrap();
        let cloned = filter_id;
        assert_eq!(filter_id, cloned);
    }

    #[test]
    fn test_copy() {
        let filter_id = DialogFilterId::new(15).unwrap();
        let copied = filter_id;
        assert_eq!(filter_id, copied);
    }

    #[test]
    fn test_partial_eq() {
        let id1 = DialogFilterId::new(20).unwrap();
        let id2 = DialogFilterId::new(20).unwrap();
        let id3 = DialogFilterId::new(21).unwrap();
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_partial_ord() {
        let id1 = DialogFilterId::new(10).unwrap();
        let id2 = DialogFilterId::new(20).unwrap();
        assert!(id1 < id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        let id1 = DialogFilterId::new(30).unwrap();
        let id2 = DialogFilterId::new(30).unwrap();
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        id1.hash(&mut h1);
        id2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_display() {
        let filter_id = DialogFilterId::new(40).unwrap();
        assert_eq!(format!("{}", filter_id), "folder 40");
    }

    #[test]
    fn test_default() {
        let filter_id = DialogFilterId::default();
        assert_eq!(filter_id.get(), 0);
    }

    #[test]
    fn test_const_min() {
        assert_eq!(DialogFilterId::MIN, 2);
    }

    #[test]
    fn test_const_max() {
        assert_eq!(DialogFilterId::MAX, 255);
    }

    // Constructor tests (2 constructors * 2 tests = 4)
    #[test]
    fn test_new_valid() {
        let filter_id = DialogFilterId::new(5).unwrap();
        assert_eq!(filter_id.get(), 5);
    }

    #[test]
    fn test_new_at_min_boundary() {
        let filter_id = DialogFilterId::new(2).unwrap();
        assert_eq!(filter_id.get(), 2);
    }

    #[test]
    fn test_new_at_max_boundary() {
        let filter_id = DialogFilterId::new(255).unwrap();
        assert_eq!(filter_id.get(), 255);
    }

    #[test]
    fn test_new_below_min_fails() {
        let result = DialogFilterId::new(1);
        assert!(result.is_err());
        match result {
            Err(Error::InvalidFilterId(_)) => (),
            _ => panic!("Expected InvalidFilterId error"),
        }
    }

    #[test]
    fn test_new_above_max_fails() {
        let result = DialogFilterId::new(256);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_zero_fails() {
        let result = DialogFilterId::new(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_new_negative_fails() {
        let result = DialogFilterId::new(-1);
        assert!(result.is_err());
    }

    // Method tests (4 methods * 3 tests = 12)
    #[test]
    fn test_get() {
        let filter_id = DialogFilterId::new(100).unwrap();
        assert_eq!(filter_id.get(), 100);
    }

    #[test]
    fn test_is_valid_true() {
        let filter_id = DialogFilterId::new(50).unwrap();
        assert!(filter_id.is_valid());
    }

    #[test]
    fn test_is_valid_default() {
        let filter_id = DialogFilterId::default();
        // Default creates ID 0, which is not valid
        assert!(!filter_id.is_valid());
    }

    #[test]
    fn test_is_valid_direct_construction() {
        // Direct construction (bypassing new()) can create invalid IDs
        let filter_id = DialogFilterId(1);
        assert!(!filter_id.is_valid());
    }

    #[test]
    fn test_min_static() {
        let min = DialogFilterId::min();
        assert_eq!(min.get(), 2);
    }

    #[test]
    fn test_max_static() {
        let max = DialogFilterId::max();
        assert_eq!(max.get(), 255);
    }

    #[test]
    fn test_min_is_valid() {
        let min = DialogFilterId::min();
        assert!(min.is_valid());
    }

    #[test]
    fn test_max_is_valid() {
        let max = DialogFilterId::max();
        assert!(max.is_valid());
    }

    #[test]
    fn test_ord_sorting() {
        let mut ids = vec![
            DialogFilterId::new(10).unwrap(),
            DialogFilterId::new(5).unwrap(),
            DialogFilterId::new(20).unwrap(),
        ];
        ids.sort();
        assert_eq!(ids[0].get(), 5);
        assert_eq!(ids[1].get(), 10);
        assert_eq!(ids[2].get(), 20);
    }

    // Serialization tests (2)
    #[test]
    fn test_serialize_deserialize() {
        let filter_id = DialogFilterId::new(42).unwrap();
        let serialized = serde_json::to_string(&filter_id).unwrap();
        let deserialized: DialogFilterId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(filter_id, deserialized);
    }

    #[test]
    fn test_deserialize_invalid() {
        // Deserialization doesn't validate, it just accepts the i32 value
        let serialized = serde_json::to_string(&1i32).unwrap();
        let deserialized: DialogFilterId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.get(), 1);
        assert!(!deserialized.is_valid());
    }

    // Doc test example (1)
    #[test]
    fn test_doc_example() {
        let filter_id = DialogFilterId::new(10).unwrap();
        assert_eq!(filter_id.get(), 10);
        assert!(filter_id.is_valid());

        assert!(DialogFilterId::new(1).is_err());
        assert!(DialogFilterId::new(256).is_err());
    }
}
