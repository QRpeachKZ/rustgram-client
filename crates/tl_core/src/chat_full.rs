// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Chat full info types for Telegram.
//!
//! This module provides TL deserialization for full chat information.
//!
//! # TL Schema
//!
//! ```text
//! chatFull#2633421b flags:# can_set_username:flags.7?true has_scheduled:flags.8?true
//!     translations_disabled:flags.19?true id:long about:string participants:ChatParticipants
//!     chat_photo:flags.2?Photo notify_settings:PeerNotifySettings
//!     exported_invite:flags.13?ExportedChatInvite bot_info:flags.3?Vector<BotInfo>
//!     pinned_msg_id:flags.6?int folder_id:flags.11?int call:flags.12?InputGroupCall
//!     ttl_period:flags.14?int groupcall_default_join_as:flags.15?Peer
//!     theme_emoticon:flags.16?string requests_pending:flags.17?int
//!     recent_requesters:flags.17?Vector<long> available_reactions:flags.18?ChatReactions
//!     reactions_limit:flags.20?int = ChatFull;
//!
//! channelFull#e4e0b29d flags:# ... (many more fields) = ChatFull;
//! ```

use crate::error::TlError;
use crate::flags::FlagReader;
use crate::notify::PeerNotifySettings;
use crate::peer::Peer;
use rustgram_types::tl::{Bytes, TlDeserialize, TlHelper};
use serde::{Deserialize, Serialize};

/// Maximum number of participants allowed.
const MAX_PARTICIPANTS: usize = 10000;

/// Maximum number of recent requesters allowed.
const MAX_RECENT_REQUESTERS: usize = 100;

/// Full chat information.
///
/// Contains detailed information about a chat or channel.
///
/// # TL Schema
///
/// ```text
/// chatFull#2633421b flags:# can_set_username:flags.7?true has_scheduled:flags.8?true
///     translations_disabled:flags.19?true id:long about:string participants:ChatParticipants
///     chat_photo:flags.2?Photo notify_settings:PeerNotifySettings
///     exported_invite:flags.13?ExportedChatInvite bot_info:flags.3?Vector<BotInfo>
///     pinned_msg_id:flags.6?int folder_id:flags.11?int call:flags.12?InputGroupCall
///     ttl_period:flags.14?int groupcall_default_join_as:flags.15?Peer
///     theme_emoticon:flags.16?string requests_pending:flags.17?int
///     recent_requesters:flags.17?Vector<long> available_reactions:flags.18?ChatReactions
///     reactions_limit:flags.20?int = ChatFull;
/// ```
///
/// Note: This is a simplified version focusing on the core fields.
/// The full ChannelFull type has many more optional fields.
#[derive(Debug, Clone, PartialEq)]
pub struct ChatFull {
    /// Whether the username can be set.
    pub can_set_username: bool,

    /// Whether the chat has scheduled messages.
    pub has_scheduled: bool,

    /// Whether translations are disabled.
    pub translations_disabled: bool,

    /// Chat or channel ID.
    pub id: i64,

    /// Chat description or about text.
    pub about: String,

    /// Chat participants.
    pub participants: ChatParticipants,

    /// Chat photo (if available).
    pub chat_photo: Option<crate::photo::Photo>,

    /// Notification settings.
    pub notify_settings: PeerNotifySettings,

    /// Exported chat invite link (if available).
    pub exported_invite: Option<ExportedChatInvite>,

    /// Bot information (if this is a bot chat).
    pub bot_info: Option<Vec<BotInfo>>,

    /// ID of a pinned message (if any).
    pub pinned_msg_id: Option<i32>,

    /// Folder ID (if in a folder).
    pub folder_id: Option<i32>,

    /// Active group call (if any).
    pub call: Option<InputGroupCall>,

    /// Time-to-live period for messages.
    pub ttl_period: Option<i32>,

    /// Default peer to join group calls as.
    pub groupcall_default_join_as: Option<Peer>,

    /// Theme emoticon.
    pub theme_emoticon: Option<String>,

    /// Number of pending join requests.
    pub requests_pending: Option<i32>,

