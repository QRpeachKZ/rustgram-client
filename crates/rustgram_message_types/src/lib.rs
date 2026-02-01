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

//! # Message Types
//!
//! Core message type definitions for Telegram messages.
//!
//! ## Overview
//!
//! This crate provides the [`Message`] struct which represents a Telegram message
//! with all its metadata and content. This is a simplified version of TDLib's
//! Message class, implementing only the core fields needed for Phase 1 Basic
//! Messaging.
//!
//! ## TDLib Reference
//!
//! Corresponds to `td/telegram/Message.h` and `Message.cpp` in TDLib.
//!
//! ## Simplification from TDLib
//!
//! TDLib's Message class has 200+ fields including:
//! - Complex nested structures for media
//! - Reaction data
//! - Thread information
//! - Story data
//! - Sponsored message flags
//!
//! Phase 1 implements only:
//! - Core identifiers (id, dialog_id, sender_id)
//! - Timestamps (date, edit_date)
//! - Text content only
//! - Basic reply-to support
//! - Optional forward info (stub)
//!
//! This represents approximately 7% of TDLib's Message functionality.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_types::{Message, MessageValidationError};
//! use rustgram_types::{DialogId, MessageId};
//! use rustgram_formatted_text::FormattedText;
//!
//! // Create a new text message
//! let text = FormattedText::new("Hello, world!");
//! let message = Message::new_with_date(
//!     MessageId::from_server_id(123),
//!     DialogId::from_user(rustgram_types::UserId::new(456).unwrap()),
//!     DialogId::from_user(rustgram_types::UserId::new(789).unwrap()),
//!     1234567890,
//!     text,
//! )?;
//!
//! assert!(message.is_valid());
//! assert!(message.is_text());
//! assert!(!message.is_edited());
//! # Ok::<(), MessageValidationError>(())
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_formatted_text::FormattedText;
use rustgram_message_content::MessageContent;
use rustgram_message_content_type::MessageContentType;
use rustgram_message_forward_info::MessageForwardInfo;
use rustgram_message_input_reply_to::MessageInputReplyTo;
use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================================
// Errors
// ============================================================================

/// Errors that can occur when creating or validating a message.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MessageValidationError {
    /// Message ID is invalid
    #[error("Invalid message ID: {0}")]
    InvalidMessageId(String),

    /// Dialog ID is invalid
    #[error("Invalid dialog ID: {0}")]
    InvalidDialogId(String),

    /// Sender ID is invalid
    #[error("Invalid sender ID: {0}")]
    InvalidSenderId(String),

    /// Message content is empty or invalid
    #[error("Invalid message content: {0}")]
    InvalidContent(String),

    /// Message date is invalid (must be positive)
    #[error("Invalid message date: {0}, must be positive")]
    InvalidDate(i32),

    /// Edit date is before message date
    #[error("Edit date ({edit_date}) is before message date ({message_date})")]
    InvalidEditDate {
        /// The message's original date
        message_date: i32,
        /// The invalid edit date
        edit_date: i32,
    },
}

// ============================================================================
// Message Struct
// ============================================================================

/// Core message structure for Telegram messages.
///
/// This struct represents a simplified Telegram message with only the essential
/// fields needed for Phase 1 Basic Messaging. It is designed to be extensible
/// for future phases.
///
/// # Field Descriptions
///
/// - `id` - Unique message identifier (server-assigned for received messages,
///   temporary for outgoing messages)
/// - `dialog_id` - Dialog containing this message
/// - `sender_id` - Sender of the message (can be the same as dialog_id for
///   private messages)
/// - `date` - Unix timestamp when message was sent
/// - `content` - Message content (text-only for Phase 1)
/// - `reply_to` - What this message replies to (optional)
/// - `edit_date` - Last edit timestamp if edited (optional)
/// - `views` - View count for channel messages (optional)
/// - `forward_info` - Forward information if forwarded (optional, stub)
///
/// # TDLib Alignment
///
/// This corresponds to TDLib's `Message` class but with only ~7% of the fields.
/// Missing fields will be added in future phases as needed.
///
/// # Example
///
/// ```rust
/// use rustgram_message_types::Message;
/// use rustgram_types::{DialogId, MessageId, UserId};
/// use rustgram_formatted_text::FormattedText;
///
/// let text = FormattedText::new("Hello!");
/// let message = Message::new(
///     MessageId::from_server_id(1),
///     DialogId::from_user(UserId::new(123).unwrap()),
///     DialogId::from_user(UserId::new(456).unwrap()),
///     text,
/// ).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier
    pub id: MessageId,

    /// Dialog containing this message
    pub dialog_id: DialogId,

    /// Sender of the message
    pub sender_id: DialogId,

    /// Unix timestamp when message was sent
    pub date: i32,

    /// Message content (text-only for Phase 1)
    pub content: MessageContent,

    /// What this message replies to (optional)
    #[serde(default)]
    pub reply_to: Option<MessageInputReplyTo>,

    /// Last edit timestamp if edited (optional)
    #[serde(default)]
    pub edit_date: Option<i32>,

    /// View count for channels (optional)
    #[serde(default)]
    pub views: Option<i32>,

    /// Forward information (stub)
    #[serde(default)]
    pub forward_info: Option<MessageForwardInfo>,
}

