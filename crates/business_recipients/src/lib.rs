//! # Business Recipients
//!
//! Represents recipients for business messages in Telegram.
//!
//! ## Overview
//!
//! This module defines the `BusinessRecipients` struct, which specifies
//! which users should receive business messages based on various criteria.
//!
//! ## TDLib Correspondence
//!
//! TDLib class: `BusinessRecipients`
//!
//! ## Examples
//!
//! ```
//! use rustgram_business_recipients::BusinessRecipients;
//! use rustgram_types::UserId;
//!
//! // Create business recipients with specific users
//! let user1 = UserId::from_i32(123456);
//! let user2 = UserId::from_i32(789012);
//! let recipients = BusinessRecipients::with_users(vec![user1, user2]);
//!
//! // Add criteria
//! let recipients = recipients.with_existing_chats(true).with_contacts(true);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use core::fmt;
use rustgram_types::UserId;

/// Represents recipients for business messages in Telegram.
///
/// BusinessRecipients allows specifying which users should receive business
/// messages through a combination of explicit user lists and criteria flags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BusinessRecipients {
    /// List of user IDs to include.
    user_ids: Vec<UserId>,
    /// List of user IDs to exclude.
    excluded_user_ids: Vec<UserId>,
    /// Include existing chats.
    existing_chats: bool,
    /// Include new chats.
    new_chats: bool,
    /// Include contacts.
    contacts: bool,
    /// Include non-contacts.
    non_contacts: bool,
    /// Exclude selected users instead of including them.
    exclude_selected: bool,
}

impl Default for BusinessRecipients {
    fn default() -> Self {
        Self::new()
    }
}

impl BusinessRecipients {
    /// Creates an empty BusinessRecipients.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    ///
    /// let recipients = BusinessRecipients::new();
    /// assert!(recipients.is_empty());
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

    /// Creates BusinessRecipients with a list of user IDs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let user1 = UserId::from_i32(123456);
    /// let user2 = UserId::from_i32(789012);
    /// let recipients = BusinessRecipients::with_users(vec![user1, user2]);
    /// ```
    #[must_use]
    pub fn with_users(user_ids: Vec<UserId>) -> Self {
        Self {
            user_ids,
            ..Default::default()
        }
    }

    /// Sets the list of user IDs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let users = vec![UserId::from_i32(123456)];
    /// let recipients = BusinessRecipients::new().set_user_ids(users);
    /// ```
    #[must_use]
    pub fn set_user_ids(mut self, user_ids: Vec<UserId>) -> Self {
        self.user_ids = user_ids;
        self
    }

    /// Sets the list of excluded user IDs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let excluded = vec![UserId::from_i32(123456)];
    /// let recipients = BusinessRecipients::new().set_excluded_user_ids(excluded);
    /// ```
    #[must_use]
    pub fn set_excluded_user_ids(mut self, excluded_user_ids: Vec<UserId>) -> Self {
        self.excluded_user_ids = excluded_user_ids;
        self
    }

    /// Sets whether to include existing chats.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    ///
    /// let recipients = BusinessRecipients::new().with_existing_chats(true);
    /// ```
    #[must_use]
    pub const fn with_existing_chats(mut self, existing_chats: bool) -> Self {
        self.existing_chats = existing_chats;
        self
    }

    /// Sets whether to include new chats.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    ///
    /// let recipients = BusinessRecipients::new().with_new_chats(true);
    /// ```
    #[must_use]
    pub const fn with_new_chats(mut self, new_chats: bool) -> Self {
        self.new_chats = new_chats;
        self
    }

    /// Sets whether to include contacts.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    ///
    /// let recipients = BusinessRecipients::new().with_contacts(true);
    /// ```
    #[must_use]
    pub const fn with_contacts(mut self, contacts: bool) -> Self {
        self.contacts = contacts;
        self
    }

