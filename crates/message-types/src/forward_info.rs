// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0

//! Message forward information.

use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Information about a forwarded message.
///
/// This is a simplified stub of TDLib's MessageForwardInfo.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageForwardInfo {
    origin_message_id: MessageId,
    origin_dialog_id: DialogId,
    origin_sender_id: DialogId,
    origin_date: i32,
}

impl MessageForwardInfo {
    /// Creates minimal forward info with just the origin message ID.
    pub fn new(origin_message_id: MessageId) -> Self {
        Self {
            origin_message_id,
            origin_dialog_id: DialogId::default(),
            origin_sender_id: DialogId::default(),
            origin_date: 0,
        }
    }

    /// Returns the origin message ID.
    pub const fn origin_message_id(&self) -> MessageId {
        self.origin_message_id
    }

    /// Returns the origin dialog ID.
    pub const fn origin_dialog_id(&self) -> DialogId {
        self.origin_dialog_id
    }

    /// Returns the origin sender ID.
    pub const fn origin_sender_id(&self) -> DialogId {
        self.origin_sender_id
    }

    /// Returns the origin date (Unix timestamp).
    pub const fn origin_date(&self) -> i32 {
        self.origin_date
    }

    /// Returns true if this has detailed forward information.
    pub fn has_details(&self) -> bool {
        self.origin_date > 0
    }
}

impl Default for MessageForwardInfo {
    fn default() -> Self {
        Self {
            origin_message_id: MessageId::default(),
            origin_dialog_id: DialogId::default(),
            origin_sender_id: DialogId::default(),
            origin_date: 0,
        }
    }
}

impl fmt::Display for MessageForwardInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "forward from {}", self.origin_message_id)
    }
}
