// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram Call ID - Call identifier for Telegram MTProto client.
//!
//! This module provides the [`CallId`] type which uniquely identifies a call
//! in the Telegram MTProto protocol.
//!
//! ## Overview
//!
//! Call IDs are used to uniquely identify voice or video calls. A valid call ID
//! is any non-zero i32 value. The value 0 is reserved and represents an invalid
//! or empty call ID.
//!
//! ## Examples
//!
//! ### Creating a Call ID
//!
//! ```
//! use rustgram_call_id::CallId;
//!
//! // Create from i32
//! let id = CallId::new(12345);
//! assert!(id.is_valid());
//! assert_eq!(id.get(), 12345);
//!
//! // Default is invalid
//! let default = CallId::default();
//! assert!(!default.is_valid());
//! assert_eq!(default.get(), 0);
//! ```
//!
//! ### Using with HashMap
//!
//! ```
//! use rustgram_call_id::CallId;
//! use std::collections::HashMap;
//!
//! let mut map = HashMap::new();
//! let id = CallId::new(123);
//! map.insert(id, "Call data");
//! assert_eq!(map.get(&CallId::new(123)), Some(&"Call data"));
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::hash::{Hash, Hasher};

/// Unique identifier for a call.
///
/// Represents a call ID in the Telegram MTProto protocol. Valid call IDs are
/// non-zero integers, while 0 represents an invalid or empty call ID.
///
/// # Examples
///
/// ```
/// use rustgram_call_id::CallId;
///
/// let id = CallId::new(12345);
/// assert!(id.is_valid());
/// assert_eq!(id.get(), 12345);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CallId(i32);

impl Default for CallId {
    fn default() -> Self {
        Self(0)
    }
}

impl CallId {
    /// Creates a new [`CallId`] from an i32 value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_id::CallId;
    ///
    /// let id = CallId::new(12345);
    /// assert_eq!(id.get(), 12345);
    /// ```
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    /// Returns the underlying i32 value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_id::CallId;
    ///
    /// let id = CallId::new(12345);
    /// assert_eq!(id.get(), 12345);
    /// ```
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Returns `true` if this is a valid call ID.
    ///
    /// A call ID is considered valid if it is non-zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_call_id::CallId;
    ///
    /// assert!(!CallId::default().is_valid());
    /// assert!(CallId::new(123).is_valid());
    /// assert!(!CallId::new(0).is_valid());
    /// ```
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

impl Hash for CallId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl std::fmt::Display for CallId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "call {}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let id = CallId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_new_valid() {
        let id = CallId::new(12345);
        assert_eq!(id.get(), 12345);
        assert!(id.is_valid());
    }

    #[test]
    fn test_new_zero() {
        let id = CallId::new(0);
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_new_negative() {
        let id = CallId::new(-1);
        assert_eq!(id.get(), -1);
        assert!(id.is_valid());
    }

    #[test]
    fn test_equality() {
        let id1 = CallId::new(123);
        let id2 = CallId::new(123);
        assert_eq!(id1, id2);

        let id3 = CallId::new(456);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_ordering() {
        let id1 = CallId::new(100);
        let id2 = CallId::new(200);
        assert!(id1 < id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_copy() {
        let id1 = CallId::new(123);
        let id2 = id1;
        assert_eq!(id1, id2);
        assert_eq!(id1.get(), 123);
        assert_eq!(id2.get(), 123);
    }

    #[test]
    fn test_clone() {
        let id = CallId::new(123);
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_display() {
        let id = CallId::new(12345);
        assert_eq!(format!("{}", id), "call 12345");
    }

    #[test]
    fn test_debug() {
        let id = CallId::new(12345);
        let debug_str = format!("{:?}", id);
        assert!(debug_str.contains("12345"));
    }

    #[test]
    fn test_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        let id1 = CallId::new(123);
        let id2 = CallId::new(456);

        map.insert(id1, "first");
        map.insert(id2, "second");

        assert_eq!(map.get(&CallId::new(123)), Some(&"first"));
        assert_eq!(map.get(&CallId::new(456)), Some(&"second"));
    }
}
