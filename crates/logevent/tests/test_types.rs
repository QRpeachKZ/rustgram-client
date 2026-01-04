// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! Integration tests for core types

#![allow(clippy::unwrap_used, clippy::expect_used)]

use rustgram_logevent::{HandlerType, LogEventIdWithGeneration, LogEventIdWithGenerationExt};

#[test]
fn test_handler_type_all_values() {
    let test_cases = [
        (0x01, HandlerType::SecretChats),
        (0x02, HandlerType::Users),
        (0x03, HandlerType::Chats),
        (0x04, HandlerType::Channels),
        (0x05, HandlerType::SecretChatInfos),
        (0x10, HandlerType::WebPages),
        (0x20, HandlerType::SetPollAnswer),
        (0x21, HandlerType::StopPoll),
        (0x100, HandlerType::SendMessage),
        (0x101, HandlerType::DeleteMessage),
        (0x102, HandlerType::DeleteMessagesOnServer),
        (0x103, HandlerType::ReadHistoryOnServer),
        (0x104, HandlerType::ForwardMessages),
        (0x105, HandlerType::ReadMessageContentsOnServer),
        (0x106, HandlerType::SendBotStartMessage),
        (0x107, HandlerType::SendScreenshotTakenNotificationMessage),
        (0x108, HandlerType::SendInlineQueryResultMessage),
        (0x109, HandlerType::DeleteDialogHistoryOnServer),
        (0x10a, HandlerType::ReadAllDialogMentionsOnServer),
        (
            0x10b,
            HandlerType::DeleteAllChannelMessagesFromSenderOnServer,
        ),
        (0x10c, HandlerType::ToggleDialogIsPinnedOnServer),
        (0x10d, HandlerType::ReorderPinnedDialogsOnServer),
        (0x10e, HandlerType::SaveDialogDraftMessageOnServer),
        (0x10f, HandlerType::UpdateDialogNotificationSettingsOnServer),
        (0x110, HandlerType::UpdateScopeNotificationSettingsOnServer),
        (0x111, HandlerType::ResetAllNotificationSettingsOnServer),
        (0x112, HandlerType::ToggleDialogReportSpamStateOnServer),
        (0x113, HandlerType::RegetDialog),
        (0x114, HandlerType::ReadHistoryInSecretChat),
        (0x115, HandlerType::ToggleDialogIsMarkedAsUnreadOnServer),
        (0x116, HandlerType::SetDialogFolderIdOnServer),
        (0x117, HandlerType::DeleteScheduledMessagesOnServer),
        (0x118, HandlerType::ToggleDialogIsBlockedOnServer),
        (0x119, HandlerType::ReadMessageThreadHistoryOnServer),
        (0x120, HandlerType::BlockMessageSenderFromRepliesOnServer),
        (0x121, HandlerType::UnpinAllDialogMessagesOnServer),
        (0x122, HandlerType::DeleteAllCallMessagesOnServer),
        (0x123, HandlerType::DeleteDialogMessagesByDateOnServer),
        (0x124, HandlerType::ReadAllDialogReactionsOnServer),
        (0x125, HandlerType::DeleteTopicHistoryOnServer),
        (0x126, HandlerType::ToggleDialogIsTranslatableOnServer),
        (0x127, HandlerType::ToggleDialogViewAsMessagesOnServer),
        (0x128, HandlerType::SendQuickReplyShortcutMessages),
        (
            0x129,
            HandlerType::UpdateReactionNotificationSettingsOnServer,
        ),
        (0x140, HandlerType::GetChannelDifference),
        (0x200, HandlerType::AddMessagePushNotification),
        (0x201, HandlerType::EditMessagePushNotification),
        (0x300, HandlerType::SaveAppLog),
        (0x400, HandlerType::DeleteStoryOnServer),
        (0x401, HandlerType::ReadStoriesOnServer),
        (0x402, HandlerType::LoadDialogExpiringStories),
        (0x403, HandlerType::SendStory),
        (0x404, HandlerType::EditStory),
        (0x500, HandlerType::ChangeAuthorizationSettingsOnServer),
        (0x501, HandlerType::ResetAuthorizationOnServer),
        (0x502, HandlerType::ResetAuthorizationsOnServer),
        (0x503, HandlerType::SetDefaultHistoryTtlOnServer),
        (0x504, HandlerType::SetAccountTtlOnServer),
        (0x505, HandlerType::SetAuthorizationTtlOnServer),
        (0x506, HandlerType::ResetWebAuthorizationOnServer),
        (0x507, HandlerType::ResetWebAuthorizationsOnServer),
        (0x508, HandlerType::InvalidateSignInCodesOnServer),
        (0x1f18, HandlerType::ConfigPmcMagic),
        (0x4327, HandlerType::BinlogPmcMagic),
    ];

    for (value, expected) in test_cases {
        let parsed = HandlerType::from_u32(value).expect("Failed to parse HandlerType");
        assert_eq!(parsed, expected, "Mismatch for value 0x{:x}", value);
        assert_eq!(parsed as u32, value);
    }
}

