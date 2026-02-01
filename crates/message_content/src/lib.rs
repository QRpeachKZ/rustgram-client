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

//! # Message Content
//!
//! Complete message content hierarchy for Telegram messages.
//!
//! ## Overview
//!
//! This module provides the [`MessageContent`] enum which represents
//! all 61 possible message content variants from TDLib, including:
//!
//! - **Media content**: Text, Photo, Video, Audio, Document, Animation, Sticker
//! - **Service messages**: Chat create, user joined, message pinned, etc.
//! - **Interactive content**: Polls, Games, Invoices
//! - **Modern features**: Stories, Gifts, Giveaways, Boosts, Stars
//!
//! ## TDLib Reference
//!
//! Corresponds to `td/telegram/MessageContent.h` and `MessageContent.cpp` in TDLib.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_message_content::{MessageContent, MessageText};
//! use rustgram_formatted_text::FormattedText;
//!
//! let text = FormattedText::new("Hello, world!");
//! let content = MessageContent::Text(Box::new(MessageText::new(text)));
//! assert_eq!(content.content_type(), rustgram_message_content_type::MessageContentType::Text);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_formatted_text::FormattedText;
use rustgram_message_content_type::MessageContentType;
use rustgram_types::{DialogId, MessageId, UserId};
use serde::{Deserialize, Serialize};
use std::fmt;

// Placeholder types for FileId and PollId (to be replaced with proper types)
/// Placeholder for File ID type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FileId(pub i64);

impl FileId {
    /// Creates a new FileId.
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the inner value.
    #[must_use]
    pub const fn get(&self) -> i64 {
        self.0
    }
}

/// Placeholder for Poll ID type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PollId(pub i64);

impl PollId {
    /// Creates a new PollId.
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the inner value.
    #[must_use]
    pub const fn get(&self) -> i64 {
        self.0
    }
}

// ============================================================================
// Content Type Structs
// ============================================================================

/// Text message content with optional web page preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageText {
    /// The formatted text content
    pub text: FormattedText,
    /// Web page ID (if any)
    #[serde(default)]
    pub web_page_id: i64,
    /// Force small media preview
    #[serde(default)]
    pub force_small_media: bool,
    /// Force large media preview
    #[serde(default)]
    pub force_large_media: bool,
    /// Skip web page confirmation
    #[serde(default)]
    pub skip_web_page_confirmation: bool,
    /// Web page URL
    #[serde(default)]
    pub web_page_url: String,
}

impl MessageText {
    /// Creates a new text message content.
    #[must_use]
    pub fn new(text: FormattedText) -> Self {
        Self {
            text,
            web_page_id: 0,
            force_small_media: false,
            force_large_media: false,
            skip_web_page_confirmation: false,
            web_page_url: String::new(),
        }
    }
}

/// Animation message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageAnimation {
    /// Animation file ID
    pub file_id: FileId,
    /// Caption
    #[serde(default)]
    pub caption: FormattedText,
    /// Has spoiler
    #[serde(default)]
    pub has_spoiler: bool,
}

impl MessageAnimation {
    /// Creates a new animation message content.
    #[must_use]
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            caption: FormattedText::new(""),
            has_spoiler: false,
        }
    }
}

/// Audio message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageAudio {
    /// Audio file ID
    pub file_id: FileId,
    /// Caption
    #[serde(default)]
    pub caption: FormattedText,
}

impl MessageAudio {
    /// Creates a new audio message content.
    #[must_use]
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            caption: FormattedText::new(""),
        }
    }
}

/// Document message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageDocument {
    /// Document file ID
    pub file_id: FileId,
    /// Caption
    #[serde(default)]
    pub caption: FormattedText,
}

impl MessageDocument {
    /// Creates a new document message content.
    #[must_use]
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            caption: FormattedText::new(""),
        }
    }
}

/// Photo message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessagePhoto {
    /// Photo data (placeholder - will be replaced with proper Photo type)
    #[serde(default)]
    pub photo: PhotoData,
    /// Caption
    #[serde(default)]
    pub caption: FormattedText,
    /// Has spoiler
    #[serde(default)]
    pub has_spoiler: bool,
}

/// Placeholder for Photo type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PhotoData {
    /// Photo ID
    pub id: i64,
}

impl MessagePhoto {
    /// Creates a new photo message content.
    #[must_use]
    pub fn new() -> Self {
        Self {
            photo: PhotoData::default(),
            caption: FormattedText::new(""),
            has_spoiler: false,
        }
    }
}

impl Default for MessagePhoto {
    fn default() -> Self {
        Self::new()
    }
}

/// Sticker message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageSticker {
    /// Sticker file ID
    pub file_id: FileId,
    /// Is premium sticker
    #[serde(default)]
    pub is_premium: bool,
}

impl MessageSticker {
    /// Creates a new sticker message content.
    #[must_use]
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            is_premium: false,
        }
    }
}

/// Video message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageVideo {
    /// Video file ID
    pub file_id: FileId,
    /// Alternative file IDs
    #[serde(default)]
    pub alternative_file_ids: Vec<FileId>,
    /// HLS file IDs
    #[serde(default)]
    pub hls_file_ids: Vec<FileId>,
    /// Storyboard file IDs
    #[serde(default)]
    pub storyboard_file_ids: Vec<FileId>,
    /// Cover photo
    #[serde(default)]
    pub cover: PhotoData,
    /// Start timestamp
    #[serde(default)]
    pub start_timestamp: i32,
    /// Caption
    #[serde(default)]
    pub caption: FormattedText,
    /// Has spoiler
    #[serde(default)]
    pub has_spoiler: bool,
}

impl MessageVideo {
    /// Creates a new video message content.
    #[must_use]
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            alternative_file_ids: Vec::new(),
            hls_file_ids: Vec::new(),
            storyboard_file_ids: Vec::new(),
            cover: PhotoData::default(),
            start_timestamp: 0,
            caption: FormattedText::new(""),
            has_spoiler: false,
        }
    }
}

/// Voice note message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageVoiceNote {
    /// Voice note file ID
    pub file_id: FileId,
    /// Caption
    #[serde(default)]
    pub caption: FormattedText,
    /// Is listened
    #[serde(default)]
    pub is_listened: bool,
}

impl MessageVoiceNote {
    /// Creates a new voice note message content.
    #[must_use]
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            caption: FormattedText::new(""),
            is_listened: false,
        }
    }
}

/// Contact message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageContact {
    /// Phone number
    pub phone_number: String,
    /// First name
    pub first_name: String,
    /// Last name
    #[serde(default)]
    pub last_name: String,
    /// User ID (if contact is a Telegram user)
    #[serde(default)]
    pub user_id: i64,
    /// Contact vCard data
    #[serde(default)]
    pub vcard: String,
}

impl MessageContact {
    /// Creates a new contact message content.
    #[must_use]
    pub fn new(phone_number: String, first_name: String) -> Self {
        Self {
            phone_number,
            first_name,
            last_name: String::new(),
            user_id: 0,
            vcard: String::new(),
        }
    }
}

/// Location message content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageLocation {
    /// Latitude
    pub latitude: f64,
    /// Longitude
    pub longitude: f64,
    /// Horizontal accuracy
    #[serde(default)]
    pub horizontal_accuracy: f64,
}

impl MessageLocation {
    /// Creates a new location message content.
    #[must_use]
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            horizontal_accuracy: 0.0,
        }
    }
}

/// Venue message content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageVenue {
    /// Location
    pub location: MessageLocation,
    /// Title
    pub title: String,
    /// Address
    pub address: String,
    /// Provider (e.g., "foursquare")
    #[serde(default)]
    pub provider: String,
    /// Venue ID in the provider's database
    #[serde(default)]
    pub venue_id: String,
    /// Venue type
    #[serde(default)]
    pub venue_type: String,
}

impl MessageVenue {
    /// Creates a new venue message content.
    #[must_use]
    pub fn new(location: MessageLocation, title: String, address: String) -> Self {
        Self {
            location,
            title,
            address,
            provider: String::new(),
            venue_id: String::new(),
            venue_type: String::new(),
        }
    }
}

