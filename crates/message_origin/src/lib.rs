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

//! # Message Origin
//!
//! Information about the origin of a forwarded message.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `MessageOrigin` functionality.
//! It contains information about who originally sent a message that was forwarded.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_origin::MessageOrigin;
//! use rustgram_types::{DialogId, MessageId, UserId};
//!
//! let origin = MessageOrigin::new(
//!     UserId::new(123).ok(),
//!     DialogId::default(),
//!     MessageId::from_server_id(100),
//! );
//! assert!(!origin.is_empty());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use rustgram_types::{DialogId, MessageId, UserId};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Message origin.
///
/// Contains information about the original sender of a forwarded message.
///
/// # TDLib Correspondence
///
/// - TDLib Type: `MessageOrigin`
/// - Fields: sender_user_id, sender_dialog_id, message_id, author_signature, sender_name
///
/// # Example
///
/// ```rust
/// use rustgram_message_origin::MessageOrigin;
/// use rustgram_types::{DialogId, MessageId, UserId};
///
/// let origin = MessageOrigin::new(
///     UserId::new(123).ok(),
///     DialogId::default(),
///     MessageId::from_server_id(100),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageOrigin {
    sender_user_id: Option<UserId>,
    sender_dialog_id: Option<DialogId>,
    message_id: MessageId,
    author_signature: String,
    sender_name: String,
}

impl MessageOrigin {
    /// Creates a new message origin.
    ///
    /// # Arguments
    ///
    /// * `sender_user_id` - User ID of the original sender
    /// * `sender_dialog_id` - Dialog ID of the original sender
    /// * `message_id` - Message ID of the original message
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_origin::MessageOrigin;
    /// use rustgram_types::{DialogId, MessageId, UserId};
    ///
    /// let origin = MessageOrigin::new(
    ///     UserId::new(123).ok(),
    ///     DialogId::default(),
    ///     MessageId::from_server_id(100),
    /// );
    /// ```
    pub fn new(
        sender_user_id: Option<UserId>,
        sender_dialog_id: Option<DialogId>,
        message_id: MessageId,
    ) -> Self {
        Self {
            sender_user_id,
            sender_dialog_id,
            message_id,
            author_signature: String::new(),
            sender_name: String::new(),
        }
    }

    /// Creates a new message origin with signature.
    ///
    /// # Arguments
    ///
    /// * `sender_user_id` - User ID of the original sender
    /// * `sender_dialog_id` - Dialog ID of the original sender
    /// * `message_id` - Message ID of the original message
    /// * `author_signature` - Author signature (for channels)
    /// * `sender_name` - Sender name (for anonymous admins)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_origin::MessageOrigin;
    /// use rustgram_types::{DialogId, MessageId, UserId};
    ///
    /// let origin = MessageOrigin::with_signature(
    ///     UserId::new(123).ok(),
    ///     DialogId::default(),
    ///     MessageId::from_server_id(100),
    ///     "author".to_string(),
    ///     "name".to_string(),
    /// );
    /// ```
    pub fn with_signature(
        sender_user_id: Option<UserId>,
        sender_dialog_id: Option<DialogId>,
        message_id: MessageId,
        author_signature: String,
        sender_name: String,
    ) -> Self {
        Self {
            sender_user_id,
            sender_dialog_id,
            message_id,
            author_signature,
            sender_name,
        }
    }

    /// Returns the sender user ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_origin::MessageOrigin;
    /// use rustgram_types::{DialogId, MessageId, UserId};
    ///
    /// let origin = MessageOrigin::new(
    ///     UserId::new(123).ok(),
    ///     DialogId::default(),
    ///     MessageId::from_server_id(100),
    /// );
    /// assert_eq!(origin.sender_user_id(), UserId::new(123).ok());
    /// ```
    #[must_use]
    pub const fn sender_user_id(&self) -> Option<UserId> {
        self.sender_user_id
    }

    /// Returns the sender dialog ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_origin::MessageOrigin;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let dialog_id = DialogId::default();
    /// let origin = MessageOrigin::new(None, Some(dialog_id), MessageId::from_server_id(100));
    /// assert_eq!(origin.sender_dialog_id(), Some(dialog_id));
    /// ```
    #[must_use]
    pub const fn sender_dialog_id(&self) -> Option<DialogId> {
        self.sender_dialog_id
    }

    /// Returns the message ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_origin::MessageOrigin;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let origin = MessageOrigin::new(None, DialogId::default(), MessageId::from_server_id(100));
    /// assert_eq!(origin.message_id(), MessageId::from_server_id(100));
    /// ```
    #[must_use]
    pub const fn message_id(&self) -> MessageId {
        self.message_id
    }

