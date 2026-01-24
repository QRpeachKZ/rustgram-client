//! # Rustgram BusinessBotRights
//!
//! Business bot rights handling for Telegram MTProto client.
//!
//! This crate provides types for managing business bot permissions and rights.
//!
//! ## Overview
//!
//! - [`BusinessBotRights`] - Bot permissions for business account management
//!
//! ## Examples
//!
//! ```
//! use rustgram_business_bot_rights::BusinessBotRights;
//!
//! let rights = BusinessBotRights::new();
//! assert!(!rights.can_reply());
//!
//! let rights = BusinessBotRights::with_reply_only(true);
//! assert!(rights.can_reply());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::fmt;

/// Business bot rights configuration.
///
/// Defines the permissions a business bot has on a business account.
///
/// # Examples
///
/// ```
/// use rustgram_business_bot_rights::BusinessBotRights;
///
/// let rights = BusinessBotRights::new();
/// assert!(!rights.can_reply());
///
/// let rights = BusinessBotRights::legacy(true);
/// assert!(rights.can_reply());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BusinessBotRights {
    /// Can reply to messages
    can_reply: bool,
    /// Can read messages
    can_read_messages: bool,
    /// Can delete sent messages
    can_delete_sent_messages: bool,
    /// Can delete received messages
    can_delete_received_messages: bool,
    /// Can edit the account name
    can_edit_name: bool,
    /// Can edit the account bio
    can_edit_bio: bool,
    /// Can edit the account profile photo
    can_edit_profile_photo: bool,
    /// Can edit the account username
    can_edit_username: bool,
    /// Can view gifts
    can_view_gifts: bool,
    /// Can sell gifts
    can_sell_gifts: bool,
    /// Can change gift settings
    can_change_gift_settings: bool,
    /// Can transfer and upgrade gifts
    can_transfer_and_upgrade_gifts: bool,
    /// Can transfer stars
    can_transfer_stars: bool,
    /// Can manage stories
    can_manage_stories: bool,
}

impl Default for BusinessBotRights {
    fn default() -> Self {
        Self::new()
    }
}

impl BusinessBotRights {
    /// Creates a new rights configuration with all permissions disabled.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_rights::BusinessBotRights;
    ///
    /// let rights = BusinessBotRights::new();
    /// assert!(!rights.can_reply());
    /// assert!(!rights.can_read_messages());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
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