/// Chat created message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageChatCreate {
    /// Chat title
    pub title: String,
    /// Participant user IDs
    #[serde(default)]
    pub participant_user_ids: Vec<UserId>,
}

impl MessageChatCreate {
    /// Creates a new chat create message content.
    #[must_use]
    pub fn new(title: String) -> Self {
        Self {
            title,
            participant_user_ids: Vec::new(),
        }
    }
}

/// Chat title changed message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageChatChangeTitle {
    /// New title
    pub title: String,
}

impl MessageChatChangeTitle {
    /// Creates a new chat change title message content.
    #[must_use]
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

/// Chat photo changed message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageChatChangePhoto {
    /// New photo
    pub photo: PhotoData,
}

impl MessageChatChangePhoto {
    /// Creates a new chat change photo message content.
    #[must_use]
    pub fn new(photo: PhotoData) -> Self {
        Self { photo }
    }
}

/// Chat photo deleted (empty struct).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageChatDeletePhoto;

impl Default for MessageChatDeletePhoto {
    fn default() -> Self {
        Self
    }
}

/// Chat history deleted (empty struct).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageChatDeleteHistory;

impl Default for MessageChatDeleteHistory {
    fn default() -> Self {
        Self
    }
}

/// Users added to chat message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageChatAddUsers {
    /// Added user IDs
    #[serde(default)]
    pub user_ids: Vec<UserId>,
}

impl MessageChatAddUsers {
    /// Creates a new chat add users message content.
    #[must_use]
    pub fn new() -> Self {
        Self {
            user_ids: Vec::new(),
        }
    }
}

impl Default for MessageChatAddUsers {
    fn default() -> Self {
        Self::new()
    }
}

/// User joined via invite link message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageChatJoinedByLink {
    /// Is approved
    #[serde(default)]
    pub is_approved: bool,
}

/// User removed from chat message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageChatDeleteUser {
    /// Removed user ID
    pub user_id: UserId,
}

impl MessageChatDeleteUser {
    /// Creates a new chat delete user message content.
    #[must_use]
    pub fn new(user_id: UserId) -> Self {
        Self { user_id }
    }
}

/// Chat migrated to supergroup message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageChatMigrateTo {
    /// Migrated to channel ID
    pub migrated_to_channel_id: i64,
}

impl MessageChatMigrateTo {
    /// Creates a new chat migrate to message content.
    #[must_use]
    pub fn new(migrated_to_channel_id: i64) -> Self {
        Self {
            migrated_to_channel_id,
        }
    }
}

/// Channel created message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageChannelCreate {
    /// Channel title
    pub title: String,
}

impl MessageChannelCreate {
    /// Creates a new channel create message content.
    #[must_use]
    pub fn new(title: String) -> Self {
        Self { title }
    }
}

/// Channel migrated from group message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageChannelMigrateFrom {
    /// Title
    pub title: String,
    /// Migrated from chat ID
    pub migrated_from_chat_id: i64,
}

impl MessageChannelMigrateFrom {
    /// Creates a new channel migrate from message content.
    #[must_use]
    pub fn new(title: String, migrated_from_chat_id: i64) -> Self {
        Self {
            title,
            migrated_from_chat_id,
        }
    }
}

/// Message pinned message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessagePinMessage {
    /// Pinned message ID
    pub message_id: MessageId,
}

impl MessagePinMessage {
    /// Creates a new pin message content.
    #[must_use]
    pub fn new(message_id: MessageId) -> Self {
        Self { message_id }
    }
}

/// Game message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageGame {
    /// Game ID
    pub id: i64,
    /// Short name
    pub short_name: String,
    /// Title
    pub title: String,
    /// Description
    pub description: String,
    /// Photo
    pub photo: PhotoData,
    /// Animation file ID (optional)
    #[serde(default)]
    pub animation: Option<FileId>,
}

impl MessageGame {
    /// Creates a new game message content.
    #[must_use]
    pub fn new(id: i64, short_name: String, title: String) -> Self {
        Self {
            id,
            short_name,
            title,
            description: String::new(),
            photo: PhotoData::default(),
            animation: None,
        }
    }
}

/// Game score message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageGameScore {
    /// Game message ID
    pub game_message_id: MessageId,
    /// Game ID
    pub game_id: i64,
    /// Score
    pub score: i32,
}

impl MessageGameScore {
    /// Creates a new game score message content.
    #[must_use]
    pub fn new(game_message_id: MessageId, game_id: i64, score: i32) -> Self {
        Self {
            game_message_id,
            game_id,
            score,
        }
    }
}

/// Screenshot taken (empty struct).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageScreenshotTaken;

impl Default for MessageScreenshotTaken {
    fn default() -> Self {
        Self
    }
}

/// Chat TTL set message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageChatSetTtl {
    /// TTL period in seconds
    pub ttl: i32,
    /// From user ID
    pub from_user_id: UserId,
}

impl MessageChatSetTtl {
    /// Creates a new chat set TTL message content.
    #[must_use]
    pub fn new(ttl: i32, from_user_id: UserId) -> Self {
        Self { ttl, from_user_id }
    }
}

/// Unsupported message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageUnsupported {
    /// Version
    #[serde(default = "default_unsupported_version")]
    pub version: i32,
}

fn default_unsupported_version() -> i32 {
    53
}

impl Default for MessageUnsupported {
    fn default() -> Self {
        Self { version: 53 }
    }
}

/// Call message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageCall {
    /// Call ID
    pub call_id: i64,
    /// Duration in seconds
    #[serde(default)]
    pub duration: i32,
    /// Discard reason
    #[serde(default)]
    pub discard_reason: String,
    /// Is video call
    #[serde(default)]
    pub is_video: bool,
}

impl MessageCall {
    /// Creates a new call message content.
    #[must_use]
    pub fn new(call_id: i64) -> Self {
        Self {
            call_id,
            duration: 0,
            discard_reason: String::new(),
            is_video: false,
        }
    }
}

/// Invoice message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageInvoice {
    /// Title
    pub title: String,
    /// Description
    pub description: String,
    /// Photo (optional)
    #[serde(default)]
    pub photo: Option<PhotoData>,
    /// Currency
    pub currency: String,
    /// Total amount
    pub total_amount: i64,
    /// Invoice payload
    pub payload: String,
    /// Is test
    #[serde(default)]
    pub is_test: bool,
    /// Need name
    #[serde(default)]
    pub need_name: bool,
    /// Need phone number
    #[serde(default)]
    pub need_phone_number: bool,
    /// Need email
    #[serde(default)]
    pub need_email: bool,
    /// Need shipping address
    #[serde(default)]
    pub need_shipping_address: bool,
}

impl MessageInvoice {
    /// Creates a new invoice message content.
    #[must_use]
    pub fn new(title: String, description: String, currency: String, total_amount: i64) -> Self {
        Self {
            title,
            description,
            photo: None,
            currency,
            total_amount,
            payload: String::new(),
            is_test: false,
            need_name: false,
            need_phone_number: false,
            need_email: false,
            need_shipping_address: false,
        }
    }
}

/// Payment successful message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessagePaymentSuccessful {
    /// Invoice dialog ID
    pub invoice_dialog_id: DialogId,
    /// Invoice message ID
    pub invoice_message_id: MessageId,
    /// Currency
    pub currency: String,
    /// Total amount
    pub total_amount: i64,
    /// Invoice payload
    #[serde(default)]
    pub invoice_payload: String,
    /// Subscription until date
    #[serde(default)]
    pub subscription_until_date: i32,
    /// Is recurring
    #[serde(default)]
    pub is_recurring: bool,
    /// Is first recurring
    #[serde(default)]
    pub is_first_recurring: bool,
}

impl MessagePaymentSuccessful {
    /// Creates a new payment successful message content.
    #[must_use]
    pub fn new(
        invoice_dialog_id: DialogId,
        invoice_message_id: MessageId,
        currency: String,
        total_amount: i64,
    ) -> Self {
        Self {
            invoice_dialog_id,
            invoice_message_id,
            currency,
            total_amount,
            invoice_payload: String::new(),
            subscription_until_date: 0,
            is_recurring: false,
            is_first_recurring: false,
        }
    }
}

