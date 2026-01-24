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

//! # Message Reactor
//!
//! Represents a dialog that reacted to a message.
//!
//! A reactor can be a user, chat, or channel that added a reaction to a message.
//! This includes anonymous reactions where the reactor is not identified.
//!
//! ## TDLib Alignment
//!
//! This type aligns with TDLib's `MessageReactor` class.
//! - TDLib header: `td/telegram/MessageReactor.h`
//! - TDLib type: Class with dialog_id, count, is_me, is_anonymous fields
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_reactor::MessageReactor;
//! use rustgram_types::{DialogId, UserId};
//!
//! let user_id = UserId::new(123).unwrap();
//! let dialog_id = DialogId::from_user(user_id);
//! let reactor = MessageReactor::new(dialog_id, 5, false, false);
//!
//! assert!(reactor.is_valid());
//! assert_eq!(reactor.count(), 5);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used))]

use rustgram_types::DialogId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A dialog that reacted to a message.
///
/// Contains information about who reacted to a message, including:
/// - The dialog that reacted (user, chat, or channel)
/// - The reaction count (for multiple identical reactions)
/// - Whether this is the current user's reaction
/// - Whether this is an anonymous reaction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageReactor {
    /// The dialog that reacted (empty for anonymous reactions)
    dialog_id: DialogId,

    /// Number of identical reactions from this dialog
    count: i32,

    /// Whether this is the current user's reaction
    is_me: bool,

    /// Whether this is an anonymous reaction
    is_anonymous: bool,
}

impl MessageReactor {
    /// Creates a new MessageReactor.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog that reacted
    /// * `count` - Number of identical reactions
    /// * `is_me` - Whether this is the current user
    /// * `is_anonymous` - Whether this is anonymous
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_reactor::MessageReactor;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let reactor = MessageReactor::new(dialog_id, 1, true, false);
    /// ```
    #[must_use]
    pub fn new(dialog_id: DialogId, count: i32, is_me: bool, is_anonymous: bool) -> Self {
        Self {
            dialog_id,
            count,
            is_me,
            is_anonymous,
        }
    }

    /// Returns the dialog ID of the reactor.
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the reaction count.
    #[must_use]
    pub const fn count(&self) -> i32 {
        self.count
    }

    /// Returns `true` if this is the current user's reaction.
    #[must_use]
    pub const fn is_me(&self) -> bool {
        self.is_me
    }

    /// Returns `true` if this is an anonymous reaction.
    #[must_use]
    pub const fn is_anonymous(&self) -> bool {
        self.is_anonymous
    }

    /// Returns `true` if this reactor is valid.
    ///
    /// A reactor is valid if:
    /// - count > 0
    /// - For "me" reactions: dialog_id must be valid
    /// - For anonymous reactions: allowed
    /// - For other reactions: dialog_id must be valid
    #[must_use]
    pub fn is_valid(&self) -> bool {
        if self.count <= 0 {
            return false;
        }

        if self.is_me {
            self.dialog_id.is_valid()
        } else if self.is_anonymous {
            true
        } else {
            self.dialog_id.is_valid()
        }
    }

    /// Adds to the reaction count.
    ///
    /// # Arguments
    ///
    /// * `amount` - Amount to add (can be negative)
    pub fn add_count(&mut self, amount: i32) {
        self.count = (self.count + amount).max(0);
    }

    /// Sets the dialog ID.
    pub fn set_dialog_id(&mut self, dialog_id: DialogId) {
        self.dialog_id = dialog_id;
    }
}

impl Default for MessageReactor {
    fn default() -> Self {
        Self {
            dialog_id: DialogId::default(),
            count: 0,
            is_me: false,
            is_anonymous: false,
        }
    }
}

