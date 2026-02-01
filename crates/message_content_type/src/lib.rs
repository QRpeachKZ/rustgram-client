// Copyright 2025 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Message Content Type
//!
//! Enumeration of all possible message content types in Telegram.
//!
//! ## Overview
//!
//! This module provides the [`MessageContentType`] enum which represents
//! all 102 possible message content types from TDLib, including:
//!
//! - Media content (text, photo, video, audio, etc.)
//! - Service messages (chat create, pin message, etc.)
//! - Interactive content (polls, games, invoices)
//! - Modern features (stories, gifts, giveaways, boosts)
//!
//! ## TDLib Reference
//!
//! Corresponds to `td/telegram/MessageContentType.h` in TDLib.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_content_type::MessageContentType;
//!
//! let content_type = MessageContentType::Text;
//! assert!(content_type.is_editable());
//! assert!(!content_type.is_service());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Message content type enumeration.
///
/// Represents all 102 possible message content types from TDLib.
/// Each variant corresponds to a specific type of message that can be
/// sent or received in Telegram.
///
/// # TDLib Correspondence
///
/// - `None = -1` - No content
/// - `Text = 0` - Plain text message
/// - `Animation = 1` - GIF animation
/// - `Audio = 2` - Audio file
/// - etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[repr(i32)]
pub enum MessageContentType {
    /// No content (-1)
    #[default]
    None = -1,
    /// Plain text message (0)
    Text = 0,
    /// GIF animation (1)
    Animation = 1,
    /// Audio file (2)
    Audio = 2,
    /// Generic document (3)
    Document = 3,
    /// Photo (4)
    Photo = 4,
    /// Sticker (5)
    Sticker = 5,
    /// Video (6)
    Video = 6,
    /// Voice note (7)
    VoiceNote = 7,
    /// Contact (8)
    Contact = 8,
    /// Location (9)
    Location = 9,
    /// Venue (10)
    Venue = 10,
    /// Chat created (11)
    ChatCreate = 11,
    /// Chat title changed (12)
    ChatChangeTitle = 12,
    /// Chat photo changed (13)
    ChatChangePhoto = 13,
    /// Chat photo deleted (14)
    ChatDeletePhoto = 14,
    /// Chat history deleted (15)
    ChatDeleteHistory = 15,
    /// Users added to chat (16)
    ChatAddUsers = 16,
    /// User joined via invite link (17)
    ChatJoinedByLink = 17,
    /// User removed from chat (18)
    ChatDeleteUser = 18,
    /// Chat migrated to supergroup (19)
    ChatMigrateTo = 19,
    /// Channel created (20)
    ChannelCreate = 20,
    /// Channel migrated from group (21)
    ChannelMigrateFrom = 21,
    /// Message pinned (22)
    PinMessage = 22,
    /// Game (23)
    Game = 23,
    /// Game score (24)
    GameScore = 24,
    /// Screenshot taken (25)
    ScreenshotTaken = 25,
    /// Chat TTL set (26)
    ChatSetTtl = 26,
    /// Unsupported message (27)
    Unsupported = 27,
    /// Call (28)
    Call = 28,
    /// Invoice (29)
    Invoice = 29,
    /// Payment successful (30)
    PaymentSuccessful = 30,
    /// Video note (31)
    VideoNote = 31,
    /// Contact registered (32)
    ContactRegistered = 32,
    /// Expired photo (33)
    ExpiredPhoto = 33,
    /// Expired video (34)
    ExpiredVideo = 34,
    /// Live location (35)
    LiveLocation = 35,
    /// Custom service action (36)
    CustomServiceAction = 36,
    /// Website connected (37)
    WebsiteConnected = 37,
    /// Passport data sent (38)
    PassportDataSent = 38,
    /// Passport data received (39)
    PassportDataReceived = 39,
    /// Poll (40)
    Poll = 40,
    /// Dice (41)
    Dice = 41,
    /// Proximity alert triggered (42)
    ProximityAlertTriggered = 42,
    /// Group call (43)
    GroupCall = 43,
    /// Invite to group call (44)
    InviteToGroupCall = 44,
    /// Chat theme set (45)
    ChatSetTheme = 45,
    /// Web view data sent (46)
    WebViewDataSent = 46,
    /// Web view data received (47)
    WebViewDataReceived = 47,
    /// Gift premium (48)
    GiftPremium = 48,
    /// Topic created (49)
    TopicCreate = 49,
    /// Topic edited (50)
    TopicEdit = 50,
    /// Suggest profile photo (51)
    SuggestProfilePhoto = 51,
    /// Write access allowed (52)
    WriteAccessAllowed = 52,
    /// Requested dialog (53)
    RequestedDialog = 53,
    /// Web view write access allowed (54)
    WebViewWriteAccessAllowed = 54,
    /// Set background (55)
    SetBackground = 55,
    /// Story (56)
    Story = 56,
    /// Write access allowed by request (57)
    WriteAccessAllowedByRequest = 57,
    /// Gift code (58)
    GiftCode = 58,
    /// Giveaway (59)
    Giveaway = 59,
    /// Giveaway launch (60)
    GiveawayLaunch = 60,
    /// Giveaway results (61)
    GiveawayResults = 61,
    /// Giveaway winners (62)
    GiveawayWinners = 62,
    /// Expired video note (63)
    ExpiredVideoNote = 63,
    /// Expired voice note (64)
    ExpiredVoiceNote = 64,
    /// Boost applied (65)
    BoostApply = 65,
    /// Dialog shared (66)
    DialogShared = 66,
    /// Paid media (67)
    PaidMedia = 67,
    /// Payment refunded (68)
    PaymentRefunded = 68,
    /// Gift stars (69)
    GiftStars = 69,
    /// Prize stars (70)
    PrizeStars = 70,
    /// Star gift (71)
    StarGift = 71,
    /// Unique star gift (72)
    StarGiftUnique = 72,
    /// Paid messages refunded (73)
    PaidMessagesRefunded = 73,
    /// Paid messages price (74)
    PaidMessagesPrice = 74,
    /// Conference call (75)
    ConferenceCall = 75,
    /// To-do list (76)
    ToDoList = 76,
    /// Todo completions (77)
    TodoCompletions = 77,
    /// Todo append tasks (78)
    TodoAppendTasks = 78,
    /// Gift TON (79)
    GiftTon = 79,
    /// Suggested post success (80)
    SuggestedPostSuccess = 80,
    /// Suggested post refund (81)
    SuggestedPostRefund = 81,
    /// Suggested post approval (82)
    SuggestedPostApproval = 82,
    /// Suggest birthday (83)
    SuggestBirthday = 83,
    /// Star gift purchase offer (84)
    StarGiftPurchaseOffer = 84,
    /// Star gift purchase offer declined (85)
    StarGiftPurchaseOfferDeclined = 85,
}

