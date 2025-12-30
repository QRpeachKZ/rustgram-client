//! Telegram identifier types.
//!
//! This module defines the various ID types used in Telegram:
//! - UserId: Identifies users
//! - ChatId: Identifies basic group chats
//! - ChannelId: Identifies megagroups and channels
//! - MessageId: Identifies messages with special bit encoding
//! - SecretChatId: Identifies secret chats
//! - DialogId: Unified identifier for all dialog types

use crate::error::{InvalidIdError, TypeResult};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::hash::{Hash, Hasher};

/// User identifier.
///
/// Valid user IDs are positive integers up to (1 << 40) - 1.
///
/// # Example
/// ```rust
/// use rustgram_types::UserId;
///
/// let user_id = UserId::new(12345678)?;
/// assert!(user_id.is_valid());
/// assert_eq!(user_id.get(), 12345678);
/// # Ok::<(), rustgram_types::TypeError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UserId(pub i64);

impl UserId {
    /// Maximum valid user ID: (1 << 40) - 1
    pub const MAX_USER_ID: i64 = (1 << 40) - 1;

    /// Creates a new UserId.
    ///
    /// Returns an error if the ID is not valid.
    #[inline]
    pub fn new(id: i64) -> TypeResult<Self> {
        let user_id = Self(id);
        if user_id.is_valid() {
            Ok(user_id)
        } else {
            Err(InvalidIdError::user_id(format!(
                "must be in range 1..={}, got {id}",
                Self::MAX_USER_ID
            ))
            .into())
        }
    }

    /// Returns the inner ID value.
    #[inline]
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Checks if this is a valid user ID.
    #[inline]
    pub const fn is_valid(self) -> bool {
        self.0 > 0 && self.0 <= Self::MAX_USER_ID
    }

    /// Creates a UserId from an i32.
    #[inline]
    pub fn from_i32(id: i32) -> Self {
        Self(id as i64)
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self(0)
    }
}

impl Hash for UserId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "user {}", self.0)
    }
}

impl From<i32> for UserId {
    fn from(id: i32) -> Self {
        Self(id as i64)
    }
}

impl TryFrom<i64> for UserId {
    type Error = InvalidIdError;

    fn try_from(id: i64) -> Result<Self, Self::Error> {
        UserId::new(id).map_err(|e| match e {
            crate::error::TypeError::InvalidId(e) => e,
            _ => InvalidIdError::user_id("unknown error"),
        })
    }
}

impl From<UserId> for i64 {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        i64::deserialize(deserializer)
            .map(|id| UserId(id))
            .map_err(|e| serde::de::Error::custom(format!("invalid UserId: {e}")))
    }
}

/// Chat identifier (for basic group chats).
///
/// Valid chat IDs are positive integers up to 999999999999.
///
/// Basic groups are small group chats (up to ~200 members) that don't have
/// a separate channel ID. They're distinguished from megagroups and channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChatId(pub i64);

impl ChatId {
    /// Maximum valid chat ID: 999999999999
    pub const MAX_CHAT_ID: i64 = 999999999999;

    /// Creates a new ChatId.
    ///
    /// Returns an error if the ID is not valid.
    #[inline]
    pub fn new(id: i64) -> TypeResult<Self> {
        let chat_id = Self(id);
        if chat_id.is_valid() {
            Ok(chat_id)
        } else {
            Err(InvalidIdError::chat_id(format!(
                "must be in range 1..={}, got {id}",
                Self::MAX_CHAT_ID
            ))
            .into())
        }
    }

    /// Returns the inner ID value.
    #[inline]
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Checks if this is a valid chat ID.
    #[inline]
    pub const fn is_valid(self) -> bool {
        self.0 > 0 && self.0 <= Self::MAX_CHAT_ID
    }
}

impl Default for ChatId {
    fn default() -> Self {
        Self(0)
    }
}

impl Hash for ChatId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for ChatId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "basic group {}", self.0)
    }
}

impl TryFrom<i64> for ChatId {
    type Error = InvalidIdError;

    fn try_from(id: i64) -> Result<Self, Self::Error> {
        ChatId::new(id).map_err(|e| match e {
            crate::error::TypeError::InvalidId(e) => e,
            _ => InvalidIdError::chat_id("unknown error"),
        })
    }
}

