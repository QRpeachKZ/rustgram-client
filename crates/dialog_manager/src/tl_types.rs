//! TL (Type Language) types for dialog operations.
//!
//! Defines request/response types for Telegram's messages.getDialogs API
//! and related dialog operations.

use bytes::BytesMut;
use rustgram_types::tl::Bytes as TlBytes;
use rustgram_types::{ChatId, DialogId, InputPeer, TlDeserialize, TlHelper, TlSerialize, UserId};

/// Request to load dialogs from server.
///
/// Corresponds to TL schema: `messages.getDialogs#a0f4cb4f`
///
/// # Examples
///
/// ```
/// use rustgram_dialog_manager::tl_types::GetDialogsRequest;
/// use rustgram_types::InputPeer;
///
/// let request = GetDialogsRequest {
///     offset_date: 0,
///     offset_id: 0,
///     offset_peer: InputPeer::empty(),
///     limit: 100,
///     hash: 0,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GetDialogsRequest {
    /// Offset date for pagination
    pub offset_date: i32,
    /// Offset dialog ID for pagination
    pub offset_id: i32,
    /// Offset peer for pagination
    pub offset_peer: InputPeer,
    /// Maximum number of dialogs to return (1-100)
    pub limit: i32,
    /// Hash for caching
    pub hash: i64,
}

impl GetDialogsRequest {
    /// TL constructor ID for messages.getDialogs.
    /// Verified from telegram_api.tl:2337
    pub const CONSTRUCTOR_ID: u32 = 0xa0f4cb4f;

    /// Creates a new get dialogs request with default values.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of dialogs to return
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::tl_types::GetDialogsRequest;
    ///
    /// let request = GetDialogsRequest::new(50);
    /// ```
    #[must_use]
    pub fn new(limit: i32) -> Self {
        Self {
            offset_date: 0,
            offset_id: 0,
            offset_peer: InputPeer::empty(),
            limit: limit.clamp(1, 100),
            hash: 0,
        }
    }

    /// Creates a new request with pagination parameters.
    ///
    /// # Arguments
    ///
    /// * `offset_date` - Offset date for pagination
    /// * `offset_id` - Offset dialog ID for pagination
    /// * `offset_peer` - Offset peer for pagination
    /// * `limit` - Maximum number of dialogs to return
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::tl_types::GetDialogsRequest;
    /// use rustgram_types::InputPeer;
    ///
    /// let request = GetDialogsRequest::with_pagination(
    ///     1234567890,
    ///     123,
    ///     InputPeer::empty(),
    ///     50
    /// );
    /// ```
    #[must_use]
    pub const fn with_pagination(
        offset_date: i32,
        offset_id: i32,
        offset_peer: InputPeer,
        limit: i32,
    ) -> Self {
        Self {
            offset_date,
            offset_id,
            offset_peer,
            limit,
            hash: 0,
        }
    }

    /// Validates the request parameters.
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err(String)` if invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::tl_types::GetDialogsRequest;
    /// use rustgram_types::InputPeer;
    ///
    /// let request = GetDialogsRequest::new(50);
    /// assert!(request.validate().is_ok());
    ///
    /// // Create invalid request with limit > 100 using with_pagination
    /// let request = GetDialogsRequest::with_pagination(0, 0, InputPeer::empty(), 101);
    /// assert!(request.validate().is_err());
    /// ```
    pub fn validate(&self) -> Result<(), String> {
        if self.limit < 1 || self.limit > 100 {
            return Err(format!(
                "limit must be between 1 and 100, got {}",
                self.limit
            ));
        }
        Ok(())
    }
}

impl TlSerialize for GetDialogsRequest {
    fn serialize_tl(&self, buf: &mut BytesMut) -> Result<(), rustgram_types::TypeError> {
        // Write constructor ID
        TlHelper::write_constructor_id(buf, Self::CONSTRUCTOR_ID);

        // Write flags (none for getDialogs)
        TlHelper::write_i32(buf, 0);

        // Write offset_date
        TlHelper::write_i32(buf, self.offset_date);

        // Write offset_id
        TlHelper::write_i32(buf, self.offset_id);

        // Write offset_peer (InputPeer)
        serialize_input_peer(buf, &self.offset_peer)?;

        // Write limit
        TlHelper::write_i32(buf, self.limit);

        // Write hash
        TlHelper::write_i64(buf, self.hash);

        Ok(())
    }
}

