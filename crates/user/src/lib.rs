// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # User
//!
//! Represents a Telegram user.
//!
//! ## TDLib Reference
//!
//! - TDLib schema: `td_api.tl` line 2054
//! - TDLib type: `user`
//!
//! ## Overview
//!
//! A `User` contains comprehensive information about a Telegram user including:
//! - Basic identity (id, first_name, last_name, usernames, phone_number)
//! - Status information (UserStatus, is_contact, is_mutual_contact, etc.)
//! - Profile customization (profile_photo, accent colors, emoji status)
//! - Account type and verification (UserType, verification_status, is_premium)
//! - Restrictions and access (restriction_info, have_access)
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_user::User;
//!
//! let user = User::builder()
//!     .with_id(123456789)
//!     .with_first_name("Alice")
//!     .with_last_name("Smith")
//!     .build()
//!     .unwrap();
//! ```
//!
//! ## Stub Implementations
//!
//! The following types are currently stubbed:
//! - `ChatPhoto` - Profile photo (TODO: full implementation when chat_photo crate exists)
//! - `RestrictionInfo` - Restriction information (TODO: full implementation)
//! - `UpgradedGiftColors` - Gift colors (TODO: full implementation)
//! - `UserStatus` - Online status (TODO: full implementation when user_status crate is complete)
//! - `UserType` - User type (TODO: full implementation when user_type crate is complete)

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use serde::{Deserialize, Serialize};
use std::fmt;

// Re-export existing types
pub use rustgram_accent_color_id::AccentColorId;
pub use rustgram_active_story_state::ActiveStoryState;
pub use rustgram_emoji_status::EmojiStatus;
pub use rustgram_usernames::Usernames;
pub use rustgram_verification_status::VerificationStatus;

// Stub types for missing dependencies

/// Stub for ChatPhoto.
///
/// TDLib: `chatPhoto` (line 877 in td_api.tl)
///
/// TODO: Full implementation when chat_photo crate exists.
/// Currently a minimal placeholder for User type compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChatPhoto {
    /// Photo identifier.
    pub id: i64,
    /// Point in time (Unix timestamp) when the photo has been added.
    pub added_date: i32,
}

impl ChatPhoto {
    /// Creates a new ChatPhoto with the given ID and date.
    #[must_use]
    pub const fn new(id: i64, added_date: i32) -> Self {
        Self { id, added_date }
    }

    /// Returns the photo identifier.
    #[must_use]
    pub const fn id(&self) -> i64 {
        self.id
    }

    /// Returns when the photo was added.
    #[must_use]
    pub const fn added_date(&self) -> i32 {
        self.added_date
    }
}

/// Stub for RestrictionInfo.
///
/// TDLib: `restrictionInfo` (line 1988 in td_api.tl)
///
/// TODO: Full implementation when restriction_info crate exists.
/// Currently a minimal placeholder for User type compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RestrictionInfo {
    /// The reason for restriction.
    pub restriction_reason: String,
    /// True, if the content is sensitive.
    pub has_sensitive_content: bool,
}

impl RestrictionInfo {
    /// Creates a new RestrictionInfo.
    #[must_use]
    pub fn new(restriction_reason: String, has_sensitive_content: bool) -> Self {
        Self {
            restriction_reason,
            has_sensitive_content,
        }
    }

    /// Returns the restriction reason.
    #[must_use]
    pub fn restriction_reason(&self) -> &str {
        &self.restriction_reason
    }

    /// Returns true if the content is sensitive.
    #[must_use]
    pub const fn has_sensitive_content(&self) -> bool {
        self.has_sensitive_content
    }
}

/// Stub for UpgradedGiftColors.
///
/// TDLib: `upgradedGiftColors` (line 1338 in td_api.tl)
///
/// TODO: Full implementation when upgraded_gift_colors crate exists.
/// Currently a minimal placeholder for User type compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UpgradedGiftColors {
    /// Unique identifier of the upgraded gift colors.
    pub id: i64,
}

impl UpgradedGiftColors {
    /// Creates a new UpgradedGiftColors.
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self { id }
    }

    /// Returns the identifier.
    #[must_use]
    pub const fn id(&self) -> i64 {
        self.id
    }
}

/// Stub for UserStatus.
///
/// TDLib: `UserStatus` enum (lines 5486-5501 in td_api.tl)
///
/// TODO: Full implementation when user_status crate is complete.
/// Currently a minimal placeholder for User type compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum UserStatus {
    /// The user status was never changed.
    #[default]
    Empty,
    /// The user is online.
    Online,
    /// The user is offline.
    Offline,
    /// The user was online recently.
    Recently,
    /// The user was online last week.
    LastWeek,
    /// The user was online last month.
    LastMonth,
}

