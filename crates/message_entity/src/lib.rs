// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0;

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_types::UserId;
use serde::{Deserialize, Serialize};

/// Custom emoji identifier stub.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct CustomEmojiId(i64);

impl CustomEmojiId {
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    #[must_use]
    pub const fn get(&self) -> i64 {
        self.0
    }

    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.0 != 0
    }
}

/// Text entity type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum MessageEntityType {
    Mention = 0,
    Hashtag = 1,
    BotCommand = 2,
    Url = 3,
    EmailAddress = 4,
    Bold = 5,
    Italic = 6,
    Code = 7,
    Pre = 8,
    PreCode = 9,
    TextUrl = 10,
    MentionName = 11,
    Cashtag = 12,
    PhoneNumber = 13,
    Underline = 14,
    Strikethrough = 15,
    BlockQuote = 16,
    BankCardNumber = 17,
    MediaTimestamp = 18,
    Spoiler = 19,
    CustomEmoji = 20,
    ExpandableBlockQuote = 21,
}

/// Message entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    entity_type: MessageEntityType,
    offset: i32,
    length: i32,
    argument: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<UserId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_emoji_id: Option<CustomEmojiId>,
}

impl MessageEntity {
    #[must_use]
    pub fn new(entity_type: MessageEntityType, offset: i32, length: i32) -> Self {
        Self {
            entity_type,
            offset,
            length,
            argument: None,
            user_id: None,
            timestamp: None,
            custom_emoji_id: None,
        }
    }

    #[must_use]
    pub fn with_argument(
        entity_type: MessageEntityType,
        offset: i32,
        length: i32,
        argument: String,
    ) -> Self {
        Self {
            entity_type,
            offset,
            length,
            argument: Some(argument),
            user_id: None,
            timestamp: None,
            custom_emoji_id: None,
        }
    }

    #[must_use]
    pub fn mention_name(offset: i32, length: i32, user_id: UserId) -> Self {
        Self {
            entity_type: MessageEntityType::MentionName,
            offset,
            length,
            argument: None,
            user_id: Some(user_id),
            timestamp: None,
            custom_emoji_id: None,
        }
    }

    #[must_use]
    pub fn with_media_timestamp(offset: i32, length: i32, ts: i32) -> Self {
        Self {
            entity_type: MessageEntityType::MediaTimestamp,
            offset,
            length,
            argument: None,
            user_id: None,
            timestamp: Some(ts),
            custom_emoji_id: None,
        }
    }

    #[must_use]
    pub fn custom_emoji(offset: i32, length: i32, emoji_id: CustomEmojiId) -> Self {
        Self {
            entity_type: MessageEntityType::CustomEmoji,
            offset,
            length,
            argument: None,
            user_id: None,
            timestamp: None,
            custom_emoji_id: Some(emoji_id),
        }
    }

    #[must_use]
    pub const fn entity_type(&self) -> MessageEntityType {
        self.entity_type
    }

    #[must_use]
    pub const fn offset(&self) -> i32 {
        self.offset
    }

    #[must_use]
    pub const fn length(&self) -> i32 {
        self.length
    }

    #[must_use]
    pub fn argument(&self) -> Option<&str> {
        self.argument.as_deref()
    }

    #[must_use]
    pub const fn user_id(&self) -> Option<UserId> {
        match self.user_id {
            Some(id) if id.get() > 0 => Some(id),
            _ => None,
        }
    }

    #[must_use]
    pub const fn get_media_timestamp(&self) -> Option<i32> {
        self.timestamp
    }

    #[must_use]
    pub const fn custom_emoji_id(&self) -> Option<CustomEmojiId> {
        match self.custom_emoji_id {
            Some(id) if id.is_valid() => Some(id),
            _ => None,
        }
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.offset >= 0 && self.length > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entity = MessageEntity::new(MessageEntityType::Bold, 0, 5);
        assert_eq!(entity.entity_type(), MessageEntityType::Bold);
        assert!(entity.is_valid());
    }
}
