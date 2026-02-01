//! # Rustgram BusinessAwayMessage
//!
//! Business away message handling for Telegram MTProto client.
//!
//! This crate provides types for managing automatic away messages for Telegram
//! business accounts. These messages are sent automatically when the business
//! is offline or outside working hours.
//!
//! ## Overview
//!
//! - [`BusinessAwayMessage`] - Automatic away message configuration
//! - [`QuickReplyShortcutId`] - Identifier for quick reply shortcut
//! - [`BusinessRecipients`] - Who receives the away message
//! - [`BusinessAwayMessageSchedule`] - When the away message is active
//!
//! ## Away Message Components
//!
//! - **Shortcut ID**: References the quick reply message to send
//! - **Recipients**: Defines which contacts/chats receive the message
//! - **Schedule**: Defines when the auto-reply is active
//! - **Offline Only**: Whether to only reply when offline
//!
//! ## Examples
//!
//! Basic away message:
//!
//! ```
//! use rustgram_business_away_message::{BusinessAwayMessage, BusinessAwayMessageSchedule};
//!
//! let message = BusinessAwayMessage::new();
//! assert!(message.is_empty());
//! assert!(!message.is_valid());
//!
//! let schedule = BusinessAwayMessageSchedule::Always;
//! let valid = BusinessAwayMessage::with_data(
//!     rustgram_business_away_message::QuickReplyShortcutId::new(123),
//!     rustgram_business_away_message::BusinessRecipients::new(),
//!     schedule,
//!     false,
//! );
//! assert!(!valid.is_empty());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Maximum server quick reply shortcut ID.
pub const MAX_SERVER_SHORTCUT_ID: i32 = 1_999_999_999;

/// Quick reply shortcut identifier.
///
/// Identifies a quick reply message that can be used for away messages.
///
/// # Examples
///
/// ```
/// use rustgram_business_away_message::QuickReplyShortcutId;
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
    /// use rustgram_business_away_message::QuickReplyShortcutId;
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
    /// use rustgram_business_away_message::QuickReplyShortcutId;
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
    /// use rustgram_business_away_message::QuickReplyShortcutId;
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
    /// use rustgram_business_away_message::QuickReplyShortcutId;
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

/// Business recipients for away messages.
///
/// Defines which contacts or chats receive the away message.
///
/// # Examples
///
/// ```
/// use rustgram_business_away_message::BusinessRecipients;
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
    /// use rustgram_business_away_message::BusinessRecipients;
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
    /// use rustgram_business_away_message::BusinessRecipients;
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

/// Away message schedule.
///
/// Defines when the away message is automatically sent.
///
/// # Examples
///
/// ```
/// use rustgram_business_away_message::BusinessAwayMessageSchedule;
///
/// let schedule = BusinessAwayMessageSchedule::Always;
/// assert!(matches!(schedule, BusinessAwayMessageSchedule::Always));
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BusinessAwayMessageSchedule {
    /// Away message is always active
    Always,
    /// Away message active outside work hours
    OutsideWorkHours,
    /// Away message active for a custom time period
    Custom {
        /// Start timestamp
        start_date: i32,
        /// End timestamp
        end_date: i32,
    },
}

impl BusinessAwayMessageSchedule {
    /// Checks if the schedule is currently active.
    ///
    /// # Returns
    ///
    /// `true` if the away message should be sent now
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_away_message::BusinessAwayMessageSchedule;
    ///
    /// assert!(BusinessAwayMessageSchedule::Always.is_active());
    /// ```
    #[must_use]
    pub fn is_active(&self) -> bool {
        match self {
            Self::Always => true,
            Self::OutsideWorkHours => Self::is_outside_work_hours(),
            Self::Custom {
                start_date,
                end_date,
            } => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs() as i32)
                    .unwrap_or(0);
                *start_date <= now && now <= *end_date
            }
        }
    }

    fn is_outside_work_hours() -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // Simple check: outside 9-17 UTC on weekdays
        let seconds_in_day = now % 86400;
        let day_of_week = (now / 86400) % 7;

        // Weekend (Saturday=5, Sunday=6)
        if day_of_week >= 5 {
            return true;
        }

        // Outside working hours: before 9 AM (32400 seconds) or after 5 PM (61200 seconds)
        !(32400..=61200).contains(&seconds_in_day)
    }
}

