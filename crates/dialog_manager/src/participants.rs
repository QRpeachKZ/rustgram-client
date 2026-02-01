//! Chat participant loading and management.
//!
//! This module provides functionality for loading participants from chats and channels
//! using Telegram's TL API methods:
//! - `messages.getFullChat` - Load full chat info with participants
//! - `channels.getFullChannel` - Load full channel info
//! - `channels.getParticipants` - Load channel participants with pagination
//!
//! # Examples
//!
//! ```no_run
//! # use rustgram_dialog_manager::participants::{ParticipantManager, GetParticipantsRequest};
//! # use rustgram_dialog_manager::NetworkClient;
//! # use rustgram_types::ChatId;
//! # use std::sync::Arc;
//! # use rustgram_net::NetQueryDispatcher;
//! #
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let dispatcher = Arc::new(NetQueryDispatcher::new());
//! let client = NetworkClient::new(dispatcher);
//! let manager = ParticipantManager::new();
//!
//! let chat_id = ChatId::new(123).unwrap();
//!
//! // Load participants from a chat
//! let request = GetParticipantsRequest::Chat(chat_id);
//! let participants = manager.get_participants(&client, request).await?;
//! # Ok(())
//! # }
//! ```

use bytes::BytesMut;
use rustgram_types::{
    AccessHash, ChannelId, ChatId, TlDeserialize, TlHelper, TlSerialize, UserId,
};

use crate::{DialogError, Result};

/// Default number of participants to fetch per page.
pub const DEFAULT_PARTICIPANTS_LIMIT: usize = 100;

/// Maximum number of participants to fetch per page.
pub const MAX_PARTICIPANTS_LIMIT: usize = 200;

/// TL constructor for `messages.getFullChat`.
/// Verified from telegram_api.tl
pub const MESSAGES_GET_FULL_CHAT: u32 = 0xaeb00b34;

/// TL constructor for `channels.getFullChannel`.
/// Verified from telegram_api.tl
pub const CHANNELS_GET_FULL_CHANNEL: u32 = 0x8736a09;

/// TL constructor for `channels.getParticipants`.
/// Verified from telegram_api.tl
pub const CHANNELS_GET_PARTICIPANTS: u32 = 0x77ced9d0;

/// Manager for loading chat and channel participants.
///
/// Provides methods to fetch participant lists from Telegram servers.
#[derive(Debug, Clone)]
pub struct ParticipantManager {
    /// Request timeout in seconds.
    #[allow(dead_code)]
    timeout_secs: u64,
}

impl Default for ParticipantManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ParticipantManager {
    /// Default request timeout (30 seconds).
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

    /// Creates a new participant manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            timeout_secs: Self::DEFAULT_TIMEOUT_SECS,
        }
    }

    /// Creates a new participant manager with custom timeout.
    #[must_use]
    pub const fn with_timeout(timeout_secs: u64) -> Self {
        Self { timeout_secs }
    }

    /// Gets participants from a chat or channel.
    ///
    /// # Arguments
    ///
    /// * `client` - Network client for making requests
    /// * `request` - Request specifying which participants to fetch
    ///
    /// # Returns
    ///
    /// List of participants
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Network request fails
    /// - Invalid access hash
    /// - Request times out
    pub async fn get_participants(
        &self,
        client: &crate::NetworkClient,
        request: GetParticipantsRequest,
    ) -> Result<Vec<Participant>> {
        match request {
            GetParticipantsRequest::Chat(chat_id) => {
                self.get_chat_participants(client, chat_id).await
            }
            GetParticipantsRequest::Channel {
                channel_id,
                access_hash,
                offset,
                limit,
            } => {
                self.get_channel_participants(client, channel_id, access_hash, offset, limit)
                    .await
            }
        }
    }

    /// Gets participants from a basic group chat.
    ///
    /// # Arguments
    ///
    /// * `client` - Network client for making requests
    /// * `chat_id` - Chat ID to fetch participants for
    ///
    /// # Returns
    ///
    /// List of chat participants
    async fn get_chat_participants(
        &self,
        client: &crate::NetworkClient,
        chat_id: ChatId,
    ) -> Result<Vec<Participant>> {
        let request = GetFullChatRequest { chat_id };

        let response: GetFullChatResponse = client
            .send_typed_query(&request, MESSAGES_GET_FULL_CHAT)
            .await
            .map_err(|e| DialogError::NetworkError(format!("Failed to get full chat: {}", e)))?;

        Ok(response.full_chat.participants)
    }

    /// Gets participants from a channel with pagination.
    ///
    /// # Arguments
    ///
    /// * `client` - Network client for making requests
    /// * `channel_id` - Channel ID to fetch participants for
    /// * `access_hash` - Access hash for the channel
    /// * `offset` - Offset for pagination
    /// * `limit` - Maximum number of participants to return
    ///
    /// # Returns
    ///
    /// List of channel participants
    async fn get_channel_participants(
        &self,
        client: &crate::NetworkClient,
        channel_id: ChannelId,
        access_hash: u64,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<Participant>> {
        // Validate limit
        let limit = limit.clamp(1, MAX_PARTICIPANTS_LIMIT);

        let input_channel = InputChannel {
            channel_id,
            access_hash: AccessHash::new(access_hash as i64),
        };

        let request = GetChannelParticipantsRequest {
            channel: input_channel,
            filter: ChannelParticipantsFilter::All,
            offset: offset as i32,
            limit: limit as i32,
        };

        let response: GetChannelParticipantsResponse = client
            .send_typed_query(&request, CHANNELS_GET_PARTICIPANTS)
            .await
            .map_err(|e| {
                DialogError::NetworkError(format!("Failed to get channel participants: {}", e))
            })?;

        Ok(response.participants)
    }

    /// Gets full chat information including all metadata.
    ///
    /// # Arguments
    ///
    /// * `client` - Network client for making requests
    /// * `chat_id` - Chat ID to fetch full info for
    ///
    /// # Returns
    ///
    /// Full chat information
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Network request fails
    /// - Chat not found
    pub async fn get_full_chat(
        &self,
        client: &crate::NetworkClient,
        chat_id: ChatId,
    ) -> Result<ChatFull> {
        let request = GetFullChatRequest { chat_id };

        let response: GetFullChatResponse = client
            .send_typed_query(&request, MESSAGES_GET_FULL_CHAT)
            .await
            .map_err(|e| DialogError::NetworkError(format!("Failed to get full chat: {}", e)))?;

        Ok(response.full_chat)
    }

    /// Gets full channel information including all metadata.
    ///
    /// # Arguments
    ///
    /// * `client` - Network client for making requests
    /// * `channel_id` - Channel ID to fetch full info for
    /// * `access_hash` - Access hash for the channel
    ///
    /// # Returns
    ///
    /// Full channel information
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Network request fails
    /// - Invalid access hash
    /// - Channel not found
    pub async fn get_full_channel(
        &self,
        client: &crate::NetworkClient,
        channel_id: ChannelId,
        access_hash: u64,
    ) -> Result<ChannelFull> {
        let input_channel = InputChannel {
            channel_id,
            access_hash: AccessHash::new(access_hash as i64),
        };

        let request = GetFullChannelRequest {
            channel: input_channel,
        };

        let response: GetFullChannelResponse = client
            .send_typed_query(&request, CHANNELS_GET_FULL_CHANNEL)
            .await
            .map_err(|e| DialogError::NetworkError(format!("Failed to get full channel: {}", e)))?;

        Ok(response.full_channel)
    }
}