/// Stub for UserType.
///
/// TDLib: `UserType` enum (lines 661-681 in td_api.tl)
///
/// TODO: Full implementation when user_type crate is complete.
/// Currently a minimal placeholder for User type compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum UserType {
    /// A regular user.
    #[default]
    Regular,
    /// A deleted user or deleted bot.
    Deleted,
    /// A bot.
    Bot,
    /// No information on the user besides the user identifier is available.
    Unknown,
}

/// Represents a Telegram user.
///
/// TDLib: `user` type from `td_api.tl` line 2054
///
/// # Fields
///
/// * `id` - User identifier
/// * `first_name` - First name of the user
/// * `last_name` - Last name of the user
/// * `usernames` - Usernames of the user
/// * `phone_number` - Phone number of the user
/// * `status` - Current user status
/// * `profile_photo` - Profile photo of the user
/// * `accent_color_id` - Identifier of the accent color for the user's profile
/// * `background_custom_emoji_id` - Identifier of a custom emoji to be shown on the reply header and background
/// * `upgraded_gift_colors` - Information about upgraded gift colors
/// * `profile_accent_color_id` - Identifier of the accent color for the user's profile background
/// * `profile_background_custom_emoji_id` - Identifier of a custom emoji to be shown on the profile background
/// * `emoji_status` - Emoji status to be shown instead of the user's profile photo
/// * `is_contact` - True, if the user is a contact
/// * `is_mutual_contact` - True, if the user is a mutual contact
/// * `is_close_friend` - True, if the user is a close friend
/// * `verification_status` - Verification status of the user
/// * `is_premium` - True, if the user is a Telegram Premium user
/// * `is_support` - True, if the user is Telegram support account
/// * `restriction_info` - Information about restrictions
/// * `active_story_state` - State of the active story
/// * `restricts_new_chats` - True, if the user restricts new chats
/// * `paid_message_star_count` - Number of Telegram Star that must be paid to send a private message
/// * `have_access` - True, if the bot can send messages to the user
/// * `type` - Type of the user
/// * `language_code` - IETF language tag of the user's language
/// * `added_to_attachment_menu` - True, if the user was added to attachment menu
///
/// # Example
///
/// ```
/// use rustgram_user::User;
///
/// let user = User::builder()
///     .with_id(123456789)
///     .with_first_name("Alice")
///     .with_last_name("Smith")
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    /// User identifier.
    id: i64,
    /// First name of the user.
    first_name: String,
    /// Last name of the user.
    last_name: String,
    /// Usernames of the user.
    usernames: Usernames,
    /// Phone number of the user.
    phone_number: String,
    /// Current user status.
    status: UserStatus,
    /// Profile photo of the user.
    profile_photo: Option<ChatPhoto>,
    /// Identifier of the accent color for the user's profile.
    accent_color_id: AccentColorId,
    /// Identifier of a custom emoji to be shown on the reply header and background.
    background_custom_emoji_id: i64,
    /// Information about upgraded gift colors.
    upgraded_gift_colors: Option<UpgradedGiftColors>,
    /// Identifier of the accent color for the user's profile background.
    profile_accent_color_id: i32,
    /// Identifier of a custom emoji to be shown on the profile background.
    profile_background_custom_emoji_id: i64,
    /// Emoji status to be shown instead of the user's profile photo.
    emoji_status: Option<EmojiStatus>,
    /// True, if the user is a contact.
    is_contact: bool,
    /// True, if the user is a mutual contact.
    is_mutual_contact: bool,
    /// True, if the user is a close friend.
    is_close_friend: bool,
    /// Verification status of the user.
    verification_status: Option<VerificationStatus>,
    /// True, if the user is a Telegram Premium user.
    is_premium: bool,
    /// True, if the user is Telegram support account.
    is_support: bool,
    /// Information about restrictions.
    restriction_info: Option<RestrictionInfo>,
    /// State of the active story.
    active_story_state: Option<ActiveStoryState>,
    /// True, if the user restricts new chats.
    restricts_new_chats: bool,
    /// Number of Telegram Star that must be paid to send a private message.
    paid_message_star_count: i64,
    /// True, if the bot can send messages to the user.
    have_access: bool,
    /// Type of the user.
    type_: UserType,
    /// IETF language tag of the user's language.
    language_code: String,
    /// True, if the user was added to attachment menu.
    added_to_attachment_menu: bool,
}

