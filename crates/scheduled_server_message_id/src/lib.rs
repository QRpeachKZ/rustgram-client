// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Scheduled Server Message ID
//!
//! Identifier for scheduled messages on the Telegram server.
//!
//! Based on TDLib's `ScheduledServerMessageId` from `td/telegram/ScheduledServerMessageId.h`.
//!
//! # Overview
//!
//! A `ScheduledServerMessageId` uniquely identifies a message that has been
//! scheduled for future delivery. Valid IDs are in the range `0 < id < 2^18`.
//!
//! # Example
//!
//! ```rust
//! use rustgram_scheduled_server_message_id::ScheduledServerMessageId;
//!
//! let id = ScheduledServerMessageId::new(123456);
//! assert!(id.is_valid());
//! assert_eq!(id.get(), 123456);
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Scheduled server message identifier.
///
/// Represents a unique identifier for a message scheduled for future delivery
/// on the Telegram server. Valid IDs are in the range `0 < id < 2^18`.
///
/// # Example
///
/// ```
/// use rustgram_scheduled_server_message_id::ScheduledServerMessageId;
///
/// let id = ScheduledServerMessageId::new(123456);
/// assert_eq!(id.get(), 123456);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ScheduledServerMessageId(i32);

impl ScheduledServerMessageId {
    /// Maximum valid scheduled server message ID.
    ///
    /// Valid IDs must be less than `2^18 = 262144`.
    pub const MAX_ID: i32 = 1 << 18;

    /// Creates a new `ScheduledServerMessageId` from an i32 value.
    ///
    /// # Arguments
    ///
    /// * `id` - The raw message ID value
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scheduled_server_message_id::ScheduledServerMessageId;
    ///
    /// let id = ScheduledServerMessageId::new(123456);
    /// assert_eq!(id.get(), 123456);
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
    /// use rustgram_scheduled_server_message_id::ScheduledServerMessageId;
    ///
    /// let id = ScheduledServerMessageId::new(123456);
    /// assert_eq!(id.get(), 123456);
    /// ```
    #[must_use]
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Checks if this is a valid scheduled server message ID.
    ///
    /// A valid scheduled server message ID must satisfy `0 < id < 2^18`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scheduled_server_message_id::ScheduledServerMessageId;
    ///
    /// assert!(ScheduledServerMessageId::new(1).is_valid());
    /// assert!(ScheduledServerMessageId::new(262143).is_valid());
    /// assert!(!ScheduledServerMessageId::new(0).is_valid());
    /// assert!(!ScheduledServerMessageId::new(262144).is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.0 > 0 && self.0 < Self::MAX_ID
    }
}

impl fmt::Display for ScheduledServerMessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "scheduled server message {}", self.0)
    }
}

impl Hash for ScheduledServerMessageId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<i32> for ScheduledServerMessageId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<ScheduledServerMessageId> for i32 {
    fn from(id: ScheduledServerMessageId) -> Self {
        id.0
    }
}

/// Hasher for ScheduledServerMessageId.
///
/// Provides a hashing function for scheduled server message ID values,
/// useful for hash map keys.
#[derive(Debug, Clone, Copy, Default)]
pub struct ScheduledServerMessageIdHash;

impl ScheduledServerMessageIdHash {
    /// Creates a new ScheduledServerMessageIdHash.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Hashes a ScheduledServerMessageId value.
    #[must_use]
    pub fn hash(self, message_id: ScheduledServerMessageId) -> u32 {
        message_id.0 as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = ScheduledServerMessageId::new(123456);
        assert_eq!(id.get(), 123456);
    }

    #[test]
    fn test_default() {
        let id = ScheduledServerMessageId::default();
        assert_eq!(id.get(), 0);
    }

    #[test]
    fn test_from_i32() {
        let id = ScheduledServerMessageId::from(123456);
        assert_eq!(id.get(), 123456);
    }

    #[test]
    fn test_to_i32() {
        let id = ScheduledServerMessageId::new(123456);
        let value: i32 = id.into();
        assert_eq!(value, 123456);
    }

    #[test]
    fn test_is_valid_positive() {
        assert!(ScheduledServerMessageId::new(1).is_valid());
        assert!(ScheduledServerMessageId::new(100).is_valid());
        assert!(ScheduledServerMessageId::new(1000).is_valid());
        assert!(ScheduledServerMessageId::new(10000).is_valid());
        assert!(ScheduledServerMessageId::new(100000).is_valid());
        assert!(ScheduledServerMessageId::new(262143).is_valid());
    }

    #[test]
    fn test_is_valid_zero() {
        assert!(!ScheduledServerMessageId::new(0).is_valid());
    }

    #[test]
    fn test_is_valid_negative() {
        assert!(!ScheduledServerMessageId::new(-1).is_valid());
        assert!(!ScheduledServerMessageId::new(-100).is_valid());
        assert!(!ScheduledServerMessageId::new(i32::MIN).is_valid());
    }

    #[test]
    fn test_is_valid_upper_bound() {
        assert!(!ScheduledServerMessageId::new(262144).is_valid());
        assert!(!ScheduledServerMessageId::new(300000).is_valid());
        assert!(!ScheduledServerMessageId::new(i32::MAX).is_valid());
    }

    #[test]
    fn test_equality() {
        let id1 = ScheduledServerMessageId::new(123456);
        let id2 = ScheduledServerMessageId::new(123456);
        let id3 = ScheduledServerMessageId::new(789012);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_display() {
        let id = ScheduledServerMessageId::new(123456);
        let display = format!("{id}");
        assert!(display.contains("scheduled server message"));
        assert!(display.contains("123456"));
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;

        let id = ScheduledServerMessageId::new(123456);
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        // Just ensure it doesn't panic
    }

    #[test]
    fn test_clone() {
        let id1 = ScheduledServerMessageId::new(123456);
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_serialization() {
        let id = ScheduledServerMessageId::new(123456);
        let json = serde_json::to_string(&id).expect("Failed to serialize");
        assert!(json.contains("123456"));

        let deserialized: ScheduledServerMessageId =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, id);
    }

    #[test]
    fn test_max_id_constant() {
        assert_eq!(ScheduledServerMessageId::MAX_ID, 262144);
    }

    #[test]
    fn test_scheduled_server_message_id_hash() {
        let hasher = ScheduledServerMessageIdHash::new();
        let id = ScheduledServerMessageId::new(123456);
        let hash = hasher.hash(id);
        assert_eq!(hash, 123456);
    }

    #[test]
    fn test_boundary_values() {
        // Test boundary at 0
        assert!(!ScheduledServerMessageId::new(0).is_valid());
        assert!(ScheduledServerMessageId::new(1).is_valid());

        // Test boundary at MAX_ID
        assert!(ScheduledServerMessageId::new(262143).is_valid());
        assert!(!ScheduledServerMessageId::new(262144).is_valid());
    }

    #[test]
    fn test_debug_formatting() {
        let id = ScheduledServerMessageId::new(123456);
        let debug = format!("{id:?}");
        assert!(debug.contains("123456"));
    }
}
