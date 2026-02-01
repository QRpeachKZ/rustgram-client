//! # Rustgram BusinessBotManageBar
//!
//! Business bot manage bar handling for Telegram MTProto client.
//!
//! This crate provides types for managing the business bot control bar
//! that appears in business chats.
//!
//! ## Overview
//!
//! - [`BusinessBotManageBar`] - Business bot management bar configuration
//!
//! ## Examples
//!
//! ```
//! use rustgram_business_bot_manage_bar::BusinessBotManageBar;
//!
//! let bar = BusinessBotManageBar::new();
//! assert!(bar.is_empty());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::fmt;

/// Business bot user ID.
///
/// Represents the user ID of the business bot.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BusinessBotId(i64);

impl Default for BusinessBotId {
    fn default() -> Self {
        Self(0)
    }
}

impl BusinessBotId {
    /// Creates a new business bot ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_manage_bar::BusinessBotId;
    ///
    /// let id = BusinessBotId::new(12345);
    /// assert_eq!(id.get(), 12345);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the raw ID value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_manage_bar::BusinessBotId;
    ///
    /// let id = BusinessBotId::new(12345);
    /// assert_eq!(id.get(), 12345);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get(&self) -> i64 {
        self.0
    }

    /// Checks if this is a valid bot ID (non-zero).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_manage_bar::BusinessBotId;
    ///
    /// assert!(BusinessBotId::new(12345).is_valid());
    /// assert!(!BusinessBotId::new(0).is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.0 != 0
    }
}

/// Business bot manage bar configuration.
///
/// Controls the appearance and behavior of the business bot management bar.
///
/// # Examples
///
/// ```
/// use rustgram_business_bot_manage_bar::{BusinessBotManageBar, BusinessBotId};
///
/// let bar = BusinessBotManageBar::new();
/// assert!(bar.is_empty());
///
/// let bar = BusinessBotManageBar::with_data(
///     BusinessBotId::new(12345),
///     "https://t.me/bot".to_string(),
///     false,
///     true,
/// );
/// assert!(!bar.is_empty());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BusinessBotManageBar {
    /// The business bot user ID
    business_bot_user_id: BusinessBotId,
    /// URL to manage the business bot
    business_bot_manage_url: String,
    /// Whether the business bot is paused
    is_business_bot_paused: bool,
    /// Whether the business bot can reply
    can_business_bot_reply: bool,
}

impl Default for BusinessBotManageBar {
    fn default() -> Self {
        Self::new()
    }
}

