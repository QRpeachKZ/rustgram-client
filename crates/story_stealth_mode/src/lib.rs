//! # Rustgram StoryStealthMode
//!
//! Story stealth mode tracking for Telegram MTProto client.
//!
//! This crate provides types for tracking the stealth mode state of stories
//! in Telegram. Stealth mode allows users to view stories anonymously.
//!
//! ## Overview
//!
//! - [`StoryStealthMode`] - Tracks active and cooldown dates for stealth mode
//!
//! ## Stealth Mode
//!
//! Stealth mode has two states tracked by timestamps:
//!
//! - **Active period**: When stealth mode is currently active (stories viewed anonymously)
//! - **Cooldown period**: When stealth mode is on cooldown (must wait before reactivating)
//!
//! ## Examples
//!
//! Basic stealth mode usage:
//!
//! ```
//! use rustgram_story_stealth_mode::StoryStealthMode;
//!
//! let mode = StoryStealthMode::new();
//! assert!(mode.is_empty());
//! assert_eq!(mode.get_update_date(), 0);
//!
//! let mode_with_dates = StoryStealthMode::with_dates(1234567890, 1234567900);
//! assert!(!mode_with_dates.is_empty());
//! assert_eq!(mode_with_dates.get_update_date(), 1234567900);
//! ```
//!
//! Updating stealth mode (simulating time passage):
//!
//! ```
//! use rustgram_story_stealth_mode::StoryStealthMode;
//!
//! let mut mode = StoryStealthMode::with_dates(100, 200);
//! assert_eq!(mode.get_update_date(), 200);
//!
//! // Update decrements dates by 1 and returns true if changed
//! assert!(mode.update());
//! assert_eq!(mode.get_update_date(), 199);
//!
//! // Once dates reach 0, update returns false
//! while mode.update() {}
//! assert!(mode.is_empty());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Story stealth mode tracking.
///
/// Tracks the active and cooldown timestamps for story stealth mode.
/// When stealth mode is active, stories can be viewed anonymously.
///
/// # Examples
///
/// ```
/// use rustgram_story_stealth_mode::StoryStealthMode;
///
/// let mode = StoryStealthMode::new();
/// assert!(mode.is_empty());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StoryStealthMode {
    /// Unix timestamp when stealth mode becomes inactive
    active_until_date: i32,
    /// Unix timestamp when cooldown period ends
    cooldown_until_date: i32,
}

impl Default for StoryStealthMode {
    fn default() -> Self {
        Self::new()
    }
}

