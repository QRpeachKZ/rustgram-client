// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0

//! # Message Types for Telegram Client

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_message_content_type::MessageContentType;
use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;

mod error;
mod forward_info;
mod reply_header;

pub use error::{Error, Result};
pub use forward_info::MessageForwardInfo;
pub use reply_header::MessageReplyHeader;

/// Information about what message this message is replying to.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MessageInputReplyTo {
    message_id: MessageId,
    dialog_id: DialogId,
}

impl MessageInputReplyTo {
    /// Creates a new empty reply-to information.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates reply-to information for a specific message.
    pub fn message(message_id: MessageId, dialog_id: DialogId) -> Self {
        Self {
            message_id,
            dialog_id,
        }
    }

    /// Returns the message ID being replied to.
    pub const fn get_message_id(&self) -> MessageId {
        self.message_id
    }

    /// Returns the dialog ID containing the message being replied to.
    pub const fn get_dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns true if this is empty (not replying to anything).
    pub fn is_empty(&self) -> bool {
        !self.message_id.is_valid()
    }

    /// Returns true if this is valid (replying to a real message).
    pub fn is_valid(&self) -> bool {
        self.message_id.is_valid()
    }
}

impl fmt::Display for MessageInputReplyTo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.message_id.is_valid() {
            write!(f, "message {}", self.message_id.get())
        } else {
            write!(f, "empty")
        }
    }
}

/// Simplified message content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageContent {
    content_type: MessageContentType,
}

impl MessageContent {
    /// Creates new message content from a content type.
    pub const fn new(content_type: MessageContentType) -> Self {
        Self { content_type }
    }

    /// Returns the content type.
    pub const fn content_type(&self) -> MessageContentType {
        self.content_type
    }
}

impl Default for MessageContent {
    fn default() -> Self {
        Self {
            content_type: MessageContentType::Text,
        }
    }
}

impl fmt::Display for MessageContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "content")
    }
}

/// Options for sending a message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageSendOptions {
    disable_web_page_preview: bool,
    silent: bool,
    background: bool,
    schedule_date: Option<i32>,
}

impl MessageSendOptions {
    /// Creates default send options.
    pub const fn new() -> Self {
        Self {
            disable_web_page_preview: false,
            silent: false,
            background: false,
            schedule_date: None,
        }
    }

    /// Creates options with web page preview disabled.
    pub fn without_preview() -> Self {
        Self {
            disable_web_page_preview: true,
            ..Self::new()
        }
    }

    /// Creates options for sending silently (without notification).
    pub fn silent() -> Self {
        Self {
            silent: true,
            ..Self::new()
        }
    }

    /// Returns true if web page preview is disabled.
    pub const fn disable_web_page_preview(&self) -> bool {
        self.disable_web_page_preview
    }

    /// Returns true if message should be sent silently.
    pub const fn is_silent(&self) -> bool {
        self.silent
    }

    /// Returns true if message is being sent from background.
    pub const fn is_background(&self) -> bool {
        self.background
    }

    /// Returns the scheduled date, if any.
    pub const fn schedule_date(&self) -> Option<i32> {
        self.schedule_date
    }
}

impl Default for MessageSendOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// A message in a dialog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    id: MessageId,
    dialog_id: DialogId,
    sender_id: DialogId,
    date: i32,
    content_type: MessageContentType,
    content: MessageContent,
    reply_to: Option<MessageInputReplyTo>,
    forward_info: Option<MessageForwardInfo>,
    reply_header: Option<MessageReplyHeader>,
    edit_date: Option<i32>,
    views: Option<i32>,
}

impl Message {
    /// Creates a new message.
    pub fn new(
        id: MessageId,
        dialog_id: DialogId,
        sender_id: DialogId,
        date: i32,
        content_type: MessageContentType,
    ) -> Self {
        let content = MessageContent::new(content_type);
        Self {
            id,
            dialog_id,
            sender_id,
            date,
            content_type,
            content,
            reply_to: None,
            forward_info: None,
            reply_header: None,
            edit_date: None,
            views: None,
        }
    }

    /// Returns the message ID.
    pub const fn id(&self) -> MessageId {
        self.id
    }