impl Message {
    /// Maximum message text length in bytes
    pub const MAX_TEXT_LENGTH: usize = 4096;

    /// Creates a new text message.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique message identifier
    /// * `dialog_id` - Dialog containing this message
    /// * `sender_id` - Sender of the message
    /// * `text` - Message text content
    ///
    /// # Errors
    ///
    /// Returns [`MessageValidationError`] if:
    /// - Message ID is invalid
    /// - Dialog ID is invalid
    /// - Sender ID is invalid
    /// - Text is empty
    /// - Text exceeds maximum length
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_types::Message;
    /// use rustgram_types::{DialogId, MessageId};
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("Hello, world!");
    /// let message = Message::new(
    ///     MessageId::from_server_id(1),
    ///     DialogId::from_user(rustgram_types::UserId::new(123).unwrap()),
    ///     DialogId::from_user(rustgram_types::UserId::new(456).unwrap()),
    ///     text,
    /// ).unwrap();
    /// ```
    pub fn new(
        id: MessageId,
        dialog_id: DialogId,
        sender_id: DialogId,
        text: FormattedText,
    ) -> Result<Self, MessageValidationError> {
        // Validate message ID
        if !id.is_valid() {
            return Err(MessageValidationError::InvalidMessageId(
                "Message ID is not valid".to_string(),
            ));
        }

        // Validate dialog ID
        if !dialog_id.is_valid() {
            return Err(MessageValidationError::InvalidDialogId(
                "Dialog ID is not valid".to_string(),
            ));
        }

        // Validate sender ID
        if !sender_id.is_valid() {
            return Err(MessageValidationError::InvalidSenderId(
                "Sender ID is not valid".to_string(),
            ));
        }

        // Create and validate content
        let content = MessageContent::text_with_validation(text)?;

        Ok(Self {
            id,
            dialog_id,
            sender_id,
            date: 0, // Will be set by server
            content,
            reply_to: None,
            edit_date: None,
            views: None,
            forward_info: None,
        })
    }

    /// Creates a new message with the specified date.
    ///
    /// This is useful when creating messages from server responses or when
    /// constructing test messages.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique message identifier
    /// * `dialog_id` - Dialog containing this message
    /// * `sender_id` - Sender of the message
    /// * `date` - Unix timestamp
    /// * `text` - Message text content
    ///
    /// # Errors
    ///
    /// Returns [`MessageValidationError`] if validation fails or if date is not positive.
    pub fn new_with_date(
        id: MessageId,
        dialog_id: DialogId,
        sender_id: DialogId,
        date: i32,
        text: FormattedText,
    ) -> Result<Self, MessageValidationError> {
        if date <= 0 {
            return Err(MessageValidationError::InvalidDate(date));
        }

        let mut message = Self::new(id, dialog_id, sender_id, text)?;
        message.date = date;
        Ok(message)
    }

    /// Validates the message structure.
    ///
    /// Returns `true` if all fields are valid:
    /// - IDs are valid
    /// - Date is positive
    /// - Content is not empty
    /// - Edit date (if present) is after message date
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_types::Message;
    /// use rustgram_types::{DialogId, MessageId};
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("Hello!");
    /// let message = Message::new_with_date(
    ///     MessageId::from_server_id(1),
    ///     DialogId::from_user(rustgram_types::UserId::new(123).unwrap()),
    ///     DialogId::from_user(rustgram_types::UserId::new(456).unwrap()),
    ///     1234567890,
    ///     text,
    /// ).unwrap();
    ///
    /// assert!(message.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        // Validate IDs
        if !self.id.is_valid() || !self.dialog_id.is_valid() || !self.sender_id.is_valid() {
            return false;
        }

