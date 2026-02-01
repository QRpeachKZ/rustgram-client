// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Draft Message
//!
//! Draft message state for Telegram dialogs and topics.
//!
//! ## Overview
//!
//! This module provides types for representing draft messages in Telegram.
//! A draft message represents a message that is being composed but not yet sent.
//!
//! ## Types
//!
//! - [`DraftMessage`] - Complete draft message state
//! - [`DraftMessageContent`] - Content of a draft message
//!
//! ## Example
//!
//! ```rust
//! use rustgram_draft_message::DraftMessage;
//!
//! let draft = DraftMessage::new();
//! assert_eq!(draft.get_date(), 0);
//! ```

use rustgram_input_message_text::InputMessageText;
use rustgram_types::MessageId;
use serde::{Deserialize, Serialize};

/// Placeholder for message input reply-to information.
///
/// In TDLib, this contains information about what message this draft is replying to.
/// For now, we use a placeholder enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MessageInputReplyTo {
    /// No reply-to information
    #[default]
    None,
    /// Replying to a specific message
    Message(MessageId),
    /// Replying to a story (placeholder)
    Story(i32),
    /// Unknown reply-to type
    Unknown,
}

/// Content of a draft message.
///
/// Represents the actual content that will be sent when the draft is completed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DraftMessageContent {
    /// Text message
    Text(InputMessageText),
    /// Unknown content type
    Unknown,
}

impl Default for DraftMessageContent {
    fn default() -> Self {
        Self::Text(InputMessageText::new())
    }
}

/// Placeholder for message effect ID.
///
/// Represents visual effects applied to messages.
pub type MessageEffectId = i64;

/// Placeholder for suggested post information.
///
/// In TDLib, this contains information about suggested posts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuggestedPost {
    /// Placeholder data
    data: String,
}

impl SuggestedPost {
    /// Creates a new suggested post.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            data: String::new(),
        }
    }
}

impl Default for SuggestedPost {
    fn default() -> Self {
        Self::new()
    }
}

/// Draft message state for Telegram dialogs and topics.
///
/// Represents a message that is being composed but not yet sent.
/// Contains all the information needed to restore the draft state.
///
/// # Fields
///
/// - `date` - Unix timestamp when the draft was last modified
/// - `message_input_reply_to` - What message this draft is replying to
/// - `input_message_text` - The text content of the draft
/// - `local_content` - Optional local content
/// - `message_effect_id` - Optional effect ID to apply
/// - `suggested_post` - Optional suggested post information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftMessage {
    /// Unix timestamp when the draft was last modified
    date: i32,
    /// What message this draft is replying to
    message_input_reply_to: MessageInputReplyTo,
    /// The text content of the draft
    input_message_text: InputMessageText,
    /// Optional local content
    local_content: Option<Box<DraftMessageContent>>,
    /// Optional effect ID to apply when sending
    message_effect_id: Option<MessageEffectId>,
    /// Optional suggested post information
    suggested_post: Option<Box<SuggestedPost>>,
}

impl DraftMessage {
    /// Creates a new empty draft message.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_draft_message::DraftMessage;
    ///
    /// let draft = DraftMessage::new();
    /// assert_eq!(draft.get_date(), 0);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            date: 0,
            message_input_reply_to: MessageInputReplyTo::None,
            input_message_text: InputMessageText::new(),
            local_content: None,
            message_effect_id: None,
            suggested_post: None,
        }
    }

    /// Creates a draft message with the given date.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_draft_message::DraftMessage;
    ///
    /// let draft = DraftMessage::with_date(1234567890);
    /// assert_eq!(draft.get_date(), 1234567890);
    /// ```
    #[must_use]
    pub fn with_date(date: i32) -> Self {
        Self {
            date,
            message_input_reply_to: MessageInputReplyTo::None,
            input_message_text: InputMessageText::new(),
            local_content: None,
            message_effect_id: None,
            suggested_post: None,
        }
    }

    /// Returns the draft modification timestamp.
    #[must_use]
    pub const fn get_date(&self) -> i32 {
        self.date
    }

    /// Sets the draft modification timestamp.
    pub fn set_date(&mut self, date: i32) {
        self.date = date;
    }

    /// Returns whether this is a local draft.
    ///
    /// A draft is considered local if it has local content.
    #[must_use]
    pub fn is_local(&self) -> bool {
        self.local_content.is_some()
    }

    /// Returns whether the local content should be cleared based on content type.
    ///
    /// In TDLib, this checks if the local content should be cleared
    /// based on the message content type. For now, this is a placeholder.
    #[must_use]
    pub fn need_clear_local(&self, _content_type: &str) -> bool {
        false
    }

    /// Returns the reply-to information.
    #[must_use]
    pub const fn message_input_reply_to(&self) -> &MessageInputReplyTo {
        &self.message_input_reply_to
    }

    /// Sets the reply-to information.
    pub fn set_message_input_reply_to(&mut self, reply_to: MessageInputReplyTo) {
        self.message_input_reply_to = reply_to;
    }

    /// Returns the input message text.
    #[must_use]
    pub const fn input_message_text(&self) -> &InputMessageText {
        &self.input_message_text
    }

    /// Sets the input message text.
    pub fn set_input_message_text(&mut self, text: InputMessageText) {
        self.input_message_text = text;
    }

    /// Returns the local content if present.
    #[must_use]
    pub const fn local_content(&self) -> &Option<Box<DraftMessageContent>> {
        &self.local_content
    }

    /// Sets the local content.
    pub fn set_local_content(&mut self, content: Option<DraftMessageContent>) {
        self.local_content = content.map(Box::new);
    }

    /// Returns the message effect ID if present.
    #[must_use]
    pub const fn message_effect_id(&self) -> Option<MessageEffectId> {
        self.message_effect_id
    }

    /// Sets the message effect ID.
    pub fn set_message_effect_id(&mut self, effect_id: Option<MessageEffectId>) {
        self.message_effect_id = effect_id;
    }

    /// Returns the suggested post information if present.
    #[must_use]
    pub const fn suggested_post(&self) -> &Option<Box<SuggestedPost>> {
        &self.suggested_post
    }

    /// Sets the suggested post information.
    pub fn set_suggested_post(&mut self, post: Option<SuggestedPost>) {
        self.suggested_post = post.map(Box::new);
    }
}