impl From<ChatId> for i64 {
    fn from(id: ChatId) -> Self {
        id.0
    }
}

impl Serialize for ChatId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ChatId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        i64::deserialize(deserializer)
            .map(|id| ChatId(id))
            .map_err(|e| serde::de::Error::custom(format!("invalid ChatId: {e}")))
    }
}

/// Channel identifier (for megagroups and channels).
///
/// Channel IDs use a special encoding. Valid channel IDs are in ranges:
/// - 1000000000000 - (1000000000000 - (1 << 31)): Regular channels/megagroups
/// - (1000000000000 + (1 << 31) + 1) - 3000000000000: Monoforum channels
///
/// Megagroups are large group chats that can have unlimited members.
/// Channels are broadcast-only channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChannelId(pub i64);

impl ChannelId {
    /// Maximum regular channel ID: 1000000000000 - (1 << 31)
    pub const MAX_CHANNEL_ID: i64 = 1000000000000 - (1 << 31);

    /// Minimum monoforum channel ID: 1000000000000 + (1 << 31) + 1
    pub const MIN_MONOFORUM_CHANNEL_ID: i64 = 1000000000000 + (1 << 31) + 1;

    /// Maximum monoforum channel ID: 3000000000000
    pub const MAX_MONOFORUM_CHANNEL_ID: i64 = 3000000000000;

    /// Creates a new ChannelId.
    ///
    /// Returns an error if the ID is not valid.
    #[inline]
    pub fn new(id: i64) -> TypeResult<Self> {
        let channel_id = Self(id);
        if channel_id.is_valid() {
            Ok(channel_id)
        } else {
            Err(
                InvalidIdError::channel_id(format!("must be in valid channel range, got {id}"))
                    .into(),
            )
        }
    }

    /// Returns the inner ID value.
    #[inline]
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Checks if this is a valid channel ID.
    #[inline]
    pub const fn is_valid(self) -> bool {
        (self.0 > 0 && self.0 < Self::MAX_CHANNEL_ID)
            || (self.0 >= Self::MIN_MONOFORUM_CHANNEL_ID && self.0 < Self::MAX_MONOFORUM_CHANNEL_ID)
    }

    /// Checks if this is a monoforum channel.
    #[inline]
    pub const fn is_monoforum(self) -> bool {
        self.0 >= Self::MIN_MONOFORUM_CHANNEL_ID && self.0 < Self::MAX_MONOFORUM_CHANNEL_ID
    }
}

impl Default for ChannelId {
    fn default() -> Self {
        Self(0)
    }
}

impl Hash for ChannelId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for ChannelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "supergroup {}", self.0)
    }
}

impl TryFrom<i64> for ChannelId {
    type Error = InvalidIdError;

    fn try_from(id: i64) -> Result<Self, Self::Error> {
        ChannelId::new(id).map_err(|e| match e {
            crate::error::TypeError::InvalidId(e) => e,
            _ => InvalidIdError::channel_id("unknown error"),
        })
    }
}

impl From<ChannelId> for i64 {
    fn from(id: ChannelId) -> Self {
        id.0
    }
}

impl Serialize for ChannelId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ChannelId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        i64::deserialize(deserializer)
            .map(|id| ChannelId(id))
            .map_err(|e| serde::de::Error::custom(format!("invalid ChannelId: {e}")))
    }
}

/// Message identifier with special bit encoding.
///
/// Message IDs use a special encoding to distinguish between different types:
/// - Server messages: Have type bits = 0
/// - Yet unsent messages: Have type bits = 1
/// - Local messages: Have type bits = 2
/// - Scheduled messages: Have scheduled bit = 4
///
/// The encoding layout is:
/// - Ordinary: |---31 server_id---|---17 local_id---|1|---2 type---|
/// - Scheduled: |---30 send_date---|---18 server_id---|1|---2 type---|
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageId(pub i64);

impl MessageId {
    /// Bit shift for server message ID.
    pub const SERVER_ID_SHIFT: i64 = 20;

    /// Mask for the short type field (lower 2 bits).
    const SHORT_TYPE_MASK: i64 = (1 << 2) - 1;

