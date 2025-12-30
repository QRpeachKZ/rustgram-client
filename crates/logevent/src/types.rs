// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! Core type definitions for log events

use crate::Result;
use std::fmt;

/// Handler type for log events
///
/// Each log event has a handler type that indicates what operation it represents.
/// These values must match TDLib's HandlerType enum exactly.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandlerType {
    /// Secret chats events
    SecretChats = 0x01,
    /// Users events
    Users = 0x02,
    /// Chats events
    Chats = 0x03,
    /// Channels events
    Channels = 0x04,
    /// Secret chat infos
    SecretChatInfos = 0x05,
    /// Web pages
    WebPages = 0x10,
    /// Set poll answer
    SetPollAnswer = 0x20,
    /// Stop poll
    StopPoll = 0x21,
    /// Send message
    SendMessage = 0x100,
    /// Delete message
    DeleteMessage = 0x101,
    /// Delete messages on server
    DeleteMessagesOnServer = 0x102,
    /// Read history on server
    ReadHistoryOnServer = 0x103,
    /// Forward messages
    ForwardMessages = 0x104,
    /// Read message contents on server
    ReadMessageContentsOnServer = 0x105,
    /// Send bot start message
    SendBotStartMessage = 0x106,
    /// Send screenshot taken notification
    SendScreenshotTakenNotificationMessage = 0x107,
    /// Send inline query result message
    SendInlineQueryResultMessage = 0x108,
    /// Delete dialog history on server
    DeleteDialogHistoryOnServer = 0x109,
    /// Read all dialog mentions on server
    ReadAllDialogMentionsOnServer = 0x10a,
    /// Delete all channel messages from sender on server
    DeleteAllChannelMessagesFromSenderOnServer = 0x10b,
    /// Toggle dialog is pinned on server
    ToggleDialogIsPinnedOnServer = 0x10c,
    /// Reorder pinned dialogs on server
    ReorderPinnedDialogsOnServer = 0x10d,
    /// Save dialog draft message on server
    SaveDialogDraftMessageOnServer = 0x10e,
    /// Update dialog notification settings on server
    UpdateDialogNotificationSettingsOnServer = 0x10f,
    /// Update scope notification settings on server
    UpdateScopeNotificationSettingsOnServer = 0x110,
    /// Reset all notification settings on server
    ResetAllNotificationSettingsOnServer = 0x111,
    /// Toggle dialog report spam state on server
    ToggleDialogReportSpamStateOnServer = 0x112,
    /// Reget dialog
    RegetDialog = 0x113,
    /// Read history in secret chat
    ReadHistoryInSecretChat = 0x114,
    /// Toggle dialog is marked as unread on server
    ToggleDialogIsMarkedAsUnreadOnServer = 0x115,
    /// Set dialog folder ID on server
    SetDialogFolderIdOnServer = 0x116,
    /// Delete scheduled messages on server
    DeleteScheduledMessagesOnServer = 0x117,
    /// Toggle dialog is blocked on server
    ToggleDialogIsBlockedOnServer = 0x118,
    /// Read message thread history on server
    ReadMessageThreadHistoryOnServer = 0x119,
    /// Block message sender from replies on server
    BlockMessageSenderFromRepliesOnServer = 0x120,
    /// Unpin all dialog messages on server
    UnpinAllDialogMessagesOnServer = 0x121,
    /// Delete all call messages on server
    DeleteAllCallMessagesOnServer = 0x122,
    /// Delete dialog messages by date on server
    DeleteDialogMessagesByDateOnServer = 0x123,
    /// Read all dialog reactions on server
    ReadAllDialogReactionsOnServer = 0x124,
    /// Delete topic history on server
    DeleteTopicHistoryOnServer = 0x125,
    /// Toggle dialog is translatable on server
    ToggleDialogIsTranslatableOnServer = 0x126,
    /// Toggle dialog view as messages on server
    ToggleDialogViewAsMessagesOnServer = 0x127,
    /// Send quick reply shortcut messages
    SendQuickReplyShortcutMessages = 0x128,
    /// Update reaction notification settings on server
    UpdateReactionNotificationSettingsOnServer = 0x129,
    /// Get channel difference
    GetChannelDifference = 0x140,
    /// Add message push notification
    AddMessagePushNotification = 0x200,
    /// Edit message push notification
    EditMessagePushNotification = 0x201,
    /// Save app log
    SaveAppLog = 0x300,
    /// Delete story on server
    DeleteStoryOnServer = 0x400,
    /// Read stories on server
    ReadStoriesOnServer = 0x401,
    /// Load dialog expiring stories
    LoadDialogExpiringStories = 0x402,
    /// Send story
    SendStory = 0x403,
    /// Edit story
    EditStory = 0x404,
    /// Change authorization settings on server
    ChangeAuthorizationSettingsOnServer = 0x500,
    /// Reset authorization on server
    ResetAuthorizationOnServer = 0x501,
    /// Reset authorizations on server
    ResetAuthorizationsOnServer = 0x502,
    /// Set default history TTL on server
    SetDefaultHistoryTtlOnServer = 0x503,
    /// Set account TTL on server
    SetAccountTtlOnServer = 0x504,
    /// Set authorization TTL on server
    SetAuthorizationTtlOnServer = 0x505,
    /// Reset web authorization on server
    ResetWebAuthorizationOnServer = 0x506,
    /// Reset web authorizations on server
    ResetWebAuthorizationsOnServer = 0x507,
    /// Invalidate sign in codes on server
    InvalidateSignInCodesOnServer = 0x508,
    /// Config PMC magic
    ConfigPmcMagic = 0x1f18,
    /// Binlog PMC magic
    BinlogPmcMagic = 0x4327,
}

