// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! Requested dialog type for Telegram bot keyboard buttons.
//!
//! This module provides the [`RequestedDialogType`] struct, which represents
//! the type of dialog requested from a user via a keyboard button. Bots can
//! request users to select and share chats (users, groups, or channels) with
//! the bot, with various validation rules.
//!
//! ## Overview
//!
//! When a bot wants to receive information about a chat from a user, it can
//! send a keyboard button that requests the user to select a chat. The
//! [`RequestedDialogType`] specifies what kind of chat can be selected and
//! any restrictions on the selection.
//!
//! ## Dialog Types
//!
//! - **User**: Individual user accounts, with optional bot/premium filters
//! - **Group**: Group chats, with optional forum/username restrictions
//! - **Channel**: Channels, with optional username/created restrictions
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_requested_dialog_type::{RequestedDialogType, DialogType};
//!
//! // Create a user dialog request (default)
//! let user_req = RequestedDialogType::default();
//! assert_eq!(user_req.get_type(), DialogType::User);
//!
//! // Create a group dialog request
//! let group_req = RequestedDialogType::new_group();
//! assert_eq!(group_req.get_type(), DialogType::Group);
//!
//! // Create a channel dialog request
//! let channel_req = RequestedDialogType::new_channel();
//! assert_eq!(channel_req.get_type(), DialogType::Channel);
//! ```
//!
//! ## TDLib Alignment
//!
//! This struct aligns with TDLib's `RequestedDialogType` class:
//! - Field names match TDLib exactly
//! - All three dialog types are supported
//! - Administrator rights are stubbed (full implementation pending)

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::fmt;

/// Dialog type enumeration.
///
/// Represents the three main types of dialogs in Telegram.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum DialogType {
    /// User dialog (individual user account).
    #[default]
    User = 0,
    /// Group dialog (group chat).
    Group = 1,
    /// Channel dialog (channel).
    Channel = 2,
}

impl fmt::Display for DialogType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User => write!(f, "User"),
            Self::Group => write!(f, "Group"),
            Self::Channel => write!(f, "Channel"),
        }
    }
}

/// Stub for AdministratorRights.
///
/// TODO: Full implementation when rustgram-administrator-rights is available.
///
/// This stub provides the minimal structure needed for RequestedDialogType.
/// The full implementation would contain bot permission flags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdministratorRights {
    /// Administrator permission flags.
    pub flags: u32,
}

impl AdministratorRights {
    /// Create a new AdministratorRights stub.
    #[must_use]
    pub const fn new(flags: u32) -> Self {
        Self { flags }
    }

    /// Get the flags.
    #[must_use]
    pub const fn flags(&self) -> u32 {
        self.flags
    }
}

impl Default for AdministratorRights {
    fn default() -> Self {
        Self::new(0)
    }
}

/// Requested dialog type for bot keyboard buttons.
///
/// Contains configuration for what type of dialog a bot can request from a user,
/// along with validation rules for the selected dialog.
///
/// # Example
///
/// ```rust
/// use rustgram_requested_dialog_type::RequestedDialogType;
///
/// // Create default user request
/// let req = RequestedDialogType::default();
/// assert_eq!(req.get_button_id(), 0);
/// assert_eq!(req.max_quantity(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestedDialogType {
    /// The type of dialog (User, Group, Channel).
    type_: DialogType,
    /// Button identifier for tracking the request.
    button_id: i32,
    /// Maximum number of dialogs that can be selected (User only).
    max_quantity: i32,
    /// Whether to restrict to bot users (User only).
    restrict_is_bot: bool,
    /// Whether the user must be a bot (User only).
    is_bot: bool,
    /// Whether to restrict to premium users (User only).
    restrict_is_premium: bool,
    /// Whether the user must have premium (User only).
    is_premium: bool,
    /// Whether to request the name.
    request_name: bool,
    /// Whether to request the username.
    request_username: bool,
    /// Whether to request the photo.
    request_photo: bool,
    /// Whether to restrict to forum groups (Group only).
    restrict_is_forum: bool,
    /// Whether the group must be a forum (Group only).
    is_forum: bool,
    /// Whether bot must be a participant (Group only).
    bot_is_participant: bool,
    /// Whether to restrict to groups/channels with username (Group/Channel only).
    restrict_has_username: bool,
    /// Whether the group/channel must have a username (Group/Channel only).
    has_username: bool,
    /// Whether the group/channel must be created by the user (Group/Channel only).
    is_created: bool,
    /// Whether to restrict user administrator rights (Group/Channel only).
    restrict_user_administrator_rights: bool,
    /// Whether to restrict bot administrator rights (Group/Channel only).
    restrict_bot_administrator_rights: bool,
    /// User administrator rights (Group/Channel only).
    user_administrator_rights: AdministratorRights,
    /// Bot administrator rights (Group/Channel only).
    bot_administrator_rights: AdministratorRights,
}