        // Validate date
        if self.date <= 0 {
            return false;
        }

        // Validate content
        if !self.content.is_valid() {
            return false;
        }

        // Validate edit date
        if let Some(edit_date) = self.edit_date {
            if edit_date <= self.date {
                return false;
            }
        }

        true
    }

    /// Returns `true` if this message was edited.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_types::Message;
    /// use rustgram_types::{DialogId, MessageId, UserId};
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("Hello!");
    /// let mut message = Message::new(
    ///     MessageId::from_server_id(1),
    ///     DialogId::from_user(UserId::new(123).unwrap()),
    ///     DialogId::from_user(UserId::new(456).unwrap()),
    ///     text,
    /// ).unwrap();
    ///
    /// assert!(!message.is_edited());
    ///
    /// message.edit_date = Some(1234567890);
    /// assert!(message.is_edited());
    /// ```
    #[must_use]
    pub const fn is_edited(&self) -> bool {
        self.edit_date.is_some()
    }

    /// Returns `true` if this is an outgoing message.
    ///
    /// An outgoing message is one where the sender is the current user.
    /// In Phase 1, this requires external context to determine.
    ///
    /// # Note
    ///
    /// This method currently always returns `false` because we don't have
    /// access to the current user's ID in this context. This will be
    /// implemented in a future phase.
    #[must_use]
    pub const fn is_outgoing(&self) -> bool {
        // TODO: Implement in future phase when we have access to current user ID
        false
    }

    /// Returns `true` if this message has text content.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_types::Message;
    /// use rustgram_types::{DialogId, MessageId, UserId};
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("Hello!");
    /// let message = Message::new(
    ///     MessageId::from_server_id(1),
    ///     DialogId::from_user(UserId::new(123).unwrap()),
    ///     DialogId::from_user(UserId::new(456).unwrap()),
    ///     text,
    /// ).unwrap();
    ///
    /// assert!(message.is_text());
    /// ```
    #[must_use]
    pub fn is_text(&self) -> bool {
        self.content.content_type() == MessageContentType::Text
    }

    /// Returns `true` if this message is a reply to another message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_types::Message;
    /// use rustgram_types::{DialogId, MessageId};
    /// use rustgram_formatted_text::FormattedText;
    /// use rustgram_message_input_reply_to::{MessageInputReplyTo, MessageQuote};
    ///
    /// let text = FormattedText::new("Reply!");
    /// let mut message = Message::new(
    ///     MessageId::from_server_id(2),
    ///     DialogId::from_user(rustgram_types::UserId::new(123).unwrap()),
    ///     DialogId::from_user(rustgram_types::UserId::new(456).unwrap()),
    ///     text,
    /// ).unwrap();
    ///
    /// assert!(!message.has_reply());
    ///
    /// message.reply_to = Some(MessageInputReplyTo::message(
    ///     MessageId::from_server_id(1),
    ///     DialogId::from_user(rustgram_types::UserId::new(123).unwrap()),
    ///     MessageQuote::new(),
    ///     0,
    /// ));
    /// assert!(message.has_reply());
    /// ```
    #[must_use]
    pub const fn has_reply(&self) -> bool {
        self.reply_to.is_some()
    }

    /// Returns `true` if this message was forwarded.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_types::Message;
    /// use rustgram_types::{DialogId, MessageId};
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("Forwarded!");
    /// let mut message = Message::new(
    ///     MessageId::from_server_id(2),
    ///     DialogId::from_user(rustgram_types::UserId::new(123).unwrap()),
    ///     DialogId::from_user(rustgram_types::UserId::new(456).unwrap()),
    ///     text,
    /// ).unwrap();
    ///
    /// assert!(!message.is_forwarded());
    ///
    /// // TODO: Add forward info test when stub is implemented
    /// // message.forward_info = Some(MessageForwardInfo::new(...));
    /// // assert!(message.is_forwarded());
    /// ```
    #[must_use]
    pub const fn is_forwarded(&self) -> bool {
        self.forward_info.is_some()
    }

    /// Returns `true` if this is a channel message (has view count).
    #[must_use]
    pub const fn is_channel_message(&self) -> bool {
        self.views.is_some()
    }

    /// Returns the content type of this message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_types::Message;
    /// use rustgram_types::{DialogId, MessageId};
    /// use rustgram_formatted_text::FormattedText;
    /// use rustgram_message_content_type::MessageContentType;
    ///
    /// let text = FormattedText::new("Hello!");
    /// let message = Message::new(
    ///     MessageId::from_server_id(1),
    ///     DialogId::from_user(rustgram_types::UserId::new(123).unwrap()),
    ///     DialogId::from_user(rustgram_types::UserId::new(456).unwrap()),
    ///     text,
    /// ).unwrap();
    ///
    /// assert_eq!(message.content_type(), MessageContentType::Text);
    /// ```
    #[must_use]
    pub fn content_type(&self) -> MessageContentType {
        self.content.content_type()
    }

    /// Sets the edit date for this message.
    ///
    /// # Arguments
    ///
    /// * `edit_date` - Unix timestamp of the edit
    ///
    /// # Errors
    ///
    /// Returns [`MessageValidationError`] if edit_date is before the message date.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_types::Message;
    /// use rustgram_types::{DialogId, MessageId, UserId};
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let text = FormattedText::new("Hello!");
    /// let mut message = Message::new(
    ///     MessageId::from_server_id(1),
    ///     DialogId::from_user(UserId::new(123).unwrap()),
    ///     DialogId::from_user(UserId::new(456).unwrap()),
    ///     text,
    /// ).unwrap();
    /// message.date = 1234567800;
    ///
    /// message.set_edit_date(1234567900).unwrap();
    /// assert!(message.is_edited());
    /// ```
    pub fn set_edit_date(&mut self, edit_date: i32) -> Result<(), MessageValidationError> {
        if edit_date <= self.date {
            return Err(MessageValidationError::InvalidEditDate {
                message_date: self.date,
                edit_date,
            });
        }

        self.edit_date = Some(edit_date);
        Ok(())
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Message(id={}, dialog={}, sender={}, type={}, date={})",
            self.id,
            self.dialog_id,
            self.sender_id,
            self.content_type(),
            self.date
        )
    }
}

