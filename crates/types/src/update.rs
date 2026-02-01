//! Update types for Telegram.
//!
//! Updates are real-time events from Telegram such as new messages,
//! user status changes, etc.

use crate::ids::{MessageId, UserId};
use std::fmt;

/// Base update type - all updates share common properties.
///
/// Updates from the TL schema include:
/// - updateNewMessage
/// - updateUserStatus
/// - updateUserName
/// - updateNewChannelMessage
/// - etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Update {
    /// The update type identifier (constructor ID).
    pub update_type: UpdateType,
    /// The PTS (permanent timestamp) for this update.
    pub pts: Option<i32>,
    /// The PTS count for this update.
    pub pts_count: Option<i32>,
}

impl Update {
    /// Creates a new update with the specified type.
    #[inline]
    pub fn new(update_type: UpdateType) -> Self {
        Self {
            update_type,
            pts: None,
            pts_count: None,
        }
    }

    /// Sets the PTS value for this update.
    #[inline]
    pub fn with_pts(mut self, pts: i32) -> Self {
        self.pts = Some(pts);
        self
    }

    /// Sets the PTS count for this update.
    #[inline]
    pub fn with_pts_count(mut self, pts_count: i32) -> Self {
        self.pts_count = Some(pts_count);
        self
    }
}

impl fmt::Display for Update {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Update({})", self.update_type)
    }
}

/// Specific update type variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateType {
    /// New message (updateNewMessage).
    NewMessage(NewMessageUpdate),
    /// Message ID was assigned (updateMessageID).
    MessageIdAssigned(MessageIdAssignedUpdate),
    /// Messages were deleted (updateDeleteMessages).
    DeleteMessages(DeleteMessagesUpdate),
    /// User typing status (updateUserTyping).
    UserTyping(UserTypingUpdate),
    /// Chat user typing status (updateChatUserTyping).
    ChatUserTyping(ChatUserTypingUpdate),
    /// User status changed (updateUserStatus).
    UserStatus(UserStatusUpdate),
    /// User name changed (updateUserName).
    UserName(UserNameUpdate),
    /// New channel message (updateNewChannelMessage).
    NewChannelMessage(NewChannelMessageUpdate),
    /// Channel messages deleted (updateDeleteChannelMessages).
    DeleteChannelMessages(DeleteChannelMessagesUpdate),
    /// New authorization (updateNewAuthorization).
    NewAuthorization(NewAuthorizationUpdate),
    /// DC options changed (updateDcOptions).
    DcOptions,
    /// Notification settings changed (updateNotifySettings).
    NotifySettings,
    /// Config updated (updateConfig).
    Config,
    /// Pinned dialog (updateDialogPinned).
    DialogPinned,
    /// Pinned dialogs changed (updatePinnedDialogs).
    PinnedDialogs,
    /// Unknown update type.
    Unknown {
        /// Constructor ID.
        constructor_id: u32,
    },
}

impl fmt::Display for UpdateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NewMessage(_) => write!(f, "NewMessage"),
            Self::MessageIdAssigned(_) => write!(f, "MessageIdAssigned"),
            Self::DeleteMessages(_) => write!(f, "DeleteMessages"),
            Self::UserTyping(_) => write!(f, "UserTyping"),
            Self::ChatUserTyping(_) => write!(f, "ChatUserTyping"),
            Self::UserStatus(_) => write!(f, "UserStatus"),
            Self::UserName(_) => write!(f, "UserName"),
            Self::NewChannelMessage(_) => write!(f, "NewChannelMessage"),
            Self::DeleteChannelMessages(_) => write!(f, "DeleteChannelMessages"),
            Self::NewAuthorization(_) => write!(f, "NewAuthorization"),
            Self::DcOptions => write!(f, "DcOptions"),
            Self::NotifySettings => write!(f, "NotifySettings"),
            Self::Config => write!(f, "Config"),
            Self::DialogPinned => write!(f, "DialogPinned"),
            Self::PinnedDialogs => write!(f, "PinnedDialogs"),
            Self::Unknown { constructor_id } => {
                write!(f, "Unknown(0x{:08x})", constructor_id)
            }
        }
    }
}