impl RequestedDialogType {
    /// Creates a new user dialog request type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_user();
    /// assert_eq!(req.get_type().to_string(), "User");
    /// ```
    #[must_use]
    pub fn new_user() -> Self {
        Self {
            type_: DialogType::User,
            button_id: 0,
            max_quantity: 1,
            restrict_is_bot: false,
            is_bot: false,
            restrict_is_premium: false,
            is_premium: false,
            request_name: false,
            request_username: false,
            request_photo: false,
            restrict_is_forum: false,
            is_forum: false,
            bot_is_participant: false,
            restrict_has_username: false,
            has_username: false,
            is_created: false,
            restrict_user_administrator_rights: false,
            restrict_bot_administrator_rights: false,
            user_administrator_rights: AdministratorRights::default(),
            bot_administrator_rights: AdministratorRights::default(),
        }
    }

    /// Creates a new group dialog request type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_group();
    /// assert_eq!(req.get_type().to_string(), "Group");
    /// ```
    #[must_use]
    pub fn new_group() -> Self {
        Self {
            type_: DialogType::Group,
            button_id: 0,
            max_quantity: 1,
            restrict_is_bot: false,
            is_bot: false,
            restrict_is_premium: false,
            is_premium: false,
            request_name: false,
            request_username: false,
            request_photo: false,
            restrict_is_forum: false,
            is_forum: false,
            bot_is_participant: false,
            restrict_has_username: false,
            has_username: false,
            is_created: false,
            restrict_user_administrator_rights: false,
            restrict_bot_administrator_rights: false,
            user_administrator_rights: AdministratorRights::default(),
            bot_administrator_rights: AdministratorRights::default(),
        }
    }

    /// Creates a new channel dialog request type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_channel();
    /// assert_eq!(req.get_type().to_string(), "Channel");
    /// ```
    #[must_use]
    pub fn new_channel() -> Self {
        Self {
            type_: DialogType::Channel,
            button_id: 0,
            max_quantity: 1,
            restrict_is_bot: false,
            is_bot: false,
            restrict_is_premium: false,
            is_premium: false,
            request_name: false,
            request_username: false,
            request_photo: false,
            restrict_is_forum: false,
            is_forum: false,
            bot_is_participant: false,
            restrict_has_username: false,
            has_username: false,
            is_created: false,
            restrict_user_administrator_rights: false,
            restrict_bot_administrator_rights: false,
            user_administrator_rights: AdministratorRights::default(),
            bot_administrator_rights: AdministratorRights::default(),
        }
    }

    /// Sets the button ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_user().with_button_id(42);
    /// assert_eq!(req.get_button_id(), 42);
    /// ```
    #[must_use]
    pub const fn with_button_id(mut self, button_id: i32) -> Self {
        self.button_id = button_id;
        self
    }

    /// Sets the maximum quantity for user selection.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_user().with_max_quantity(10);
    /// assert_eq!(req.max_quantity(), 10);
    /// ```
    #[must_use]
    pub const fn with_max_quantity(mut self, max_quantity: i32) -> Self {
        self.max_quantity = max_quantity;
        self
    }

    /// Sets whether to restrict to bots.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_user().with_restrict_is_bot(true);
    /// assert!(req.restrict_is_bot());
    /// ```
    #[must_use]
    pub const fn with_restrict_is_bot(mut self, restrict: bool) -> Self {
        self.restrict_is_bot = restrict;
        self
    }

    /// Sets whether the user must be a bot.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_user().with_is_bot(true);
    /// assert!(req.is_bot());
    /// ```
    #[must_use]
    pub const fn with_is_bot(mut self, is_bot: bool) -> Self {
        self.is_bot = is_bot;
        self
    }

    /// Sets whether to restrict to premium users.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_user().with_restrict_is_premium(true);
    /// assert!(req.restrict_is_premium());
    /// ```
    #[must_use]
    pub const fn with_restrict_is_premium(mut self, restrict: bool) -> Self {
        self.restrict_is_premium = restrict;
        self
    }

    /// Sets whether the user must have premium.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_user().with_is_premium(true);
    /// assert!(req.is_premium());
    /// ```
    #[must_use]
    pub const fn with_is_premium(mut self, is_premium: bool) -> Self {
        self.is_premium = is_premium;
        self
    }