    /// Returns the author signature.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_origin::MessageOrigin;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let origin = MessageOrigin::with_signature(
    ///     None,
    ///     DialogId::default(),
    ///     MessageId::from_server_id(100),
    ///     "author".to_string(),
    ///     String::new(),
    /// );
    /// assert_eq!(origin.author_signature(), "author");
    /// ```
    #[must_use]
    pub fn author_signature(&self) -> &str {
        &self.author_signature
    }

    /// Returns the sender name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_origin::MessageOrigin;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let origin = MessageOrigin::with_signature(
    ///     None,
    ///     DialogId::default(),
    ///     MessageId::from_server_id(100),
    ///     String::new(),
    ///     "name".to_string(),
    /// );
    /// assert_eq!(origin.sender_name(), "name");
    /// ```
    #[must_use]
    pub fn sender_name(&self) -> &str {
        &self.sender_name
    }

    /// Checks if this origin is empty (no sender information).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_origin::MessageOrigin;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let origin = MessageOrigin::new(None, DialogId::default(), MessageId::default());
    /// assert!(origin.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sender_user_id.map_or(true, |id| !id.is_valid())
            && self.sender_dialog_id.map_or(true, |id| !id.is_valid())
            && !self.message_id.is_valid()
            && self.author_signature.is_empty()
            && self.sender_name.is_empty()
    }

    /// Checks if the sender is hidden.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_origin::MessageOrigin;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let origin = MessageOrigin::new(None, DialogId::default(), MessageId::from_server_id(100));
    /// assert!(origin.is_sender_hidden());
    /// ```
    #[must_use]
    pub fn is_sender_hidden(&self) -> bool {
        self.sender_user_id.map_or(true, |id| !id.is_valid())
    }

    /// Checks if this is a channel post (has valid message ID).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_origin::MessageOrigin;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let origin = MessageOrigin::new(None, DialogId::default(), MessageId::from_server_id(100));
    /// assert!(origin.is_channel_post());
    /// ```
    #[must_use]
    pub fn is_channel_post(&self) -> bool {
        self.message_id.is_valid()
    }

    /// Checks if this origin has a sender signature.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_origin::MessageOrigin;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let origin = MessageOrigin::with_signature(
    ///     None,
    ///     DialogId::default(),
    ///     MessageId::from_server_id(100),
    ///     "author".to_string(),
    ///     String::new(),
    /// );
    /// assert!(origin.has_sender_signature());
    /// ```
    #[must_use]
    pub fn has_sender_signature(&self) -> bool {
        !self.author_signature.is_empty() || !self.sender_name.is_empty()
    }

    /// Returns the sender dialog ID (user or sender_dialog_id).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_origin::MessageOrigin;
    /// use rustgram_types::{DialogId, MessageId, UserId};
    ///
    /// let user_id = UserId::new(123).unwrap();
    /// let origin = MessageOrigin::new(Some(user_id), DialogId::default(), MessageId::from_server_id(100));
    /// assert_eq!(origin.get_sender(), DialogId::from(user_id));
    /// ```
    #[must_use]
    pub fn get_sender(&self) -> DialogId {
        self.sender_user_id
            .map(DialogId::from)
            .or(self.sender_dialog_id)
            .unwrap_or_default()
    }

    /// Sets the author signature.
    ///
    /// # Arguments
    ///
    /// * `signature` - Author signature to set
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_origin::MessageOrigin;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let mut origin = MessageOrigin::new(None, DialogId::default(), MessageId::from_server_id(100));
    /// origin.set_author_signature("author");
    /// assert_eq!(origin.author_signature(), "author");
    /// ```
    pub fn set_author_signature(&mut self, signature: &str) {
        self.author_signature = signature.to_string();
    }

    /// Sets the sender name.
    ///
    /// # Arguments
    ///
    /// * `name` - Sender name to set
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_origin::MessageOrigin;
    /// use rustgram_types::{DialogId, MessageId};
    ///
    /// let mut origin = MessageOrigin::new(None, DialogId::default(), MessageId::from_server_id(100));
    /// origin.set_sender_name("name");
    /// assert_eq!(origin.sender_name(), "name");
    /// ```
    pub fn set_sender_name(&mut self, name: &str) {
        self.sender_name = name.to_string();
    }
}