impl fmt::Display for MessageReactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_anonymous {
            write!(f, "anonymous reactor (count: {})", self.count)
        } else {
            write!(f, "{} (count: {})", self.dialog_id, self.count)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    // Constructor tests (2)
    #[test]
    fn test_new() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let reactor = MessageReactor::new(dialog_id, 5, false, false);
        assert_eq!(reactor.count(), 5);
        assert!(!reactor.is_me());
    }

    #[test]
    fn test_default() {
        let reactor = MessageReactor::default();
        assert_eq!(reactor.count(), 0);
        assert!(!reactor.is_valid());
    }

    // Property tests (6)
    #[test]
    fn test_dialog_id() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let reactor = MessageReactor::new(dialog_id, 1, false, false);
        assert_eq!(reactor.dialog_id(), dialog_id);
    }

    #[test]
    fn test_count() {
        let reactor = MessageReactor::new(DialogId::default(), 5, false, false);
        assert_eq!(reactor.count(), 5);
    }

    #[test]
    fn test_is_me() {
        let reactor = MessageReactor::new(DialogId::default(), 1, true, false);
        assert!(reactor.is_me());
    }

    #[test]
    fn test_is_anonymous() {
        let reactor = MessageReactor::new(DialogId::default(), 1, false, true);
        assert!(reactor.is_anonymous());
    }

    #[test]
    fn test_is_valid_normal() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let reactor = MessageReactor::new(dialog_id, 1, false, false);
        assert!(reactor.is_valid());
    }

    #[test]
    fn test_is_valid_anonymous() {
        let reactor = MessageReactor::new(DialogId::default(), 1, false, true);
        assert!(reactor.is_valid());
    }

    // Invalid tests (3)
    #[test]
    fn test_is_invalid_zero_count() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let reactor = MessageReactor::new(dialog_id, 0, false, false);
        assert!(!reactor.is_valid());
    }

    #[test]
    fn test_is_invalid_negative_count() {
        let reactor = MessageReactor::new(DialogId::default(), -1, false, false);
        assert!(!reactor.is_valid());
    }

    #[test]
    fn test_is_invalid_empty_dialog() {
        let reactor = MessageReactor::new(DialogId::default(), 1, false, false);
        assert!(!reactor.is_valid());
    }

    // Method tests (3)
    #[test]
    fn test_add_count() {
        let mut reactor = MessageReactor::new(DialogId::default(), 5, false, true);
        reactor.add_count(2);
        assert_eq!(reactor.count(), 7);
    }

    #[test]
    fn test_add_count_clamped() {
        let mut reactor = MessageReactor::new(DialogId::default(), 1, false, true);
        reactor.add_count(-5);
        assert_eq!(reactor.count(), 0);
    }

    #[test]
    fn test_set_dialog_id() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let mut reactor = MessageReactor::new(DialogId::default(), 1, false, true);
        reactor.set_dialog_id(dialog_id);
        assert_eq!(reactor.dialog_id(), dialog_id);
    }

    // Equality tests (3)
    #[test]
    fn test_equality_equal() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let reactor1 = MessageReactor::new(dialog_id, 5, false, false);
        let reactor2 = MessageReactor::new(dialog_id, 5, false, false);
        assert_eq!(reactor1, reactor2);
    }

    #[test]
    fn test_equality_different_count() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let reactor1 = MessageReactor::new(dialog_id, 5, false, false);
        let reactor2 = MessageReactor::new(dialog_id, 10, false, false);
        assert_ne!(reactor1, reactor2);
    }

    #[test]
    fn test_equality_different_is_me() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let reactor1 = MessageReactor::new(dialog_id, 5, false, false);
        let reactor2 = MessageReactor::new(dialog_id, 5, true, false);
        assert_ne!(reactor1, reactor2);
    }

    // Clone tests (2)
    #[test]
    fn test_clone() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let reactor1 = MessageReactor::new(dialog_id, 5, true, false);
        let reactor2 = reactor1.clone();
        assert_eq!(reactor1, reactor2);
    }

    #[test]
    fn test_clone_independence() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let mut reactor1 = MessageReactor::new(dialog_id, 5, false, false);
        let reactor2 = reactor1.clone();
        reactor1.add_count(10);
        assert_eq!(reactor2.count(), 5);
    }

    // Display tests (2)
    #[test]
    fn test_display_normal() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let reactor = MessageReactor::new(dialog_id, 5, false, false);
        let display = format!("{}", reactor);
        assert!(display.contains("5"));
    }

    #[test]
    fn test_display_anonymous() {
        let reactor = MessageReactor::new(DialogId::default(), 1, false, true);
        let display = format!("{}", reactor);
        assert!(display.contains("anonymous"));
    }

    // Serialization tests (2)
    #[test]
    fn test_serialize() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let reactor = MessageReactor::new(dialog_id, 5, true, false);
        let json = serde_json::to_string(&reactor).unwrap();
        let parsed: MessageReactor = serde_json::from_str(&json).unwrap();
        assert_eq!(reactor, parsed);
    }

    #[test]
    fn test_serialize_anonymous() {
        let reactor = MessageReactor::new(DialogId::default(), 1, false, true);
        let json = serde_json::to_string(&reactor).unwrap();
        let parsed: MessageReactor = serde_json::from_str(&json).unwrap();
        assert_eq!(reactor, parsed);
    }
}