impl Default for GetDialogsRequest {
    fn default() -> Self {
        Self::new(20)
    }
}

/// Response containing dialogs from server.
///
/// Corresponds to TL schema variants:
/// - `messages.dialogs#3646d098`
/// - `messages.dialogsSlice#1834175b`
///
/// # Examples
///
/// ```
/// use rustgram_dialog_manager::tl_types::GetDialogsResponse;
///
/// let response = GetDialogsResponse {
///     dialogs: vec![],
///     messages: vec![],
///     chats: vec![],
///     users: vec![],
///     total_count: None,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GetDialogsResponse {
    /// List of dialogs
    pub dialogs: Vec<Dialog>,
    /// List of messages
    pub messages: Vec<Message>,
    /// List of chats
    pub chats: Vec<Chat>,
    /// List of users
    pub users: Vec<User>,
    /// Total count (for Slice variant)
    pub total_count: Option<i32>,
}

impl TlDeserialize for GetDialogsResponse {
    fn deserialize_tl(buf: &mut TlBytes) -> Result<Self, rustgram_types::TypeError> {
        // Read constructor ID to determine variant
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        match constructor_id {
            // messages.dialogs#3646d098
            0x3646d098 => {
                let dialogs = deserialize_vector_dialog(buf)?;
                let messages = deserialize_vector_message(buf)?;
                let chats = deserialize_vector_chat(buf)?;
                let users = deserialize_vector_user(buf)?;

                Ok(Self {
                    dialogs,
                    messages,
                    chats,
                    users,
                    total_count: None,
                })
            }
            // messages.dialogsSlice#1834175b
            0x1834175b => {
                let count = TlHelper::read_i32(buf)?;
                let dialogs = deserialize_vector_dialog(buf)?;
                let messages = deserialize_vector_message(buf)?;
                let chats = deserialize_vector_chat(buf)?;
                let users = deserialize_vector_user(buf)?;

                Ok(Self {
                    dialogs,
                    messages,
                    chats,
                    users,
                    total_count: Some(count),
                })
            }
            _ => Err(rustgram_types::TypeError::DeserializationError(format!(
                "Unknown constructor ID for GetDialogsResponse: 0x{:08x}",
                constructor_id
            ))),
        }
    }
}

/// Simplified dialog representation.
///
/// # Examples
///
/// ```
/// # use rustgram_dialog_manager::tl_types::{Dialog, Peer};
/// # use rustgram_types::{ChatId, DialogId};
///
/// let dialog = Dialog {
///     id: DialogId::Chat(ChatId::new(1).unwrap()),
///     peer: Peer::Chat { chat_id: ChatId::new(1).unwrap() },
///     top_message: 123,
///     unread_count: 5,
///     read_inbox_max_id: 120,
///     read_outbox_max_id: 115,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Dialog {
    /// Dialog ID
    pub id: DialogId,
    /// Peer information
    pub peer: Peer,
    /// Top message ID
    pub top_message: i32,
    /// Unread count
    pub unread_count: i32,
    /// Read inbox max ID
    pub read_inbox_max_id: i32,
    /// Read outbox max ID
    pub read_outbox_max_id: i32,
}

/// Peer information.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_manager::tl_types::Peer;
/// use rustgram_types::{ChatId, UserId};
///
/// let user_peer = Peer::User { user_id: UserId::new(123).unwrap() };
/// let chat_peer = Peer::Chat { chat_id: ChatId::new(456).unwrap() };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Peer {
    /// User peer
    User {
        /// User ID
        user_id: UserId,
    },
    /// Chat peer
    Chat {
        /// Chat ID
        chat_id: ChatId,
    },
    /// Channel peer
    Channel {
        /// Channel ID
        channel_id: u64,
    },
    /// Empty peer
    Empty,
}

/// Simplified message representation.
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    /// Message ID
    pub id: i32,
    /// Message text
    pub text: String,
    /// Sender peer
    pub from_id: Option<Peer>,
    /// Timestamp
    pub date: i32,
}

/// Simplified chat representation.
#[derive(Debug, Clone, PartialEq)]
pub struct Chat {
    /// Chat ID
    pub id: ChatId,
    /// Chat title
    pub title: String,
    /// Participant count
    pub participant_count: Option<i32>,
}

/// Simplified user representation.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    /// User ID
    pub id: UserId,
    /// First name
    pub first_name: Option<String>,
    /// Last name
    pub last_name: Option<String>,
    /// Username
    pub username: Option<String>,
}