/// Request to get participants from a chat or channel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetParticipantsRequest {
    /// Get participants from a basic group chat.
    Chat(ChatId),
    /// Get participants from a channel with pagination.
    Channel {
        /// Channel ID
        channel_id: ChannelId,
        /// Access hash for authentication
        access_hash: u64,
        /// Offset for pagination
        offset: usize,
        /// Maximum number to return
        limit: usize,
    },
}

impl GetParticipantsRequest {
    /// Creates a request for chat participants.
    #[must_use]
    pub const fn chat(chat_id: ChatId) -> Self {
        Self::Chat(chat_id)
    }

    /// Creates a request for channel participants.
    #[must_use]
    pub const fn channel(channel_id: ChannelId, access_hash: u64) -> Self {
        Self::Channel {
            channel_id,
            access_hash,
            offset: 0,
            limit: DEFAULT_PARTICIPANTS_LIMIT,
        }
    }

    /// Creates a request for channel participants with pagination.
    #[must_use]
    pub const fn channel_paginated(
        channel_id: ChannelId,
        access_hash: u64,
        offset: usize,
        limit: usize,
    ) -> Self {
        Self::Channel {
            channel_id,
            access_hash,
            offset,
            limit,
        }
    }
}

/// A chat or channel participant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Participant {
    /// User ID of the participant
    pub user_id: UserId,
    /// Participant ID in the chat/channel
    pub participant_id: i64,
    /// Date when participant joined
    pub joined_date: Option<i32>,
    /// Invitee information (if invited by someone)
    pub inviter_id: Option<UserId>,
    /// Whether this participant is the creator
    pub is_creator: bool,
    /// Whether this participant is an admin
    pub is_admin: bool,
    /// Admin rank/title (if admin)
    pub admin_rank: Option<String>,
    /// Whether this participant can be invited to chat
    pub can_be_invited: bool,
}

impl Participant {
    /// Creates a new participant.
    #[must_use]
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            participant_id: user_id.get(),
            joined_date: None,
            inviter_id: None,
            is_creator: false,
            is_admin: false,
            admin_rank: None,
            can_be_invited: true,
        }
    }

    /// Returns `true` if this participant is a regular member.
    #[must_use]
    pub const fn is_member(&self) -> bool {
        !self.is_creator && !self.is_admin
    }

    /// Returns the participant's display role.
    #[must_use]
    pub fn role(&self) -> &str {
        if self.is_creator {
            "creator"
        } else if self.is_admin {
            self.admin_rank.as_deref().unwrap_or("admin")
        } else {
            "member"
        }
    }
}

/// Full chat information from `messages.getFullChat`.
///
/// Contains complete chat data including participants, settings, and metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatFull {
    /// Chat ID
    pub chat_id: ChatId,
    /// List of participants
    pub participants: Vec<Participant>,
    /// Chat title
    pub title: String,
    /// Chat photo
    pub photo: Option<ChatPhoto>,
    /// About/description text
    pub about: Option<String>,
    /// Whether participants can be invited
    pub can_participants_invite: bool,
    /// Whether all participants are admins
    pub all_participants_are_admins: bool,
    /// Whether to migrate to supergroup
    pub migrate_to_supergroup: bool,
    /// Migration channel ID (if migrating)
    pub migrated_to_channel_id: Option<ChannelId>,
    /// Chat creation date
    pub created_date: Option<i32>,
    /// Admin list (for groups with admins)
    pub admin_list: Vec<UserId>,
    /// Banned user IDs
    pub banned_user_ids: Vec<UserId>,
    /// Kicked user IDs
    pub kicked_user_ids: Vec<UserId>,
}

impl ChatFull {
    /// Creates a new empty ChatFull.
    #[must_use]
    pub fn new(chat_id: ChatId) -> Self {
        Self {
            chat_id,
            participants: Vec::new(),
            title: String::new(),
            photo: None,
            about: None,
            can_participants_invite: false,
            all_participants_are_admins: false,
            migrate_to_supergroup: false,
            migrated_to_channel_id: None,
            created_date: None,
            admin_list: Vec::new(),
            banned_user_ids: Vec::new(),
            kicked_user_ids: Vec::new(),
        }
    }

    /// Returns the number of participants.
    #[must_use]
    pub fn participant_count(&self) -> usize {
        self.participants.len()
    }

    /// Returns `true` if this is a full group (all participants are admins).
    #[must_use]
    pub const fn is_full_group(&self) -> bool {
        self.all_participants_are_admins
    }
}

/// Full channel information from `channels.getFullChannel`.
///
/// Contains complete channel data including settings, statistics, and metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelFull {
    /// Channel ID
    pub channel_id: ChannelId,
    /// Channel title
    pub title: String,
    /// Channel photo
    pub photo: Option<ChatPhoto>,
    /// About/description text
    pub about: Option<String>,
    /// Participant count
    pub participant_count: i32,
    /// Admin count
    pub admin_count: i32,
    /// Kicked user count
    pub kicked_count: i32,
    /// Banned user count
    pub banned_count: i32,
    /// Online participant count
    pub online_count: i32,
    /// View count (for channels)
    pub view_count: Option<i32>,
    /// Whether this is a broadcast channel
    pub is_broadcast: bool,
    /// Whether to hide participants list
    pub hide_participants: bool,
    /// Whether signatures are enabled
    pub signatures_enabled: bool,
    /// Whether this channel has scheduled messages
    pub has_scheduled: bool,
    /// Linked chat ID (for discussion groups)
    pub linked_chat_id: Option<ChatId>,
    /// Slow mode delay (in seconds)
    pub slowmode_delay: Option<i32>,
    /// Channel creation date
    pub created_date: Option<i32>,
    /// Sticker set username
    pub sticker_set_username: Option<String>,
}

