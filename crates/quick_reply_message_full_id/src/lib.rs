// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Quick Reply Message Full ID
//!
//! Combined identifier for quick reply messages.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_quick_reply_shortcut_id::QuickReplyShortcutId;
use rustgram_types::MessageId;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Quick reply message full identifier.
///
/// Combines a shortcut ID with a message ID.
/// Based on TDLib's `QuickReplyMessageFullId` struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QuickReplyMessageFullId {
    shortcut_id: QuickReplyShortcutId,
    message_id: MessageId,
}

impl QuickReplyMessageFullId {
    /// Creates a new quick reply message full ID.
    #[must_use]
    pub const fn new(shortcut_id: QuickReplyShortcutId, message_id: MessageId) -> Self {
        Self {
            shortcut_id,
            message_id,
        }
    }

    /// Returns the shortcut ID.
    #[must_use]
    pub const fn shortcut_id(&self) -> QuickReplyShortcutId {
        self.shortcut_id
    }

    /// Returns the message ID.
    #[must_use]
    pub const fn message_id(&self) -> MessageId {
        self.message_id
    }

    /// Checks if this is a valid ID.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.shortcut_id.is_valid() && self.message_id.is_valid()
    }

    /// Checks if this is a server ID.
    #[must_use]
    pub fn is_server(&self) -> bool {
        self.shortcut_id.is_valid() && self.message_id.is_server()
    }
}

impl Hash for QuickReplyMessageFullId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.shortcut_id.hash(state);
        self.message_id.hash(state);
    }
}

impl fmt::Display for QuickReplyMessageFullId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} from {}",
            self.message_id.get(),
            self.shortcut_id.get()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let shortcut_id = QuickReplyShortcutId::new(123);
        let message_id = MessageId::from_server_id(456);
        let full_id = QuickReplyMessageFullId::new(shortcut_id, message_id);
        assert_eq!(full_id.shortcut_id().get(), 123);
    }

    #[test]
    fn test_is_valid() {
        let shortcut_id = QuickReplyShortcutId::new(123);
        let message_id = MessageId::from_server_id(456);
        let full_id = QuickReplyMessageFullId::new(shortcut_id, message_id);
        assert!(full_id.is_valid());
    }

    #[test]
    fn test_is_server() {
        let shortcut_id = QuickReplyShortcutId::new(123);
        let message_id = MessageId::from_server_id(456);
        let full_id = QuickReplyMessageFullId::new(shortcut_id, message_id);
        assert!(full_id.is_server());
    }

    #[test]
    fn test_hash() {
        let shortcut_id = QuickReplyShortcutId::new(123);
        let message_id = MessageId::from_server_id(456);
        let id1 = QuickReplyMessageFullId::new(shortcut_id, message_id);
        let id2 = QuickReplyMessageFullId::new(shortcut_id, message_id);
        use std::collections::hash_map::DefaultHasher;
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        id1.hash(&mut h1);
        id2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }
}