    /// Mask for the full type field (lower 20 bits).
    const FULL_TYPE_MASK: i64 = (1 << Self::SERVER_ID_SHIFT) - 1;

    /// Bit indicating a scheduled message.
    const SCHEDULED_MASK: i64 = 4;

    /// Type value for yet unsent messages.
    const TYPE_YET_UNSENT: i64 = 1;

    /// Type value for local messages.
    const TYPE_LOCAL: i64 = 2;

    /// Creates a new MessageId.
    ///
    /// Returns an error if the ID is not valid.
    #[inline]
    pub fn new(id: i64) -> TypeResult<Self> {
        let msg_id = Self(id);
        if msg_id.is_valid() || msg_id.is_scheduled() {
            Ok(msg_id)
        } else {
            Err(InvalidIdError::message_id(format!("invalid message ID: {id}")).into())
        }
    }

    /// Creates a server MessageId from a server message ID.
    #[inline]
    pub const fn from_server_id(server_id: i32) -> Self {
        Self((server_id as i64) << Self::SERVER_ID_SHIFT)
    }

    /// Returns the inner ID value.
    #[inline]
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Returns the type of this message.
    #[inline]
    pub const fn get_type(self) -> MessageType {
        if self.is_scheduled() {
            MessageType::Scheduled
        } else {
            match self.0 & Self::SHORT_TYPE_MASK {
                Self::TYPE_YET_UNSENT => MessageType::YetUnsent,
                Self::TYPE_LOCAL => MessageType::Local,
                _ => MessageType::Server,
            }
        }
    }

    /// Checks if this is a valid (non-scheduled) message ID.
    #[inline]
    pub const fn is_valid(self) -> bool {
        // Server messages have zero in the type field
        (self.0 & Self::FULL_TYPE_MASK == 0) && (self.0 > 0)
    }

    /// Checks if this is a valid scheduled message ID.
    #[inline]
    pub const fn is_scheduled(self) -> bool {
        (self.0 & Self::SCHEDULED_MASK) != 0
    }

    /// Checks if this is a server message (assigned by server).
    #[inline]
    pub const fn is_server(self) -> bool {
        (self.0 & Self::FULL_TYPE_MASK == 0) && (self.0 > 0)
    }

    /// Checks if this is a yet-unsent message.
    #[inline]
    pub const fn is_yet_unsent(self) -> bool {
        (self.0 & Self::SHORT_TYPE_MASK) == Self::TYPE_YET_UNSENT
    }

    /// Checks if this is a local message.
    #[inline]
    pub const fn is_local(self) -> bool {
        (self.0 & Self::SHORT_TYPE_MASK) == Self::TYPE_LOCAL
    }

    /// Checks if this is a scheduled server message.
    #[inline]
    pub const fn is_scheduled_server(self) -> bool {
        self.is_scheduled() && (self.0 & Self::SHORT_TYPE_MASK == 0)
    }

    /// Gets the server message ID part (for server/scheduled messages).
    #[inline]
    pub const fn get_server_id(self) -> i32 {
        ((self.0 >> Self::SERVER_ID_SHIFT) & 0x7FFFFFFF) as i32
    }

    /// Returns the smallest valid message ID.
    #[inline]
    pub const fn min() -> Self {
        Self(Self::TYPE_YET_UNSENT)
    }

    /// Returns the largest valid message ID.
    #[inline]
    pub const fn max() -> Self {
        Self((i32::MAX as i64) << Self::SERVER_ID_SHIFT)
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self(0)
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "message {}", self.0)
    }
}

impl TryFrom<i64> for MessageId {
    type Error = InvalidIdError;

    fn try_from(id: i64) -> Result<Self, Self::Error> {
        MessageId::new(id).map_err(|e| match e {
            crate::error::TypeError::InvalidId(e) => e,
            _ => InvalidIdError::message_id("unknown error"),
        })
    }
}

impl From<MessageId> for i64 {
    fn from(id: MessageId) -> Self {
        id.0
    }
}

impl PartialOrd for MessageId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MessageId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare server IDs only, ignoring type bits
        let self_server = self.0 & !MessageId::FULL_TYPE_MASK;
        let other_server = other.0 & !MessageId::FULL_TYPE_MASK;
        self_server.cmp(&other_server)
    }
}