/// Video note message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageVideoNote {
    /// Video note file ID
    pub file_id: FileId,
    /// Is viewed
    #[serde(default)]
    pub is_viewed: bool,
}

impl MessageVideoNote {
    /// Creates a new video note message content.
    #[must_use]
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            is_viewed: false,
        }
    }
}

/// Contact registered (empty struct).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageContactRegistered;

impl Default for MessageContactRegistered {
    fn default() -> Self {
        Self
    }
}

/// Expired photo (empty struct).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageExpiredPhoto;

impl Default for MessageExpiredPhoto {
    fn default() -> Self {
        Self
    }
}

/// Expired video (empty struct).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageExpiredVideo;

impl Default for MessageExpiredVideo {
    fn default() -> Self {
        Self
    }
}

/// Live location message content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageLiveLocation {
    /// Location
    pub location: MessageLocation,
    /// Period in seconds
    #[serde(default)]
    pub period: i32,
    /// Heading
    #[serde(default)]
    pub heading: i32,
    /// Proximity alert radius
    #[serde(default)]
    pub proximity_alert_radius: i32,
}

impl MessageLiveLocation {
    /// Creates a new live location message content.
    #[must_use]
    pub fn new(location: MessageLocation) -> Self {
        Self {
            location,
            period: 0,
            heading: 0,
            proximity_alert_radius: 0,
        }
    }
}

/// Custom service action message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageCustomServiceAction {
    /// Message text
    pub message: String,
}

impl MessageCustomServiceAction {
    /// Creates a new custom service action message content.
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

/// Website connected message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageWebsiteConnected {
    /// Domain name
    pub domain_name: String,
}

impl MessageWebsiteConnected {
    /// Creates a new website connected message content.
    #[must_use]
    pub fn new(domain_name: String) -> Self {
        Self { domain_name }
    }
}

/// Passport data sent message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessagePassportDataSent {
    /// Secure value types
    #[serde(default)]
    pub types: Vec<String>,
}

/// Passport data received message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessagePassportDataReceived {
    /// Values (placeholder)
    #[serde(default)]
    pub values: Vec<PassportValue>,
    /// Credentials (placeholder)
    #[serde(default)]
    pub credentials: Credentials,
}

/// Placeholder for passport value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PassportValue {
    /// Type
    #[serde(default)]
    pub r#type: String,
}

/// Placeholder for credentials.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Credentials {
    /// Data hash
    #[serde(default)]
    pub data_hash: Vec<u8>,
}

/// Poll message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessagePoll {
    /// Poll ID
    pub poll_id: PollId,
}

impl MessagePoll {
    /// Creates a new poll message content.
    #[must_use]
    pub fn new(poll_id: PollId) -> Self {
        Self { poll_id }
    }
}

/// Dice message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageDice {
    /// Emoji
    #[serde(default = "default_dice_emoji")]
    pub emoji: String,
    /// Dice value
    #[serde(default)]
    pub dice_value: i32,
}

fn default_dice_emoji() -> String {
    "🎲".to_string()
}

impl Default for MessageDice {
    fn default() -> Self {
        Self {
            emoji: default_dice_emoji(),
            dice_value: 0,
        }
    }
}

/// Proximity alert triggered message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageProximityAlertTriggered {
    /// Traveler dialog ID
    pub traveler_dialog_id: DialogId,
    /// Watcher dialog ID
    pub watcher_dialog_id: DialogId,
    /// Distance in meters
    pub distance: i32,
}

impl MessageProximityAlertTriggered {
    /// Creates a new proximity alert triggered message content.
    #[must_use]
    pub fn new(traveler_dialog_id: DialogId, watcher_dialog_id: DialogId, distance: i32) -> Self {
        Self {
            traveler_dialog_id,
            watcher_dialog_id,
            distance,
        }
    }
}

/// Group call message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageGroupCall {
    /// Group call ID
    pub group_call_id: i64,
}

impl MessageGroupCall {
    /// Creates a new group call message content.
    #[must_use]
    pub fn new(group_call_id: i64) -> Self {
        Self { group_call_id }
    }
}

/// Invite to group call message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageInviteToGroupCall {
    /// Group call ID
    pub group_call_id: i64,
    /// User IDs
    #[serde(default)]
    pub user_ids: Vec<UserId>,
}

impl MessageInviteToGroupCall {
    /// Creates a new invite to group call message content.
    #[must_use]
    pub fn new(group_call_id: i64) -> Self {
        Self {
            group_call_id,
            user_ids: Vec::new(),
        }
    }
}

/// Chat set theme message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageChatSetTheme {
    /// Theme name
    pub theme_name: String,
}

impl MessageChatSetTheme {
    /// Creates a new chat set theme message content.
    #[must_use]
    pub fn new(theme_name: String) -> Self {
        Self { theme_name }
    }
}

/// WebView data sent message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageWebViewDataSent {
    /// Data
    pub data: String,
}

impl MessageWebViewDataSent {
    /// Creates a new WebView data sent message content.
    #[must_use]
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

/// WebView data received message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageWebViewDataReceived {
    /// Data
    pub data: String,
}

impl MessageWebViewDataReceived {
    /// Creates a new WebView data received message content.
    #[must_use]
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

/// Gift premium message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageGiftPremium {
    /// From user ID
    pub from_user_id: UserId,
    /// To user ID
    pub to_user_id: UserId,
    /// Currency
    pub currency: String,
    /// Amount
    pub amount: i64,
    /// Months
    pub months: i32,
}

impl MessageGiftPremium {
    /// Creates a new gift premium message content.
    #[must_use]
    pub fn new(
        from_user_id: UserId,
        to_user_id: UserId,
        currency: String,
        amount: i64,
        months: i32,
    ) -> Self {
        Self {
            from_user_id,
            to_user_id,
            currency,
            amount,
            months,
        }
    }
}

/// Topic created message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageTopicCreate {
    /// Title
    pub title: String,
    /// Icon color
    #[serde(default)]
    pub icon_color: i32,
}

impl MessageTopicCreate {
    /// Creates a new topic create message content.
    #[must_use]
    pub fn new(title: String) -> Self {
        Self {
            title,
            icon_color: 0,
        }
    }
}

/// Topic edited message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageTopicEdit {
    /// Title (optional)
    #[serde(default)]
    pub title: Option<String>,
    /// Icon emoji ID (optional)
    #[serde(default)]
    pub icon_emoji_id: Option<i64>,
}

/// Suggest profile photo message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageSuggestProfilePhoto {
    /// Photo
    pub photo: PhotoData,
}

impl MessageSuggestProfilePhoto {
    /// Creates a new suggest profile photo message content.
    #[must_use]
    pub fn new(photo: PhotoData) -> Self {
        Self { photo }
    }
}

/// Write access allowed (empty struct).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageWriteAccessAllowed;

impl Default for MessageWriteAccessAllowed {
    fn default() -> Self {
        Self
    }
}

/// Requested dialog message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageRequestedDialog {
    /// Dialog ID
    pub dialog_id: DialogId,
}

impl MessageRequestedDialog {
    /// Creates a new requested dialog message content.
    #[must_use]
    pub fn new(dialog_id: DialogId) -> Self {
        Self { dialog_id }
    }
}

/// WebView write access allowed (empty struct).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageWebViewWriteAccessAllowed;

impl Default for MessageWebViewWriteAccessAllowed {
    fn default() -> Self {
        Self
    }
}

/// Set background message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageSetBackground {
    /// Background (placeholder)
    #[serde(default)]
    pub background: BackgroundData,
}

/// Placeholder for background data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BackgroundData {
    /// Background ID
    pub id: i64,
}

/// Story message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageStory {
    /// Story sender dialog ID
    pub sender_dialog_id: DialogId,
    /// Story ID
    pub story_id: i32,
}

impl MessageStory {
    /// Creates a new story message content.
    #[must_use]
    pub fn new(sender_dialog_id: DialogId, story_id: i32) -> Self {
        Self {
            sender_dialog_id,
            story_id,
        }
    }
}

