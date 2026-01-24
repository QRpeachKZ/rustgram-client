// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Quick Reply Shortcut ID
//!
//! Quick reply shortcut identifier.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Quick reply shortcut identifier.
///
/// Based on TDLib's `QuickReplyShortcutId` class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct QuickReplyShortcutId(i32);

impl QuickReplyShortcutId {
    /// Maximum server shortcut ID.
    pub const MAX_SERVER_SHORTCUT_ID: i32 = 1_999_999_999;

    /// Creates a new quick reply shortcut ID.
    #[must_use]
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    /// Returns the inner ID value.
    #[must_use]
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Checks if this is a valid shortcut ID.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 > 0
    }

    /// Checks if this is a server shortcut ID.
    #[must_use]
    pub const fn is_server(self) -> bool {
        self.0 > 0 && self.0 <= Self::MAX_SERVER_SHORTCUT_ID
    }

    /// Checks if this is a local shortcut ID.
    #[must_use]
    pub const fn is_local(self) -> bool {
        self.0 > Self::MAX_SERVER_SHORTCUT_ID
    }
}

impl fmt::Display for QuickReplyShortcutId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "shortcut {}", self.0)
    }
}

impl From<i32> for QuickReplyShortcutId {
    fn from(id: i32) -> Self {
        Self(id)
    }
}

impl From<QuickReplyShortcutId> for i32 {
    fn from(id: QuickReplyShortcutId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = QuickReplyShortcutId::new(123);
        assert_eq!(id.get(), 123);
    }

    #[test]
    fn test_is_valid() {
        let valid = QuickReplyShortcutId::new(123);
        assert!(valid.is_valid());

        let invalid = QuickReplyShortcutId::new(0);
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_is_server() {
        let server = QuickReplyShortcutId::new(123);
        assert!(server.is_server());
        assert!(!server.is_local());
    }

    #[test]
    fn test_is_local() {
        let local = QuickReplyShortcutId::new(2_000_000_000);
        assert!(local.is_local());
        assert!(!local.is_server());
    }
}
