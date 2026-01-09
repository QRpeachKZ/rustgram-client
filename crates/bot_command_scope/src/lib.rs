// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! Bot command scope type for Telegram MTProto client.

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_types::{DialogId, UserId};
use std::hash::{Hash, Hasher};

/// Bot command scope type enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum BotCommandScopeType {
    /// Default scope.
    #[default]
    Default = 0,
    /// All users.
    AllUsers = 1,
    /// All chats.
    AllChats = 2,
    /// All chat administrators.
    AllChatAdministrators = 3,
    /// Specific dialog.
    Dialog = 4,
    /// Dialog administrators.
    DialogAdministrators = 5,
    /// Dialog participant.
    DialogParticipant = 6,
}

/// Bot command scope type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BotCommandScope {
    scope_type: BotCommandScopeType,
    dialog_id: DialogId,
    user_id: UserId,
}

impl BotCommandScope {
    /// Create a default bot command scope.
    #[must_use]
    pub fn new() -> Self {
        Self {
            scope_type: BotCommandScopeType::Default,
            dialog_id: DialogId::User(UserId(0)),
            user_id: UserId(0),
        }
    }

    /// Create an all-users scope.
    #[must_use]
    pub fn all_users() -> Self {
        Self {
            scope_type: BotCommandScopeType::AllUsers,
            dialog_id: DialogId::User(UserId(0)),
            user_id: UserId(0),
        }
    }

    /// Create an all-chats scope.
    #[must_use]
    pub fn all_chats() -> Self {
        Self {
            scope_type: BotCommandScopeType::AllChats,
            dialog_id: DialogId::User(UserId(0)),
            user_id: UserId(0),
        }
    }

    /// Create an all-chat-administrators scope.
    #[must_use]
    pub fn all_chat_administrators() -> Self {
        Self {
            scope_type: BotCommandScopeType::AllChatAdministrators,
            dialog_id: DialogId::User(UserId(0)),
            user_id: UserId(0),
        }
    }

    /// Create a dialog-specific scope.
    #[must_use]
    pub fn dialog(dialog_id: DialogId) -> Self {
        Self {
            scope_type: BotCommandScopeType::Dialog,
            dialog_id,
            user_id: UserId(0),
        }
    }

    /// Create a dialog administrators scope.
    #[must_use]
    pub fn dialog_administrators(dialog_id: DialogId) -> Self {
        Self {
            scope_type: BotCommandScopeType::DialogAdministrators,
            dialog_id,
            user_id: UserId(0),
        }
    }

    /// Create a dialog participant scope.
    #[must_use]
    pub fn dialog_participant(dialog_id: DialogId, user_id: UserId) -> Self {
        Self {
            scope_type: BotCommandScopeType::DialogParticipant,
            dialog_id,
            user_id,
        }
    }

    /// Get the scope type.
    #[must_use]
    pub const fn scope_type(&self) -> BotCommandScopeType {
        self.scope_type
    }

    /// Get the dialog ID.
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Get the user ID.
    #[must_use]
    pub const fn user_id(&self) -> UserId {
        self.user_id
    }
}

impl Hash for BotCommandScope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.scope_type.hash(state);
        // Use discriminant for DialogId enum hashing
        std::mem::discriminant(&self.dialog_id).hash(state);
        self.user_id.hash(state);
    }
}

impl Default for BotCommandScope {
    fn default() -> Self {
        Self::new()
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-bot-command-scope";

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::{ChatId, ChannelId};

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-bot-command-scope");
    }

    #[test]
    fn test_new() {
        assert_eq!(
            BotCommandScope::new().scope_type(),
            BotCommandScopeType::Default
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(
            BotCommandScope::default().scope_type(),
            BotCommandScopeType::Default
        );
    }

    #[test]
    fn test_all_users() {
        assert_eq!(
            BotCommandScope::all_users().scope_type(),
            BotCommandScopeType::AllUsers
        );
    }

    #[test]
    fn test_all_chats() {
        assert_eq!(
            BotCommandScope::all_chats().scope_type(),
            BotCommandScopeType::AllChats
        );
    }

    #[test]
    fn test_dialog_user() {
        let d = DialogId::User(UserId(1));
        assert_eq!(BotCommandScope::dialog(d).dialog_id(), d);
    }

    #[test]
    fn test_dialog_chat() {
        let d = DialogId::Chat(ChatId(1));
        assert_eq!(BotCommandScope::dialog(d).dialog_id(), d);
    }

    #[test]
    fn test_dialog_channel() {
        let d = DialogId::Channel(ChannelId(1));
        assert_eq!(BotCommandScope::dialog(d).dialog_id(), d);
    }

    #[test]
    fn test_participant() {
        let d = DialogId::User(UserId(1));
        let u = UserId(2);
        assert_eq!(BotCommandScope::dialog_participant(d, u).user_id(), u);
    }

    #[test]
    fn test_equality() {
        let s1 = BotCommandScope::all_users();
        let s2 = BotCommandScope::all_users();
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(BotCommandScope::all_users());
        set.insert(BotCommandScope::all_chats());
        set.insert(BotCommandScope::default());
        assert_eq!(set.len(), 3);
    }
}
