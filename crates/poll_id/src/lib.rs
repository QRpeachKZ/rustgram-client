//! # Poll Identifier
//!
//! Unique identifier for polls in Telegram.
//!
//! ## TDLib Reference
//!
//! - TDLib header: `td/telegram/PollId.h`
//! - TDLib class: `PollId`
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_poll_id::PollId;
//!
//! let poll_id = PollId::new(12345);
//! assert!(poll_id.is_valid());
//! ```

use core::fmt;

/// Unique identifier for polls.
///
/// TDLib: `class PollId`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PollId {
    id: i64,
}

impl PollId {
    /// Create a new PollId.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_poll_id::PollId;
    ///
    /// let poll_id = PollId::new(12345);
    /// ```
    #[inline]
    pub const fn new(id: i64) -> Self {
        Self { id }
    }

    /// Get the inner i64 value.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_poll_id::PollId;
    ///
    /// let poll_id = PollId::new(12345);
    /// assert_eq!(poll_id.get(), 12345);
    /// ```
    #[inline]
    pub const fn get(self) -> i64 {
        self.id
    }

    /// Check if this PollId is valid (non-zero).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_poll_id::PollId;
    ///
    /// assert!(PollId::new(12345).is_valid());
    /// assert!(!PollId::new(0).is_valid());
    /// ```
    #[inline]
    pub const fn is_valid(self) -> bool {
        self.id != 0
    }
}

impl Default for PollId {
    fn default() -> Self {
        Self { id: 0 }
    }
}

impl fmt::Display for PollId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "poll {}", self.id)
    }
}

impl From<i64> for PollId {
    fn from(id: i64) -> Self {
        Self::new(id)
    }
}

impl From<PollId> for i64 {
    fn from(id: PollId) -> Self {
        id.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (10 tests)
    #[test]
    fn test_clone() {
        let a = PollId::new(12345);
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_copy() {
        let a = PollId::new(12345);
        let b = a;
        assert_eq!(a, PollId::new(12345));
        assert_eq!(b, PollId::new(12345));
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(PollId::new(12345), PollId::new(12345));
        assert_ne!(PollId::new(12345), PollId::new(54321));
    }

    #[test]
    fn test_default() {
        assert_eq!(PollId::default(), PollId::new(0));
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        PollId::new(12345).hash(&mut hasher);
        let h1 = hasher.finish();

        hasher = DefaultHasher::new();
        PollId::new(12345).hash(&mut hasher);
        let h2 = hasher.finish();

        assert_eq!(h1, h2);
    }

    // Constructor tests (4 tests)
    #[test]
    fn test_new() {
        let poll_id = PollId::new(12345);
        assert_eq!(poll_id.get(), 12345);
    }

    #[test]
    fn test_new_zero() {
        let poll_id = PollId::new(0);
        assert_eq!(poll_id.get(), 0);
        assert!(!poll_id.is_valid());
    }

    #[test]
    fn test_new_negative() {
        let poll_id = PollId::new(-1);
        assert_eq!(poll_id.get(), -1);
        // Negative IDs are valid in TDLib
        assert!(poll_id.is_valid());
    }

    #[test]
    fn test_from_i64() {
        let poll_id = PollId::from(12345);
        assert_eq!(poll_id.get(), 12345);
    }

    // Method tests (4 tests)
    #[test]
    fn test_get() {
        assert_eq!(PollId::new(12345).get(), 12345);
        assert_eq!(PollId::new(0).get(), 0);
        assert_eq!(PollId::new(-1).get(), -1);
    }

    #[test]
    fn test_is_valid_true() {
        assert!(PollId::new(1).is_valid());
        assert!(PollId::new(-1).is_valid());
        assert!(PollId::new(i64::MAX).is_valid());
        assert!(PollId::new(i64::MIN).is_valid());
    }

    #[test]
    fn test_is_valid_false() {
        assert!(!PollId::new(0).is_valid());
    }

    #[test]
    fn test_into_i64() {
        let poll_id = PollId::new(12345);
        let value: i64 = poll_id.into();
        assert_eq!(value, 12345);
    }

    // Display tests (3 tests)
    #[test]
    fn test_display() {
        assert_eq!(format!("{}", PollId::new(12345)), "poll 12345");
        assert_eq!(format!("{}", PollId::new(0)), "poll 0");
        assert_eq!(format!("{}", PollId::new(-1)), "poll -1");
    }

    // Debug tests (2 tests)
    #[test]
    fn test_debug() {
        let poll_id = PollId::new(12345);
        assert_eq!(format!("{:?}", poll_id), "PollId { id: 12345 }");
    }

    // Round-trip tests (2 tests)
    #[test]
    fn test_round_trip_i64() {
        for value in [0, 1, -1, i64::MAX, i64::MIN] {
            let poll_id = PollId::new(value);
            assert_eq!(poll_id.get(), value);
        }
    }

    #[test]
    fn test_round_trip_conversion() {
        let original = 12345;
        let poll_id: PollId = original.into();
        let result: i64 = poll_id.into();
        assert_eq!(result, original);
    }
}
