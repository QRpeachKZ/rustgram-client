//! Peer types for Telegram.
//!
//! Peers represent entities that can participate in conversations:
//! users, chats, channels, etc.

use crate::access::AccessHash;
use crate::ids::{ChannelId, ChatId, DialogId, UserId};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::hash::Hash;

/// Peer type - identifies a conversation participant.
///
/// From the TL schema:
/// - peerUser#59511722 user_id:long = Peer
/// - peerChat#36c6019a chat_id:long = Peer
/// - peerChannel#a2a5371e channel_id:long = Peer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Peer {
    /// User peer (private chat).
    User(UserId),
    /// Chat peer (basic group).
    Chat(ChatId),
    /// Channel peer (megagroup or broadcast channel).
    Channel(ChannelId),
}

impl Peer {
    /// Constructor ID for peerUser.
    pub const USER_CONSTRUCTOR: u32 = 0x59511722;

    /// Constructor ID for peerChat.
    pub const CHAT_CONSTRUCTOR: u32 = 0x36c6019a;

    /// Constructor ID for peerChannel.
    pub const CHANNEL_CONSTRUCTOR: u32 = 0xa2a5371e;

    /// Creates a user peer.
    #[inline]
    pub const fn user(user_id: UserId) -> Self {
        Self::User(user_id)
    }

    /// Creates a chat peer.
    #[inline]
    pub const fn chat(chat_id: ChatId) -> Self {
        Self::Chat(chat_id)
    }

    /// Creates a channel peer.
    #[inline]
    pub const fn channel(channel_id: ChannelId) -> Self {
        Self::Channel(channel_id)
    }

    /// Returns the constructor ID for this peer type.
    #[inline]
    pub const fn constructor_id(self) -> u32 {
        match self {
            Self::User(_) => Self::USER_CONSTRUCTOR,
            Self::Chat(_) => Self::CHAT_CONSTRUCTOR,
            Self::Channel(_) => Self::CHANNEL_CONSTRUCTOR,
        }
    }

    /// Gets the UserId if this is a user peer.
    #[inline]
    pub const fn get_user_id(self) -> Option<UserId> {
        match self {
            Self::User(id) => Some(id),
            _ => None,
        }
    }

    /// Gets the ChatId if this is a chat peer.
    #[inline]
    pub const fn get_chat_id(self) -> Option<ChatId> {
        match self {
            Self::Chat(id) => Some(id),
            _ => None,
        }
    }

    /// Gets the ChannelId if this is a channel peer.
    #[inline]
    pub const fn get_channel_id(self) -> Option<ChannelId> {
        match self {
            Self::Channel(id) => Some(id),
            _ => None,
        }
    }

    /// Converts this peer to a DialogId.
    #[inline]
    pub fn to_dialog_id(self) -> DialogId {
        match self {
            Self::User(id) => DialogId::from_user(id),
            Self::Chat(id) => DialogId::from_chat(id),
            Self::Channel(id) => DialogId::from_channel(id),
        }
    }

    /// Checks if this is a user peer.
    #[inline]
    pub const fn is_user(self) -> bool {
        matches!(self, Self::User(_))
    }

    /// Checks if this is a chat peer.
    #[inline]
    pub const fn is_chat(self) -> bool {
        matches!(self, Self::Chat(_))
    }

    /// Checks if this is a channel peer.
    #[inline]
    pub const fn is_channel(self) -> bool {
        matches!(self, Self::Channel(_))
    }
}

impl Default for Peer {
    fn default() -> Self {
        Self::User(UserId::default())
    }
}

impl fmt::Display for Peer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User(id) => write!(f, "PeerUser({})", id.get()),
            Self::Chat(id) => write!(f, "PeerChat({})", id.get()),
            Self::Channel(id) => write!(f, "PeerChannel({})", id.get()),
        }
    }
}

impl From<UserId> for Peer {
    fn from(id: UserId) -> Self {
        Self::User(id)
    }
}