/// Updates response for mutations (create, update).
///
/// This is a simplified placeholder for the full TL Updates type.
#[derive(Debug, Clone, PartialEq)]
pub struct Updates {
    /// List of updates
    pub updates: Vec<Update>,
    /// List of users
    pub users: Vec<User>,
    /// List of chats
    pub chats: Vec<Chat>,
    /// Date of the updates
    pub date: i32,
    /// Sequence number
    pub seq: i32,
}

/// Individual update.
#[derive(Debug, Clone, PartialEq)]
pub enum Update {
    /// New dialog created
    NewDialog,
    /// Dialog title updated
    EditDialogTitle,
    /// Dialog photo updated
    EditDialogPhoto,
    /// Unknown update
    Unknown,
}

impl rustgram_types::TlDeserialize for Updates {
    fn deserialize_tl(buf: &mut TlBytes) -> Result<Self, rustgram_types::TypeError> {
        // Read constructor ID and discard it
        let _constructor_id = TlHelper::read_constructor_id(buf)?;

        // For now, just skip the buffer and return a dummy response
        // TODO: Properly deserialize updates
        Ok(Self {
            updates: vec![],
            users: vec![],
            chats: vec![],
            date: 0,
            seq: 0,
        })
    }
}

/// Pagination token for dialog loading.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_manager::tl_types::DialogPagination;
/// use rustgram_types::InputPeer;
///
/// let pagination = DialogPagination {
///     offset_date: 1234567890,
///     offset_id: 123,
///     offset_peer: InputPeer::empty(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogPagination {
    /// Last dialog date
    pub offset_date: i32,
    /// Last dialog ID
    pub offset_id: i32,
    /// Last peer
    pub offset_peer: InputPeer,
}

impl DialogPagination {
    /// Creates a new pagination token.
    #[must_use]
    pub const fn new(offset_date: i32, offset_id: i32, offset_peer: InputPeer) -> Self {
        Self {
            offset_date,
            offset_id,
            offset_peer,
        }
    }

    /// Creates an empty pagination token (for first page).
    #[must_use]
    pub fn first_page() -> Self {
        Self {
            offset_date: 0,
            offset_id: 0,
            offset_peer: InputPeer::empty(),
        }
    }

    /// Returns `true` if this is the first page.
    #[must_use]
    pub fn is_first_page(&self) -> bool {
        self.offset_date == 0 && self.offset_id == 0
    }
}

impl Default for DialogPagination {
    fn default() -> Self {
        Self::first_page()
    }
}

/// Request to create a new chat.
///
/// Corresponds to TL schema: `messages.createChat#92ceddd4`
///
/// # Examples
///
/// ```
/// use rustgram_dialog_manager::tl_types::CreateChatRequest;
/// use rustgram_types::UserId;
///
/// let request = CreateChatRequest {
///     user_ids: vec![UserId::new(123).unwrap()],
///     title: "Test Group".to_string(),
///     ttl_period: None,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CreateChatRequest {
    /// User IDs to add to chat
    pub user_ids: Vec<UserId>,
    /// Chat title
    pub title: String,
    /// TTL period (optional)
    pub ttl_period: Option<i32>,
}

impl CreateChatRequest {
    /// TL constructor ID for messages.createChat.
    pub const CONSTRUCTOR_ID: u32 = 0x92ceddd4;

    /// Creates a new create chat request.
    ///
    /// # Arguments
    ///
    /// * `user_ids` - User IDs to add to chat
    /// * `title` - Chat title
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustgram_dialog_manager::tl_types::CreateChatRequest;
    /// # use rustgram_types::UserId;
    ///
    /// let request = CreateChatRequest::new(
    ///     vec![UserId::new(123).unwrap()],
    ///     "Test Group".to_string()
    /// );
    /// ```
    #[must_use]
    pub fn new(user_ids: Vec<UserId>, title: String) -> Self {
        Self {
            user_ids,
            title,
            ttl_period: None,
        }
    }

    /// Validates the request parameters.
    ///
    /// # Returns
    ///
    /// `Ok(())` if valid, `Err(String)` if invalid
    pub fn validate(&self) -> Result<(), String> {
        if self.user_ids.is_empty() {
            return Err("user_ids cannot be empty".to_string());
        }
        if self.title.is_empty() {
            return Err("title cannot be empty".to_string());
        }
        if self.title.len() > 128 {
            return Err("title cannot exceed 128 characters".to_string());
        }
        Ok(())
    }
}

