// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Replied Message Info
//!
//! Information about a replied message.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};

/// Information about a replied message.
///
/// Based on TDLib's `RepliedMessageInfo` class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepliedMessageInfo {
    /// Message ID being replied to.
    message_id: MessageId,
    /// Dialog ID (for replies from other chats).
    dialog_id: DialogId,
}

impl Default for RepliedMessageInfo {
    fn default() -> Self {
        Self {
            message_id: MessageId::default(),
            dialog_id: DialogId::default(),
        }
    }
}

impl RepliedMessageInfo {
    /// Creates a legacy replied message info.
    #[must_use]
    pub const fn legacy(message_id: MessageId, dialog_id: DialogId) -> Self {
        Self {
            message_id,
            dialog_id,
        }
    }

    /// Returns the message ID.
    #[must_use]
    pub const fn message_id(&self) -> MessageId {
        self.message_id
    }

    /// Returns the dialog ID.
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Checks if this is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.message_id == MessageId::default() && self.dialog_id == DialogId::default()
    }

    /// Checks if this is an external reply.
    #[must_use]
    pub fn is_external(&self) -> bool {
        self.dialog_id != DialogId::default()
    }

    /// Sets the message ID.
    pub fn set_message_id(&mut self, message_id: MessageId) {
        self.message_id = message_id;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    #[test]
    fn test_default() {
        let info = RepliedMessageInfo::default();
        assert!(info.is_empty());
    }

    #[test]
    fn test_legacy() {
        let msg_id = MessageId::from_server_id(123);
        let info = RepliedMessageInfo::legacy(msg_id, DialogId::default());
        assert_eq!(info.message_id().get(), 123 << 20);
    }

    #[test]
    fn test_is_external() {
        let msg_id = MessageId::from_server_id(123);
        let dialog_id = DialogId::from_user(UserId::new(456).unwrap());
        let info = RepliedMessageInfo::legacy(msg_id, dialog_id);
        assert!(info.is_external());
    }

    #[test]
    fn test_set_message_id() {
        let mut info = RepliedMessageInfo::default();
        let new_id = MessageId::from_server_id(789);
        info.set_message_id(new_id);
        assert_eq!(info.message_id().get(), 789 << 20);
    }
}