impl From<ChatId> for Peer {
    fn from(id: ChatId) -> Self {
        Self::Chat(id)
    }
}

impl From<ChannelId> for Peer {
    fn from(id: ChannelId) -> Self {
        Self::Channel(id)
    }
}

impl Serialize for Peer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::User(id) => (0u8, id.get()).serialize(serializer),
            Self::Chat(id) => (1u8, id.get()).serialize(serializer),
            Self::Channel(id) => (2u8, id.get()).serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Peer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PeerVisitor;

        impl<'de> serde::de::Visitor<'de> for PeerVisitor {
            type Value = Peer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a tuple containing peer type and ID")
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
                    0 => Peer::User(UserId(id)),
                    1 => Peer::Chat(ChatId(id)),
                    2 => Peer::Channel(ChannelId(id)),
                    _ => {
                        return Err(serde::de::Error::custom(format!(
                            "invalid peer type tag: {tag}"
                        )))
                    }
                })
            }
        }

        deserializer.deserialize_any(PeerVisitor)
    }
}

/// Input peer - used for API calls to specify a target peer.
///
/// From the TL schema:
/// - inputPeerEmpty#7f3b18ea = InputPeer
/// - inputPeerSelf#7da07ec9 = InputPeer
/// - inputPeerChat#35a95cb9 chat_id:long = InputPeer
/// - inputPeerUser#dde8a54c user_id:long access_hash:long = InputPeer
/// - inputPeerChannel#27bcbbfc channel_id:long access_hash:long = InputPeer
/// - inputPeerUserFromMessage#a87b0a1c peer:InputPeer msg_id:int user_id:long = InputPeer
/// - inputPeerChannelFromMessage#bd2a0840 peer:InputPeer msg_id:int channel_id:long = InputPeer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputPeer {
    /// Empty peer (placeholder).
    Empty,
    /// Self peer (current user).
    InputPeerSelf,
    /// User peer with access hash.
    User {
        /// User ID.
        user_id: UserId,
        /// Access hash for authentication.
        access_hash: AccessHash,
    },
    /// Chat peer (no access hash needed).
    Chat(ChatId),
    /// Channel peer with access hash.
    Channel {
        /// Channel ID.
        channel_id: ChannelId,
        /// Access hash for authentication.
        access_hash: AccessHash,
    },
    /// User peer from a message (for embedded user references).
    UserFromMessage {
        /// Source peer.
        peer: Box<InputPeer>,
        /// Message ID containing the user reference.
        msg_id: i32,
        /// User ID.
        user_id: UserId,
    },
    /// Channel peer from a message (for embedded channel references).
    ChannelFromMessage {
        /// Source peer.
        peer: Box<InputPeer>,
        /// Message ID containing the channel reference.
        msg_id: i32,
        /// Channel ID.
        channel_id: ChannelId,
    },
}

impl InputPeer {
    /// Constructor ID for inputPeerEmpty.
    pub const EMPTY_CONSTRUCTOR: u32 = 0x7f3b18ea;

    /// Constructor ID for inputPeerSelf.
    pub const SELF_CONSTRUCTOR: u32 = 0x7da07ec9;

    /// Constructor ID for inputPeerChat.
    pub const CHAT_CONSTRUCTOR: u32 = 0x35a95cb9;

    /// Constructor ID for inputPeerUser.
    pub const USER_CONSTRUCTOR: u32 = 0xdde8a54c;

    /// Constructor ID for inputPeerChannel.
    pub const CHANNEL_CONSTRUCTOR: u32 = 0x27bcbbfc;

    /// Creates an empty input peer.
    #[inline]
    pub fn empty() -> Self {
        Self::Empty
    }

    /// Creates a self input peer.
    #[inline]
    pub fn self_() -> Self {
        Self::InputPeerSelf
    }