impl TlSerialize for CreateChatRequest {
    fn serialize_tl(&self, buf: &mut BytesMut) -> Result<(), rustgram_types::TypeError> {
        // Write constructor ID
        TlHelper::write_constructor_id(buf, Self::CONSTRUCTOR_ID);

        // Write flags (0x4 if ttl_period is set)
        let flags = if self.ttl_period.is_some() { 4 } else { 0 };
        TlHelper::write_i32(buf, flags);

        // Write user_ids vector
        serialize_vector_user_id(buf, &self.user_ids)?;

        // Write title
        TlHelper::write_string(buf, &self.title);

        // Write ttl_period if present
        if let Some(ttl) = self.ttl_period {
            TlHelper::write_i32(buf, ttl);
        }

        Ok(())
    }
}

/// Request to update dialog title.
///
/// Corresponds to TL schema: `messages.editChatTitle#73783ffd`
///
/// # Examples
///
/// ```
/// use rustgram_dialog_manager::tl_types::UpdateTitleRequest;
/// use rustgram_types::ChatId;
///
/// let request = UpdateTitleRequest {
///     chat_id: ChatId::new(123).unwrap(),
///     title: "New Title".to_string(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct UpdateTitleRequest {
    /// Chat ID
    pub chat_id: ChatId,
    /// New title
    pub title: String,
}

impl UpdateTitleRequest {
    /// TL constructor ID for messages.editChatTitle.
    pub const CONSTRUCTOR_ID: u32 = 0x73783ffd;

    /// Creates a new update title request.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Chat ID
    /// * `title` - New title
    #[must_use]
    pub fn new(chat_id: ChatId, title: String) -> Self {
        Self { chat_id, title }
    }

    /// Validates the request parameters.
    pub fn validate(&self) -> Result<(), String> {
        if self.title.is_empty() {
            return Err("title cannot be empty".to_string());
        }
        if self.title.len() > 128 {
            return Err("title cannot exceed 128 characters".to_string());
        }
        Ok(())
    }
}

impl TlSerialize for UpdateTitleRequest {
    fn serialize_tl(&self, buf: &mut BytesMut) -> Result<(), rustgram_types::TypeError> {
        // Write constructor ID
        TlHelper::write_constructor_id(buf, Self::CONSTRUCTOR_ID);

        // Write flags (none)
        TlHelper::write_i32(buf, 0);

        // Write chat_id as InputPeer
        serialize_input_peer_from_chat_id(buf, self.chat_id)?;

        // Write title
        TlHelper::write_string(buf, &self.title);

        Ok(())
    }
}

/// Request to update dialog photo.
///
/// Corresponds to TL schema: `messages.editChatPhoto#35ddd674`
///
/// # Examples
///
/// ```
/// use rustgram_dialog_manager::tl_types::UpdatePhotoRequest;
/// use rustgram_types::{ChatId, InputPeer};
///
/// let request = UpdatePhotoRequest {
///     chat_id: ChatId::new(123).unwrap(),
///     photo: InputPeer::empty(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePhotoRequest {
    /// Chat ID
    pub chat_id: ChatId,
    /// Photo input peer
    pub photo: InputPeer,
}

impl UpdatePhotoRequest {
    /// TL constructor ID for messages.editChatPhoto.
    pub const CONSTRUCTOR_ID: u32 = 0x35ddd674;

    /// Creates a new update photo request.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Chat ID
    /// * `photo` - Photo input peer
    #[must_use]
    pub const fn new(chat_id: ChatId, photo: InputPeer) -> Self {
        Self { chat_id, photo }
    }
}

impl TlSerialize for UpdatePhotoRequest {
    fn serialize_tl(&self, buf: &mut BytesMut) -> Result<(), rustgram_types::TypeError> {
        // Write constructor ID
        TlHelper::write_constructor_id(buf, Self::CONSTRUCTOR_ID);

        // Write flags (none)
        TlHelper::write_i32(buf, 0);

        // Write chat_id as InputPeer
        serialize_input_peer_from_chat_id(buf, self.chat_id)?;

        // Write photo (InputPeer or InputChatPhoto)
        serialize_input_peer(buf, &self.photo)?;

        Ok(())
    }
}

// ========== Serialization Helpers ==========