impl Serialize for MessageId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MessageId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        i64::deserialize(deserializer)
            .map(|id| MessageId(id))
            .map_err(|e| serde::de::Error::custom(format!("invalid MessageId: {e}")))
    }
}

/// Message type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageType {
    /// None/invalid type.
    None,
    /// Server-assigned message.
    Server,
    /// Yet unsent message (queued but not sent).
    YetUnsent,
    /// Local message (not yet synced to server).
    Local,
    /// Scheduled message.
    Scheduled,
}

/// Secret chat identifier.
///
/// Secret chats are end-to-end encrypted chats between two users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecretChatId(i32);

impl SecretChatId {
    /// Creates a new SecretChatId.
    ///
    /// Returns an error if the ID is not valid.
    #[inline]
    pub fn new(id: i32) -> TypeResult<Self> {
        let chat_id = Self(id);
        if chat_id.is_valid() {
            Ok(chat_id)
        } else {
            Err(InvalidIdError::secret_chat_id("must be non-zero").into())
        }
    }

    /// Creates a SecretChatId without validation.
    ///
    /// # Safety
    /// The caller must ensure the ID is valid.
    #[inline]
    pub const unsafe fn new_unchecked(id: i32) -> Self {
        Self(id)
    }

    /// Returns the inner ID value.
    #[inline]
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Checks if this is a valid secret chat ID (non-zero).
    #[inline]
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

impl Default for SecretChatId {
    fn default() -> Self {
        Self(0)
    }
}

impl Hash for SecretChatId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for SecretChatId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "secret chat {}", self.0)
    }
}

impl TryFrom<i32> for SecretChatId {
    type Error = InvalidIdError;

    fn try_from(id: i32) -> Result<Self, Self::Error> {
        SecretChatId::new(id).map_err(|e| match e {
            crate::error::TypeError::InvalidId(e) => e,
            _ => InvalidIdError::secret_chat_id("unknown error"),
        })
    }
}

impl From<SecretChatId> for i32 {
    fn from(id: SecretChatId) -> Self {
        id.0
    }
}

impl Serialize for SecretChatId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SecretChatId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        i32::deserialize(deserializer)
            .map(|id| SecretChatId(id))
            .map_err(|e| serde::de::Error::custom(format!("invalid SecretChatId: {e}")))
    }
}

/// Dialog identifier.
///
/// Dialogs represent any chat conversation in Telegram.
/// DialogId uses a special encoding to distinguish between different types:
/// - User dialogs: positive ID up to MAX_USER_ID
/// - Chat dialogs: positive ID up to MAX_CHAT_ID
/// - Channel dialogs: encoded as -1000000000000 + channel_id
/// - Secret chat dialogs: encoded as -2000000000000 + secret_chat_id
///
/// This encoding allows all dialog types to be stored in a single i64 field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogId {
    /// User dialog.
    User(UserId),
    /// Basic group chat dialog.
    Chat(ChatId),
    /// Channel/megagroup dialog.
    Channel(ChannelId),
    /// Secret chat dialog.
    SecretChat(SecretChatId),
}

impl DialogId {
    /// Base value for encoding channel dialog IDs.
    const ZERO_CHANNEL_ID: i64 = -1000000000000;

    /// Base value for encoding secret chat dialog IDs.
    const ZERO_SECRET_CHAT_ID: i64 = -2000000000000;

    /// Creates a DialogId from a UserId.
    #[inline]
    pub fn from_user(user_id: UserId) -> Self {
        Self::User(user_id)
    }

    /// Creates a DialogId from a ChatId.
    #[inline]
    pub fn from_chat(chat_id: ChatId) -> Self {
        Self::Chat(chat_id)
    }

    /// Creates a DialogId from a ChannelId.
    #[inline]
    pub fn from_channel(channel_id: ChannelId) -> Self {
        Self::Channel(channel_id)
    }

    /// Creates a DialogId from a SecretChatId.
    #[inline]
    pub fn from_secret_chat(secret_chat_id: SecretChatId) -> Self {
        Self::SecretChat(secret_chat_id)
    }