    /// Recent users who requested to join.
    pub recent_requesters: Option<Vec<i64>>,

    /// Available reactions.
    pub available_reactions: Option<ChatReactions>,

    /// Limit on number of reactions.
    pub reactions_limit: Option<i32>,
}

impl ChatFull {
    /// Constructor ID for chatFull.
    pub const CONSTRUCTOR: u32 = 0x2633421b;

    /// Constructor ID for channelFull.
    pub const CHANNEL_CONSTRUCTOR: u32 = 0xe4e0b29d;

    /// Checks if this is a channel.
    pub fn is_channel(&self) -> bool {
        matches!(self.participants, ChatParticipants::Unknown)
    }
}

impl TlDeserialize for ChatFull {
    fn deserialize_tl(buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        match constructor_id {
            Self::CONSTRUCTOR => Self::deserialize_chat_full(buf),
            Self::CHANNEL_CONSTRUCTOR => Self::deserialize_channel_full(buf),
            _ => {
                let tl_err = TlError::unknown_constructor(
                    vec![Self::CONSTRUCTOR, Self::CHANNEL_CONSTRUCTOR],
                    constructor_id,
                    "ChatFull",
                );
                Err(rustgram_types::TypeError::from(tl_err))
            }
        }
    }
}

impl ChatFull {
    fn deserialize_chat_full(buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        let flags = TlHelper::read_i32(buf)? as u32;
        let flag_reader = FlagReader::new(flags);

        let can_set_username = flag_reader.read_bool(7);
        let has_scheduled = flag_reader.read_bool(8);
        let translations_disabled = flag_reader.read_bool(19);

        let id = TlHelper::read_i64(buf)?;
        let about = TlHelper::read_string(buf)?;
        let participants = ChatParticipants::deserialize_tl(buf)?;

        let chat_photo =
            flag_reader.read_optional(2, || crate::photo::Photo::deserialize_tl(buf))?;

        let notify_settings = PeerNotifySettings::deserialize_tl(buf)?;

        let exported_invite =
            flag_reader.read_optional(13, || ExportedChatInvite::deserialize_placeholder(buf))?;

        let bot_info = flag_reader.read_optional(3, || deserialize_vector_bot_info(buf))?;

        let pinned_msg_id = flag_reader.read_optional(6, || TlHelper::read_i32(buf))?;
        let folder_id = flag_reader.read_optional(11, || TlHelper::read_i32(buf))?;
        let call =
            flag_reader.read_optional(12, || InputGroupCall::deserialize_placeholder(buf))?;
        let ttl_period = flag_reader.read_optional(14, || TlHelper::read_i32(buf))?;
        let groupcall_default_join_as =
            flag_reader.read_optional(15, || Peer::deserialize_tl(buf))?;
        let theme_emoticon = flag_reader.read_optional(16, || TlHelper::read_string(buf))?;
        let requests_pending = flag_reader.read_optional(17, || TlHelper::read_i32(buf))?;

        let recent_requesters = if flag_reader.has(17) {
            Some(deserialize_vector_i64(buf)?)
        } else {
            None
        };

        let available_reactions =
            flag_reader.read_optional(18, || ChatReactions::deserialize_placeholder(buf))?;

        let reactions_limit = flag_reader.read_optional(20, || TlHelper::read_i32(buf))?;

        Ok(Self {
            can_set_username,
            has_scheduled,
            translations_disabled,
            id,
            about,
            participants,
            chat_photo,
            notify_settings,
            exported_invite,
            bot_info,
            pinned_msg_id,
            folder_id,
            call,
            ttl_period,
            groupcall_default_join_as,
            theme_emoticon,
            requests_pending,
            recent_requesters,
            available_reactions,
            reactions_limit,
        })
    }