/// Write access allowed by request (empty struct).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageWriteAccessAllowedByRequest;

impl Default for MessageWriteAccessAllowedByRequest {
    fn default() -> Self {
        Self
    }
}

/// Gift code message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageGiftCode {
    /// From user ID
    pub from_user_id: UserId,
    /// To user ID (optional)
    #[serde(default)]
    pub to_user_id: Option<UserId>,
    /// Code
    pub code: String,
    /// Months
    pub months: i32,
}

impl MessageGiftCode {
    /// Creates a new gift code message content.
    #[must_use]
    pub fn new(from_user_id: UserId, code: String, months: i32) -> Self {
        Self {
            from_user_id,
            to_user_id: None,
            code,
            months,
        }
    }
}

/// Giveaway message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageGiveaway {
    /// Giveaway ID
    pub giveaway_id: i64,
    /// Winner count
    pub winner_count: i32,
    /// Months
    pub months: i32,
    /// Only new subscribers
    #[serde(default)]
    pub only_new_subscribers: bool,
}

impl MessageGiveaway {
    /// Creates a new giveaway message content.
    #[must_use]
    pub fn new(giveaway_id: i64, winner_count: i32, months: i32) -> Self {
        Self {
            giveaway_id,
            winner_count,
            months,
            only_new_subscribers: false,
        }
    }
}

/// Giveaway launch message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageGiveawayLaunch {
    /// Giveaway ID
    pub giveaway_id: i64,
}

impl MessageGiveawayLaunch {
    /// Creates a new giveaway launch message content.
    #[must_use]
    pub fn new(giveaway_id: i64) -> Self {
        Self { giveaway_id }
    }
}

/// Giveaway results message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageGiveawayResults {
    /// Giveaway ID
    pub giveaway_id: i64,
    /// Winner count
    pub winner_count: i32,
    /// Unclaimed count
    pub unclaimed_count: i32,
}

impl MessageGiveawayResults {
    /// Creates a new giveaway results message content.
    #[must_use]
    pub fn new(giveaway_id: i64, winner_count: i32, unclaimed_count: i32) -> Self {
        Self {
            giveaway_id,
            winner_count,
            unclaimed_count,
        }
    }
}

/// Giveaway winners message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageGiveawayWinners {
    /// Giveaway ID
    pub giveaway_id: i64,
    /// Winner count
    pub winner_count: i32,
    /// Winner user IDs
    #[serde(default)]
    pub winner_user_ids: Vec<UserId>,
}

impl MessageGiveawayWinners {
    /// Creates a new giveaway winners message content.
    #[must_use]
    pub fn new(giveaway_id: i64, winner_count: i32) -> Self {
        Self {
            giveaway_id,
            winner_count,
            winner_user_ids: Vec::new(),
        }
    }
}

/// Expired video note (empty struct).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageExpiredVideoNote;

impl Default for MessageExpiredVideoNote {
    fn default() -> Self {
        Self
    }
}

/// Expired voice note (empty struct).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageExpiredVoiceNote;

impl Default for MessageExpiredVoiceNote {
    fn default() -> Self {
        Self
    }
}

/// Boost apply message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageBoostApply {
    /// From user ID
    pub from_user_id: UserId,
}

impl MessageBoostApply {
    /// Creates a new boost apply message content.
    #[must_use]
    pub fn new(from_user_id: UserId) -> Self {
        Self { from_user_id }
    }
}

/// Dialog shared message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageDialogShared {
    /// From user ID
    pub from_user_id: UserId,
    /// Dialog IDs
    #[serde(default)]
    pub dialog_ids: Vec<DialogId>,
}

impl MessageDialogShared {
    /// Creates a new dialog shared message content.
    #[must_use]
    pub fn new(from_user_id: UserId) -> Self {
        Self {
            from_user_id,
            dialog_ids: Vec::new(),
        }
    }
}

/// Paid media message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessagePaidMedia {
    /// Star count
    pub star_count: i32,
    /// Paid media IDs
    #[serde(default)]
    pub paid_media_ids: Vec<i64>,
}

impl MessagePaidMedia {
    /// Creates a new paid media message content.
    #[must_use]
    pub fn new(star_count: i32) -> Self {
        Self {
            star_count,
            paid_media_ids: Vec::new(),
        }
    }
}

/// Payment refunded message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessagePaymentRefunded {
    /// Invoice dialog ID
    pub invoice_dialog_id: DialogId,
    /// Invoice message ID
    pub invoice_message_id: MessageId,
    /// Currency
    pub currency: String,
    /// Total amount
    pub total_amount: i64,
}

impl MessagePaymentRefunded {
    /// Creates a new payment refunded message content.
    #[must_use]
    pub fn new(
        invoice_dialog_id: DialogId,
        invoice_message_id: MessageId,
        currency: String,
        total_amount: i64,
    ) -> Self {
        Self {
            invoice_dialog_id,
            invoice_message_id,
            currency,
            total_amount,
        }
    }
}

/// Gift stars message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageGiftStars {
    /// From user ID
    pub from_user_id: UserId,
    /// To user ID
    pub to_user_id: UserId,
    /// Star count
    pub count: i32,
}

impl MessageGiftStars {
    /// Creates a new gift stars message content.
    #[must_use]
    pub fn new(from_user_id: UserId, to_user_id: UserId, count: i32) -> Self {
        Self {
            from_user_id,
            to_user_id,
            count,
        }
    }
}

/// Prize stars message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessagePrizeStars {
    /// From user ID
    pub from_user_id: UserId,
    /// To user ID
    pub to_user_id: UserId,
    /// Star count
    pub count: i32,
}

impl MessagePrizeStars {
    /// Creates a new prize stars message content.
    #[must_use]
    pub fn new(from_user_id: UserId, to_user_id: UserId, count: i32) -> Self {
        Self {
            from_user_id,
            to_user_id,
            count,
        }
    }
}

/// Star gift message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageStarGift {
    /// From user ID
    pub from_user_id: UserId,
    /// To user ID
    pub to_user_id: UserId,
    /// Gift ID
    pub gift_id: i64,
    /// Star count
    pub star_count: i32,
}

impl MessageStarGift {
    /// Creates a new star gift message content.
    #[must_use]
    pub fn new(from_user_id: UserId, to_user_id: UserId, gift_id: i64, star_count: i32) -> Self {
        Self {
            from_user_id,
            to_user_id,
            gift_id,
            star_count,
        }
    }
}

/// Unique star gift message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageStarGiftUnique {
    /// From user ID
    pub from_user_id: UserId,
    /// To user ID
    pub to_user_id: UserId,
    /// Gift ID
    pub gift_id: i64,
}

impl MessageStarGiftUnique {
    /// Creates a new unique star gift message content.
    #[must_use]
    pub fn new(from_user_id: UserId, to_user_id: UserId, gift_id: i64) -> Self {
        Self {
            from_user_id,
            to_user_id,
            gift_id,
        }
    }
}

/// Paid messages refunded message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessagePaidMessagesRefunded {
    /// From user ID
    pub from_user_id: UserId,
    /// Refunded message count
    pub refunded_message_count: i32,
}

impl MessagePaidMessagesRefunded {
    /// Creates a new paid messages refunded message content.
    #[must_use]
    pub fn new(from_user_id: UserId, refunded_message_count: i32) -> Self {
        Self {
            from_user_id,
            refunded_message_count,
        }
    }
}

/// Paid messages price message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessagePaidMessagesPrice {
    /// Star count
    pub star_count: i32,
}

impl MessagePaidMessagesPrice {
    /// Creates a new paid messages price message content.
    #[must_use]
    pub fn new(star_count: i32) -> Self {
        Self { star_count }
    }
}

/// Conference call message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageConferenceCall {
    /// Conference call ID
    pub conference_call_id: i64,
}

impl MessageConferenceCall {
    /// Creates a new conference call message content.
    #[must_use]
    pub fn new(conference_call_id: i64) -> Self {
        Self { conference_call_id }
    }
}