    /// Creates a user input peer.
    #[inline]
    pub fn user(user_id: UserId, access_hash: AccessHash) -> Self {
        Self::User {
            user_id,
            access_hash,
        }
    }

    /// Creates a chat input peer.
    #[inline]
    pub fn chat(chat_id: ChatId) -> Self {
        Self::Chat(chat_id)
    }

    /// Creates a channel input peer.
    #[inline]
    pub fn channel(channel_id: ChannelId, access_hash: AccessHash) -> Self {
        Self::Channel {
            channel_id,
            access_hash,
        }
    }

    /// Returns true if this is the empty peer.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    /// Returns true if this is the self peer.
    #[inline]
    pub const fn is_self(&self) -> bool {
        matches!(self, Self::InputPeerSelf)
    }

    /// Gets the UserId if this is a user peer.
    #[inline]
    pub const fn get_user_id(&self) -> Option<UserId> {
        match self {
            Self::User { user_id, .. } => Some(*user_id),
            Self::UserFromMessage { user_id, .. } => Some(*user_id),
            _ => None,
        }
    }

    /// Gets the ChatId if this is a chat peer.
    #[inline]
    pub const fn get_chat_id(&self) -> Option<ChatId> {
        match self {
            Self::Chat(id) => Some(*id),
            _ => None,
        }
    }

    /// Gets the ChannelId if this is a channel peer.
    #[inline]
    pub const fn get_channel_id(&self) -> Option<ChannelId> {
        match self {
            Self::Channel { channel_id, .. } => Some(*channel_id),
            Self::ChannelFromMessage { channel_id, .. } => Some(*channel_id),
            _ => None,
        }
    }
}

impl Default for InputPeer {
    fn default() -> Self {
        Self::Empty
    }
}

impl fmt::Display for InputPeer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "InputPeerEmpty"),
            Self::InputPeerSelf => write!(f, "InputPeerSelf"),
            Self::User { user_id, .. } => write!(f, "InputPeerUser({})", user_id.get()),
            Self::Chat(id) => write!(f, "InputPeerChat({})", id.get()),
            Self::Channel { channel_id, .. } => {
                write!(f, "InputPeerChannel({})", channel_id.get())
            }
            Self::UserFromMessage {
                msg_id, user_id, ..
            } => write!(
                f,
                "InputPeerUserFromMessage(msg_id={}, user_id={})",
                msg_id,
                user_id.get()
            ),
            Self::ChannelFromMessage {
                msg_id, channel_id, ..
            } => write!(
                f,
                "InputPeerChannelFromMessage(msg_id={}, channel_id={})",
                msg_id,
                channel_id.get()
            ),
        }
    }
}

impl From<UserId> for InputPeer {
    fn from(id: UserId) -> Self {
        Self::User {
            user_id: id,
            access_hash: AccessHash::default(),
        }
    }
}

impl From<ChatId> for InputPeer {
    fn from(id: ChatId) -> Self {
        Self::Chat(id)
    }
}

impl From<ChannelId> for InputPeer {
    fn from(id: ChannelId) -> Self {
        Self::Channel {
            channel_id: id,
            access_hash: AccessHash::default(),
        }
    }
}