impl ChannelFull {
    /// Creates a new empty ChannelFull.
    #[must_use]
    pub fn new(channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            title: String::new(),
            photo: None,
            about: None,
            participant_count: 0,
            admin_count: 0,
            kicked_count: 0,
            banned_count: 0,
            online_count: 0,
            view_count: None,
            is_broadcast: false,
            hide_participants: false,
            signatures_enabled: false,
            has_scheduled: false,
            linked_chat_id: None,
            slowmode_delay: None,
            created_date: None,
            sticker_set_username: None,
        }
    }

    /// Returns `true` if this is a group (not a broadcast channel).
    #[must_use]
    pub const fn is_group(&self) -> bool {
        !self.is_broadcast
    }

    /// Returns `true` if slow mode is enabled.
    #[must_use]
    pub fn has_slowmode(&self) -> bool {
        self.slowmode_delay.is_some() && self.slowmode_delay.unwrap_or(0) > 0
    }
}

/// Chat photo information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatPhoto {
    /// Photo ID
    pub photo_id: i64,
    /// Small photo thumbnail
    pub small: Option<FileLocation>,
    /// Big photo
    pub big: Option<FileLocation>,
}

impl ChatPhoto {
    /// Creates a new empty ChatPhoto.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            photo_id: 0,
            small: None,
            big: None,
        }
    }
}

impl Default for ChatPhoto {
    fn default() -> Self {
        Self::new()
    }
}

/// File location for downloading media.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileLocation {
    /// Volume ID
    pub volume_id: i64,
    /// Local ID within volume
    pub local_id: i32,
    /// Secret for authentication
    pub secret: i64,
    /// File reference for cloud files
    pub file_reference: Vec<u8>,
}

impl FileLocation {
    /// Creates a new empty FileLocation.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            volume_id: 0,
            local_id: 0,
            secret: 0,
            file_reference: Vec::new(),
        }
    }
}

impl Default for FileLocation {
    fn default() -> Self {
        Self::new()
    }
}

// ========== TL Request/Response Types ==========

/// Request for `messages.getFullChat`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFullChatRequest {
    /// Chat ID to fetch full info for.
    pub chat_id: ChatId,
}

impl TlSerialize for GetFullChatRequest {
    fn serialize_tl(&self, buf: &mut BytesMut) -> std::result::Result<(), rustgram_types::TypeError> {
        TlHelper::write_constructor_id(buf, MESSAGES_GET_FULL_CHAT);
        TlHelper::write_i64(buf, self.chat_id.get());
        Ok(())
    }
}

/// Response from `messages.getFullChat`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFullChatResponse {
    /// Full chat information.
    pub full_chat: ChatFull,
    /// Associated chat information.
    pub chats: Vec<ChatInfo>,
    /// Associated user information.
    pub users: Vec<UserInfo>,
}

impl TlDeserialize for GetFullChatResponse {
    fn deserialize_tl(
        buf: &mut rustgram_types::tl::Bytes,
    ) -> std::result::Result<Self, rustgram_types::TypeError> {
        // messages.chatFull#e5d7d19c full_chat:ChatFull chats:Vector<Chat> users:Vector<User>
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        if constructor_id != 0xe5d7d19c {
            return Err(rustgram_types::TypeError::DeserializationError(format!(
                "Expected messages.chatFull (0xe5d7d19c), got 0x{:08x}",
                constructor_id
            )));
        }

        let full_chat = deserialize_chat_full(buf)?;
        let chats = deserialize_chat_vector(buf)?;
        let users = deserialize_user_vector(buf)?;

        Ok(Self {
            full_chat,
            chats,
            users,
        })
    }
}

/// Request for `channels.getFullChannel`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFullChannelRequest {
    /// Input channel to fetch full info for.
    pub channel: InputChannel,
}

impl TlSerialize for GetFullChannelRequest {
    fn serialize_tl(&self, buf: &mut BytesMut) -> std::result::Result<(), rustgram_types::TypeError> {
        TlHelper::write_constructor_id(buf, CHANNELS_GET_FULL_CHANNEL);
        self.channel.serialize_tl(buf)?;
        Ok(())
    }
}

/// Response from `channels.getFullChannel`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetFullChannelResponse {
    /// Full channel information.
    pub full_channel: ChannelFull,
    /// Associated chat information.
    pub chats: Vec<ChatInfo>,
    /// Associated user information.
    pub users: Vec<UserInfo>,
}

impl TlDeserialize for GetFullChannelResponse {
    fn deserialize_tl(
        buf: &mut rustgram_types::tl::Bytes,
    ) -> std::result::Result<Self, rustgram_types::TypeError> {
        // messages.chatFull#e5d7d19c full_chat:ChatFull chats:Vector<Chat> users:Vector<User>
        // Note: channels.getFullChannel also returns messages.ChatFull, not a separate type
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        if constructor_id != 0xe5d7d19c {
            return Err(rustgram_types::TypeError::DeserializationError(format!(
                "Expected messages.chatFull (0xe5d7d19c), got 0x{:08x}",
                constructor_id
            )));
        }

        let chat_full = deserialize_chat_full(buf)?;
        let chats = deserialize_chat_vector(buf)?;
        let users = deserialize_user_vector(buf)?;

        // Extract channel_id from ChatFull
        let channel_id = ChannelId::new(chat_full.chat_id.get())
            .map_err(|e| rustgram_types::TypeError::Other(e.to_string()))?;

        // Convert ChatFull to ChannelFull
        let full_channel = ChannelFull {
            channel_id,
            title: chat_full.title,
            photo: chat_full.photo,
            about: chat_full.about,
            participant_count: chat_full.participants.len() as i32,
            admin_count: chat_full.admin_list.len() as i32,
            kicked_count: chat_full.kicked_user_ids.len() as i32,
            banned_count: chat_full.banned_user_ids.len() as i32,
            online_count: 0, // Not available in ChatFull
            view_count: None,
            is_broadcast: false,
            hide_participants: false,
            signatures_enabled: false,
            has_scheduled: false,
            linked_chat_id: None,
            slowmode_delay: None,
            created_date: chat_full.created_date,
            sticker_set_username: None,
        };

        Ok(Self {
            full_channel,
            chats,
            users,
        })
    }
}

