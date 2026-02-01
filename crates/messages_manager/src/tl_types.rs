// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! TL (Type Language) schema types for messaging operations.
//!
//! This module defines the TL request/response types used for MTProto
//! communication with Telegram servers for message operations.
//!
//! # TDLib Alignment
//!
//! Based on TDLib's TL schema from `td/telegram/mtproto/scheme.tl`.
//! These types correspond to the `messages.sendMessage` and related
//! RPC methods in the MTProto protocol.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use rustgram_types::DialogId;

/// TL constructor numbers for message types.
pub mod constructors {
    /// messages.sendMessage
    pub const MESSAGES_SEND_MESSAGE: i32 = -1872427999; // 0x90650421 as i32

    /// messages.sendMedia
    pub const MESSAGES_SEND_MEDIA: i32 = 1116092968; // 0x50ddb2b8 as i32

    /// UpdateNewMessage
    pub const UPDATE_NEW_MESSAGE: i32 = 1273992; // 0x001373c8 as i32

    /// UpdateShortMessage
    pub const UPDATE_SHORT_MESSAGE: i32 = 2415915; // 0x176068b as i32

    /// UpdateShortChatMessage
    pub const UPDATE_SHORT_CHAT_MESSAGE: i32 = 57423706; // 0x222591b5 as i32

    /// UpdateShortSentMessage
    pub const UPDATE_SHORT_SENT_MESSAGE: i32 = 298377884; // 0x11f1331c as i32

    /// Updates
    pub const UPDATES: i32 = 481402225; // 0x1ca93831 as i32

    /// UpdatesTooLong
    pub const UPDATES_TOO_LONG: i32 = -484987010; // 0xe317af7e as i32
}

// ============================================================================
// Request Types
// ============================================================================

/// Request for sending a text message.
///
/// Corresponds to `messages.sendMessage` in MTProto.
/// Based on TDLib's SendMessageRequest from `td/telegram/tl/tl_message.hpp`.
///
/// # Fields
///
/// * `peer` - Target dialog (user, chat, or channel)
/// * `message` - Message text
/// * `random_id` - Unique request identifier for deduplication
/// * `reply_to_msg_id` - Optional message ID being replied to
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SendMessageRequest {
    /// Request flags (bitmask for optional fields)
    pub flags: i32,

    /// Target dialog (InputPeer)
    pub peer: InputPeer,

    /// Message text (or caption for media)
    pub message: String,

    /// Random ID for deduplication (must be unique per message)
    pub random_id: i64,

    /// Optional reply-to message ID (present if flags & 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_msg_id: Option<i32>,

    /// Optional entities for formatted text (present if flags & 4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    /// Optional: send as silent message (no notification)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,

    /// Optional: background message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,

    /// Optional: clear draft
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_draft: Option<bool>,

    /// Optional: schedule date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_date: Option<i32>,

    /// Optional: send as (different peer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_as: Option<InputPeer>,

    /// Optional: quick reply shortcut ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_reply_shortcut_id: Option<i32>,

    /// Optional: effect ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<i64>,
}

impl SendMessageRequest {
    /// Creates a new send message request.
    ///
    /// # Arguments
    ///
    /// * `peer` - Target dialog
    /// * `message` - Message text
    /// * `random_id` - Unique random ID
    pub fn new(peer: InputPeer, message: String, random_id: i64) -> Self {
        Self {
            flags: 0,
            peer,
            message,
            random_id,
            reply_to_msg_id: None,
            entities: None,
            silent: None,
            background: None,
            clear_draft: None,
            schedule_date: None,
            send_as: None,
            quick_reply_shortcut_id: None,
            effect: None,
        }
    }

    /// Sets the reply-to message ID.
    #[must_use]
    pub fn with_reply_to(mut self, reply_to_msg_id: i32) -> Self {
        self.reply_to_msg_id = Some(reply_to_msg_id);
        self.flags |= 0x01;
        self
    }

    /// Sets message entities for formatted text.
    #[must_use]
    pub fn with_entities(mut self, entities: Vec<MessageEntity>) -> Self {
        self.entities = Some(entities);
        self.flags |= 0x04;
        self
    }