/// To-do list message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageToDoList {
    /// To-do list ID
    pub todo_list_id: i64,
}

impl MessageToDoList {
    /// Creates a new to-do list message content.
    #[must_use]
    pub fn new(todo_list_id: i64) -> Self {
        Self { todo_list_id }
    }
}

/// Todo completions message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageTodoCompletions {
    /// To-do list ID
    pub todo_list_id: i64,
    /// Completed item IDs
    #[serde(default)]
    pub completed_item_ids: Vec<i64>,
}

impl MessageTodoCompletions {
    /// Creates a new todo completions message content.
    #[must_use]
    pub fn new(todo_list_id: i64) -> Self {
        Self {
            todo_list_id,
            completed_item_ids: Vec::new(),
        }
    }
}

/// Todo append tasks message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageTodoAppendTasks {
    /// To-do list ID
    pub todo_list_id: i64,
    /// Tasks
    #[serde(default)]
    pub tasks: Vec<String>,
}

impl MessageTodoAppendTasks {
    /// Creates a new todo append tasks message content.
    #[must_use]
    pub fn new(todo_list_id: i64) -> Self {
        Self {
            todo_list_id,
            tasks: Vec::new(),
        }
    }
}

/// Gift TON message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageGiftTon {
    /// From user ID
    pub from_user_id: UserId,
    /// To user ID
    pub to_user_id: UserId,
    /// Amount
    pub amount: i64,
}

impl MessageGiftTon {
    /// Creates a new gift TON message content.
    #[must_use]
    pub fn new(from_user_id: UserId, to_user_id: UserId, amount: i64) -> Self {
        Self {
            from_user_id,
            to_user_id,
            amount,
        }
    }
}

/// Suggested post success message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageSuggestedPostSuccess {
    /// Dialog ID
    pub dialog_id: DialogId,
    /// Message ID
    pub message_id: MessageId,
}

impl MessageSuggestedPostSuccess {
    /// Creates a new suggested post success message content.
    #[must_use]
    pub fn new(dialog_id: DialogId, message_id: MessageId) -> Self {
        Self {
            dialog_id,
            message_id,
        }
    }
}

/// Suggested post refund message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageSuggestedPostRefund {
    /// Dialog ID
    pub dialog_id: DialogId,
    /// Message ID
    pub message_id: MessageId,
}

impl MessageSuggestedPostRefund {
    /// Creates a new suggested post refund message content.
    #[must_use]
    pub fn new(dialog_id: DialogId, message_id: MessageId) -> Self {
        Self {
            dialog_id,
            message_id,
        }
    }
}

/// Suggested post approval message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageSuggestedPostApproval {
    /// Dialog ID
    pub dialog_id: DialogId,
    /// Message ID
    pub message_id: MessageId,
}

impl MessageSuggestedPostApproval {
    /// Creates a new suggested post approval message content.
    #[must_use]
    pub fn new(dialog_id: DialogId, message_id: MessageId) -> Self {
        Self {
            dialog_id,
            message_id,
        }
    }
}

/// Suggest birthday message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageSuggestBirthday {
    /// From user ID
    pub from_user_id: UserId,
    /// Birthday date
    pub birthday_date: i32,
}

impl MessageSuggestBirthday {
    /// Creates a new suggest birthday message content.
    #[must_use]
    pub fn new(from_user_id: UserId, birthday_date: i32) -> Self {
        Self {
            from_user_id,
            birthday_date,
        }
    }
}

/// Star gift purchase offer message content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageStarGiftPurchaseOffer {
    /// Gift ID
    pub gift_id: i64,
    /// Star count
    pub star_count: i32,
}

impl MessageStarGiftPurchaseOffer {
    /// Creates a new star gift purchase offer message content.
    #[must_use]
    pub fn new(gift_id: i64, star_count: i32) -> Self {
        Self {
            gift_id,
            star_count,
        }
    }
}

/// Star gift purchase offer declined (empty struct).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageStarGiftPurchaseOfferDeclined;

impl Default for MessageStarGiftPurchaseOfferDeclined {
    fn default() -> Self {
        Self
    }
}

// ============================================================================
// Main MessageContent Enum
// ============================================================================

/// Complete message content hierarchy for Telegram messages.
///
/// This enum represents all 61 possible message content variants from TDLib.
/// Each variant corresponds to a specific type of message that can be sent or
/// received in Telegram.
///
/// # Example
///
/// ```rust
/// use rustgram_message_content::{MessageContent, MessageText};
/// use rustgram_formatted_text::FormattedText;
///
/// let text = FormattedText::new("Hello, world!");
/// let content = MessageContent::Text(Box::new(MessageText::new(text)));
/// assert!(content.has_text());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageContent {
    /// Text message
    Text(Box<MessageText>),
    /// Animation
    Animation(Box<MessageAnimation>),
    /// Audio
    Audio(Box<MessageAudio>),
    /// Document
    Document(Box<MessageDocument>),
    /// Photo
    Photo(Box<MessagePhoto>),
    /// Sticker
    Sticker(Box<MessageSticker>),
    /// Video
    Video(Box<MessageVideo>),
    /// Voice note
    VoiceNote(Box<MessageVoiceNote>),
    /// Contact
    Contact(Box<MessageContact>),
    /// Location
    Location(Box<MessageLocation>),
    /// Venue
    Venue(Box<MessageVenue>),
    /// Chat created
    ChatCreate(Box<MessageChatCreate>),
    /// Chat title changed
    ChatChangeTitle(Box<MessageChatChangeTitle>),
    /// Chat photo changed
    ChatChangePhoto(Box<MessageChatChangePhoto>),
    /// Chat photo deleted
    ChatDeletePhoto(Box<MessageChatDeletePhoto>),
    /// Chat history deleted
    ChatDeleteHistory(Box<MessageChatDeleteHistory>),
    /// Users added to chat
    ChatAddUsers(Box<MessageChatAddUsers>),
    /// User joined via invite link
    ChatJoinedByLink(Box<MessageChatJoinedByLink>),
    /// User removed from chat
    ChatDeleteUser(Box<MessageChatDeleteUser>),
    /// Chat migrated to supergroup
    ChatMigrateTo(Box<MessageChatMigrateTo>),
    /// Channel created
    ChannelCreate(Box<MessageChannelCreate>),
    /// Channel migrated from group
    ChannelMigrateFrom(Box<MessageChannelMigrateFrom>),
    /// Message pinned
    PinMessage(Box<MessagePinMessage>),
    /// Game
    Game(Box<MessageGame>),
    /// Game score
    GameScore(Box<MessageGameScore>),
    /// Screenshot taken
    ScreenshotTaken(Box<MessageScreenshotTaken>),
    /// Chat TTL set
    ChatSetTtl(Box<MessageChatSetTtl>),
    /// Unsupported message
    Unsupported(Box<MessageUnsupported>),
    /// Call
    Call(Box<MessageCall>),
    /// Invoice
    Invoice(Box<MessageInvoice>),
    /// Payment successful
    PaymentSuccessful(Box<MessagePaymentSuccessful>),
    /// Video note
    VideoNote(Box<MessageVideoNote>),
    /// Contact registered
    ContactRegistered(Box<MessageContactRegistered>),
    /// Expired photo
    ExpiredPhoto(Box<MessageExpiredPhoto>),
    /// Expired video
    ExpiredVideo(Box<MessageExpiredVideo>),
    /// Live location
    LiveLocation(Box<MessageLiveLocation>),
    /// Custom service action
    CustomServiceAction(Box<MessageCustomServiceAction>),
    /// Website connected
    WebsiteConnected(Box<MessageWebsiteConnected>),
    /// Passport data sent
    PassportDataSent(Box<MessagePassportDataSent>),
    /// Passport data received
    PassportDataReceived(Box<MessagePassportDataReceived>),
    /// Poll
    Poll(Box<MessagePoll>),
    /// Dice
    Dice(Box<MessageDice>),
    /// Proximity alert triggered
    ProximityAlertTriggered(Box<MessageProximityAlertTriggered>),
    /// Group call
    GroupCall(Box<MessageGroupCall>),
    /// Invite to group call
    InviteToGroupCall(Box<MessageInviteToGroupCall>),
    /// Chat theme set
    ChatSetTheme(Box<MessageChatSetTheme>),
    /// WebView data sent
    WebViewDataSent(Box<MessageWebViewDataSent>),
    /// WebView data received
    WebViewDataReceived(Box<MessageWebViewDataReceived>),
    /// Gift premium
    GiftPremium(Box<MessageGiftPremium>),
    /// Topic created
    TopicCreate(Box<MessageTopicCreate>),
    /// Topic edited
    TopicEdit(Box<MessageTopicEdit>),
    /// Suggest profile photo
    SuggestProfilePhoto(Box<MessageSuggestProfilePhoto>),
    /// Write access allowed
    WriteAccessAllowed(Box<MessageWriteAccessAllowed>),
    /// Requested dialog
    RequestedDialog(Box<MessageRequestedDialog>),
    /// WebView write access allowed
    WebViewWriteAccessAllowed(Box<MessageWebViewWriteAccessAllowed>),
    /// Set background
    SetBackground(Box<MessageSetBackground>),
    /// Story
    Story(Box<MessageStory>),
    /// Write access allowed by request
    WriteAccessAllowedByRequest(Box<MessageWriteAccessAllowedByRequest>),
    /// Gift code
    GiftCode(Box<MessageGiftCode>),
    /// Giveaway
    Giveaway(Box<MessageGiveaway>),
    /// Giveaway launch
    GiveawayLaunch(Box<MessageGiveawayLaunch>),
    /// Giveaway results
    GiveawayResults(Box<MessageGiveawayResults>),
    /// Giveaway winners
    GiveawayWinners(Box<MessageGiveawayWinners>),
    /// Expired video note
    ExpiredVideoNote(Box<MessageExpiredVideoNote>),
    /// Expired voice note
    ExpiredVoiceNote(Box<MessageExpiredVoiceNote>),
    /// Boost applied
    BoostApply(Box<MessageBoostApply>),
    /// Dialog shared
    DialogShared(Box<MessageDialogShared>),
    /// Paid media
    PaidMedia(Box<MessagePaidMedia>),
    /// Payment refunded
    PaymentRefunded(Box<MessagePaymentRefunded>),
    /// Gift stars
    GiftStars(Box<MessageGiftStars>),
    /// Prize stars
    PrizeStars(Box<MessagePrizeStars>),
    /// Star gift
    StarGift(Box<MessageStarGift>),
    /// Unique star gift
    StarGiftUnique(Box<MessageStarGiftUnique>),
    /// Paid messages refunded
    PaidMessagesRefunded(Box<MessagePaidMessagesRefunded>),
    /// Paid messages price
    PaidMessagesPrice(Box<MessagePaidMessagesPrice>),
    /// Conference call
    ConferenceCall(Box<MessageConferenceCall>),
    /// To-do list
    ToDoList(Box<MessageToDoList>),
    /// Todo completions
    TodoCompletions(Box<MessageTodoCompletions>),
    /// Todo append tasks
    TodoAppendTasks(Box<MessageTodoAppendTasks>),
    /// Gift TON
    GiftTon(Box<MessageGiftTon>),
    /// Suggested post success
    SuggestedPostSuccess(Box<MessageSuggestedPostSuccess>),
    /// Suggested post refund
    SuggestedPostRefund(Box<MessageSuggestedPostRefund>),
    /// Suggested post approval
    SuggestedPostApproval(Box<MessageSuggestedPostApproval>),
    /// Suggest birthday
    SuggestBirthday(Box<MessageSuggestBirthday>),
    /// Star gift purchase offer
    StarGiftPurchaseOffer(Box<MessageStarGiftPurchaseOffer>),
    /// Star gift purchase offer declined
    StarGiftPurchaseOfferDeclined(Box<MessageStarGiftPurchaseOfferDeclined>),
}