/// Request for `channels.getParticipants`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetChannelParticipantsRequest {
    /// Input channel to fetch participants from.
    pub channel: InputChannel,
    /// Filter for which participants to fetch.
    pub filter: ChannelParticipantsFilter,
    /// Offset for pagination.
    pub offset: i32,
    /// Maximum number of participants to return.
    pub limit: i32,
}

impl TlSerialize for GetChannelParticipantsRequest {
    fn serialize_tl(&self, buf: &mut BytesMut) -> std::result::Result<(), rustgram_types::TypeError> {
        TlHelper::write_constructor_id(buf, CHANNELS_GET_PARTICIPANTS);
        self.channel.serialize_tl(buf)?;
        self.filter.serialize_tl(buf)?;
        TlHelper::write_i32(buf, self.offset);
        TlHelper::write_i32(buf, self.limit);
        TlHelper::write_i64(buf, 0); // hash
        Ok(())
    }
}

/// Response from `channels.getParticipants`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetChannelParticipantsResponse {
    /// List of participants.
    pub participants: Vec<Participant>,
    /// Total participant count.
    pub count: i32,
}

impl TlDeserialize for GetChannelParticipantsResponse {
    fn deserialize_tl(
        buf: &mut rustgram_types::tl::Bytes,
    ) -> std::result::Result<Self, rustgram_types::TypeError> {
        // channels.channelParticipants#9ab0feaf count:int participants:Vector<ChannelParticipant> ...
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        if constructor_id == 0xf0173fe9 {
            // channels.channelParticipantsNotModified#f0173fe9
            return Ok(Self {
                participants: Vec::new(),
                count: 0,
            });
        }

        if constructor_id != 0x9ab0feaf {
            return Err(rustgram_types::TypeError::DeserializationError(format!(
                "Expected channels.channelParticipants (0x9ab0feaf), got 0x{:08x}",
                constructor_id
            )));
        }

        let count = TlHelper::read_i32(buf)?;
        let participants = deserialize_channel_participant_vector(buf)?;

        // Skip chats and users vectors for now
        let _chats = deserialize_chat_vector(buf)?;
        let _users = deserialize_user_vector(buf)?;

        Ok(Self {
            participants,
            count,
        })
    }
}

/// Deserializes a vector of ChannelParticipant.
fn deserialize_channel_participant_vector(
    buf: &mut rustgram_types::tl::Bytes,
) -> std::result::Result<Vec<Participant>, rustgram_types::TypeError> {
    let vector_constructor = TlHelper::read_constructor_id(buf)?;

    if vector_constructor != 0x1cb5c415 {
        return Err(rustgram_types::TypeError::DeserializationError(format!(
            "Expected vector constructor (0x1cb5c415), got 0x{:08x}",
            vector_constructor
        )));
    }

    let count = TlHelper::read_i32(buf)? as usize;
    let mut participants = Vec::with_capacity(count);

    for _ in 0..count {
        participants.push(deserialize_channel_participant(buf)?);
    }

    Ok(participants)
}

/// Deserializes a single ChannelParticipant.
fn deserialize_channel_participant(
    buf: &mut rustgram_types::tl::Bytes,
) -> std::result::Result<Participant, rustgram_types::TypeError> {
    let constructor_id = TlHelper::read_constructor_id(buf)?;

    match constructor_id {
        // channelParticipant#cb397619 flags:# user_id:long date:int
        0xcb397619 => {
            let _flags = TlHelper::read_i32(buf)?;
            let user_id_val = TlHelper::read_i64(buf)?;
            let date = TlHelper::read_i32(buf)?;

            let user_id = UserId::new(user_id_val)?;

            Ok(Participant {
                user_id,
                participant_id: user_id_val,
                joined_date: Some(date),
                inviter_id: None,
                is_creator: false,
                is_admin: false,
                admin_rank: None,
                can_be_invited: true,
            })
        }
        // channelParticipantCreator#2fe601d3 flags:# user_id:long ...
        0x2fe601d3 => {
            let _flags = TlHelper::read_i32(buf)?;
            let user_id_val = TlHelper::read_i64(buf)?;

            // Skip admin_rights and rank for now
            let _admin_rights_constructor = TlHelper::read_constructor_id(buf)?;
            let _admin_rights_flags = TlHelper::read_i32(buf)?;

            let user_id = UserId::new(user_id_val)?;

            Ok(Participant {
                user_id,
                participant_id: user_id_val,
                joined_date: None,
                inviter_id: None,
                is_creator: true,
                is_admin: false,
                admin_rank: None,
                can_be_invited: true,
            })
        }
        // channelParticipantAdmin#34c3bb53 flags:# ... user_id:long ...
        0x34c3bb53 => {
            let flags = TlHelper::read_i32(buf)?;
            let user_id_val = TlHelper::read_i64(buf)?;

            // Skip inviter_id, promoted_by, date, admin_rights, rank for now
            if flags & (1 << 1) != 0 {
                let _ = TlHelper::read_i64(buf)?; // inviter_id
            }
            let _promoted_by = TlHelper::read_i64(buf)?;
            let _date = TlHelper::read_i32(buf)?;

            // Skip admin_rights
            let _ = TlHelper::read_constructor_id(buf)?;
            let _ = TlHelper::read_i32(buf)?; // admin rights flags

            // Read rank if present
            let admin_rank = if flags & (1 << 2) != 0 {
                Some(TlHelper::read_string(buf)?)
            } else {
                None
            };

            let user_id = UserId::new(user_id_val)?;

            Ok(Participant {
                user_id,
                participant_id: user_id_val,
                joined_date: None,
                inviter_id: None,
                is_creator: false,
                is_admin: true,
                admin_rank,
                can_be_invited: true,
            })
        }
        // Other participant types - return simplified version
        _ => {
            // Skip remaining data and return a basic participant
            let user_id_val = TlHelper::read_i64(buf)?;
            let user_id = UserId::new(user_id_val)?;

            Ok(Participant {
                user_id,
                participant_id: user_id_val,
                joined_date: None,
                inviter_id: None,
                is_creator: false,
                is_admin: false,
                admin_rank: None,
                can_be_invited: true,
            })
        }
    }
}