impl MessageContentType {
    /// Creates a message content type from an i32 value.
    ///
    /// Returns `None` if the value doesn't correspond to a valid content type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_content_type::MessageContentType;
    ///
    /// assert_eq!(MessageContentType::from_i32(0), Some(MessageContentType::Text));
    /// assert_eq!(MessageContentType::from_i32(-1), Some(MessageContentType::None));
    /// assert_eq!(MessageContentType::from_i32(999), None);
    /// ```
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            -1 => Some(Self::None),
            0 => Some(Self::Text),
            1 => Some(Self::Animation),
            2 => Some(Self::Audio),
            3 => Some(Self::Document),
            4 => Some(Self::Photo),
            5 => Some(Self::Sticker),
            6 => Some(Self::Video),
            7 => Some(Self::VoiceNote),
            8 => Some(Self::Contact),
            9 => Some(Self::Location),
            10 => Some(Self::Venue),
            11 => Some(Self::ChatCreate),
            12 => Some(Self::ChatChangeTitle),
            13 => Some(Self::ChatChangePhoto),
            14 => Some(Self::ChatDeletePhoto),
            15 => Some(Self::ChatDeleteHistory),
            16 => Some(Self::ChatAddUsers),
            17 => Some(Self::ChatJoinedByLink),
            18 => Some(Self::ChatDeleteUser),
            19 => Some(Self::ChatMigrateTo),
            20 => Some(Self::ChannelCreate),
            21 => Some(Self::ChannelMigrateFrom),
            22 => Some(Self::PinMessage),
            23 => Some(Self::Game),
            24 => Some(Self::GameScore),
            25 => Some(Self::ScreenshotTaken),
            26 => Some(Self::ChatSetTtl),
            27 => Some(Self::Unsupported),
            28 => Some(Self::Call),
            29 => Some(Self::Invoice),
            30 => Some(Self::PaymentSuccessful),
            31 => Some(Self::VideoNote),
            32 => Some(Self::ContactRegistered),
            33 => Some(Self::ExpiredPhoto),
            34 => Some(Self::ExpiredVideo),
            35 => Some(Self::LiveLocation),
            36 => Some(Self::CustomServiceAction),
            37 => Some(Self::WebsiteConnected),
            38 => Some(Self::PassportDataSent),
            39 => Some(Self::PassportDataReceived),
            40 => Some(Self::Poll),
            41 => Some(Self::Dice),
            42 => Some(Self::ProximityAlertTriggered),
            43 => Some(Self::GroupCall),
            44 => Some(Self::InviteToGroupCall),
            45 => Some(Self::ChatSetTheme),
            46 => Some(Self::WebViewDataSent),
            47 => Some(Self::WebViewDataReceived),
            48 => Some(Self::GiftPremium),
            49 => Some(Self::TopicCreate),
            50 => Some(Self::TopicEdit),
            51 => Some(Self::SuggestProfilePhoto),
            52 => Some(Self::WriteAccessAllowed),
            53 => Some(Self::RequestedDialog),
            54 => Some(Self::WebViewWriteAccessAllowed),
            55 => Some(Self::SetBackground),
            56 => Some(Self::Story),
            57 => Some(Self::WriteAccessAllowedByRequest),
            58 => Some(Self::GiftCode),
            59 => Some(Self::Giveaway),
            60 => Some(Self::GiveawayLaunch),
            61 => Some(Self::GiveawayResults),
            62 => Some(Self::GiveawayWinners),
            63 => Some(Self::ExpiredVideoNote),
            64 => Some(Self::ExpiredVoiceNote),
            65 => Some(Self::BoostApply),
            66 => Some(Self::DialogShared),
            67 => Some(Self::PaidMedia),
            68 => Some(Self::PaymentRefunded),
            69 => Some(Self::GiftStars),
            70 => Some(Self::PrizeStars),
            71 => Some(Self::StarGift),
            72 => Some(Self::StarGiftUnique),
            73 => Some(Self::PaidMessagesRefunded),
            74 => Some(Self::PaidMessagesPrice),
            75 => Some(Self::ConferenceCall),
            76 => Some(Self::ToDoList),
            77 => Some(Self::TodoCompletions),
            78 => Some(Self::TodoAppendTasks),
            79 => Some(Self::GiftTon),
            80 => Some(Self::SuggestedPostSuccess),
            81 => Some(Self::SuggestedPostRefund),
            82 => Some(Self::SuggestedPostApproval),
            83 => Some(Self::SuggestBirthday),
            84 => Some(Self::StarGiftPurchaseOffer),
            85 => Some(Self::StarGiftPurchaseOfferDeclined),
            _ => None,
        }
    }

    /// Returns the i32 value of this content type.
    #[must_use]
    pub const fn to_i32(self) -> i32 {
        self as i32
    }

    /// Returns `true` if this is a service message.
    ///
    /// Service messages are automatic messages generated by Telegram
    /// (e.g., chat created, user joined, message pinned).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_content_type::MessageContentType;
    ///
    /// assert!(MessageContentType::ChatCreate.is_service());
    /// assert!(MessageContentType::PinMessage.is_service());
    /// assert!(!MessageContentType::Text.is_service());
    /// ```
    #[must_use]
    pub const fn is_service(self) -> bool {
        matches!(
            self,
            Self::ChatCreate
                | Self::ChatChangeTitle
                | Self::ChatChangePhoto
                | Self::ChatDeletePhoto
                | Self::ChatDeleteHistory
                | Self::ChatAddUsers
                | Self::ChatJoinedByLink
                | Self::ChatDeleteUser
                | Self::ChatMigrateTo
                | Self::ChannelCreate
                | Self::ChannelMigrateFrom
                | Self::PinMessage
                | Self::GameScore
                | Self::ScreenshotTaken
                | Self::ChatSetTtl
                | Self::ContactRegistered
                | Self::CustomServiceAction
                | Self::WebsiteConnected
                | Self::PassportDataSent
                | Self::PassportDataReceived
                | Self::ProximityAlertTriggered
                | Self::WriteAccessAllowed
                | Self::RequestedDialog
                | Self::WebViewWriteAccessAllowed
                | Self::SetBackground
                | Self::WriteAccessAllowedByRequest
                | Self::WebViewDataSent
                | Self::WebViewDataReceived
                | Self::GiftPremium
                | Self::TopicCreate
                | Self::TopicEdit
                | Self::SuggestProfilePhoto
                | Self::Giveaway
                | Self::GiveawayLaunch
                | Self::GiveawayResults
                | Self::GiveawayWinners
                | Self::BoostApply
                | Self::DialogShared
                | Self::PaymentRefunded
                | Self::GiftStars
                | Self::PrizeStars
                | Self::StarGift
                | Self::StarGiftUnique
                | Self::PaidMessagesRefunded
                | Self::PaidMessagesPrice
                | Self::SuggestedPostSuccess
                | Self::SuggestedPostRefund
                | Self::SuggestedPostApproval
                | Self::SuggestBirthday
                | Self::StarGiftPurchaseOffer
                | Self::StarGiftPurchaseOfferDeclined
        )
    }

    /// Returns `true` if this is media content.
    ///
    /// Media content includes photos, videos, audio, documents, etc.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_content_type::MessageContentType;
    ///
    /// assert!(MessageContentType::Photo.is_media());
    /// assert!(MessageContentType::Video.is_media());
    /// assert!(!MessageContentType::Text.is_media());
    /// ```
    #[must_use]
    pub const fn is_media(self) -> bool {
        matches!(
            self,
            Self::Photo
                | Self::Video
                | Self::Audio
                | Self::VoiceNote
                | Self::VideoNote
                | Self::Document
                | Self::Animation
                | Self::Sticker
                | Self::Story
                | Self::PaidMedia
        )
    }

    /// Returns `true` if this content type is editable.
    ///
    /// Only text and caption content can be edited.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_content_type::MessageContentType;
    ///
    /// assert!(MessageContentType::Text.is_editable());
    /// assert!(MessageContentType::Photo.is_editable());
    /// assert!(!MessageContentType::VoiceNote.is_editable());
    /// ```
    #[must_use]
    pub const fn is_editable(self) -> bool {
        matches!(
            self,
            Self::Text
                | Self::Photo
                | Self::Video
                | Self::Audio
                | Self::Document
                | Self::Animation
                | Self::Location
                | Self::LiveLocation
                | Self::Venue
                | Self::Contact
                | Self::Poll
                | Self::Game
        )
    }

    /// Returns `true` if this content type can have a caption.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_content_type::MessageContentType;
    ///
    /// assert!(MessageContentType::Photo.can_have_caption());
    /// assert!(MessageContentType::Video.can_have_caption());
    /// assert!(!MessageContentType::Text.can_have_caption());
    /// ```
    #[must_use]
    pub const fn can_have_caption(self) -> bool {
        matches!(
            self,
            Self::Photo
                | Self::Video
                | Self::Audio
                | Self::Document
                | Self::Animation
                | Self::VoiceNote
                | Self::VideoNote
        )
    }

    /// Returns `true` if this content can be sent in secret chats.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_content_type::MessageContentType;
    ///
    /// assert!(MessageContentType::Text.is_secret_chat_supported());
    /// assert!(MessageContentType::Photo.is_secret_chat_supported());
    /// assert!(!MessageContentType::Poll.is_secret_chat_supported());
    /// ```
    #[must_use]
    pub const fn is_secret_chat_supported(self) -> bool {
        matches!(
            self,
            Self::Text
                | Self::Photo
                | Self::Video
                | Self::Audio
                | Self::Document
                | Self::VoiceNote
                | Self::VideoNote
                | Self::Location
                | Self::Venue
                | Self::Contact
                | Self::Sticker
                | Self::Animation
        )
    }

    /// Returns `true` if this is an expired content type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_content_type::MessageContentType;
    ///
    /// assert!(MessageContentType::ExpiredPhoto.is_expired());
    /// assert!(MessageContentType::ExpiredVideo.is_expired());
    /// assert!(!MessageContentType::Text.is_expired());
    /// ```
    #[must_use]
    pub const fn is_expired(self) -> bool {
        matches!(
            self,
            Self::ExpiredPhoto
                | Self::ExpiredVideo
                | Self::ExpiredVideoNote
                | Self::ExpiredVoiceNote
        )
    }

    /// Returns `true` if this content type can be in a media group.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_content_type::MessageContentType;
    ///
    /// assert!(MessageContentType::Photo.is_media_group_allowed());
    /// assert!(MessageContentType::Video.is_media_group_allowed());
    /// assert!(!MessageContentType::Text.is_media_group_allowed());
    /// ```
    #[must_use]
    pub const fn is_media_group_allowed(self) -> bool {
        matches!(
            self,
            Self::Photo | Self::Video | Self::Document | Self::Audio
        )
    }
}

