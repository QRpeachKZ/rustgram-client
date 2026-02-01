// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Types for business connected bots.
//!
//! This module defines the core types for representing bots connected to
//! business accounts in Telegram, including their rights and recipients.

use rustgram_types::UserId;
use std::fmt;

/// Individual bot right/permission.
///
/// Represents a specific permission that a business bot can have.
/// These rights control what actions the bot can perform on behalf of
/// the business account.
///
/// # Correspondence
///
/// | Rust variant | TDLib field | Description |
/// |--------------|-------------|-------------|
/// | `CanReply` | `can_reply` | Can reply to messages |
/// | `CanReadMessages` | `can_read_messages` | Can read messages |
/// | `CanDeleteSentMessages` | `can_delete_sent_messages` | Can delete sent messages |
/// | `CanDeleteReceivedMessages` | `can_delete_received_messages` | Can delete received messages |
/// | `CanEditName` | `can_edit_name` | Can edit account name |
/// | `CanEditBio` | `can_edit_bio` | Can edit account bio |
/// | `CanEditProfilePhoto` | `can_edit_profile_photo` | Can edit profile photo |
/// | `CanEditUsername` | `can_edit_username` | Can edit username |
/// | `CanViewGifts` | `can_view_gifts` | Can view gifts and stars |
/// | `CanSellGifts` | `can_sell_gifts` | Can sell gifts |
/// | `CanChangeGiftSettings` | `can_change_gift_settings` | Can change gift settings |
/// | `CanTransferAndUpgradeGifts` | `can_transfer_and_upgrade_gifts` | Can transfer and upgrade gifts |
/// | `CanTransferStars` | `can_transfer_stars` | Can transfer stars |
/// | `CanManageStories` | `can_manage_stories` | Can manage stories |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BusinessBotRight {
    /// Can reply to messages
    CanReply,
    /// Can read messages
    CanReadMessages,
    /// Can delete sent messages
    CanDeleteSentMessages,
    /// Can delete received messages
    CanDeleteReceivedMessages,
    /// Can edit account name
    CanEditName,
    /// Can edit account bio
    CanEditBio,
    /// Can edit profile photo
    CanEditProfilePhoto,
    /// Can edit username
    CanEditUsername,
    /// Can view gifts and stars
    CanViewGifts,
    /// Can sell gifts
    CanSellGifts,
    /// Can change gift settings
    CanChangeGiftSettings,
    /// Can transfer and upgrade gifts
    CanTransferAndUpgradeGifts,
    /// Can transfer stars
    CanTransferStars,
    /// Can manage stories
    CanManageStories,
}

impl fmt::Display for BusinessBotRight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BusinessBotRight::CanReply => write!(f, "can_reply"),
            BusinessBotRight::CanReadMessages => write!(f, "can_read_messages"),
            BusinessBotRight::CanDeleteSentMessages => write!(f, "can_delete_sent_messages"),
            BusinessBotRight::CanDeleteReceivedMessages => {
                write!(f, "can_delete_received_messages")
            }
            BusinessBotRight::CanEditName => write!(f, "can_edit_name"),
            BusinessBotRight::CanEditBio => write!(f, "can_edit_bio"),
            BusinessBotRight::CanEditProfilePhoto => write!(f, "can_edit_profile_photo"),
            BusinessBotRight::CanEditUsername => write!(f, "can_edit_username"),
            BusinessBotRight::CanViewGifts => write!(f, "can_view_gifts"),
            BusinessBotRight::CanSellGifts => write!(f, "can_sell_gifts"),
            BusinessBotRight::CanChangeGiftSettings => write!(f, "can_change_gift_settings"),
            BusinessBotRight::CanTransferAndUpgradeGifts => {
                write!(f, "can_transfer_and_upgrade_gifts")
            }
            BusinessBotRight::CanTransferStars => write!(f, "can_transfer_stars"),
            BusinessBotRight::CanManageStories => write!(f, "can_manage_stories"),
        }
    }
}

/// Collection of rights/permissions for a business bot.
///
/// Contains 14 boolean flags representing different permissions that
/// a connected business bot can have. These rights control what the
/// bot is allowed to do on behalf of the business account.
///
/// # Examples
///
/// ```
/// use rustgram_business_connected_bot::BusinessBotRights;
///
/// // Create empty rights (all false)
/// let rights = BusinessBotRights::new();
/// assert!(!rights.can_reply());
/// assert!(!rights.can_read_messages());
///
/// // Create with all rights enabled
/// let all_rights = BusinessBotRights::all();
/// assert!(all_rights.can_reply());
/// assert!(all_rights.can_read_messages());
///
/// // Create with specific right
/// let mut rights = BusinessBotRights::new();
/// rights.set_can_reply(true);
/// assert!(rights.can_reply());
/// ```
///
/// # Correspondence
///
/// Corresponds to TDLib's `td::BusinessBotRights` class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BusinessBotRights {
    can_reply: bool,
    can_read_messages: bool,
    can_delete_sent_messages: bool,
    can_delete_received_messages: bool,
    can_edit_name: bool,
    can_edit_bio: bool,
    can_edit_profile_photo: bool,
    can_edit_username: bool,
    can_view_gifts: bool,
    can_sell_gifts: bool,
    can_change_gift_settings: bool,
    can_transfer_and_upgrade_gifts: bool,
    can_transfer_stars: bool,
    can_manage_stories: bool,
}

