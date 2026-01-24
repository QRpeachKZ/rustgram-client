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

//! # User Full Info
//!
//! Comprehensive information about a Telegram user including profile photos,
//! contact settings, business info, bot details, and various flags about user
//! capabilities and restrictions.
//!
//! ## TDLib Correspondence
//!
//! This module implements the TDLib `UserFullInfo` class from `td/telegram/UserManager.h`.
//!
//! ## Examples
//!
//! ```rust
//! use rustgram_user_full_info::UserFullInfo;
//!
//! let info = UserFullInfo::new();
//! assert!(info.is_empty());
//! assert!(!info.can_be_called());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::let_and_return)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_birthdate::Birthdate;
use rustgram_block_list_id::BlockListId;
use rustgram_business_info::BusinessInfo;
use rustgram_file_id::FileId;
use rustgram_formatted_text::FormattedText;
use rustgram_profile_tab::ProfileTab;
use rustgram_star_gift_settings::StarGiftSettings;
use rustgram_star_rating::StarRating;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Placeholder type for BotInfo.
///
/// This is a simplified placeholder until the full BotInfo type is available.
/// In the full implementation, this would contain bot-specific information
/// like bot description, about text, commands, menu button, etc.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BotInfo {
    /// Bot description
    description: Option<String>,
    /// Bot about text
    about: Option<String>,
}

impl BotInfo {
    /// Creates a new empty BotInfo.
    #[must_use]
    pub fn new() -> Self {
        Self {
            description: None,
            about: None,
        }
    }

    /// Creates a BotInfo with description and about text.
    #[must_use]
    pub fn with_data(description: Option<String>, about: Option<String>) -> Self {
        Self { description, about }
    }
}

/// Comprehensive user information.
///
/// Contains detailed information about a Telegram user including profile photos,
/// communication settings, privacy flags, business info, and bot details.
///
/// # Examples
///
/// ```rust
/// use rustgram_user_full_info::UserFullInfo;
///
/// let info = UserFullInfo::new();
/// assert!(info.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserFullInfo {
    // Photo fields (3)
    /// Personal profile photo of the user
    personal_photo: Option<FileId>,
    /// Profile photo of the user
    photo: Option<FileId>,
    /// Public photo of the user
    public_photo: Option<FileId>,

    // Block list (1)
    /// Block list the user is in
    block_list: BlockListId,

    // Communication flags (3)
    /// True, if the user can be called
    can_be_called: bool,
    /// True, if the user supports video calls
    supports_video_calls: bool,
    /// True, if the user has private calls
    has_private_calls: bool,

    // Privacy flags (4)
    /// True, if the user has private forwards enabled
    has_private_forwards: bool,
    /// True, if voice/video notes are restricted
    has_restricted_voice_and_video_note_messages: bool,
    /// True, if user posted to profile stories
    has_posted_to_profile_stories: bool,
    /// True, if sponsored messages are enabled
    has_sponsored_messages_enabled: bool,

    // Privacy exception (1)
    /// True, if phone number privacy exception needed
    need_phone_number_privacy_exception: bool,

    // Chat background (1)
    /// True, if chat background is set
    set_chat_background: bool,

    // Text fields (2)
    /// User biography
    bio: FormattedText,
    /// User note
    note: FormattedText,

    // Birthdate (1)
    /// User birthdate
    birthdate: Option<Birthdate>,

    // Personal chat (1)
    /// Personal chat ID
    personal_chat_id: i64,

    // Gift info (1)
    /// Number of gifts received
    gift_count: i32,

    // Groups in common (1)
    /// Number of groups in common
    group_in_common_count: i32,

    // Paid message star counts (2)
    /// Incoming paid message star count
    incoming_paid_message_star_count: i64,
    /// Outgoing paid message star count
    outgoing_paid_message_star_count: i64,

    // Gift settings (1)
    /// Gift settings
    gift_settings: StarGiftSettings,

    // Bot verification (1)
    /// Bot verification info
    bot_verification: Option<rustgram_bot_verification::BotVerification>,

    // Profile tab (1)
    /// Main profile tab
    main_profile_tab: ProfileTab,

    // First profile audio (1)
    /// First profile audio (placeholder using FileId)
    first_profile_audio: Option<FileId>,

    // Rating (3)
    /// Current user rating
    rating: Option<StarRating>,
    /// Pending user rating
    pending_rating: Option<StarRating>,
    /// Pending rating date (unix timestamp)
    pending_rating_date: i32,

    // Business info (1)
    /// Business account info
    business_info: Option<BusinessInfo>,

    // Bot info (1)
    /// Bot information
    bot_info: Option<BotInfo>,
}