impl User {
    /// Creates a builder for User.
    #[must_use]
    pub const fn builder() -> UserBuilder {
        UserBuilder::new()
    }

    /// Returns the user identifier.
    #[must_use]
    pub const fn id(&self) -> i64 {
        self.id
    }

    /// Returns the first name.
    #[must_use]
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    /// Returns the last name.
    #[must_use]
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Returns the usernames.
    #[must_use]
    pub fn usernames(&self) -> &Usernames {
        &self.usernames
    }

    /// Returns the phone number.
    #[must_use]
    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    /// Returns the user status.
    #[must_use]
    pub const fn status(&self) -> &UserStatus {
        &self.status
    }

    /// Returns the profile photo.
    #[must_use]
    pub const fn profile_photo(&self) -> &Option<ChatPhoto> {
        &self.profile_photo
    }

    /// Returns the accent color ID.
    #[must_use]
    pub const fn accent_color_id(&self) -> &AccentColorId {
        &self.accent_color_id
    }

    /// Returns the background custom emoji ID.
    #[must_use]
    pub const fn background_custom_emoji_id(&self) -> i64 {
        self.background_custom_emoji_id
    }

    /// Returns the upgraded gift colors.
    #[must_use]
    pub const fn upgraded_gift_colors(&self) -> &Option<UpgradedGiftColors> {
        &self.upgraded_gift_colors
    }

    /// Returns the profile accent color ID.
    #[must_use]
    pub const fn profile_accent_color_id(&self) -> i32 {
        self.profile_accent_color_id
    }

    /// Returns the profile background custom emoji ID.
    #[must_use]
    pub const fn profile_background_custom_emoji_id(&self) -> i64 {
        self.profile_background_custom_emoji_id
    }

    /// Returns the emoji status.
    #[must_use]
    pub const fn emoji_status(&self) -> &Option<EmojiStatus> {
        &self.emoji_status
    }

    /// Returns true if the user is a contact.
    #[must_use]
    pub const fn is_contact(&self) -> bool {
        self.is_contact
    }

    /// Returns true if the user is a mutual contact.
    #[must_use]
    pub const fn is_mutual_contact(&self) -> bool {
        self.is_mutual_contact
    }

    /// Returns true if the user is a close friend.
    #[must_use]
    pub const fn is_close_friend(&self) -> bool {
        self.is_close_friend
    }

    /// Returns the verification status.
    #[must_use]
    pub const fn verification_status(&self) -> &Option<VerificationStatus> {
        &self.verification_status
    }

    /// Returns true if the user is a Premium user.
    #[must_use]
    pub const fn is_premium(&self) -> bool {
        self.is_premium
    }

    /// Returns true if the user is a support account.
    #[must_use]
    pub const fn is_support(&self) -> bool {
        self.is_support
    }

    /// Returns the restriction info.
    #[must_use]
    pub const fn restriction_info(&self) -> &Option<RestrictionInfo> {
        &self.restriction_info
    }

    /// Returns the active story state.
    #[must_use]
    pub const fn active_story_state(&self) -> &Option<ActiveStoryState> {
        &self.active_story_state
    }

    /// Returns true if the user restricts new chats.
    #[must_use]
    pub const fn restricts_new_chats(&self) -> bool {
        self.restricts_new_chats
    }

    /// Returns the paid message star count.
    #[must_use]
    pub const fn paid_message_star_count(&self) -> i64 {
        self.paid_message_star_count
    }

    /// Returns true if the bot can send messages to the user.
    #[must_use]
    pub const fn have_access(&self) -> bool {
        self.have_access
    }

    /// Returns the user type.
    #[must_use]
    pub const fn type_(&self) -> &UserType {
        &self.type_
    }

    /// Returns the language code.
    #[must_use]
    pub fn language_code(&self) -> &str {
        &self.language_code
    }

    /// Returns true if the user was added to attachment menu.
    #[must_use]
    pub const fn added_to_attachment_menu(&self) -> bool {
        self.added_to_attachment_menu
    }

    /// Returns the full name (first + last).
    #[must_use]
    pub fn full_name(&self) -> String {
        if self.last_name.is_empty() {
            self.first_name.clone()
        } else {
            format!("{} {}", self.first_name, self.last_name)
        }
    }

    /// Returns true if the user is deleted.
    #[must_use]
    pub const fn is_deleted(&self) -> bool {
        matches!(self.type_, UserType::Deleted)
    }

    /// Returns true if the user is a bot.
    #[must_use]
    pub const fn is_bot(&self) -> bool {
        matches!(self.type_, UserType::Bot)
    }

