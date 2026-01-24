// Copyright 2024 rustgram-client contributors
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

//! # Rustgram Business Greeting Message
//!
//! Business greeting message handling for Telegram MTProto client.
//!
//! This crate provides types for managing automatic greeting messages for Telegram
//! business accounts. These messages are sent automatically when a new conversation
//! is started.
//!
//! ## Overview
//!
//! - [`BusinessGreetingMessage`] - Automatic greeting message configuration
//! - [`QuickReplyShortcutId`] - Identifier for quick reply shortcut
//! - [`BusinessRecipients`] - Who receives the greeting message
//! - [`Error`] - Error types for invalid inactivity days
//!
//! ## Greeting Message Components
//!
//! - **Shortcut ID**: References the quick reply message to send
//! - **Recipients**: Defines which contacts/chats receive the message
//! - **Inactivity Days**: Days of inactivity before sending greeting (7, 14, 21, or 28)
//!
//! ## Examples
//!
//! Basic greeting message:
//!
//! ```
//! use rustgram_business_greeting_message::BusinessGreetingMessage;
//!
//! let message = BusinessGreetingMessage::new();
//! assert!(message.is_empty());
//! assert!(!message.is_valid());
//!
//! let valid = BusinessGreetingMessage::with_data(
//!     rustgram_business_greeting_message::QuickReplyShortcutId::new(123),
//!     rustgram_business_greeting_message::BusinessRecipients::new(),
//!     7,
//! ).unwrap();
//! assert!(!valid.is_empty());
//! assert!(valid.is_valid());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::fmt;
use thiserror::Error;

/// Maximum server quick reply shortcut ID.
pub const MAX_SERVER_SHORTCUT_ID: i32 = 1_999_999_999;

/// Valid inactivity days values for greeting messages.
pub const VALID_INACTIVITY_DAYS: [i32; 4] = [7, 14, 21, 28];

/// Quick reply shortcut identifier.
///
/// Identifies a quick reply message that can be used for greeting messages.
///
/// # Examples
///
/// ```
/// use rustgram_business_greeting_message::QuickReplyShortcutId;
///
/// let id = QuickReplyShortcutId::new(123);
/// assert!(id.is_server());
/// assert!(id.is_valid());
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct QuickReplyShortcutId(i32);

impl QuickReplyShortcutId {
    /// Creates a new shortcut ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::QuickReplyShortcutId;
    ///
    /// let id = QuickReplyShortcutId::new(42);
    /// assert_eq!(id.get(), 42);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    /// Returns the raw ID value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::QuickReplyShortcutId;
    ///
    /// let id = QuickReplyShortcutId::new(123);
    /// assert_eq!(id.get(), 123);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get(&self) -> i32 {
        self.0
    }

    /// Checks if this is a server shortcut ID.
    ///
    /// # Returns
    ///
    /// `true` if this is a valid server shortcut ID (1..=MAX_SERVER_SHORTCUT_ID)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::QuickReplyShortcutId;
    ///
    /// assert!(QuickReplyShortcutId::new(1).is_server());
    /// assert!(QuickReplyShortcutId::new(1_999_999_999).is_server());
    /// assert!(!QuickReplyShortcutId::new(0).is_server());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_server(&self) -> bool {
        self.0 > 0 && self.0 <= MAX_SERVER_SHORTCUT_ID
    }

    /// Checks if this shortcut ID is valid (non-zero).
    ///
    /// # Returns
    ///
    /// `true` if the ID is non-zero
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::QuickReplyShortcutId;
    ///
    /// assert!(QuickReplyShortcutId::new(1).is_valid());
    /// assert!(!QuickReplyShortcutId::new(0).is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.0 != 0
    }
}

impl Default for QuickReplyShortcutId {
    fn default() -> Self {
        Self(0)
    }
}

impl fmt::Display for QuickReplyShortcutId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Business recipients for greeting messages.
///
/// Defines which contacts or chats receive the greeting message.
///
/// # Examples
///
/// ```
/// use rustgram_business_greeting_message::BusinessRecipients;
///
/// let recipients = BusinessRecipients::new();
/// assert!(recipients.is_empty());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BusinessRecipients {
    /// Whether to send to all contacts
    all_contacts: bool,
    /// Whether to send to selected contacts only
    selected_contacts: bool,
    /// Whether to send to all chats
    all_chats: bool,
    /// Whether to send to selected chats only
    selected_chats: bool,
}