impl Default for UserFullInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl UserFullInfo {
    // ========== Constructors ==========

    /// Creates a new empty UserFullInfo with default values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_full_info::UserFullInfo;
    ///
    /// let info = UserFullInfo::new();
    /// assert!(info.is_empty());
    /// assert!(!info.can_be_called());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            personal_photo: None,
            photo: None,
            public_photo: None,
            block_list: BlockListId::default(),
            can_be_called: false,
            supports_video_calls: false,
            has_private_calls: false,
            has_private_forwards: false,
            has_restricted_voice_and_video_note_messages: false,
            has_posted_to_profile_stories: false,
            has_sponsored_messages_enabled: false,
            need_phone_number_privacy_exception: false,
            set_chat_background: false,
            bio: FormattedText::new(""),
            note: FormattedText::new(""),
            birthdate: None,
            personal_chat_id: 0,
            gift_count: 0,
            group_in_common_count: 0,
            incoming_paid_message_star_count: 0,
            outgoing_paid_message_star_count: 0,
            gift_settings: StarGiftSettings::new(),
            bot_verification: None,
            main_profile_tab: ProfileTab::default(),
            first_profile_audio: None,
            rating: None,
            pending_rating: None,
            pending_rating_date: 0,
            business_info: None,
            bot_info: None,
        }
    }

    /// Creates UserFullInfo with basic bio and note.
    ///
    /// # Arguments
    ///
    /// * `bio` - User biography
    /// * `note` - User note
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_full_info::UserFullInfo;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let bio = FormattedText::new("Software Developer");
    /// let note = FormattedText::new("Met at conference");
    /// let info = UserFullInfo::with_basic_info(bio.clone(), note.clone());
    ///
    /// assert_eq!(info.bio().text(), "Software Developer");
    /// assert_eq!(info.note().text(), "Met at conference");
    /// ```
    #[must_use]
    pub fn with_basic_info(bio: FormattedText, note: FormattedText) -> Self {
        Self {
            bio,
            note,
            ..Self::new()
        }
    }

    // ========== Getters ==========

    /// Returns the personal profile photo.
    #[must_use]
    pub const fn personal_photo(&self) -> Option<&FileId> {
        self.personal_photo.as_ref()
    }

    /// Returns the profile photo.
    #[must_use]
    pub const fn photo(&self) -> Option<&FileId> {
        self.photo.as_ref()
    }

    /// Returns the public photo.
    #[must_use]
    pub const fn public_photo(&self) -> Option<&FileId> {
        self.public_photo.as_ref()
    }

    /// Returns the block list.
    #[must_use]
    pub const fn block_list(&self) -> BlockListId {
        self.block_list
    }

    /// Returns whether the user can be called.
    #[must_use]
    pub const fn can_be_called(&self) -> bool {
        self.can_be_called
    }

    /// Returns whether the user supports video calls.
    #[must_use]
    pub const fn supports_video_calls(&self) -> bool {
        self.supports_video_calls
    }

    /// Returns whether the user has private calls.
    #[must_use]
    pub const fn has_private_calls(&self) -> bool {
        self.has_private_calls
    }

    /// Returns whether the user has private forwards enabled.
    #[must_use]
    pub const fn has_private_forwards(&self) -> bool {
        self.has_private_forwards
    }

    /// Returns whether voice/video notes are restricted.
    #[must_use]
    pub const fn has_restricted_voice_and_video_note_messages(&self) -> bool {
        self.has_restricted_voice_and_video_note_messages
    }

    /// Returns whether user posted to profile stories.
    #[must_use]
    pub const fn has_posted_to_profile_stories(&self) -> bool {
        self.has_posted_to_profile_stories
    }

    /// Returns whether sponsored messages are enabled.
    #[must_use]
    pub const fn has_sponsored_messages_enabled(&self) -> bool {
        self.has_sponsored_messages_enabled
    }

    /// Returns whether phone number privacy exception is needed.
    #[must_use]
    pub const fn need_phone_number_privacy_exception(&self) -> bool {
        self.need_phone_number_privacy_exception
    }

    /// Returns whether chat background is set.
    #[must_use]
    pub const fn set_chat_background(&self) -> bool {
        self.set_chat_background
    }

    /// Returns the user biography.
    #[must_use]
    pub const fn bio(&self) -> &FormattedText {
        &self.bio
    }

    /// Returns the birthdate.
    #[must_use]
    pub const fn birthdate(&self) -> Option<&Birthdate> {
        self.birthdate.as_ref()
    }

    /// Returns the personal chat ID.
    #[must_use]
    pub const fn personal_chat_id(&self) -> i64 {
        self.personal_chat_id
    }

    /// Returns the gift count.
    #[must_use]
    pub const fn gift_count(&self) -> i32 {
        self.gift_count
    }

    /// Returns the group in common count.
    #[must_use]
    pub const fn group_in_common_count(&self) -> i32 {
        self.group_in_common_count
    }

    /// Returns the incoming paid message star count.
    #[must_use]
    pub const fn incoming_paid_message_star_count(&self) -> i64 {
        self.incoming_paid_message_star_count
    }

    /// Returns the outgoing paid message star count.
    #[must_use]
    pub const fn outgoing_paid_message_star_count(&self) -> i64 {
        self.outgoing_paid_message_star_count
    }

    /// Returns the gift settings.
    #[must_use]
    pub const fn gift_settings(&self) -> &StarGiftSettings {
        &self.gift_settings
    }

    /// Returns the bot verification info.
    #[must_use]
    pub const fn bot_verification(&self) -> Option<&rustgram_bot_verification::BotVerification> {
        self.bot_verification.as_ref()
    }

    /// Returns the main profile tab.
    #[must_use]
    pub const fn main_profile_tab(&self) -> ProfileTab {
        self.main_profile_tab
    }

    /// Returns the first profile audio.
    #[must_use]
    pub const fn first_profile_audio(&self) -> Option<&FileId> {
        self.first_profile_audio.as_ref()
    }

    /// Returns the note.
    #[must_use]
    pub const fn note(&self) -> &FormattedText {
        &self.note
    }

    /// Returns the rating.
    #[must_use]
    pub const fn rating(&self) -> Option<&StarRating> {
        self.rating.as_ref()
    }

    /// Returns the pending rating.
    #[must_use]
    pub const fn pending_rating(&self) -> Option<&StarRating> {
        self.pending_rating.as_ref()
    }

    /// Returns the pending rating date.
    #[must_use]
    pub const fn pending_rating_date(&self) -> i32 {
        self.pending_rating_date
    }

    /// Returns the business info.
    #[must_use]
    pub const fn business_info(&self) -> Option<&BusinessInfo> {
        self.business_info.as_ref()
    }

    /// Returns the bot info.
    #[must_use]
    pub const fn bot_info(&self) -> Option<&BotInfo> {
        self.bot_info.as_ref()
    }

    // ========== Setters ==========

    /// Sets the bio.
    ///
    /// # Returns
    ///
    /// `true` if the bio was changed
    pub fn set_bio(&mut self, bio: FormattedText) -> bool {
        if self.bio.text() == bio.text() {
            return false;
        }
        self.bio = bio;
        true
    }

    /// Sets the note.
    ///
    /// # Returns
    ///
    /// `true` if the note was changed
    pub fn set_note(&mut self, note: FormattedText) -> bool {
        if self.note.text() == note.text() {
            return false;
        }
        self.note = note;
        true
    }

    /// Sets the birthdate.
    ///
    /// # Returns
    ///
    /// `true` if the birthdate was changed
    pub fn set_birthdate(&mut self, birthdate: Option<Birthdate>) -> bool {
        if self.birthdate == birthdate {
            return false;
        }
        self.birthdate = birthdate;
        true
    }

    // ========== Utility Methods ==========

    /// Returns `true` if all optional fields are None and counts are zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_full_info::UserFullInfo;
    ///
    /// let info = UserFullInfo::new();
    /// assert!(info.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.personal_photo.is_none()
            && self.photo.is_none()
            && self.public_photo.is_none()
            && self.birthdate.is_none()
            && self.first_profile_audio.is_none()
            && self.bot_verification.is_none()
            && self.rating.is_none()
            && self.pending_rating.is_none()
            && self.business_info.is_none()
            && self.bot_info.is_none()
            && self.gift_count == 0
            && self.group_in_common_count == 0
            && self.incoming_paid_message_star_count == 0
            && self.outgoing_paid_message_star_count == 0
            && self.personal_chat_id == 0
            && self.pending_rating_date == 0
    }

    /// Returns `true` if bot_info is Some.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_full_info::UserFullInfo;
    ///
    /// let info = UserFullInfo::new();
    /// assert!(!info.is_bot());
    /// ```
    #[must_use]
    pub const fn is_bot(&self) -> bool {
        self.bot_info.is_some()
    }

    /// Returns `true` if business_info is Some.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_full_info::UserFullInfo;
    ///
    /// let info = UserFullInfo::new();
    /// assert!(!info.is_business());
    /// ```
    #[must_use]
    pub const fn is_business(&self) -> bool {
        self.business_info.is_some()
    }

    /// Returns `true` if incoming_paid_message_star_count > 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_full_info::UserFullInfo;
    ///
    /// let info = UserFullInfo::new();
    /// assert!(!info.can_receive_paid_messages());
    /// ```
    #[must_use]
    pub const fn can_receive_paid_messages(&self) -> bool {
        self.incoming_paid_message_star_count > 0
    }

    /// Returns `true` if rating is Some.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_user_full_info::UserFullInfo;
    ///
    /// let info = UserFullInfo::new();
    /// assert!(!info.has_rating());
    /// ```
    #[must_use]
    pub const fn has_rating(&self) -> bool {
        self.rating.is_some()
    }
}

