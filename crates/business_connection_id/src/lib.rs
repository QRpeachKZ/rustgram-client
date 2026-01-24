// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram Business Connection ID - Business connection identifier for Telegram MTProto client.
//!
//! This module provides the [`BusinessConnectionId`] type which uniquely identifies
//! a business connection in the Telegram MTProto protocol.
//!
//! ## Overview
//!
//! Business connection IDs are string-based identifiers used for business bot connections.
//! A valid connection ID is a non-empty string.
//!
//! ## Examples
//!
//! ### Creating a Business Connection ID
//!
//! ```
//! use rustgram_business_connection_id::BusinessConnectionId;
//!
//! // Create from string
//! let id = BusinessConnectionId::new("conn_abc123".to_string());
//! assert!(id.is_valid());
//! assert_eq!(id.get(), "conn_abc123");
//!
//! // Default is empty/invalid
//! let default = BusinessConnectionId::default();
//! assert!(!default.is_valid());
//! assert!(default.is_empty());
//! ```
//!
//! ### Checking Validity
//!
//! ```
//! use rustgram_business_connection_id::BusinessConnectionId;
//!
//! let id = BusinessConnectionId::new("valid_id".to_string());
//! assert!(id.is_valid());
//! assert!(!id.is_empty());
//!
//! let empty = BusinessConnectionId::new("".to_string());
//! assert!(!empty.is_valid());
//! assert!(empty.is_empty());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::hash::{Hash, Hasher};

/// Unique identifier for a business connection.
///
/// Represents a business connection ID in the Telegram MTProto protocol.
/// Connection IDs are non-empty strings.
///
/// # Examples
///
/// ```
/// use rustgram_business_connection_id::BusinessConnectionId;
///
/// let id = BusinessConnectionId::new("conn_abc123".to_string());
/// assert!(id.is_valid());
/// assert_eq!(id.get(), "conn_abc123");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BusinessConnectionId(String);

impl Default for BusinessConnectionId {
    fn default() -> Self {
        Self(String::new())
    }
}

impl BusinessConnectionId {
    /// Creates a new [`BusinessConnectionId`] from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connection_id::BusinessConnectionId;
    ///
    /// let id = BusinessConnectionId::new("conn_abc123".to_string());
    /// assert_eq!(id.get(), "conn_abc123");
    /// ```
    pub fn new(id: String) -> Self {
        Self(id)
    }

    /// Returns the underlying string value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connection_id::BusinessConnectionId;
    ///
    /// let id = BusinessConnectionId::new("conn_abc123".to_string());
    /// assert_eq!(id.get(), "conn_abc123");
    /// ```
    pub fn get(&self) -> &str {
        &self.0
    }

    /// Returns `true` if the connection ID is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connection_id::BusinessConnectionId;
    ///
    /// assert!(BusinessConnectionId::default().is_empty());
    /// assert!(!BusinessConnectionId::new("test".to_string()).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if this is a valid connection ID.
    ///
    /// A connection ID is valid if it is non-empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connection_id::BusinessConnectionId;
    ///
    /// assert!(!BusinessConnectionId::default().is_valid());
    /// assert!(BusinessConnectionId::new("test".to_string()).is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.0.is_empty()
    }

    /// Consumes `self` and returns the inner string.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connection_id::BusinessConnectionId;
    ///
    /// let id = BusinessConnectionId::new("conn_abc123".to_string());
    /// let s = id.into_inner();
    /// assert_eq!(s, "conn_abc123");
    /// ```
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Hash for BusinessConnectionId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl std::fmt::Display for BusinessConnectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "business connection {}", self.0)
    }
}

impl From<String> for BusinessConnectionId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for BusinessConnectionId {
    fn from(s: &str) -> Self {
        Self::new(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let id = BusinessConnectionId::default();
        assert!(id.is_empty());
        assert!(!id.is_valid());
        assert_eq!(id.get(), "");
    }

    #[test]
    fn test_new_valid() {
        let id = BusinessConnectionId::new("conn_abc123".to_string());
        assert!(!id.is_empty());
        assert!(id.is_valid());
        assert_eq!(id.get(), "conn_abc123");
    }

    #[test]
    fn test_new_empty() {
        let id = BusinessConnectionId::new("".to_string());
        assert!(id.is_empty());
        assert!(!id.is_valid());
        assert_eq!(id.get(), "");
    }

    #[test]
    fn test_from_string() {
        let s = "conn_test".to_string();
        let id = BusinessConnectionId::from(s.clone());
        assert_eq!(id.get(), "conn_test");
    }

    #[test]
    fn test_from_str() {
        let id = BusinessConnectionId::from("conn_test");
        assert_eq!(id.get(), "conn_test");
    }

    #[test]
    fn test_into_inner() {
        let id = BusinessConnectionId::new("conn_abc123".to_string());
        let s = id.into_inner();
        assert_eq!(s, "conn_abc123");
    }

    #[test]
    fn test_equality() {
        let id1 = BusinessConnectionId::new("conn_abc".to_string());
        let id2 = BusinessConnectionId::new("conn_abc".to_string());
        assert_eq!(id1, id2);

        let id3 = BusinessConnectionId::new("conn_xyz".to_string());
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_clone() {
        let id = BusinessConnectionId::new("conn_abc123".to_string());
        let cloned = id.clone();
        assert_eq!(id, cloned);
        assert_eq!(id.get(), cloned.get());
    }

    #[test]
    fn test_display() {
        let id = BusinessConnectionId::new("conn_abc123".to_string());
        assert_eq!(format!("{}", id), "business connection conn_abc123");
    }

    #[test]
    fn test_debug() {
        let id = BusinessConnectionId::new("conn_abc123".to_string());
        let debug_str = format!("{:?}", id);
        assert!(debug_str.contains("conn_abc123"));
    }

    #[test]
    fn test_hash() {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        let id1 = BusinessConnectionId::new("conn_abc".to_string());
        let id2 = BusinessConnectionId::new("conn_xyz".to_string());

        map.insert(id1, "first");
        map.insert(id2, "second");

        assert_eq!(
            map.get(&BusinessConnectionId::new("conn_abc".to_string())),
            Some(&"first")
        );
        assert_eq!(
            map.get(&BusinessConnectionId::new("conn_xyz".to_string())),
            Some(&"second")
        );
    }

    #[test]
    fn test_is_empty() {
        assert!(BusinessConnectionId::default().is_empty());
        assert!(BusinessConnectionId::new("".to_string()).is_empty());
        assert!(!BusinessConnectionId::new("test".to_string()).is_empty());
    }

    #[test]
    fn test_is_valid() {
        assert!(!BusinessConnectionId::default().is_valid());
        assert!(!BusinessConnectionId::new("".to_string()).is_valid());
        assert!(BusinessConnectionId::new("test".to_string()).is_valid());
    }
}