impl BusinessBotRights {
    /// Creates a new `BusinessBotRights` with all rights set to `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::BusinessBotRights;
    ///
    /// let rights = BusinessBotRights::new();
    /// assert!(!rights.can_reply());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            can_reply: false,
            can_read_messages: false,
            can_delete_sent_messages: false,
            can_delete_received_messages: false,
            can_edit_name: false,
            can_edit_bio: false,
            can_edit_profile_photo: false,
            can_edit_username: false,
            can_view_gifts: false,
            can_sell_gifts: false,
            can_change_gift_settings: false,
            can_transfer_and_upgrade_gifts: false,
            can_transfer_stars: false,
            can_manage_stories: false,
        }
    }

    /// Creates a new `BusinessBotRights` with all rights set to `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::BusinessBotRights;
    ///
    /// let rights = BusinessBotRights::all();
    /// assert!(rights.can_reply());
    /// assert!(rights.can_read_messages());
    /// ```
    #[must_use]
    pub const fn all() -> Self {
        Self {
            can_reply: true,
            can_read_messages: true,
            can_delete_sent_messages: true,
            can_delete_received_messages: true,
            can_edit_name: true,
            can_edit_bio: true,
            can_edit_profile_photo: true,
            can_edit_username: true,
            can_view_gifts: true,
            can_sell_gifts: true,
            can_change_gift_settings: true,
            can_transfer_and_upgrade_gifts: true,
            can_transfer_stars: true,
            can_manage_stories: true,
        }
    }

    /// Checks if the bot has the specified right.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::{BusinessBotRights, BusinessBotRight};
    ///
    /// let mut rights = BusinessBotRights::new();
    /// rights.set_can_reply(true);
    ///
    /// assert!(rights.has_right(BusinessBotRight::CanReply));
    /// assert!(!rights.has_right(BusinessBotRight::CanReadMessages));
    /// ```
    #[must_use]
    pub const fn has_right(&self, right: BusinessBotRight) -> bool {
        match right {
            BusinessBotRight::CanReply => self.can_reply,
            BusinessBotRight::CanReadMessages => self.can_read_messages,
            BusinessBotRight::CanDeleteSentMessages => self.can_delete_sent_messages,
            BusinessBotRight::CanDeleteReceivedMessages => self.can_delete_received_messages,
            BusinessBotRight::CanEditName => self.can_edit_name,
            BusinessBotRight::CanEditBio => self.can_edit_bio,
            BusinessBotRight::CanEditProfilePhoto => self.can_edit_profile_photo,
            BusinessBotRight::CanEditUsername => self.can_edit_username,
            BusinessBotRight::CanViewGifts => self.can_view_gifts,
            BusinessBotRight::CanSellGifts => self.can_sell_gifts,
            BusinessBotRight::CanChangeGiftSettings => self.can_change_gift_settings,
            BusinessBotRight::CanTransferAndUpgradeGifts => self.can_transfer_and_upgrade_gifts,
            BusinessBotRight::CanTransferStars => self.can_transfer_stars,
            BusinessBotRight::CanManageStories => self.can_manage_stories,
        }
    }

    /// Sets the specified right.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::{BusinessBotRights, BusinessBotRight};
    ///
    /// let mut rights = BusinessBotRights::new();
    /// rights.set_right(BusinessBotRight::CanReply, true);
    ///
    /// assert!(rights.can_reply());
    /// ```
    pub fn set_right(&mut self, right: BusinessBotRight, value: bool) {
        match right {
            BusinessBotRight::CanReply => self.can_reply = value,
            BusinessBotRight::CanReadMessages => self.can_read_messages = value,
            BusinessBotRight::CanDeleteSentMessages => self.can_delete_sent_messages = value,
            BusinessBotRight::CanDeleteReceivedMessages => {
                self.can_delete_received_messages = value
            }
            BusinessBotRight::CanEditName => self.can_edit_name = value,
            BusinessBotRight::CanEditBio => self.can_edit_bio = value,
            BusinessBotRight::CanEditProfilePhoto => self.can_edit_profile_photo = value,
            BusinessBotRight::CanEditUsername => self.can_edit_username = value,
            BusinessBotRight::CanViewGifts => self.can_view_gifts = value,
            BusinessBotRight::CanSellGifts => self.can_sell_gifts = value,
            BusinessBotRight::CanChangeGiftSettings => self.can_change_gift_settings = value,
            BusinessBotRight::CanTransferAndUpgradeGifts => {
                self.can_transfer_and_upgrade_gifts = value
            }
            BusinessBotRight::CanTransferStars => self.can_transfer_stars = value,
            BusinessBotRight::CanManageStories => self.can_manage_stories = value,
        }
    }

    /// Returns `true` if the bot can reply to messages.
    #[must_use]
    pub const fn can_reply(&self) -> bool {
        self.can_reply
    }

    /// Sets whether the bot can reply to messages.
    pub fn set_can_reply(&mut self, value: bool) {
        self.can_reply = value;
    }

    /// Returns `true` if the bot can read messages.
    #[must_use]
    pub const fn can_read_messages(&self) -> bool {
        self.can_read_messages
    }

    /// Sets whether the bot can read messages.
    pub fn set_can_read_messages(&mut self, value: bool) {
        self.can_read_messages = value;
    }

    /// Returns `true` if the bot can delete sent messages.
    #[must_use]
    pub const fn can_delete_sent_messages(&self) -> bool {
        self.can_delete_sent_messages
    }

    /// Sets whether the bot can delete sent messages.
    pub fn set_can_delete_sent_messages(&mut self, value: bool) {
        self.can_delete_sent_messages = value;
    }

    /// Returns `true` if the bot can delete received messages.
    #[must_use]
    pub const fn can_delete_received_messages(&self) -> bool {
        self.can_delete_received_messages
    }

    /// Sets whether the bot can delete received messages.
    pub fn set_can_delete_received_messages(&mut self, value: bool) {
        self.can_delete_received_messages = value;
    }

    /// Returns `true` if the bot can edit the account name.
    #[must_use]
    pub const fn can_edit_name(&self) -> bool {
        self.can_edit_name
    }

    /// Sets whether the bot can edit the account name.
    pub fn set_can_edit_name(&mut self, value: bool) {
        self.can_edit_name = value;
    }

    /// Returns `true` if the bot can edit the account bio.
    #[must_use]
    pub const fn can_edit_bio(&self) -> bool {
        self.can_edit_bio
    }

    /// Sets whether the bot can edit the account bio.
    pub fn set_can_edit_bio(&mut self, value: bool) {
        self.can_edit_bio = value;
    }

    /// Returns `true` if the bot can edit the profile photo.
    #[must_use]
    pub const fn can_edit_profile_photo(&self) -> bool {
        self.can_edit_profile_photo
    }

    /// Sets whether the bot can edit the profile photo.
    pub fn set_can_edit_profile_photo(&mut self, value: bool) {
        self.can_edit_profile_photo = value;
    }

    /// Returns `true` if the bot can edit the username.
    #[must_use]
    pub const fn can_edit_username(&self) -> bool {
        self.can_edit_username
    }

    /// Sets whether the bot can edit the username.
    pub fn set_can_edit_username(&mut self, value: bool) {
        self.can_edit_username = value;
    }

    /// Returns `true` if the bot can view gifts and stars.
    #[must_use]
    pub const fn can_view_gifts(&self) -> bool {
        self.can_view_gifts
    }

    /// Sets whether the bot can view gifts and stars.
    pub fn set_can_view_gifts(&mut self, value: bool) {
        self.can_view_gifts = value;
    }

    /// Returns `true` if the bot can sell gifts.
    #[must_use]
    pub const fn can_sell_gifts(&self) -> bool {
        self.can_sell_gifts
    }

    /// Sets whether the bot can sell gifts.
    pub fn set_can_sell_gifts(&mut self, value: bool) {
        self.can_sell_gifts = value;
    }

    /// Returns `true` if the bot can change gift settings.
    #[must_use]
    pub const fn can_change_gift_settings(&self) -> bool {
        self.can_change_gift_settings
    }

    /// Sets whether the bot can change gift settings.
    pub fn set_can_change_gift_settings(&mut self, value: bool) {
        self.can_change_gift_settings = value;
    }

    /// Returns `true` if the bot can transfer and upgrade gifts.
    #[must_use]
    pub const fn can_transfer_and_upgrade_gifts(&self) -> bool {
        self.can_transfer_and_upgrade_gifts
    }

    /// Sets whether the bot can transfer and upgrade gifts.
    pub fn set_can_transfer_and_upgrade_gifts(&mut self, value: bool) {
        self.can_transfer_and_upgrade_gifts = value;
    }

    /// Returns `true` if the bot can transfer stars.
    #[must_use]
    pub const fn can_transfer_stars(&self) -> bool {
        self.can_transfer_stars
    }

    /// Sets whether the bot can transfer stars.
    pub fn set_can_transfer_stars(&mut self, value: bool) {
        self.can_transfer_stars = value;
    }

    /// Returns `true` if the bot can manage stories.
    #[must_use]
    pub const fn can_manage_stories(&self) -> bool {
        self.can_manage_stories
    }

    /// Sets whether the bot can manage stories.
    pub fn set_can_manage_stories(&mut self, value: bool) {
        self.can_manage_stories = value;
    }

    /// Returns the number of enabled rights.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::BusinessBotRights;
    ///
    /// let rights = BusinessBotRights::new();
    /// assert_eq!(rights.count_enabled(), 0);
    ///
    /// let all_rights = BusinessBotRights::all();
    /// assert_eq!(all_rights.count_enabled(), 14);
    /// ```
    #[must_use]
    pub const fn count_enabled(&self) -> usize {
        let mut count = 0;
        if self.can_reply {
            count += 1;
        }
        if self.can_read_messages {
            count += 1;
        }
        if self.can_delete_sent_messages {
            count += 1;
        }
        if self.can_delete_received_messages {
            count += 1;
        }
        if self.can_edit_name {
            count += 1;
        }
        if self.can_edit_bio {
            count += 1;
        }
        if self.can_edit_profile_photo {
            count += 1;
        }
        if self.can_edit_username {
            count += 1;
        }
        if self.can_view_gifts {
            count += 1;
        }
        if self.can_sell_gifts {
            count += 1;
        }
        if self.can_change_gift_settings {
            count += 1;
        }
        if self.can_transfer_and_upgrade_gifts {
            count += 1;
        }
        if self.can_transfer_stars {
            count += 1;
        }
        if self.can_manage_stories {
            count += 1;
        }
        count
    }
}