impl fmt::Display for UserFullInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UserFullInfo {{ bio: {:?}, note: {:?}, block_list: {:?} }}",
            self.bio.text(),
            self.note.text(),
            self.block_list
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_bot_verification::BotVerification;
    use rustgram_types::UserId;

    // ========== Constructor Tests ==========

    #[test]
    fn test_new_creates_empty() {
        let info = UserFullInfo::new();
        assert!(info.is_empty());
        assert!(!info.can_be_called());
        assert!(!info.supports_video_calls());
        assert!(!info.has_private_calls());
    }

    #[test]
    fn test_default_creates_empty() {
        let info = UserFullInfo::default();
        assert!(info.is_empty());
    }

    #[test]
    fn test_with_basic_info() {
        let bio = FormattedText::new("Software Developer");
        let note = FormattedText::new("Met at conference");
        let info = UserFullInfo::with_basic_info(bio.clone(), note.clone());

        assert_eq!(info.bio().text(), "Software Developer");
        assert_eq!(info.note().text(), "Met at conference");
    }

    // ========== Photo Field Getter Tests ==========

    #[test]
    fn test_personal_photo_getter() {
        let mut info = UserFullInfo::new();
        assert!(info.personal_photo().is_none());

        let file_id = FileId::new(1, 0);
        info.personal_photo = Some(file_id);
        assert_eq!(info.personal_photo(), Some(&file_id));
    }

    #[test]
    fn test_photo_getter() {
        let mut info = UserFullInfo::new();
        assert!(info.photo().is_none());

        let file_id = FileId::new(2, 0);
        info.photo = Some(file_id);
        assert_eq!(info.photo(), Some(&file_id));
    }

    #[test]
    fn test_public_photo_getter() {
        let mut info = UserFullInfo::new();
        assert!(info.public_photo().is_none());

        let file_id = FileId::new(3, 0);
        info.public_photo = Some(file_id);
        assert_eq!(info.public_photo(), Some(&file_id));
    }

    // ========== Block List Getter Tests ==========

    #[test]
    fn test_block_list_getter() {
        let info = UserFullInfo::new();
        assert_eq!(info.block_list(), BlockListId::None);
    }

    // ========== Communication Flag Getter Tests ==========

    #[test]
    fn test_can_be_called_getter() {
        let mut info = UserFullInfo::new();
        assert!(!info.can_be_called());

        info.can_be_called = true;
        assert!(info.can_be_called());
    }

    #[test]
    fn test_supports_video_calls_getter() {
        let mut info = UserFullInfo::new();
        assert!(!info.supports_video_calls());

        info.supports_video_calls = true;
        assert!(info.supports_video_calls());
    }

    #[test]
    fn test_has_private_calls_getter() {
        let mut info = UserFullInfo::new();
        assert!(!info.has_private_calls());

        info.has_private_calls = true;
        assert!(info.has_private_calls());
    }

    // ========== Privacy Flag Getter Tests ==========

    #[test]
    fn test_has_private_forwards_getter() {
        let mut info = UserFullInfo::new();
        assert!(!info.has_private_forwards());

        info.has_private_forwards = true;
        assert!(info.has_private_forwards());
    }

    #[test]
    fn test_has_restricted_voice_and_video_note_messages_getter() {
        let mut info = UserFullInfo::new();
        assert!(!info.has_restricted_voice_and_video_note_messages());

        info.has_restricted_voice_and_video_note_messages = true;
        assert!(info.has_restricted_voice_and_video_note_messages());
    }

    #[test]
    fn test_has_posted_to_profile_stories_getter() {
        let mut info = UserFullInfo::new();
        assert!(!info.has_posted_to_profile_stories());

        info.has_posted_to_profile_stories = true;
        assert!(info.has_posted_to_profile_stories());
    }

    #[test]
    fn test_has_sponsored_messages_enabled_getter() {
        let mut info = UserFullInfo::new();
        assert!(!info.has_sponsored_messages_enabled());

        info.has_sponsored_messages_enabled = true;
        assert!(info.has_sponsored_messages_enabled());
    }

    // ========== Privacy Exception Getter Tests ==========

    #[test]
    fn test_need_phone_number_privacy_exception_getter() {
        let mut info = UserFullInfo::new();
        assert!(!info.need_phone_number_privacy_exception());

        info.need_phone_number_privacy_exception = true;
        assert!(info.need_phone_number_privacy_exception());
    }

    // ========== Chat Background Getter Tests ==========

    #[test]
    fn test_set_chat_background_getter() {
        let mut info = UserFullInfo::new();
        assert!(!info.set_chat_background());

        info.set_chat_background = true;
        assert!(info.set_chat_background());
    }

    // ========== Text Field Getter Tests ==========

    #[test]
    fn test_bio_getter() {
        let bio = FormattedText::new("Test Bio");
        let info = UserFullInfo::with_basic_info(bio.clone(), FormattedText::new(""));

        assert_eq!(info.bio().text(), "Test Bio");
    }

    #[test]
    fn test_note_getter() {
        let note = FormattedText::new("Test Note");
        let info = UserFullInfo::with_basic_info(FormattedText::new(""), note.clone());

        assert_eq!(info.note().text(), "Test Note");
    }

    // ========== Birthdate Getter Tests ==========

    #[test]
    fn test_birthdate_getter() {
        let mut info = UserFullInfo::new();
        assert!(info.birthdate().is_none());

        let birthdate = Birthdate::new(15, 6, 1990).unwrap();
        info.birthdate = Some(birthdate);
        assert!(info.birthdate().is_some());
        assert_eq!(info.birthdate().unwrap().day(), 15);
    }

    // ========== Personal Chat ID Getter Tests ==========

    #[test]
    fn test_personal_chat_id_getter() {
        let mut info = UserFullInfo::new();
        assert_eq!(info.personal_chat_id(), 0);

        info.personal_chat_id = 12345;
        assert_eq!(info.personal_chat_id(), 12345);
    }

    // ========== Gift Count Getter Tests ==========

    #[test]
    fn test_gift_count_getter() {
        let mut info = UserFullInfo::new();
        assert_eq!(info.gift_count(), 0);

        info.gift_count = 10;
        assert_eq!(info.gift_count(), 10);
    }

    // ========== Group In Common Count Getter Tests ==========

    #[test]
    fn test_group_in_common_count_getter() {
        let mut info = UserFullInfo::new();
        assert_eq!(info.group_in_common_count(), 0);

        info.group_in_common_count = 5;
        assert_eq!(info.group_in_common_count(), 5);
    }

    // ========== Paid Message Star Count Getter Tests ==========

    #[test]
    fn test_incoming_paid_message_star_count_getter() {
        let mut info = UserFullInfo::new();
        assert_eq!(info.incoming_paid_message_star_count(), 0);

        info.incoming_paid_message_star_count = 100;
        assert_eq!(info.incoming_paid_message_star_count(), 100);
    }

    #[test]
    fn test_outgoing_paid_message_star_count_getter() {
        let mut info = UserFullInfo::new();
        assert_eq!(info.outgoing_paid_message_star_count(), 0);

        info.outgoing_paid_message_star_count = 50;
        assert_eq!(info.outgoing_paid_message_star_count(), 50);
    }

    // ========== Gift Settings Getter Tests ==========

    #[test]
    fn test_gift_settings_getter() {
        let info = UserFullInfo::new();
        assert!(info.gift_settings().is_default());
    }

    // ========== Bot Verification Getter Tests ==========

    #[test]
    fn test_bot_verification_getter() {
        let mut info = UserFullInfo::new();
        assert!(info.bot_verification().is_none());

        let bot_id = UserId::new(123).unwrap();
        let verification = BotVerification::new(bot_id, 1234567890, None);
        info.bot_verification = Some(verification);

        assert!(info.bot_verification().is_some());
    }

    // ========== Profile Tab Getter Tests ==========

    #[test]
    fn test_main_profile_tab_getter() {
        let info = UserFullInfo::new();
        assert_eq!(info.main_profile_tab(), ProfileTab::Unknown);
    }

    // ========== First Profile Audio Getter Tests ==========

    #[test]
    fn test_first_profile_audio_getter() {
        let mut info = UserFullInfo::new();
        assert!(info.first_profile_audio().is_none());

        let file_id = FileId::new(4, 0);
        info.first_profile_audio = Some(file_id);
        assert_eq!(info.first_profile_audio(), Some(&file_id));
    }

    // ========== Rating Getter Tests ==========

    #[test]
    fn test_rating_getter() {
        let mut info = UserFullInfo::new();
        assert!(info.rating().is_none());

        let rating = StarRating::with_values(5, 1000, 500, 1500);
        info.rating = Some(rating);
        assert!(info.rating().is_some());
        assert_eq!(info.rating().unwrap().level(), 5);
    }

    #[test]
    fn test_pending_rating_getter() {
        let mut info = UserFullInfo::new();
        assert!(info.pending_rating().is_none());

        let rating = StarRating::with_values(6, 1200, 600, 1800);
        info.pending_rating = Some(rating);
        assert!(info.pending_rating().is_some());
        assert_eq!(info.pending_rating().unwrap().level(), 6);
    }

    #[test]
    fn test_pending_rating_date_getter() {
        let mut info = UserFullInfo::new();
        assert_eq!(info.pending_rating_date(), 0);

        info.pending_rating_date = 1234567890;
        assert_eq!(info.pending_rating_date(), 1234567890);
    }

    // ========== Business Info Getter Tests ==========

    #[test]
    fn test_business_info_getter() {
        let mut info = UserFullInfo::new();
        assert!(info.business_info().is_none());

        let business_info = BusinessInfo::new();
        info.business_info = Some(business_info);
        assert!(info.business_info().is_some());
    }

    // ========== Bot Info Getter Tests ==========

    #[test]
    fn test_bot_info_getter() {
        let mut info = UserFullInfo::new();
        assert!(info.bot_info().is_none());

        let bot_info = BotInfo::with_data(
            Some("Bot Description".to_string()),
            Some("Bot About".to_string()),
        );
        info.bot_info = Some(bot_info);
        assert!(info.bot_info().is_some());
    }

    // ========== Setter Tests ==========

    #[test]
    fn test_set_bio_changes() {
        let mut info = UserFullInfo::new();
        let bio = FormattedText::new("New Bio");

        let changed = info.set_bio(bio.clone());
        assert!(changed);
        assert_eq!(info.bio().text(), "New Bio");
    }

    #[test]
    fn test_set_bio_same_value() {
        let bio = FormattedText::new("Same Bio");
        let mut info = UserFullInfo::with_basic_info(bio.clone(), FormattedText::new(""));

        let changed = info.set_bio(bio);
        assert!(!changed);
    }

    #[test]
    fn test_set_note_changes() {
        let mut info = UserFullInfo::new();
        let note = FormattedText::new("New Note");

        let changed = info.set_note(note.clone());
        assert!(changed);
        assert_eq!(info.note().text(), "New Note");
    }

    #[test]
    fn test_set_note_same_value() {
        let note = FormattedText::new("Same Note");
        let mut info = UserFullInfo::with_basic_info(FormattedText::new(""), note.clone());

        let changed = info.set_note(note);
        assert!(!changed);
    }

    #[test]
    fn test_set_birthdate_changes() {
        let mut info = UserFullInfo::new();
        let birthdate = Birthdate::new(15, 6, 1990).unwrap();

        let changed = info.set_birthdate(Some(birthdate));
        assert!(changed);
        assert!(info.birthdate().is_some());
    }

    #[test]
    fn test_set_birthdate_same_value() {
        let birthdate = Birthdate::new(15, 6, 1990).unwrap();
        let mut info = UserFullInfo::new();
        info.birthdate = Some(birthdate);

        let changed = info.set_birthdate(Some(birthdate));
        assert!(!changed);
    }

    // ========== Utility Method Tests ==========

    #[test]
    fn test_is_empty_when_all_empty() {
        let info = UserFullInfo::new();
        assert!(info.is_empty());
    }

    #[test]
    fn test_is_empty_with_personal_photo() {
        let mut info = UserFullInfo::new();
        info.personal_photo = Some(FileId::new(1, 0));
        assert!(!info.is_empty());
    }

    #[test]
    fn test_is_empty_with_birthdate() {
        let mut info = UserFullInfo::new();
        info.birthdate = Some(Birthdate::new(15, 6, 1990).unwrap());
        assert!(!info.is_empty());
    }

    #[test]
    fn test_is_empty_with_gift_count() {
        let mut info = UserFullInfo::new();
        info.gift_count = 5;
        assert!(!info.is_empty());
    }

    #[test]
    fn test_is_empty_with_business_info() {
        let mut info = UserFullInfo::new();
        info.business_info = Some(BusinessInfo::new());
        assert!(!info.is_empty());
    }

    #[test]
    fn test_is_bot_when_bot_info_set() {
        let mut info = UserFullInfo::new();
        assert!(!info.is_bot());

        info.bot_info = Some(BotInfo::new());
        assert!(info.is_bot());
    }

    #[test]
    fn test_is_business_when_business_info_set() {
        let mut info = UserFullInfo::new();
        assert!(!info.is_business());

        info.business_info = Some(BusinessInfo::new());
        assert!(info.is_business());
    }

    #[test]
    fn test_can_receive_paid_messages_when_count_positive() {
        let mut info = UserFullInfo::new();
        assert!(!info.can_receive_paid_messages());

        info.incoming_paid_message_star_count = 100;
        assert!(info.can_receive_paid_messages());
    }

    #[test]
    fn test_has_rating_when_rating_set() {
        let mut info = UserFullInfo::new();
        assert!(!info.has_rating());

        info.rating = Some(StarRating::new());
        assert!(info.has_rating());
    }

    // ========== Trait Tests ==========

    #[test]
    fn test_clone() {
        let bio = FormattedText::new("Test Bio");
        let note = FormattedText::new("Test Note");
        let info1 = UserFullInfo::with_basic_info(bio, note);
        let info2 = info1.clone();

        assert_eq!(info1, info2);
    }

    #[test]
    fn test_equality() {
        let bio = FormattedText::new("Test Bio");
        let note = FormattedText::new("Test Note");
        let info1 = UserFullInfo::with_basic_info(bio.clone(), note.clone());
        let info2 = UserFullInfo::with_basic_info(bio, note);

        assert_eq!(info1, info2);
    }

    #[test]
    fn test_inequality() {
        let info1 = UserFullInfo::with_basic_info(
            FormattedText::new("Bio 1"),
            FormattedText::new("Note 1"),
        );
        let info2 = UserFullInfo::with_basic_info(
            FormattedText::new("Bio 2"),
            FormattedText::new("Note 2"),
        );

        assert_ne!(info1, info2);
    }

    #[test]
    fn test_debug() {
        let info = UserFullInfo::new();
        let debug_str = format!("{:?}", info);
        assert!(debug_str.contains("UserFullInfo"));
    }

    #[test]
    fn test_display() {
        let info = UserFullInfo::new();
        let display = format!("{}", info);
        assert!(display.contains("UserFullInfo"));
    }

    // ========== BotInfo Placeholder Tests ==========

    #[test]
    fn test_bot_info_new() {
        let bot_info = BotInfo::new();
        assert!(bot_info.description.is_none());
        assert!(bot_info.about.is_none());
    }

    #[test]
    fn test_bot_info_with_data() {
        let bot_info =
            BotInfo::with_data(Some("Description".to_string()), Some("About".to_string()));
        assert_eq!(bot_info.description, Some("Description".to_string()));
        assert_eq!(bot_info.about, Some("About".to_string()));
    }

    // ========== Comprehensive State Tests ==========

    #[test]
    fn test_all_communication_flags() {
        let mut info = UserFullInfo::new();
        info.can_be_called = true;
        info.supports_video_calls = true;
        info.has_private_calls = true;

        assert!(info.can_be_called());
        assert!(info.supports_video_calls());
        assert!(info.has_private_calls());
    }

    #[test]
    fn test_all_privacy_flags() {
        let mut info = UserFullInfo::new();
        info.has_private_forwards = true;
        info.has_restricted_voice_and_video_note_messages = true;
        info.has_posted_to_profile_stories = true;
        info.has_sponsored_messages_enabled = true;

        assert!(info.has_private_forwards());
        assert!(info.has_restricted_voice_and_video_note_messages());
        assert!(info.has_posted_to_profile_stories());
        assert!(info.has_sponsored_messages_enabled());
    }

    #[test]
    fn test_all_ids_and_counts() {
        let mut info = UserFullInfo::new();
        info.personal_chat_id = 12345;
        info.gift_count = 10;
        info.group_in_common_count = 5;
        info.incoming_paid_message_star_count = 1000;
        info.outgoing_paid_message_star_count = 500;
        info.pending_rating_date = 1234567890;

        assert_eq!(info.personal_chat_id(), 12345);
        assert_eq!(info.gift_count(), 10);
        assert_eq!(info.group_in_common_count(), 5);
        assert_eq!(info.incoming_paid_message_star_count(), 1000);
        assert_eq!(info.outgoing_paid_message_star_count(), 500);
        assert_eq!(info.pending_rating_date(), 1234567890);
    }
}