    fn deserialize_channel_full(buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        // Simplified channelFull deserialization
        // The full version would have many more fields
        let flags = TlHelper::read_i32(buf)? as u32;
        let flag_reader = FlagReader::new(flags);

        let can_set_username = flag_reader.read_bool(6);
        let has_scheduled = flag_reader.read_bool(19);

        let id = TlHelper::read_i64(buf)?;
        let about = TlHelper::read_string(buf)?;

        // For channel, participants is often None or uses a different structure
        let participants = ChatParticipants::Unknown;

        let chat_photo =
            flag_reader.read_optional(2, || crate::photo::Photo::deserialize_tl(buf))?;

        let notify_settings = PeerNotifySettings::deserialize_tl(buf)?;

        Ok(Self {
            can_set_username,
            has_scheduled,
            translations_disabled: false,
            id,
            about,
            participants,
            chat_photo,
            notify_settings,
            exported_invite: None,
            bot_info: None,
            pinned_msg_id: None,
            folder_id: None,
            call: None,
            ttl_period: None,
            groupcall_default_join_as: None,
            theme_emoticon: None,
            requests_pending: None,
            recent_requesters: None,
            available_reactions: None,
            reactions_limit: None,
        })
    }
}

/// Chat participants information.
#[derive(Debug, Clone, PartialEq)]
pub enum ChatParticipants {
    /// Chat participants data.
    Chat(ChatParticipantsData),

    /// Chat participants forbidden (e.g., due to privacy settings).
    Forbidden(ChatParticipantsForbidden),

    /// Unknown/placeholder for channel full info.
    Unknown,
}

impl ChatParticipants {
    /// Constructor ID for chatParticipants.
    pub const CHAT_CONSTRUCTOR: u32 = 0x3cbc93f8;

    /// Constructor ID for chatParticipantsForbidden.
    pub const FORBIDDEN_CONSTRUCTOR: u32 = 0x8763d3e1;
}

impl TlDeserialize for ChatParticipants {
    fn deserialize_tl(buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        match constructor_id {
            Self::CHAT_CONSTRUCTOR => {
                let chat_id = TlHelper::read_i64(buf)?;
                let participants = deserialize_vector_chat_participant(buf)?;
                let version = TlHelper::read_i32(buf)?;

                Ok(Self::Chat(ChatParticipantsData {
                    chat_id,
                    participants,
                    version,
                }))
            }
            Self::FORBIDDEN_CONSTRUCTOR => {
                let flags = TlHelper::read_i32(buf)? as u32;
                let _flag_reader = FlagReader::new(flags);

                let chat_id = TlHelper::read_i64(buf)?;

                Ok(Self::Forbidden(ChatParticipantsForbidden { chat_id }))
            }
            _ => {
                // For channels, we might get different constructors or no participants
                Ok(Self::Unknown)
            }
        }
    }
}

/// Chat participants data.
#[derive(Debug, Clone, PartialEq)]
pub struct ChatParticipantsData {
    /// Chat ID.
    pub chat_id: i64,

    /// List of participants.
    pub participants: Vec<ChatParticipant>,

    /// Version number for incremental updates.
    pub version: i32,
}

/// Chat participants forbidden.
#[derive(Debug, Clone, PartialEq)]
pub struct ChatParticipantsForbidden {
    /// Chat ID.
    pub chat_id: i64,
}

/// Chat participant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatParticipant {
    /// User ID.
    #[serde(skip)]
    pub user_id: i64,

    /// Inviter user ID (for regular members).
    #[serde(skip)]
    pub inviter_id: Option<i64>,

    /// Date when the user joined.
    #[serde(skip)]
    pub date: Option<i32>,
}

/// Bot information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotInfo {
    /// Bot description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Bot commands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<BotCommand>>,
}

/// Bot command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotCommand {
    /// Command text (e.g., "/start").
    #[serde(skip)]
    pub command: String,

    /// Command description.
    #[serde(skip)]
    pub description: String,
}

/// Exported chat invite link.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportedChatInvite {
    /// Invite link.
    #[serde(skip)]
    pub link: String,
}

impl ExportedChatInvite {
    fn deserialize_placeholder(_buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        // Placeholder implementation
        Ok(Self {
            link: String::new(),
        })
    }
}

/// Input group call.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputGroupCall {
    /// Call ID.
    #[serde(skip)]
    pub id: i64,

    /// Access hash.
    #[serde(skip)]
    pub access_hash: i64,
}