/// New message update (updateNewMessage).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewMessageUpdate {
    /// The message ID.
    pub message_id: MessageId,
    /// The user ID of the sender (if available).
    pub sender_id: Option<UserId>,
    /// Whether the message is outgoing.
    pub is_outgoing: bool,
    /// Whether the message was mentioned.
    pub is_mentioned: bool,
    /// Whether the media is unread.
    pub is_media_unread: bool,
    /// Whether to send silently.
    pub is_silent: bool,
    /// Whether this is a post.
    pub is_post: bool,
    /// Message date (Unix timestamp).
    pub date: i32,
    /// The message content (as a placeholder for now).
    pub message_data: String,
}

impl NewMessageUpdate {
    /// Creates a new new message update.
    #[inline]
    pub fn new(message_id: MessageId) -> Self {
        Self {
            message_id,
            sender_id: None,
            is_outgoing: false,
            is_mentioned: false,
            is_media_unread: false,
            is_silent: false,
            is_post: false,
            date: 0,
            message_data: String::new(),
        }
    }
}

/// Message ID assigned update (updateMessageID).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageIdAssignedUpdate {
    /// The local message ID.
    pub local_id: i32,
    /// The new server-assigned message ID.
    pub server_id: MessageId,
    /// The random ID for this message.
    pub random_id: i64,
}

impl MessageIdAssignedUpdate {
    /// Creates a new message ID assigned update.
    #[inline]
    pub const fn new(local_id: i32, server_id: MessageId, random_id: i64) -> Self {
        Self {
            local_id,
            server_id,
            random_id,
        }
    }
}

/// Delete messages update (updateDeleteMessages).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteMessagesUpdate {
    /// IDs of deleted messages.
    pub message_ids: Vec<i32>,
    /// Whether the messages were permanently deleted.
    pub is_permanent: bool,
}

impl DeleteMessagesUpdate {
    /// Creates a new delete messages update.
    #[inline]
    pub fn new(message_ids: Vec<i32>) -> Self {
        Self {
            message_ids,
            is_permanent: false,
        }
    }
}

/// User typing update (updateUserTyping).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserTypingUpdate {
    /// User ID.
    pub user_id: UserId,
    /// Typing action.
    pub action: SendMessageAction,
}

impl UserTypingUpdate {
    /// Creates a new user typing update.
    #[inline]
    pub const fn new(user_id: UserId, action: SendMessageAction) -> Self {
        Self { user_id, action }
    }
}

/// Chat user typing update (updateChatUserTyping).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChatUserTypingUpdate {
    /// Chat ID.
    pub chat_id: i64,
    /// User ID.
    pub user_id: UserId,
    /// Typing action.
    pub action: SendMessageAction,
}

impl ChatUserTypingUpdate {
    /// Creates a new chat user typing update.
    #[inline]
    pub const fn new(chat_id: i64, user_id: UserId, action: SendMessageAction) -> Self {
        Self {
            chat_id,
            user_id,
            action,
        }
    }
}

/// User status update (updateUserStatus).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserStatusUpdate {
    /// User ID.
    pub user_id: UserId,
    /// New user status.
    pub status: UserStatus,
}

impl UserStatusUpdate {
    /// Creates a new user status update.
    #[inline]
    pub const fn new(user_id: UserId, status: UserStatus) -> Self {
        Self { user_id, status }
    }
}

/// User name update (updateUserName).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserNameUpdate {
    /// User ID.
    pub user_id: UserId,
    /// New first name.
    pub first_name: Option<String>,
    /// New last name.
    pub last_name: Option<String>,
}

impl UserNameUpdate {
    /// Creates a new user name update.
    #[inline]
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            first_name: None,
            last_name: None,
        }
    }
}

/// New channel message update (updateNewChannelMessage).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewChannelMessageUpdate {
    /// The message ID.
    pub message_id: MessageId,
    /// The channel ID.
    pub channel_id: i64,
    /// Whether the message is outgoing.
    pub is_outgoing: bool,
    /// Message date (Unix timestamp).
    pub date: i32,
    /// The message content (as a placeholder for now).
    pub message_data: String,
}

