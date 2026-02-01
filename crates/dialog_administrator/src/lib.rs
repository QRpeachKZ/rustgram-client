// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram Dialog Administrator - Dialog administrator types for Telegram MTProto client.
//!
//! This module provides types for working with chat administrators in the Telegram
//! MTProto protocol, including their custom ranks/titles and owner status.
//!
//! ## Overview
//!
//! The dialog administrator module provides the [`DialogAdministrator`] type which
//! represents an administrator in a Telegram chat with their custom title and
//! ownership status.
//!
//! ## TDLib Correspondence
//!
//! | Rust type | TDLib type | File |
//! |-----------|------------|------|
//! | [`DialogAdministrator`] | `td::DialogAdministrator` | `DialogAdministrator.h/cpp` |
//!
//! ## Examples
//!
//! ### Creating a Dialog Administrator
//!
//! ```
//! use rustgram_dialog_administrator::DialogAdministrator;
//! use rustgram_types::UserId;
//!
//! // Create a regular administrator with a custom rank
//! let admin = DialogAdministrator::new(
//!     UserId(12345678),
//!     "Moderator".to_string(),
//!     false,
//! );
//!
//! assert_eq!(admin.user_id(), UserId(12345678));
//! assert_eq!(admin.rank(), "Moderator");
//! assert!(!admin.is_creator());
//! ```
//!
//! ### Creating a Chat Owner
//!
//! ```
//! use rustgram_dialog_administrator::DialogAdministrator;
//! use rustgram_types::UserId;
//!
//! // Create the chat owner/creator
//! let owner = DialogAdministrator::new(
//!     UserId(1),
//!     "Founder".to_string(),
//!     true,
//! );
//!
//! assert!(owner.is_creator());
//! assert_eq!(owner.rank(), "Founder");
//! ```
//!
//! ### Default Construction
//!
//! ```
//! use rustgram_dialog_administrator::DialogAdministrator;
//!
//! // Create a default (empty) administrator
//! let admin = DialogAdministrator::default();
//! assert_eq!(admin.user_id().get(), 0);
//! assert!(admin.rank().is_empty());
//! assert!(!admin.is_creator());
//! ```
//!
//! ### Comparison and Equality
//!
//! ```
//! use rustgram_dialog_administrator::DialogAdministrator;
//! use rustgram_types::UserId;
//!
//! let admin1 = DialogAdministrator::new(
//!     UserId(123),
//!     "Admin".to_string(),
//!     false,
//! );
//!
//! let admin2 = DialogAdministrator::new(
//!     UserId(123),
//!     "Admin".to_string(),
//!     false,
//! );
//!
//! assert_eq!(admin1, admin2);
//! ```
//!
//! ## TL Correspondence
//!
//! ### TD API
//!
//! ```text
//! chatAdministrator
//!   user_id: int53
//!   custom_title: string
//!   is_owner: Bool
//! = ChatAdministrator
//! ```
//!
//! ## Fields
//!
//! - **user_id**: The ID of the user who is an administrator
//! - **rank**: Custom title/rank displayed in the chat (e.g., "Moderator", "Admin")
//! - **is_creator**: Whether this user is the owner/creator of the chat
//!
//! ## Features
//!
//! - **serde**: Enable `Serialize` and `Deserialize` implementations for JSON/bincode support
//! - **proptest**: Enable property-based testing utilities

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
#![allow(clippy::redundant_closure)]
#![allow(clippy::let_and_return)]

pub mod types;

// Re-export public API
pub use types::DialogAdministrator;

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-dialog-administrator";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-dialog-administrator");
    }

    #[test]
    fn test_create_admin() {
        use rustgram_types::UserId;

        let admin = DialogAdministrator::new(UserId(12345), "Admin".to_string(), false);
        assert_eq!(admin.user_id(), UserId(12345));
        assert_eq!(admin.rank(), "Admin");
        assert!(!admin.is_creator());
    }

    #[test]
    fn test_create_owner() {
        use rustgram_types::UserId;

        let owner = DialogAdministrator::new(UserId(1), "Owner".to_string(), true);
        assert!(owner.is_creator());
        assert_eq!(owner.rank(), "Owner");
    }

    #[test]
    fn test_default_admin() {
        let admin = DialogAdministrator::default();
        assert_eq!(admin.user_id().get(), 0);
        assert!(admin.rank().is_empty());
        assert!(!admin.is_creator());
    }

    #[test]
    fn test_equality() {
        use rustgram_types::UserId;

        let admin1 = DialogAdministrator::new(UserId(123), "Admin".to_string(), false);
        let admin2 = DialogAdministrator::new(UserId(123), "Admin".to_string(), false);
        assert_eq!(admin1, admin2);
    }

    #[test]
    fn test_inequality() {
        use rustgram_types::UserId;

        let admin1 = DialogAdministrator::new(UserId(123), "Admin".to_string(), false);
        let admin2 = DialogAdministrator::new(UserId(456), "Admin".to_string(), false);
        assert_ne!(admin1, admin2);
    }
}