impl fmt::Display for BusinessBotRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BusinessBotRights[")?;
        let mut first = true;
        if self.can_reply {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "reply")?;
            first = false;
        }
        if self.can_read_messages {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "read_messages")?;
            first = false;
        }
        if self.can_delete_sent_messages {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "delete_sent_messages")?;
            first = false;
        }
        if self.can_delete_received_messages {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "delete_received_messages")?;
            first = false;
        }
        if self.can_edit_name {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "edit_name")?;
            first = false;
        }
        if self.can_edit_bio {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "edit_bio")?;
            first = false;
        }
        if self.can_edit_profile_photo {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "edit_profile_photo")?;
            first = false;
        }
        if self.can_edit_username {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "edit_username")?;
            first = false;
        }
        if self.can_view_gifts {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "view_gifts")?;
            first = false;
        }
        if self.can_sell_gifts {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "sell_gifts")?;
            first = false;
        }
        if self.can_change_gift_settings {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "change_gift_settings")?;
            first = false;
        }
        if self.can_transfer_and_upgrade_gifts {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "transfer_and_upgrade_gifts")?;
            first = false;
        }
        if self.can_transfer_stars {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "transfer_stars")?;
            first = false;
        }
        if self.can_manage_stories {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "manage_stories")?;
        }
        write!(f, "]")
    }
}

