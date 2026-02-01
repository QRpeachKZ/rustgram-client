// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Peer types for Telegram.
//!
//! This module provides TL deserialization for peer types.
//!
//! # TL Schema
//!
//! ```text
//! peerUser#59511722 user_id:long = Peer;
//! peerChat#36c6019a chat_id:long = Peer;
//! peerChannel#a2a5371e channel_id:long = Peer;
//!
//! inputPeerEmpty#7f3b18ea = InputPeer;
//! inputPeerSelf#7da07ec9 = InputPeer;
//! inputPeerChat#35a95cb9 chat_id:long = InputPeer;
//! inputPeerUser#dde8a54c user_id:long access_hash:long = InputPeer;
//! inputPeerChannel#27bcbbfc channel_id:long access_hash:long = InputPeer;
//! inputPeerUserFromMessage#a87b0a1c peer:InputPeer msg_id:int user_id:long = InputPeer;
//! inputPeerChannelFromMessage#bd2a0840 peer:InputPeer msg_id:int channel_id:long = InputPeer;
//! ```

use rustgram_types::tl::{Bytes, TlDeserialize, TlHelper};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Peer type - identifies a conversation participant.
///
/// # TL Schema
///
/// ```text
/// peerUser#59511722 user_id:long = Peer;
/// peerChat#36c6019a chat_id:long = Peer;
/// peerChannel#a2a5371e channel_id:long = Peer;
/// ```
///
/// # Example
///
/// ```
/// use rustgram_tl_core::Peer;
///
/// let user_peer = Peer::User { user_id: 12345 };
/// assert!(user_peer.is_user());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Peer {
    /// User peer (private chat).
    User {
        /// User ID.
        user_id: i64,
    },

    /// Chat peer (basic group).
    Chat {
        /// Chat ID.
        chat_id: i64,
    },

    /// Channel peer (megagroup or broadcast channel).
    Channel {
        /// Channel ID.
        channel_id: i64,
    },
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
    pub const fn user(user_id: i64) -> Self {
        Self::User { user_id }
    }

    /// Creates a chat peer.
    #[inline]
    pub const fn chat(chat_id: i64) -> Self {
        Self::Chat { chat_id }
    }

    /// Creates a channel peer.
    #[inline]
    pub const fn channel(channel_id: i64) -> Self {
        Self::Channel { channel_id }
    }

    /// Checks if this is a user peer.
    #[inline]
    pub const fn is_user(&self) -> bool {
        matches!(self, Self::User { .. })
    }

    /// Checks if this is a chat peer.
    #[inline]
    pub const fn is_chat(&self) -> bool {
        matches!(self, Self::Chat { .. })
    }

    /// Checks if this is a channel peer.
    #[inline]
    pub const fn is_channel(&self) -> bool {
        matches!(self, Self::Channel { .. })
    }

    /// Gets the user ID if this is a user peer.
    #[inline]
    pub const fn get_user_id(&self) -> Option<i64> {
        match self {
            Self::User { user_id } => Some(*user_id),
            _ => None,
        }
    }

    /// Gets the chat ID if this is a chat peer.
    #[inline]
    pub const fn get_chat_id(&self) -> Option<i64> {
        match self {
            Self::Chat { chat_id } => Some(*chat_id),
            _ => None,
        }
    }

    /// Gets the channel ID if this is a channel peer.
    #[inline]
    pub const fn get_channel_id(&self) -> Option<i64> {
        match self {
            Self::Channel { channel_id } => Some(*channel_id),
            _ => None,
        }
    }

    /// Returns the constructor ID for this peer type.
    #[inline]
    pub const fn constructor_id(&self) -> u32 {
        match self {
            Self::User { .. } => Self::USER_CONSTRUCTOR,
            Self::Chat { .. } => Self::CHAT_CONSTRUCTOR,
            Self::Channel { .. } => Self::CHANNEL_CONSTRUCTOR,
        }
    }
}