    /// Returns true if the user is online.
    #[must_use]
    pub const fn is_online(&self) -> bool {
        matches!(self.status, UserStatus::Online)
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User[id={}, name={}, type={:?}]",
            self.id,
            self.full_name(),
            self.type_
        )
    }
}

/// Builder for creating [`User`] instances.
///
/// # Example
///
/// ```
/// use rustgram_user::User;
///
/// let user = User::builder()
///     .with_id(123456789)
///     .with_first_name("Alice")
///     .with_last_name("Smith")
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct UserBuilder {
    id: Option<i64>,
    first_name: Option<String>,
    last_name: Option<String>,
    usernames: Option<Usernames>,
    phone_number: Option<String>,
    status: UserStatus,
    profile_photo: Option<ChatPhoto>,
    accent_color_id: Option<AccentColorId>,
    background_custom_emoji_id: i64,
    upgraded_gift_colors: Option<UpgradedGiftColors>,
    profile_accent_color_id: i32,
    profile_background_custom_emoji_id: i64,
    emoji_status: Option<EmojiStatus>,
    is_contact: bool,
    is_mutual_contact: bool,
    is_close_friend: bool,
    verification_status: Option<VerificationStatus>,
    is_premium: bool,
    is_support: bool,
    restriction_info: Option<RestrictionInfo>,
    active_story_state: Option<ActiveStoryState>,
    restricts_new_chats: bool,
    paid_message_star_count: i64,
    have_access: bool,
    type_: UserType,
    language_code: Option<String>,
    added_to_attachment_menu: bool,
}

