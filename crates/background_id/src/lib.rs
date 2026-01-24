// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram Background ID - Background identifier for Telegram MTProto client.
//!
//! This module provides the [`BackgroundId`] type which uniquely identifies a
//! chat background in the Telegram MTProto protocol.
//!
//! ## Overview
//!
//! Background IDs are used to uniquely identify chat backgrounds. Valid background
//! IDs are non-zero i64 values. Local backgrounds have IDs in the range (0, 0x7FFFFFFF].
//!
//! ## Examples
//!
//! ### Creating a Background ID
//!
//! ```
//! use rustgram_background_id::BackgroundId;
//!
//! // Create from i64
//! let id = BackgroundId::new(123456789);
//! assert!(id.is_valid());
//! assert_eq!(id.get(), 123456789);
//!
//! // Default is invalid
//! let default = BackgroundId::default();
//! assert!(!default.is_valid());
//! assert_eq!(default.get(), 0);
//! ```
//!
//! ### Checking if Local
//!
//! ```
//! use rustgram_background_id::BackgroundId;
//!
//! // Local backgrounds have IDs in range (0, 0x7FFFFFFF]
//! let local = BackgroundId::new(1000);
//! assert!(local.is_local());
//!
//! // Remote backgrounds have larger IDs
//! let remote = BackgroundId::new(0x80000000);
//! assert!(!remote.is_local());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::hash::{Hash, Hasher};

/// Unique identifier for a chat background.
///
/// Represents a background ID in the Telegram MTProto protocol. Valid background
/// IDs are non-zero integers. Local backgrounds have IDs in the range (0, 0x7FFFFFFF].
///
/// # Examples
///
/// ```
/// use rustgram_background_id::BackgroundId;
///
/// let id = BackgroundId::new(123456789);
/// assert!(id.is_valid());
/// assert_eq!(id.get(), 123456789);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BackgroundId(i64);

impl BackgroundId {
    /// Creates a new [`BackgroundId`] from an i64 value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_id::BackgroundId;
    ///
    /// let id = BackgroundId::new(123456789);
    /// assert_eq!(id.get(), 123456789);
    /// ```
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the underlying i64 value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_id::BackgroundId;
    ///
    /// let id = BackgroundId::new(123456789);
    /// assert_eq!(id.get(), 123456789);
    /// ```
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Returns `true` if this is a valid background ID.
    ///
    /// A background ID is considered valid if it is non-zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_id::BackgroundId;
    ///
    /// assert!(!BackgroundId::default().is_valid());
    /// assert!(BackgroundId::new(123).is_valid());
    /// assert!(!BackgroundId::new(0).is_valid());
    /// ```
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }

    /// Returns `true` if this is a local background ID.
    ///
    /// Local backgrounds have IDs in the range (0, 0x7FFFFFFF] (positive values
    /// that fit in an i32).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_id::BackgroundId;
    ///
    /// assert!(BackgroundId::new(1000).is_local());
    /// assert!(BackgroundId::new(0x7FFFFFFF).is_local());
    /// assert!(!BackgroundId::new(0).is_local());
    /// assert!(!BackgroundId::new(0x80000000).is_local());
    /// ```
    pub const fn is_local(self) -> bool {
        self.0 > 0 && self.0 <= 0x7FFFFFFF
    }
}

impl Hash for BackgroundId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl std::fmt::Display for BackgroundId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "background {}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let id = BackgroundId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
        assert!(!id.is_local());
    }

    #[test]
    fn test_new_valid() {
        let id = BackgroundId::new(123456789);
        assert_eq!(id.get(), 123456789);
        assert!(id.is_valid());
    }

    #[test]
    fn test_new_zero() {
        let id = BackgroundId::new(0);
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
        assert!(!id.is_local());
    }

    #[test]
    fn test_new_negative() {
        let id = BackgroundId::new(-1);
        assert_eq!(id.get(), -1);
        assert!(id.is_valid());
        assert!(!id.is_local());
    }

    #[test]
    fn test_is_local() {
        // Positive small values are local
        assert!(BackgroundId::new(1).is_local());
        assert!(BackgroundId::new(1000).is_local());
        assert!(BackgroundId::new(0x7FFFFFFF).is_local());

        // Zero is not local
        assert!(!BackgroundId::new(0).is_local());

        // Large positive values are not local
        assert!(!BackgroundId::new(0x80000000).is_local());
        assert!(!BackgroundId::new(0xFFFFFFFF).is_local());

        // Negative values are not local
        assert!(!BackgroundId::new(-1).is_local());
    }

    #[test]
    fn test_equality() {
        let id1 = BackgroundId::new(123);
        let id2 = BackgroundId::new(123);
        assert_eq!(id1, id2);

        let id3 = BackgroundId::new(456);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_ordering() {
        let id1 = BackgroundId::new(100);
        let id2 = BackgroundId::new(200);
        assert!(id1 < id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_copy() {
        let id1 = BackgroundId::new(123);
        let id2 = id1;
        assert_eq!(id1, id2);
        assert_eq!(id1.get(), 123);
        assert_eq!(id2.get(), 123);
    }

    #[test]
    fn test_clone() {
        let id = BackgroundId::new(123);
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_display() {
        let id = BackgroundId::new(123456789);
        assert_eq!(format!("{}", id), "background 123456789");
    }

    #[test]
    fn test_debug() {
        let id = BackgroundId::new(123456789);
        let debug_str = format!("{:?}", id);
        assert!(debug_str.contains("123456789"));
    }

    #[test]
    fn test_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        let id1 = BackgroundId::new(123);
        let id2 = BackgroundId::new(456);

        map.insert(id1, "first");
        map.insert(id2, "second");

        assert_eq!(map.get(&BackgroundId::new(123)), Some(&"first"));
        assert_eq!(map.get(&BackgroundId::new(456)), Some(&"second"));
    }

    #[test]
    fn test_boundary_local_max() {
        // Max local value
        let id = BackgroundId::new(0x7FFFFFFF);
        assert!(id.is_local());
        assert!(id.is_valid());
    }

    #[test]
    fn test_boundary_above_local_max() {
        // One above max local value
        let id = BackgroundId::new(0x80000000);
        assert!(!id.is_local());
        assert!(id.is_valid());
    }
}