impl TlDeserialize for Peer {
    fn deserialize_tl(buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        match constructor_id {
            Self::USER_CONSTRUCTOR => {
                let user_id = TlHelper::read_i64(buf)?;
                Ok(Self::User { user_id })
            }
            Self::CHAT_CONSTRUCTOR => {
                let chat_id = TlHelper::read_i64(buf)?;
                Ok(Self::Chat { chat_id })
            }
            Self::CHANNEL_CONSTRUCTOR => {
                let channel_id = TlHelper::read_i64(buf)?;
                Ok(Self::Channel { channel_id })
            }
            _ => {
                let tl_err = crate::error::TlError::unknown_constructor(
                    vec![
                        Self::USER_CONSTRUCTOR,
                        Self::CHAT_CONSTRUCTOR,
                        Self::CHANNEL_CONSTRUCTOR,
                    ],
                    constructor_id,
                    "Peer",
                );
                Err(rustgram_types::TypeError::from(tl_err))
            }
        }
    }
}

impl fmt::Display for Peer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User { user_id } => write!(f, "PeerUser({})", user_id),
            Self::Chat { chat_id } => write!(f, "PeerChat({})", chat_id),
            Self::Channel { channel_id } => write!(f, "PeerChannel({})", channel_id),
        }
    }
}