impl Default for BusinessRecipients {
    fn default() -> Self {
        Self::new()
    }
}

impl BusinessRecipients {
    /// Creates a new empty recipients configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::BusinessRecipients;
    ///
    /// let recipients = BusinessRecipients::new();
    /// assert!(recipients.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            all_contacts: false,
            selected_contacts: false,
            all_chats: false,
            selected_chats: false,
        }
    }

    /// Checks if recipients configuration is empty.
    ///
    /// # Returns
    ///
    /// `true` if no recipients are selected
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::BusinessRecipients;
    ///
    /// assert!(BusinessRecipients::new().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.all_contacts && !self.selected_contacts && !self.all_chats && !self.selected_chats
    }

    /// Returns whether to send to all contacts.
    #[inline]
    #[must_use]
    pub const fn all_contacts(&self) -> bool {
        self.all_contacts
    }

    /// Returns whether to send to selected contacts.
    #[inline]
    #[must_use]
    pub const fn selected_contacts(&self) -> bool {
        self.selected_contacts
    }

    /// Returns whether to send to all chats.
    #[inline]
    #[must_use]
    pub const fn all_chats(&self) -> bool {
        self.all_chats
    }

    /// Returns whether to send to selected chats.
    #[inline]
    #[must_use]
    pub const fn selected_chats(&self) -> bool {
        self.selected_chats
    }

    /// Sets all contacts mode.
    pub fn set_all_contacts(&mut self, value: bool) {
        self.all_contacts = value;
    }

    /// Sets selected contacts mode.
    pub fn set_selected_contacts(&mut self, value: bool) {
        self.selected_contacts = value;
    }

    /// Sets all chats mode.
    pub fn set_all_chats(&mut self, value: bool) {
        self.all_chats = value;
    }

    /// Sets selected chats mode.
    pub fn set_selected_chats(&mut self, value: bool) {
        self.selected_chats = value;
    }
}

/// Error types for business greeting message operations.
///
/// # Examples
///
/// ```
/// use rustgram_business_greeting_message::Error;
///
/// let err = Error::invalid_inactivity_days(5);
/// assert!(err.to_string().contains("Invalid inactivity days"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    /// Invalid inactivity days value.
    ///
    /// Inactivity days must be one of: 7, 14, 21, or 28.
    #[error("Invalid inactivity days: {0}. Must be one of: 7, 14, 21, or 28")]
    InvalidInactivityDays(i32),
}

impl Error {
    /// Creates a new invalid inactivity days error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::Error;
    ///
    /// let err = Error::invalid_inactivity_days(5);
    /// assert_eq!(err, Error::InvalidInactivityDays(5));
    /// ```
    #[must_use]
    pub const fn invalid_inactivity_days(days: i32) -> Self {
        Self::InvalidInactivityDays(days)
    }

    /// Returns the invalid inactivity days value if this is an `InvalidInactivityDays` error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::Error;
    ///
    /// let err = Error::invalid_inactivity_days(5);
    /// assert_eq!(err.invalid_value(), Some(5));
    /// ```
    #[must_use]
    pub const fn invalid_value(&self) -> Option<i32> {
        match self {
            Self::InvalidInactivityDays(days) => Some(*days),
        }
    }
}

/// Business greeting message configuration.
///
/// Manages automatic greeting messages for business accounts that are sent
/// when a new conversation starts.
///
/// # Examples
///
/// ```
/// use rustgram_business_greeting_message::BusinessGreetingMessage;
///
/// let message = BusinessGreetingMessage::new();
/// assert!(message.is_empty());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BusinessGreetingMessage {
    /// The quick reply shortcut to use
    shortcut_id: QuickReplyShortcutId,
    /// Who receives the greeting message
    recipients: BusinessRecipients,
    /// Days of inactivity before sending greeting
    inactivity_days: i32,
}

