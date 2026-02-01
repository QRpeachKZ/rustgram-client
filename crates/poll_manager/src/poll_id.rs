// Copyright (c) 2025 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Poll ID type.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Poll identifier.
///
/// Poll IDs can be either local (negative) or server (positive).
/// Local poll IDs are generated client-side for polls that haven't
/// been sent to the server yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PollId(i64);

impl PollId {
    /// Creates a new PollId.
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the inner i64 value.
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Checks if this is a valid poll ID.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

impl Default for PollId {
    fn default() -> Self {
        Self(0)
    }
}

impl fmt::Display for PollId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "poll {}", self.0)
    }
}

impl From<i64> for PollId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<PollId> for i64 {
    fn from(id: PollId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poll_id_new() {
        let id = PollId::new(123);
        assert_eq!(id.get(), 123);
    }

    #[test]
    fn test_poll_id_default() {
        let id = PollId::default();
        assert_eq!(id.get(), 0);
    }

    #[test]
    fn test_poll_id_from_i64() {
        let id = PollId::from(456);
        assert_eq!(id.get(), 456);
    }

    #[test]
    fn test_poll_id_to_i64() {
        let id = PollId::new(789);
        let value: i64 = id.into();
        assert_eq!(value, 789);
    }

    #[test]
    fn test_poll_id_is_valid() {
        assert!(PollId::new(1).is_valid());
        assert!(PollId::new(-1).is_valid());
        assert!(!PollId::new(0).is_valid());
    }

    #[test]
    fn test_poll_id_display() {
        let id = PollId::new(123);
        assert_eq!(format!("{}", id), "poll 123");
    }

    #[test]
    fn test_poll_id_equality() {
        let id1 = PollId::new(123);
        let id2 = PollId::new(123);
        let id3 = PollId::new(456);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_poll_id_ordering() {
        let id1 = PollId::new(100);
        let id2 = PollId::new(200);

        assert!(id1 < id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_poll_id_local_vs_server() {
        let local = PollId::new(-1);
        let server = PollId::new(1);

        assert!(local.is_valid());
        assert!(server.is_valid());
        assert!(local < server); // Negative is less than positive
    }
}