    /// Sets whether to include non-contacts.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    ///
    /// let recipients = BusinessRecipients::new().with_non_contacts(true);
    /// ```
    #[must_use]
    pub const fn with_non_contacts(mut self, non_contacts: bool) -> Self {
        self.non_contacts = non_contacts;
        self
    }

    /// Sets whether to exclude selected users.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    ///
    /// let recipients = BusinessRecipients::new().with_exclude_selected(true);
    /// ```
    #[must_use]
    pub const fn with_exclude_selected(mut self, exclude_selected: bool) -> Self {
        self.exclude_selected = exclude_selected;
        self
    }

    /// Returns the list of user IDs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let users = vec![UserId::from_i32(123456)];
    /// let recipients = BusinessRecipients::new().set_user_ids(users.clone());
    /// assert_eq!(recipients.user_ids(), &users);
    /// ```
    #[must_use]
    pub const fn user_ids(&self) -> &Vec<UserId> {
        &self.user_ids
    }

    /// Returns the list of excluded user IDs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let excluded = vec![UserId::from_i32(123456)];
    /// let recipients = BusinessRecipients::new().set_excluded_user_ids(excluded.clone());
    /// assert_eq!(recipients.excluded_user_ids(), &excluded);
    /// ```
    #[must_use]
    pub const fn excluded_user_ids(&self) -> &Vec<UserId> {
        &self.excluded_user_ids
    }

    /// Returns whether existing chats are included.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    ///
    /// let recipients = BusinessRecipients::new().with_existing_chats(true);
    /// assert!(recipients.existing_chats());
    /// ```
    #[must_use]
    pub const fn existing_chats(&self) -> bool {
        self.existing_chats
    }

    /// Returns whether new chats are included.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    ///
    /// let recipients = BusinessRecipients::new().with_new_chats(true);
    /// assert!(recipients.new_chats());
    /// ```
    #[must_use]
    pub const fn new_chats(&self) -> bool {
        self.new_chats
    }

    /// Returns whether contacts are included.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    ///
    /// let recipients = BusinessRecipients::new().with_contacts(true);
    /// assert!(recipients.contacts());
    /// ```
    #[must_use]
    pub const fn contacts(&self) -> bool {
        self.contacts
    }

    /// Returns whether non-contacts are included.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    ///
    /// let recipients = BusinessRecipients::new().with_non_contacts(true);
    /// assert!(recipients.non_contacts());
    /// ```
    #[must_use]
    pub const fn non_contacts(&self) -> bool {
        self.non_contacts
    }

    /// Returns whether selected users are excluded.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    ///
    /// let recipients = BusinessRecipients::new().with_exclude_selected(true);
    /// assert!(recipients.exclude_selected());
    /// ```
    #[must_use]
    pub const fn exclude_selected(&self) -> bool {
        self.exclude_selected
    }

    /// Checks if this BusinessRecipients is empty (no users or criteria set).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    ///
    /// let recipients = BusinessRecipients::new();
    /// assert!(recipients.is_empty());
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

    /// Returns the total count of user IDs (included and excluded).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_recipients::BusinessRecipients;
    /// use rustgram_types::UserId;
    ///
    /// let recipients = BusinessRecipients::new()
    ///     .set_user_ids(vec![UserId::from_i32(123456)])
    ///     .set_excluded_user_ids(vec![UserId::from_i32(789012)]);
    /// assert_eq!(recipients.user_count(), 2);
    /// ```
    #[must_use]
    pub fn user_count(&self) -> usize {
        self.user_ids.len() + self.excluded_user_ids.len()
    }
}

