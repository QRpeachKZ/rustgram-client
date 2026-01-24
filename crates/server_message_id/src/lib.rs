// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Server Message ID
//!
//! Server-side message identifier for Telegram MTProto.
//!
//! Based on TDLib's `ServerMessageId` from `td/telegram/ServerMessageId.h`.
//!
//! # Overview
//!
//! A `ServerMessageId` represents a message identifier assigned by the server.
//! Valid server message IDs are positive integers (> 0).
//!
//! # Example
//!
//! ```rust
//! use rustgram_server_message_id::ServerMessageId;
//!
//! let id = ServerMessageId::new(12345);
//! assert!(id.is_valid());
//!
//! let invalid = ServerMessageId::new(0);
//! assert!(!invalid.is_valid());
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Server-side message identifier.
///
/// Represents a message ID assigned by the Telegram server.
/// Valid message IDs are positive integers (> 0).
///
/// # TDLib Mapping
///
/// - `ServerMessageId::new(id)` → TDLib: `ServerMessageId(int32)`
/// - `is_valid()` → TDLib: Checks if `id > 0`
///
/// # Example
///
/// ```rust
/// use rustgram_server_message_id::ServerMessageId;
///
/// let id = ServerMessageId::new(12345);
/// assert_eq!(id.get(), 12345);
/// assert!(id.is_valid());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ServerMessageId(i32);

impl ServerMessageId {
    /// Creates a new `ServerMessageId` from an i32 value.
    ///
    /// # Arguments
    ///
    /// * `id` - The raw message ID value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_server_message_id::ServerMessageId;
    ///
    /// let id = ServerMessageId::new(12345);
    /// assert_eq!(id.get(), 12345);
    /// ```
    #[must_use]
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    /// Returns the inner i32 value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_server_message_id::ServerMessageId;
    ///
    /// let id = ServerMessageId::new(12345);
    /// assert_eq!(id.get(), 12345);
    /// ```
    #[must_use]
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Checks if this is a valid server message ID.
    ///
    /// A valid server message ID is a positive integer (> 0).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_server_message_id::ServerMessageId;
    ///
    /// assert!(ServerMessageId::new(1).is_valid());
    /// assert!(ServerMessageId::new(12345).is_valid());
    /// assert!(!ServerMessageId::new(0).is_valid());
    /// assert!(!ServerMessageId::new(-1).is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.0 > 0
    }
}

impl fmt::Display for ServerMessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "msg {}", self.0)
    }
}

impl From<i32> for ServerMessageId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<ServerMessageId> for i32 {
    fn from(id: ServerMessageId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = ServerMessageId::new(12345);
        assert_eq!(id.get(), 12345);
    }

    #[test]
    fn test_default() {
        let id = ServerMessageId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_from_i32() {
        let id = ServerMessageId::from(12345);
        assert_eq!(id.get(), 12345);
    }

    #[test]
    fn test_to_i32() {
        let id = ServerMessageId::new(12345);
        let value: i32 = id.into();
        assert_eq!(value, 12345);
    }

    #[test]
    fn test_is_valid_positive() {
        assert!(ServerMessageId::new(1).is_valid());
        assert!(ServerMessageId::new(100).is_valid());
        assert!(ServerMessageId::new(i32::MAX).is_valid());
    }

    #[test]
    fn test_is_valid_zero() {
        assert!(!ServerMessageId::new(0).is_valid());
    }

    #[test]
    fn test_is_valid_negative() {
        assert!(!ServerMessageId::new(-1).is_valid());
        assert!(!ServerMessageId::new(-100).is_valid());
        assert!(!ServerMessageId::new(i32::MIN).is_valid());
    }

    #[test]
    fn test_equality() {
        let id1 = ServerMessageId::new(12345);
        let id2 = ServerMessageId::new(12345);
        let id3 = ServerMessageId::new(54321);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_clone() {
        let id1 = ServerMessageId::new(12345);
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_display() {
        let id = ServerMessageId::new(12345);
        let display = format!("{id}");
        assert!(display.contains("msg"));
        assert!(display.contains("12345"));
    }

    #[test]
    fn test_display_zero() {
        let id = ServerMessageId::new(0);
        let display = format!("{id}");
        assert!(display.contains("msg"));
        assert!(display.contains("0"));
    }

    #[test]
    fn test_serialization() {
        let id = ServerMessageId::new(12345);
        let json = serde_json::to_string(&id).expect("Failed to serialize");
        assert!(json.contains("12345"));

        let deserialized: ServerMessageId =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, id);
    }

    #[test]
    fn test_serialization_invalid() {
        let id = ServerMessageId::new(0);
        let json = serde_json::to_string(&id).expect("Failed to serialize");

        let deserialized: ServerMessageId =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, id);
        assert!(!deserialized.is_valid());
    }

    #[test]
    fn test_copy() {
        let id1 = ServerMessageId::new(12345);
        let id2 = id1;
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_boundary_values() {
        assert!(ServerMessageId::new(i32::MAX).is_valid());
        assert!(!ServerMessageId::new(i32::MIN).is_valid());
    }
}