impl HandlerType {
    /// Parse a u32 into a HandlerType
    pub fn from_u32(value: u32) -> Result<Self> {
        match value {
            0x01 => Ok(Self::SecretChats),
            0x02 => Ok(Self::Users),
            0x03 => Ok(Self::Chats),
            0x04 => Ok(Self::Channels),
            0x05 => Ok(Self::SecretChatInfos),
            0x10 => Ok(Self::WebPages),
            0x20 => Ok(Self::SetPollAnswer),
            0x21 => Ok(Self::StopPoll),
            0x100 => Ok(Self::SendMessage),
            0x101 => Ok(Self::DeleteMessage),
            0x102 => Ok(Self::DeleteMessagesOnServer),
            0x103 => Ok(Self::ReadHistoryOnServer),
            0x104 => Ok(Self::ForwardMessages),
            0x105 => Ok(Self::ReadMessageContentsOnServer),
            0x106 => Ok(Self::SendBotStartMessage),
            0x107 => Ok(Self::SendScreenshotTakenNotificationMessage),
            0x108 => Ok(Self::SendInlineQueryResultMessage),
            0x109 => Ok(Self::DeleteDialogHistoryOnServer),
            0x10a => Ok(Self::ReadAllDialogMentionsOnServer),
            0x10b => Ok(Self::DeleteAllChannelMessagesFromSenderOnServer),
            0x10c => Ok(Self::ToggleDialogIsPinnedOnServer),
            0x10d => Ok(Self::ReorderPinnedDialogsOnServer),
            0x10e => Ok(Self::SaveDialogDraftMessageOnServer),
            0x10f => Ok(Self::UpdateDialogNotificationSettingsOnServer),
            0x110 => Ok(Self::UpdateScopeNotificationSettingsOnServer),
            0x111 => Ok(Self::ResetAllNotificationSettingsOnServer),
            0x112 => Ok(Self::ToggleDialogReportSpamStateOnServer),
            0x113 => Ok(Self::RegetDialog),
            0x114 => Ok(Self::ReadHistoryInSecretChat),
            0x115 => Ok(Self::ToggleDialogIsMarkedAsUnreadOnServer),
            0x116 => Ok(Self::SetDialogFolderIdOnServer),
            0x117 => Ok(Self::DeleteScheduledMessagesOnServer),
            0x118 => Ok(Self::ToggleDialogIsBlockedOnServer),
            0x119 => Ok(Self::ReadMessageThreadHistoryOnServer),
            0x120 => Ok(Self::BlockMessageSenderFromRepliesOnServer),
            0x121 => Ok(Self::UnpinAllDialogMessagesOnServer),
            0x122 => Ok(Self::DeleteAllCallMessagesOnServer),
            0x123 => Ok(Self::DeleteDialogMessagesByDateOnServer),
            0x124 => Ok(Self::ReadAllDialogReactionsOnServer),
            0x125 => Ok(Self::DeleteTopicHistoryOnServer),
            0x126 => Ok(Self::ToggleDialogIsTranslatableOnServer),
            0x127 => Ok(Self::ToggleDialogViewAsMessagesOnServer),
            0x128 => Ok(Self::SendQuickReplyShortcutMessages),
            0x129 => Ok(Self::UpdateReactionNotificationSettingsOnServer),
            0x140 => Ok(Self::GetChannelDifference),
            0x200 => Ok(Self::AddMessagePushNotification),
            0x201 => Ok(Self::EditMessagePushNotification),
            0x300 => Ok(Self::SaveAppLog),
            0x400 => Ok(Self::DeleteStoryOnServer),
            0x401 => Ok(Self::ReadStoriesOnServer),
            0x402 => Ok(Self::LoadDialogExpiringStories),
            0x403 => Ok(Self::SendStory),
            0x404 => Ok(Self::EditStory),
            0x500 => Ok(Self::ChangeAuthorizationSettingsOnServer),
            0x501 => Ok(Self::ResetAuthorizationOnServer),
            0x502 => Ok(Self::ResetAuthorizationsOnServer),
            0x503 => Ok(Self::SetDefaultHistoryTtlOnServer),
            0x504 => Ok(Self::SetAccountTtlOnServer),
            0x505 => Ok(Self::SetAuthorizationTtlOnServer),
            0x506 => Ok(Self::ResetWebAuthorizationOnServer),
            0x507 => Ok(Self::ResetWebAuthorizationsOnServer),
            0x508 => Ok(Self::InvalidateSignInCodesOnServer),
            0x1f18 => Ok(Self::ConfigPmcMagic),
            0x4327 => Ok(Self::BinlogPmcMagic),
            _ => Err(crate::LogEventError::UnknownEventType(value)),
        }
    }
}