    /// Sets the message to be sent silently (no notification).
    #[must_use]
    pub fn silent(mut self) -> Self {
        self.silent = Some(true);
        self.flags |= 0x10;
        self
    }

    /// Returns the TL constructor number.
    pub const fn tl_constructor(&self) -> i32 {
        constructors::MESSAGES_SEND_MESSAGE
    }

    /// Serializes this request to bytes for MTProto transport.
    pub fn serialize(&self) -> Result<Bytes, TlSerializationError> {
        bincode::serialize(self)
            .map(Bytes::from)
            .map_err(|e| TlSerializationError::SerializationError(e.to_string()))
    }

    /// Deserializes a request from bytes.
    pub fn deserialize(bytes: &[u8]) -> Result<Self, TlSerializationError> {
        bincode::deserialize(bytes)
            .map_err(|e| TlSerializationError::DeserializationError(e.to_string()))
    }
}

// ============================================================================
// Response Types
// ============================================================================

/// Response from sendMessage request.
///
/// Contains the assigned message ID and server metadata.
/// Based on TDLib's SendMessageResult from `td/telegram/messages_manager.cpp`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SendMessageResult {
    /// Assigned message ID
    pub message_id: i32,

    /// Server timestamp when message was sent
    pub date: i32,

    /// Permanent timestamp (for update ordering)
    pub pts: i32,

    /// PTS change count
    pub pts_count: i32,

    /// Optional: ID of the container message (if sent in a container)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<i32>,
}

impl SendMessageResult {
    /// Creates a new send message result.
    pub fn new(message_id: i32, date: i32, pts: i32, pts_count: i32) -> Self {
        Self {
            message_id,
            date,
            pts,
            pts_count,
            container_id: None,
        }
    }
}

/// Updates wrapper response.
///
/// Telegram wraps most responses in an Updates object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Updates {
    /// List of updates
    pub updates: Vec<Update>,

    /// List of users mentioned in updates
    pub users: Vec<User>,

    /// List of chats mentioned in updates
    pub chats: Vec<Chat>,

    /// Server date
    pub date: i32,

    /// Sequence number for ordering
    pub seq: i32,
}

impl Updates {
    /// Creates a new Updates response.
    pub fn new(updates: Vec<Update>, date: i32, seq: i32) -> Self {
        Self {
            updates,
            users: Vec::new(),
            chats: Vec::new(),
            date,
            seq,
        }
    }

    /// Extracts new messages from the updates.
    pub fn new_messages(&self) -> Vec<&UpdateNewMessage> {
        self.updates
            .iter()
            .filter_map(|u| match u {
                Update::NewMessage(m) => Some(m),
                _ => None,
            })
            .collect()
    }
}

// ============================================================================
// Update Types
// ============================================================================

/// Telegram update.
///
/// Represents a server push notification of new events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Update {
    /// New message received
    #[serde(rename = "new_message")]
    NewMessage(UpdateNewMessage),

    /// Short message (optimized for text-only)
    #[serde(rename = "short_message")]
    ShortMessage(UpdateShortMessage),

    /// Short chat message
    #[serde(rename = "short_chat_message")]
    ShortChatMessage(UpdateShortChatMessage),

    /// Message was edited
    #[serde(rename = "edit_message")]
    EditMessage(UpdateEditMessage),

    /// Messages were deleted
    #[serde(rename = "delete_messages")]
    DeleteMessages(UpdateDeleteMessages),

    /// Read history update
    #[serde(rename = "read_history")]
    ReadHistory(UpdateReadHistory),

    /// Other update (unhandled)
    #[serde(rename = "other")]
    Other(OtherUpdate),
}

/// New message update.
///
/// Indicates a new message was received in a dialog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateNewMessage {
    /// The message data
    pub message: MessageData,

    /// Permanent timestamp for ordering
    pub pts: i32,

    /// PTS change count
    pub pts_count: i32,
}