impl Default for BusinessGreetingMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl BusinessGreetingMessage {
    /// Creates a new empty greeting message.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::BusinessGreetingMessage;
    ///
    /// let message = BusinessGreetingMessage::new();
    /// assert!(message.is_empty());
    /// assert!(!message.is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            shortcut_id: QuickReplyShortcutId::default(),
            recipients: BusinessRecipients::new(),
            inactivity_days: 7,
        }
    }

    /// Creates a greeting message with the given data.
    ///
    /// # Arguments
    ///
    /// * `shortcut_id` - The quick reply shortcut to use
    /// * `recipients` - Who receives the greeting message
    /// * `inactivity_days` - Days of inactivity (must be 7, 14, 21, or 28)
    ///
    /// # Errors
    ///
    /// Returns an error if `inactivity_days` is not one of 7, 14, 21, or 28.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::{BusinessGreetingMessage, QuickReplyShortcutId, BusinessRecipients};
    ///
    /// let message = BusinessGreetingMessage::with_data(
    ///     QuickReplyShortcutId::new(123),
    ///     BusinessRecipients::new(),
    ///     7,
    /// );
    /// assert!(message.is_ok());
    /// assert!(message.unwrap().is_valid());
    ///
    /// let invalid = BusinessGreetingMessage::with_data(
    ///     QuickReplyShortcutId::new(123),
    ///     BusinessRecipients::new(),
    ///     5,
    /// );
    /// assert!(invalid.is_err());
    /// ```
    #[inline]
    pub fn with_data(
        shortcut_id: QuickReplyShortcutId,
        recipients: BusinessRecipients,
        inactivity_days: i32,
    ) -> Result<Self, Error> {
        if !Self::is_valid_inactivity_days(inactivity_days) {
            return Err(Error::invalid_inactivity_days(inactivity_days));
        }

        Ok(Self {
            shortcut_id,
            recipients,
            inactivity_days,
        })
    }

    /// Checks if the greeting message is empty (invalid).
    ///
    /// # Returns
    ///
    /// `true` if the shortcut ID is not a valid server ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::BusinessGreetingMessage;
    ///
    /// assert!(BusinessGreetingMessage::new().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.is_valid()
    }

    /// Checks if the greeting message is valid.
    ///
    /// # Returns
    ///
    /// `true` if the shortcut ID is a valid server ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::{BusinessGreetingMessage, QuickReplyShortcutId, BusinessRecipients};
    ///
    /// let message = BusinessGreetingMessage::with_data(
    ///     QuickReplyShortcutId::new(123),
    ///     BusinessRecipients::new(),
    ///     7,
    /// ).unwrap();
    /// assert!(message.is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.shortcut_id.is_server()
    }

    /// Checks if the given inactivity days value is valid.
    ///
    /// # Arguments
    ///
    /// * `days` - The inactivity days value to check
    ///
    /// # Returns
    ///
    /// `true` if the value is one of 7, 14, 21, or 28
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::BusinessGreetingMessage;
    ///
    /// assert!(BusinessGreetingMessage::is_valid_inactivity_days(7));
    /// assert!(BusinessGreetingMessage::is_valid_inactivity_days(14));
    /// assert!(BusinessGreetingMessage::is_valid_inactivity_days(21));
    /// assert!(BusinessGreetingMessage::is_valid_inactivity_days(28));
    /// assert!(!BusinessGreetingMessage::is_valid_inactivity_days(5));
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_valid_inactivity_days(days: i32) -> bool {
        days == 7 || days == 14 || days == 21 || days == 28
    }

    /// Returns the shortcut ID.
    #[inline]
    #[must_use]
    pub const fn shortcut_id(&self) -> QuickReplyShortcutId {
        self.shortcut_id
    }

    /// Returns the recipients.
    #[inline]
    #[must_use]
    pub const fn recipients(&self) -> &BusinessRecipients {
        &self.recipients
    }

    /// Returns the inactivity days.
    #[inline]
    #[must_use]
    pub const fn inactivity_days(&self) -> i32 {
        self.inactivity_days
    }

    /// Sets the shortcut ID.
    pub fn set_shortcut_id(&mut self, id: QuickReplyShortcutId) {
        self.shortcut_id = id;
    }

    /// Sets the recipients.
    pub fn set_recipients(&mut self, recipients: BusinessRecipients) {
        self.recipients = recipients;
    }

    /// Sets the inactivity days.
    ///
    /// # Errors
    ///
    /// Returns an error if `days` is not one of 7, 14, 21, or 28.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::{BusinessGreetingMessage, QuickReplyShortcutId, BusinessRecipients};
    ///
    /// let mut message = BusinessGreetingMessage::with_data(
    ///     QuickReplyShortcutId::new(123),
    ///     BusinessRecipients::new(),
    ///     7,
    /// ).unwrap();
    ///
    /// assert!(message.set_inactivity_days(14).is_ok());
    /// assert_eq!(message.inactivity_days(), 14);
    ///
    /// assert!(message.set_inactivity_days(5).is_err());
    /// ```
    pub fn set_inactivity_days(&mut self, days: i32) -> Result<(), Error> {
        if !Self::is_valid_inactivity_days(days) {
            return Err(Error::invalid_inactivity_days(days));
        }
        self.inactivity_days = days;
        Ok(())
    }

    /// Converts to TD API representation.
    ///
    /// Returns a tuple of (shortcut_id, recipients, inactivity_days).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::{BusinessGreetingMessage, QuickReplyShortcutId, BusinessRecipients};
    ///
    /// let message = BusinessGreetingMessage::with_data(
    ///     QuickReplyShortcutId::new(123),
    ///     BusinessRecipients::new(),
    ///     7,
    /// ).unwrap();
    ///
    /// let (id, recipients, days) = message.to_td_api();
    /// assert_eq!(id, 123);
    /// assert_eq!(days, 7);
    /// ```
    #[must_use]
    pub fn to_td_api(&self) -> (i32, &BusinessRecipients, i32) {
        (
            self.shortcut_id.get(),
            &self.recipients,
            self.inactivity_days,
        )
    }

    /// Creates from TD API representation.
    ///
    /// # Arguments
    ///
    /// * `shortcut_id` - The quick reply shortcut ID
    /// * `recipients` - Who receives the greeting message
    /// * `inactivity_days` - Days of inactivity (must be 7, 14, 21, or 28)
    ///
    /// # Errors
    ///
    /// Returns an error if `inactivity_days` is not one of 7, 14, 21, or 28.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_greeting_message::{BusinessGreetingMessage, BusinessRecipients};
    ///
    /// let message = BusinessGreetingMessage::from_td_api(
    ///     123,
    ///     BusinessRecipients::new(),
    ///     14,
    /// );
    /// assert!(message.is_ok());
    /// assert_eq!(message.unwrap().inactivity_days(), 14);
    /// ```
    pub fn from_td_api(
        shortcut_id: i32,
        recipients: BusinessRecipients,
        inactivity_days: i32,
    ) -> Result<Self, Error> {
        Self::with_data(
            QuickReplyShortcutId::new(shortcut_id),
            recipients,
            inactivity_days,
        )
    }
}