impl fmt::Display for HandlerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::SecretChats => "SecretChats",
            Self::Users => "Users",
            Self::Chats => "Chats",
            Self::Channels => "Channels",
            Self::SecretChatInfos => "SecretChatInfos",
            Self::WebPages => "WebPages",
            Self::SetPollAnswer => "SetPollAnswer",
            Self::StopPoll => "StopPoll",
            Self::SendMessage => "SendMessage",
            Self::DeleteMessage => "DeleteMessage",
            Self::DeleteMessagesOnServer => "DeleteMessagesOnServer",
            Self::ReadHistoryOnServer => "ReadHistoryOnServer",
            Self::ForwardMessages => "ForwardMessages",
            Self::ReadMessageContentsOnServer => "ReadMessageContentsOnServer",
            Self::SendBotStartMessage => "SendBotStartMessage",
            Self::SendScreenshotTakenNotificationMessage => {
                "SendScreenshotTakenNotificationMessage"
            }
            Self::SendInlineQueryResultMessage => "SendInlineQueryResultMessage",
            Self::DeleteDialogHistoryOnServer => "DeleteDialogHistoryOnServer",
            Self::ReadAllDialogMentionsOnServer => "ReadAllDialogMentionsOnServer",
            Self::DeleteAllChannelMessagesFromSenderOnServer => {
                "DeleteAllChannelMessagesFromSenderOnServer"
            }
            Self::ToggleDialogIsPinnedOnServer => "ToggleDialogIsPinnedOnServer",
            Self::ReorderPinnedDialogsOnServer => "ReorderPinnedDialogsOnServer",
            Self::SaveDialogDraftMessageOnServer => "SaveDialogDraftMessageOnServer",
            Self::UpdateDialogNotificationSettingsOnServer => {
                "UpdateDialogNotificationSettingsOnServer"
            }
            Self::UpdateScopeNotificationSettingsOnServer => {
                "UpdateScopeNotificationSettingsOnServer"
            }
            Self::ResetAllNotificationSettingsOnServer => "ResetAllNotificationSettingsOnServer",
            Self::ToggleDialogReportSpamStateOnServer => "ToggleDialogReportSpamStateOnServer",
            Self::RegetDialog => "RegetDialog",
            Self::ReadHistoryInSecretChat => "ReadHistoryInSecretChat",
            Self::ToggleDialogIsMarkedAsUnreadOnServer => "ToggleDialogIsMarkedAsUnreadOnServer",
            Self::SetDialogFolderIdOnServer => "SetDialogFolderIdOnServer",
            Self::DeleteScheduledMessagesOnServer => "DeleteScheduledMessagesOnServer",
            Self::ToggleDialogIsBlockedOnServer => "ToggleDialogIsBlockedOnServer",
            Self::ReadMessageThreadHistoryOnServer => "ReadMessageThreadHistoryOnServer",
            Self::BlockMessageSenderFromRepliesOnServer => "BlockMessageSenderFromRepliesOnServer",
            Self::UnpinAllDialogMessagesOnServer => "UnpinAllDialogMessagesOnServer",
            Self::DeleteAllCallMessagesOnServer => "DeleteAllCallMessagesOnServer",
            Self::DeleteDialogMessagesByDateOnServer => "DeleteDialogMessagesByDateOnServer",
            Self::ReadAllDialogReactionsOnServer => "ReadAllDialogReactionsOnServer",
            Self::DeleteTopicHistoryOnServer => "DeleteTopicHistoryOnServer",
            Self::ToggleDialogIsTranslatableOnServer => "ToggleDialogIsTranslatableOnServer",
            Self::ToggleDialogViewAsMessagesOnServer => "ToggleDialogViewAsMessagesOnServer",
            Self::SendQuickReplyShortcutMessages => "SendQuickReplyShortcutMessages",
            Self::UpdateReactionNotificationSettingsOnServer => {
                "UpdateReactionNotificationSettingsOnServer"
            }
            Self::GetChannelDifference => "GetChannelDifference",
            Self::AddMessagePushNotification => "AddMessagePushNotification",
            Self::EditMessagePushNotification => "EditMessagePushNotification",
            Self::SaveAppLog => "SaveAppLog",
            Self::DeleteStoryOnServer => "DeleteStoryOnServer",
            Self::ReadStoriesOnServer => "ReadStoriesOnServer",
            Self::LoadDialogExpiringStories => "LoadDialogExpiringStories",
            Self::SendStory => "SendStory",
            Self::EditStory => "EditStory",
            Self::ChangeAuthorizationSettingsOnServer => "ChangeAuthorizationSettingsOnServer",
            Self::ResetAuthorizationOnServer => "ResetAuthorizationOnServer",
            Self::ResetAuthorizationsOnServer => "ResetAuthorizationsOnServer",
            Self::SetDefaultHistoryTtlOnServer => "SetDefaultHistoryTtlOnServer",
            Self::SetAccountTtlOnServer => "SetAccountTtlOnServer",
            Self::SetAuthorizationTtlOnServer => "SetAuthorizationTtlOnServer",
            Self::ResetWebAuthorizationOnServer => "ResetWebAuthorizationOnServer",
            Self::ResetWebAuthorizationsOnServer => "ResetWebAuthorizationsOnServer",
            Self::InvalidateSignInCodesOnServer => "InvalidateSignInCodesOnServer",
            Self::ConfigPmcMagic => "ConfigPmcMagic",
            Self::BinlogPmcMagic => "BinlogPmcMagic",
        };
        write!(f, "{}", name)
    }
}