/// Recipients configuration for business bot messages.
///
/// Defines which users/chats should receive messages from the business bot.
/// Supports inclusion/exclusion lists and various category filters.
///
/// # Examples
///
/// ```
/// use rustgram_business_connected_bot::BusinessRecipients;
/// use rustgram_types::UserId;
///
/// // Create empty recipients
/// let recipients = BusinessRecipients::new();
/// assert!(recipients.is_empty());
///
/// // Create with specific users
/// let mut recipients = BusinessRecipients::new();
/// recipients.add_user(UserId(12345));
/// recipients.add_user(UserId(67890));
/// assert_eq!(recipients.user_ids().len(), 2);
/// ```
///
/// # Correspondence
///
/// Corresponds to TDLib's `td::BusinessRecipients` class.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BusinessRecipients {
    /// Included user IDs
    user_ids: Vec<UserId>,
    /// Excluded user IDs
    excluded_user_ids: Vec<UserId>,
    /// Include existing chats
    existing_chats: bool,
    /// Include new chats
    new_chats: bool,
    /// Include contacts
    contacts: bool,
    /// Include non-contacts
    non_contacts: bool,
    /// Exclude selected users (inverts user_ids selection)
    exclude_selected: bool,
}

impl BusinessRecipients {
    /// Creates a new empty `BusinessRecipients`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::BusinessRecipients;
    ///
    /// let recipients = BusinessRecipients::new();
    /// assert!(recipients.is_empty());
    /// assert!(!recipients.existing_chats());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            user_ids: Vec::new(),
            excluded_user_ids: Vec::new(),
            existing_chats: false,
            new_chats: false,
            contacts: false,
            non_contacts: false,
            exclude_selected: false,
        }
    }

    /// Creates a new `BusinessRecipients` with the specified parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let recipients = BusinessRecipients::with_params(
    ///     vec![UserId(123), UserId(456)],
    ///     vec![UserId(789)],
    ///     true,
    ///     false,
    ///     true,
    ///     false,
    ///     false,
    /// );
    /// ```
    #[must_use]
    pub fn with_params(
        user_ids: Vec<UserId>,
        excluded_user_ids: Vec<UserId>,
        existing_chats: bool,
        new_chats: bool,
        contacts: bool,
        non_contacts: bool,
        exclude_selected: bool,
    ) -> Self {
        Self {
            user_ids,
            excluded_user_ids,
            existing_chats,
            new_chats,
            contacts,
            non_contacts,
            exclude_selected,
        }
    }

    /// Returns `true` if no recipients are configured.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let recipients = BusinessRecipients::new();
    /// assert!(recipients.is_empty());
    ///
    /// let mut recipients = BusinessRecipients::new();
    /// recipients.set_existing_chats(true);
    /// assert!(!recipients.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.user_ids.is_empty()
            && self.excluded_user_ids.is_empty()
            && !self.existing_chats
            && !self.new_chats
            && !self.contacts
            && !self.non_contacts
    }

    /// Checks if the specified user ID is in the recipients list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let mut recipients = BusinessRecipients::new();
    /// recipients.add_user(UserId(12345));
    ///
    /// assert!(recipients.contains_user(UserId(12345)));
    /// assert!(!recipients.contains_user(UserId(67890)));
    /// ```
    #[must_use]
    pub fn contains_user(&self, user_id: UserId) -> bool {
        self.user_ids.contains(&user_id)
    }

    /// Checks if the specified user ID is in the excluded list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let mut recipients = BusinessRecipients::new();
    /// recipients.add_excluded_user(UserId(12345));
    ///
    /// assert!(recipients.contains_excluded_user(UserId(12345)));
    /// assert!(!recipients.contains_excluded_user(UserId(67890)));
    /// ```
    #[must_use]
    pub fn contains_excluded_user(&self, user_id: UserId) -> bool {
        self.excluded_user_ids.contains(&user_id)
    }

    /// Adds a user to the recipients list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let mut recipients = BusinessRecipients::new();
    /// recipients.add_user(UserId(12345));
    /// assert!(recipients.contains_user(UserId(12345)));
    /// ```
    pub fn add_user(&mut self, user_id: UserId) {
        if !self.contains_user(user_id) {
            self.user_ids.push(user_id);
        }
    }

    /// Adds a user to the excluded list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let mut recipients = BusinessRecipients::new();
    /// recipients.add_excluded_user(UserId(12345));
    /// assert!(recipients.contains_excluded_user(UserId(12345)));
    /// ```
    pub fn add_excluded_user(&mut self, user_id: UserId) {
        if !self.contains_excluded_user(user_id) {
            self.excluded_user_ids.push(user_id);
        }
    }

    /// Removes a user from the recipients list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let mut recipients = BusinessRecipients::new();
    /// recipients.add_user(UserId(12345));
    /// recipients.remove_user(UserId(12345));
    /// assert!(!recipients.contains_user(UserId(12345)));
    /// ```
    pub fn remove_user(&mut self, user_id: UserId) {
        self.user_ids.retain(|&id| id != user_id);
    }

    /// Removes a user from the excluded list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let mut recipients = BusinessRecipients::new();
    /// recipients.add_excluded_user(UserId(12345));
    /// recipients.remove_excluded_user(UserId(12345));
    /// assert!(!recipients.contains_excluded_user(UserId(12345)));
    /// ```
    pub fn remove_excluded_user(&mut self, user_id: UserId) {
        self.excluded_user_ids.retain(|&id| id != user_id);
    }

    /// Clears all user IDs from the recipients list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let mut recipients = BusinessRecipients::new();
    /// recipients.add_user(UserId(12345));
    /// recipients.clear_users();
    /// assert!(recipients.user_ids().is_empty());
    /// ```
    pub fn clear_users(&mut self) {
        self.user_ids.clear();
    }

    /// Clears all user IDs from the excluded list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let mut recipients = BusinessRecipients::new();
    /// recipients.add_excluded_user(UserId(12345));
    /// recipients.clear_excluded_users();
    /// assert!(recipients.excluded_user_ids().is_empty());
    /// ```
    pub fn clear_excluded_users(&mut self) {
        self.excluded_user_ids.clear();
    }

    /// Returns a reference to the user IDs list.
    #[must_use]
    pub const fn user_ids(&self) -> &Vec<UserId> {
        &self.user_ids
    }

    /// Returns a reference to the excluded user IDs list.
    #[must_use]
    pub const fn excluded_user_ids(&self) -> &Vec<UserId> {
        &self.excluded_user_ids
    }

    /// Returns `true` if existing chats are included.
    #[must_use]
    pub const fn existing_chats(&self) -> bool {
        self.existing_chats
    }

    /// Sets whether existing chats are included.
    pub fn set_existing_chats(&mut self, value: bool) {
        self.existing_chats = value;
    }

    /// Returns `true` if new chats are included.
    #[must_use]
    pub const fn new_chats(&self) -> bool {
        self.new_chats
    }

    /// Sets whether new chats are included.
    pub fn set_new_chats(&mut self, value: bool) {
        self.new_chats = value;
    }

    /// Returns `true` if contacts are included.
    #[must_use]
    pub const fn contacts(&self) -> bool {
        self.contacts
    }

    /// Sets whether contacts are included.
    pub fn set_contacts(&mut self, value: bool) {
        self.contacts = value;
    }

    /// Returns `true` if non-contacts are included.
    #[must_use]
    pub const fn non_contacts(&self) -> bool {
        self.non_contacts
    }

    /// Sets whether non-contacts are included.
    pub fn set_non_contacts(&mut self, value: bool) {
        self.non_contacts = value;
    }

    /// Returns `true` if selected users should be excluded.
    #[must_use]
    pub const fn exclude_selected(&self) -> bool {
        self.exclude_selected
    }

    /// Sets whether selected users should be excluded.
    pub fn set_exclude_selected(&mut self, value: bool) {
        self.exclude_selected = value;
    }
}