/// Serializes an InputPeer to bytes.
fn serialize_input_peer(
    buf: &mut BytesMut,
    peer: &InputPeer,
) -> Result<(), rustgram_types::TypeError> {
    use rustgram_types::InputPeer::*;

    match peer {
        Empty => {
            // inputPeerEmpty#7f3b18ea
            TlHelper::write_constructor_id(buf, 0x7f3b18ea);
        }
        InputPeerSelf => {
            // inputPeerSelf#7da07ec9
            TlHelper::write_constructor_id(buf, 0x7da07ec9);
        }
        User {
            user_id,
            access_hash,
        } => {
            // inputPeerUser#dde8a54c
            TlHelper::write_constructor_id(buf, 0xdde8a54c);
            TlHelper::write_i64(buf, user_id.get());
            TlHelper::write_i64(buf, access_hash.get());
        }
        Chat(chat_id) => {
            // inputPeerChat#35a95cb9
            TlHelper::write_constructor_id(buf, 0x35a95cb9);
            TlHelper::write_i64(buf, chat_id.get());
        }
        Channel {
            channel_id,
            access_hash,
        } => {
            // inputPeerChannel#27bcbbfc
            TlHelper::write_constructor_id(buf, 0x27bcbbfc);
            TlHelper::write_i64(buf, channel_id.get());
            TlHelper::write_i64(buf, access_hash.get());
        }
        UserFromMessage {
            peer,
            msg_id,
            user_id,
        } => {
            // inputPeerUserFromMessage#a87b0a1c
            TlHelper::write_constructor_id(buf, 0xa87b0a1c);
            serialize_input_peer(buf, peer)?;
            TlHelper::write_i32(buf, *msg_id);
            TlHelper::write_i64(buf, user_id.get());
        }
        ChannelFromMessage {
            peer,
            msg_id,
            channel_id,
        } => {
            // inputPeerChannelFromMessage#bd2a0840
            TlHelper::write_constructor_id(buf, 0xbd2a0840);
            serialize_input_peer(buf, peer)?;
            TlHelper::write_i32(buf, *msg_id);
            TlHelper::write_i64(buf, channel_id.get());
        }
    }

    Ok(())
}

/// Serializes a ChatId as InputPeerChat.
fn serialize_input_peer_from_chat_id(
    buf: &mut BytesMut,
    chat_id: ChatId,
) -> Result<(), rustgram_types::TypeError> {
    // inputPeerChat#35a95cb9
    TlHelper::write_constructor_id(buf, 0x35a95cb9);
    TlHelper::write_i64(buf, chat_id.get());
    Ok(())
}

/// Serializes a vector of UserId.
fn serialize_vector_user_id(
    buf: &mut BytesMut,
    user_ids: &[UserId],
) -> Result<(), rustgram_types::TypeError> {
    // Vector constructor ID
    TlHelper::write_constructor_id(buf, 0x1cb5c415);

    // Count
    TlHelper::write_i32(buf, user_ids.len() as i32);

    // Each element as inputUser
    for user_id in user_ids {
        // inputUser#f21158c6
        TlHelper::write_constructor_id(buf, 0xf21158c6);
        TlHelper::write_i64(buf, user_id.get());
        // Access hash - using 0 for now (should come from UserManager)
        TlHelper::write_i64(buf, 0);
    }

    Ok(())
}

/// Deserializes a vector of Dialog.
fn deserialize_vector_dialog(buf: &mut TlBytes) -> Result<Vec<Dialog>, rustgram_types::TypeError> {
    let constructor_id = TlHelper::read_constructor_id(buf)?;

    if constructor_id != 0x1cb5c415 {
        // Not a vector
        return Err(rustgram_types::TypeError::DeserializationError(format!(
            "Expected vector constructor, got 0x{:08x}",
            constructor_id
        )));
    }

    let count = TlHelper::read_i32(buf)? as usize;
    let mut dialogs = Vec::with_capacity(count);

    for _ in 0..count {
        dialogs.push(deserialize_dialog(buf)?);
    }

    Ok(dialogs)
}