impl fmt::Display for BusinessRecipients {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BusinessRecipients(")?;
        if self.exclude_selected {
            write!(f, "except ")?;
        }
        write!(f, "{} users", self.user_ids.len())?;
        if !self.excluded_user_ids.is_empty() {
            write!(f, ", {} excluded", self.excluded_user_ids.len())?;
        }
        if self.existing_chats {
            write!(f, ", +existing")?;
        }
        if self.new_chats {
            write!(f, ", +new")?;
        }
        if self.contacts {
            write!(f, ", +contacts")?;
        }
        if self.non_contacts {
            write!(f, ", +non-contacts")?;
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (5)
    #[test]
    fn test_default() {
        let recipients = BusinessRecipients::default();
        assert!(recipients.is_empty());
        assert!(!recipients.existing_chats());
    }

    #[test]
    fn test_clone() {
        let recipients = BusinessRecipients::new().with_existing_chats(true);
        let cloned = recipients.clone();
        assert_eq!(recipients, cloned);
    }

    #[test]
    fn test_partial_eq() {
        let r1 = BusinessRecipients::new().with_existing_chats(true);
        let r2 = BusinessRecipients::new().with_existing_chats(true);
        assert_eq!(r1, r2);

        let r3 = BusinessRecipients::new().with_new_chats(true);
        assert_ne!(r1, r3);
    }

    #[test]
    fn test_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<BusinessRecipients>();
        assert_sync::<BusinessRecipients>();
    }

    #[test]
    fn test_debug() {
        let recipients = BusinessRecipients::new();
        let debug = format!("{:?}", recipients);
        assert!(debug.contains("BusinessRecipients"));
    }

    // new() tests (2)
    #[test]
    fn test_new_empty() {
        let recipients = BusinessRecipients::new();
        assert!(recipients.is_empty());
        assert!(!recipients.existing_chats());
        assert!(!recipients.new_chats());
        assert!(!recipients.contacts());
        assert!(!recipients.non_contacts());
        assert!(!recipients.exclude_selected());
    }

    #[test]
    fn test_with_users() {
        let users = vec![UserId::from_i32(123456)];
        let recipients = BusinessRecipients::with_users(users.clone());
        assert_eq!(recipients.user_ids(), &users);
    }

    // Builder tests (10)
    #[test]
    fn test_set_user_ids() {
        let users = vec![UserId::from_i32(123456), UserId::from_i32(789012)];
        let recipients = BusinessRecipients::new().set_user_ids(users.clone());
        assert_eq!(recipients.user_ids(), &users);
    }

    #[test]
    fn test_set_excluded_user_ids() {
        let excluded = vec![UserId::from_i32(123456)];
        let recipients = BusinessRecipients::new().set_excluded_user_ids(excluded.clone());
        assert_eq!(recipients.excluded_user_ids(), &excluded);
    }

    #[test]
    fn test_with_existing_chats() {
        let recipients = BusinessRecipients::new().with_existing_chats(true);
        assert!(recipients.existing_chats());

        let recipients = recipients.with_existing_chats(false);
        assert!(!recipients.existing_chats());
    }

    #[test]
    fn test_with_new_chats() {
        let recipients = BusinessRecipients::new().with_new_chats(true);
        assert!(recipients.new_chats());

        let recipients = recipients.with_new_chats(false);
        assert!(!recipients.new_chats());
    }

    #[test]
    fn test_with_contacts() {
        let recipients = BusinessRecipients::new().with_contacts(true);
        assert!(recipients.contacts());

        let recipients = recipients.with_contacts(false);
        assert!(!recipients.contacts());
    }

    #[test]
    fn test_with_non_contacts() {
        let recipients = BusinessRecipients::new().with_non_contacts(true);
        assert!(recipients.non_contacts());

        let recipients = recipients.with_non_contacts(false);
        assert!(!recipients.non_contacts());
    }

    #[test]
    fn test_with_exclude_selected() {
        let recipients = BusinessRecipients::new().with_exclude_selected(true);
        assert!(recipients.exclude_selected());

        let recipients = recipients.with_exclude_selected(false);
        assert!(!recipients.exclude_selected());
    }

    #[test]
    fn test_builder_chain() {
        let recipients = BusinessRecipients::new()
            .with_existing_chats(true)
            .with_new_chats(true)
            .with_contacts(true);
        assert!(recipients.existing_chats());
        assert!(recipients.new_chats());
        assert!(recipients.contacts());
        assert!(!recipients.non_contacts());
    }

    #[test]
    fn test_user_count() {
        let recipients = BusinessRecipients::new()
            .set_user_ids(vec![UserId::from_i32(123456), UserId::from_i32(789012)])
            .set_excluded_user_ids(vec![UserId::from_i32(345678)]);
        assert_eq!(recipients.user_count(), 3);
    }

    #[test]
    fn test_user_count_empty() {
        let recipients = BusinessRecipients::new();
        assert_eq!(recipients.user_count(), 0);
    }

    // is_empty() tests (3)
    #[test]
    fn test_is_empty_true() {
        let recipients = BusinessRecipients::new();
        assert!(recipients.is_empty());
    }

    #[test]
    fn test_is_empty_with_users() {
        let recipients = BusinessRecipients::new().set_user_ids(vec![UserId::from_i32(123456)]);
        assert!(!recipients.is_empty());
    }

    #[test]
    fn test_is_empty_with_flags() {
        let recipients = BusinessRecipients::new().with_existing_chats(true);
        assert!(!recipients.is_empty());
    }

    // Display tests (3)
    #[test]
    fn test_display_empty() {
        let recipients = BusinessRecipients::new();
        let display = format!("{}", recipients);
        assert!(display.contains("BusinessRecipients"));
        assert!(display.contains("0 users"));
    }

    #[test]
    fn test_display_with_users() {
        let recipients = BusinessRecipients::new()
            .set_user_ids(vec![UserId::from_i32(123456), UserId::from_i32(789012)]);
        let display = format!("{}", recipients);
        assert!(display.contains("2 users"));
    }

    #[test]
    fn test_display_with_excluded() {
        let recipients = BusinessRecipients::new()
            .set_user_ids(vec![UserId::from_i32(123456)])
            .set_excluded_user_ids(vec![UserId::from_i32(789012)])
            .with_exclude_selected(true);
        let display = format!("{}", recipients);
        assert!(display.contains("except"));
        assert!(display.contains("excluded"));
    }

    // Edge case tests (5)
    #[test]
    fn test_multiple_users() {
        let users = vec![
            UserId::from_i32(1),
            UserId::from_i32(2),
            UserId::from_i32(3),
            UserId::from_i32(4),
            UserId::from_i32(5),
        ];
        let recipients = BusinessRecipients::new().set_user_ids(users);
        assert_eq!(recipients.user_ids().len(), 5);
        assert_eq!(recipients.user_count(), 5);
    }

    #[test]
    fn test_all_flags_true() {
        let recipients = BusinessRecipients::new()
            .with_existing_chats(true)
            .with_new_chats(true)
            .with_contacts(true)
            .with_non_contacts(true);
        assert!(recipients.existing_chats());
        assert!(recipients.new_chats());
        assert!(recipients.contacts());
        assert!(recipients.non_contacts());
    }

    #[test]
    fn test_only_excluded_users() {
        let excluded = vec![UserId::from_i32(123456)];
        let recipients = BusinessRecipients::new()
            .set_excluded_user_ids(excluded)
            .with_exclude_selected(true);
        assert!(!recipients.is_empty());
        assert!(recipients.exclude_selected());
    }

    #[test]
    fn test_users_and_flags() {
        let users = vec![UserId::from_i32(123456)];
        let recipients = BusinessRecipients::new()
            .set_user_ids(users)
            .with_contacts(true);
        assert_eq!(recipients.user_ids().len(), 1);
        assert!(recipients.contacts());
    }

    #[test]
    fn test_const_builder() {
        const RECIPIENTS: BusinessRecipients = BusinessRecipients {
            user_ids: Vec::new(),
            excluded_user_ids: Vec::new(),
            existing_chats: true,
            new_chats: false,
            contacts: false,
            non_contacts: false,
            exclude_selected: false,
        };
        assert!(RECIPIENTS.existing_chats());
    }
}
