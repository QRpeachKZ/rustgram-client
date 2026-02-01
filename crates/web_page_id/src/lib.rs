// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram WebPageId - WebPage identifier type for Telegram MTProto client.
//!
//! This module provides the [`WebPageId`] type, which is used to uniquely identify
//! webpage previews (link previews) in the Telegram MTProto protocol.
//!
//! ## Overview
//!
//! WebPage IDs are 64-bit integers that uniquely identify webpage previews.
//! A value of 0 indicates an invalid or empty WebPage ID.
//!
//! ## TDLib Correspondence
//!
//! | Rust type | TDLib type | File |
//! |-----------|------------|------|
//! | [`WebPageId`] | `td::WebPageId` | `WebPageId.h` |
//!
//! ## Examples
//!
//! ### Creating a WebPageId
//!
//! ```
//! use rustgram_web_page_id::WebPageId;
//!
//! // Create from i64
//! let id = WebPageId::new(12345);
//! assert_eq!(id.get(), 12345);
//! assert!(id.is_valid());
//!
//! // Default is invalid (id = 0)
//! let default_id = WebPageId::default();
//! assert_eq!(default_id.get(), 0);
//! assert!(!default_id.is_valid());
//! ```
//!
//! ### Using with Collections
//!
//! ```
//! use rustgram_web_page_id::WebPageId;
//! use std::collections::HashMap;
//!
//! let id1 = WebPageId::new(100);
//! let id2 = WebPageId::new(200);
//!
//! let mut map = HashMap::new();
//! map.insert(id1, "https://example.com");
//! map.insert(id2, "https://telegram.org");
//!
//! assert_eq!(map.get(&id1), Some(&"https://example.com"));
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::fmt;
use std::hash::{Hash, Hasher};

/// WebPage identifier for link previews in Telegram MTProto.
///
/// This type wraps an `i64` value representing a unique identifier for webpage previews.
/// A value of `0` indicates an invalid or uninitialized WebPage ID.
///
/// # TDLib Correspondence
///
/// TDLib reference: `td/telegram/WebPageId.h`
///
/// # Examples
///
/// ```
/// use rustgram_web_page_id::WebPageId;
///
/// // Create a valid ID
/// let id = WebPageId::new(12345);
/// assert!(id.is_valid());
///
/// // Zero is invalid
/// let invalid = WebPageId::new(0);
/// assert!(!invalid.is_valid());
/// ```
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct WebPageId {
    /// The underlying i64 identifier value.
    id: i64,
}

impl WebPageId {
    /// Creates a new WebPageId from an i64 value.
    ///
    /// # Arguments
    ///
    /// * `id` - The i64 value to use as the identifier
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_web_page_id::WebPageId;
    ///
    /// let id = WebPageId::new(42);
    /// assert_eq!(id.get(), 42);
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(id: i64) -> Self {
        Self { id }
    }

    /// Returns the underlying i64 value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_web_page_id::WebPageId;
    ///
    /// let id = WebPageId::new(12345);
    /// assert_eq!(id.get(), 12345);
    /// ```
    #[must_use]
    #[inline]
    pub const fn get(self) -> i64 {
        self.id
    }

    /// Checks if this WebPageId is valid (non-zero).
    ///
    /// # Returns
    ///
    /// `true` if the ID is non-zero, `false` if it is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_web_page_id::WebPageId;
    ///
    /// assert!(WebPageId::new(1).is_valid());
    /// assert!(WebPageId::new(-1).is_valid());
    /// assert!(!WebPageId::new(0).is_valid());
    /// assert!(!WebPageId::default().is_valid());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_valid(self) -> bool {
        self.id != 0
    }
}

impl fmt::Display for WebPageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "link preview {}", self.id)
    }
}

impl fmt::Debug for WebPageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebPageId").field("id", &self.id).finish()
    }
}

impl Hash for WebPageId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl From<i64> for WebPageId {
    fn from(id: i64) -> Self {
        Self::new(id)
    }
}

impl From<WebPageId> for i64 {
    fn from(id: WebPageId) -> Self {
        id.get()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for WebPageId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for WebPageId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = i64::deserialize(deserializer)?;
        Ok(WebPageId::new(id))
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-web-page-id";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-web-page-id");
    }

    #[test]
    fn test_new() {
        let id = WebPageId::new(12345);
        assert_eq!(id.get(), 12345);
    }

    #[test]
    fn test_default() {
        let id = WebPageId::default();
        assert_eq!(id.get(), 0);
    }

    #[test]
    fn test_is_valid() {
        assert!(WebPageId::new(1).is_valid());
        assert!(WebPageId::new(-1).is_valid());
        assert!(WebPageId::new(i64::MAX).is_valid());
        assert!(WebPageId::new(i64::MIN).is_valid());
        assert!(!WebPageId::new(0).is_valid());
        assert!(!WebPageId::default().is_valid());
    }

    #[test]
    fn test_equality() {
        let id1 = WebPageId::new(123);
        let id2 = WebPageId::new(123);
        let id3 = WebPageId::new(456);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_clone() {
        let id1 = WebPageId::new(789);
        let id2 = id1;

        assert_eq!(id1, id2);
        assert_eq!(id1.get(), 789);
        assert_eq!(id2.get(), 789);
    }

    #[test]
    fn test_display() {
        let id = WebPageId::new(42);
        assert_eq!(format!("{}", id), "link preview 42");
    }

    #[test]
    fn test_debug() {
        let id = WebPageId::new(42);
        let debug_str = format!("{:?}", id);
        assert!(debug_str.contains("WebPageId"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_hash() {
        use std::collections::HashMap;

        let id1 = WebPageId::new(100);
        let id2 = WebPageId::new(200);
        let id3 = WebPageId::new(100); // Same as id1

        let mut map = HashMap::new();
        map.insert(id1, "first");
        map.insert(id2, "second");

        assert_eq!(map.get(&id1), Some(&"first"));
        assert_eq!(map.get(&id2), Some(&"second"));
        // id3 hashes the same as id1
        assert_eq!(map.get(&id3), Some(&"first"));
    }

    #[test]
    fn test_from_i64() {
        let id: WebPageId = 12345.into();
        assert_eq!(id.get(), 12345);
    }

    #[test]
    fn test_into_i64() {
        let id = WebPageId::new(54321);
        let value: i64 = id.into();
        assert_eq!(value, 54321);
    }

    #[test]
    fn test_copy() {
        let id1 = WebPageId::new(111);
        let id2 = id1; // Copy, not move
        assert_eq!(id1.get(), 111);
        assert_eq!(id2.get(), 111);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize() {
        let id = WebPageId::new(999);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "999");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize() {
        let json = "12345";
        let id: WebPageId = serde_json::from_str(json).unwrap();
        assert_eq!(id.get(), 12345);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let original = WebPageId::new(42);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: WebPageId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