/// Deserializes a single Dialog.
fn deserialize_dialog(buf: &mut TlBytes) -> Result<Dialog, rustgram_types::TypeError> {
    let constructor_id = TlHelper::read_constructor_id(buf)?;

    match constructor_id {
        // dialog#e4def5db
        0xe4def5db => {
            let _flags = TlHelper::read_i32(buf)?;
            let peer = deserialize_peer(buf)?;
            let top_message = TlHelper::read_i32(buf)?;
            let _read_inbox_max_id = TlHelper::read_i32(buf)?;
            let _read_outbox_max_id = TlHelper::read_i32(buf)?;
            let unread_count = TlHelper::read_i32(buf)?;
            let _unread_mentions_count = TlHelper::read_i32(buf)?;
            let _notify_settings = TlHelper::read_constructor_id(buf)?;
            let _pts = TlHelper::read_i32(buf)?;
            let _draft = TlHelper::read_constructor_id(buf)?;

            // For now, create a simplified dialog
            // TODO: Properly extract dialog_id from peer
            let dialog_id = match &peer {
                Peer::Chat { chat_id } => DialogId::from_chat(*chat_id),
                Peer::User { user_id } => DialogId::from_user(*user_id),
                Peer::Channel { channel_id } => {
                    let channel_id_i64 = *channel_id as i64;
                    match rustgram_types::ChannelId::new(channel_id_i64) {
                        Ok(id) => DialogId::from_channel(id),
                        Err(_) => {
                            return Err(rustgram_types::TypeError::DeserializationError(
                                "Invalid channel ID".to_string(),
                            ))
                        }
                    }
                }
                Peer::Empty => {
                    return Err(rustgram_types::TypeError::DeserializationError(
                        "Empty peer in dialog".to_string(),
                    ))
                }
            };

            Ok(Dialog {
                id: dialog_id,
                peer,
                top_message,
                unread_count,
                read_inbox_max_id: 0,
                read_outbox_max_id: 0,
            })
        }
        _ => Err(rustgram_types::TypeError::DeserializationError(format!(
            "Unknown dialog constructor: 0x{:08x}",
            constructor_id
        ))),
    }
}

/// Deserializes a Peer.
fn deserialize_peer(buf: &mut TlBytes) -> Result<Peer, rustgram_types::TypeError> {
    let constructor_id = TlHelper::read_constructor_id(buf)?;

    match constructor_id {
        // peerUser#9db1bc6d
        0x9db1bc6d => {
            let user_id_val = TlHelper::read_i64(buf)?;
            let user_id = UserId::new(user_id_val)?;
            Ok(Peer::User { user_id })
        }
        // peerChat#db99c3d4
        0xdb99c3d4 => {
            let chat_id_val = TlHelper::read_i64(buf)?;
            let chat_id = ChatId::new(chat_id_val)?;
            Ok(Peer::Chat { chat_id })
        }
        // peerChannel#bcde67a5
        0xbcde67a5 => {
            let channel_id = TlHelper::read_i64(buf)?;
            Ok(Peer::Channel {
                channel_id: channel_id as u64,
            })
        }
        // peerEmpty#9ed4a7b
        0x9ed4a7b => Ok(Peer::Empty),
        _ => Err(rustgram_types::TypeError::DeserializationError(format!(
            "Unknown peer constructor: 0x{:08x}",
            constructor_id
        ))),
    }
}

/// Deserializes a vector of Message (simplified).
fn deserialize_vector_message(
    buf: &mut TlBytes,
) -> Result<Vec<Message>, rustgram_types::TypeError> {
    let constructor_id = TlHelper::read_constructor_id(buf)?;

    if constructor_id != 0x1cb5c415 {
        return Ok(Vec::new()); // Skip for now
    }

    let count = TlHelper::read_i32(buf)? as usize;

    // Skip message content for now - just advance buffer
    for _ in 0..count {
        let _msg_constructor = TlHelper::read_constructor_id(buf)?;
        // Skip remaining message data (simplified)
        // TODO: Properly deserialize messages
    }

    Ok(Vec::new())
}

/// Deserializes a vector of Chat (simplified).
fn deserialize_vector_chat(buf: &mut TlBytes) -> Result<Vec<Chat>, rustgram_types::TypeError> {
    let constructor_id = TlHelper::read_constructor_id(buf)?;

    if constructor_id != 0x1cb5c415 {
        return Ok(Vec::new()); // Skip for now
    }

    let count = TlHelper::read_i32(buf)? as usize;

    // Skip chat content for now - just advance buffer
    for _ in 0..count {
        let _chat_constructor = TlHelper::read_constructor_id(buf)?;
        // Skip remaining chat data (simplified)
        // TODO: Properly deserialize chats
    }

    Ok(Vec::new())
}