impl Default for DraftMessage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_formatted_text::FormattedText;

    #[test]
    fn test_draft_message_new() {
        let draft = DraftMessage::new();
        assert_eq!(draft.get_date(), 0);
        assert!(!draft.is_local());
    }

    #[test]
    fn test_draft_message_with_date() {
        let draft = DraftMessage::with_date(1234567890);
        assert_eq!(draft.get_date(), 1234567890);
    }

    #[test]
    fn test_draft_message_default() {
        let draft = DraftMessage::default();
        assert_eq!(draft.get_date(), 0);
    }

    #[test]
    fn test_set_date() {
        let mut draft = DraftMessage::new();
        draft.set_date(999);
        assert_eq!(draft.get_date(), 999);
    }

    #[test]
    fn test_is_local() {
        let mut draft = DraftMessage::new();
        assert!(!draft.is_local());

        draft.set_local_content(Some(DraftMessageContent::Text(InputMessageText::new())));
        assert!(draft.is_local());
    }

    #[test]
    fn test_need_clear_local() {
        let draft = DraftMessage::new();
        assert!(!draft.need_clear_local("text"));
    }

    #[test]
    fn test_message_input_reply_to() {
        let mut draft = DraftMessage::new();
        assert!(matches!(
            draft.message_input_reply_to(),
            MessageInputReplyTo::None
        ));

        draft.set_message_input_reply_to(MessageInputReplyTo::Message(MessageId::from_server_id(
            123,
        )));
        assert!(matches!(
            draft.message_input_reply_to(),
            MessageInputReplyTo::Message(_)
        ));
    }

    #[test]
    fn test_input_message_text() {
        let mut draft = DraftMessage::new();
        let text = InputMessageText::with_text(FormattedText::new("Hello"));
        draft.set_input_message_text(text);
        assert_eq!(draft.input_message_text().text().text(), "Hello");
    }

    #[test]
    fn test_local_content() {
        let mut draft = DraftMessage::new();
        assert!(draft.local_content().is_none());

        let content = DraftMessageContent::Text(InputMessageText::new());
        draft.set_local_content(Some(content));
        assert!(draft.local_content().is_some());
    }

    #[test]
    fn test_message_effect_id() {
        let mut draft = DraftMessage::new();
        assert!(draft.message_effect_id().is_none());

        draft.set_message_effect_id(Some(12345));
        assert_eq!(draft.message_effect_id(), Some(12345));
    }

    #[test]
    fn test_suggested_post() {
        let mut draft = DraftMessage::new();
        assert!(draft.suggested_post().is_none());

        draft.set_suggested_post(Some(SuggestedPost::new()));
        assert!(draft.suggested_post().is_some());
    }

    #[test]
    fn test_equality() {
        let draft1 = DraftMessage::with_date(100);
        let draft2 = DraftMessage::with_date(100);
        assert_eq!(draft1, draft2);

        let draft3 = DraftMessage::with_date(200);
        assert_ne!(draft1, draft3);
    }

    #[test]
    fn test_clone() {
        let draft1 = DraftMessage::with_date(100);
        let draft2 = draft1.clone();
        assert_eq!(draft1, draft2);
    }

    #[test]
    fn test_serialize() {
        let draft = DraftMessage::with_date(12345);
        let serialized = bincode::serialize(&draft).unwrap();
        let deserialized: DraftMessage = bincode::deserialize(&serialized).unwrap();
        assert_eq!(draft, deserialized);
    }

    #[test]
    fn test_message_input_reply_to_default() {
        let reply_to = MessageInputReplyTo::default();
        assert!(matches!(reply_to, MessageInputReplyTo::None));
    }

    #[test]
    fn test_input_message_text_default() {
        let text = InputMessageText::default();
        assert!(text.text().text().is_empty());
    }
}