impl UpdateNewMessage {
    /// Creates a new update for a message.
    pub fn new(message: MessageData, pts: i32, pts_count: i32) -> Self {
        Self {
            message,
            pts,
            pts_count,
        }
    }

    /// Returns the message ID.
    pub fn message_id(&self) -> i32 {
        self.message.id
    }

    /// Returns the dialog ID.
    pub fn dialog_id(&self) -> i64 {
        self.message.dialog_id
    }
}

/// Short message update (optimized).
///
/// Used for text-only messages to reduce bandwidth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateShortMessage {
    /// Message flags
    pub flags: i32,

    /// Message ID
    pub id: i32,

    /// User ID who sent the message
    pub user_id: i64,

    /// Message text
    pub message: String,

    /// Permanent timestamp
    pub pts: i32,

    /// PTS change count
    pub pts_count: i32,

    /// Server date
    pub date: i32,

    /// Optional: message entities (if flags & 8)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    /// Optional: forward info (if flags & 4)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fwd_from: Option<MessageFwdHeader>,

    /// Optional: reply-to info (if flags & 16)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<MessageReplyHeader>,
}

/// Short chat message update.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateShortChatMessage {
    /// Message flags
    pub flags: i32,

    /// Message ID
    pub id: i32,

    /// Chat ID
    pub chat_id: i64,

    /// Sender user ID
    pub from_id: i64,

    /// Message text
    pub message: String,

    /// Permanent timestamp
    pub pts: i32,

    /// PTS change count
    pub pts_count: i32,

    /// Server date
    pub date: i32,

    /// Optional: message entities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
}

/// Message edit update.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateEditMessage {
    /// The edited message
    pub message: MessageData,

    /// Permanent timestamp
    pub pts: i32,

    /// PTS change count
    pub pts_count: i32,
}

/// Delete messages update.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateDeleteMessages {
    /// Message flags
    pub flags: i32,

    /// List of deleted message IDs
    pub messages: Vec<i32>,

    /// Permanent timestamp
    pub pts: i32,

    /// PTS change count
    pub pts_count: i32,
}

/// Read history update.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateReadHistory {
    /// Peer whose history was read
    pub peer: Peer,

    /// Maximum read message ID
    pub max_id: i32,

    /// Permanent timestamp
    pub pts: i32,

    /// PTS change count
    pub pts_count: i32,
}

/// Placeholder for unhandled update types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OtherUpdate {
    /// Update type identifier
    pub update_type: String,

    /// Raw data
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

// ============================================================================
// Message Data Types
// ============================================================================

/// Message data from server.
///
/// Simplified version of TDLib's message object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageData {
    /// Message ID
    pub id: i32,

    /// Dialog ID (peer ID)
    pub dialog_id: i64,

    /// Sender user ID
    pub sender_id: i64,

    /// Server date
    pub date: i32,

    /// Message text
    pub message: String,

    /// Optional: edit date (if edited)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i32>,

    /// Optional: entities for formatted text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    /// Optional: forward info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fwd_from: Option<MessageFwdHeader>,

    /// Optional: reply-to info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<MessageReplyHeader>,

    /// Message flags
    pub flags: i32,
}

impl MessageData {
    /// Creates a new message data.
    pub fn new(
        id: i32,
        dialog_id: i64,
        sender_id: i64,
        date: i32,
        message: String,
    ) -> Self {
        Self {
            id,
            dialog_id,
            sender_id,
            date,
            message,
            edit_date: None,
            entities: None,
            fwd_from: None,
            reply_to: None,
            flags: 0,
        }
    }

    /// Returns `true` if this is an outgoing message.
    pub fn is_outgoing(&self) -> bool {
        self.flags & 0x02 != 0
    }

    /// Returns `true` if this message was edited.
    pub fn is_edited(&self) -> bool {
        self.edit_date.is_some()
    }

    /// Returns `true` if this is a forwarded message.
    pub fn is_forwarded(&self) -> bool {
        self.fwd_from.is_some()
    }

    /// Returns `true` if this is a reply to another message.
    pub fn is_reply(&self) -> bool {
        self.reply_to.is_some()
    }
}