impl NewChannelMessageUpdate {
    /// Creates a new channel message update.
    #[inline]
    pub fn new(message_id: MessageId, channel_id: i64) -> Self {
        Self {
            message_id,
            channel_id,
            is_outgoing: false,
            date: 0,
            message_data: String::new(),
        }
    }
}

/// Delete channel messages update (updateDeleteChannelMessages).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteChannelMessagesUpdate {
    /// Channel ID.
    pub channel_id: i64,
    /// IDs of deleted messages.
    pub message_ids: Vec<i32>,
}

impl DeleteChannelMessagesUpdate {
    /// Creates a new delete channel messages update.
    #[inline]
    pub fn new(channel_id: i64, message_ids: Vec<i32>) -> Self {
        Self {
            channel_id,
            message_ids,
        }
    }
}

/// New authorization update (updateNewAuthorization).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewAuthorizationUpdate {
    /// Authorization hash.
    pub hash: i64,
    /// Device information.
    pub device: String,
    /// Location information.
    pub location: String,
    /// Date when the authorization was created.
    pub date: Option<i32>,
    /// Whether the authorization is unconfirmed.
    pub is_unconfirmed: bool,
}

impl NewAuthorizationUpdate {
    /// Creates a new new authorization update.
    #[inline]
    pub fn new(hash: i64) -> Self {
        Self {
            hash,
            device: String::new(),
            location: String::new(),
            date: None,
            is_unconfirmed: false,
        }
    }
}

/// User status in Telegram.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    /// User is online (expires at the given timestamp).
    Online(i32),
    /// User is offline (was last online at the given timestamp).
    Offline(i32),
    /// User was recently online.
    Recently,
    /// User was last seen within the last week.
    LastWeek,
    /// User was last seen within the last month.
    LastMonth,
    /// Empty status (user has hidden their last seen time).
    Empty,
}

impl UserStatus {
    /// Returns true if the user is currently online.
    #[inline]
    pub fn is_online(&self) -> bool {
        matches!(self, Self::Online(_))
    }
}

/// Send message action (for typing indicators).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendMessageAction {
    /// Typing a text message.
    Typing,
    /// Cancelled action.
    Cancel,
    /// Recording a voice message.
    RecordVoice,
    /// Uploading a photo.
    UploadPhoto,
    /// Uploading a video.
    UploadVideo,
    /// Recording a video.
    RecordVideo,
    /// Uploading a document.
    UploadDocument,
    /// Choosing a geographic location.
    GeoLocation,
    /// Choosing a contact.
    ChooseContact,
    /// Playing a game.
    PlayGame,
    /// Recording a round video.
    RecordRound,
    /// Uploading a round video.
    UploadRound,
    /// Speaking in a voice chat.
    SpeakingInVoiceChat,
    /// Listening in a voice chat.
    ListeningInVoiceChat,
    /// Speaking in group voice chat.
    SpeakingInGroupVoiceChat,
    /// Choosing a sticker.
    ChooseSticker,
    /// Emulating a reaction.
    EmulateReaction,
}

/// Updates state - contains current state information for updates.
///
/// From the TL schema: updates.state#a56c2a3e
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdatesState {
    /// Permanent timestamp.
    pub pts: i32,
    /// QTS (query timestamp).
    pub qts: i32,
    /// Date.
    pub date: i32,
    /// Sequence number.
    pub seq: i32,
    /// Unread count.
    pub unread_count: i32,
}

impl UpdatesState {
    /// Creates a new updates state.
    #[inline]
    pub const fn new() -> Self {
        Self {
            pts: 0,
            qts: 0,
            date: 0,
            seq: 0,
            unread_count: 0,
        }
    }
}

impl Default for UpdatesState {
    fn default() -> Self {
        Self::new()
    }
}

/// Updates difference - represents changes since last state.
///
/// From the TL schema:
/// - updates.differenceEmpty
/// - updates.difference
/// - updates.differenceSlice
/// - updates.differenceTooLong
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdatesDifference {
    /// No updates (empty difference).
    Empty {
        /// The date.
        date: i32,
        /// The sequence number.
        seq: i32,
    },
    /// Normal difference.
    Difference {
        /// New messages.
        new_messages: Vec<Update>,
        /// Other updates.
        other_updates: Vec<Update>,
        /// The new state.
        state: UpdatesState,
    },
    /// Partial difference (slice).
    DifferenceSlice {
        /// New messages.
        new_messages: Vec<Update>,
        /// Other updates.
        other_updates: Vec<Update>,
        /// The intermediate state.
        intermediate_state: UpdatesState,
    },
    /// Too many updates, need to call again.
    TooLong {
        /// The current PTS.
        pts: i32,
    },
}