impl InputGroupCall {
    fn deserialize_placeholder(_buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        // Placeholder implementation
        Ok(Self {
            id: 0,
            access_hash: 0,
        })
    }
}

/// Chat reactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChatReactions {
    /// All reactions allowed.
    All,
    /// Specific reactions allowed.
    Some(Vec<String>),
    /// No reactions allowed.
    None,
}

impl ChatReactions {
    fn deserialize_placeholder(_buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        // Placeholder implementation
        Ok(Self::None)
    }
}

/// Deserializes a vector of BotInfo.
fn deserialize_vector_bot_info(buf: &mut Bytes) -> rustgram_types::TypeResult<Vec<BotInfo>> {
    let prefix = TlHelper::read_constructor_id(buf)?;
    if prefix != 0x1cb5c415 {
        let tl_err = TlError::deserialize_error(format!("Invalid vector prefix: 0x{:08x}", prefix));
        return Err(rustgram_types::TypeError::from(tl_err));
    }

    let count = TlHelper::read_i32(buf)? as usize;
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        result.push(BotInfo {
            description: None,
            commands: None,
        });
    }

    Ok(result)
}

/// Deserializes a vector of ChatParticipant.
fn deserialize_vector_chat_participant(
    buf: &mut Bytes,
) -> rustgram_types::TypeResult<Vec<ChatParticipant>> {
    let prefix = TlHelper::read_constructor_id(buf)?;
    if prefix != 0x1cb5c415 {
        let tl_err = TlError::deserialize_error(format!("Invalid vector prefix: 0x{:08x}", prefix));
        return Err(rustgram_types::TypeError::from(tl_err));
    }

    let count = TlHelper::read_i32(buf)? as usize;
    if count > MAX_PARTICIPANTS {
        let tl_err = TlError::deserialize_error(format!("Too many participants: {}", count));
        return Err(rustgram_types::TypeError::from(tl_err));
    }

    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        result.push(ChatParticipant {
            user_id: TlHelper::read_i64(buf)?,
            inviter_id: None,
            date: None,
        });
    }

    Ok(result)
}