impl Serialize for InputPeer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Empty => (0u8,).serialize(serializer),
            Self::InputPeerSelf => (1u8,).serialize(serializer),
            Self::User {
                user_id,
                access_hash,
            } => (2u8, user_id.get(), access_hash.get()).serialize(serializer),
            Self::Chat(id) => (3u8, id.get()).serialize(serializer),
            Self::Channel {
                channel_id,
                access_hash,
            } => (4u8, channel_id.get(), access_hash.get()).serialize(serializer),
            Self::UserFromMessage {
                peer: _,
                msg_id,
                user_id,
            } => (5u8, msg_id, user_id.get()).serialize(serializer),
            Self::ChannelFromMessage {
                peer: _,
                msg_id,
                channel_id,
            } => (6u8, msg_id, channel_id.get()).serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for InputPeer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InputPeerVisitor;

        impl<'de> serde::de::Visitor<'de> for InputPeerVisitor {
            type Value = InputPeer;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a tagged InputPeer value")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let tag: u8 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                Ok(match tag {
                    0 => InputPeer::Empty,
                    1 => InputPeer::InputPeerSelf,
                    2 => {
                        let user_id = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                        let access_hash = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                        InputPeer::User {
                            user_id: UserId(user_id),
                            access_hash: AccessHash(access_hash),
                        }
                    }
                    3 => {
                        let chat_id = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                        InputPeer::Chat(ChatId(chat_id))
                    }
                    4 => {
                        let channel_id = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                        let access_hash = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                        InputPeer::Channel {
                            channel_id: ChannelId(channel_id),
                            access_hash: AccessHash(access_hash),
                        }
                    }
                    5 => {
                        let msg_id = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                        let user_id = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                        InputPeer::UserFromMessage {
                            peer: Box::new(InputPeer::Empty),
                            msg_id,
                            user_id: UserId(user_id),
                        }
                    }
                    6 => {
                        let msg_id = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                        let channel_id = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                        InputPeer::ChannelFromMessage {
                            peer: Box::new(InputPeer::Empty),
                            msg_id,
                            channel_id: ChannelId(channel_id),
                        }
                    }
                    _ => {
                        return Err(serde::de::Error::custom(format!(
                            "invalid InputPeer tag: {tag}"
                        )))
                    }
                })
            }
        }

        deserializer.deserialize_any(InputPeerVisitor)
    }
}

/// Dialog peer - used for specifying dialogs in operations.
///
/// From the TL schema:
/// - inputDialogPeer#f4bbb515 peer:InputPeer = InputDialogPeer
/// - inputDialogPeerEmpty#3266e21bd = InputDialogPeer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DialogPeer {
    /// Empty dialog peer.
    Empty,
    /// Dialog peer with peer specification.
    Peer(InputPeer),
}

impl DialogPeer {
    /// Creates an empty dialog peer.
    #[inline]
    pub fn empty() -> Self {
        Self::Empty
    }

    /// Creates a dialog peer from an input peer.
    #[inline]
    pub fn new(peer: InputPeer) -> Self {
        Self::Peer(peer)
    }
}

impl Default for DialogPeer {
    fn default() -> Self {
        Self::Empty
    }
}

impl fmt::Display for DialogPeer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "DialogPeerEmpty"),
            Self::Peer(peer) => write!(f, "DialogPeer({})", peer),
        }
    }
}

impl From<InputPeer> for DialogPeer {
    fn from(peer: InputPeer) -> Self {
        Self::Peer(peer)
    }
}

impl From<UserId> for DialogPeer {
    fn from(id: UserId) -> Self {
        Self::Peer(InputPeer::User {
            user_id: id,
            access_hash: AccessHash::default(),
        })
    }
}

impl From<ChatId> for DialogPeer {
    fn from(id: ChatId) -> Self {
        Self::Peer(InputPeer::Chat(id))
    }
}

impl From<ChannelId> for DialogPeer {
    fn from(id: ChannelId) -> Self {
        Self::Peer(InputPeer::Channel {
            channel_id: id,
            access_hash: AccessHash::default(),
        })
    }
}

impl Serialize for DialogPeer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Empty => (0u8,).serialize(serializer),
            Self::Peer(peer) => (1u8, peer).serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for DialogPeer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DialogPeerVisitor;

        impl<'de> serde::de::Visitor<'de> for DialogPeerVisitor {
            type Value = DialogPeer;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a DialogPeer")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let tag: u8 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                Ok(match tag {
                    0 => DialogPeer::Empty,
                    1 => {
                        let peer = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                        DialogPeer::Peer(peer)
                    }
                    _ => {
                        return Err(serde::de::Error::custom(format!(
                            "invalid DialogPeer tag: {tag}"
                        )))
                    }
                })
            }
        }

        deserializer.deserialize_any(DialogPeerVisitor)
    }
}