    /// Creates a legacy rights configuration with only reply permission.
    ///
    /// # Arguments
    ///
    /// * `can_reply` - Whether the bot can reply to messages
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_rights::BusinessBotRights;
    ///
    /// let rights = BusinessBotRights::legacy(true);
    /// assert!(rights.can_reply());
    /// assert!(!rights.can_read_messages());
    /// ```
    #[inline]
    #[must_use]
    pub const fn legacy(can_reply: bool) -> Self {
        Self {
            can_reply,
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

    /// Creates a rights configuration with reply only permission.
    ///
    /// # Arguments
    ///
    /// * `can_reply` - Whether the bot can reply to messages
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_rights::BusinessBotRights;
    ///
    /// let rights = BusinessBotRights::with_reply_only(true);
    /// assert!(rights.can_reply());
    /// ```
    #[inline]
    #[must_use]
    pub const fn with_reply_only(can_reply: bool) -> Self {
        Self::legacy(can_reply)
    }

    /// Returns whether the bot can reply to messages.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_rights::BusinessBotRights;
    ///
    /// let rights = BusinessBotRights::legacy(true);
    /// assert!(rights.can_reply());
    /// ```
    #[inline]
    #[must_use]
    pub const fn can_reply(&self) -> bool {
        self.can_reply
    }

    /// Returns whether the bot can read messages.
    #[inline]
    #[must_use]
    pub const fn can_read_messages(&self) -> bool {
        self.can_read_messages
    }

    /// Returns whether the bot can delete sent messages.
    #[inline]
    #[must_use]
    pub const fn can_delete_sent_messages(&self) -> bool {
        self.can_delete_sent_messages
    }

    /// Returns whether the bot can delete received messages.
    #[inline]
    #[must_use]
    pub const fn can_delete_received_messages(&self) -> bool {
        self.can_delete_received_messages
    }

    /// Returns whether the bot can edit the account name.
    #[inline]
    #[must_use]
    pub const fn can_edit_name(&self) -> bool {
        self.can_edit_name
    }

    /// Returns whether the bot can edit the account bio.
    #[inline]
    #[must_use]
    pub const fn can_edit_bio(&self) -> bool {
        self.can_edit_bio
    }

    /// Returns whether the bot can edit the account profile photo.
    #[inline]
    #[must_use]
    pub const fn can_edit_profile_photo(&self) -> bool {
        self.can_edit_profile_photo
    }

    /// Returns whether the bot can edit the account username.
    #[inline]
    #[must_use]
    pub const fn can_edit_username(&self) -> bool {
        self.can_edit_username
    }

    /// Returns whether the bot can view gifts.
    #[inline]
    #[must_use]
    pub const fn can_view_gifts(&self) -> bool {
        self.can_view_gifts
    }

    /// Returns whether the bot can sell gifts.
    #[inline]
    #[must_use]
    pub const fn can_sell_gifts(&self) -> bool {
        self.can_sell_gifts
    }

    /// Returns whether the bot can change gift settings.
    #[inline]
    #[must_use]
    pub const fn can_change_gift_settings(&self) -> bool {
        self.can_change_gift_settings
    }

    /// Returns whether the bot can transfer and upgrade gifts.
    #[inline]
    #[must_use]
    pub const fn can_transfer_and_upgrade_gifts(&self) -> bool {
        self.can_transfer_and_upgrade_gifts
    }

    /// Returns whether the bot can transfer stars.
    #[inline]
    #[must_use]
    pub const fn can_transfer_stars(&self) -> bool {
        self.can_transfer_stars
    }

    /// Returns whether the bot can manage stories.
    #[inline]
    #[must_use]
    pub const fn can_manage_stories(&self) -> bool {
        self.can_manage_stories
    }

    /// Sets the reply permission.
    pub fn set_can_reply(&mut self, value: bool) {
        self.can_reply = value;
    }

    /// Sets the read messages permission.
    pub fn set_can_read_messages(&mut self, value: bool) {
        self.can_read_messages = value;
    }

    /// Sets the delete sent messages permission.
    pub fn set_can_delete_sent_messages(&mut self, value: bool) {
        self.can_delete_sent_messages = value;
    }

    /// Sets the delete received messages permission.
    pub fn set_can_delete_received_messages(&mut self, value: bool) {
        self.can_delete_received_messages = value;
    }

    /// Sets the edit name permission.
    pub fn set_can_edit_name(&mut self, value: bool) {
        self.can_edit_name = value;
    }

    /// Sets the edit bio permission.
    pub fn set_can_edit_bio(&mut self, value: bool) {
        self.can_edit_bio = value;
    }

    /// Sets the edit profile photo permission.
    pub fn set_can_edit_profile_photo(&mut self, value: bool) {
        self.can_edit_profile_photo = value;
    }

    /// Sets the edit username permission.
    pub fn set_can_edit_username(&mut self, value: bool) {
        self.can_edit_username = value;
    }

    /// Sets the view gifts permission.
    pub fn set_can_view_gifts(&mut self, value: bool) {
        self.can_view_gifts = value;
    }

    /// Sets the sell gifts permission.
    pub fn set_can_sell_gifts(&mut self, value: bool) {
        self.can_sell_gifts = value;
    }

    /// Sets the change gift settings permission.
    pub fn set_can_change_gift_settings(&mut self, value: bool) {
        self.can_change_gift_settings = value;
    }

    /// Sets the transfer and upgrade gifts permission.
    pub fn set_can_transfer_and_upgrade_gifts(&mut self, value: bool) {
        self.can_transfer_and_upgrade_gifts = value;
    }

    /// Sets the transfer stars permission.
    pub fn set_can_transfer_stars(&mut self, value: bool) {
        self.can_transfer_stars = value;
    }

    /// Sets the manage stories permission.
    pub fn set_can_manage_stories(&mut self, value: bool) {
        self.can_manage_stories = value;
    }

    /// Checks if any permissions are granted.
    ///
    /// # Returns
    ///
    /// `true` if at least one permission is enabled
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_rights::BusinessBotRights;
    ///
    /// let rights = BusinessBotRights::new();
    /// assert!(!rights.has_any_permission());
    ///
    /// let rights = BusinessBotRights::legacy(true);
    /// assert!(rights.has_any_permission());
    /// ```
    #[must_use]
    pub fn has_any_permission(&self) -> bool {
        self.can_reply
            || self.can_read_messages
            || self.can_delete_sent_messages
            || self.can_delete_received_messages
            || self.can_edit_name
            || self.can_edit_bio
            || self.can_edit_profile_photo
            || self.can_edit_username
            || self.can_view_gifts
            || self.can_sell_gifts
            || self.can_change_gift_settings
            || self.can_transfer_and_upgrade_gifts
            || self.can_transfer_stars
            || self.can_manage_stories
    }
}

impl fmt::Display for BusinessBotRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BusinessBotRights[")?;
        let mut first = true;
        if self.can_reply {
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
            write!(f, "delete_sent")?;
            first = false;
        }
        if self.can_delete_received_messages {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "delete_received")?;
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
            write!(f, "edit_photo")?;
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
            write!(f, "gift_settings")?;
            first = false;
        }
        if self.can_transfer_and_upgrade_gifts {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "transfer_gifts")?;
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

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-business-bot-rights";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Constructor Tests ==========

    #[test]
    fn test_new_creates_empty() {
        let rights = BusinessBotRights::new();
        assert!(!rights.can_reply());
        assert!(!rights.can_read_messages());
        assert!(!rights.has_any_permission());
    }

    #[test]
    fn test_default_creates_empty() {
        let rights = BusinessBotRights::default();
        assert!(!rights.has_any_permission());
    }

    #[test]
    fn test_legacy_true() {
        let rights = BusinessBotRights::legacy(true);
        assert!(rights.can_reply());
        assert!(!rights.can_read_messages());
    }

    #[test]
    fn test_legacy_false() {
        let rights = BusinessBotRights::legacy(false);
        assert!(!rights.can_reply());
    }

    #[test]
    fn test_with_reply_only() {
        let rights = BusinessBotRights::with_reply_only(true);
        assert!(rights.can_reply());
        assert!(!rights.can_read_messages());
    }

    // ========== Accessor Tests ==========

    #[test]
    fn test_can_reply() {
        let rights = BusinessBotRights::legacy(true);
        assert!(rights.can_reply());
    }

    #[test]
    fn test_all_accessors_false_by_default() {
        let rights = BusinessBotRights::new();
        assert!(!rights.can_reply());
        assert!(!rights.can_read_messages());
        assert!(!rights.can_delete_sent_messages());
        assert!(!rights.can_delete_received_messages());
        assert!(!rights.can_edit_name());
        assert!(!rights.can_edit_bio());
        assert!(!rights.can_edit_profile_photo());
        assert!(!rights.can_edit_username());
        assert!(!rights.can_view_gifts());
        assert!(!rights.can_sell_gifts());
        assert!(!rights.can_change_gift_settings());
        assert!(!rights.can_transfer_and_upgrade_gifts());
        assert!(!rights.can_transfer_stars());
        assert!(!rights.can_manage_stories());
    }

    // ========== Mutator Tests ==========

    #[test]
    fn test_set_can_reply() {
        let mut rights = BusinessBotRights::new();
        assert!(!rights.can_reply());

        rights.set_can_reply(true);
        assert!(rights.can_reply());

        rights.set_can_reply(false);
        assert!(!rights.can_reply());
    }

    #[test]
    fn test_set_can_read_messages() {
        let mut rights = BusinessBotRights::new();
        rights.set_can_read_messages(true);
        assert!(rights.can_read_messages());
    }

    #[test]
    fn test_set_multiple_permissions() {
        let mut rights = BusinessBotRights::new();
        rights.set_can_reply(true);
        rights.set_can_read_messages(true);
        rights.set_can_edit_name(true);

        assert!(rights.can_reply());
        assert!(rights.can_read_messages());
        assert!(rights.can_edit_name());
    }

    // ========== has_any_permission Tests ==========

    #[test]
    fn test_has_any_permission_when_empty() {
        let rights = BusinessBotRights::new();
        assert!(!rights.has_any_permission());
    }

    #[test]
    fn test_has_any_permission_with_reply() {
        let rights = BusinessBotRights::legacy(true);
        assert!(rights.has_any_permission());
    }

    #[test]
    fn test_has_any_permission_with_read_messages() {
        let mut rights = BusinessBotRights::new();
        rights.set_can_read_messages(true);
        assert!(rights.has_any_permission());
    }

    // ========== Equality Tests ==========

    #[test]
    fn test_equality_same_permissions() {
        let rights1 = BusinessBotRights::legacy(true);
        let rights2 = BusinessBotRights::legacy(true);
        assert_eq!(rights1, rights2);
    }

    #[test]
    fn test_inequality_different_reply() {
        let rights1 = BusinessBotRights::legacy(true);
        let rights2 = BusinessBotRights::legacy(false);
        assert_ne!(rights1, rights2);
    }

    #[test]
    fn test_inequality_different_permissions() {
        let mut rights1 = BusinessBotRights::new();
        rights1.set_can_read_messages(true);

        let mut rights2 = BusinessBotRights::new();
        rights2.set_can_edit_name(true);

        assert_ne!(rights1, rights2);
    }

    // ========== Clone Tests ==========

    #[test]
    fn test_clone() {
        let mut rights1 = BusinessBotRights::new();
        rights1.set_can_reply(true);
        rights1.set_can_read_messages(true);

        let rights2 = rights1.clone();
        assert_eq!(rights1, rights2);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_empty() {
        let rights = BusinessBotRights::new();
        let s = format!("{}", rights);
        assert!(s.contains("BusinessBotRights"));
    }

    #[test]
    fn test_display_with_reply() {
        let rights = BusinessBotRights::legacy(true);
        let s = format!("{}", rights);
        assert!(s.contains("reply"));
    }

    #[test]
    fn test_display_with_multiple() {
        let mut rights = BusinessBotRights::new();
        rights.set_can_reply(true);
        rights.set_can_read_messages(true);

        let s = format!("{}", rights);
        assert!(s.contains("reply"));
        assert!(s.contains("read_messages"));
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-business-bot-rights");
    }
}
