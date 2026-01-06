// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Rustgram Passkey
//!
//! Passkey support for Telegram MTProto client authentication.
//!
//! ## Overview
//!
//! This module provides types and utilities for working with WebAuthn passkeys
//! used for authentication in Telegram. It implements the passkey type from
//! TDLib with support for custom emoji icons and usage tracking.
//!
//! ## Features
//!
//! - [`Passkey`] - Represents a registered passkey with metadata
//! - [`CustomEmojiId`] - Wrapper for custom emoji identifiers
//! - TDLib and TL API compatibility
//!
//! ## Examples
//!
//! ### Creating a Passkey
//!
//! ```rust
//! use rustgram_passkey::{Passkey, CustomEmojiId};
//!
//! // Create a passkey with all fields
//! let passkey = Passkey::builder()
//!     .with_id("credential-id-123".to_string())
//!     .with_name("My YubiKey 5".to_string())
//!     .with_added_date(1704067200) // 2024-01-01
//!     .with_last_usage_date(1704153600) // 2024-01-02
//!     .with_software_emoji_id(Some(CustomEmojiId::new(54321)))
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(passkey.id(), "credential-id-123");
//! assert_eq!(passkey.name(), "My YubiKey 5");
//! assert!(passkey.last_usage_date().is_some());
//! ```
//!
//! ### Checking Passkey Usage
//!
//! ```rust
//! use rustgram_passkey::Passkey;
//!
//! let passkey = Passkey::builder()
//!     .with_id("id".to_string())
//!     .with_name("Security Key".to_string())
//!     .with_added_date(1704067200)
//!     .build()
//!     .unwrap();
//!
//! // Never been used
//! assert!(!passkey.has_been_used());
//! assert!(passkey.last_usage_date().is_none());
//!
//! let mut passkey = passkey.clone();
//! passkey.update_last_usage(1704153600);
//! assert!(passkey.has_been_used());
//! ```
//!
//! ### Custom Emoji Icons
//!
//! ```rust
//! use rustgram_passkey::{Passkey, CustomEmojiId};
//!
//! let passkey = Passkey::builder()
//!     .with_id("id".to_string())
//!     .with_name("Windows Hello".to_string())
//!     .with_added_date(1704067200)
//!     .with_software_emoji_id(Some(CustomEmojiId::new(12345)))
//!     .build()
//!     .unwrap();
//!
//! assert!(passkey.has_software_icon());
//! assert_eq!(passkey.software_emoji_id().unwrap().get(), 12345);
//! ```
//!
//! ## TDLib Compatibility
//!
//! - **Reference**: `references/td/td/telegram/Passkey.{h,cpp}`
//! - **TL Type**: `passkey`
//! - **TL Constructor**: `account.passkeys`, `account.passkeyRegistrationOptions`
//!
//! ## TL Correspondence
//!
//! ### MTProto (telegram_api.tl)
//!
//! ```text
//! passkey#98613ebf flags:# id:string name:string date:int
//!   software_emoji_id:flags.0?long last_usage_date:flags.1?int = Passkey;
//!
//! account.passkeys#f8e0aa1c passkeys:Vector<Passkey> = account.Passkeys;
//! ```
//!
//! ### TD API (td_api.tl)
//!
//! ```text
//! passkey id:string name:string addition_date:int32
//!   last_usage_date:int32 software_icon_custom_emoji_id:int64 = Passkey;
//!
//! passkeys passkeys:vector<passkey> = Passkeys;
//! ```
//!
//! ## Design Decisions
//!
//! 1. **String IDs**: Passkey credential IDs are stored as strings for
//!    compatibility with base64url encoding used in WebAuthn.
//!
//! 2. **Optional Last Usage**: If `last_usage_date` is `None`, the passkey
//!    has never been used.
//!
//! 3. **Custom Emoji Icons**: Software vendors can associate custom emoji
//!    with their passkeys for visual identification. `0` means no icon.
//!
//! 4. **Builder Pattern**: Provides convenient construction with validation.
//!
//! ## Limitations
//!
//! - Passkey IDs must be non-empty strings
//! - Names must be non-empty
//! - Added date must be a valid Unix timestamp
//! - Custom emoji ID of `0` is treated as "no icon"

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