    /// Sets whether to request the name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_user().with_request_name(true);
    /// assert!(req.request_name());
    /// ```
    #[must_use]
    pub const fn with_request_name(mut self, request: bool) -> Self {
        self.request_name = request;
        self
    }

    /// Sets whether to request the username.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_user().with_request_username(true);
    /// assert!(req.request_username());
    /// ```
    #[must_use]
    pub const fn with_request_username(mut self, request: bool) -> Self {
        self.request_username = request;
        self
    }

    /// Sets whether to request the photo.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_user().with_request_photo(true);
    /// assert!(req.request_photo());
    /// ```
    #[must_use]
    pub const fn with_request_photo(mut self, request: bool) -> Self {
        self.request_photo = request;
        self
    }

    /// Sets whether to restrict to forum groups.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_group().with_restrict_is_forum(true);
    /// assert!(req.restrict_is_forum());
    /// ```
    #[must_use]
    pub const fn with_restrict_is_forum(mut self, restrict: bool) -> Self {
        self.restrict_is_forum = restrict;
        self
    }

    /// Sets whether the group must be a forum.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_group().with_is_forum(true);
    /// assert!(req.is_forum());
    /// ```
    #[must_use]
    pub const fn with_is_forum(mut self, is_forum: bool) -> Self {
        self.is_forum = is_forum;
        self
    }

    /// Sets whether the bot must be a participant.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_group().with_bot_is_participant(true);
    /// assert!(req.bot_is_participant());
    /// ```
    #[must_use]
    pub const fn with_bot_is_participant(mut self, is_participant: bool) -> Self {
        self.bot_is_participant = is_participant;
        self
    }

    /// Sets whether to restrict to groups/channels with username.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_channel().with_restrict_has_username(true);
    /// assert!(req.restrict_has_username());
    /// ```
    #[must_use]
    pub const fn with_restrict_has_username(mut self, restrict: bool) -> Self {
        self.restrict_has_username = restrict;
        self
    }

    /// Sets whether the group/channel must have a username.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_channel().with_has_username(true);
    /// assert!(req.has_username());
    /// ```
    #[must_use]
    pub const fn with_has_username(mut self, has_username: bool) -> Self {
        self.has_username = has_username;
        self
    }

    /// Sets whether the group/channel must be created by the user.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_channel().with_is_created(true);
    /// assert!(req.is_created());
    /// ```
    #[must_use]
    pub const fn with_is_created(mut self, is_created: bool) -> Self {
        self.is_created = is_created;
        self
    }

    /// Returns the button ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::RequestedDialogType;
    ///
    /// let req = RequestedDialogType::new_user().with_button_id(123);
    /// assert_eq!(req.get_button_id(), 123);
    /// ```
    #[must_use]
    pub const fn get_button_id(&self) -> i32 {
        self.button_id
    }

    /// Returns the dialog type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_requested_dialog_type::{RequestedDialogType, DialogType};
    ///
    /// let req = RequestedDialogType::new_user();
    /// assert_eq!(req.get_type(), DialogType::User);
    /// ```
    #[must_use]
    pub const fn get_type(&self) -> DialogType {
        self.type_
    }

    /// Returns the maximum quantity for user selection.
    #[must_use]
    pub const fn max_quantity(&self) -> i32 {
        self.max_quantity
    }

    /// Returns whether to restrict to bot users.
    #[must_use]
    pub const fn restrict_is_bot(&self) -> bool {
        self.restrict_is_bot
    }

    /// Returns whether the user must be a bot.
    #[must_use]
    pub const fn is_bot(&self) -> bool {
        self.is_bot
    }

    /// Returns whether to restrict to premium users.
    #[must_use]
    pub const fn restrict_is_premium(&self) -> bool {
        self.restrict_is_premium
    }

    /// Returns whether the user must have premium.
    #[must_use]
    pub const fn is_premium(&self) -> bool {
        self.is_premium
    }

    /// Returns whether to request the name.
    #[must_use]
    pub const fn request_name(&self) -> bool {
        self.request_name
    }

    /// Returns whether to request the username.
    #[must_use]
    pub const fn request_username(&self) -> bool {
        self.request_username
    }

    /// Returns whether to request the photo.
    #[must_use]
    pub const fn request_photo(&self) -> bool {
        self.request_photo
    }

    /// Returns whether to restrict to forum groups.
    #[must_use]
    pub const fn restrict_is_forum(&self) -> bool {
        self.restrict_is_forum
    }