impl fmt::Display for BusinessRecipients {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BusinessRecipients[")?;
        let mut first = true;

        if self.exclude_selected {
            write!(f, "except ")?;
            first = false;
        }

        if !self.user_ids.is_empty() {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "users: {:?}", self.user_ids)?;
            first = false;
        }

        if !self.excluded_user_ids.is_empty() {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "excluded: {:?}", self.excluded_user_ids)?;
            first = false;
        }

        if self.existing_chats {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "existing_chats")?;
            first = false;
        }

        if self.new_chats {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "new_chats")?;
            first = false;
        }

        if self.contacts {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "contacts")?;
            first = false;
        }

        if self.non_contacts {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "non_contacts")?;
        }

        write!(f, "]")
    }
}

/// A bot connected to a business account.
///
/// Represents a bot that has been connected to a Telegram business account
/// with specific permissions and recipient configuration.
///
/// # Examples
///
/// ```
/// use rustgram_business_connected_bot::{BusinessConnectedBot, BusinessBotRights, BusinessRecipients};
/// use rustgram_types::UserId;
///
/// let recipients = BusinessRecipients::new();
/// let rights = BusinessBotRights::new();
///
/// let bot = BusinessConnectedBot::new(
///     UserId(123456789),
///     recipients,
///     rights,
/// );
///
/// assert!(bot.is_valid());
/// assert_eq!(bot.user_id(), UserId(123456789));
/// ```
///
/// # Correspondence
///
/// Corresponds to TDLib's `td::BusinessConnectedBot` class.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BusinessConnectedBot {
    /// Bot user ID
    user_id: UserId,
    /// Message recipients
    recipients: BusinessRecipients,
    /// Bot permissions
    rights: BusinessBotRights,
}

