// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0;

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_types::{DialogId, MessageId};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct StoryId(i32);

impl StoryId {
    #[must_use]
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    #[must_use]
    pub const fn get(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct StoryFullId {
    dialog_id: DialogId,
    story_id: StoryId,
}

impl StoryFullId {
    #[must_use]
    pub const fn new(dialog_id: DialogId, story_id: StoryId) -> Self {
        Self {
            dialog_id,
            story_id,
        }
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.story_id.get() != 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageQuote {
    text: String,
}

impl MessageQuote {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageInputReplyTo {
    message_id: MessageId,
    dialog_id: DialogId,
    quote: MessageQuote,
    todo_item_id: i32,
    story_full_id: StoryFullId,
}

impl MessageInputReplyTo {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn message(
        message_id: MessageId,
        dialog_id: DialogId,
        quote: MessageQuote,
        todo_item_id: i32,
    ) -> Self {
        Self {
            message_id,
            dialog_id,
            quote,
            todo_item_id,
            story_full_id: StoryFullId::default(),
        }
    }

    #[must_use]
    pub fn story(story_full_id: StoryFullId) -> Self {
        Self {
            message_id: MessageId::default(),
            dialog_id: DialogId::default(),
            quote: MessageQuote::new(),
            todo_item_id: 0,
            story_full_id,
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.message_id.is_valid() && !self.story_full_id.is_valid()
    }

    #[must_use]
    pub const fn get_message_id(&self) -> MessageId {
        self.message_id
    }

    #[must_use]
    pub const fn get_dialog_id(&self) -> DialogId {
        self.dialog_id
    }
}

impl Default for MessageInputReplyTo {
    fn default() -> Self {
        Self {
            message_id: MessageId::default(),
            dialog_id: DialogId::default(),
            quote: MessageQuote::new(),
            todo_item_id: 0,
            story_full_id: StoryFullId::default(),
        }
    }
}

impl fmt::Display for MessageInputReplyTo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.story_full_id.is_valid() {
            write!(f, "story {}", self.story_full_id.story_id.get())
        } else if self.message_id.is_valid() {
            write!(f, "message {}", self.message_id.get())
        } else {
            write!(f, "empty")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    #[test]
    fn test_new() {
        let reply_to = MessageInputReplyTo::new();
        assert!(reply_to.is_empty());
    }

    #[test]
    fn test_message() {
        let dialog_id = DialogId::from_user(UserId::new(123).unwrap());
        let message_id = MessageId::from_server_id(456);
        let reply_to = MessageInputReplyTo::message(message_id, dialog_id, MessageQuote::new(), 0);
        assert!(!reply_to.is_empty());
    }
}