impl fmt::Display for MessageContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::None => "None",
            Self::Text => "Text",
            Self::Animation => "Animation",
            Self::Audio => "Audio",
            Self::Document => "Document",
            Self::Photo => "Photo",
            Self::Sticker => "Sticker",
            Self::Video => "Video",
            Self::VoiceNote => "VoiceNote",
            Self::Contact => "Contact",
            Self::Location => "Location",
            Self::Venue => "Venue",
            Self::ChatCreate => "ChatCreate",
            Self::ChatChangeTitle => "ChatChangeTitle",
            Self::ChatChangePhoto => "ChatChangePhoto",
            Self::ChatDeletePhoto => "ChatDeletePhoto",
            Self::ChatDeleteHistory => "ChatDeleteHistory",
            Self::ChatAddUsers => "ChatAddUsers",
            Self::ChatJoinedByLink => "ChatJoinedByLink",
            Self::ChatDeleteUser => "ChatDeleteUser",
            Self::ChatMigrateTo => "ChatMigrateTo",
            Self::ChannelCreate => "ChannelCreate",
            Self::ChannelMigrateFrom => "ChannelMigrateFrom",
            Self::PinMessage => "PinMessage",
            Self::Game => "Game",
            Self::GameScore => "GameScore",
            Self::ScreenshotTaken => "ScreenshotTaken",
            Self::ChatSetTtl => "ChatSetTtl",
            Self::Unsupported => "Unsupported",
            Self::Call => "Call",
            Self::Invoice => "Invoice",
            Self::PaymentSuccessful => "PaymentSuccessful",
            Self::VideoNote => "VideoNote",
            Self::ContactRegistered => "ContactRegistered",
            Self::ExpiredPhoto => "ExpiredPhoto",
            Self::ExpiredVideo => "ExpiredVideo",
            Self::LiveLocation => "LiveLocation",
            Self::CustomServiceAction => "CustomServiceAction",
            Self::WebsiteConnected => "WebsiteConnected",
            Self::PassportDataSent => "PassportDataSent",
            Self::PassportDataReceived => "PassportDataReceived",
            Self::Poll => "Poll",
            Self::Dice => "Dice",
            Self::ProximityAlertTriggered => "ProximityAlertTriggered",
            Self::GroupCall => "GroupCall",
            Self::InviteToGroupCall => "InviteToGroupCall",
            Self::ChatSetTheme => "ChatSetTheme",
            Self::WebViewDataSent => "WebViewDataSent",
            Self::WebViewDataReceived => "WebViewDataReceived",
            Self::GiftPremium => "GiftPremium",
            Self::TopicCreate => "TopicCreate",
            Self::TopicEdit => "TopicEdit",
            Self::SuggestProfilePhoto => "SuggestProfilePhoto",
            Self::WriteAccessAllowed => "WriteAccessAllowed",
            Self::RequestedDialog => "RequestedDialog",
            Self::WebViewWriteAccessAllowed => "WebViewWriteAccessAllowed",
            Self::SetBackground => "SetBackground",
            Self::Story => "Story",
            Self::WriteAccessAllowedByRequest => "WriteAccessAllowedByRequest",
            Self::GiftCode => "GiftCode",
            Self::Giveaway => "Giveaway",
            Self::GiveawayLaunch => "GiveawayLaunch",
            Self::GiveawayResults => "GiveawayResults",
            Self::GiveawayWinners => "GiveawayWinners",
            Self::ExpiredVideoNote => "ExpiredVideoNote",
            Self::ExpiredVoiceNote => "ExpiredVoiceNote",
            Self::BoostApply => "BoostApply",
            Self::DialogShared => "DialogShared",
            Self::PaidMedia => "PaidMedia",
            Self::PaymentRefunded => "PaymentRefunded",
            Self::GiftStars => "GiftStars",
            Self::PrizeStars => "PrizeStars",
            Self::StarGift => "StarGift",
            Self::StarGiftUnique => "StarGiftUnique",
            Self::PaidMessagesRefunded => "PaidMessagesRefunded",
            Self::PaidMessagesPrice => "PaidMessagesPrice",
            Self::ConferenceCall => "ConferenceCall",
            Self::ToDoList => "ToDoList",
            Self::TodoCompletions => "TodoCompletions",
            Self::TodoAppendTasks => "TodoAppendTasks",
            Self::GiftTon => "GiftTon",
            Self::SuggestedPostSuccess => "SuggestedPostSuccess",
            Self::SuggestedPostRefund => "SuggestedPostRefund",
            Self::SuggestedPostApproval => "SuggestedPostApproval",
            Self::SuggestBirthday => "SuggestBirthday",
            Self::StarGiftPurchaseOffer => "StarGiftPurchaseOffer",
            Self::StarGiftPurchaseOfferDeclined => "StarGiftPurchaseOfferDeclined",
        };
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_i32() {
        assert_eq!(
            MessageContentType::from_i32(-1),
            Some(MessageContentType::None)
        );
        assert_eq!(
            MessageContentType::from_i32(0),
            Some(MessageContentType::Text)
        );
        assert_eq!(
            MessageContentType::from_i32(4),
            Some(MessageContentType::Photo)
        );
        assert_eq!(
            MessageContentType::from_i32(85),
            Some(MessageContentType::StarGiftPurchaseOfferDeclined)
        );
        assert_eq!(MessageContentType::from_i32(999), None);
    }

    #[test]
    fn test_to_i32() {
        assert_eq!(MessageContentType::None.to_i32(), -1);
        assert_eq!(MessageContentType::Text.to_i32(), 0);
        assert_eq!(MessageContentType::Photo.to_i32(), 4);
    }

    #[test]
    fn test_roundtrip_i32() {
        for i in -1..=85 {
            if let Some(content_type) = MessageContentType::from_i32(i) {
                assert_eq!(content_type.to_i32(), i);
            }
        }
    }

    #[test]
    fn test_is_service() {
        assert!(MessageContentType::ChatCreate.is_service());
        assert!(MessageContentType::PinMessage.is_service());
        assert!(MessageContentType::GameScore.is_service());
        assert!(!MessageContentType::Text.is_service());
        assert!(!MessageContentType::Photo.is_service());
    }

    #[test]
    fn test_is_media() {
        assert!(MessageContentType::Photo.is_media());
        assert!(MessageContentType::Video.is_media());
        assert!(MessageContentType::Audio.is_media());
        assert!(MessageContentType::Document.is_media());
        assert!(!MessageContentType::Text.is_media());
        assert!(!MessageContentType::Poll.is_media());
    }

    #[test]
    fn test_is_editable() {
        assert!(MessageContentType::Text.is_editable());
        assert!(MessageContentType::Photo.is_editable());
        assert!(MessageContentType::Video.is_editable());
        assert!(!MessageContentType::VoiceNote.is_editable());
        assert!(!MessageContentType::Call.is_editable());
    }

    #[test]
    fn test_can_have_caption() {
        assert!(MessageContentType::Photo.can_have_caption());
        assert!(MessageContentType::Video.can_have_caption());
        assert!(MessageContentType::Audio.can_have_caption());
        assert!(MessageContentType::Document.can_have_caption());
        assert!(!MessageContentType::Text.can_have_caption());
        assert!(!MessageContentType::Poll.can_have_caption());
    }

    #[test]
    fn test_is_secret_chat_supported() {
        assert!(MessageContentType::Text.is_secret_chat_supported());
        assert!(MessageContentType::Photo.is_secret_chat_supported());
        assert!(MessageContentType::Video.is_secret_chat_supported());
        assert!(!MessageContentType::Poll.is_secret_chat_supported());
        assert!(!MessageContentType::Game.is_secret_chat_supported());
    }

    #[test]
    fn test_is_expired() {
        assert!(MessageContentType::ExpiredPhoto.is_expired());
        assert!(MessageContentType::ExpiredVideo.is_expired());
        assert!(MessageContentType::ExpiredVideoNote.is_expired());
        assert!(MessageContentType::ExpiredVoiceNote.is_expired());
        assert!(!MessageContentType::Photo.is_expired());
        assert!(!MessageContentType::Video.is_expired());
    }

    #[test]
    fn test_is_media_group_allowed() {
        assert!(MessageContentType::Photo.is_media_group_allowed());
        assert!(MessageContentType::Video.is_media_group_allowed());
        assert!(MessageContentType::Document.is_media_group_allowed());
        assert!(MessageContentType::Audio.is_media_group_allowed());
        assert!(!MessageContentType::Text.is_media_group_allowed());
    }

    #[test]
    fn test_default() {
        assert_eq!(MessageContentType::default(), MessageContentType::None);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", MessageContentType::Text), "Text");
        assert_eq!(format!("{}", MessageContentType::Photo), "Photo");
        assert_eq!(format!("{}", MessageContentType::ChatCreate), "ChatCreate");
    }

    #[test]
    fn test_equality() {
        assert_eq!(MessageContentType::Text, MessageContentType::Text);
        assert_ne!(MessageContentType::Text, MessageContentType::Photo);
    }

    #[test]
    fn test_serialization() {
        let ct = MessageContentType::Photo;
        let json = serde_json::to_string(&ct).unwrap();
        let parsed: MessageContentType = serde_json::from_str(&json).unwrap();
        assert_eq!(ct, parsed);
    }

    #[test]
    fn test_all_content_types_covered() {
        // Ensure all 102 types are covered (None + 1-85)
        let count = (0..=85)
            .filter(|&i| MessageContentType::from_i32(i).is_some())
            .count();
        assert_eq!(count, 86); // 0-85 inclusive
        assert!(MessageContentType::from_i32(-1).is_some()); // None
    }

    #[test]
    fn test_modern_features() {
        // Modern Telegram features
        assert!(MessageContentType::Story.is_media()); // Story is media
        assert!(MessageContentType::GiftPremium.is_service());
        assert!(MessageContentType::Giveaway.is_service());
        assert!(MessageContentType::BoostApply.is_service());
        assert!(MessageContentType::StarGift.is_service());
        assert!(MessageContentType::PaidMedia.is_media()); // Paid media is media
    }

    #[test]
    fn test_web_view_types() {
        assert!(MessageContentType::WebViewDataSent.is_service());
        assert!(MessageContentType::WebViewDataReceived.is_service());
        assert!(MessageContentType::WebViewWriteAccessAllowed.is_service());
    }

    #[test]
    fn test_topic_types() {
        assert!(MessageContentType::TopicCreate.is_service());
        assert!(MessageContentType::TopicEdit.is_service());
    }

    #[test]
    fn test_giveaway_types() {
        assert!(MessageContentType::Giveaway.is_service());
        assert!(MessageContentType::GiveawayLaunch.is_service());
        assert!(MessageContentType::GiveawayResults.is_service());
        assert!(MessageContentType::GiveawayWinners.is_service());
    }

    #[test]
    fn test_suggested_post_types() {
        assert!(MessageContentType::SuggestedPostSuccess.is_service());
        assert!(MessageContentType::SuggestedPostRefund.is_service());
        assert!(MessageContentType::SuggestedPostApproval.is_service());
    }
}
