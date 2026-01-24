//! # Group Call ID
//!
//! Identifier for group voice/video calls in Telegram.
//!
//! ## Overview
//!
//! `GroupCallId` is a unique identifier for group calls in Telegram.
//! It wraps an i32 value and provides validation to ensure the ID is positive.
//!
//! ## Usage
//!
//! ```
//! use rustgram_group_call_id::GroupCallId;
//!
//! let id = GroupCallId::new(12345);
//! assert!(id.is_valid());
//!
//! let invalid = GroupCallId::new(0);
//! assert!(!invalid.is_valid());
//! ```

use core::fmt;
use core::hash::Hash;

/// Unique identifier for a group call in Telegram.
///
/// Group calls are voice or video calls that can include multiple participants.
/// The ID must be a positive integer to be valid.
///
/// # Examples
///
/// ```
/// use rustgram_group_call_id::GroupCallId;
///
/// let id = GroupCallId::new(42);
/// assert_eq!(id.get(), 42);
/// assert!(id.is_valid());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GroupCallId(i32);

impl GroupCallId {
    /// Creates a new `GroupCallId` with the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_id::GroupCallId;
    ///
    /// let id = GroupCallId::new(100);
    /// ```
    #[inline]
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    /// Returns the underlying i32 value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_id::GroupCallId;
    ///
    /// let id = GroupCallId::new(42);
    /// assert_eq!(id.get(), 42);
    /// ```
    #[inline]
    pub const fn get(&self) -> i32 {
        self.0
    }

    /// Returns `true` if this is a valid group call ID.
    ///
    /// A valid ID must be positive (greater than 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_id::GroupCallId;
    ///
    /// assert!(GroupCallId::new(1).is_valid());
    /// assert!(GroupCallId::new(100).is_valid());
    /// assert!(!GroupCallId::new(0).is_valid());
    /// assert!(!GroupCallId::new(-1).is_valid());
    /// ```
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.0 > 0
    }
}

impl Default for GroupCallId {
    fn default() -> Self {
        Self(0)
    }
}

impl fmt::Display for GroupCallId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "group call {}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_new() {
        let id = GroupCallId::new(42);
        assert_eq!(id.get(), 42);
    }

    #[test]
    fn test_get() {
        let id = GroupCallId::new(123);
        assert_eq!(id.get(), 123);
    }

    #[rstest]
    #[case(1, true)]
    #[case(100, true)]
    #[case(i32::MAX, true)]
    #[case(0, false)]
    #[case(-1, false)]
    #[case(i32::MIN, false)]
    fn test_is_valid(#[case] value: i32, #[case] expected: bool) {
        let id = GroupCallId::new(value);
        assert_eq!(id.is_valid(), expected);
    }

    #[test]
    fn test_default() {
        let id = GroupCallId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_equality() {
        let id1 = GroupCallId::new(42);
        let id2 = GroupCallId::new(42);
        let id3 = GroupCallId::new(43);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_copy() {
        let id1 = GroupCallId::new(42);
        let id2 = id1;
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_clone() {
        let id1 = GroupCallId::new(42);
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(GroupCallId::new(1));
        set.insert(GroupCallId::new(2));
        set.insert(GroupCallId::new(1)); // Duplicate
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_display() {
        let id = GroupCallId::new(42);
        assert_eq!(format!("{}", id), "group call 42");
    }

    #[test]
    fn test_debug() {
        let id = GroupCallId::new(42);
        assert_eq!(format!("{:?}", id), "GroupCallId(42)");
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    #[case(100)]
    #[case(-1)]
    fn test_const_context(#[case] value: i32) {
        const ID: GroupCallId = GroupCallId::new(42);
        assert_eq!(ID.get(), 42);
        assert!(ID.is_valid());
    }
}