impl fmt::Display for BusinessGreetingMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BusinessGreetingMessage {{ shortcut: {}, valid: {}, inactivity_days: {} }}",
            self.shortcut_id.get(),
            self.is_valid(),
            self.inactivity_days
        )
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-business-greeting-message";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== QuickReplyShortcutId Tests ==========

    #[test]
    fn test_shortcut_id_new() {
        let id = QuickReplyShortcutId::new(42);
        assert_eq!(id.get(), 42);
    }

    #[test]
    fn test_shortcut_id_is_server_valid() {
        assert!(QuickReplyShortcutId::new(1).is_server());
        assert!(QuickReplyShortcutId::new(100).is_server());
        assert!(QuickReplyShortcutId::new(MAX_SERVER_SHORTCUT_ID).is_server());
    }

    #[test]
    fn test_shortcut_id_is_server_invalid() {
        assert!(!QuickReplyShortcutId::new(0).is_server());
        assert!(!QuickReplyShortcutId::new(-1).is_server());
        assert!(!QuickReplyShortcutId::new(MAX_SERVER_SHORTCUT_ID + 1).is_server());
    }

    #[test]
    fn test_shortcut_id_is_valid() {
        assert!(QuickReplyShortcutId::new(1).is_valid());
        assert!(QuickReplyShortcutId::new(-1).is_valid());
        assert!(!QuickReplyShortcutId::new(0).is_valid());
    }

    #[test]
    fn test_shortcut_id_default() {
        let id = QuickReplyShortcutId::default();
        assert_eq!(id.get(), 0);
    }

    #[test]
    fn test_shortcut_id_display() {
        let id = QuickReplyShortcutId::new(123);
        assert_eq!(format!("{}", id), "123");
    }

    // ========== BusinessRecipients Tests ==========

    #[test]
    fn test_recipients_new_is_empty() {
        let recipients = BusinessRecipients::new();
        assert!(recipients.is_empty());
        assert!(!recipients.all_contacts());
        assert!(!recipients.selected_contacts());
        assert!(!recipients.all_chats());
        assert!(!recipients.selected_chats());
    }

    #[test]
    fn test_recipients_all_contacts() {
        let mut recipients = BusinessRecipients::new();
        recipients.set_all_contacts(true);
        assert!(!recipients.is_empty());
        assert!(recipients.all_contacts());
    }

    #[test]
    fn test_recipients_selected_contacts() {
        let mut recipients = BusinessRecipients::new();
        recipients.set_selected_contacts(true);
        assert!(!recipients.is_empty());
        assert!(recipients.selected_contacts());
    }

    #[test]
    fn test_recipients_all_chats() {
        let mut recipients = BusinessRecipients::new();
        recipients.set_all_chats(true);
        assert!(!recipients.is_empty());
        assert!(recipients.all_chats());
    }

    #[test]
    fn test_recipients_selected_chats() {
        let mut recipients = BusinessRecipients::new();
        recipients.set_selected_chats(true);
        assert!(!recipients.is_empty());
        assert!(recipients.selected_chats());
    }

    #[test]
    fn test_recipients_multiple_flags() {
        let mut recipients = BusinessRecipients::new();
        recipients.set_all_contacts(true);
        recipients.set_selected_chats(true);
        assert!(!recipients.is_empty());
        assert!(recipients.all_contacts());
        assert!(recipients.selected_chats());
    }

    // ========== Error Tests ==========

    #[test]
    fn test_error_invalid_inactivity_days() {
        let err = Error::invalid_inactivity_days(5);
        assert_eq!(err, Error::InvalidInactivityDays(5));
        assert!(err.to_string().contains("Invalid inactivity days"));
        assert!(err.to_string().contains("5"));
    }

    #[test]
    fn test_error_invalid_value() {
        let err = Error::invalid_inactivity_days(10);
        assert_eq!(err.invalid_value(), Some(10));
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(
            Error::invalid_inactivity_days(5),
            Error::invalid_inactivity_days(5)
        );
        assert_ne!(
            Error::invalid_inactivity_days(5),
            Error::invalid_inactivity_days(10)
        );
    }

    // ========== BusinessGreetingMessage Constructor Tests ==========

    #[test]
    fn test_new_creates_empty() {
        let message = BusinessGreetingMessage::new();
        assert!(message.is_empty());
        assert!(!message.is_valid());
        assert_eq!(message.inactivity_days(), 7);
    }

    #[test]
    fn test_default_creates_empty() {
        let message = BusinessGreetingMessage::default();
        assert!(message.is_empty());
    }

    #[test]
    fn test_with_data_valid_inactivity() {
        for days in VALID_INACTIVITY_DAYS {
            let message = BusinessGreetingMessage::with_data(
                QuickReplyShortcutId::new(123),
                BusinessRecipients::new(),
                days,
            );
            assert!(message.is_ok());
            assert_eq!(message.unwrap().inactivity_days(), days);
        }
    }

    #[test]
    fn test_with_data_invalid_inactivity() {
        let message = BusinessGreetingMessage::with_data(
            QuickReplyShortcutId::new(123),
            BusinessRecipients::new(),
            5,
        );
        assert!(message.is_err());
        assert_eq!(message.unwrap_err(), Error::invalid_inactivity_days(5));
    }

    #[test]
    fn test_with_data_sets_values() {
        let mut recipients = BusinessRecipients::new();
        recipients.set_all_contacts(true);

        let message = BusinessGreetingMessage::with_data(
            QuickReplyShortcutId::new(123),
            recipients.clone(),
            14,
        );

        assert!(message.is_ok());
        let message = message.unwrap();
        assert_eq!(message.shortcut_id(), QuickReplyShortcutId::new(123));
        assert_eq!(message.recipients(), &recipients);
        assert_eq!(message.inactivity_days(), 14);
    }

    // ========== BusinessGreetingMessage is_empty/is_valid Tests ==========

    #[test]
    fn test_is_empty_when_invalid_shortcut() {
        let message = BusinessGreetingMessage::new();
        assert!(message.is_empty());
        assert!(!message.is_valid());
    }

    #[test]
    fn test_is_empty_when_local_shortcut() {
        let message = BusinessGreetingMessage::with_data(
            QuickReplyShortcutId::new(-1),
            BusinessRecipients::new(),
            7,
        )
        .unwrap();
        assert!(message.is_empty());
        assert!(!message.is_valid());
    }

    #[test]
    fn test_is_valid_when_server_shortcut() {
        let message = BusinessGreetingMessage::with_data(
            QuickReplyShortcutId::new(123),
            BusinessRecipients::new(),
            7,
        )
        .unwrap();
        assert!(!message.is_empty());
        assert!(message.is_valid());
    }

    // ========== is_valid_inactivity_days Tests ==========

    #[test]
    fn test_is_valid_inactivity_days_valid() {
        assert!(BusinessGreetingMessage::is_valid_inactivity_days(7));
        assert!(BusinessGreetingMessage::is_valid_inactivity_days(14));
        assert!(BusinessGreetingMessage::is_valid_inactivity_days(21));
        assert!(BusinessGreetingMessage::is_valid_inactivity_days(28));
    }

    #[test]
    fn test_is_valid_inactivity_days_invalid() {
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(0));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(5));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(10));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(15));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(30));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(-1));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(100));
    }

    // ========== BusinessGreetingMessage Mutator Tests ==========

    #[test]
    fn test_set_shortcut_id() {
        let mut message = BusinessGreetingMessage::new();
        assert!(!message.is_valid());

        message.set_shortcut_id(QuickReplyShortcutId::new(123));
        assert!(message.is_valid());
    }

    #[test]
    fn test_set_recipients() {
        let mut message = BusinessGreetingMessage::new();
        let mut recipients = BusinessRecipients::new();
        recipients.set_all_chats(true);

        message.set_recipients(recipients);
        assert!(message.recipients().all_chats());
    }

    #[test]
    fn test_set_inactivity_days_valid() {
        let mut message = BusinessGreetingMessage::new();
        assert_eq!(message.inactivity_days(), 7);

        for days in VALID_INACTIVITY_DAYS {
            assert!(message.set_inactivity_days(days).is_ok());
            assert_eq!(message.inactivity_days(), days);
        }
    }

    #[test]
    fn test_set_inactivity_days_invalid() {
        let mut message = BusinessGreetingMessage::new();
        assert!(message.set_inactivity_days(5).is_err());
        assert_eq!(message.inactivity_days(), 7); // Unchanged
    }

    // ========== TD API Conversion Tests ==========

    #[test]
    fn test_to_td_api() {
        let recipients = BusinessRecipients::new();

        let message =
            BusinessGreetingMessage::with_data(QuickReplyShortcutId::new(456), recipients, 21)
                .unwrap();

        let (id, ret_recipients, days) = message.to_td_api();
        assert_eq!(id, 456);
        assert_eq!(days, 21);
        assert!(ret_recipients.is_empty());
    }

    #[test]
    fn test_from_td_api_valid() {
        let recipients = BusinessRecipients::new();

        let message = BusinessGreetingMessage::from_td_api(789, recipients, 28);
        assert!(message.is_ok());

        let message = message.unwrap();
        assert_eq!(message.shortcut_id().get(), 789);
        assert_eq!(message.inactivity_days(), 28);
    }

    #[test]
    fn test_from_td_api_invalid() {
        let recipients = BusinessRecipients::new();

        let message = BusinessGreetingMessage::from_td_api(789, recipients, 5);
        assert!(message.is_err());
        assert_eq!(message.unwrap_err(), Error::invalid_inactivity_days(5));
    }

    // ========== Equality Tests ==========

    #[test]
    fn test_equality_same_values() {
        let recipients = BusinessRecipients::new();

        let message1 = BusinessGreetingMessage::with_data(
            QuickReplyShortcutId::new(123),
            recipients.clone(),
            7,
        )
        .unwrap();
        let message2 =
            BusinessGreetingMessage::with_data(QuickReplyShortcutId::new(123), recipients, 7)
                .unwrap();
        assert_eq!(message1, message2);
    }

    #[test]
    fn test_equality_different_shortcut() {
        let recipients = BusinessRecipients::new();

        let message1 = BusinessGreetingMessage::with_data(
            QuickReplyShortcutId::new(123),
            recipients.clone(),
            7,
        )
        .unwrap();
        let message2 =
            BusinessGreetingMessage::with_data(QuickReplyShortcutId::new(456), recipients, 7)
                .unwrap();
        assert_ne!(message1, message2);
    }

    #[test]
    fn test_equality_different_inactivity_days() {
        let recipients = BusinessRecipients::new();

        let message1 = BusinessGreetingMessage::with_data(
            QuickReplyShortcutId::new(123),
            recipients.clone(),
            7,
        )
        .unwrap();
        let message2 =
            BusinessGreetingMessage::with_data(QuickReplyShortcutId::new(123), recipients, 14)
                .unwrap();
        assert_ne!(message1, message2);
    }

    // ========== Clone Tests ==========

    #[test]
    fn test_clone_message() {
        let message = BusinessGreetingMessage::with_data(
            QuickReplyShortcutId::new(123),
            BusinessRecipients::new(),
            7,
        )
        .unwrap();
        let cloned = message.clone();
        assert_eq!(message, cloned);
    }

    #[test]
    fn test_clone_recipients() {
        let mut recipients = BusinessRecipients::new();
        recipients.set_all_contacts(true);
        let cloned = recipients.clone();
        assert_eq!(recipients, cloned);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_format() {
        let message = BusinessGreetingMessage::with_data(
            QuickReplyShortcutId::new(123),
            BusinessRecipients::new(),
            7,
        )
        .unwrap();
        let display = format!("{}", message);
        assert!(display.contains("123"));
        assert!(display.contains("true")); // is_valid
        assert!(display.contains("7"));
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-business-greeting-message");
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_shortcut_id_max_value() {
        let id = QuickReplyShortcutId::new(MAX_SERVER_SHORTCUT_ID);
        assert!(id.is_server());
    }

    #[test]
    fn test_shortcut_id_beyond_max() {
        let id = QuickReplyShortcutId::new(MAX_SERVER_SHORTCUT_ID + 1);
        assert!(!id.is_server());
        assert!(id.is_valid()); // Still valid (non-zero)
    }

    #[test]
    fn test_recipients_toggle() {
        let mut recipients = BusinessRecipients::new();
        assert!(!recipients.all_contacts());

        recipients.set_all_contacts(true);
        assert!(recipients.all_contacts());

        recipients.set_all_contacts(false);
        assert!(!recipients.all_contacts());
    }

    #[test]
    fn test_all_valid_inactivity_days() {
        for days in VALID_INACTIVITY_DAYS {
            assert!(BusinessGreetingMessage::is_valid_inactivity_days(days));

            let message = BusinessGreetingMessage::with_data(
                QuickReplyShortcutId::new(123),
                BusinessRecipients::new(),
                days,
            );
            assert!(message.is_ok());
        }
    }

    #[test]
    fn test_inactivity_days_boundary_values() {
        // Test values just outside valid range
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(6));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(8));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(13));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(15));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(20));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(22));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(27));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(29));
    }

    #[test]
    fn test_inactivity_days_negative() {
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(-1));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(-7));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(i32::MIN));
    }

    #[test]
    fn test_inactivity_days_large_values() {
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(100));
        assert!(!BusinessGreetingMessage::is_valid_inactivity_days(i32::MAX));
    }

    #[test]
    fn test_message_with_all_recipient_types() {
        let mut recipients = BusinessRecipients::new();
        recipients.set_all_contacts(true);
        recipients.set_selected_contacts(true);
        recipients.set_all_chats(true);
        recipients.set_selected_chats(true);

        let message =
            BusinessGreetingMessage::with_data(QuickReplyShortcutId::new(123), recipients, 28)
                .unwrap();

        assert!(!message.recipients().is_empty());
        assert!(message.recipients().all_contacts());
        assert!(message.recipients().selected_contacts());
        assert!(message.recipients().all_chats());
        assert!(message.recipients().selected_chats());
    }

    #[test]
    fn test_roundtrip_td_api() {
        let mut recipients = BusinessRecipients::new();
        recipients.set_all_contacts(true);

        let original = BusinessGreetingMessage::with_data(
            QuickReplyShortcutId::new(999),
            recipients.clone(),
            21,
        )
        .unwrap();

        let (id, rec, days) = original.to_td_api();
        let restored = BusinessGreetingMessage::from_td_api(id, rec.clone(), days).unwrap();

        assert_eq!(original, restored);
    }

    #[test]
    fn test_error_message_formatting() {
        let err = Error::invalid_inactivity_days(5);
        let msg = format!("{}", err);
        assert!(msg.contains("5"));
        assert!(
            msg.contains("7") || msg.contains("14") || msg.contains("21") || msg.contains("28")
        );
    }
}