    /// Returns the dialog ID containing this message.
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the sender's ID.
    pub const fn sender_id(&self) -> DialogId {
        self.sender_id
    }

    /// Returns the message date (Unix timestamp).
    pub const fn date(&self) -> i32 {
        self.date
    }

    /// Returns the content type.
    pub const fn content_type(&self) -> MessageContentType {
        self.content_type
    }

    /// Returns the message content.
    pub const fn content(&self) -> &MessageContent {
        &self.content
    }

    /// Returns reply information, if this is a reply.
    pub const fn reply_to(&self) -> Option<&MessageInputReplyTo> {
        self.reply_to.as_ref()
    }

    /// Returns forward information, if this is a forwarded message.
    pub const fn forward_info(&self) -> Option<&MessageForwardInfo> {
        self.forward_info.as_ref()
    }

    /// Returns reply header for thread replies, if any.
    pub const fn reply_header(&self) -> Option<&MessageReplyHeader> {
        self.reply_header.as_ref()
    }

    /// Returns the edit date, if this message was edited.
    pub const fn edit_date(&self) -> Option<i32> {
        self.edit_date
    }

    /// Returns the view count, if available.
    pub const fn views(&self) -> Option<i32> {
        self.views
    }

    /// Sets reply information.
    pub fn set_reply_to(&mut self, reply_to: MessageInputReplyTo) {
        self.reply_to = Some(reply_to);
    }

    /// Sets forward information.
    pub fn set_forward_info(&mut self, forward_info: MessageForwardInfo) {
        self.forward_info = Some(forward_info);
    }

    /// Sets reply header for thread replies.
    pub fn set_reply_header(&mut self, reply_header: MessageReplyHeader) {
        self.reply_header = Some(reply_header);
    }

    /// Sets the edit date.
    pub fn set_edit_date(&mut self, edit_date: i32) {
        self.edit_date = Some(edit_date);
    }

    /// Sets the view count.
    pub fn set_views(&mut self, views: i32) {
        self.views = Some(views);
    }

    /// Returns true if this message has valid ID and date.
    pub fn is_valid(&self) -> bool {
        self.id.is_valid() && self.date > 0
    }

    /// Returns true if this message has been edited.
    pub fn is_edited(&self) -> bool {
        self.edit_date.is_some()
    }

    /// Returns true if this message is a reply to another message.
    pub fn is_reply(&self) -> bool {
        self.reply_to.as_ref().map_or(false, |r| r.is_valid())
    }

    /// Returns true if this message was forwarded.
    pub fn is_forwarded(&self) -> bool {
        self.forward_info.is_some()
    }

    /// Returns true if this is an outgoing message (sender == self).
    pub fn is_outgoing(&self) -> bool {
        self.sender_id == self.dialog_id
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Message {} in {} at {}",
            self.id.get(),
            self.dialog_id,
            self.date
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    fn test_msg() -> Message {
        let uid = UserId::new(123).unwrap();
        let did = DialogId::from_user(uid);
        Message::new(
            MessageId::from_server_id(1),
            did,
            did,
            1234567890,
            MessageContentType::Text,
        )
    }

    #[test]
    fn test_new() {
        let m = test_msg();
        assert!(m.is_valid());
        assert_eq!(m.date(), 1234567890);
    }

    #[test]
    fn test_edit() {
        let mut m = test_msg();
        assert!(!m.is_edited());
        m.set_edit_date(1234567900);
        assert!(m.is_edited());
    }

    #[test]
    fn test_reply() {
        let mut m = test_msg();
        assert!(!m.is_reply());
        let uid = UserId::new(123).unwrap();
        let did = DialogId::from_user(uid);
        m.set_reply_to(MessageInputReplyTo::message(
            MessageId::from_server_id(5),
            did,
        ));
        assert!(m.is_reply());
    }

    #[test]
    fn test_forwarded() {
        let mut m = test_msg();
        assert!(!m.is_forwarded());
        m.set_forward_info(MessageForwardInfo::new(MessageId::from_server_id(10)));
        assert!(m.is_forwarded());
    }

    #[test]
    fn test_views() {
        let mut m = test_msg();
        assert!(m.views().is_none());
        m.set_views(100);
        assert_eq!(m.views(), Some(100));
    }
}