impl Default for BusinessAwayMessageSchedule {
    fn default() -> Self {
        Self::Always
    }
}

/// Business away message configuration.
///
/// Manages automatic away messages for business accounts.
///
/// # Examples
///
/// ```
/// use rustgram_business_away_message::{BusinessAwayMessage, BusinessAwayMessageSchedule};
///
/// let message = BusinessAwayMessage::new();
/// assert!(message.is_empty());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BusinessAwayMessage {
    /// The quick reply shortcut to use
    shortcut_id: QuickReplyShortcutId,
    /// Who receives the away message
    recipients: BusinessRecipients,
    /// When the away message is active
    schedule: BusinessAwayMessageSchedule,
    /// Whether to only reply when offline
    offline_only: bool,
}

impl Default for BusinessAwayMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl BusinessAwayMessage {
    /// Creates a new empty away message.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_away_message::BusinessAwayMessage;
    ///
    /// let message = BusinessAwayMessage::new();
    /// assert!(message.is_empty());
    /// assert!(!message.is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            shortcut_id: QuickReplyShortcutId::default(),
            recipients: BusinessRecipients::new(),
            schedule: BusinessAwayMessageSchedule::Always,
            offline_only: false,
        }
    }

    /// Creates an away message with the given data.
    ///
    /// # Arguments
    ///
    /// * `shortcut_id` - The quick reply shortcut to use
    /// * `recipients` - Who receives the away message
    /// * `schedule` - When the away message is active
    /// * `offline_only` - Whether to only reply when offline
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_away_message::{BusinessAwayMessage, BusinessAwayMessageSchedule, QuickReplyShortcutId, BusinessRecipients};
    ///
    /// let message = BusinessAwayMessage::with_data(
    ///     QuickReplyShortcutId::new(123),
    ///     BusinessRecipients::new(),
    ///     BusinessAwayMessageSchedule::Always,
    ///     false,
    /// );
    /// assert!(message.is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub fn with_data(
        shortcut_id: QuickReplyShortcutId,
        recipients: BusinessRecipients,
        schedule: BusinessAwayMessageSchedule,
        offline_only: bool,
    ) -> Self {
        Self {
            shortcut_id,
            recipients,
            schedule,
            offline_only,
        }
    }

    /// Checks if the away message is empty (invalid).
    ///
    /// # Returns
    ///
    /// `true` if the shortcut ID is not a valid server ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_away_message::BusinessAwayMessage;
    ///
    /// assert!(BusinessAwayMessage::new().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.is_valid()
    }

    /// Checks if the away message is valid.
    ///
    /// # Returns
    ///
    /// `true` if the shortcut ID is a valid server ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_away_message::{BusinessAwayMessage, QuickReplyShortcutId, BusinessRecipients, BusinessAwayMessageSchedule};
    ///
    /// let message = BusinessAwayMessage::with_data(
    ///     QuickReplyShortcutId::new(123),
    ///     BusinessRecipients::new(),
    ///     BusinessAwayMessageSchedule::Always,
    ///     false,
    /// );
    /// assert!(message.is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.shortcut_id.is_server()
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

    /// Returns the schedule.
    #[inline]
    #[must_use]
    pub const fn schedule(&self) -> &BusinessAwayMessageSchedule {
        &self.schedule
    }

    /// Returns whether offline-only mode is enabled.
    #[inline]
    #[must_use]
    pub const fn offline_only(&self) -> bool {
        self.offline_only
    }

    /// Checks if the away message should be sent.
    ///
    /// # Returns
    ///
    /// `true` if conditions are met for sending the away message
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_away_message::{BusinessAwayMessage, BusinessAwayMessageSchedule};
    ///
    /// let message = BusinessAwayMessage::new();
    /// assert!(!message.should_send(false)); // Not valid
    /// ```
    #[must_use]
    pub fn should_send(&self, is_offline: bool) -> bool {
        if !self.is_valid() || self.recipients.is_empty() {
            return false;
        }

        if self.offline_only && !is_offline {
            return false;
        }

        self.schedule.is_active()
    }

    /// Sets the shortcut ID.
    pub fn set_shortcut_id(&mut self, id: QuickReplyShortcutId) {
        self.shortcut_id = id;
    }

    /// Sets the recipients.
    pub fn set_recipients(&mut self, recipients: BusinessRecipients) {
        self.recipients = recipients;
    }

    /// Sets the schedule.
    pub fn set_schedule(&mut self, schedule: BusinessAwayMessageSchedule) {
        self.schedule = schedule;
    }

    /// Sets offline-only mode.
    pub fn set_offline_only(&mut self, value: bool) {
        self.offline_only = value;
    }

    /// Converts to TD API representation.
    ///
    /// Returns data for `td_api::businessAwayMessageSettings`.
    #[must_use]
    pub fn to_td_api(&self) -> (i32, bool, BusinessAwayMessageSchedule) {
        (
            self.shortcut_id.get(),
            self.offline_only,
            self.schedule.clone(),
        )
    }

    /// Creates from TD API representation.
    ///
    /// Creates from `telegram_api::businessAwayMessage`.
    #[must_use]
    pub fn from_td_api(
        shortcut_id: i32,
        recipients: BusinessRecipients,
        schedule: BusinessAwayMessageSchedule,
        offline_only: bool,
    ) -> Self {
        Self {
            shortcut_id: QuickReplyShortcutId::new(shortcut_id),
            recipients,
            schedule,
            offline_only,
        }
    }
}

