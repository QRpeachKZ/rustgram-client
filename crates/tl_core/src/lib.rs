// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Core TL (Type Language) deserialization for Telegram MTProto.
//!
//! This crate provides manual implementations of TL binary format
//! deserialization for core Telegram types used by the rustgram-client.
//!
//! # Overview
//!
//! TL (Type Language) is the custom binary serialization format used by
//! Telegram's MTProto protocol. This crate implements deserialization for
//! commonly-used types without relying on code generation, providing:
//!
//! - Type-safe Rust representations of TL types
//! - Manual `impl TlDeserialize` for full control
//! - Proper error handling with `thiserror`
//! - Flag-based optional field support
//! - Comprehensive test coverage
//!
//! # Modules
//!
//! - [`error`] - Error types for TL operations
//! - [`flags`] - Flag reading utilities for optional fields
//! - [`photo`] - Photo and PhotoSize types
//! - [`peer`] - Peer and InputPeer types
//! - [`notify`] - Notification settings types
//! - [`chat_full`] - ChatFull and ChatParticipants types
//! - [`user_full`] - UserFull and BotInfo types
//! - [`utils`] - Utility functions
//!
//! # Example
//!
//! ```rust,ignore
//! use rustgram_tl_core::{Photo, Peer};
//! use rustgram_types::Bytes;
//!
//! // Deserialize a Photo from network response
//! let mut buf = Bytes::new(response_bytes);
//! let photo = Photo::deserialize_tl(&mut buf)?;
//!
//! // Access photo data
//! match photo {
//!     Photo::Empty { id } => println!("Empty photo: {}", id),
//!     Photo::Photo { sizes, .. } => {
//!         for size in sizes {
//!             println!("Photo size: {}x{}", size.w, size.h);
//!         }
//!     }
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Constructor IDs
//!
//! All TL types have a 32-bit constructor ID that identifies the type.
//! The constructor IDs used in this crate are verified against the official
//! Telegram TL schema (`telegram_api.tl`).
//!
//! | Type | Constructor ID | Hex |
//! |------|---------------|-----|
//! | photoEmpty | 0x2331b22d | #2331b22d |
//! | photo | 0xfb197a65 | #fb197a65 |
//! | peerUser | 0x59511722 | #59511722 |
//! | peerChat | 0x36c6019a | #36c6019a |
//! | peerChannel | 0xa2a5371e | #a2a5371e |
//! | peerNotifySettings | 0x99622c0c | #99622c0c |
//! | chatFull | 0x2633421b | #2633421b |
//! | userFull | 0xa02bc13e | #a02bc13e |
//!
//! # Testing
//!
//! ```bash
//! # Run all tests
//! cargo test --package rustgram-tl-core
//!
//! # Run tests with output
//! cargo test --package rustgram-tl-core -- --nocapture
//!
//! # Run specific test
//! cargo test --package rustgram-tl-core test_photo_empty_deserialize
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::let_and_return)]

pub mod chat_full;
pub mod error;
pub mod flags;
pub mod notify;
pub mod peer;
pub mod photo;
pub mod user_full;
pub mod utils;

// Re-exports for convenience
pub use chat_full::{
    BotCommand, BotInfo, ChatFull, ChatParticipant, ChatParticipants, ChatParticipantsData,
    ChatParticipantsForbidden, ChatReactions, ExportedChatInvite, InputGroupCall,
};
pub use error::{Result, TlError, VectorError};
pub use flags::FlagReader;
pub use notify::{InputPeerNotifySettings, NotificationSound, PeerNotifySettings, TlBool};
pub use peer::{InputPeer, Peer};
pub use photo::{Photo, PhotoSize, VideoSize};
pub use user_full::{
    Birthday, BotCommand as UserBotCommand, BotInfo as UserBotInfo, BotMenuButton,
    BusinessWorkHours, ChatTheme, Document, PeerSettings, UserFull,
};

/// Crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-tl-core";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-tl-core");
    }

    #[test]
    fn test_photo_constructors() {
        assert_eq!(Photo::EMPTY_CONSTRUCTOR, 0x2331b22d);
        assert_eq!(Photo::PHOTO_CONSTRUCTOR, 0xfb197a65);
    }

    #[test]
    fn test_peer_constructors() {
        assert_eq!(Peer::USER_CONSTRUCTOR, 0x59511722);
        assert_eq!(Peer::CHAT_CONSTRUCTOR, 0x36c6019a);
        assert_eq!(Peer::CHANNEL_CONSTRUCTOR, 0xa2a5371e);
    }

    #[test]
    fn test_notify_constructors() {
        assert_eq!(PeerNotifySettings::CONSTRUCTOR, 0x99622c0c);
        assert_eq!(NotificationSound::DEFAULT_CONSTRUCTOR, 0x97e8bebe);
        assert_eq!(NotificationSound::NONE_CONSTRUCTOR, 0x6f0c34df);
        assert_eq!(TlBool::TRUE_CONSTRUCTOR, 0x997275b5);
        assert_eq!(TlBool::FALSE_CONSTRUCTOR, 0xbc799737);
    }

    #[test]
    fn test_chat_full_constructors() {
        assert_eq!(ChatFull::CONSTRUCTOR, 0x2633421b);
        assert_eq!(ChatFull::CHANNEL_CONSTRUCTOR, 0xe4e0b29d);
        assert_eq!(ChatParticipants::CHAT_CONSTRUCTOR, 0x3cbc93f8);
        assert_eq!(ChatParticipants::FORBIDDEN_CONSTRUCTOR, 0x8763d3e1);
    }

    #[test]
    fn test_user_full_constructors() {
        assert_eq!(UserFull::CONSTRUCTOR, 0xa02bc13e);
    }

    #[test]
    fn test_flag_reader() {
        let reader = FlagReader::new(0b00001010u32);
        assert!(reader.has(1));
        assert!(reader.has(3));
        assert!(!reader.has(0));
        assert!(!reader.has(2));
    }

    #[test]
    fn test_error_types() {
        let err = TlError::unknown_constructor(vec![0x12345678], 0xAAAAAAAA, "TestType");
        assert!(matches!(err, TlError::UnknownConstructor { .. }));

        let vec_err = VectorError::too_large(10000, 1000);
        assert!(matches!(vec_err, VectorError::TooLarge { .. }));
    }
}