    /// Returns the type of this dialog.
    #[inline]
    pub const fn get_type(&self) -> DialogType {
        match self {
            Self::User(_) => DialogType::User,
            Self::Chat(_) => DialogType::Chat,
            Self::Channel(_) => DialogType::Channel,
            Self::SecretChat(_) => DialogType::SecretChat,
        }
    }

    /// Checks if this is a valid dialog ID.
    #[inline]
    pub fn is_valid(&self) -> bool {
        match self {
            Self::User(id) => id.is_valid(),
            Self::Chat(id) => id.is_valid(),
            Self::Channel(id) => id.is_valid(),
            Self::SecretChat(id) => id.is_valid(),
        }
    }

    /// Gets the UserId if this is a user dialog.
    #[inline]
    pub const fn get_user_id(&self) -> Option<UserId> {
        match self {
            Self::User(id) => Some(*id),
            _ => None,
        }
    }

    /// Gets the ChatId if this is a chat dialog.
    #[inline]
    pub const fn get_chat_id(&self) -> Option<ChatId> {
        match self {
            Self::Chat(id) => Some(*id),
            _ => None,
        }
    }

    /// Gets the ChannelId if this is a channel dialog.
    #[inline]
    pub const fn get_channel_id(&self) -> Option<ChannelId> {
        match self {
            Self::Channel(id) => Some(*id),
            _ => None,
        }
    }

    /// Gets the SecretChatId if this is a secret chat dialog.
    #[inline]
    pub const fn get_secret_chat_id(&self) -> Option<SecretChatId> {
        match self {
            Self::SecretChat(id) => Some(*id),
            _ => None,
        }
    }

    /// Creates a DialogId from an encoded i64 value.
    ///
    /// The encoding scheme (following TDLib convention):
    /// - 0 < value <= UserId::MAX_USER_ID: User dialog
    /// - UserId::MAX_USER_ID < value <= ChatId::MAX_CHAT_ID: Chat dialog
    /// - ZERO_CHANNEL_ID < value < 0: Channel dialog (negative offset encoding)
    /// - value < ZERO_SECRET_CHAT_ID: Secret chat dialog (negative offset encoding)
    ///
    /// Note: The constants are negative, so comparisons use > instead of <.
    pub fn from_encoded(value: i64) -> TypeResult<Self> {
        if value < Self::ZERO_SECRET_CHAT_ID {
            // Secret chat (most negative, e.g., -2000000000100)
            let secret_chat_id = (value - Self::ZERO_SECRET_CHAT_ID) as i32;
            let id = SecretChatId::new(secret_chat_id)?;
            Ok(Self::SecretChat(id))
        } else if value < Self::ZERO_CHANNEL_ID {
            // Between ZERO_SECRET_CHAT_ID and ZERO_CHANNEL_ID: invalid range
            Err(InvalidIdError::dialog_id(format!("invalid encoded value (gap): {value}")).into())
        } else if value < 0 {
            // Between ZERO_CHANNEL_ID and 0: Channel (negative encoding, e.g., -999000000000)
            let channel_id = value - Self::ZERO_CHANNEL_ID;
            let id = ChannelId::new(channel_id)?;
            Ok(Self::Channel(id))
        } else if value > 0 {
            // Positive values: user or chat
            if value <= UserId::MAX_USER_ID {
                Ok(Self::User(UserId(value)))
            } else if value <= ChatId::MAX_CHAT_ID {
                Ok(Self::Chat(ChatId(value)))
            } else {
                Err(InvalidIdError::dialog_id(format!("invalid encoded value: {value}")).into())
            }
        } else {
            Err(InvalidIdError::dialog_id(format!("invalid encoded value: {value}")).into())
        }
    }

    /// Encodes this dialog ID to an i64 value.
    ///
    /// Uses the same encoding scheme as from_encoded (TDLib style).
    pub fn to_encoded(&self) -> i64 {
        match self {
            Self::User(id) => id.get(),
            Self::Chat(id) => id.get(),
            Self::Channel(id) => Self::ZERO_CHANNEL_ID + id.get(),
            Self::SecretChat(id) => Self::ZERO_SECRET_CHAT_ID + id.get() as i64,
        }
    }
}

