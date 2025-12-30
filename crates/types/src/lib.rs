//! # Rustgram Types
//!
//! Base types for Telegram MTProto client implementation.
//!
//! This crate provides type-safe representations of Telegram's TL (Type Language)
//! schema types, including identifiers, peers, updates, and serialization utilities.
//!
//! ## Modules
//!
//! - [`ids`] - Telegram identifier types (UserId, ChatId, ChannelId, MessageId, etc.)
//! - [`peer`] - Peer types for conversation participants
//! - [`access`] - Access hash types for authenticated entities
//! - [`primitive`] - Primitive TL types (int, long, double, string, bytes, bool)
//! - [`vector`] - Collection types (Vector, Maybe, Dictionary)
//! - [`update`] - Update types for real-time events
//! - [`tl`] - TL serialization/deserialization traits
//! - [`error`] - Error types for the crate

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::let_and_return)]
#![allow(mismatched_lifetime_syntaxes)]

pub mod access;
pub mod error;
pub mod ids;
pub mod peer;
pub mod primitive;
pub mod tl;
pub mod update;
pub mod vector;

// Re-export commonly used types at the crate root
pub use access::{AccessHash, FileReference};
pub use error::{InvalidIdError, TypeError, TypeResult};
pub use ids::{
    ChannelId, ChatId, DialogId, DialogType, MessageId, MessageType, SecretChatId, UserId,
};
pub use peer::{DialogPeer, InputPeer, NotifyPeer, Peer};
pub use primitive::{
    TlBool, TlBytes, TlDouble, TlInt, TlInt128, TlInt256, TlLong, TlString, TlTrue,
};
pub use tl::{TlBoxed, TlConstructor, TlDeserialize, TlHelper, TlSerialize};
pub use update::{
    ChatUserTypingUpdate, DeleteChannelMessagesUpdate, DeleteMessagesUpdate,
    MessageIdAssignedUpdate, NewAuthorizationUpdate, NewChannelMessageUpdate, NewMessageUpdate,
    SendMessageAction, Update, UpdateShort, UpdateType, UpdatesCombined, UpdatesDifference,
    UpdatesState, UserNameUpdate, UserStatus, UserStatusUpdate, UserTypingUpdate,
};
pub use vector::{Maybe, TlDictionary, TlVector};

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-types";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-types");
    }

    #[test]
    fn test_user_id_creation() {
        let user_id = UserId::new(12345).unwrap();
        assert!(user_id.is_valid());
        assert_eq!(user_id.get(), 12345);
    }

    #[test]
    fn test_peer_creation() {
        let user_id = UserId(123);
        let peer = Peer::user(user_id);
        assert!(peer.is_user());
        assert_eq!(peer.get_user_id(), Some(user_id));
    }

    #[test]
    fn test_access_hash() {
        let hash = AccessHash::new(0x1234567890abcdef);
        assert!(hash.is_valid());
        assert_eq!(hash.get(), 0x1234567890abcdef);
    }

    #[test]
    fn test_dialog_id_from_user() {
        let user_id = UserId(123);
        let dialog_id = DialogId::from_user(user_id);
        assert_eq!(dialog_id.get_user_id(), Some(user_id));
    }

    #[test]
    fn test_vector() {
        let mut vec = TlVector::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0], 1);
    }

    #[test]
    fn test_maybe() {
        let some = Maybe::Some(42);
        assert!(some.is_some());
        assert_eq!(some.to_option(), Some(42));

        let none: Maybe<i32> = Maybe::None;
        assert!(none.is_none());
    }

    #[test]
    fn test_tl_bool() {
        assert!(TlBool::True.as_bool());
        assert!(!TlBool::False.as_bool());
        assert_eq!(TlBool::True.constructor_id(), 0x997275b5);
    }

    #[test]
    fn test_message_id_type() {
        let server_msg = MessageId::from_server_id(100);
        assert!(server_msg.is_server());
        assert_eq!(server_msg.get_type(), MessageType::Server);
        assert_eq!(server_msg.get_server_id(), 100);
    }
}