/// Input peer - used for API calls to specify a target peer.
///
/// # TL Schema
///
/// ```text
/// inputPeerEmpty#7f3b18ea = InputPeer;
/// inputPeerSelf#7da07ec9 = InputPeer;
/// inputPeerChat#35a95cb9 chat_id:long = InputPeer;
/// inputPeerUser#dde8a54c user_id:long access_hash:long = InputPeer;
/// inputPeerChannel#27bcbbfc channel_id:long access_hash:long = InputPeer;
/// inputPeerUserFromMessage#a87b0a1c peer:InputPeer msg_id:int user_id:long = InputPeer;
/// inputPeerChannelFromMessage#bd2a0840 peer:InputPeer msg_id:int channel_id:long = InputPeer;
/// ```
///
/// # Example
///
/// ```
/// use rustgram_tl_core::InputPeer;
///
/// let empty = InputPeer::Empty;
/// assert!(empty.is_empty());
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputPeer {
    /// Empty peer (placeholder).
    #[default]
    Empty,

    /// Self peer (current user).
    InputPeerSelf,

    /// Chat peer (no access hash needed).
    Chat {
        /// Chat ID.
        chat_id: i64,
    },

    /// User peer with access hash.
    User {
        /// User ID.
        user_id: i64,
        /// Access hash for authentication.
        access_hash: i64,
    },

    /// Channel peer with access hash.
    Channel {
        /// Channel ID.
        channel_id: i64,
        /// Access hash for authentication.
        access_hash: i64,
    },

    /// User peer from a message (for embedded user references).
    UserFromMessage {
        /// Source peer.
        peer: Box<InputPeer>,
        /// Message ID containing the user reference.
        msg_id: i32,
        /// User ID.
        user_id: i64,
    },

    /// Channel peer from a message (for embedded channel references).
    ChannelFromMessage {
        /// Source peer.
        peer: Box<InputPeer>,
        /// Message ID containing the channel reference.
        msg_id: i32,
        /// Channel ID.
        channel_id: i64,
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

    /// Constructor ID for inputPeerUserFromMessage.
    pub const USER_FROM_MESSAGE_CONSTRUCTOR: u32 = 0xa87b0a1c;

    /// Constructor ID for inputPeerChannelFromMessage.
    pub const CHANNEL_FROM_MESSAGE_CONSTRUCTOR: u32 = 0xbd2a0840;

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
    pub fn user(user_id: i64, access_hash: i64) -> Self {
        Self::User {
            user_id,
            access_hash,
        }
    }

    /// Creates a chat input peer.
    #[inline]
    pub fn chat(chat_id: i64) -> Self {
        Self::Chat { chat_id }
    }

    /// Creates a channel input peer.
    #[inline]
    pub fn channel(channel_id: i64, access_hash: i64) -> Self {
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

    /// Gets the user ID if this is a user peer.
    #[inline]
    pub const fn get_user_id(&self) -> Option<i64> {
        match self {
            Self::User { user_id, .. } => Some(*user_id),
            Self::UserFromMessage { user_id, .. } => Some(*user_id),
            _ => None,
        }
    }

    /// Gets the chat ID if this is a chat peer.
    #[inline]
    pub const fn get_chat_id(&self) -> Option<i64> {
        match self {
            Self::Chat { chat_id } => Some(*chat_id),
            _ => None,
        }
    }

    /// Gets the channel ID if this is a channel peer.
    #[inline]
    pub const fn get_channel_id(&self) -> Option<i64> {
        match self {
            Self::Channel { channel_id, .. } => Some(*channel_id),
            Self::ChannelFromMessage { channel_id, .. } => Some(*channel_id),
            _ => None,
        }
    }

    /// Returns the constructor ID for this input peer type.
    pub const fn constructor_id(&self) -> u32 {
        match self {
            Self::Empty => Self::EMPTY_CONSTRUCTOR,
            Self::InputPeerSelf => Self::SELF_CONSTRUCTOR,
            Self::Chat { .. } => Self::CHAT_CONSTRUCTOR,
            Self::User { .. } => Self::USER_CONSTRUCTOR,
            Self::Channel { .. } => Self::CHANNEL_CONSTRUCTOR,
            Self::UserFromMessage { .. } => Self::USER_FROM_MESSAGE_CONSTRUCTOR,
            Self::ChannelFromMessage { .. } => Self::CHANNEL_FROM_MESSAGE_CONSTRUCTOR,
        }
    }
}

impl TlDeserialize for InputPeer {
    fn deserialize_tl(buf: &mut Bytes) -> rustgram_types::TypeResult<Self> {
        let constructor_id = TlHelper::read_constructor_id(buf)?;

        match constructor_id {
            Self::EMPTY_CONSTRUCTOR => Ok(Self::Empty),
            Self::SELF_CONSTRUCTOR => Ok(Self::InputPeerSelf),
            Self::CHAT_CONSTRUCTOR => {
                let chat_id = TlHelper::read_i64(buf)?;
                Ok(Self::Chat { chat_id })
            }
            Self::USER_CONSTRUCTOR => {
                let user_id = TlHelper::read_i64(buf)?;
                let access_hash = TlHelper::read_i64(buf)?;
                Ok(Self::User {
                    user_id,
                    access_hash,
                })
            }
            Self::CHANNEL_CONSTRUCTOR => {
                let channel_id = TlHelper::read_i64(buf)?;
                let access_hash = TlHelper::read_i64(buf)?;
                Ok(Self::Channel {
                    channel_id,
                    access_hash,
                })
            }
            Self::USER_FROM_MESSAGE_CONSTRUCTOR => {
                let peer = Box::new(Self::deserialize_tl(buf)?);
                let msg_id = TlHelper::read_i32(buf)?;
                let user_id = TlHelper::read_i64(buf)?;
                Ok(Self::UserFromMessage {
                    peer,
                    msg_id,
                    user_id,
                })
            }
            Self::CHANNEL_FROM_MESSAGE_CONSTRUCTOR => {
                let peer = Box::new(Self::deserialize_tl(buf)?);
                let msg_id = TlHelper::read_i32(buf)?;
                let channel_id = TlHelper::read_i64(buf)?;
                Ok(Self::ChannelFromMessage {
                    peer,
                    msg_id,
                    channel_id,
                })
            }
            _ => {
                let tl_err = crate::error::TlError::unknown_constructor(
                    vec![
                        Self::EMPTY_CONSTRUCTOR,
                        Self::SELF_CONSTRUCTOR,
                        Self::CHAT_CONSTRUCTOR,
                        Self::USER_CONSTRUCTOR,
                        Self::CHANNEL_CONSTRUCTOR,
                        Self::USER_FROM_MESSAGE_CONSTRUCTOR,
                        Self::CHANNEL_FROM_MESSAGE_CONSTRUCTOR,
                    ],
                    constructor_id,
                    "InputPeer",
                );
                Err(rustgram_types::TypeError::from(tl_err))
            }
        }
    }
}

impl fmt::Display for InputPeer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "InputPeerEmpty"),
            Self::InputPeerSelf => write!(f, "InputPeerSelf"),
            Self::Chat { chat_id } => write!(f, "InputPeerChat({})", chat_id),
            Self::User { user_id, .. } => write!(f, "InputPeerUser({})", user_id),
            Self::Channel { channel_id, .. } => write!(f, "InputPeerChannel({})", channel_id),
            Self::UserFromMessage {
                msg_id, user_id, ..
            } => write!(
                f,
                "InputPeerUserFromMessage(msg_id={}, user_id={})",
                msg_id, user_id
            ),
            Self::ChannelFromMessage {
                msg_id, channel_id, ..
            } => write!(
                f,
                "InputPeerChannelFromMessage(msg_id={}, channel_id={})",
                msg_id, channel_id
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_buffer(data: &[u8]) -> Bytes {
        Bytes::new(bytes::Bytes::copy_from_slice(data))
    }

    #[test]
    fn test_peer_constructors() {
        assert_eq!(Peer::USER_CONSTRUCTOR, 0x59511722);
        assert_eq!(Peer::CHAT_CONSTRUCTOR, 0x36c6019a);
        assert_eq!(Peer::CHANNEL_CONSTRUCTOR, 0xa2a5371e);
    }

    #[test]
    fn test_peer_user_deserialize() {
        // peerUser#59511722 user_id:long = Peer;
        let mut data = vec![0x22, 0x17, 0x51, 0x59]; // constructor
        data.extend_from_slice(&12345i64.to_le_bytes()); // user_id

        let mut buf = create_buffer(&data);
        let result = Peer::deserialize_tl(&mut buf).unwrap();

        assert_eq!(result, Peer::User { user_id: 12345 });
        assert!(result.is_user());
        assert_eq!(result.get_user_id(), Some(12345));
    }

    #[test]
    fn test_peer_chat_deserialize() {
        // peerChat#36c6019a chat_id:long = Peer;
        let mut data = vec![0x9a, 0x01, 0xc6, 0x36]; // constructor
        data.extend_from_slice(&67890i64.to_le_bytes()); // chat_id

        let mut buf = create_buffer(&data);
        let result = Peer::deserialize_tl(&mut buf).unwrap();

        assert_eq!(result, Peer::Chat { chat_id: 67890 });
        assert!(result.is_chat());
        assert_eq!(result.get_chat_id(), Some(67890));
    }

    #[test]
    fn test_peer_channel_deserialize() {
        // peerChannel#a2a5371e channel_id:long = Peer;
        let mut data = vec![0x1e, 0x37, 0xa5, 0xa2]; // constructor
        data.extend_from_slice(&11111i64.to_le_bytes()); // channel_id

        let mut buf = create_buffer(&data);
        let result = Peer::deserialize_tl(&mut buf).unwrap();

        assert_eq!(result, Peer::Channel { channel_id: 11111 });
        assert!(result.is_channel());
        assert_eq!(result.get_channel_id(), Some(11111));
    }

    #[test]
    fn test_peer_display() {
        let user = Peer::User { user_id: 123 };
        assert_eq!(format!("{}", user), "PeerUser(123)");

        let chat = Peer::Chat { chat_id: 456 };
        assert_eq!(format!("{}", chat), "PeerChat(456)");

        let channel = Peer::Channel { channel_id: 789 };
        assert_eq!(format!("{}", channel), "PeerChannel(789)");
    }

    #[test]
    fn test_peer_constructor_id() {
        let user = Peer::user(123);
        assert_eq!(user.constructor_id(), Peer::USER_CONSTRUCTOR);

        let chat = Peer::chat(456);
        assert_eq!(chat.constructor_id(), Peer::CHAT_CONSTRUCTOR);

        let channel = Peer::channel(789);
        assert_eq!(channel.constructor_id(), Peer::CHANNEL_CONSTRUCTOR);
    }

    #[test]
    fn test_input_peer_constructors() {
        assert_eq!(InputPeer::EMPTY_CONSTRUCTOR, 0x7f3b18ea);
        assert_eq!(InputPeer::SELF_CONSTRUCTOR, 0x7da07ec9);
        assert_eq!(InputPeer::CHAT_CONSTRUCTOR, 0x35a95cb9);
        assert_eq!(InputPeer::USER_CONSTRUCTOR, 0xdde8a54c);
        assert_eq!(InputPeer::CHANNEL_CONSTRUCTOR, 0x27bcbbfc);
        assert_eq!(InputPeer::USER_FROM_MESSAGE_CONSTRUCTOR, 0xa87b0a1c);
        assert_eq!(InputPeer::CHANNEL_FROM_MESSAGE_CONSTRUCTOR, 0xbd2a0840);
    }

    #[test]
    fn test_input_peer_empty_deserialize() {
        // inputPeerEmpty#7f3b18ea = InputPeer;
        let data = [0xea, 0x18, 0x3b, 0x7f]; // constructor

        let mut buf = create_buffer(&data);
        let result = InputPeer::deserialize_tl(&mut buf).unwrap();

        assert_eq!(result, InputPeer::Empty);
        assert!(result.is_empty());
    }

    #[test]
    fn test_input_peer_self_deserialize() {
        // inputPeerSelf#7da07ec9 = InputPeer;
        let data = [0xc9, 0x7e, 0xa0, 0x7d]; // constructor

        let mut buf = create_buffer(&data);
        let result = InputPeer::deserialize_tl(&mut buf).unwrap();

        assert_eq!(result, InputPeer::InputPeerSelf);
        assert!(result.is_self());
    }

    #[test]
    fn test_input_peer_chat_deserialize() {
        // inputPeerChat#35a95cb9 chat_id:long = InputPeer;
        let mut data = vec![0xb9, 0x5c, 0xa9, 0x35]; // constructor
        data.extend_from_slice(&999i64.to_le_bytes()); // chat_id

        let mut buf = create_buffer(&data);
        let result = InputPeer::deserialize_tl(&mut buf).unwrap();

        assert_eq!(result, InputPeer::Chat { chat_id: 999 });
        assert_eq!(result.get_chat_id(), Some(999));
    }

    #[test]
    fn test_input_peer_user_deserialize() {
        // inputPeerUser#dde8a54c user_id:long access_hash:long = InputPeer;
        let mut data = vec![0x4c, 0xa5, 0xe8, 0xdd]; // constructor
        data.extend_from_slice(&123i64.to_le_bytes()); // user_id
        data.extend_from_slice(&456i64.to_le_bytes()); // access_hash

        let mut buf = create_buffer(&data);
        let result = InputPeer::deserialize_tl(&mut buf).unwrap();

        assert_eq!(
            result,
            InputPeer::User {
                user_id: 123,
                access_hash: 456
            }
        );
        assert_eq!(result.get_user_id(), Some(123));
    }

    #[test]
    fn test_input_peer_channel_deserialize() {
        // inputPeerChannel#27bcbbfc channel_id:long access_hash:long = InputPeer;
        let mut data = vec![0xfc, 0xbb, 0xbc, 0x27]; // constructor
        data.extend_from_slice(&789i64.to_le_bytes()); // channel_id
        data.extend_from_slice(&999888i64.to_le_bytes()); // access_hash

        let mut buf = create_buffer(&data);
        let result = InputPeer::deserialize_tl(&mut buf).unwrap();

        assert_eq!(
            result,
            InputPeer::Channel {
                channel_id: 789,
                access_hash: 999888
            }
        );
        assert_eq!(result.get_channel_id(), Some(789));
    }

    #[test]
    fn test_input_peer_default() {
        let peer = InputPeer::default();
        assert_eq!(peer, InputPeer::Empty);
        assert!(peer.is_empty());
    }

    #[test]
    fn test_input_peer_display() {
        assert_eq!(format!("{}", InputPeer::Empty), "InputPeerEmpty");
        assert_eq!(format!("{}", InputPeer::InputPeerSelf), "InputPeerSelf");
        assert_eq!(
            format!("{}", InputPeer::Chat { chat_id: 123 }),
            "InputPeerChat(123)"
        );
        assert_eq!(
            format!(
                "{}",
                InputPeer::User {
                    user_id: 456,
                    access_hash: 789
                }
            ),
            "InputPeerUser(456)"
        );
    }

    #[test]
    fn test_input_peer_constructor_id() {
        assert_eq!(
            InputPeer::Empty.constructor_id(),
            InputPeer::EMPTY_CONSTRUCTOR
        );
        assert_eq!(
            InputPeer::InputPeerSelf.constructor_id(),
            InputPeer::SELF_CONSTRUCTOR
        );
        assert_eq!(
            InputPeer::Chat { chat_id: 0 }.constructor_id(),
            InputPeer::CHAT_CONSTRUCTOR
        );
        assert_eq!(
            InputPeer::User {
                user_id: 0,
                access_hash: 0
            }
            .constructor_id(),
            InputPeer::USER_CONSTRUCTOR
        );
    }

    #[test]
    fn test_peer_unknown_constructor() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0]; // invalid constructor
        let mut buf = create_buffer(&data);
        let result = Peer::deserialize_tl(&mut buf);

        assert!(result.is_err());
    }

    #[test]
    fn test_input_peer_unknown_constructor() {
        let data = [0xFF, 0xFF, 0xFF, 0xFF]; // invalid constructor
        let mut buf = create_buffer(&data);
        let result = InputPeer::deserialize_tl(&mut buf);

        assert!(result.is_err());
    }

    #[test]
    fn test_peer_from_message_constructors() {
        assert_eq!(InputPeer::USER_FROM_MESSAGE_CONSTRUCTOR, 0xa87b0a1c);
        assert_eq!(InputPeer::CHANNEL_FROM_MESSAGE_CONSTRUCTOR, 0xbd2a0840);
    }

    #[test]
    fn test_input_peer_getters() {
        let user = InputPeer::User {
            user_id: 123,
            access_hash: 456,
        };
        assert_eq!(user.get_user_id(), Some(123));
        assert_eq!(user.get_chat_id(), None);
        assert_eq!(user.get_channel_id(), None);

        let chat = InputPeer::Chat { chat_id: 789 };
        assert_eq!(chat.get_user_id(), None);
        assert_eq!(chat.get_chat_id(), Some(789));
        assert_eq!(chat.get_channel_id(), None);

        let channel = InputPeer::Channel {
            channel_id: 999,
            access_hash: 888,
        };
        assert_eq!(channel.get_user_id(), None);
        assert_eq!(channel.get_chat_id(), None);
        assert_eq!(channel.get_channel_id(), Some(999));
    }

    #[test]
    fn test_peer_copy_and_hash() {
        let peer1 = Peer::User { user_id: 123 };
        let peer2 = peer1;
        assert_eq!(peer1, peer2);

        let peer3 = Peer::Chat { chat_id: 456 };
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(peer3);
        assert!(set.contains(&peer3));
    }

    // Additional tests to increase coverage

    #[test]
    fn test_peer_const_creators() {
        let user = Peer::user(123);
        assert_eq!(user, Peer::User { user_id: 123 });

        let chat = Peer::chat(456);
        assert_eq!(chat, Peer::Chat { chat_id: 456 });

        let channel = Peer::channel(789);
        assert_eq!(channel, Peer::Channel { channel_id: 789 });
    }

    #[test]
    fn test_peer_getters() {
        let user = Peer::User { user_id: 123 };
        assert_eq!(user.get_user_id(), Some(123));
        assert_eq!(user.get_chat_id(), None);
        assert_eq!(user.get_channel_id(), None);

        let chat = Peer::Chat { chat_id: 456 };
        assert_eq!(chat.get_user_id(), None);
        assert_eq!(chat.get_chat_id(), Some(456));
        assert_eq!(chat.get_channel_id(), None);

        let channel = Peer::Channel { channel_id: 789 };
        assert_eq!(channel.get_user_id(), None);
        assert_eq!(channel.get_chat_id(), None);
        assert_eq!(channel.get_channel_id(), Some(789));
    }

    #[test]
    fn test_peer_is_methods() {
        let user = Peer::User { user_id: 123 };
        assert!(user.is_user());
        assert!(!user.is_chat());
        assert!(!user.is_channel());

        let chat = Peer::Chat { chat_id: 456 };
        assert!(!chat.is_user());
        assert!(chat.is_chat());
        assert!(!chat.is_channel());

        let channel = Peer::Channel { channel_id: 789 };
        assert!(!channel.is_user());
        assert!(!channel.is_chat());
        assert!(channel.is_channel());
    }

    #[test]
    fn test_peer_equality() {
        let user1 = Peer::User { user_id: 123 };
        let user2 = Peer::User { user_id: 123 };
        assert_eq!(user1, user2);

        let user3 = Peer::User { user_id: 456 };
        assert_ne!(user1, user3);

        let chat = Peer::Chat { chat_id: 123 };
        assert_ne!(user1, chat);
    }

    #[test]
    fn test_input_peer_const_creators() {
        assert_eq!(InputPeer::empty(), InputPeer::Empty);
        assert_eq!(InputPeer::self_(), InputPeer::InputPeerSelf);

        let chat = InputPeer::chat(123);
        assert_eq!(chat, InputPeer::Chat { chat_id: 123 });

        let user = InputPeer::user(456, 789);
        assert_eq!(user, InputPeer::User {
            user_id: 456,
            access_hash: 789
        });

        let channel = InputPeer::channel(999, 888);
        assert_eq!(channel, InputPeer::Channel {
            channel_id: 999,
            access_hash: 888
        });
    }

    #[test]
    fn test_input_peer_is_methods() {
        assert!(InputPeer::Empty.is_empty());
        assert!(!InputPeer::Empty.is_self());

        assert!(!InputPeer::InputPeerSelf.is_empty());
        assert!(InputPeer::InputPeerSelf.is_self());

        assert!(!InputPeer::Chat { chat_id: 0 }.is_empty());
        assert!(!InputPeer::Chat { chat_id: 0 }.is_self());
    }

    #[test]
    fn test_input_peer_getters_for_user_from_message() {
        let user_msg = InputPeer::UserFromMessage {
            peer: Box::new(InputPeer::Chat { chat_id: 111 }),
            msg_id: 222,
            user_id: 333,
        };

        assert_eq!(user_msg.get_user_id(), Some(333));
        assert_eq!(user_msg.get_chat_id(), None);
        assert_eq!(user_msg.get_channel_id(), None);
    }

    #[test]
    fn test_input_peer_getters_for_channel_from_message() {
        let channel_msg = InputPeer::ChannelFromMessage {
            peer: Box::new(InputPeer::Chat { chat_id: 111 }),
            msg_id: 222,
            channel_id: 333,
        };

        assert_eq!(channel_msg.get_user_id(), None);
        assert_eq!(channel_msg.get_chat_id(), None);
        assert_eq!(channel_msg.get_channel_id(), Some(333));
    }

    #[test]
    fn test_input_peer_display_from_message() {
        let user_msg = InputPeer::UserFromMessage {
            peer: Box::new(InputPeer::Empty),
            msg_id: 123,
            user_id: 456,
        };

        let display = format!("{}", user_msg);
        assert!(display.contains("123"));
        assert!(display.contains("456"));

        let channel_msg = InputPeer::ChannelFromMessage {
            peer: Box::new(InputPeer::Empty),
            msg_id: 789,
            channel_id: 999,
        };

        let display = format!("{}", channel_msg);
        assert!(display.contains("789"));
        assert!(display.contains("999"));
    }

    #[test]
    fn test_input_peer_equality() {
        let user1 = InputPeer::User {
            user_id: 123,
            access_hash: 456
        };
        let user2 = InputPeer::User {
            user_id: 123,
            access_hash: 456
        };
        assert_eq!(user1, user2);

        let user3 = InputPeer::User {
            user_id: 123,
            access_hash: 789
        };
        assert_ne!(user1, user3);

        let chat = InputPeer::Chat { chat_id: 123 };
        assert_ne!(user1, chat);
    }

    #[test]
    fn test_input_peer_clone() {
        let user1 = InputPeer::User {
            user_id: 123,
            access_hash: 456
        };
        let user2 = user1.clone();
        assert_eq!(user1, user2);
    }

    #[test]
    fn test_peer_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let peer1 = Peer::User { user_id: 123 };
        let peer2 = Peer::User { user_id: 123 };
        let peer3 = Peer::User { user_id: 456 };

        let mut hasher1 = DefaultHasher::new();
        peer1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        peer2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        let mut hasher3 = DefaultHasher::new();
        peer3.hash(&mut hasher3);
        let hash3 = hasher3.finish();

        assert_eq!(hash1, hash2); // Same values should hash the same
        assert_ne!(hash1, hash3); // Different values should hash differently
    }

    #[test]
    fn test_peer_partial_ord() {
        // Test that Peer can be used in HashMap (which requires Hash)
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(Peer::User { user_id: 123 }, "user");
        map.insert(Peer::Chat { chat_id: 456 }, "chat");

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&Peer::User { user_id: 123 }), Some(&"user"));
    }
}