impl StoryStealthMode {
    /// Creates a new empty stealth mode.
    ///
    /// Both dates are initialized to 0, indicating no active stealth mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_stealth_mode::StoryStealthMode;
    ///
    /// let mode = StoryStealthMode::new();
    /// assert!(mode.is_empty());
    /// assert_eq!(mode.active_until_date(), 0);
    /// assert_eq!(mode.cooldown_until_date(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            active_until_date: 0,
            cooldown_until_date: 0,
        }
    }

    /// Creates a stealth mode with specific dates.
    ///
    /// # Arguments
    ///
    /// * `active_until_date` - Unix timestamp when stealth mode becomes inactive
    /// * `cooldown_until_date` - Unix timestamp when cooldown period ends
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_stealth_mode::StoryStealthMode;
    ///
    /// let mode = StoryStealthMode::with_dates(100, 200);
    /// assert!(!mode.is_empty());
    /// assert_eq!(mode.active_until_date(), 100);
    /// assert_eq!(mode.cooldown_until_date(), 200);
    /// ```
    #[inline]
    #[must_use]
    pub const fn with_dates(active_until_date: i32, cooldown_until_date: i32) -> Self {
        Self {
            active_until_date,
            cooldown_until_date,
        }
    }

    /// Checks if stealth mode is empty (both dates are zero).
    ///
    /// # Returns
    ///
    /// `true` if both dates are 0, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_stealth_mode::StoryStealthMode;
    ///
    /// assert!(StoryStealthMode::new().is_empty());
    /// assert!(!StoryStealthMode::with_dates(1, 0).is_empty());
    /// assert!(!StoryStealthMode::with_dates(0, 1).is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.active_until_date == 0 && self.cooldown_until_date == 0
    }

    /// Returns the active until date.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_stealth_mode::StoryStealthMode;
    ///
    /// let mode = StoryStealthMode::with_dates(100, 200);
    /// assert_eq!(mode.active_until_date(), 100);
    /// ```
    #[inline]
    #[must_use]
    pub const fn active_until_date(&self) -> i32 {
        self.active_until_date
    }

    /// Returns the cooldown until date.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_stealth_mode::StoryStealthMode;
    ///
    /// let mode = StoryStealthMode::with_dates(100, 200);
    /// assert_eq!(mode.cooldown_until_date(), 200);
    /// ```
    #[inline]
    #[must_use]
    pub const fn cooldown_until_date(&self) -> i32 {
        self.cooldown_until_date
    }

    /// Gets the update date (maximum of active and cooldown dates).
    ///
    /// This is useful for determining when the next state change occurs.
    ///
    /// # Returns
    ///
    /// The maximum of `active_until_date` and `cooldown_until_date`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_stealth_mode::StoryStealthMode;
    ///
    /// let mode = StoryStealthMode::with_dates(100, 200);
    /// assert_eq!(mode.get_update_date(), 200);
    ///
    /// let mode2 = StoryStealthMode::with_dates(300, 200);
    /// assert_eq!(mode2.get_update_date(), 300);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_update_date(&self) -> i32 {
        self.active_until_date.max(self.cooldown_until_date)
    }

    /// Updates the stealth mode by decrementing dates.
    ///
    /// Decrements both dates by 1 (clamped to 0). This simulates the passage
    /// of time and is typically called once per second.
    ///
    /// # Returns
    ///
    /// `true` if either date was changed, `false` if both were already 0
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_stealth_mode::StoryStealthMode;
    ///
    /// let mut mode = StoryStealthMode::with_dates(100, 200);
    ///
    /// assert!(mode.update());
    /// assert_eq!(mode.active_until_date(), 99);
    /// assert_eq!(mode.cooldown_until_date(), 199);
    ///
    /// // Create a new mode with all dates 0
    /// let mut empty_mode = StoryStealthMode::new();
    /// assert!(!empty_mode.update());
    /// ```
    pub fn update(&mut self) -> bool {
        let mut changed = false;

        if self.active_until_date > 0 {
            self.active_until_date -= 1;
            changed = true;
        }

        if self.cooldown_until_date > 0 {
            self.cooldown_until_date -= 1;
            changed = true;
        }

        changed
    }

    /// Checks if stealth mode is currently active.
    ///
    /// Uses the current system time to determine if stealth mode is active.
    ///
    /// # Returns
    ///
    /// `true` if current time is before `active_until_date`, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_stealth_mode::StoryStealthMode;
    /// use std::time::{SystemTime, UNIX_EPOCH};
    ///
    /// let now = SystemTime::now()
    ///     .duration_since(UNIX_EPOCH)
    ///     .unwrap()
    ///     .as_secs() as i32;
    ///
    /// // Active for 60 seconds from now
    /// let mode = StoryStealthMode::with_dates(now + 60, now + 120);
    /// assert!(mode.is_active());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.current_time() < self.active_until_date
    }

    /// Checks if stealth mode is on cooldown.
    ///
    /// # Returns
    ///
    /// `true` if current time is before `cooldown_until_date`, `false` otherwise
    #[inline]
    #[must_use]
    pub fn is_on_cooldown(&self) -> bool {
        self.current_time() < self.cooldown_until_date
    }

    #[inline]
    fn current_time(&self) -> i32 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0)
    }
}