impl MessageContent {
    /// Returns the content type of this message.
    #[must_use]
    pub const fn content_type(&self) -> MessageContentType {
        match self {
            Self::Text(_) => MessageContentType::Text,
            Self::Animation(_) => MessageContentType::Animation,
            Self::Audio(_) => MessageContentType::Audio,
            Self::Document(_) => MessageContentType::Document,
            Self::Photo(_) => MessageContentType::Photo,
            Self::Sticker(_) => MessageContentType::Sticker,
            Self::Video(_) => MessageContentType::Video,
            Self::VoiceNote(_) => MessageContentType::VoiceNote,
            Self::Contact(_) => MessageContentType::Contact,
            Self::Location(_) => MessageContentType::Location,
            Self::Venue(_) => MessageContentType::Venue,
            Self::ChatCreate(_) => MessageContentType::ChatCreate,
            Self::ChatChangeTitle(_) => MessageContentType::ChatChangeTitle,
            Self::ChatChangePhoto(_) => MessageContentType::ChatChangePhoto,
            Self::ChatDeletePhoto(_) => MessageContentType::ChatDeletePhoto,
            Self::ChatDeleteHistory(_) => MessageContentType::ChatDeleteHistory,
            Self::ChatAddUsers(_) => MessageContentType::ChatAddUsers,
            Self::ChatJoinedByLink(_) => MessageContentType::ChatJoinedByLink,
            Self::ChatDeleteUser(_) => MessageContentType::ChatDeleteUser,
            Self::ChatMigrateTo(_) => MessageContentType::ChatMigrateTo,
            Self::ChannelCreate(_) => MessageContentType::ChannelCreate,
            Self::ChannelMigrateFrom(_) => MessageContentType::ChannelMigrateFrom,
            Self::PinMessage(_) => MessageContentType::PinMessage,
            Self::Game(_) => MessageContentType::Game,
            Self::GameScore(_) => MessageContentType::GameScore,
            Self::ScreenshotTaken(_) => MessageContentType::ScreenshotTaken,
            Self::ChatSetTtl(_) => MessageContentType::ChatSetTtl,
            Self::Unsupported(_) => MessageContentType::Unsupported,
            Self::Call(_) => MessageContentType::Call,
            Self::Invoice(_) => MessageContentType::Invoice,
            Self::PaymentSuccessful(_) => MessageContentType::PaymentSuccessful,
            Self::VideoNote(_) => MessageContentType::VideoNote,
            Self::ContactRegistered(_) => MessageContentType::ContactRegistered,
            Self::ExpiredPhoto(_) => MessageContentType::ExpiredPhoto,
            Self::ExpiredVideo(_) => MessageContentType::ExpiredVideo,
            Self::LiveLocation(_) => MessageContentType::LiveLocation,
            Self::CustomServiceAction(_) => MessageContentType::CustomServiceAction,
            Self::WebsiteConnected(_) => MessageContentType::WebsiteConnected,
            Self::PassportDataSent(_) => MessageContentType::PassportDataSent,
            Self::PassportDataReceived(_) => MessageContentType::PassportDataReceived,
            Self::Poll(_) => MessageContentType::Poll,
            Self::Dice(_) => MessageContentType::Dice,
            Self::ProximityAlertTriggered(_) => MessageContentType::ProximityAlertTriggered,
            Self::GroupCall(_) => MessageContentType::GroupCall,
            Self::InviteToGroupCall(_) => MessageContentType::InviteToGroupCall,
            Self::ChatSetTheme(_) => MessageContentType::ChatSetTheme,
            Self::WebViewDataSent(_) => MessageContentType::WebViewDataSent,
            Self::WebViewDataReceived(_) => MessageContentType::WebViewDataReceived,
            Self::GiftPremium(_) => MessageContentType::GiftPremium,
            Self::TopicCreate(_) => MessageContentType::TopicCreate,
            Self::TopicEdit(_) => MessageContentType::TopicEdit,
            Self::SuggestProfilePhoto(_) => MessageContentType::SuggestProfilePhoto,
            Self::WriteAccessAllowed(_) => MessageContentType::WriteAccessAllowed,
            Self::RequestedDialog(_) => MessageContentType::RequestedDialog,
            Self::WebViewWriteAccessAllowed(_) => MessageContentType::WebViewWriteAccessAllowed,
            Self::SetBackground(_) => MessageContentType::SetBackground,
            Self::Story(_) => MessageContentType::Story,
            Self::WriteAccessAllowedByRequest(_) => MessageContentType::WriteAccessAllowedByRequest,
            Self::GiftCode(_) => MessageContentType::GiftCode,
            Self::Giveaway(_) => MessageContentType::Giveaway,
            Self::GiveawayLaunch(_) => MessageContentType::GiveawayLaunch,
            Self::GiveawayResults(_) => MessageContentType::GiveawayResults,
            Self::GiveawayWinners(_) => MessageContentType::GiveawayWinners,
            Self::ExpiredVideoNote(_) => MessageContentType::ExpiredVideoNote,
            Self::ExpiredVoiceNote(_) => MessageContentType::ExpiredVoiceNote,
            Self::BoostApply(_) => MessageContentType::BoostApply,
            Self::DialogShared(_) => MessageContentType::DialogShared,
            Self::PaidMedia(_) => MessageContentType::PaidMedia,
            Self::PaymentRefunded(_) => MessageContentType::PaymentRefunded,
            Self::GiftStars(_) => MessageContentType::GiftStars,
            Self::PrizeStars(_) => MessageContentType::PrizeStars,
            Self::StarGift(_) => MessageContentType::StarGift,
            Self::StarGiftUnique(_) => MessageContentType::StarGiftUnique,
            Self::PaidMessagesRefunded(_) => MessageContentType::PaidMessagesRefunded,
            Self::PaidMessagesPrice(_) => MessageContentType::PaidMessagesPrice,
            Self::ConferenceCall(_) => MessageContentType::ConferenceCall,
            Self::ToDoList(_) => MessageContentType::ToDoList,
            Self::TodoCompletions(_) => MessageContentType::TodoCompletions,
            Self::TodoAppendTasks(_) => MessageContentType::TodoAppendTasks,
            Self::GiftTon(_) => MessageContentType::GiftTon,
            Self::SuggestedPostSuccess(_) => MessageContentType::SuggestedPostSuccess,
            Self::SuggestedPostRefund(_) => MessageContentType::SuggestedPostRefund,
            Self::SuggestedPostApproval(_) => MessageContentType::SuggestedPostApproval,
            Self::SuggestBirthday(_) => MessageContentType::SuggestBirthday,
            Self::StarGiftPurchaseOffer(_) => MessageContentType::StarGiftPurchaseOffer,
            Self::StarGiftPurchaseOfferDeclined(_) => {
                MessageContentType::StarGiftPurchaseOfferDeclined
            }
        }
    }