mod emoji_id;
mod error;
mod passkey;

// Re-exports
pub use emoji_id::CustomEmojiId;
pub use error::{PasskeyError, Result};
pub use passkey::Passkey;

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-passkey";

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::const_is_empty)]
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-passkey");
    }

    #[test]
    fn test_custom_emoji_id_validation() {
        // Zero is invalid
        assert!(!CustomEmojiId::new(0).is_valid());

        // Non-zero is valid
        assert!(CustomEmojiId::new(12345).is_valid());
    }

    #[test]
    fn test_custom_emoji_id_get() {
        let id = CustomEmojiId::new(54321);
        assert_eq!(id.get(), 54321);
    }

    #[test]
    fn test_passkey_builder_minimum() {
        let passkey = Passkey::builder()
            .with_id("test-id".to_string())
            .with_name("Test Key".to_string())
            .with_added_date(1704067200)
            .build()
            .unwrap();

        assert_eq!(passkey.id(), "test-id");
        assert_eq!(passkey.name(), "Test Key");
        assert_eq!(passkey.added_date(), 1704067200);
        assert!(!passkey.has_been_used());
        assert!(!passkey.has_software_icon());
    }

    #[test]
    fn test_passkey_builder_full() {
        let emoji = CustomEmojiId::new(12345);
        let passkey = Passkey::builder()
            .with_id("test-id".to_string())
            .with_name("Test Key".to_string())
            .with_added_date(1704067200)
            .with_last_usage_date(Some(1704153600))
            .with_software_emoji_id(Some(emoji))
            .build()
            .unwrap();

        assert!(passkey.has_been_used());
        assert_eq!(passkey.last_usage_date(), Some(1704153600));
        assert!(passkey.has_software_icon());
        assert_eq!(passkey.software_emoji_id().unwrap().get(), 12345);
    }

    #[test]
    fn test_passkey_builder_empty_id() {
        let result = Passkey::builder()
            .with_id("".to_string())
            .with_name("Test Key".to_string())
            .with_added_date(1704067200)
            .build();

        assert!(matches!(result, Err(PasskeyError::InvalidId)));
    }

    #[test]
    fn test_passkey_builder_empty_name() {
        let result = Passkey::builder()
            .with_id("test-id".to_string())
            .with_name("".to_string())
            .with_added_date(1704067200)
            .build();

        assert!(matches!(result, Err(PasskeyError::InvalidName)));
    }

    #[test]
    fn test_passkey_update_last_usage() {
        let mut passkey = Passkey::builder()
            .with_id("test-id".to_string())
            .with_name("Test Key".to_string())
            .with_added_date(1704067200)
            .build()
            .unwrap();

        assert!(!passkey.has_been_used());

        passkey.update_last_usage(1704153600);
        assert!(passkey.has_been_used());
        assert_eq!(passkey.last_usage_date(), Some(1704153600));
    }

    #[test]
    fn test_passkey_set_software_emoji() {
        let mut passkey = Passkey::builder()
            .with_id("test-id".to_string())
            .with_name("Test Key".to_string())
            .with_added_date(1704067200)
            .build()
            .unwrap();

        assert!(!passkey.has_software_icon());

        passkey.set_software_emoji_id(Some(CustomEmojiId::new(99999)));
        assert!(passkey.has_software_icon());
        assert_eq!(passkey.software_emoji_id().unwrap().get(), 99999);
    }

    #[test]
    fn test_passkey_clear_software_emoji() {
        let mut passkey = Passkey::builder()
            .with_id("test-id".to_string())
            .with_name("Test Key".to_string())
            .with_added_date(1704067200)
            .with_software_emoji_id(Some(CustomEmojiId::new(12345)))
            .build()
            .unwrap();

        assert!(passkey.has_software_icon());

        passkey.set_software_emoji_id(None);
        assert!(!passkey.has_software_icon());
    }
}