/// Log event ID with generation tracking
///
/// Tracks a log event ID across multiple generations. Each time an event is
/// rewritten (e.g., during a binlog rewrite), the generation increments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LogEventIdWithGeneration {
    /// The log event ID (0 if not yet persisted)
    pub log_event_id: u64,
    /// The generation counter (increments on each rewrite)
    pub generation: u64,
}

impl LogEventIdWithGeneration {
    /// Creates a new LogEventIdWithGeneration
    #[must_use]
    pub const fn new(log_event_id: u64, generation: u64) -> Self {
        Self {
            log_event_id,
            generation,
        }
    }

    /// Returns true if this has a valid log event ID
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.log_event_id != 0
    }

    /// Resets the log event ID to 0
    pub fn reset(&mut self) {
        self.log_event_id = 0;
    }

    /// Returns the current generation
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }
}

/// Extension trait for LogEventIdWithGeneration
pub trait LogEventIdWithGenerationExt {
    /// Creates a new instance with zero values
    fn zero() -> Self;
}

impl LogEventIdWithGenerationExt for LogEventIdWithGeneration {
    fn zero() -> Self {
        Self {
            log_event_id: 0,
            generation: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LogEventError;

    #[test]
    fn test_handler_type_roundtrip() {
        let types = [
            HandlerType::SecretChats,
            HandlerType::SendMessage,
            HandlerType::ReadHistoryInSecretChat,
            HandlerType::BinlogPmcMagic,
        ];

        for ht in types {
            let value = ht as u32;
            let parsed = HandlerType::from_u32(value).unwrap();
            assert_eq!(ht, parsed);
        }
    }

    #[test]
    fn test_handler_type_invalid() {
        let result = HandlerType::from_u32(0xDEADBEEF);
        assert!(matches!(
            result,
            Err(LogEventError::UnknownEventType(0xDEADBEEF))
        ));
    }

    #[test]
    fn test_log_event_id_with_generation_default() {
        let id = LogEventIdWithGeneration::default();
        assert_eq!(id.log_event_id, 0);
        assert_eq!(id.generation, 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_log_event_id_with_generation_new() {
        let id = LogEventIdWithGeneration::new(12345, 2);
        assert_eq!(id.log_event_id, 12345);
        assert_eq!(id.generation, 2);
        assert!(id.is_valid());
    }

    #[test]
    fn test_log_event_id_with_generation_reset() {
        let mut id = LogEventIdWithGeneration::new(12345, 2);
        id.reset();
        assert_eq!(id.log_event_id, 0);
        assert!(!id.is_valid());
    }

    #[test]
    fn test_log_event_id_with_generation_ext() {
        let id = LogEventIdWithGeneration::default();
        assert_eq!(id.log_event_id, 0);
        assert_eq!(id.generation, 0);
    }

    #[test]
    fn test_handler_type_display() {
        assert_eq!(format!("{}", HandlerType::SendMessage), "SendMessage");
        assert_eq!(format!("{}", HandlerType::SecretChats), "SecretChats");
    }
}