impl Default for UserBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl UserBuilder {
    /// Creates a new UserBuilder with default values.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: None,
            first_name: None,
            last_name: None,
            usernames: None,
            phone_number: None,
            status: UserStatus::Empty,
            profile_photo: None,
            accent_color_id: None,
            background_custom_emoji_id: 0,
            upgraded_gift_colors: None,
            profile_accent_color_id: 0,
            profile_background_custom_emoji_id: 0,
            emoji_status: None,
            is_contact: false,
            is_mutual_contact: false,
            is_close_friend: false,
            verification_status: None,
            is_premium: false,
            is_support: false,
            restriction_info: None,
            active_story_state: None,
            restricts_new_chats: false,
            paid_message_star_count: 0,
            have_access: true,
            type_: UserType::Regular,
            language_code: None,
            added_to_attachment_menu: false,
        }
    }

    /// Sets the user identifier.
    #[must_use]
    pub const fn with_id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the first name.
    #[must_use]
    pub fn with_first_name(mut self, first_name: String) -> Self {
        self.first_name = Some(first_name);
        self
    }

    /// Sets the last name.
    #[must_use]
    pub fn with_last_name(mut self, last_name: String) -> Self {
        self.last_name = Some(last_name);
        self
    }

    /// Sets the usernames.
    #[must_use]
    pub fn with_usernames(mut self, usernames: Usernames) -> Self {
        self.usernames = Some(usernames);
        self
    }

    /// Sets the phone number.
    #[must_use]
    pub fn with_phone_number(mut self, phone_number: String) -> Self {
        self.phone_number = Some(phone_number);
        self
    }

    /// Sets the user status.
    #[must_use]
    pub const fn with_status(mut self, status: UserStatus) -> Self {
        self.status = status;
        self
    }

    /// Sets the profile photo.
    #[must_use]
    pub const fn with_profile_photo(mut self, profile_photo: ChatPhoto) -> Self {
        self.profile_photo = Some(profile_photo);
        self
    }

    /// Sets the accent color ID.
    #[must_use]
    pub const fn with_accent_color_id(mut self, accent_color_id: AccentColorId) -> Self {
        self.accent_color_id = Some(accent_color_id);
        self
    }

    /// Sets the background custom emoji ID.
    #[must_use]
    pub const fn with_background_custom_emoji_id(mut self, id: i64) -> Self {
        self.background_custom_emoji_id = id;
        self
    }

    /// Sets the upgraded gift colors.
    #[must_use]
    pub const fn with_upgraded_gift_colors(mut self, colors: UpgradedGiftColors) -> Self {
        self.upgraded_gift_colors = Some(colors);
        self
    }

    /// Sets the profile accent color ID.
    #[must_use]
    pub const fn with_profile_accent_color_id(mut self, id: i32) -> Self {
        self.profile_accent_color_id = id;
        self
    }

    /// Sets the profile background custom emoji ID.
    #[must_use]
    pub const fn with_profile_background_custom_emoji_id(mut self, id: i64) -> Self {
        self.profile_background_custom_emoji_id = id;
        self
    }

    /// Sets the emoji status.
    #[must_use]
    pub const fn with_emoji_status(mut self, emoji_status: EmojiStatus) -> Self {
        self.emoji_status = Some(emoji_status);
        self
    }

    /// Sets whether the user is a contact.
    #[must_use]
    pub const fn with_is_contact(mut self, is_contact: bool) -> Self {
        self.is_contact = is_contact;
        self
    }

    /// Sets whether the user is a mutual contact.
    #[must_use]
    pub const fn with_is_mutual_contact(mut self, is_mutual_contact: bool) -> Self {
        self.is_mutual_contact = is_mutual_contact;
        self
    }

    /// Sets whether the user is a close friend.
    #[must_use]
    pub const fn with_is_close_friend(mut self, is_close_friend: bool) -> Self {
        self.is_close_friend = is_close_friend;
        self
    }

    /// Sets the verification status.
    #[must_use]
    pub const fn with_verification_status(
        mut self,
        verification_status: VerificationStatus,
    ) -> Self {
        self.verification_status = Some(verification_status);
        self
    }

    /// Sets whether the user is Premium.
    #[must_use]
    pub const fn with_is_premium(mut self, is_premium: bool) -> Self {
        self.is_premium = is_premium;
        self
    }

    /// Sets whether the user is support.
    #[must_use]
    pub const fn with_is_support(mut self, is_support: bool) -> Self {
        self.is_support = is_support;
        self
    }

    /// Sets the restriction info.
    #[must_use]
    pub const fn with_restriction_info(mut self, restriction_info: RestrictionInfo) -> Self {
        self.restriction_info = Some(restriction_info);
        self
    }

    /// Sets the active story state.
    #[must_use]
    pub const fn with_active_story_state(mut self, active_story_state: ActiveStoryState) -> Self {
        self.active_story_state = Some(active_story_state);
        self
    }

    /// Sets whether the user restricts new chats.
    #[must_use]
    pub const fn with_restricts_new_chats(mut self, restricts: bool) -> Self {
        self.restricts_new_chats = restricts;
        self
    }

    /// Sets the paid message star count.
    #[must_use]
    pub const fn with_paid_message_star_count(mut self, count: i64) -> Self {
        self.paid_message_star_count = count;
        self
    }

    /// Sets whether the bot has access.
    #[must_use]
    pub const fn with_have_access(mut self, have_access: bool) -> Self {
        self.have_access = have_access;
        self
    }

    /// Sets the user type.
    #[must_use]
    pub const fn with_type(mut self, type_: UserType) -> Self {
        self.type_ = type_;
        self
    }

    /// Sets the language code.
    #[must_use]
    pub fn with_language_code(mut self, language_code: String) -> Self {
        self.language_code = Some(language_code);
        self
    }

    /// Sets whether the user was added to attachment menu.
    #[must_use]
    pub const fn with_added_to_attachment_menu(mut self, added: bool) -> Self {
        self.added_to_attachment_menu = added;
        self
    }

    /// Builds the User.
    ///
    /// Returns an error if required fields (id, first_name) are not set.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `id` is not set
    /// - `first_name` is not set
    pub fn build(self) -> Result<User, String> {
        let id = self.id.ok_or("id is required")?;
        let first_name = self.first_name.ok_or("first_name is required")?;
        let last_name = self.last_name.unwrap_or_default();
        let usernames = self.usernames.unwrap_or_default();
        let phone_number = self.phone_number.unwrap_or_default();
        let accent_color_id = self.accent_color_id.unwrap_or_default();
        let language_code = self.language_code.unwrap_or_else(|| String::from("en"));

        Ok(User {
            id,
            first_name,
            last_name,
            usernames,
            phone_number,
            status: self.status,
            profile_photo: self.profile_photo,
            accent_color_id,
            background_custom_emoji_id: self.background_custom_emoji_id,
            upgraded_gift_colors: self.upgraded_gift_colors,
            profile_accent_color_id: self.profile_accent_color_id,
            profile_background_custom_emoji_id: self.profile_background_custom_emoji_id,
            emoji_status: self.emoji_status,
            is_contact: self.is_contact,
            is_mutual_contact: self.is_mutual_contact,
            is_close_friend: self.is_close_friend,
            verification_status: self.verification_status,
            is_premium: self.is_premium,
            is_support: self.is_support,
            restriction_info: self.restriction_info,
            active_story_state: self.active_story_state,
            restricts_new_chats: self.restricts_new_chats,
            paid_message_star_count: self.paid_message_star_count,
            have_access: self.have_access,
            type_: self.type_,
            language_code,
            added_to_attachment_menu: self.added_to_attachment_menu,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a minimal valid user for testing
    fn create_test_user(id: i64, first_name: &str) -> User {
        User::builder()
            .with_id(id)
            .with_first_name(first_name.to_string())
            .build()
            .unwrap()
    }

    #[test]
    fn test_builder_minimal() {
        let user = create_test_user(123, "Alice");
        assert_eq!(user.id(), 123);
        assert_eq!(user.first_name(), "Alice");
        assert_eq!(user.last_name(), "");
        assert_eq!(user.full_name(), "Alice");
    }

    #[test]
    fn test_builder_full() {
        let user = User::builder()
            .with_id(123456789)
            .with_first_name("Alice".to_string())
            .with_last_name("Smith".to_string())
            .with_is_premium(true)
            .with_is_bot(false)
            .with_type(UserType::Regular)
            .with_status(UserStatus::Online)
            .build()
            .unwrap();

        assert_eq!(user.id(), 123456789);
        assert_eq!(user.first_name(), "Alice");
        assert_eq!(user.last_name(), "Smith");
        assert_eq!(user.full_name(), "Alice Smith");
        assert!(user.is_premium());
        assert!(!user.is_bot());
        assert!(user.is_online());
    }

    #[test]
    fn test_builder_missing_id() {
        let result = User::builder().with_first_name("Alice".to_string()).build();
        assert_eq!(result, Err("id is required".to_string()));
    }

    #[test]
    fn test_builder_missing_first_name() {
        let result = User::builder().with_id(123).build();
        assert_eq!(result, Err("first_name is required".to_string()));
    }

    #[test]
    fn test_full_name() {
        let user1 = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .build()
            .unwrap();
        assert_eq!(user1.full_name(), "Alice");

        let user2 = User::builder()
            .with_id(2)
            .with_first_name("Bob".to_string())
            .with_last_name("Jones".to_string())
            .build()
            .unwrap();
        assert_eq!(user2.full_name(), "Bob Jones");
    }

    #[test]
    fn test_is_deleted() {
        let user1 = create_test_user(1, "Alice");
        assert!(!user1.is_deleted());

        let user2 = User::builder()
            .with_id(2)
            .with_first_name("Deleted".to_string())
            .with_type(UserType::Deleted)
            .build()
            .unwrap();
        assert!(user2.is_deleted());
    }

    #[test]
    fn test_is_bot() {
        let user1 = create_test_user(1, "Alice");
        assert!(!user1.is_bot());

        let user2 = User::builder()
            .with_id(2)
            .with_first_name("Bot".to_string())
            .with_type(UserType::Bot)
            .build()
            .unwrap();
        assert!(user2.is_bot());
    }

    #[test]
    fn test_is_online() {
        let user1 = create_test_user(1, "Alice");
        assert!(!user1.is_online());

        let user2 = User::builder()
            .with_id(2)
            .with_first_name("Bob".to_string())
            .with_status(UserStatus::Online)
            .build()
            .unwrap();
        assert!(user2.is_online());
    }

    #[test]
    fn test_is_contact() {
        let user1 = create_test_user(1, "Alice");
        assert!(!user1.is_contact());

        let user2 = User::builder()
            .with_id(2)
            .with_first_name("Bob".to_string())
            .with_is_contact(true)
            .build()
            .unwrap();
        assert!(user2.is_contact());
    }

    #[test]
    fn test_is_mutual_contact() {
        let user1 = create_test_user(1, "Alice");
        assert!(!user1.is_mutual_contact());

        let user2 = User::builder()
            .with_id(2)
            .with_first_name("Bob".to_string())
            .with_is_mutual_contact(true)
            .build()
            .unwrap();
        assert!(user2.is_mutual_contact());
    }

    #[test]
    fn test_is_close_friend() {
        let user1 = create_test_user(1, "Alice");
        assert!(!user1.is_close_friend());

        let user2 = User::builder()
            .with_id(2)
            .with_first_name("Bob".to_string())
            .with_is_close_friend(true)
            .build()
            .unwrap();
        assert!(user2.is_close_friend());
    }

    #[test]
    fn test_is_premium() {
        let user1 = create_test_user(1, "Alice");
        assert!(!user1.is_premium());

        let user2 = User::builder()
            .with_id(2)
            .with_first_name("Bob".to_string())
            .with_is_premium(true)
            .build()
            .unwrap();
        assert!(user2.is_premium());
    }

    #[test]
    fn test_is_support() {
        let user1 = create_test_user(1, "Alice");
        assert!(!user1.is_support());

        let user2 = User::builder()
            .with_id(2)
            .with_first_name("Telegram".to_string())
            .with_is_support(true)
            .build()
            .unwrap();
        assert!(user2.is_support());
    }

    #[test]
    fn test_user_type_variants() {
        let regular = User::builder()
            .with_id(1)
            .with_first_name("Regular".to_string())
            .with_type(UserType::Regular)
            .build()
            .unwrap();
        assert!(!regular.is_deleted());
        assert!(!regular.is_bot());

        let deleted = User::builder()
            .with_id(2)
            .with_first_name("Deleted".to_string())
            .with_type(UserType::Deleted)
            .build()
            .unwrap();
        assert!(deleted.is_deleted());
        assert!(!deleted.is_bot());

        let bot = User::builder()
            .with_id(3)
            .with_first_name("Bot".to_string())
            .with_type(UserType::Bot)
            .build()
            .unwrap();
        assert!(!bot.is_deleted());
        assert!(bot.is_bot());

        let unknown = User::builder()
            .with_id(4)
            .with_first_name("Unknown".to_string())
            .with_type(UserType::Unknown)
            .build()
            .unwrap();
        assert!(!unknown.is_deleted());
        assert!(!unknown.is_bot());
    }

    #[test]
    fn test_user_status_variants() {
        let empty = User::builder()
            .with_id(1)
            .with_first_name("User".to_string())
            .with_status(UserStatus::Empty)
            .build()
            .unwrap();
        assert!(!empty.is_online());

        let online = User::builder()
            .with_id(2)
            .with_first_name("User".to_string())
            .with_status(UserStatus::Online)
            .build()
            .unwrap();
        assert!(online.is_online());

        let offline = User::builder()
            .with_id(3)
            .with_first_name("User".to_string())
            .with_status(UserStatus::Offline)
            .build()
            .unwrap();
        assert!(!offline.is_online());

        let recently = User::builder()
            .with_id(4)
            .with_first_name("User".to_string())
            .with_status(UserStatus::Recently)
            .build()
            .unwrap();
        assert!(!recently.is_online());
    }

    #[test]
    fn test_equality() {
        let user1 = create_test_user(1, "Alice");
        let user2 = create_test_user(1, "Alice");
        assert_eq!(user1, user2);

        let user3 = create_test_user(2, "Bob");
        assert_ne!(user1, user3);
    }

    #[test]
    fn test_clone() {
        let user1 = create_test_user(1, "Alice");
        let user2 = user1.clone();
        assert_eq!(user1, user2);
    }

    #[test]
    fn test_display() {
        let user = create_test_user(123, "Alice");
        let display = format!("{user}");
        assert!(display.contains("123"));
        assert!(display.contains("Alice"));
        assert!(display.contains("User"));
    }

    #[test]
    fn test_debug_formatting() {
        let user = create_test_user(123, "Alice");
        let debug = format!("{user:?}");
        assert!(debug.contains("123"));
        assert!(debug.contains("Alice"));
    }

    #[test]
    fn test_accent_color_id() {
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_accent_color_id(AccentColorId::new(5))
            .build()
            .unwrap();
        assert_eq!(user.accent_color_id().0, 5);
    }

    #[test]
    fn test_profile_photo() {
        let photo = ChatPhoto::new(12345, 1234567890);
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_profile_photo(photo.clone())
            .build()
            .unwrap();
        assert_eq!(user.profile_photo(), &Some(photo));
        assert_eq!(user.profile_photo().as_ref().unwrap().id(), 12345);
    }

    #[test]
    fn test_emoji_status() {
        let emoji = EmojiStatus::new();
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_emoji_status(emoji.clone())
            .build()
            .unwrap();
        assert_eq!(user.emoji_status(), &Some(emoji));
    }

    #[test]
    fn test_verification_status() {
        let verification = VerificationStatus::new();
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_verification_status(verification.clone())
            .build()
            .unwrap();
        assert_eq!(user.verification_status(), &Some(verification));
    }

    #[test]
    fn test_restriction_info() {
        let restriction = RestrictionInfo::new("sensitive".to_string(), true);
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_restriction_info(restriction.clone())
            .build()
            .unwrap();
        assert_eq!(user.restriction_info(), &Some(restriction));
        assert!(user
            .restriction_info()
            .as_ref()
            .unwrap()
            .has_sensitive_content());
    }

    #[test]
    fn test_active_story_state() {
        let story_state = ActiveStoryState::new();
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_active_story_state(story_state.clone())
            .build()
            .unwrap();
        assert_eq!(user.active_story_state(), &Some(story_state));
    }

    #[test]
    fn test_upgraded_gift_colors() {
        let colors = UpgradedGiftColors::new(12345);
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_upgraded_gift_colors(colors.clone())
            .build()
            .unwrap();
        assert_eq!(user.upgraded_gift_colors(), &Some(colors));
        assert_eq!(user.upgraded_gift_colors().as_ref().unwrap().id(), 12345);
    }

    #[test]
    fn test_background_custom_emoji_id() {
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_background_custom_emoji_id(12345)
            .build()
            .unwrap();
        assert_eq!(user.background_custom_emoji_id(), 12345);
    }

    #[test]
    fn test_profile_accent_color_id() {
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_profile_accent_color_id(10)
            .build()
            .unwrap();
        assert_eq!(user.profile_accent_color_id(), 10);
    }

    #[test]
    fn test_profile_background_custom_emoji_id() {
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_profile_background_custom_emoji_id(54321)
            .build()
            .unwrap();
        assert_eq!(user.profile_background_custom_emoji_id(), 54321);
    }

    #[test]
    fn test_restricts_new_chats() {
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_restricts_new_chats(true)
            .build()
            .unwrap();
        assert!(user.restricts_new_chats());
    }

    #[test]
    fn test_paid_message_star_count() {
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_paid_message_star_count(100)
            .build()
            .unwrap();
        assert_eq!(user.paid_message_star_count(), 100);
    }

    #[test]
    fn test_have_access() {
        let user1 = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_have_access(true)
            .build()
            .unwrap();
        assert!(user1.have_access());

        let user2 = User::builder()
            .with_id(2)
            .with_first_name("Bob".to_string())
            .with_have_access(false)
            .build()
            .unwrap();
        assert!(!user2.have_access());
    }

    #[test]
    fn test_language_code() {
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_language_code("en".to_string())
            .build()
            .unwrap();
        assert_eq!(user.language_code(), "en");
    }

    #[test]
    fn test_default_language_code() {
        let user = create_test_user(1, "Alice");
        assert_eq!(user.language_code(), "en");
    }

    #[test]
    fn test_added_to_attachment_menu() {
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_added_to_attachment_menu(true)
            .build()
            .unwrap();
        assert!(user.added_to_attachment_menu());
    }

    #[test]
    fn test_phone_number() {
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_phone_number("+1234567890".to_string())
            .build()
            .unwrap();
        assert_eq!(user.phone_number(), "+1234567890");
    }

    #[test]
    fn test_usernames() {
        let usernames = Usernames::new();
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_usernames(usernames.clone())
            .build()
            .unwrap();
        assert_eq!(user.usernames(), &usernames);
    }

    #[test]
    fn test_serialization() {
        let user = create_test_user(123, "Alice");
        let json = serde_json::to_string(&user).expect("Failed to serialize");
        assert!(json.contains("123"));
        assert!(json.contains("Alice"));

        let deserialized: User = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, user);
    }

    #[test]
    fn test_chat_photo_stub() {
        let photo = ChatPhoto::new(12345, 1234567890);
        assert_eq!(photo.id(), 12345);
        assert_eq!(photo.added_date(), 1234567890);
        assert_eq!(photo.id(), photo.id());
    }

    #[test]
    fn test_restriction_info_stub() {
        let info = RestrictionInfo::new("sensitive".to_string(), true);
        assert_eq!(info.restriction_reason(), "sensitive");
        assert!(info.has_sensitive_content());
    }

    #[test]
    fn test_upgraded_gift_colors_stub() {
        let colors = UpgradedGiftColors::new(12345);
        assert_eq!(colors.id(), 12345);
    }

    #[test]
    fn test_user_status_default() {
        let status = UserStatus::default();
        assert!(matches!(status, UserStatus::Empty));
    }

    #[test]
    fn test_user_type_default() {
        let type_ = UserType::default();
        assert!(matches!(type_, UserType::Regular));
    }

    #[test]
    fn test_builder_chaining() {
        let user = User::builder()
            .with_id(1)
            .with_first_name("Alice".to_string())
            .with_last_name("Smith".to_string())
            .with_is_premium(true)
            .with_is_bot(false)
            .with_type(UserType::Regular)
            .with_status(UserStatus::Online)
            .with_accent_color_id(AccentColorId::new(5))
            .with_language_code("en".to_string())
            .build()
            .unwrap();

        assert_eq!(user.id(), 1);
        assert_eq!(user.first_name(), "Alice");
        assert_eq!(user.last_name(), "Smith");
        assert!(user.is_premium());
        assert!(!user.is_bot());
        assert!(user.is_online());
    }
}