// ============================================================================
// Input Types (for requests)
// ============================================================================

/// Input peer (target dialog for messages).
///
/// Represents a dialog in a format suitable for sending requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InputPeer {
    /// Empty peer (invalid)
    #[serde(rename = "empty")]
    Empty,

    /// Self user
    #[serde(rename = "self")]
    SelfUser,

    /// User
    #[serde(rename = "user")]
    User {
        /// User ID
        user_id: i64,
    },

    /// Chat
    #[serde(rename = "chat")]
    Chat {
        /// Chat ID
        chat_id: i64,
    },

    /// Channel
    #[serde(rename = "channel")]
    Channel {
        /// Channel ID
        channel_id: i64,
        /// Access hash for authentication
        access_hash: i64,
    },

    /// User from (for forwarded messages)
    #[serde(rename = "user_from")]
    UserFrom {
        /// User ID
        user_id: i64,
        /// Access hash for authentication
        access_hash: i64,
    },

    /// Channel from (for forwarded messages)
    #[serde(rename = "channel_from")]
    ChannelFrom {
        /// Channel ID
        channel_id: i64,
        /// Access hash for authentication
        access_hash: i64,
    },
}

impl InputPeer {
    /// Creates an InputPeer from a DialogId.
    ///
    /// # Note
    ///
    /// This is a simplified version. In production, access_hash must be
    /// provided for channels and some users.
    pub fn from_dialog_id(dialog_id: DialogId) -> Result<Self, InputPeerError> {
        match dialog_id {
            DialogId::User(user_id) => Ok(Self::User { user_id: user_id.get() }),
            DialogId::Chat(chat_id) => Ok(Self::Chat { chat_id: chat_id.get() }),
            DialogId::Channel(channel_id) => {
                // In production, access_hash is required
                Ok(Self::Channel {
                    channel_id: channel_id.get(),
                    access_hash: 0, // TODO: Get actual access_hash
                })
            }
            DialogId::SecretChat(secret_chat_id) => {
                // Secret chats require access_hash
                Ok(Self::ChannelFrom {
                    channel_id: secret_chat_id.get() as i64,
                    access_hash: 0, // TODO: Get actual access_hash
                })
            }
        }
    }

    /// Returns the numeric peer ID.
    pub fn peer_id(&self) -> i64 {
        match self {
            Self::Empty => 0,
            Self::SelfUser => 0, // TODO: Get actual self user ID
            Self::User { user_id } => *user_id,
            Self::Chat { chat_id } => -chat_id,
            Self::Channel { channel_id, .. } => -channel_id,
            Self::UserFrom { user_id, .. } => *user_id,
            Self::ChannelFrom { channel_id, .. } => -channel_id,
        }
    }
}

/// Peer (response type).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Peer {
    /// User
    #[serde(rename = "user")]
    User {
        /// User ID
        user_id: i64,
    },

    /// Chat
    #[serde(rename = "chat")]
    Chat {
        /// Chat ID
        chat_id: i64,
    },

    /// Channel
    #[serde(rename = "channel")]
    Channel {
        /// Channel ID
        channel_id: i64,
    },
}

/// User (minimal representation).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: i64,

    /// First name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// Last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    /// Username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Access hash (for API calls)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_hash: Option<i64>,
}

/// Chat (minimal representation).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chat {
    /// Chat ID
    pub id: i64,

    /// Chat title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Access hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_hash: Option<i64>,
}

// ============================================================================
// Helper Types
// ============================================================================