impl fmt::Display for BusinessAwayMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BusinessAwayMessage {{ shortcut: {}, valid: {}, offline_only: {} }}",
            self.shortcut_id.get(),
            self.is_valid(),
            self.offline_only
        )
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-business-away-message";

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

    // ========== BusinessAwayMessageSchedule Tests ==========

    #[test]
    fn test_schedule_always_is_active() {
        assert!(BusinessAwayMessageSchedule::Always.is_active());
    }

    #[test]
    fn test_schedule_outside_work_hours() {
        let result = BusinessAwayMessageSchedule::OutsideWorkHours.is_active();
        // Result depends on current time
        // Just ensure it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_schedule_custom_active_range() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        let schedule = BusinessAwayMessageSchedule::Custom {
            start_date: now - 100,
            end_date: now + 100,
        };
        assert!(schedule.is_active());
    }

    #[test]
    fn test_schedule_custom_before_range() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        let schedule = BusinessAwayMessageSchedule::Custom {
            start_date: now + 100,
            end_date: now + 200,
        };
        assert!(!schedule.is_active());
    }

    #[test]
    fn test_schedule_custom_after_range() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        let schedule = BusinessAwayMessageSchedule::Custom {
            start_date: now - 200,
            end_date: now - 100,
        };
        assert!(!schedule.is_active());
    }

    #[test]
    fn test_schedule_default() {
        let schedule = BusinessAwayMessageSchedule::default();
        assert!(matches!(schedule, BusinessAwayMessageSchedule::Always));
    }

    // ========== BusinessAwayMessage Constructor Tests ==========

    #[test]
    fn test_new_creates_empty() {
        let message = BusinessAwayMessage::new();
        assert!(message.is_empty());
        assert!(!message.is_valid());
        assert!(!message.offline_only());
    }

    #[test]
    fn test_default_creates_empty() {
        let message = BusinessAwayMessage::default();
        assert!(message.is_empty());
    }

    #[test]
    fn test_with_data_sets_values() {
        let shortcut_id = QuickReplyShortcutId::new(123);
        let recipients = BusinessRecipients::new();
        let schedule = BusinessAwayMessageSchedule::Always;

        let message =
            BusinessAwayMessage::with_data(shortcut_id, recipients.clone(), schedule.clone(), true);

        assert_eq!(message.shortcut_id(), shortcut_id);
        assert_eq!(message.recipients(), &recipients);
        assert_eq!(message.schedule(), &schedule);
        assert!(message.offline_only());
    }

    // ========== BusinessAwayMessage is_empty/is_valid Tests ==========

    #[test]
    fn test_is_empty_when_invalid_shortcut() {
        let message = BusinessAwayMessage::new();
        assert!(message.is_empty());
        assert!(!message.is_valid());
    }

    #[test]
    fn test_is_empty_when_local_shortcut() {
        let message = BusinessAwayMessage::with_data(
            QuickReplyShortcutId::new(-1),
            BusinessRecipients::new(),
            BusinessAwayMessageSchedule::Always,
            false,
        );
        assert!(message.is_empty());
        assert!(!message.is_valid());
    }

    #[test]
    fn test_is_valid_when_server_shortcut() {
        let message = BusinessAwayMessage::with_data(
            QuickReplyShortcutId::new(123),
            BusinessRecipients::new(),
            BusinessAwayMessageSchedule::Always,
            false,
        );
        assert!(!message.is_empty());
        assert!(message.is_valid());
    }

    // ========== BusinessAwayMessage should_send Tests ==========

    #[test]
    fn test_should_send_when_invalid() {
        let message = BusinessAwayMessage::new();
        assert!(!message.should_send(true));
        assert!(!message.should_send(false));
    }

    #[test]
    fn test_should_send_when_no_recipients() {
        let mut message = BusinessAwayMessage::with_data(
            QuickReplyShortcutId::new(123),
            BusinessRecipients::new(),
            BusinessAwayMessageSchedule::Always,
            false,
        );
        assert!(!message.should_send(false)); // Empty recipients
    }

    #[test]
    fn test_should_send_with_recipients() {
        let mut recipients = BusinessRecipients::new();
        recipients.set_all_contacts(true);

        let message = BusinessAwayMessage::with_data(
            QuickReplyShortcutId::new(123),
            recipients,
            BusinessAwayMessageSchedule::Always,
            false,
        );
        assert!(message.should_send(false));
    }

    #[test]
    fn test_should_send_offline_only_when_online() {
        let mut recipients = BusinessRecipients::new();
        recipients.set_all_contacts(true);

        let message = BusinessAwayMessage::with_data(
            QuickReplyShortcutId::new(123),
            recipients,
            BusinessAwayMessageSchedule::Always,
            true, // offline_only
        );
        assert!(!message.should_send(false)); // Online
        assert!(message.should_send(true)); // Offline
    }

    #[test]
    fn test_should_send_checks_schedule() {
        let mut recipients = BusinessRecipients::new();
        recipients.set_all_contacts(true);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        let schedule = BusinessAwayMessageSchedule::Custom {
            start_date: now - 100,
            end_date: now + 100,
        };

        let message = BusinessAwayMessage::with_data(
            QuickReplyShortcutId::new(123),
            recipients,
            schedule,
            false,
        );
        assert!(message.should_send(false));
    }

    // ========== BusinessAwayMessage Mutator Tests ==========

    #[test]
    fn test_set_shortcut_id() {
        let mut message = BusinessAwayMessage::new();
        assert!(!message.is_valid());

        message.set_shortcut_id(QuickReplyShortcutId::new(123));
        assert!(message.is_valid());
    }

    #[test]
    fn test_set_recipients() {
        let mut message = BusinessAwayMessage::new();
        let mut recipients = BusinessRecipients::new();
        recipients.set_all_chats(true);

        message.set_recipients(recipients);
        assert!(message.recipients().all_chats());
    }

    #[test]
    fn test_set_schedule() {
        let mut message = BusinessAwayMessage::new();
        let schedule = BusinessAwayMessageSchedule::OutsideWorkHours;

        message.set_schedule(schedule.clone());
        assert_eq!(message.schedule(), &schedule);
    }

    #[test]
    fn test_set_offline_only() {
        let mut message = BusinessAwayMessage::new();
        assert!(!message.offline_only());

        message.set_offline_only(true);
        assert!(message.offline_only());

        message.set_offline_only(false);
        assert!(!message.offline_only());
    }

    // ========== TD API Conversion Tests ==========

    #[test]
    fn test_to_td_api() {
        let recipients = BusinessRecipients::new();
        let schedule = BusinessAwayMessageSchedule::OutsideWorkHours;

        let message = BusinessAwayMessage::with_data(
            QuickReplyShortcutId::new(456),
            recipients,
            schedule.clone(),
            true,
        );

        let (id, offline_only, ret_schedule) = message.to_td_api();
        assert_eq!(id, 456);
        assert!(offline_only);
        assert_eq!(ret_schedule, schedule);
    }

    #[test]
    fn test_from_td_api() {
        let recipients = BusinessRecipients::new();
        let schedule = BusinessAwayMessageSchedule::Always;

        let message =
            BusinessAwayMessage::from_td_api(789, recipients.clone(), schedule.clone(), false);

        assert_eq!(message.shortcut_id().get(), 789);
        assert_eq!(message.recipients(), &recipients);
        assert_eq!(message.schedule(), &schedule);
        assert!(!message.offline_only());
    }

    // ========== Equality Tests ==========

    #[test]
    fn test_equality_same_values() {
        let recipients = BusinessRecipients::new();
        let schedule = BusinessAwayMessageSchedule::Always;

        let message1 = BusinessAwayMessage::with_data(
            QuickReplyShortcutId::new(123),
            recipients.clone(),
            schedule.clone(),
            false,
        );
        let message2 = BusinessAwayMessage::with_data(
            QuickReplyShortcutId::new(123),
            recipients,
            schedule,
            false,
        );
        assert_eq!(message1, message2);
    }

    #[test]
    fn test_equality_different_shortcut() {
        let recipients = BusinessRecipients::new();
        let schedule = BusinessAwayMessageSchedule::Always;

        let message1 = BusinessAwayMessage::with_data(
            QuickReplyShortcutId::new(123),
            recipients.clone(),
            schedule.clone(),
            false,
        );
        let message2 = BusinessAwayMessage::with_data(
            QuickReplyShortcutId::new(456),
            recipients,
            schedule,
            false,
        );
        assert_ne!(message1, message2);
    }

    #[test]
    fn test_schedule_equality() {
        assert_eq!(
            BusinessAwayMessageSchedule::Always,
            BusinessAwayMessageSchedule::Always
        );
        assert_ne!(
            BusinessAwayMessageSchedule::Always,
            BusinessAwayMessageSchedule::OutsideWorkHours
        );
    }

    // ========== Clone Tests ==========

    #[test]
    fn test_clone_message() {
        let message = BusinessAwayMessage::with_data(
            QuickReplyShortcutId::new(123),
            BusinessRecipients::new(),
            BusinessAwayMessageSchedule::Always,
            false,
        );
        let cloned = message.clone();
        assert_eq!(message, cloned);
    }

    #[test]
    fn test_clone_schedule() {
        let schedule = BusinessAwayMessageSchedule::OutsideWorkHours;
        let cloned = schedule.clone();
        assert_eq!(schedule, cloned);
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
        let message = BusinessAwayMessage::with_data(
            QuickReplyShortcutId::new(123),
            BusinessRecipients::new(),
            BusinessAwayMessageSchedule::Always,
            false,
        );
        let display = format!("{}", message);
        assert!(display.contains("123"));
        assert!(display.contains("true")); // is_valid
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-business-away-message");
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
    fn test_schedule_custom_exactly_now() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        let schedule = BusinessAwayMessageSchedule::Custom {
            start_date: now,
            end_date: now,
        };
        assert!(schedule.is_active());
    }
}