#[test]
fn test_handler_type_roundtrip_all() {
    let types = [
        HandlerType::SecretChats,
        HandlerType::Users,
        HandlerType::Chats,
        HandlerType::Channels,
        HandlerType::SecretChatInfos,
        HandlerType::WebPages,
        HandlerType::SetPollAnswer,
        HandlerType::StopPoll,
        HandlerType::SendMessage,
        HandlerType::DeleteMessage,
        HandlerType::DeleteMessagesOnServer,
        HandlerType::ReadHistoryOnServer,
        HandlerType::ForwardMessages,
        HandlerType::ReadMessageContentsOnServer,
        HandlerType::SendBotStartMessage,
        HandlerType::SendScreenshotTakenNotificationMessage,
        HandlerType::SendInlineQueryResultMessage,
        HandlerType::DeleteDialogHistoryOnServer,
        HandlerType::ReadAllDialogMentionsOnServer,
        HandlerType::DeleteAllChannelMessagesFromSenderOnServer,
        HandlerType::ToggleDialogIsPinnedOnServer,
        HandlerType::ReorderPinnedDialogsOnServer,
        HandlerType::SaveDialogDraftMessageOnServer,
        HandlerType::UpdateDialogNotificationSettingsOnServer,
        HandlerType::UpdateScopeNotificationSettingsOnServer,
        HandlerType::ResetAllNotificationSettingsOnServer,
        HandlerType::ToggleDialogReportSpamStateOnServer,
        HandlerType::RegetDialog,
        HandlerType::ReadHistoryInSecretChat,
        HandlerType::ToggleDialogIsMarkedAsUnreadOnServer,
        HandlerType::SetDialogFolderIdOnServer,
        HandlerType::DeleteScheduledMessagesOnServer,
        HandlerType::ToggleDialogIsBlockedOnServer,
        HandlerType::ReadMessageThreadHistoryOnServer,
        HandlerType::BlockMessageSenderFromRepliesOnServer,
        HandlerType::UnpinAllDialogMessagesOnServer,
        HandlerType::DeleteAllCallMessagesOnServer,
        HandlerType::DeleteDialogMessagesByDateOnServer,
        HandlerType::ReadAllDialogReactionsOnServer,
        HandlerType::DeleteTopicHistoryOnServer,
        HandlerType::ToggleDialogIsTranslatableOnServer,
        HandlerType::ToggleDialogViewAsMessagesOnServer,
        HandlerType::SendQuickReplyShortcutMessages,
        HandlerType::UpdateReactionNotificationSettingsOnServer,
        HandlerType::GetChannelDifference,
        HandlerType::AddMessagePushNotification,
        HandlerType::EditMessagePushNotification,
        HandlerType::SaveAppLog,
        HandlerType::DeleteStoryOnServer,
        HandlerType::ReadStoriesOnServer,
        HandlerType::LoadDialogExpiringStories,
        HandlerType::SendStory,
        HandlerType::EditStory,
        HandlerType::ChangeAuthorizationSettingsOnServer,
        HandlerType::ResetAuthorizationOnServer,
        HandlerType::ResetAuthorizationsOnServer,
        HandlerType::SetDefaultHistoryTtlOnServer,
        HandlerType::SetAccountTtlOnServer,
        HandlerType::SetAuthorizationTtlOnServer,
        HandlerType::ResetWebAuthorizationOnServer,
        HandlerType::ResetWebAuthorizationsOnServer,
        HandlerType::InvalidateSignInCodesOnServer,
        HandlerType::ConfigPmcMagic,
        HandlerType::BinlogPmcMagic,
    ];

    for ht in types {
        let value = ht as u32;
        let parsed = HandlerType::from_u32(value)
            .unwrap_or_else(|_| panic!("Failed to parse HandlerType 0x{:x} ({})", value, ht));
        assert_eq!(ht, parsed, "Roundtrip failed for {}", ht);
    }
}

#[test]
fn test_log_event_id_with_generation_is_valid() {
    let id = LogEventIdWithGeneration::default();
    assert!(!id.is_valid());

    let id = LogEventIdWithGeneration::new(12345, 1);
    assert!(id.is_valid());
}

#[test]
fn test_log_event_id_with_generation_reset() {
    let mut id = LogEventIdWithGeneration::new(12345, 1);
    assert!(id.is_valid());

    id.reset();
    assert!(!id.is_valid());
    assert_eq!(id.log_event_id, 0);
    assert_eq!(id.generation, 1); // generation unchanged
}

#[test]
fn test_log_event_id_with_generation_zero() {
    let id = LogEventIdWithGeneration::zero();
    assert_eq!(id.log_event_id, 0);
    assert_eq!(id.generation, 0);
    assert!(!id.is_valid());
}

#[test]
fn test_log_event_id_with_generation_generation_getter() {
    let id = LogEventIdWithGeneration::new(12345, 5);
    assert_eq!(id.generation(), 5);
}

#[test]
fn test_handler_type_display() {
    assert_eq!(format!("{}", HandlerType::SendMessage), "SendMessage");
    assert_eq!(
        format!("{}", HandlerType::ReadHistoryInSecretChat),
        "ReadHistoryInSecretChat"
    );
    assert_eq!(format!("{}", HandlerType::BinlogPmcMagic), "BinlogPmcMagic");
}