// ============================================================================
// MessageContent Extension
// ============================================================================

/// Extension trait for MessageContent validation.
trait MessageContentExt {
    fn is_valid(&self) -> bool;
}

impl MessageContentExt for MessageContent {
    fn is_valid(&self) -> bool {
        match self {
            Self::Text(text_msg) => {
                !text_msg.text.text().is_empty()
                    && text_msg.text.text().len() <= Message::MAX_TEXT_LENGTH
            }
            // For Phase 1, only text content is supported
            _ => false,
        }
    }
}

/// Extension trait for creating validated text content.
trait MessageContentFactory {
    fn text_with_validation(text: FormattedText) -> Result<Self, MessageValidationError>
    where
        Self: Sized;
}

impl MessageContentFactory for MessageContent {
    fn text_with_validation(text: FormattedText) -> Result<Self, MessageValidationError> {
        let text_str = text.text();

        if text_str.is_empty() {
            return Err(MessageValidationError::InvalidContent(
                "Message text cannot be empty".to_string(),
            ));
        }

        if text_str.len() > Message::MAX_TEXT_LENGTH {
            return Err(MessageValidationError::InvalidContent(format!(
                "Message text too long: {} > {}",
                text_str.len(),
                Message::MAX_TEXT_LENGTH
            )));
        }

        Ok(Self::Text(Box::new(
            rustgram_message_content::MessageText::new(text),
        )))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_message_input_reply_to::MessageQuote;
    use rustgram_types::UserId;

    // ============================================================================
    // Constructor Tests (5 tests)
    // ============================================================================

    #[test]
    fn test_message_new() {
        let text = FormattedText::new("Hello, world!");
        let message = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        )
        .unwrap();

        assert_eq!(message.id, MessageId::from_server_id(1));
        // Note: message.is_valid() returns false because date is 0 (will be set by server)
        assert!(message.is_text());
    }