/// Message entity for formatted text.
///
/// Represents a single formatting entity (bold, italic, URL, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageEntity {
    /// Bold text
    #[serde(rename = "bold")]
    Bold {
        /// Offset in UTF-16 code units
        offset: i32,
        /// Length in UTF-16 code units
        length: i32,
    },

    /// Italic text
    #[serde(rename = "italic")]
    Italic {
        /// Offset in UTF-16 code units
        offset: i32,
        /// Length in UTF-16 code units
        length: i32,
    },

    /// Underline text
    #[serde(rename = "underline")]
    Underline {
        /// Offset in UTF-16 code units
        offset: i32,
        /// Length in UTF-16 code units
        length: i32,
    },

    /// Strikethrough text
    #[serde(rename = "strike")]
    Strike {
        /// Offset in UTF-16 code units
        offset: i32,
        /// Length in UTF-16 code units
        length: i32,
    },

    /// Code (monospace)
    #[serde(rename = "code")]
    Code {
        /// Offset in UTF-16 code units
        offset: i32,
        /// Length in UTF-16 code units
        length: i32,
    },

    /// Pre-formatted text block
    #[serde(rename = "pre")]
    Pre {
        /// Offset in UTF-16 code units
        offset: i32,
        /// Length in UTF-16 code units
        length: i32,
        /// Programming language
        language: Option<String>,
    },

    /// Text URL
    #[serde(rename = "text_url")]
    TextUrl {
        /// Offset in UTF-16 code units
        offset: i32,
        /// Length in UTF-16 code units
        length: i32,
        /// URL to open
        url: String,
    },

    /// Mention
    #[serde(rename = "mention")]
    Mention {
        /// Offset in UTF-16 code units
        offset: i32,
        /// Length in UTF-16 code units
        length: i32,
        /// User ID being mentioned
        user_id: i64,
    },

    /// Hashtag
    #[serde(rename = "hashtag")]
    Hashtag {
        /// Offset in UTF-16 code units
        offset: i32,
        /// Length in UTF-16 code units
        length: i32,
    },

    /// Cashtag
    #[serde(rename = "cashtag")]
    Cashtag {
        /// Offset in UTF-16 code units
        offset: i32,
        /// Length in UTF-16 code units
        length: i32,
    },

    /// Bot command
    #[serde(rename = "bot_command")]
    BotCommand {
        /// Offset in UTF-16 code units
        offset: i32,
        /// Length in UTF-16 code units
        length: i32,
        /// Bot command string
        command: String,
    },

    /// Custom emoji
    #[serde(rename = "custom_emoji")]
    CustomEmoji {
        /// Byte offset in the text
        offset: i32,
        /// Length of the emoji
        length: i32,
        /// Unique identifier of the custom emoji
        document_id: i64,
    },
}

/// Message forward header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageFwdHeader {
    /// Optional: from ID (who sent the original message)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<Peer>,

    /// Optional: from which channel/chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,

    /// Server date of original message
    pub date: i32,

    /// Optional: channel post ID (if forwarded from channel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<i32>,

    /// Optional: signature of the post author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_author: Option<String>,

    /// Optional: saved from peer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_from_peer: Option<Peer>,

    /// Optional: saved from msg ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_from_msg_id: Option<i32>,

    /// Optional: saved from ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_from_id: Option<i32>,

    /// Optional: saved from date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_from_date: Option<i32>,

    /// Optional: PSA type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psa_type: Option<String>,
}

/// Message reply header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageReplyHeader {
    /// Reply-to message ID
    pub reply_to_msg_id: i32,

    /// Optional: reply to peer ID (for replies in different chats)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_peer_id: Option<i64>,

    /// Optional: reply to top ID (for threaded replies)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_top_id: Option<i32>,

    /// Optional: forum topic (if reply is a forum topic)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic: Option<bool>,
}

// ============================================================================
// Error Types
// ============================================================================

/// TL serialization/deserialization error.
#[derive(Debug, Error)]
pub enum TlSerializationError {
    /// Error during serialization
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Error during deserialization
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// Invalid data format
    #[error("Invalid data format: {0}")]
    InvalidFormat(String),

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),
}

/// Input peer conversion error.
#[derive(Debug, Error)]
pub enum InputPeerError {
    /// Invalid dialog ID
    #[error("Invalid dialog ID: {0}")]
    InvalidDialogId(i64),

    /// Missing access hash
    #[error("Missing access hash for peer: {0}")]
    MissingAccessHash(i64),

    /// Unsupported peer type
    #[error("Unsupported peer type")]
    UnsupportedType,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::{ChatId, UserId};