impl Default for DialogId {
    fn default() -> Self {
        Self::User(UserId::default())
    }
}

impl fmt::Display for DialogId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User(id) => write!(f, "{id}"),
            Self::Chat(id) => write!(f, "{id}"),
            Self::Channel(id) => write!(f, "{id}"),
            Self::SecretChat(id) => write!(f, "{id}"),
        }
    }
}

impl From<UserId> for DialogId {
    fn from(id: UserId) -> Self {
        Self::User(id)
    }
}

impl From<ChatId> for DialogId {
    fn from(id: ChatId) -> Self {
        Self::Chat(id)
    }
}

impl From<ChannelId> for DialogId {
    fn from(id: ChannelId) -> Self {
        Self::Channel(id)
    }
}

impl From<SecretChatId> for DialogId {
    fn from(id: SecretChatId) -> Self {
        Self::SecretChat(id)
    }
}

impl Serialize for DialogId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as tagged enum
        match self {
            Self::User(id) => (0u8, id.get()).serialize(serializer),
            Self::Chat(id) => (1u8, id.get()).serialize(serializer),
            Self::Channel(id) => (2u8, id.get()).serialize(serializer),
            Self::SecretChat(id) => (3u8, id.get()).serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for DialogId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DialogIdVisitor;

        impl<'de> serde::de::Visitor<'de> for DialogIdVisitor {
            type Value = DialogId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a tuple containing dialog type and ID")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let tag: u8 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let id: i64 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;

                Ok(match tag {
                    0 => DialogId::User(UserId(id)),
                    1 => DialogId::Chat(ChatId(id)),
                    2 => DialogId::Channel(ChannelId(id)),
                    3 => DialogId::SecretChat(SecretChatId(id as i32)),
                    _ => {
                        return Err(serde::de::Error::custom(format!(
                            "invalid dialog type tag: {tag}"
                        )))
                    }
                })
            }
        }

        deserializer.deserialize_any(DialogIdVisitor)
    }
}

/// Dialog type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogType {
    /// User dialog (private chat).
    User,
    /// Basic group chat.
    Chat,
    /// Channel or megagroup.
    Channel,
    /// Secret chat.
    SecretChat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id() {
        let id = UserId::new(12345678).unwrap();
        assert!(id.is_valid());
        assert_eq!(id.get(), 12345678);
    }

    #[test]
    fn test_chat_id() {
        let id = ChatId::new(1234567890).unwrap();
        assert!(id.is_valid());
        assert_eq!(id.get(), 1234567890);
    }

    #[test]
    fn test_channel_id() {
        // Valid channel ID (less than MAX_CHANNEL_ID)
        let id = ChannelId::new(100000000000).unwrap();
        assert!(id.is_valid());
        assert!(!id.is_monoforum());
    }

    #[test]
    fn test_message_id() {
        let server_msg = MessageId::from_server_id(100);
        assert!(server_msg.is_server());
        assert_eq!(server_msg.get_server_id(), 100);
        assert_eq!(server_msg.get_type(), MessageType::Server);
    }

    #[test]
    fn test_dialog_id_encoding() {
        // User IDs are in range 0..MAX_USER_ID, and decode as user first
        let user_id = UserId(12345);
        let dialog_id = DialogId::from_user(user_id);
        let encoded = dialog_id.to_encoded();
        assert_eq!(encoded, 12345);

        let decoded = DialogId::from_encoded(encoded).unwrap();
        assert_eq!(decoded, dialog_id);
    }

    #[test]
    fn test_channel_dialog_encoding() {
        // Channel IDs are encoded with negative offset
        // The encoded value must be < ZERO_CHANNEL_ID (-1000000000000)
        // So channel_id must be < 2^31 = 2147483648
        let channel_id = ChannelId::new(1000000000i64).unwrap(); // 1 billion < 2^31
        let dialog_id = DialogId::from_channel(channel_id);
        let encoded = dialog_id.to_encoded();
        // Channel IDs are encoded as ZERO_CHANNEL_ID + channel_id
        assert_eq!(encoded, -1000000000000 + 1000000000);

        let decoded = DialogId::from_encoded(encoded).unwrap();
        assert_eq!(decoded, dialog_id);
    }
}