impl BusinessBotManageBar {
    /// Creates a new empty manage bar.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_manage_bar::BusinessBotManageBar;
    ///
    /// let bar = BusinessBotManageBar::new();
    /// assert!(bar.is_empty());
    /// assert!(!bar.is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            business_bot_user_id: BusinessBotId::default(),
            business_bot_manage_url: String::new(),
            is_business_bot_paused: false,
            can_business_bot_reply: false,
        }
    }

    /// Creates a manage bar with the given data.
    ///
    /// # Arguments
    ///
    /// * `business_bot_user_id` - The business bot user ID
    /// * `business_bot_manage_url` - URL to manage the bot
    /// * `is_business_bot_paused` - Whether the bot is paused
    /// * `can_business_bot_reply` - Whether the bot can reply
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_manage_bar::{BusinessBotManageBar, BusinessBotId};
    ///
    /// let bar = BusinessBotManageBar::with_data(
    ///     BusinessBotId::new(12345),
    ///     "https://t.me/bot".to_string(),
    ///     false,
    ///     true,
    /// );
    /// assert!(!bar.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn with_data(
        business_bot_user_id: BusinessBotId,
        business_bot_manage_url: String,
        is_business_bot_paused: bool,
        can_business_bot_reply: bool,
    ) -> Self {
        Self {
            business_bot_user_id,
            business_bot_manage_url,
            is_business_bot_paused,
            can_business_bot_reply,
        }
    }

    /// Checks if the manage bar is empty (no bot configured).
    ///
    /// # Returns
    ///
    /// `true` if no valid bot user ID is set
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_manage_bar::BusinessBotManageBar;
    ///
    /// assert!(BusinessBotManageBar::new().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.business_bot_user_id.is_valid()
    }

    /// Checks if the manage bar is valid.
    ///
    /// # Returns
    ///
    /// `true` if a valid bot user ID is set
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_manage_bar::{BusinessBotManageBar, BusinessBotId};
    ///
    /// assert!(!BusinessBotManageBar::new().is_valid());
    ///
    /// let bar = BusinessBotManageBar::with_data(
    ///     BusinessBotId::new(12345),
    ///     "https://t.me/bot".to_string(),
    ///     false,
    ///     true,
    /// );
    /// assert!(bar.is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.business_bot_user_id.is_valid()
    }

    /// Returns the business bot user ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_manage_bar::{BusinessBotManageBar, BusinessBotId};
    ///
    /// let bar = BusinessBotManageBar::with_data(
    ///     BusinessBotId::new(12345),
    ///     String::new(),
    ///     false,
    ///     false,
    /// );
    /// assert_eq!(bar.business_bot_user_id().get(), 12345);
    /// ```
    #[inline]
    #[must_use]
    pub const fn business_bot_user_id(&self) -> BusinessBotId {
        self.business_bot_user_id
    }

    /// Returns the business bot manage URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_manage_bar::{BusinessBotManageBar, BusinessBotId};
    ///
    /// let bar = BusinessBotManageBar::with_data(
    ///     BusinessBotId::new(12345),
    ///     "https://t.me/bot".to_string(),
    ///     false,
    ///     false,
    /// );
    /// assert_eq!(bar.business_bot_manage_url(), "https://t.me/bot");
    /// ```
    #[inline]
    #[must_use]
    pub fn business_bot_manage_url(&self) -> &str {
        &self.business_bot_manage_url
    }

    /// Returns whether the business bot is paused.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_manage_bar::{BusinessBotManageBar, BusinessBotId};
    ///
    /// let bar = BusinessBotManageBar::with_data(
    ///     BusinessBotId::new(12345),
    ///     String::new(),
    ///     true,
    ///     false,
    /// );
    /// assert!(bar.is_business_bot_paused());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_business_bot_paused(&self) -> bool {
        self.is_business_bot_paused
    }

    /// Returns whether the business bot can reply.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_manage_bar::{BusinessBotManageBar, BusinessBotId};
    ///
    /// let bar = BusinessBotManageBar::with_data(
    ///     BusinessBotId::new(12345),
    ///     String::new(),
    ///     false,
    ///     true,
    /// );
    /// assert!(bar.can_business_bot_reply());
    /// ```
    #[inline]
    #[must_use]
    pub const fn can_business_bot_reply(&self) -> bool {
        self.can_business_bot_reply
    }

    /// Sets the business bot paused state.
    ///
    /// # Returns
    ///
    /// `true` if the state was changed
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_manage_bar::{BusinessBotManageBar, BusinessBotId};
    ///
    /// let mut bar = BusinessBotManageBar::with_data(
    ///     BusinessBotId::new(12345),
    ///     String::new(),
    ///     false,
    ///     false,
    /// );
    /// assert!(!bar.is_business_bot_paused());
    ///
    /// bar.set_business_bot_paused(true);
    /// assert!(bar.is_business_bot_paused());
    /// ```
    pub fn set_business_bot_paused(&mut self, is_paused: bool) -> bool {
        if !self.business_bot_user_id.is_valid() || self.is_business_bot_paused == is_paused {
            return false;
        }
        self.is_business_bot_paused = is_paused;
        true
    }

    /// Clears the manage bar (removes bot configuration).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_bot_manage_bar::{BusinessBotManageBar, BusinessBotId};
    ///
    /// let mut bar = BusinessBotManageBar::with_data(
    ///     BusinessBotId::new(12345),
    ///     "https://t.me/bot".to_string(),
    ///     false,
    ///     true,
    /// );
    /// assert!(!bar.is_empty());
    ///
    /// bar.clear();
    /// assert!(bar.is_empty());
    /// ```
    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl fmt::Display for BusinessBotManageBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BusinessBotManageBar {{ bot_id: {}, paused: {}, can_reply: {} }}",
            self.business_bot_user_id.get(),
            self.is_business_bot_paused,
            self.can_business_bot_reply
        )
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-business-bot-manage-bar";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== BusinessBotId Tests ==========

    #[test]
    fn test_bot_id_new() {
        let id = BusinessBotId::new(12345);
        assert_eq!(id.get(), 12345);
    }

    #[test]
    fn test_bot_id_is_valid() {
        assert!(BusinessBotId::new(12345).is_valid());
        assert!(BusinessBotId::new(-1).is_valid());
        assert!(!BusinessBotId::new(0).is_valid());
    }

    #[test]
    fn test_bot_id_default() {
        let id = BusinessBotId::default();
        assert_eq!(id.get(), 0);
        assert!(!id.is_valid());
    }

    // ========== BusinessBotManageBar Constructor Tests ==========

    #[test]
    fn test_new_creates_empty() {
        let bar = BusinessBotManageBar::new();
        assert!(bar.is_empty());
        assert!(!bar.is_valid());
        assert!(!bar.is_business_bot_paused());
        assert!(!bar.can_business_bot_reply());
    }

    #[test]
    fn test_default_creates_empty() {
        let bar = BusinessBotManageBar::default();
        assert!(bar.is_empty());
    }

    #[test]
    fn test_with_data_sets_values() {
        let bar = BusinessBotManageBar::with_data(
            BusinessBotId::new(12345),
            "https://t.me/bot".to_string(),
            true,
            true,
        );

        assert_eq!(bar.business_bot_user_id().get(), 12345);
        assert_eq!(bar.business_bot_manage_url(), "https://t.me/bot");
        assert!(bar.is_business_bot_paused());
        assert!(bar.can_business_bot_reply());
    }

    // ========== is_empty/is_valid Tests ==========

    #[test]
    fn test_is_empty_when_no_bot() {
        let bar = BusinessBotManageBar::new();
        assert!(bar.is_empty());
        assert!(!bar.is_valid());
    }

    #[test]
    fn test_is_empty_with_zero_bot_id() {
        let bar = BusinessBotManageBar::with_data(
            BusinessBotId::new(0),
            "https://t.me/bot".to_string(),
            false,
            false,
        );
        assert!(bar.is_empty());
        assert!(!bar.is_valid());
    }

    #[test]
    fn test_is_valid_with_bot() {
        let bar = BusinessBotManageBar::with_data(
            BusinessBotId::new(12345),
            "https://t.me/bot".to_string(),
            false,
            false,
        );
        assert!(!bar.is_empty());
        assert!(bar.is_valid());
    }

    // ========== Accessor Tests ==========

    #[test]
    fn test_accessors() {
        let bar = BusinessBotManageBar::with_data(
            BusinessBotId::new(12345),
            "https://example.com".to_string(),
            true,
            true,
        );

        assert_eq!(bar.business_bot_user_id().get(), 12345);
        assert_eq!(bar.business_bot_manage_url(), "https://example.com");
        assert!(bar.is_business_bot_paused());
        assert!(bar.can_business_bot_reply());
    }

    // ========== Mutator Tests ==========

    #[test]
    fn test_set_business_bot_paused_changes() {
        let mut bar =
            BusinessBotManageBar::with_data(BusinessBotId::new(12345), String::new(), false, false);

        assert!(!bar.is_business_bot_paused());
        let changed = bar.set_business_bot_paused(true);
        assert!(changed);
        assert!(bar.is_business_bot_paused());
    }

    #[test]
    fn test_set_business_bot_paused_same_value() {
        let mut bar =
            BusinessBotManageBar::with_data(BusinessBotId::new(12345), String::new(), true, false);

        let changed = bar.set_business_bot_paused(true);
        assert!(!changed);
    }

    #[test]
    fn test_set_business_bot_paused_empty_bar() {
        let mut bar = BusinessBotManageBar::new();
        let changed = bar.set_business_bot_paused(true);
        assert!(!changed);
    }

    #[test]
    fn test_clear() {
        let mut bar = BusinessBotManageBar::with_data(
            BusinessBotId::new(12345),
            "https://t.me/bot".to_string(),
            true,
            true,
        );

        assert!(!bar.is_empty());
        bar.clear();
        assert!(bar.is_empty());
    }

    // ========== Equality Tests ==========

    #[test]
    fn test_equality_same_values() {
        let bar1 = BusinessBotManageBar::with_data(
            BusinessBotId::new(12345),
            "https://t.me/bot".to_string(),
            true,
            true,
        );
        let bar2 = BusinessBotManageBar::with_data(
            BusinessBotId::new(12345),
            "https://t.me/bot".to_string(),
            true,
            true,
        );
        assert_eq!(bar1, bar2);
    }

    #[test]
    fn test_inequality_different_bot_id() {
        let bar1 = BusinessBotManageBar::with_data(
            BusinessBotId::new(12345),
            "https://t.me/bot".to_string(),
            true,
            true,
        );
        let bar2 = BusinessBotManageBar::with_data(
            BusinessBotId::new(54321),
            "https://t.me/bot".to_string(),
            true,
            true,
        );
        assert_ne!(bar1, bar2);
    }

    #[test]
    fn test_inequality_different_url() {
        let bar1 = BusinessBotManageBar::with_data(
            BusinessBotId::new(12345),
            "https://t.me/bot1".to_string(),
            false,
            false,
        );
        let bar2 = BusinessBotManageBar::with_data(
            BusinessBotId::new(12345),
            "https://t.me/bot2".to_string(),
            false,
            false,
        );
        assert_ne!(bar1, bar2);
    }

    // ========== Clone Tests ==========

    #[test]
    fn test_clone() {
        let bar1 = BusinessBotManageBar::with_data(
            BusinessBotId::new(12345),
            "https://t.me/bot".to_string(),
            true,
            true,
        );
        let bar2 = bar1.clone();
        assert_eq!(bar1, bar2);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_format() {
        let bar = BusinessBotManageBar::with_data(
            BusinessBotId::new(12345),
            "https://t.me/bot".to_string(),
            true,
            true,
        );
        let s = format!("{}", bar);
        assert!(s.contains("12345"));
        assert!(s.contains("true"));
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-business-bot-manage-bar");
    }
}