impl fmt::Display for StoryStealthMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StoryStealthMode {{ active_until: {}, cooldown_until: {} }}",
            self.active_until_date, self.cooldown_until_date
        )
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-story-stealth-mode";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Constructor Tests ==========

    #[test]
    fn test_new_creates_empty_mode() {
        let mode = StoryStealthMode::new();
        assert_eq!(mode.active_until_date(), 0);
        assert_eq!(mode.cooldown_until_date(), 0);
        assert!(mode.is_empty());
    }

    #[test]
    fn test_default_creates_empty_mode() {
        let mode = StoryStealthMode::default();
        assert!(mode.is_empty());
    }

    #[test]
    fn test_with_dates_sets_values() {
        let mode = StoryStealthMode::with_dates(100, 200);
        assert_eq!(mode.active_until_date(), 100);
        assert_eq!(mode.cooldown_until_date(), 200);
    }

    #[test]
    fn test_with_dates_negative_values() {
        let mode = StoryStealthMode::with_dates(-1, -2);
        assert_eq!(mode.active_until_date(), -1);
        assert_eq!(mode.cooldown_until_date(), -2);
    }

    // ========== is_empty Tests ==========

    #[test]
    fn test_is_empty_when_both_zero() {
        let mode = StoryStealthMode::new();
        assert!(mode.is_empty());
    }

    #[test]
    fn test_is_empty_when_active_nonzero() {
        let mode = StoryStealthMode::with_dates(1, 0);
        assert!(!mode.is_empty());
    }

    #[test]
    fn test_is_empty_when_cooldown_nonzero() {
        let mode = StoryStealthMode::with_dates(0, 1);
        assert!(!mode.is_empty());
    }

    #[test]
    fn test_is_empty_when_both_nonzero() {
        let mode = StoryStealthMode::with_dates(1, 1);
        assert!(!mode.is_empty());
    }

    // ========== get_update_date Tests ==========

    #[test]
    fn test_get_update_date_when_empty() {
        let mode = StoryStealthMode::new();
        assert_eq!(mode.get_update_date(), 0);
    }

    #[test]
    fn test_get_update_date_returns_max() {
        let mode = StoryStealthMode::with_dates(100, 200);
        assert_eq!(mode.get_update_date(), 200);

        let mode2 = StoryStealthMode::with_dates(300, 200);
        assert_eq!(mode2.get_update_date(), 300);
    }

    #[test]
    fn test_get_update_date_equal_values() {
        let mode = StoryStealthMode::with_dates(100, 100);
        assert_eq!(mode.get_update_date(), 100);
    }

    // ========== update Tests ==========

    #[test]
    fn test_update_decrements_both_dates() {
        let mut mode = StoryStealthMode::with_dates(10, 20);
        assert!(mode.update());
        assert_eq!(mode.active_until_date(), 9);
        assert_eq!(mode.cooldown_until_date(), 19);
    }

    #[test]
    fn test_update_clamps_to_zero() {
        let mut mode = StoryStealthMode::with_dates(1, 1);
        assert!(mode.update());
        assert_eq!(mode.active_until_date(), 0);
        assert_eq!(mode.cooldown_until_date(), 0);
    }

    #[test]
    fn test_update_returns_false_when_empty() {
        let mut mode = StoryStealthMode::new();
        assert!(!mode.update());
    }

    #[test]
    fn test_update_returns_false_after_depletion() {
        let mut mode = StoryStealthMode::with_dates(1, 0);
        assert!(mode.update());
        assert_eq!(mode.active_until_date(), 0);
        assert!(!mode.update());
    }

    #[test]
    fn test_update_multiple_times() {
        let mut mode = StoryStealthMode::with_dates(3, 5);

        assert!(mode.update());
        assert_eq!(mode.active_until_date(), 2);
        assert_eq!(mode.cooldown_until_date(), 4);

        assert!(mode.update());
        assert_eq!(mode.active_until_date(), 1);
        assert_eq!(mode.cooldown_until_date(), 3);

        assert!(mode.update());
        assert_eq!(mode.active_until_date(), 0);
        assert_eq!(mode.cooldown_until_date(), 2);

        // Continue updating cooldown
        assert!(mode.update());
        assert_eq!(mode.active_until_date(), 0);
        assert_eq!(mode.cooldown_until_date(), 1);
    }

    // ========== is_active Tests ==========

    #[test]
    fn test_is_active_when_future_date() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32;
        let mode = StoryStealthMode::with_dates(now + 100, now + 200);
        assert!(mode.is_active());
    }

    #[test]
    fn test_is_active_when_past_date() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32;
        let mode = StoryStealthMode::with_dates(now - 100, now + 200);
        assert!(!mode.is_active());
    }

    #[test]
    fn test_is_active_when_zero() {
        let mode = StoryStealthMode::new();
        assert!(!mode.is_active());
    }

    // ========== is_on_cooldown Tests ==========

    #[test]
    fn test_is_on_cooldown_when_future_date() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32;
        let mode = StoryStealthMode::with_dates(now - 100, now + 200);
        assert!(mode.is_on_cooldown());
    }

    #[test]
    fn test_is_on_cooldown_when_past_date() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i32;
        let mode = StoryStealthMode::with_dates(now + 100, now - 200);
        assert!(!mode.is_on_cooldown());
    }

    // ========== Equality Tests ==========

    #[test]
    fn test_equality_same_values() {
        let mode1 = StoryStealthMode::with_dates(100, 200);
        let mode2 = StoryStealthMode::with_dates(100, 200);
        assert_eq!(mode1, mode2);
    }

    #[test]
    fn test_equality_different_values() {
        let mode1 = StoryStealthMode::with_dates(100, 200);
        let mode2 = StoryStealthMode::with_dates(200, 100);
        assert_ne!(mode1, mode2);
    }

    #[test]
    fn test_clone() {
        let mode = StoryStealthMode::with_dates(100, 200);
        let cloned = mode.clone();
        assert_eq!(mode, cloned);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_format() {
        let mode = StoryStealthMode::with_dates(100, 200);
        let display = format!("{}", mode);
        assert!(display.contains("100"));
        assert!(display.contains("200"));
        assert!(display.contains("StoryStealthMode"));
    }

    #[test]
    fn test_debug_format() {
        let mode = StoryStealthMode::with_dates(100, 200);
        let debug = format!("{:?}", mode);
        assert!(debug.contains("100"));
        assert!(debug.contains("200"));
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-story-stealth-mode");
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_with_dates_both_negative() {
        let mode = StoryStealthMode::with_dates(-100, -200);
        assert_eq!(mode.active_until_date(), -100);
        assert_eq!(mode.cooldown_until_date(), -200);
        assert!(!mode.is_empty());
    }

    #[test]
    fn test_get_update_date_with_negative() {
        let mode = StoryStealthMode::with_dates(-100, 200);
        assert_eq!(mode.get_update_date(), 200);

        let mode2 = StoryStealthMode::with_dates(-200, -100);
        assert_eq!(mode2.get_update_date(), -100);
    }

    #[test]
    fn test_update_with_negative_values() {
        let mut mode = StoryStealthMode::with_dates(-1, 10);
        // Negative values don't get decremented (condition > 0)
        assert!(mode.update());
        assert_eq!(mode.active_until_date(), -1);
        assert_eq!(mode.cooldown_until_date(), 9);
    }

    #[test]
    fn test_update_only_active() {
        let mut mode = StoryStealthMode::with_dates(5, 0);
        assert!(mode.update());
        assert_eq!(mode.active_until_date(), 4);
        assert_eq!(mode.cooldown_until_date(), 0);
    }

    #[test]
    fn test_update_only_cooldown() {
        let mut mode = StoryStealthMode::with_dates(0, 5);
        assert!(mode.update());
        assert_eq!(mode.active_until_date(), 0);
        assert_eq!(mode.cooldown_until_date(), 4);
    }

    #[test]
    fn test_is_active_handles_large_dates() {
        let mode = StoryStealthMode::with_dates(i32::MAX, i32::MAX);
        assert!(mode.is_active());
    }
}