/// Filter for channel participants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelParticipantsFilter {
    /// All participants
    All,
    /// Only creators and admins
    Admins,
    /// Recently online participants
    Online,
    /// Participants with custom emoji
    Custom,
}

impl ChannelParticipantsFilter {
    /// Returns the constructor ID for this filter.
    pub const fn constructor_id(&self) -> u32 {
        match self {
            Self::All => 0xfede841,
            Self::Admins => 0x6a4b38de,
            Self::Online => 0x14b2450d,
            Self::Custom => 0x7b1b990e,
        }
    }

    /// Serializes this filter to TL format.
    pub fn serialize_tl(&self, buf: &mut BytesMut) -> std::result::Result<(), rustgram_types::TypeError> {
        TlHelper::write_constructor_id(buf, self.constructor_id());
        Ok(())
    }
}

/// Input channel for API requests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputChannel {
    /// Channel ID.
    pub channel_id: ChannelId,
    /// Access hash for authentication.
    pub access_hash: AccessHash,
}

impl InputChannel {
    /// Serializes this InputChannel to TL format.
    pub fn serialize_tl(&self, buf: &mut BytesMut) -> std::result::Result<(), rustgram_types::TypeError> {
        // inputChannel#fc8c9d90
        TlHelper::write_constructor_id(buf, 0xfc8c9d90);
        TlHelper::write_i64(buf, self.channel_id.get());
        TlHelper::write_i64(buf, self.access_hash.get());
        Ok(())
    }
}

/// Simplified chat information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatInfo {
    /// Chat ID.
    pub id: ChatId,
    /// Chat title.
    pub title: String,
    /// Participant count if available.
    pub participant_count: Option<i32>,
}

/// Simplified user information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserInfo {
    /// User ID.
    pub id: UserId,
    /// First name.
    pub first_name: Option<String>,
    /// Last name.
    pub last_name: Option<String>,
    /// Username.
    pub username: Option<String>,
}

// ========== TL Deserialization Helpers ==========

/// Deserializes a ChatFull from TL buffer.
fn deserialize_chat_full(
    buf: &mut rustgram_types::tl::Bytes,
) -> std::result::Result<ChatFull, rustgram_types::TypeError> {
    // chatFull#2633421b flags:# ...
    let constructor_id = TlHelper::read_constructor_id(buf)?;

    if constructor_id != 0x2633421b {
        return Err(rustgram_types::TypeError::DeserializationError(format!(
            "Expected chatFull (0x2633421b), got 0x{:08x}",
            constructor_id
        )));
    }

    let flags = TlHelper::read_i32(buf)?;
    let id = TlHelper::read_i64(buf)?;
    let chat_id = ChatId::new(id)?;

    let about = TlHelper::read_string(buf)?;

    // Read ChatParticipants
    let participants = deserialize_chat_participants(buf)?;

    let chat_photo = if flags & (1 << 2) != 0 {
        Some(deserialize_photo(buf)?)
    } else {
        None
    };

    // Skip notify_settings for now
    let _notify_settings_constructor = TlHelper::read_constructor_id(buf)?;

    // Skip exported_invite if present
    if flags & (1 << 13) != 0 {
        let _ = TlHelper::read_constructor_id(buf)?;
    }

    // Skip bot_info, pinned_msg_id, folder_id, call, ttl_period, etc.
    // These are optional fields based on flags

    let title = String::new(); // Would need to read from Chat
    let can_participants_invite = true;
    let all_participants_are_admins = false;
    let migrate_to_supergroup = false;
    let migrated_to_channel_id = None;
    let created_date = None;
    let admin_list = Vec::new();
    let banned_user_ids = Vec::new();
    let kicked_user_ids = Vec::new();

    Ok(ChatFull {
        chat_id,
        participants,
        title,
        photo: chat_photo.map(|p| ChatPhoto {
            photo_id: p.id,
            small: None,
            big: None,
        }),
        about: if about.is_empty() { None } else { Some(about) },
        can_participants_invite,
        all_participants_are_admins,
        migrate_to_supergroup,
        migrated_to_channel_id,
        created_date,
        admin_list,
        banned_user_ids,
        kicked_user_ids,
    })
}

/// Deserializes ChatParticipants from TL buffer.
fn deserialize_chat_participants(
    buf: &mut rustgram_types::tl::Bytes,
) -> std::result::Result<Vec<Participant>, rustgram_types::TypeError> {
    // chatParticipants#3cbc93f8 or chatParticipantsForbidden#8763d3e1
    let constructor_id = TlHelper::read_constructor_id(buf)?;

    match constructor_id {
        // chatParticipants#3cbc93f8 chat_id:long participants:Vector<ChatParticipant> version:int
        0x3cbc93f8 => {
            let _chat_id = TlHelper::read_i64(buf)?;
            let participants = deserialize_chat_participant_vector(buf)?;
            let _version = TlHelper::read_i32(buf)?;
            Ok(participants)
        }
        // chatParticipantsForbidden#8763d3e1 - user was kicked or doesn't have access
        0x8763d3e1 => {
            let flags = TlHelper::read_i32(buf)?;
            let _chat_id = TlHelper::read_i64(buf)?;

            // Read self_participant if present
            if flags & 1 != 0 {
                let _self_participant = deserialize_chat_participant(buf)?;
            }

            Ok(Vec::new()) // No participants accessible
        }
        _ => Err(rustgram_types::TypeError::DeserializationError(format!(
            "Unknown ChatParticipants constructor: 0x{:08x}",
            constructor_id
        ))),
    }
}

/// Deserializes a vector of ChatParticipant.
fn deserialize_chat_participant_vector(
    buf: &mut rustgram_types::tl::Bytes,
) -> std::result::Result<Vec<Participant>, rustgram_types::TypeError> {
    let vector_constructor = TlHelper::read_constructor_id(buf)?;

    if vector_constructor != 0x1cb5c415 {
        return Err(rustgram_types::TypeError::DeserializationError(format!(
            "Expected vector constructor (0x1cb5c415), got 0x{:08x}",
            vector_constructor
        )));
    }

    let count = TlHelper::read_i32(buf)? as usize;
    let mut participants = Vec::with_capacity(count);

    for _ in 0..count {
        participants.push(deserialize_chat_participant(buf)?);
    }

    Ok(participants)
}

