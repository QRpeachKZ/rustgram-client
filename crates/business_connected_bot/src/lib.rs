// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram Business Connected Bot - Business connected bot types for Telegram MTProto client.
//!
//! This module provides types for working with bots connected to business accounts
//! in the Telegram MTProto protocol, including their permissions and recipient configurations.
//!
//! ## Overview
//!
//! The business connected bot module provides three main types:
//!
//! - [`BusinessConnectedBot`] - Represents a bot connected to a business account
//! - [`BusinessBotRights`] - Defines the permissions/rights of a business bot
//! - [`BusinessRecipients`] - Configures which users receive messages from the bot
//!
//! ## TDLib Correspondence
//!
//! | Rust type | TDLib type | File |
//! |-----------|------------|------|
//! | [`BusinessConnectedBot`] | `td::BusinessConnectedBot` | `BusinessConnectedBot.h/cpp` |
//! | [`BusinessBotRights`] | `td::BusinessBotRights` | `BusinessBotRights.h/cpp` |
//! | [`BusinessRecipients`] | `td::BusinessRecipients` | `BusinessRecipients.h/cpp` |
//!
//! ## Examples
//!
//! ### Creating a Business Connected Bot
//!
//! ```
//! use rustgram_business_connected_bot::{BusinessConnectedBot, BusinessBotRights, BusinessRecipients};
//! use rustgram_types::UserId;
//!
//! // Create recipients configuration
//! let mut recipients = BusinessRecipients::new();
//! recipients.set_existing_chats(true);
//! recipients.set_contacts(true);
//!
//! // Create bot rights
//! let mut rights = BusinessBotRights::new();
//! rights.set_can_reply(true);
//! rights.set_can_read_messages(true);
//!
//! // Create the connected bot
//! let bot = BusinessConnectedBot::new(
//!     UserId(123456789),
//!     recipients,
//!     rights,
//! );
//!
//! assert!(bot.is_valid());
//! assert!(bot.rights().can_reply());
//! ```
//!
//! ### Configuring Bot Rights
//!
//! ```
//! use rustgram_business_connected_bot::{BusinessBotRights, BusinessBotRight};
//!
//! // Start with no rights
//! let mut rights = BusinessBotRights::new();
//!
//! // Grant specific permissions
//! rights.set_right(BusinessBotRight::CanReply, true);
//! rights.set_right(BusinessBotRight::CanReadMessages, true);
//! rights.set_right(BusinessBotRight::CanEditName, true);
//!
//! assert!(rights.has_right(BusinessBotRight::CanReply));
//! assert_eq!(rights.count_enabled(), 3);
//! ```
//!
//! ### Configuring Recipients
//!
//! ```
//! use rustgram_business_connected_bot::BusinessRecipients;
//! use rustgram_types::UserId;
//!
//! let mut recipients = BusinessRecipients::new();
//!
//! // Include specific users
//! recipients.add_user(UserId(123));
//! recipients.add_user(UserId(456));
//!
//! // Exclude specific users
//! recipients.add_excluded_user(UserId(789));
//!
//! // Include all existing chats and contacts
//! recipients.set_existing_chats(true);
//! recipients.set_contacts(true);
//!
//! assert!(!recipients.is_empty());
//! assert!(recipients.contains_user(UserId(123)));
//! ```
//!
//! ## TL Correspondence
//!
//! ### TD API
//!
//! ```text
//! businessConnectedBot
//!   bot_user_id: int53
//!   recipients: businessRecipients
//!   rights: businessBotRights
//! = BusinessConnectedBot
//!
//! businessBotRights
//!   can_reply: Bool
//!   can_read_messages: Bool
//!   can_delete_sent_messages: Bool
//!   can_delete_received_messages: Bool
//!   can_edit_name: Bool
//!   can_edit_bio: Bool
//!   can_edit_profile_photo: Bool
//!   can_edit_username: Bool
//!   can_view_gifts: Bool
//!   can_sell_gifts: Bool
//!   can_change_gift_settings: Bool
//!   can_transfer_and_upgrade_gifts: Bool
//!   can_transfer_stars: Bool
//!   can_manage_stories: Bool
//! = BusinessBotRights
//!
//! businessRecipients
//!   chat_ids: vector<int53>
//!   excluded_chat_ids: vector<int53>
//!   select_existing_chats: Bool
//!   select_new_chats: Bool
//!   select_contacts: Bool
//!   select_non_contacts: Bool
//!   exclude_selected: Bool
//! = BusinessRecipients
//! ```
//!
//! ## Fields
//!
//! ### BusinessConnectedBot
//!
//! - **user_id**: The ID of the connected bot
//! - **recipients**: Configuration of who receives messages from the bot
//! - **rights**: Permissions granted to the bot
//!
//! ### BusinessBotRights
//!
//! Contains 14 boolean permission flags:
//! - `can_reply` - Can reply to messages
//! - `can_read_messages` - Can read messages
//! - `can_delete_sent_messages` - Can delete sent messages
//! - `can_delete_received_messages` - Can delete received messages
//! - `can_edit_name` - Can edit account name
//! - `can_edit_bio` - Can edit account bio
//! - `can_edit_profile_photo` - Can edit profile photo
//! - `can_edit_username` - Can edit username
//! - `can_view_gifts` - Can view gifts and stars
//! - `can_sell_gifts` - Can sell gifts
//! - `can_change_gift_settings` - Can change gift settings
//! - `can_transfer_and_upgrade_gifts` - Can transfer and upgrade gifts
//! - `can_transfer_stars` - Can transfer stars
//! - `can_manage_stories` - Can manage stories
//!
//! ### BusinessRecipients
//!
//! - **user_ids**: List of included user IDs
//! - **excluded_user_ids**: List of excluded user IDs
//! - **existing_chats**: Include existing chats
//! - **new_chats**: Include new chats
//! - **contacts**: Include contacts
//! - **non_contacts**: Include non-contacts
//! - **exclude_selected**: Invert user selection (exclude instead of include)
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
pub use types::{BusinessBotRight, BusinessBotRights, BusinessConnectedBot, BusinessRecipients};

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-business-connected-bot";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-business-connected-bot");
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_create_connected_bot() {
        use rustgram_types::UserId;

        let bot = BusinessConnectedBot::new(
            UserId(123456789),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );

        assert!(bot.is_valid());
        assert_eq!(bot.user_id(), UserId(123456789));
    }

    #[test]
    fn test_bot_rights_all() {
        let rights = BusinessBotRights::all();
        assert_eq!(rights.count_enabled(), 14);
        assert!(rights.can_reply());
        assert!(rights.can_read_messages());
    }

    #[test]
    fn test_bot_rights_empty() {
        let rights = BusinessBotRights::new();
        assert_eq!(rights.count_enabled(), 0);
        assert!(!rights.can_reply());
        assert!(!rights.can_read_messages());
    }

    #[test]
    fn test_recipients_empty() {
        let recipients = BusinessRecipients::new();
        assert!(recipients.is_empty());
        assert!(recipients.user_ids().is_empty());
        assert!(recipients.excluded_user_ids().is_empty());
    }

    #[test]
    fn test_connected_bot_display() {
        use rustgram_types::UserId;

        let bot = BusinessConnectedBot::new(
            UserId(123),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );

        let display = format!("{}", bot);
        assert!(display.contains("BusinessConnectedBot"));
        assert!(display.contains("123"));
    }

    #[test]
    fn test_connected_bot_equality() {
        use rustgram_types::UserId;

        let bot1 = BusinessConnectedBot::new(
            UserId(123),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );

        let bot2 = BusinessConnectedBot::new(
            UserId(123),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );

        assert_eq!(bot1, bot2);
    }

    #[test]
    fn test_connected_bot_clone() {
        use rustgram_types::UserId;

        let bot = BusinessConnectedBot::new(
            UserId(123),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );

        let cloned = bot.clone();
        assert_eq!(bot, cloned);
    }
}
