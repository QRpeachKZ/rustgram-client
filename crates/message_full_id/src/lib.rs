// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Message Full ID
//!
//! A combination of DialogId and MessageId.
//!
//! ## Overview
//!
//! Represents the full identifier of a message, combining both the dialog ID
//! and the message ID into a single compound type.
//!

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Full identifier for a message.
///
/// Combines a dialog ID and message ID into a single compound type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageFullId {
    dialog_id: DialogId,
    message_id: MessageId,
}

impl MessageFullId {
    /// Creates a new message full ID.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    /// * `message_id` - The message ID
    #[must_use]
    pub const fn new(dialog_id: DialogId, message_id: MessageId) -> Self {
        Self {
            dialog_id,
            message_id,
        }
    }

    /// Returns the dialog ID.
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the message ID.
    #[must_use]
    pub const fn message_id(&self) -> MessageId {
        self.message_id
    }

    /// Returns whether the message ID is valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.message_id.is_valid()
    }
}

impl Hash for MessageFullId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dialog_id.hash(state);
        self.message_id.hash(state);
    }
}

impl fmt::Display for MessageFullId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} in {}", self.message_id, self.dialog_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    #[test]
    fn test_new() {
        let dialog_id = DialogId::from_user(UserId::new(123).unwrap());
        let message_id = MessageId::from_server_id(456);
        let full_id = MessageFullId::new(dialog_id, message_id);
        assert_eq!(full_id.dialog_id(), dialog_id);
    }
}