impl BusinessConnectedBot {
    /// Creates a new `BusinessConnectedBot`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::{BusinessConnectedBot, BusinessBotRights, BusinessRecipients};
    /// use rustgram_types::UserId;
    ///
    /// let bot = BusinessConnectedBot::new(
    ///     UserId(123456789),
    ///     BusinessRecipients::new(),
    ///     BusinessBotRights::new(),
    /// );
    /// ```
    #[must_use]
    pub fn new(user_id: UserId, recipients: BusinessRecipients, rights: BusinessBotRights) -> Self {
        Self {
            user_id,
            recipients,
            rights,
        }
    }

    /// Checks if the connected bot is valid.
    ///
    /// A bot is considered valid if it has a valid user ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::{BusinessConnectedBot, BusinessBotRights, BusinessRecipients};
    /// use rustgram_types::UserId;
    ///
    /// let bot = BusinessConnectedBot::new(
    ///     UserId(123456789),
    ///     BusinessRecipients::new(),
    ///     BusinessBotRights::new(),
    /// );
    ///
    /// assert!(bot.is_valid());
    ///
    /// let invalid_bot = BusinessConnectedBot::new(
    ///     UserId(0),
    ///     BusinessRecipients::new(),
    ///     BusinessBotRights::new(),
    /// );
    ///
    /// assert!(!invalid_bot.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.user_id.is_valid()
    }

    /// Returns the bot's user ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::{BusinessConnectedBot, BusinessBotRights, BusinessRecipients};
    /// use rustgram_types::UserId;
    ///
    /// let bot = BusinessConnectedBot::new(
    ///     UserId(123456789),
    ///     BusinessRecipients::new(),
    ///     BusinessBotRights::new(),
    /// );
    ///
    /// assert_eq!(bot.user_id(), UserId(123456789));
    /// ```
    #[must_use]
    pub const fn user_id(&self) -> UserId {
        self.user_id
    }

    /// Returns a reference to the recipients configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::{BusinessConnectedBot, BusinessBotRights, BusinessRecipients};
    /// use rustgram_types::UserId;
    ///
    /// let recipients = BusinessRecipients::new();
    /// let bot = BusinessConnectedBot::new(
    ///     UserId(123456789),
    ///     recipients.clone(),
    ///     BusinessBotRights::new(),
    /// );
    ///
    /// assert!(bot.recipients().is_empty());
    /// ```
    #[must_use]
    pub const fn recipients(&self) -> &BusinessRecipients {
        &self.recipients
    }

    /// Returns a reference to the bot's rights.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_connected_bot::{BusinessConnectedBot, BusinessBotRights, BusinessRecipients};
    /// use rustgram_types::UserId;
    ///
    /// let rights = BusinessBotRights::all();
    /// let bot = BusinessConnectedBot::new(
    ///     UserId(123456789),
    ///     BusinessRecipients::new(),
    ///     rights,
    /// );
    ///
    /// assert!(bot.rights().can_reply());
    /// ```
    #[must_use]
    pub const fn rights(&self) -> &BusinessBotRights {
        &self.rights
    }

    /// Sets the bot's user ID.
    pub fn set_user_id(&mut self, user_id: UserId) {
        self.user_id = user_id;
    }

    /// Sets the recipients configuration.
    pub fn set_recipients(&mut self, recipients: BusinessRecipients) {
        self.recipients = recipients;
    }

    /// Sets the bot's rights.
    pub fn set_rights(&mut self, rights: BusinessBotRights) {
        self.rights = rights;
    }
}