/// Deserializes a single ChatParticipant.
fn deserialize_chat_participant(
    buf: &mut rustgram_types::tl::Bytes,
) -> std::result::Result<Participant, rustgram_types::TypeError> {
    let constructor_id = TlHelper::read_constructor_id(buf)?;

    match constructor_id {
        // chatParticipant#c02d4007 user_id:long inviter_id:long date:int
        0xc02d4007 => {
            let user_id_val = TlHelper::read_i64(buf)?;
            let inviter_id_val = TlHelper::read_i64(buf)?;
            let date = TlHelper::read_i32(buf)?;

            let user_id = UserId::new(user_id_val)?;
            let inviter_id = UserId::new(inviter_id_val).ok();

            Ok(Participant {
                user_id,
                participant_id: user_id_val,
                joined_date: Some(date),
                inviter_id,
                is_creator: false,
                is_admin: false,
                admin_rank: None,
                can_be_invited: true,
            })
        }
        // chatParticipantCreator#e46bcee4 user_id:long
        0xe46bcee4 => {
            let user_id_val = TlHelper::read_i64(buf)?;
            let user_id = UserId::new(user_id_val)?;

            Ok(Participant {
                user_id,
                participant_id: user_id_val,
                joined_date: None,
                inviter_id: None,
                is_creator: true,
                is_admin: false,
                admin_rank: None,
                can_be_invited: true,
            })
        }
        // chatParticipantAdmin#a0933f5b user_id:long inviter_id:long date:int
        0xa0933f5b => {
            let user_id_val = TlHelper::read_i64(buf)?;
            let inviter_id_val = TlHelper::read_i64(buf)?;
            let date = TlHelper::read_i32(buf)?;

            let user_id = UserId::new(user_id_val)?;
            let inviter_id = UserId::new(inviter_id_val).ok();

            Ok(Participant {
                user_id,
                participant_id: user_id_val,
                joined_date: Some(date),
                inviter_id,
                is_creator: false,
                is_admin: true,
                admin_rank: Some("admin".to_string()),
                can_be_invited: true,
            })
        }
        _ => Err(rustgram_types::TypeError::DeserializationError(format!(
            "Unknown ChatParticipant constructor: 0x{:08x}",
            constructor_id
        ))),
    }
}

/// Deserializes a Photo (simplified).
fn deserialize_photo(
    buf: &mut rustgram_types::tl::Bytes,
) -> std::result::Result<Photo, rustgram_types::TypeError> {
    let constructor_id = TlHelper::read_constructor_id(buf)?;

    match constructor_id {
        // photoEmpty#69613b5b id:long
        0x69613b5b => {
            let id = TlHelper::read_i64(buf)?;
            Ok(Photo { id, has_video: false })
        }
        // Other photo variants - simplified
        _ => {
            // Skip remaining data for now
            Ok(Photo { id: 0, has_video: false })
        }
    }
}

/// Simplified Photo struct for deserialization.
#[allow(dead_code)]
struct Photo {
    id: i64,
    #[allow(dead_code)]
    has_video: bool,
}

/// Deserializes a vector of Chat (simplified).
fn deserialize_chat_vector(
    buf: &mut rustgram_types::tl::Bytes,
) -> std::result::Result<Vec<ChatInfo>, rustgram_types::TypeError> {
    let vector_constructor = TlHelper::read_constructor_id(buf)?;

    if vector_constructor != 0x1cb5c415 {
        // Empty vector or skip
        return Ok(Vec::new());
    }

    let count = TlHelper::read_i32(buf)? as usize;
    let chats = Vec::with_capacity(count);

    for _ in 0..count {
        let _chat_constructor = TlHelper::read_constructor_id(buf)?;
        // Skip chat data for now - just advance past it
        // A full implementation would deserialize all chat fields
    }

    Ok(chats)
}