    #[test]
    fn test_send_message_request_new() {
        let peer = InputPeer::User { user_id: 123456 };
        let request = SendMessageRequest::new(peer, "Hello, world!".to_string(), 789);

        assert_eq!(request.message, "Hello, world!");
        assert_eq!(request.random_id, 789);
        assert!(request.reply_to_msg_id.is_none());
        assert!(request.entities.is_none());
    }

    #[test]
    fn test_send_message_request_with_reply() {
        let peer = InputPeer::Chat { chat_id: 123456 };
        let request = SendMessageRequest::new(peer, "Reply!".to_string(), 1000)
            .with_reply_to(999);

        assert_eq!(request.reply_to_msg_id, Some(999));
        assert!(request.flags & 0x01 != 0);
    }

    #[test]
    fn test_send_message_request_silent() {
        let peer = InputPeer::Channel {
            channel_id: 123456,
            access_hash: 789,
        };
        let request = SendMessageRequest::new(peer, "Silent message".to_string(), 2000)
            .silent();

        assert_eq!(request.silent, Some(true));
        assert!(request.flags & 0x10 != 0);
    }

    #[test]
    fn test_input_peer_from_dialog_id() {
        let user_id = UserId::new(123456).unwrap();
        let user_dialog = DialogId::from(user_id);
        let peer = InputPeer::from_dialog_id(user_dialog).unwrap();
        assert_eq!(peer, InputPeer::User { user_id: 123456 });

        let chat_id = ChatId::new(123456).unwrap();
        let chat_dialog = DialogId::from(chat_id);
        let peer = InputPeer::from_dialog_id(chat_dialog).unwrap();
        assert_eq!(peer, InputPeer::Chat { chat_id: 123456 });
    }

    #[test]
    fn test_send_message_result_new() {
        let result = SendMessageResult::new(1, 1700000000, 100, 1);

        assert_eq!(result.message_id, 1);
        assert_eq!(result.date, 1700000000);
        assert_eq!(result.pts, 100);
        assert_eq!(result.pts_count, 1);
    }

    #[test]
    fn test_update_new_message() {
        let message = MessageData::new(1, 123456, 789, 1700000000, "Test".to_string());
        let update = UpdateNewMessage::new(message, 100, 1);

        assert_eq!(update.message_id(), 1);
        assert_eq!(update.dialog_id(), 123456);
        assert_eq!(update.pts, 100);
    }

    #[test]
    fn test_message_data_is_edited() {
        let mut msg = MessageData::new(1, 123456, 789, 1700000000, "Test".to_string());
        assert!(!msg.is_edited());

        msg.edit_date = Some(1700000100);
        assert!(msg.is_edited());
    }

    #[test]
    fn test_message_data_is_reply() {
        let mut msg = MessageData::new(1, 123456, 789, 1700000000, "Test".to_string());
        assert!(!msg.is_reply());

        msg.reply_to = Some(MessageReplyHeader {
            reply_to_msg_id: 10,
            reply_to_peer_id: None,
            reply_to_top_id: None,
            forum_topic: None,
        });
        assert!(msg.is_reply());
    }

    #[test]
    fn test_message_data_is_outgoing() {
        let mut msg = MessageData::new(1, 123456, 789, 1700000000, "Test".to_string());
        assert!(!msg.is_outgoing());

        msg.flags = 0x02;
        assert!(msg.is_outgoing());
    }

    #[test]
    fn test_updates_new_messages() {
        let msg1 = MessageData::new(1, 123456, 789, 1700000000, "Test 1".to_string());
        let msg2 = MessageData::new(2, 123456, 789, 1700000001, "Test 2".to_string());

        let update1 = Update::NewMessage(UpdateNewMessage::new(msg1, 100, 1));
        let update2 = Update::NewMessage(UpdateNewMessage::new(msg2, 101, 1));

        let updates = Updates::new(vec![update1, update2], 1700000001, 1);
        let new_msgs = updates.new_messages();

        assert_eq!(new_msgs.len(), 2);
        assert_eq!(new_msgs[0].message_id(), 1);
        assert_eq!(new_msgs[1].message_id(), 2);
    }
}