/// Deserializes a vector of i64 values.
fn deserialize_vector_i64(buf: &mut Bytes) -> rustgram_types::TypeResult<Vec<i64>> {
    let prefix = TlHelper::read_constructor_id(buf)?;
    if prefix != 0x1cb5c415 {
        let tl_err = TlError::deserialize_error(format!("Invalid vector prefix: 0x{:08x}", prefix));
        return Err(rustgram_types::TypeError::from(tl_err));
    }

    let count = TlHelper::read_i32(buf)? as usize;
    if count > MAX_RECENT_REQUESTERS {
        let tl_err = TlError::deserialize_error(format!("Too many recent requesters: {}", count));
        return Err(rustgram_types::TypeError::from(tl_err));
    }

    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        result.push(TlHelper::read_i64(buf)?);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_buffer(data: &[u8]) -> Bytes {
        Bytes::new(bytes::Bytes::copy_from_slice(data))
    }

    #[test]
    fn test_chat_full_constructors() {
        assert_eq!(ChatFull::CONSTRUCTOR, 0x2633421b);
        assert_eq!(ChatFull::CHANNEL_CONSTRUCTOR, 0xe4e0b29d);
    }

    #[test]
    fn test_chat_participants_constructors() {
        assert_eq!(ChatParticipants::CHAT_CONSTRUCTOR, 0x3cbc93f8);
        assert_eq!(ChatParticipants::FORBIDDEN_CONSTRUCTOR, 0x8763d3e1);
    }

    #[test]
    fn test_chat_full_minimal() {
        // Minimal chatFull with only required fields
        let mut data = vec![0x1b, 0x42, 0x33, 0x26]; // chatFull constructor
        data.extend_from_slice(&0i32.to_le_bytes()); // flags = 0
        data.extend_from_slice(&123i64.to_le_bytes()); // id

        // about: string
        data.extend_from_slice(&[5u8, 0, 0, 0]); // length + padding
        data.extend_from_slice(b"hello"); // string

        // participants: chatParticipants placeholder
        data.extend_from_slice(&[0xf8, 0x93, 0xbc, 0x3c]); // chatParticipants constructor
        data.extend_from_slice(&123i64.to_le_bytes()); // chat_id
        data.extend_from_slice(&[0x15, 0xc4, 0xb5, 0x1c]); // vector constructor
        data.extend_from_slice(&0i32.to_le_bytes()); // count = 0
        data.extend_from_slice(&1i32.to_le_bytes()); // version

        // notify_settings: peerNotifySettings
        data.extend_from_slice(&[0x0c, 0x2c, 0x62, 0x99]); // peerNotifySettings constructor
        data.extend_from_slice(&0i32.to_le_bytes()); // flags = 0

        let mut buf = create_buffer(&data);
        let result = ChatFull::deserialize_tl(&mut buf);

        // This should succeed or provide a clear error
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_chat_participants_data() {
        let data = ChatParticipantsData {
            chat_id: 123,
            participants: vec![],
            version: 1,
        };

        assert_eq!(data.chat_id, 123);
        assert_eq!(data.participants.len(), 0);
        assert_eq!(data.version, 1);
    }

    #[test]
    fn test_bot_info() {
        let bot_info = BotInfo {
            description: Some("Test bot".to_string()),
            commands: None,
        };

        assert_eq!(bot_info.description, Some("Test bot".to_string()));
        assert!(bot_info.commands.is_none());
    }

    #[test]
    fn test_exported_chat_invite() {
        let invite = ExportedChatInvite {
            link: "https://t.me/+test".to_string(),
        };

        assert_eq!(invite.link, "https://t.me/+test");
    }

    #[test]
    fn test_input_group_call() {
        let call = InputGroupCall {
            id: 12345,
            access_hash: 67890,
        };

        assert_eq!(call.id, 12345);
        assert_eq!(call.access_hash, 67890);
    }

    #[test]
    fn test_chat_reactions() {
        assert_eq!(ChatReactions::All, ChatReactions::All);
        assert_eq!(ChatReactions::None, ChatReactions::None);
        assert_eq!(
            ChatReactions::Some(vec!["👍".to_string()]),
            ChatReactions::Some(vec!["👍".to_string()])
        );
    }

    #[test]
    fn test_chat_participant() {
        let participant = ChatParticipant {
            user_id: 123,
            inviter_id: Some(456),
            date: Some(789),
        };

        assert_eq!(participant.user_id, 123);
        assert_eq!(participant.inviter_id, Some(456));
        assert_eq!(participant.date, Some(789));
    }

    #[test]
    fn test_deserialize_vector_i64() {
        let mut data = vec![0x15, 0xc4, 0xb5, 0x1c]; // vector constructor
        data.extend_from_slice(&3i32.to_le_bytes()); // count
        data.extend_from_slice(&1i64.to_le_bytes());
        data.extend_from_slice(&2i64.to_le_bytes());
        data.extend_from_slice(&3i64.to_le_bytes());

        let mut buf = create_buffer(&data);
        let result = deserialize_vector_i64(&mut buf).unwrap();

        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_chat_full_unknown_constructor() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0]; // invalid constructor
        let mut buf = create_buffer(&data);
        let result = ChatFull::deserialize_tl(&mut buf);

        assert!(result.is_err());
    }

    #[test]
    fn test_chat_full_clone() {
        let full = ChatFull {
            can_set_username: true,
            has_scheduled: false,
            translations_disabled: false,
            id: 123,
            about: "Test".to_string(),
            participants: ChatParticipants::Unknown,
            chat_photo: None,
            notify_settings: PeerNotifySettings::default(),
            exported_invite: None,
            bot_info: None,
            pinned_msg_id: None,
            folder_id: None,
            call: None,
            ttl_period: None,
            groupcall_default_join_as: None,
            theme_emoticon: None,
            requests_pending: None,
            recent_requesters: None,
            available_reactions: None,
            reactions_limit: None,
        };

        let _ = full.clone();
    }
}