/// Deserializes a vector of User (simplified).
fn deserialize_user_vector(
    buf: &mut rustgram_types::tl::Bytes,
) -> std::result::Result<Vec<UserInfo>, rustgram_types::TypeError> {
    let vector_constructor = TlHelper::read_constructor_id(buf)?;

    if vector_constructor != 0x1cb5c415 {
        // Empty vector or skip
        return Ok(Vec::new());
    }

    let count = TlHelper::read_i32(buf)? as usize;
    let users = Vec::with_capacity(count);

    for _ in 0..count {
        let _user_constructor = TlHelper::read_constructor_id(buf)?;
        // Skip user data for now - just advance past it
        // A full implementation would deserialize all user fields
    }

    Ok(users)
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // ParticipantManager tests
    // =========================================================================

    #[test]
    fn test_participant_manager_new() {
        let manager = ParticipantManager::new();
        assert_eq!(manager.timeout_secs, 30);
    }

    #[test]
    fn test_participant_manager_default() {
        let manager = ParticipantManager::default();
        assert_eq!(manager.timeout_secs, 30);
    }

    #[test]
    fn test_participant_manager_with_timeout() {
        let manager = ParticipantManager::with_timeout(60);
        assert_eq!(manager.timeout_secs, 60);
    }

    #[test]
    fn test_constants() {
        assert_eq!(DEFAULT_PARTICIPANTS_LIMIT, 100);
        assert_eq!(MAX_PARTICIPANTS_LIMIT, 200);
        assert_eq!(ParticipantManager::DEFAULT_TIMEOUT_SECS, 30);
        assert_eq!(MESSAGES_GET_FULL_CHAT, 0xaeb00b34);
        assert_eq!(CHANNELS_GET_FULL_CHANNEL, 0x8736a09);
        assert_eq!(CHANNELS_GET_PARTICIPANTS, 0x77ced9d0);
    }

    // =========================================================================
    // GetParticipantsRequest tests
    // =========================================================================

    #[test]
    fn test_get_participants_request_chat() {
        let chat_id = ChatId::new(123).unwrap();
        let request = GetParticipantsRequest::chat(chat_id);
        assert!(matches!(request, GetParticipantsRequest::Chat(id) if id == chat_id));
    }

    #[test]
    fn test_get_participants_request_channel() {
        let channel_id = ChannelId::new(456).unwrap();
        let request = GetParticipantsRequest::channel(channel_id, 789);

        match request {
            GetParticipantsRequest::Channel {
                channel_id: id,
                access_hash,
                offset,
                limit,
            } => {
                assert_eq!(id, channel_id);
                assert_eq!(access_hash, 789);
                assert_eq!(offset, 0);
                assert_eq!(limit, 100);
            }
            _ => panic!("Expected Channel variant"),
        }
    }

    #[test]
    fn test_get_participants_request_channel_paginated() {
        let channel_id = ChannelId::new(456).unwrap();
        let request = GetParticipantsRequest::channel_paginated(channel_id, 789, 50, 150);

        match request {
            GetParticipantsRequest::Channel {
                channel_id: id,
                access_hash,
                offset,
                limit,
            } => {
                assert_eq!(id, channel_id);
                assert_eq!(access_hash, 789);
                assert_eq!(offset, 50);
                assert_eq!(limit, 150);
            }
            _ => panic!("Expected Channel variant"),
        }
    }

    #[test]
    fn test_get_participants_request_clone() {
        let request = GetParticipantsRequest::Chat(ChatId::new(123).unwrap());
        let cloned = request.clone();
        assert_eq!(request, cloned);
    }

    // =========================================================================
    // Participant tests
    // =========================================================================

    #[test]
    fn test_participant_new() {
        let user_id = UserId::new(123).unwrap();
        let participant = Participant::new(user_id);

        assert_eq!(participant.user_id, user_id);
        assert_eq!(participant.participant_id, user_id.get());
        assert!(participant.joined_date.is_none());
        assert!(participant.inviter_id.is_none());
        assert!(!participant.is_creator);
        assert!(!participant.is_admin);
        assert!(participant.admin_rank.is_none());
        assert!(participant.can_be_invited);
    }

    #[test]
    fn test_participant_is_member() {
        let user_id = UserId::new(123).unwrap();
        let mut participant = Participant::new(user_id);

        assert!(participant.is_member());

        participant.is_admin = true;
        assert!(!participant.is_member());

        participant.is_admin = false;
        participant.is_creator = true;
        assert!(!participant.is_member());
    }

    #[test]
    fn test_participant_role() {
        let user_id = UserId::new(123).unwrap();
        let mut participant = Participant::new(user_id);

        assert_eq!(participant.role(), "member");

        participant.is_admin = true;
        assert_eq!(participant.role(), "admin");

        participant.admin_rank = Some("moderator".to_string());
        assert_eq!(participant.role(), "moderator");

        participant.is_admin = false;
        participant.is_creator = true;
        assert_eq!(participant.role(), "creator");
    }

    #[test]
    fn test_participant_clone() {
        let user_id = UserId::new(123).unwrap();
        let participant = Participant::new(user_id);
        let cloned = participant.clone();

        assert_eq!(participant, cloned);
    }

    #[test]
    fn test_participant_equality() {
        let user_id = UserId::new(123).unwrap();
        let p1 = Participant::new(user_id);
        let p2 = Participant::new(user_id);

        assert_eq!(p1, p2);
    }

    // =========================================================================
    // ChatFull tests
    // =========================================================================

    #[test]
    fn test_chat_full_new() {
        let chat_id = ChatId::new(123).unwrap();
        let full = ChatFull::new(chat_id);

        assert_eq!(full.chat_id, chat_id);
        assert!(full.participants.is_empty());
        assert!(full.title.is_empty());
        assert!(full.photo.is_none());
        assert!(full.about.is_none());
        assert!(!full.can_participants_invite);
        assert!(!full.all_participants_are_admins);
        assert!(!full.migrate_to_supergroup);
        assert!(full.migrated_to_channel_id.is_none());
    }

    #[test]
    fn test_chat_full_participant_count() {
        let chat_id = ChatId::new(123).unwrap();
        let mut full = ChatFull::new(chat_id);

        assert_eq!(full.participant_count(), 0);

        full.participants
            .push(Participant::new(UserId::new(1).unwrap()));
        full.participants
            .push(Participant::new(UserId::new(2).unwrap()));
        assert_eq!(full.participant_count(), 2);
    }

    #[test]
    fn test_chat_full_is_full_group() {
        let chat_id = ChatId::new(123).unwrap();
        let mut full = ChatFull::new(chat_id);

        assert!(!full.is_full_group());

        full.all_participants_are_admins = true;
        assert!(full.is_full_group());
    }

    #[test]
    fn test_chat_full_clone() {
        let chat_id = ChatId::new(123).unwrap();
        let full = ChatFull::new(chat_id);
        let cloned = full.clone();

        assert_eq!(full, cloned);
    }

    // =========================================================================
    // ChannelFull tests
    // =========================================================================

    #[test]
    fn test_channel_full_new() {
        let channel_id = ChannelId::new(456).unwrap();
        let full = ChannelFull::new(channel_id);

        assert_eq!(full.channel_id, channel_id);
        assert!(full.title.is_empty());
        assert!(full.photo.is_none());
        assert!(full.about.is_none());
        assert_eq!(full.participant_count, 0);
        assert_eq!(full.admin_count, 0);
        assert!(!full.is_broadcast);
        assert!(!full.hide_participants);
        assert!(!full.signatures_enabled);
    }

    #[test]
    fn test_channel_full_is_group() {
        let channel_id = ChannelId::new(456).unwrap();
        let mut full = ChannelFull::new(channel_id);

        assert!(full.is_group());

        full.is_broadcast = true;
        assert!(!full.is_group());
    }

    #[test]
    fn test_channel_full_has_slowmode() {
        let channel_id = ChannelId::new(456).unwrap();
        let mut full = ChannelFull::new(channel_id);

        assert!(!full.has_slowmode());

        full.slowmode_delay = Some(30);
        assert!(full.has_slowmode());

        full.slowmode_delay = Some(0);
        assert!(!full.has_slowmode());
    }

    #[test]
    fn test_channel_full_clone() {
        let channel_id = ChannelId::new(456).unwrap();
        let full = ChannelFull::new(channel_id);
        let cloned = full.clone();

        assert_eq!(full, cloned);
    }

    // =========================================================================
    // ChatPhoto tests
    // =========================================================================

    #[test]
    fn test_chat_photo_new() {
        let photo = ChatPhoto::new();

        assert_eq!(photo.photo_id, 0);
        assert!(photo.small.is_none());
        assert!(photo.big.is_none());
    }

    #[test]
    fn test_chat_photo_default() {
        let photo = ChatPhoto::default();

        assert_eq!(photo.photo_id, 0);
        assert!(photo.small.is_none());
        assert!(photo.big.is_none());
    }

    #[test]
    fn test_chat_photo_clone() {
        let mut photo = ChatPhoto::new();
        photo.photo_id = 123;

        let cloned = photo.clone();
        assert_eq!(photo, cloned);
    }

    // =========================================================================
    // FileLocation tests
    // =========================================================================

    #[test]
    fn test_file_location_new() {
        let loc = FileLocation::new();

        assert_eq!(loc.volume_id, 0);
        assert_eq!(loc.local_id, 0);
        assert_eq!(loc.secret, 0);
        assert!(loc.file_reference.is_empty());
    }

    #[test]
    fn test_file_location_default() {
        let loc = FileLocation::default();

        assert_eq!(loc.volume_id, 0);
        assert_eq!(loc.local_id, 0);
        assert_eq!(loc.secret, 0);
        assert!(loc.file_reference.is_empty());
    }

    #[test]
    fn test_file_location_clone() {
        let loc = FileLocation::new();
        let cloned = loc.clone();

        assert_eq!(loc, cloned);
    }

    // =========================================================================
    // ChannelParticipantsFilter tests
    // =========================================================================

    #[test]
    fn test_filter_all() {
        let filter = ChannelParticipantsFilter::All;
        assert_eq!(filter.constructor_id(), 0xfede841);
    }

    #[test]
    fn test_filter_admins() {
        let filter = ChannelParticipantsFilter::Admins;
        assert_eq!(filter.constructor_id(), 0x6a4b38de);
    }

    #[test]
    fn test_filter_online() {
        let filter = ChannelParticipantsFilter::Online;
        assert_eq!(filter.constructor_id(), 0x14b2450d);
    }

    #[test]
    fn test_filter_custom() {
        let filter = ChannelParticipantsFilter::Custom;
        assert_eq!(filter.constructor_id(), 0x7b1b990e);
    }

    #[test]
    fn test_filter_serialize() {
        let filter = ChannelParticipantsFilter::All;
        let mut buf = BytesMut::new();

        let result = filter.serialize_tl(&mut buf);
        assert!(result.is_ok());
        assert!(buf.len() >= 4);
    }

    #[test]
    fn test_filter_clone() {
        let filter = ChannelParticipantsFilter::All;
        let cloned = filter;

        assert_eq!(filter, cloned);
    }

    #[test]
    fn test_filter_copy() {
        let filter1 = ChannelParticipantsFilter::Admins;
        let filter2 = filter1;

        assert_eq!(filter1, filter2);
    }

    // =========================================================================
    // InputChannel tests
    // =========================================================================

    #[test]
    fn test_input_channel_serialize() {
        let channel_id = ChannelId::new(123).unwrap();
        let access_hash = rustgram_types::AccessHash::new(456);
        let channel = InputChannel {
            channel_id,
            access_hash,
        };

        let mut buf = BytesMut::new();
        let result = channel.serialize_tl(&mut buf);

        assert!(result.is_ok());
        assert!(buf.len() >= 20); // constructor (4) + channel_id (8) + access_hash (8)
    }

    #[test]
    fn test_input_channel_clone() {
        let channel_id = ChannelId::new(123).unwrap();
        let access_hash = rustgram_types::AccessHash::new(456);
        let channel = InputChannel {
            channel_id,
            access_hash,
        };
        let cloned = channel.clone();

        assert_eq!(channel, cloned);
    }

    // =========================================================================
    // TL Request tests
    // =========================================================================

    #[test]
    fn test_get_full_chat_request_serialize() {
        let chat_id = ChatId::new(123).unwrap();
        let request = GetFullChatRequest { chat_id };

        let mut buf = BytesMut::new();
        let result = request.serialize_tl(&mut buf);

        assert!(result.is_ok());
        assert!(buf.len() >= 12); // constructor (4) + chat_id (8)
    }

    #[test]
    fn test_get_full_channel_request_serialize() {
        let channel_id = ChannelId::new(123).unwrap();
        let access_hash = rustgram_types::AccessHash::new(456);
        let channel = InputChannel {
            channel_id,
            access_hash,
        };
        let request = GetFullChannelRequest { channel };

        let mut buf = BytesMut::new();
        let result = request.serialize_tl(&mut buf);

        assert!(result.is_ok());
        assert!(buf.len() >= 24); // constructor (4) + channel_id (8) + access_hash (8)
    }

    #[test]
    fn test_get_channel_participants_request_serialize() {
        let channel_id = ChannelId::new(123).unwrap();
        let access_hash = rustgram_types::AccessHash::new(456);
        let channel = InputChannel {
            channel_id,
            access_hash,
        };
        let request = GetChannelParticipantsRequest {
            channel,
            filter: ChannelParticipantsFilter::All,
            offset: 0,
            limit: 100,
        };

        let mut buf = BytesMut::new();
        let result = request.serialize_tl(&mut buf);

        assert!(result.is_ok());
        assert!(buf.len() >= 32); // Full serialization
    }

    // =========================================================================
    // Integration tests
    // =========================================================================

    #[test]
    fn test_full_workflow() {
        let chat_id = ChatId::new(123).unwrap();
        let mut chat_full = ChatFull::new(chat_id);

        // Add participants
        for i in 1..=5 {
            chat_full
                .participants
                .push(Participant::new(UserId::new(i).unwrap()));
        }

        assert_eq!(chat_full.participant_count(), 5);
        assert!(!chat_full.is_full_group());

        // Add admins
        for i in 1..=2 {
            chat_full.admin_list.push(UserId::new(i).unwrap());
        }

        assert_eq!(chat_full.admin_list.len(), 2);
    }

    #[test]
    fn test_channel_full_with_stats() {
        let channel_id = ChannelId::new(456).unwrap();
        let mut channel_full = ChannelFull::new(channel_id);

        channel_full.participant_count = 1000;
        channel_full.admin_count = 5;
        channel_full.online_count = 50;
        channel_full.view_count = Some(10000);
        channel_full.is_broadcast = true;

        assert!(!channel_full.is_group());
        assert_eq!(channel_full.participant_count, 1000);
        assert_eq!(channel_full.admin_count, 5);
        assert_eq!(channel_full.online_count, 50);
        assert_eq!(channel_full.view_count, Some(10000));
    }
}