impl UpdatesDifference {
    /// Returns true if there are no updates.
    #[inline]
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty { .. })
    }
}

/// Combined updates - a batch of updates with users and chats.
///
/// From the TL schema:
/// - updates
/// - updatesCombined
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatesCombined {
    /// The updates.
    pub updates: Vec<Update>,
    /// Users mentioned in the updates.
    pub users: Vec<i64>,
    /// Chats mentioned in the updates.
    pub chats: Vec<i64>,
    /// Date.
    pub date: i32,
    /// Sequence start.
    pub seq_start: i32,
    /// Sequence end.
    pub seq: i32,
}

impl UpdatesCombined {
    /// Creates a new updates combined.
    #[inline]
    pub fn new() -> Self {
        Self {
            updates: Vec::new(),
            users: Vec::new(),
            chats: Vec::new(),
            date: 0,
            seq_start: 0,
            seq: 0,
        }
    }
}

impl Default for UpdatesCombined {
    fn default() -> Self {
        Self::new()
    }
}

/// Short update format - for single updates with minimal data.
///
/// From the TL schema:
/// - updateShort
/// - updateShortMessage
/// - updateShortChatMessage
/// - updateShortSentMessage
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateShort {
    /// Generic short update (updateShort).
    Update {
        /// The update.
        update: Box<Update>,
        /// Date.
        date: i32,
    },
    /// Short message from user (updateShortMessage).
    ShortMessage {
        /// Message ID.
        id: i32,
        /// User ID.
        user_id: UserId,
        /// Message text.
        message: String,
        /// PTS.
        pts: i32,
        /// PTS count.
        pts_count: i32,
        /// Date.
        date: i32,
        /// Whether outgoing.
        is_outgoing: bool,
        /// Whether mentioned.
        is_mentioned: bool,
        /// Whether media is unread.
        is_media_unread: bool,
    },
    /// Short message from chat (updateShortChatMessage).
    ShortChatMessage {
        /// Message ID.
        id: i32,
        /// Sender ID.
        from_id: i64,
        /// Chat ID.
        chat_id: i64,
        /// Message text.
        message: String,
        /// PTS.
        pts: i32,
        /// PTS count.
        pts_count: i32,
        /// Date.
        date: i32,
        /// Whether outgoing.
        is_outgoing: bool,
        /// Whether mentioned.
        is_mentioned: bool,
        /// Whether media is unread.
        is_media_unread: bool,
    },
    /// Short sent message (updateShortSentMessage).
    ShortSentMessage {
        /// Message ID.
        id: i32,
        /// PTS.
        pts: i32,
        /// PTS count.
        pts_count: i32,
        /// Date.
        date: i32,
    },
}

impl UpdateShort {
    /// Returns the date of this short update.
    #[inline]
    pub fn date(&self) -> i32 {
        match self {
            Self::Update { date, .. } => *date,
            Self::ShortMessage { date, .. } => *date,
            Self::ShortChatMessage { date, .. } => *date,
            Self::ShortSentMessage { date, .. } => *date,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_message_update() {
        let msg_id = MessageId(100);
        let update = NewMessageUpdate::new(msg_id);
        assert_eq!(update.message_id.get(), 100);
    }

    #[test]
    fn test_user_status() {
        let online = UserStatus::Online(12345);
        assert!(online.is_online());

        let offline = UserStatus::Offline(12345);
        assert!(!offline.is_online());
    }

    #[test]
    fn test_update_type_display() {
        let update_type = UpdateType::NewMessage(NewMessageUpdate::new(MessageId(100)));
        assert_eq!(format!("{}", update_type), "NewMessage");
    }

    #[test]
    fn test_updates_state() {
        let state = UpdatesState::new();
        assert_eq!(state.pts, 0);
        assert_eq!(state.seq, 0);
    }
}