/// Deserializes a vector of User (simplified).
fn deserialize_vector_user(buf: &mut TlBytes) -> Result<Vec<User>, rustgram_types::TypeError> {
    let constructor_id = TlHelper::read_constructor_id(buf)?;

    if constructor_id != 0x1cb5c415 {
        return Ok(Vec::new()); // Skip for now
    }

    let count = TlHelper::read_i32(buf)? as usize;

    // Skip user content for now - just advance buffer
    for _ in 0..count {
        let _user_constructor = TlHelper::read_constructor_id(buf)?;
        // Skip remaining user data (simplified)
        // TODO: Properly deserialize users
    }

    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_dialogs_request_new() {
        let request = GetDialogsRequest::new(50);

        assert_eq!(request.offset_date, 0);
        assert_eq!(request.offset_id, 0);
        assert_eq!(request.limit, 50);
        assert_eq!(request.hash, 0);
    }

    #[test]
    fn test_get_dialogs_request_with_pagination() {
        let offset_peer = InputPeer::empty();
        let request = GetDialogsRequest::with_pagination(1234567890, 123, offset_peer, 50);

        assert_eq!(request.offset_date, 1234567890);
        assert_eq!(request.offset_id, 123);
        assert_eq!(request.limit, 50);
    }

    #[test]
    fn test_get_dialogs_request_default() {
        let request = GetDialogsRequest::default();

        assert_eq!(request.limit, 20);
    }

    #[test]
    fn test_get_dialogs_request_validate() {
        let valid_request = GetDialogsRequest::new(50);
        assert!(valid_request.validate().is_ok());

        // Create invalid request directly to test validation
        let invalid_request = GetDialogsRequest {
            offset_date: 0,
            offset_id: 0,
            offset_peer: InputPeer::empty(),
            limit: 0,
            hash: 0,
        };
        assert!(invalid_request.validate().is_err());

        let invalid_request = GetDialogsRequest {
            offset_date: 0,
            offset_id: 0,
            offset_peer: InputPeer::empty(),
            limit: 101,
            hash: 0,
        };
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_get_dialogs_request_serialize() {
        let request = GetDialogsRequest::new(50);

        let mut buf = BytesMut::new();
        let result = request.serialize_tl(&mut buf);

        assert!(result.is_ok());
        assert!(buf.len() > 0);

        // Verify constructor ID
        assert_eq!(&buf[0..4], &[0x4f, 0xcb, 0xf4, 0xa0]);
    }

    #[test]
    fn test_dialog_pagination_new() {
        let offset_peer = InputPeer::empty();
        let pagination = DialogPagination::new(1234567890, 123, offset_peer);

        assert_eq!(pagination.offset_date, 1234567890);
        assert_eq!(pagination.offset_id, 123);
    }

    #[test]
    fn test_dialog_pagination_first_page() {
        let pagination = DialogPagination::first_page();

        assert!(pagination.is_first_page());
        assert_eq!(pagination.offset_date, 0);
        assert_eq!(pagination.offset_id, 0);
    }

    #[test]
    fn test_dialog_pagination_default() {
        let pagination = DialogPagination::default();

        assert!(pagination.is_first_page());
    }

    #[test]
    fn test_create_chat_request_new() {
        let user_ids = vec![UserId::new(123).unwrap()];
        let request = CreateChatRequest::new(user_ids.clone(), "Test Group".to_string());

        assert_eq!(request.user_ids, user_ids);
        assert_eq!(request.title, "Test Group");
        assert!(request.ttl_period.is_none());
    }

    #[test]
    fn test_create_chat_request_validate() {
        let valid_request =
            CreateChatRequest::new(vec![UserId::new(123).unwrap()], "Test Group".to_string());
        assert!(valid_request.validate().is_ok());

        let invalid_request = CreateChatRequest::new(vec![], "Test".to_string());
        assert!(invalid_request.validate().is_err());

        let invalid_request =
            CreateChatRequest::new(vec![UserId::new(123).unwrap()], "".to_string());
        assert!(invalid_request.validate().is_err());

        let invalid_request =
            CreateChatRequest::new(vec![UserId::new(123).unwrap()], "a".repeat(129));
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_create_chat_request_serialize() {
        let user_ids = vec![UserId::new(123).unwrap()];
        let request = CreateChatRequest::new(user_ids, "Test Group".to_string());

        let mut buf = BytesMut::new();
        let result = request.serialize_tl(&mut buf);

        assert!(result.is_ok());
        assert!(buf.len() > 0);

        // Verify constructor ID
        assert_eq!(&buf[0..4], &[0xd4, 0xdd, 0xce, 0x92]);
    }

    #[test]
    fn test_update_title_request_new() {
        let chat_id = ChatId::new(123).unwrap();
        let request = UpdateTitleRequest::new(chat_id, "New Title".to_string());

        assert_eq!(request.chat_id, chat_id);
        assert_eq!(request.title, "New Title");
    }

    #[test]
    fn test_update_title_request_validate() {
        let chat_id = ChatId::new(123).unwrap();
        let valid_request = UpdateTitleRequest::new(chat_id, "Valid Title".to_string());
        assert!(valid_request.validate().is_ok());

        let invalid_request = UpdateTitleRequest::new(chat_id, "".to_string());
        assert!(invalid_request.validate().is_err());

        let invalid_request = UpdateTitleRequest::new(chat_id, "a".repeat(129));
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_update_title_request_serialize() {
        let chat_id = ChatId::new(123).unwrap();
        let request = UpdateTitleRequest::new(chat_id, "New Title".to_string());

        let mut buf = BytesMut::new();
        let result = request.serialize_tl(&mut buf);

        assert!(result.is_ok());
        assert!(buf.len() > 0);

        // Verify constructor ID
        assert_eq!(&buf[0..4], &[0xfd, 0x3f, 0x78, 0x73]);
    }

    #[test]
    fn test_update_photo_request_new() {
        let chat_id = ChatId::new(123).unwrap();
        let photo = InputPeer::empty();
        let request = UpdatePhotoRequest::new(chat_id, photo.clone());

        assert_eq!(request.chat_id, chat_id);
        assert_eq!(request.photo, photo);
    }

    #[test]
    fn test_constants() {
        // Verified from telegram_api.tl
        assert_eq!(GetDialogsRequest::CONSTRUCTOR_ID, 0xa0f4cb4f);
        assert_eq!(CreateChatRequest::CONSTRUCTOR_ID, 0x92ceddd4);
        assert_eq!(UpdateTitleRequest::CONSTRUCTOR_ID, 0x73783ffd);
        assert_eq!(UpdatePhotoRequest::CONSTRUCTOR_ID, 0x35ddd674);
    }

    #[test]
    fn test_get_dialogs_response_empty() {
        let response = GetDialogsResponse {
            dialogs: vec![],
            messages: vec![],
            chats: vec![],
            users: vec![],
            total_count: Some(0),
        };

        assert_eq!(response.dialogs.len(), 0);
        assert_eq!(response.total_count, Some(0));
    }

    #[test]
    fn test_dialog_new() {
        let dialog_id = DialogId::Chat(ChatId::new(1).unwrap());
        let peer = Peer::Chat {
            chat_id: ChatId::new(1).unwrap(),
        };

        let dialog = Dialog {
            id: dialog_id,
            peer: peer.clone(),
            top_message: 123,
            unread_count: 5,
            read_inbox_max_id: 120,
            read_outbox_max_id: 115,
        };

        assert_eq!(dialog.id, dialog_id);
        assert_eq!(dialog.top_message, 123);
        assert_eq!(dialog.unread_count, 5);
    }

    #[test]
    fn test_peer_variants() {
        let user_peer = Peer::User {
            user_id: UserId::new(123).unwrap(),
        };
        let chat_peer = Peer::Chat {
            chat_id: ChatId::new(456).unwrap(),
        };
        let channel_peer = Peer::Channel { channel_id: 789 };
        let empty_peer = Peer::Empty;

        assert!(matches!(user_peer, Peer::User { .. }));
        assert!(matches!(chat_peer, Peer::Chat { .. }));
        assert!(matches!(channel_peer, Peer::Channel { .. }));
        assert!(matches!(empty_peer, Peer::Empty));
    }

    #[test]
    fn test_serialize_input_peer_empty() {
        let peer = InputPeer::empty();
        let mut buf = BytesMut::new();

        let result = serialize_input_peer(&mut buf, &peer);

        assert!(result.is_ok());
        // inputPeerEmpty#7f3b18ea (little-endian)
        assert_eq!(&buf[0..4], &[0xea, 0x18, 0x3b, 0x7f]);
    }

    #[test]
    fn test_serialize_input_peer_chat() {
        let chat_id = ChatId::new(123).unwrap();
        let peer = InputPeer::Chat(chat_id);
        let mut buf = BytesMut::new();

        let result = serialize_input_peer(&mut buf, &peer);

        assert!(result.is_ok());
        // inputPeerChat#35a95cb9
        assert_eq!(&buf[0..4], &[0xb9, 0x5c, 0xa9, 0x35]);
    }
}