    #[test]
    fn test_message_new_with_date() {
        let text = FormattedText::new("Hello!");
        let message = Message::new_with_date(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            1234567890,
            text,
        )
        .unwrap();

        assert_eq!(message.date, 1234567890);
        assert!(message.is_valid());
    }

    #[test]
    fn test_message_new_invalid_date() {
        let text = FormattedText::new("Hello!");
        let result = Message::new_with_date(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            0,
            text,
        );

        assert!(matches!(
            result,
            Err(MessageValidationError::InvalidDate(0))
        ));
    }

    #[test]
    fn test_message_new_empty_text() {
        let text = FormattedText::new("");
        let result = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        );

        assert!(matches!(
            result,
            Err(MessageValidationError::InvalidContent(_))
        ));
    }

    #[test]
    fn test_message_new_too_long() {
        let long_text = "x".repeat(Message::MAX_TEXT_LENGTH + 1);
        let text = FormattedText::new(&long_text);
        let result = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        );

        assert!(matches!(
            result,
            Err(MessageValidationError::InvalidContent(_))
        ));
    }

    // ============================================================================
    // Validation Tests (5 tests)
    // ============================================================================

    #[test]
    fn test_message_is_valid() {
        let text = FormattedText::new("Valid message");
        let message = Message::new_with_date(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            1234567890,
            text,
        )
        .unwrap();

        assert!(message.is_valid());
    }

    #[test]
    fn test_message_invalid_date() {
        let text = FormattedText::new("Test");
        let mut message = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        )
        .unwrap();

        message.date = 0;
        assert!(!message.is_valid());
    }

    #[test]
    fn test_message_invalid_edit_date() {
        let text = FormattedText::new("Test");
        let mut message = Message::new_with_date(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            1234567800,
            text,
        )
        .unwrap();

        message.edit_date = Some(1234567700); // Before message date
        assert!(!message.is_valid());
    }

    #[test]
    fn test_message_valid_edit_date() {
        let text = FormattedText::new("Test");
        let mut message = Message::new_with_date(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            1234567800,
            text,
        )
        .unwrap();

        message.edit_date = Some(1234567900); // After message date
        assert!(message.is_valid());
    }

    // ============================================================================
    // Query Tests (5 tests)
    // ============================================================================

    #[test]
    fn test_is_edited() {
        let text = FormattedText::new("Test");
        let mut message = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        )
        .unwrap();

        assert!(!message.is_edited());

        message.edit_date = Some(1234567890);
        assert!(message.is_edited());
    }

    #[test]
    fn test_is_text() {
        let text = FormattedText::new("Text message");
        let message = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        )
        .unwrap();

        assert!(message.is_text());
    }

    #[test]
    fn test_has_reply() {
        let text = FormattedText::new("Reply");
        let mut message = Message::new(
            MessageId::from_server_id(2),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        )
        .unwrap();

        assert!(!message.has_reply());

        message.reply_to = Some(MessageInputReplyTo::message(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            MessageQuote::new(),
            0,
        ));
        assert!(message.has_reply());
    }

    #[test]
    fn test_content_type() {
        let text = FormattedText::new("Test");
        let message = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        )
        .unwrap();

        assert_eq!(message.content_type(), MessageContentType::Text);
    }

    #[test]
    fn test_is_channel_message() {
        let text = FormattedText::new("Channel post");
        let mut message = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        )
        .unwrap();

        assert!(!message.is_channel_message());

        message.views = Some(100);
        assert!(message.is_channel_message());
    }

    // ============================================================================
    // Serialization Tests (5 tests)
    // ============================================================================

    #[test]
    fn test_serialize_message() {
        let text = FormattedText::new("Hello!");
        let message = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        )
        .unwrap();

        let json = serde_json::to_string(&message).unwrap();
        assert!(json.contains("\"id\""));
        assert!(json.contains("\"dialog_id\""));
        assert!(json.contains("\"content\""));
    }

    #[test]
    fn test_deserialize_message() {
        let text = FormattedText::new("Hello!");
        let message = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        )
        .unwrap();

        let json = serde_json::to_string(&message).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.id, message.id);
        assert_eq!(parsed.dialog_id, message.dialog_id);
        assert_eq!(parsed.sender_id, message.sender_id);
    }

    #[test]
    fn test_round_trip() {
        let text = FormattedText::new("Hello, world!");
        let original = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        )
        .unwrap();

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Message = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_serialize_with_edit_date() {
        let text = FormattedText::new("Edited!");
        let mut message = Message::new_with_date(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            1234567800,
            text,
        )
        .unwrap();

        message.edit_date = Some(1234567900);

        let json = serde_json::to_string(&message).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.edit_date, Some(1234567900));
    }

    #[test]
    fn test_serialize_with_reply() {
        let text = FormattedText::new("Reply!");
        let mut message = Message::new(
            MessageId::from_server_id(2),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        )
        .unwrap();

        message.reply_to = Some(MessageInputReplyTo::message(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            MessageQuote::new(),
            0,
        ));

        let json = serde_json::to_string(&message).unwrap();
        let parsed: Message = serde_json::from_str(&json).unwrap();

        assert!(parsed.reply_to.is_some());
    }

    // ============================================================================
    // Display Tests (2 tests)
    // ============================================================================

    #[test]
    fn test_display() {
        let text = FormattedText::new("Test");
        let message = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        )
        .unwrap();

        let display = format!("{}", message);
        assert!(display.contains("Message"));
        assert!(display.contains("id="));
        assert!(display.contains("Text"));
    }

    #[test]
    fn test_error_display() {
        let error = MessageValidationError::InvalidDate(0);
        let display = format!("{}", error);
        assert!(display.contains("Invalid message date"));
    }

    // ============================================================================
    // Clone Tests (2 tests)
    // ============================================================================

    #[test]
    fn test_clone() {
        let text = FormattedText::new("Test");
        let message = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        )
        .unwrap();

        let cloned = message.clone();
        assert_eq!(message, cloned);
    }

    #[test]
    fn test_clone_independence() {
        let text = FormattedText::new("Test");
        let message = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text.clone(),
        )
        .unwrap();

        let mut cloned = message.clone();
        cloned.edit_date = Some(1234567890);

        assert!(cloned.is_edited());
        assert!(!message.is_edited());
    }

    // ============================================================================
    // Equality Tests (2 tests)
    // ============================================================================

    #[test]
    fn test_equality() {
        let text1 = FormattedText::new("Same");
        let text2 = FormattedText::new("Same");

        let message1 = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text1,
        )
        .unwrap();

        let message2 = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text2,
        )
        .unwrap();

        assert_eq!(message1, message2);
    }

    #[test]
    fn test_inequality() {
        let text1 = FormattedText::new("First");
        let text2 = FormattedText::new("Second");

        let message1 = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text1,
        )
        .unwrap();

        let message2 = Message::new(
            MessageId::from_server_id(2),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text2,
        )
        .unwrap();

        assert_ne!(message1, message2);
    }

    // ============================================================================
    // Edge Case Tests (4 tests)
    // ============================================================================

    #[test]
    fn test_max_length_text() {
        let max_text = "x".repeat(Message::MAX_TEXT_LENGTH);
        let text = FormattedText::new(&max_text);
        let message = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        );

        assert!(message.is_ok());
    }

    #[test]
    fn test_single_char_text() {
        let text = FormattedText::new("x");
        let message = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        );

        assert!(message.is_ok());
    }

    #[test]
    fn test_unicode_text() {
        let text = FormattedText::new("Hello 🌍🌎🌏");
        let message = Message::new(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            text,
        );

        assert!(message.is_ok());
    }

    #[test]
    fn test_set_edit_date_valid() {
        let text = FormattedText::new("Test");
        let mut message = Message::new_with_date(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            1234567800,
            text,
        )
        .unwrap();

        let result = message.set_edit_date(1234567900);
        assert!(result.is_ok());
        assert_eq!(message.edit_date, Some(1234567900));
    }

    #[test]
    fn test_set_edit_date_invalid() {
        let text = FormattedText::new("Test");
        let mut message = Message::new_with_date(
            MessageId::from_server_id(1),
            DialogId::from_user(UserId::new(123).unwrap()),
            DialogId::from_user(UserId::new(456).unwrap()),
            1234567800,
            text,
        )
        .unwrap();

        let result = message.set_edit_date(1234567700);
        assert!(matches!(
            result,
            Err(MessageValidationError::InvalidEditDate { .. })
        ));
    }
}