    /// Returns whether the group must be a forum.
    #[must_use]
    pub const fn is_forum(&self) -> bool {
        self.is_forum
    }

    /// Returns whether the bot must be a participant.
    #[must_use]
    pub const fn bot_is_participant(&self) -> bool {
        self.bot_is_participant
    }

    /// Returns whether to restrict to groups/channels with username.
    #[must_use]
    pub const fn restrict_has_username(&self) -> bool {
        self.restrict_has_username
    }

    /// Returns whether the group/channel has a username.
    #[must_use]
    pub const fn has_username(&self) -> bool {
        self.has_username
    }

    /// Returns whether the group/channel must be created by the user.
    #[must_use]
    pub const fn is_created(&self) -> bool {
        self.is_created
    }

    /// Returns the user administrator rights.
    #[must_use]
    pub const fn user_administrator_rights(&self) -> &AdministratorRights {
        &self.user_administrator_rights
    }

    /// Returns the bot administrator rights.
    #[must_use]
    pub const fn bot_administrator_rights(&self) -> &AdministratorRights {
        &self.bot_administrator_rights
    }
}

impl Default for RequestedDialogType {
    fn default() -> Self {
        Self::new_user()
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-requested-dialog-type";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-requested-dialog-type");
    }

    // DialogType tests
    #[test]
    fn test_dialog_type_user() {
        let dt = DialogType::User;
        assert_eq!(dt as i32, 0);
        assert_eq!(dt.to_string().as_str(), "User");
    }

    #[test]
    fn test_dialog_type_group() {
        let dt = DialogType::Group;
        assert_eq!(dt as i32, 1);
        assert_eq!(dt.to_string().as_str(), "Group");
    }

    #[test]
    fn test_dialog_type_channel() {
        let dt = DialogType::Channel;
        assert_eq!(dt as i32, 2);
        assert_eq!(dt.to_string().as_str(), "Channel");
    }

    #[test]
    fn test_dialog_type_default() {
        assert_eq!(DialogType::default(), DialogType::User);
    }

    // AdministratorRights stub tests
    #[test]
    fn test_administrator_rights_new() {
        let rights = AdministratorRights::new(42);
        assert_eq!(rights.flags(), 42);
    }

    #[test]
    fn test_administrator_rights_default() {
        let rights = AdministratorRights::default();
        assert_eq!(rights.flags(), 0);
    }

    #[test]
    fn test_administrator_rights_clone() {
        let rights1 = AdministratorRights::new(100);
        let rights2 = rights1.clone();
        assert_eq!(rights1.flags(), rights2.flags());
    }

    #[test]
    fn test_administrator_rights_equality() {
        let rights1 = AdministratorRights::new(50);
        let rights2 = AdministratorRights::new(50);
        let rights3 = AdministratorRights::new(51);
        assert_eq!(rights1, rights2);
        assert_ne!(rights1, rights3);
    }

    // RequestedDialogType constructor tests
    #[test]
    fn test_new_user() {
        let req = RequestedDialogType::new_user();
        assert_eq!(req.get_type(), DialogType::User);
        assert_eq!(req.get_button_id(), 0);
        assert_eq!(req.max_quantity(), 1);
        assert!(!req.restrict_is_bot());
        assert!(!req.is_bot());
        assert!(!req.restrict_is_premium());
        assert!(!req.is_premium());
    }

    #[test]
    fn test_new_group() {
        let req = RequestedDialogType::new_group();
        assert_eq!(req.get_type(), DialogType::Group);
        assert_eq!(req.get_button_id(), 0);
        assert!(!req.restrict_is_forum());
        assert!(!req.is_forum());
        assert!(!req.bot_is_participant());
    }

    #[test]
    fn test_new_channel() {
        let req = RequestedDialogType::new_channel();
        assert_eq!(req.get_type(), DialogType::Channel);
        assert_eq!(req.get_button_id(), 0);
        assert!(!req.restrict_has_username());
        assert!(!req.has_username());
        assert!(!req.is_created());
    }

    #[test]
    fn test_default() {
        let req = RequestedDialogType::default();
        assert_eq!(req.get_type(), DialogType::User);
    }

    // Builder method tests
    #[test]
    fn test_with_button_id() {
        let req = RequestedDialogType::new_user().with_button_id(999);
        assert_eq!(req.get_button_id(), 999);
    }

    #[test]
    fn test_with_max_quantity() {
        let req = RequestedDialogType::new_user().with_max_quantity(10);
        assert_eq!(req.max_quantity(), 10);
    }

    #[test]
    fn test_with_restrict_is_bot() {
        let req = RequestedDialogType::new_user().with_restrict_is_bot(true);
        assert!(req.restrict_is_bot());
    }

    #[test]
    fn test_with_is_bot() {
        let req = RequestedDialogType::new_user().with_is_bot(true);
        assert!(req.is_bot());
    }

    #[test]
    fn test_with_restrict_is_premium() {
        let req = RequestedDialogType::new_user().with_restrict_is_premium(true);
        assert!(req.restrict_is_premium());
    }

    #[test]
    fn test_with_is_premium() {
        let req = RequestedDialogType::new_user().with_is_premium(true);
        assert!(req.is_premium());
    }

    #[test]
    fn test_with_request_name() {
        let req = RequestedDialogType::new_user().with_request_name(true);
        assert!(req.request_name());
    }

    #[test]
    fn test_with_request_username() {
        let req = RequestedDialogType::new_user().with_request_username(true);
        assert!(req.request_username());
    }

    #[test]
    fn test_with_request_photo() {
        let req = RequestedDialogType::new_user().with_request_photo(true);
        assert!(req.request_photo());
    }

    #[test]
    fn test_with_restrict_is_forum() {
        let req = RequestedDialogType::new_group().with_restrict_is_forum(true);
        assert!(req.restrict_is_forum());
    }

    #[test]
    fn test_with_is_forum() {
        let req = RequestedDialogType::new_group().with_is_forum(true);
        assert!(req.is_forum());
    }

    #[test]
    fn test_with_bot_is_participant() {
        let req = RequestedDialogType::new_group().with_bot_is_participant(true);
        assert!(req.bot_is_participant());
    }

    #[test]
    fn test_with_restrict_has_username() {
        let req = RequestedDialogType::new_channel().with_restrict_has_username(true);
        assert!(req.restrict_has_username());
    }

    #[test]
    fn test_with_has_username() {
        let req = RequestedDialogType::new_channel().with_has_username(true);
        assert!(req.has_username());
    }

    #[test]
    fn test_with_is_created() {
        let req = RequestedDialogType::new_channel().with_is_created(true);
        assert!(req.is_created());
    }

    // Chained builder tests
    #[test]
    fn test_chained_builder_user() {
        let req = RequestedDialogType::new_user()
            .with_button_id(1)
            .with_max_quantity(5)
            .with_is_bot(true)
            .with_is_premium(true)
            .with_request_name(true)
            .with_request_username(true)
            .with_request_photo(true);

        assert_eq!(req.get_button_id(), 1);
        assert_eq!(req.max_quantity(), 5);
        assert!(req.is_bot());
        assert!(req.is_premium());
        assert!(req.request_name());
        assert!(req.request_username());
        assert!(req.request_photo());
    }

    #[test]
    fn test_chained_builder_group() {
        let req = RequestedDialogType::new_group()
            .with_button_id(2)
            .with_is_forum(true)
            .with_bot_is_participant(true)
            .with_has_username(true);

        assert_eq!(req.get_button_id(), 2);
        assert!(req.is_forum());
        assert!(req.bot_is_participant());
        assert!(req.has_username());
    }

    #[test]
    fn test_chained_builder_channel() {
        let req = RequestedDialogType::new_channel()
            .with_button_id(3)
            .with_has_username(true)
            .with_is_created(true);

        assert_eq!(req.get_button_id(), 3);
        assert!(req.has_username());
        assert!(req.is_created());
    }

    // Trait tests
    #[test]
    fn test_clone() {
        let req1 = RequestedDialogType::new_user().with_button_id(42);
        let req2 = req1.clone();
        assert_eq!(req1, req2);
    }

    #[test]
    fn test_equality() {
        let req1 = RequestedDialogType::new_user().with_button_id(1);
        let req2 = RequestedDialogType::new_user().with_button_id(1);
        assert_eq!(req1, req2);
    }

    #[test]
    fn test_inequality_type() {
        let req1 = RequestedDialogType::new_user();
        let req2 = RequestedDialogType::new_group();
        assert_ne!(req1, req2);
    }

    #[test]
    fn test_inequality_button_id() {
        let req1 = RequestedDialogType::new_user().with_button_id(1);
        let req2 = RequestedDialogType::new_user().with_button_id(2);
        assert_ne!(req1, req2);
    }

    #[test]
    fn test_debug() {
        let req = RequestedDialogType::new_user().with_button_id(123);
        let debug_str = format!("{:?}", req);
        assert!(debug_str.contains("RequestedDialogType"));
        assert!(debug_str.contains("123"));
    }
}