/// Notify peer - used for notification settings.
///
/// From the TL schema:
/// - notifyPeer#9fd40bd8 peer:Peer = NotifyPeer
/// - notifyUsers#b4c83b4c = NotifyPeer
/// - notifyChats#c007cec3 = NotifyPeer
/// - notifyBroadcasts#b8331b21 = NotifyPeer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotifyPeer {
    /// Notifications from a specific peer.
    Peer(Peer),
    /// Notifications from all users.
    Users,
    /// Notifications from all chats.
    Chats,
    /// Notifications from all broadcasts (channels).
    Broadcasts,
}

impl NotifyPeer {
    /// Creates a notify peer from a peer.
    #[inline]
    pub const fn peer(peer: Peer) -> Self {
        Self::Peer(peer)
    }

    /// Creates a users notify peer.
    #[inline]
    pub const fn users() -> Self {
        Self::Users
    }

    /// Creates a chats notify peer.
    #[inline]
    pub const fn chats() -> Self {
        Self::Chats
    }

    /// Creates a broadcasts notify peer.
    #[inline]
    pub const fn broadcasts() -> Self {
        Self::Broadcasts
    }
}

impl Default for NotifyPeer {
    fn default() -> Self {
        Self::Users
    }
}

impl fmt::Display for NotifyPeer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Peer(peer) => write!(f, "NotifyPeer({})", peer),
            Self::Users => write!(f, "NotifyUsers"),
            Self::Chats => write!(f, "NotifyChats"),
            Self::Broadcasts => write!(f, "NotifyBroadcasts"),
        }
    }
}

impl From<Peer> for NotifyPeer {
    fn from(peer: Peer) -> Self {
        Self::Peer(peer)
    }
}

impl From<UserId> for NotifyPeer {
    fn from(id: UserId) -> Self {
        Self::Peer(Peer::User(id))
    }
}

impl Serialize for NotifyPeer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Peer(peer) => (0u8, peer).serialize(serializer),
            Self::Users => (1u8,).serialize(serializer),
            Self::Chats => (2u8,).serialize(serializer),
            Self::Broadcasts => (3u8,).serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for NotifyPeer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotifyPeerVisitor;

        impl<'de> serde::de::Visitor<'de> for NotifyPeerVisitor {
            type Value = NotifyPeer;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a NotifyPeer")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let tag: u8 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                Ok(match tag {
                    0 => {
                        let peer = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                        NotifyPeer::Peer(peer)
                    }
                    1 => NotifyPeer::Users,
                    2 => NotifyPeer::Chats,
                    3 => NotifyPeer::Broadcasts,
                    _ => {
                        return Err(serde::de::Error::custom(format!(
                            "invalid NotifyPeer tag: {tag}"
                        )))
                    }
                })
            }
        }

        deserializer.deserialize_any(NotifyPeerVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_constructors() {
        let user_id = UserId(123);
        let peer = Peer::user(user_id);
        assert!(peer.is_user());
        assert_eq!(peer.constructor_id(), Peer::USER_CONSTRUCTOR);
    }

    #[test]
    fn test_peer_to_dialog_id() {
        let user_id = UserId(123);
        let peer = Peer::user(user_id);
        let dialog_id = peer.to_dialog_id();
        assert_eq!(dialog_id.get_user_id(), Some(user_id));
    }

    #[test]
    fn test_input_peer() {
        let user_id = UserId(123);
        let access_hash = AccessHash::new(456);
        let input_peer = InputPeer::user(user_id, access_hash);
        assert_eq!(input_peer.get_user_id(), Some(user_id));
    }

    #[test]
    fn test_dialog_peer() {
        let user_id = UserId(123);
        let dialog_peer = DialogPeer::from(user_id);
        assert!(matches!(dialog_peer, DialogPeer::Peer(_)));
    }
}