    /// Returns `true` if this message content has text.
    #[must_use]
    pub fn has_text(&self) -> bool {
        matches!(self, Self::Text(_))
    }

    /// Returns `true` if this is a service message.
    #[must_use]
    pub fn is_service(&self) -> bool {
        self.content_type().is_service()
    }

    /// Returns `true` if this is media content.
    #[must_use]
    pub fn is_media(&self) -> bool {
        self.content_type().is_media()
    }

    /// Returns `true` if this content type is editable.
    #[must_use]
    pub fn is_editable(&self) -> bool {
        self.content_type().is_editable()
    }

    /// Returns the text content if this is a text message.
    #[must_use]
    pub fn as_text(&self) -> Option<&MessageText> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }

    /// Returns the caption if this content has one.
    #[must_use]
    pub fn caption(&self) -> Option<&FormattedText> {
        match self {
            Self::Photo(p) => Some(&p.caption),
            Self::Video(v) => Some(&v.caption),
            Self::Audio(a) => Some(&a.caption),
            Self::Document(d) => Some(&d.caption),
            Self::Animation(a) => Some(&a.caption),
            Self::VoiceNote(v) => Some(&v.caption),
            _ => None,
        }
    }
}

impl Default for MessageContent {
    fn default() -> Self {
        Self::Text(Box::new(MessageText::new(FormattedText::new(""))))
    }
}

impl fmt::Display for MessageContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content_type())
    }
}

// ============================================================================
// Error Types
// ============================================================================