impl fmt::Display for BusinessConnectedBot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BusinessConnectedBot[user_id={}, recipients={}, rights={}]",
            self.user_id, self.recipients, self.rights
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BusinessBotRight tests
    #[test]
    fn test_business_bot_right_display() {
        assert_eq!(format!("{}", BusinessBotRight::CanReply), "can_reply");
        assert_eq!(
            format!("{}", BusinessBotRight::CanReadMessages),
            "can_read_messages"
        );
    }

    // BusinessBotRights tests
    #[test]
    fn test_business_bot_rights_new() {
        let rights = BusinessBotRights::new();
        assert!(!rights.can_reply());
        assert!(!rights.can_read_messages());
        assert!(!rights.can_edit_name());
        assert_eq!(rights.count_enabled(), 0);
    }

    #[test]
    fn test_business_bot_rights_all() {
        let rights = BusinessBotRights::all();
        assert!(rights.can_reply());
        assert!(rights.can_read_messages());
        assert!(rights.can_edit_name());
        assert_eq!(rights.count_enabled(), 14);
    }

    #[test]
    fn test_business_bot_rights_setters() {
        let mut rights = BusinessBotRights::new();
        rights.set_can_reply(true);
        rights.set_can_read_messages(true);
        rights.set_can_edit_name(true);

        assert!(rights.can_reply());
        assert!(rights.can_read_messages());
        assert!(rights.can_edit_name());
        assert_eq!(rights.count_enabled(), 3);
    }

    #[test]
    fn test_business_bot_rights_has_right() {
        let mut rights = BusinessBotRights::new();
        rights.set_can_reply(true);

        assert!(rights.has_right(BusinessBotRight::CanReply));
        assert!(!rights.has_right(BusinessBotRight::CanReadMessages));
    }

    #[test]
    fn test_business_bot_rights_set_right() {
        let mut rights = BusinessBotRights::new();
        rights.set_right(BusinessBotRight::CanReply, true);
        rights.set_right(BusinessBotRight::CanReadMessages, true);

        assert!(rights.can_reply());
        assert!(rights.can_read_messages());
        assert_eq!(rights.count_enabled(), 2);
    }

    #[test]
    fn test_business_bot_rights_equality() {
        let rights1 = BusinessBotRights::new();
        let rights2 = BusinessBotRights::new();
        assert_eq!(rights1, rights2);

        let mut rights3 = BusinessBotRights::new();
        rights3.set_can_reply(true);
        assert_ne!(rights1, rights3);
    }

    #[test]
    fn test_business_bot_rights_display() {
        let rights = BusinessBotRights::new();
        let display = format!("{}", rights);
        assert!(display.contains("BusinessBotRights"));
        assert!(display.contains("]"));

        let mut rights = BusinessBotRights::new();
        rights.set_can_reply(true);
        let display = format!("{}", rights);
        assert!(display.contains("reply"));
    }

    // BusinessRecipients tests
    #[test]
    fn test_business_recipients_new() {
        let recipients = BusinessRecipients::new();
        assert!(recipients.is_empty());
        assert!(recipients.user_ids().is_empty());
        assert!(recipients.excluded_user_ids().is_empty());
        assert!(!recipients.existing_chats());
        assert!(!recipients.new_chats());
        assert!(!recipients.contacts());
        assert!(!recipients.non_contacts());
        assert!(!recipients.exclude_selected());
    }

    #[test]
    fn test_business_recipients_with_params() {
        let recipients = BusinessRecipients::with_params(
            vec![UserId(123), UserId(456)],
            vec![UserId(789)],
            true,
            false,
            true,
            false,
            false,
        );

        assert_eq!(recipients.user_ids().len(), 2);
        assert_eq!(recipients.excluded_user_ids().len(), 1);
        assert!(recipients.existing_chats());
        assert!(!recipients.new_chats());
        assert!(recipients.contacts());
        assert!(!recipients.non_contacts());
        assert!(!recipients.exclude_selected());
    }

    #[test]
    fn test_business_recipients_add_user() {
        let mut recipients = BusinessRecipients::new();
        recipients.add_user(UserId(123));
        recipients.add_user(UserId(456));

        assert_eq!(recipients.user_ids().len(), 2);
        assert!(recipients.contains_user(UserId(123)));
        assert!(recipients.contains_user(UserId(456)));

        // Adding duplicate should not increase count
        recipients.add_user(UserId(123));
        assert_eq!(recipients.user_ids().len(), 2);
    }

    #[test]
    fn test_business_recipients_add_excluded_user() {
        let mut recipients = BusinessRecipients::new();
        recipients.add_excluded_user(UserId(123));

        assert_eq!(recipients.excluded_user_ids().len(), 1);
        assert!(recipients.contains_excluded_user(UserId(123)));

        // Adding duplicate should not increase count
        recipients.add_excluded_user(UserId(123));
        assert_eq!(recipients.excluded_user_ids().len(), 1);
    }

    #[test]
    fn test_business_recipients_remove_user() {
        let mut recipients = BusinessRecipients::new();
        recipients.add_user(UserId(123));
        recipients.add_user(UserId(456));

        recipients.remove_user(UserId(123));
        assert!(!recipients.contains_user(UserId(123)));
        assert!(recipients.contains_user(UserId(456)));
        assert_eq!(recipients.user_ids().len(), 1);
    }

    #[test]
    fn test_business_recipients_remove_excluded_user() {
        let mut recipients = BusinessRecipients::new();
        recipients.add_excluded_user(UserId(123));
        recipients.add_excluded_user(UserId(456));

        recipients.remove_excluded_user(UserId(123));
        assert!(!recipients.contains_excluded_user(UserId(123)));
        assert!(recipients.contains_excluded_user(UserId(456)));
        assert_eq!(recipients.excluded_user_ids().len(), 1);
    }

    #[test]
    fn test_business_recipients_clear_users() {
        let mut recipients = BusinessRecipients::new();
        recipients.add_user(UserId(123));
        recipients.add_user(UserId(456));

        recipients.clear_users();
        assert!(recipients.user_ids().is_empty());
        // After clearing users, recipients is empty again (no other settings)
        assert!(recipients.is_empty());
    }

    #[test]
    fn test_business_recipients_clear_excluded_users() {
        let mut recipients = BusinessRecipients::new();
        recipients.add_excluded_user(UserId(123));
        recipients.add_excluded_user(UserId(456));

        recipients.clear_excluded_users();
        assert!(recipients.excluded_user_ids().is_empty());
    }

    #[test]
    fn test_business_recipients_setters() {
        let mut recipients = BusinessRecipients::new();
        recipients.set_existing_chats(true);
        recipients.set_new_chats(true);
        recipients.set_contacts(true);
        recipients.set_non_contacts(true);
        recipients.set_exclude_selected(true);

        assert!(recipients.existing_chats());
        assert!(recipients.new_chats());
        assert!(recipients.contacts());
        assert!(recipients.non_contacts());
        assert!(recipients.exclude_selected());
        assert!(!recipients.is_empty());
    }

    #[test]
    fn test_business_recipients_is_empty() {
        let recipients = BusinessRecipients::new();
        assert!(recipients.is_empty());

        let mut recipients = BusinessRecipients::new();
        recipients.add_user(UserId(123));
        assert!(!recipients.is_empty());

        let mut recipients = BusinessRecipients::new();
        recipients.set_existing_chats(true);
        assert!(!recipients.is_empty());
    }

    #[test]
    fn test_business_recipients_equality() {
        let recipients1 = BusinessRecipients::new();
        let recipients2 = BusinessRecipients::new();
        assert_eq!(recipients1, recipients2);

        let mut recipients3 = BusinessRecipients::new();
        recipients3.add_user(UserId(123));
        assert_ne!(recipients1, recipients3);
    }

    #[test]
    fn test_business_recipients_display() {
        let recipients = BusinessRecipients::new();
        let display = format!("{}", recipients);
        assert!(display.contains("BusinessRecipients"));

        let mut recipients = BusinessRecipients::new();
        recipients.set_existing_chats(true);
        let display = format!("{}", recipients);
        assert!(display.contains("existing_chats"));
    }

    // BusinessConnectedBot tests
    #[test]
    fn test_business_connected_bot_new() {
        let bot = BusinessConnectedBot::new(
            UserId(123456789),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );

        assert_eq!(bot.user_id(), UserId(123456789));
        assert!(bot.recipients().is_empty());
        assert!(!bot.rights().can_reply());
    }

    #[test]
    fn test_business_connected_bot_is_valid() {
        let bot = BusinessConnectedBot::new(
            UserId(123456789),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );
        assert!(bot.is_valid());

        let invalid_bot = BusinessConnectedBot::new(
            UserId(0),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );
        assert!(!invalid_bot.is_valid());
    }

    #[test]
    fn test_business_connected_bot_setters() {
        let mut bot = BusinessConnectedBot::new(
            UserId(123456789),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );

        bot.set_user_id(UserId(987654321));
        assert_eq!(bot.user_id(), UserId(987654321));

        let mut recipients = BusinessRecipients::new();
        recipients.add_user(UserId(111));
        bot.set_recipients(recipients);
        assert_eq!(bot.recipients().user_ids().len(), 1);

        let mut rights = BusinessBotRights::new();
        rights.set_can_reply(true);
        bot.set_rights(rights);
        assert!(bot.rights().can_reply());
    }

    #[test]
    fn test_business_connected_bot_equality() {
        let bot1 = BusinessConnectedBot::new(
            UserId(123456789),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );

        let bot2 = BusinessConnectedBot::new(
            UserId(123456789),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );

        assert_eq!(bot1, bot2);

        let bot3 = BusinessConnectedBot::new(
            UserId(987654321),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );

        assert_ne!(bot1, bot3);
    }

    #[test]
    fn test_business_connected_bot_display() {
        let bot = BusinessConnectedBot::new(
            UserId(123456789),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );

        let display = format!("{}", bot);
        assert!(display.contains("BusinessConnectedBot"));
        assert!(display.contains("user 123456789"));
    }

    #[test]
    fn test_business_connected_bot_clone() {
        let bot = BusinessConnectedBot::new(
            UserId(123456789),
            BusinessRecipients::new(),
            BusinessBotRights::new(),
        );

        let bot_clone = bot.clone();
        assert_eq!(bot, bot_clone);
    }
}