impl fmt::Display for MessageOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(user_id) = self.sender_user_id {
            write!(f, "user {}", user_id.get())?;
        } else if let Some(dialog_id) = self.sender_dialog_id {
            write!(f, "dialog {}", dialog_id)?;
        }

        if self.message_id.is_valid() {
            write!(f, ", msg {}", self.message_id.get())?;
        }

        if !self.author_signature.is_empty() {
            write!(f, ", signed: {}", self.author_signature)?;
        }

        if !self.sender_name.is_empty() {
            write!(f, ", name: {}", self.sender_name)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            DialogId::default(),
            MessageId::from_server_id(100),
        );
        assert_eq!(origin.sender_user_id(), UserId::new(123).ok());
        assert_eq!(origin.message_id(), MessageId::from_server_id(100));
    }

    #[test]
    fn test_with_signature() {
        let origin = MessageOrigin::with_signature(
            UserId::new(123).ok(),
            DialogId::default(),
            MessageId::from_server_id(100),
            "author".to_string(),
            "name".to_string(),
        );
        assert_eq!(origin.author_signature(), "author");
        assert_eq!(origin.sender_name(), "name");
    }

    #[test]
    fn test_default() {
        let origin = MessageOrigin::default();
        assert!(origin.is_empty());
    }

    #[test]
    fn test_is_empty() {
        let origin = MessageOrigin::new(None, DialogId::default(), MessageId::default());
        assert!(origin.is_empty());

        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            DialogId::default(),
            MessageId::from_server_id(100),
        );
        assert!(!origin.is_empty());
    }

    #[test]
    fn test_is_sender_hidden() {
        let origin = MessageOrigin::new(None, DialogId::default(), MessageId::from_server_id(100));
        assert!(origin.is_sender_hidden());

        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            DialogId::default(),
            MessageId::from_server_id(100),
        );
        assert!(!origin.is_sender_hidden());
    }

    #[test]
    fn test_is_channel_post() {
        let origin = MessageOrigin::new(None, DialogId::default(), MessageId::from_server_id(100));
        assert!(origin.is_channel_post());

        let origin = MessageOrigin::new(None, DialogId::default(), MessageId::default());
        assert!(!origin.is_channel_post());
    }

    #[test]
    fn test_has_sender_signature() {
        let origin = MessageOrigin::new(None, DialogId::default(), MessageId::from_server_id(100));
        assert!(!origin.has_sender_signature());

        let origin = MessageOrigin::with_signature(
            None,
            DialogId::default(),
            MessageId::from_server_id(100),
            "author".to_string(),
            String::new(),
        );
        assert!(origin.has_sender_signature());
    }

    #[test]
    fn test_get_sender() {
        let user_id = UserId::new(123).unwrap();
        let origin = MessageOrigin::new(
            Some(user_id),
            DialogId::default(),
            MessageId::from_server_id(100),
        );
        assert_eq!(origin.get_sender(), DialogId::from(user_id));
    }

    #[test]
    fn test_set_author_signature() {
        let mut origin =
            MessageOrigin::new(None, DialogId::default(), MessageId::from_server_id(100));
        origin.set_author_signature("author");
        assert_eq!(origin.author_signature(), "author");
    }

    #[test]
    fn test_set_sender_name() {
        let mut origin =
            MessageOrigin::new(None, DialogId::default(), MessageId::from_server_id(100));
        origin.set_sender_name("name");
        assert_eq!(origin.sender_name(), "name");
    }

    #[test]
    fn test_display() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            DialogId::default(),
            MessageId::from_server_id(100),
        );
        let s = format!("{}", origin);
        assert!(s.contains("user"));
        assert!(s.contains("123"));
    }

    #[test]
    fn test_equality() {
        let origin1 = MessageOrigin::new(
            UserId::new(123).ok(),
            DialogId::default(),
            MessageId::from_server_id(100),
        );
        let origin2 = MessageOrigin::new(
            UserId::new(123).ok(),
            DialogId::default(),
            MessageId::from_server_id(100),
        );
        assert_eq!(origin1, origin2);

        let origin3 = MessageOrigin::new(
            UserId::new(456).ok(),
            DialogId::default(),
            MessageId::from_server_id(100),
        );
        assert_ne!(origin1, origin3);
    }

    #[test]
    fn test_serialization() {
        let origin = MessageOrigin::new(
            UserId::new(123).ok(),
            DialogId::default(),
            MessageId::from_server_id(100),
        );
        let json = serde_json::to_string(&origin).unwrap();
        let parsed: MessageOrigin = serde_json::from_str(&json).unwrap();
        assert_eq!(origin, parsed);
    }
}