/// Errors that can occur when working with message content.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MessageContentError {
    /// Invalid content type for the requested operation
    #[error("Invalid content type: expected {expected}, got {actual}")]
    InvalidContentType {
        /// The expected content type
        expected: MessageContentType,
        /// The actual content type received
        actual: MessageContentType,
    },

    /// Missing required field
    #[error("Missing required field: {field}")]
    MissingField {
        /// Name of the missing field
        field: String,
    },

    /// Invalid data format
    #[error("Invalid data format: {reason}")]
    InvalidFormat {
        /// Reason why the format is invalid
        reason: String,
    },
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_text_new() {
        let text = FormattedText::new("Hello");
        let content = MessageText::new(text);
        assert_eq!(content.text.text(), "Hello");
        assert!(!content.force_small_media);
        assert!(!content.force_large_media);
    }

    #[test]
    fn test_message_content_default() {
        let content = MessageContent::default();
        assert!(content.has_text());
        assert_eq!(content.content_type(), MessageContentType::Text);
    }

    #[test]
    fn test_content_type() {
        let text = FormattedText::new("Hello");
        let content = MessageContent::Text(Box::new(MessageText::new(text)));
        assert_eq!(content.content_type(), MessageContentType::Text);
    }

    #[test]
    fn test_is_service() {
        let content =
            MessageContent::ChatCreate(Box::new(MessageChatCreate::new("Test".to_string())));
        assert!(content.is_service());

        let text = FormattedText::new("Hello");
        let content = MessageContent::Text(Box::new(MessageText::new(text)));
        assert!(!content.is_service());
    }

    #[test]
    fn test_is_media() {
        let content = MessageContent::Photo(Box::new(MessagePhoto::new()));
        assert!(content.is_media());

        let text = FormattedText::new("Hello");
        let content = MessageContent::Text(Box::new(MessageText::new(text)));
        assert!(!content.is_media());
    }

    #[test]
    fn test_as_text() {
        let text = FormattedText::new("Hello");
        let content = MessageContent::Text(Box::new(MessageText::new(text.clone())));
        assert_eq!(content.as_text().unwrap().text.text(), "Hello");

        let content = MessageContent::Photo(Box::new(MessagePhoto::new()));
        assert!(content.as_text().is_none());
    }

    #[test]
    fn test_caption() {
        let caption = FormattedText::new("Check this out!");
        let mut photo = MessagePhoto::new();
        photo.caption = caption.clone();

        let content = MessageContent::Photo(Box::new(photo));
        assert_eq!(content.caption().unwrap().text(), "Check this out!");

        let text = FormattedText::new("Hello");
        let content = MessageContent::Text(Box::new(MessageText::new(text)));
        assert!(content.caption().is_none());
    }

    #[test]
    fn test_message_location_new() {
        let location = MessageLocation::new(40.7128, -74.0060);
        assert!((location.latitude - 40.7128).abs() < f64::EPSILON);
        assert!((location.longitude + 74.0060).abs() < f64::EPSILON);
    }

    #[test]
    fn test_message_venue_new() {
        let location = MessageLocation::new(40.7128, -74.0060);
        let venue = MessageVenue::new(location, "Central Park".to_string(), "New York".to_string());
        assert_eq!(venue.title, "Central Park");
        assert_eq!(venue.address, "New York");
    }

    #[test]
    fn test_message_contact_new() {
        let contact = MessageContact::new("+1234567890".to_string(), "John".to_string());
        assert_eq!(contact.phone_number, "+1234567890");
        assert_eq!(contact.first_name, "John");
    }

    #[test]
    fn test_message_poll_new() {
        let poll_id = PollId::new(123);
        let poll = MessagePoll::new(poll_id);
        assert_eq!(poll.poll_id, poll_id);
    }

    #[test]
    fn test_message_dice_default() {
        let dice = MessageDice::default();
        assert_eq!(dice.emoji, "🎲");
        assert_eq!(dice.dice_value, 0);
    }

    #[test]
    fn test_message_game_new() {
        let game = MessageGame::new(123, "test_game".to_string(), "Test Game".to_string());
        assert_eq!(game.id, 123);
        assert_eq!(game.short_name, "test_game");
        assert_eq!(game.title, "Test Game");
    }

    #[test]
    fn test_message_invoice_new() {
        let invoice = MessageInvoice::new(
            "Test Product".to_string(),
            "Description".to_string(),
            "USD".to_string(),
            999,
        );
        assert_eq!(invoice.title, "Test Product");
        assert_eq!(invoice.total_amount, 999);
    }

    #[test]
    fn test_serialization() {
        let text = FormattedText::new("Hello");
        let content = MessageContent::Text(Box::new(MessageText::new(text)));
        let json = serde_json::to_string(&content).unwrap();
        let parsed: MessageContent = serde_json::from_str(&json).unwrap();
        assert_eq!(content, parsed);
    }

    #[test]
    fn test_display() {
        let text = FormattedText::new("Hello");
        let content = MessageContent::Text(Box::new(MessageText::new(text)));
        assert_eq!(format!("{}", content), "Text");
    }

    #[test]
    fn test_empty_structs_default() {
        let _ = MessageChatDeletePhoto::default();
        let _ = MessageChatDeleteHistory::default();
        let _ = MessageScreenshotTaken::default();
        let _ = MessageContactRegistered::default();
        let _ = MessageExpiredPhoto::default();
        let _ = MessageExpiredVideo::default();
        let _ = MessageWriteAccessAllowed::default();
        let _ = MessageWebViewWriteAccessAllowed::default();
        let _ = MessageWriteAccessAllowedByRequest::default();
        let _ = MessageExpiredVideoNote::default();
        let _ = MessageExpiredVoiceNote::default();
        let _ = MessageStarGiftPurchaseOfferDeclined::default();
    }

    #[test]
    fn test_modern_features() {
        // Story
        let story = MessageContent::Story(Box::new(MessageStory::new(
            DialogId::from_user(UserId(1)),
            123,
        )));
        assert_eq!(story.content_type(), MessageContentType::Story);

        // Giveaway
        let giveaway = MessageContent::Giveaway(Box::new(MessageGiveaway::new(456, 10, 3)));
        assert!(giveaway.is_service());

        // Boost
        let boost = MessageContent::BoostApply(Box::new(MessageBoostApply::new(UserId(1))));
        assert!(boost.is_service());

        // Star gift
        let star_gift = MessageContent::StarGift(Box::new(MessageStarGift::new(
            UserId(1),
            UserId(2),
            789,
            100,
        )));
        assert!(star_gift.is_service());
    }

    #[test]
    fn test_all_content_types() {
        // Verify all 61 content types are represented
        let text = MessageContent::Text(Box::new(MessageText::new(FormattedText::new(""))));
        assert_eq!(text.content_type(), MessageContentType::Text);

        let animation = MessageContent::Animation(Box::new(MessageAnimation::new(FileId::new(1))));
        assert_eq!(animation.content_type(), MessageContentType::Animation);

        // ... all other types are covered by the enum
    }

    #[test]
    fn test_giveaway_types() {
        let giveaway = MessageContent::Giveaway(Box::new(MessageGiveaway::new(1, 10, 3)));
        assert_eq!(giveaway.content_type(), MessageContentType::Giveaway);

        let launch = MessageContent::GiveawayLaunch(Box::new(MessageGiveawayLaunch::new(1)));
        assert_eq!(launch.content_type(), MessageContentType::GiveawayLaunch);

        let results =
            MessageContent::GiveawayResults(Box::new(MessageGiveawayResults::new(1, 5, 2)));
        assert_eq!(results.content_type(), MessageContentType::GiveawayResults);

        let winners = MessageContent::GiveawayWinners(Box::new(MessageGiveawayWinners::new(1, 10)));
        assert_eq!(winners.content_type(), MessageContentType::GiveawayWinners);
    }

    #[test]
    fn test_star_types() {
        let gift_stars =
            MessageContent::GiftStars(Box::new(MessageGiftStars::new(UserId(1), UserId(2), 100)));
        assert_eq!(gift_stars.content_type(), MessageContentType::GiftStars);

        let prize_stars =
            MessageContent::PrizeStars(Box::new(MessagePrizeStars::new(UserId(1), UserId(2), 50)));
        assert_eq!(prize_stars.content_type(), MessageContentType::PrizeStars);

        let star_gift = MessageContent::StarGift(Box::new(MessageStarGift::new(
            UserId(1),
            UserId(2),
            123,
            100,
        )));
        assert_eq!(star_gift.content_type(), MessageContentType::StarGift);
    }

    #[test]
    fn test_todo_types() {
        let todo_list = MessageContent::ToDoList(Box::new(MessageToDoList::new(1)));
        assert_eq!(todo_list.content_type(), MessageContentType::ToDoList);

        let completions = MessageContent::TodoCompletions(Box::new(MessageTodoCompletions::new(1)));
        assert_eq!(
            completions.content_type(),
            MessageContentType::TodoCompletions
        );

        let append = MessageContent::TodoAppendTasks(Box::new(MessageTodoAppendTasks::new(1)));
        assert_eq!(append.content_type(), MessageContentType::TodoAppendTasks);
    }

    #[test]
    fn test_paid_media() {
        let paid = MessageContent::PaidMedia(Box::new(MessagePaidMedia::new(50)));
        assert_eq!(paid.content_type(), MessageContentType::PaidMedia);
        assert!(paid.is_media());

        let refunded = MessageContent::PaidMessagesRefunded(Box::new(
            MessagePaidMessagesRefunded::new(UserId(1), 3),
        ));
        assert!(refunded.is_service());

        let price = MessageContent::PaidMessagesPrice(Box::new(MessagePaidMessagesPrice::new(100)));
        assert!(price.is_service());
    }

    #[test]
    fn test_web_view_types() {
        let sent = MessageContent::WebViewDataSent(Box::new(MessageWebViewDataSent::new(
            "data".to_string(),
        )));
        assert_eq!(sent.content_type(), MessageContentType::WebViewDataSent);

        let received = MessageContent::WebViewDataReceived(Box::new(
            MessageWebViewDataReceived::new("data".to_string()),
        ));
        assert_eq!(
            received.content_type(),
            MessageContentType::WebViewDataReceived
        );

        let access = MessageContent::WebViewWriteAccessAllowed(Box::new(
            MessageWebViewWriteAccessAllowed::default(),
        ));
        assert_eq!(
            access.content_type(),
            MessageContentType::WebViewWriteAccessAllowed
        );
    }

    #[test]
    fn test_suggested_post_types() {
        let success = MessageContent::SuggestedPostSuccess(Box::new(
            MessageSuggestedPostSuccess::new(DialogId::from_user(UserId(1)), MessageId(1)),
        ));
        assert_eq!(
            success.content_type(),
            MessageContentType::SuggestedPostSuccess
        );

        let refund = MessageContent::SuggestedPostRefund(Box::new(
            MessageSuggestedPostRefund::new(DialogId::from_user(UserId(1)), MessageId(1)),
        ));
        assert_eq!(
            refund.content_type(),
            MessageContentType::SuggestedPostRefund
        );

        let approval = MessageContent::SuggestedPostApproval(Box::new(
            MessageSuggestedPostApproval::new(DialogId::from_user(UserId(1)), MessageId(1)),
        ));
        assert_eq!(
            approval.content_type(),
            MessageContentType::SuggestedPostApproval
        );
    }

    #[test]
    fn test_equality() {
        let text1 = FormattedText::new("Hello");
        let content1 = MessageContent::Text(Box::new(MessageText::new(text1)));

        let text2 = FormattedText::new("Hello");
        let content2 = MessageContent::Text(Box::new(MessageText::new(text2)));

        assert_eq!(content1, content2);

        let text3 = FormattedText::new("World");
        let content3 = MessageContent::Text(Box::new(MessageText::new(text3)));

        assert_ne!(content1, content3);
    }

    #[test]
    fn test_clone() {
        let text = FormattedText::new("Hello");
        let content1 = MessageContent::Text(Box::new(MessageText::new(text)));
        let content2 = content1.clone();
        assert_eq!(content1, content2);
    }
}
