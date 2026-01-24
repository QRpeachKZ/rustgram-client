// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Story Notification Settings
//!
//! Story notification settings for Telegram.
//!
//! ## Overview
//!
//! This module provides the [`StoryNotificationSettings`] struct, which represents
//! notification settings for stories.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_story_notification_settings::StoryNotificationSettings;
//!
//! let settings = StoryNotificationSettings::new(false, false, false, false, 0);
//! assert!(!settings.are_muted());
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Story notification settings.
///
/// Contains settings for how story notifications should be handled.
///
/// Based on TDLib's `StoryNotificationSettings` class.
///
/// # TDLib Alignment
///
/// Aligns with TDLib's `StoryNotificationSettings` class in `StoryNotificationSettings.h`.
///
/// # Example
///
/// ```rust
/// use rustgram_story_notification_settings::StoryNotificationSettings;
///
/// let settings = StoryNotificationSettings::new(false, false, false, false, 0);
/// assert!(!settings.are_muted());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoryNotificationSettings {
    /// Whether to use dialog-specific notification settings.
    need_dialog_settings: bool,

    /// Whether to include in top dialogs.
    need_top_dialogs: bool,

    /// Whether story notifications are muted.
    are_muted: bool,

    /// Whether to hide the sender of story notifications.
    hide_sender: bool,

    /// The ringtone ID to use for story notifications.
    ringtone_id: i64,
}

impl StoryNotificationSettings {
    /// Creates new story notification settings.
    ///
    /// # Arguments
    ///
    /// * `need_dialog_settings` - Whether to use dialog-specific notification settings
    /// * `need_top_dialogs` - Whether to include in top dialogs
    /// * `are_muted` - Whether story notifications are muted
    /// * `hide_sender` - Whether to hide the sender of story notifications
    /// * `ringtone_id` - The ringtone ID to use for story notifications
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_notification_settings::StoryNotificationSettings;
    ///
    /// let settings = StoryNotificationSettings::new(false, false, true, false, 0);
    /// assert!(settings.are_muted());
    /// ```
    #[must_use]
    pub const fn new(
        need_dialog_settings: bool,
        need_top_dialogs: bool,
        are_muted: bool,
        hide_sender: bool,
        ringtone_id: i64,
    ) -> Self {
        Self {
            need_dialog_settings,
            need_top_dialogs,
            are_muted,
            hide_sender,
            ringtone_id,
        }
    }

    /// Returns whether to use dialog-specific notification settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_notification_settings::StoryNotificationSettings;
    ///
    /// let settings = StoryNotificationSettings::new(true, false, true, false, 0);
    /// assert!(settings.need_dialog_settings());
    /// ```
    #[must_use]
    pub const fn need_dialog_settings(&self) -> bool {
        self.need_dialog_settings
    }

    /// Returns whether to include in top dialogs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_notification_settings::StoryNotificationSettings;
    ///
    /// let settings = StoryNotificationSettings::new(false, true, true, false, 0);
    /// assert!(settings.need_top_dialogs());
    /// ```
    #[must_use]
    pub const fn need_top_dialogs(&self) -> bool {
        self.need_top_dialogs
    }

    /// Returns whether story notifications are muted.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_notification_settings::StoryNotificationSettings;
    ///
    /// let settings = StoryNotificationSettings::new(false, false, true, false, 0);
    /// assert!(settings.are_muted());
    /// ```
    #[must_use]
    pub const fn are_muted(&self) -> bool {
        self.are_muted
    }

    /// Returns whether to hide the sender of story notifications.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_notification_settings::StoryNotificationSettings;
    ///
    /// let settings = StoryNotificationSettings::new(false, false, true, true, 0);
    /// assert!(settings.hide_sender());
    /// ```
    #[must_use]
    pub const fn hide_sender(&self) -> bool {
        self.hide_sender
    }

    /// Returns the ringtone ID to use for story notifications.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_notification_settings::StoryNotificationSettings;
    ///
    /// let settings = StoryNotificationSettings::new(false, false, true, false, 12345);
    /// assert_eq!(settings.ringtone_id(), 12345);
    /// ```
    #[must_use]
    pub const fn ringtone_id(&self) -> i64 {
        self.ringtone_id
    }
}

impl fmt::Display for StoryNotificationSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StoryNotificationSettings {{ muted: {}, hide_sender: {}, ringtone: {} }}",
            self.are_muted, self.hide_sender, self.ringtone_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== new Tests ==========

    #[test]
    fn test_new() {
        let settings = StoryNotificationSettings::new(false, false, true, false, 0);
        assert!(settings.are_muted());
        assert!(!settings.hide_sender());
    }

    // ========== need_dialog_settings Tests ==========

    #[test]
    fn test_need_dialog_settings_true() {
        let settings = StoryNotificationSettings::new(true, false, true, false, 0);
        assert!(settings.need_dialog_settings());
    }

    #[test]
    fn test_need_dialog_settings_false() {
        let settings = StoryNotificationSettings::new(false, false, true, false, 0);
        assert!(!settings.need_dialog_settings());
    }

    // ========== need_top_dialogs Tests ==========

    #[test]
    fn test_need_top_dialogs_true() {
        let settings = StoryNotificationSettings::new(false, true, true, false, 0);
        assert!(settings.need_top_dialogs());
    }

    #[test]
    fn test_need_top_dialogs_false() {
        let settings = StoryNotificationSettings::new(false, false, true, false, 0);
        assert!(!settings.need_top_dialogs());
    }

    // ========== are_muted Tests ==========

    #[test]
    fn test_are_muted_true() {
        let settings = StoryNotificationSettings::new(false, false, true, false, 0);
        assert!(settings.are_muted());
    }

    #[test]
    fn test_are_muted_false() {
        let settings = StoryNotificationSettings::new(false, false, false, false, 0);
        assert!(!settings.are_muted());
    }

    // ========== hide_sender Tests ==========

    #[test]
    fn test_hide_sender_true() {
        let settings = StoryNotificationSettings::new(false, false, true, true, 0);
        assert!(settings.hide_sender());
    }

    #[test]
    fn test_hide_sender_false() {
        let settings = StoryNotificationSettings::new(false, false, true, false, 0);
        assert!(!settings.hide_sender());
    }

    // ========== ringtone_id Tests ==========

    #[test]
    fn test_ringtone_id_zero() {
        let settings = StoryNotificationSettings::new(false, false, true, false, 0);
        assert_eq!(settings.ringtone_id(), 0);
    }

    #[test]
    fn test_ringtone_id_positive() {
        let settings = StoryNotificationSettings::new(false, false, true, false, 12345);
        assert_eq!(settings.ringtone_id(), 12345);
    }

    #[test]
    fn test_ringtone_id_negative() {
        let settings = StoryNotificationSettings::new(false, false, true, false, -999);
        assert_eq!(settings.ringtone_id(), -999);
    }

    // ========== default Tests ==========

    #[test]
    fn test_default() {
        let settings = StoryNotificationSettings::default();
        assert!(!settings.need_dialog_settings());
        assert!(!settings.need_top_dialogs());
        assert!(!settings.are_muted());
        assert!(!settings.hide_sender());
        assert_eq!(settings.ringtone_id(), 0);
    }

    // ========== equality Tests ==========

    #[test]
    fn test_equality_same() {
        let settings1 = StoryNotificationSettings::new(true, false, true, false, 123);
        let settings2 = StoryNotificationSettings::new(true, false, true, false, 123);
        assert_eq!(settings1, settings2);
    }

    #[test]
    fn test_equality_different() {
        let settings1 = StoryNotificationSettings::new(true, false, true, false, 123);
        let settings2 = StoryNotificationSettings::new(false, false, true, false, 123);
        assert_ne!(settings1, settings2);
    }

    // ========== clone Tests ==========

    #[test]
    fn test_clone() {
        let settings1 = StoryNotificationSettings::new(true, false, true, false, 123);
        let settings2 = settings1;
        assert_eq!(settings1, settings2);
    }

    // ========== display Tests ==========

    #[test]
    fn test_display() {
        let settings = StoryNotificationSettings::new(true, false, true, false, 123);
        let display = format!("{}", settings);
        assert!(display.contains("StoryNotificationSettings"));
        assert!(display.contains("muted"));
    }

    #[test]
    fn test_debug() {
        let settings = StoryNotificationSettings::new(true, false, true, false, 123);
        let debug_str = format!("{:?}", settings);
        assert!(debug_str.contains("StoryNotificationSettings"));
    }

    // ========== all_combinations Tests ==========

    #[test]
    fn test_all_false() {
        let settings = StoryNotificationSettings::new(false, false, false, false, 0);
        assert!(!settings.need_dialog_settings());
        assert!(!settings.need_top_dialogs());
        assert!(!settings.are_muted());
        assert!(!settings.hide_sender());
        assert_eq!(settings.ringtone_id(), 0);
    }

    #[test]
    fn test_all_true() {
        let settings = StoryNotificationSettings::new(true, true, true, true, 999);
        assert!(settings.need_dialog_settings());
        assert!(settings.need_top_dialogs());
        assert!(settings.are_muted());
        assert!(settings.hide_sender());
        assert_eq!(settings.ringtone_id(), 999);
    }

    // ========== Serialization Tests ==========

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let settings = StoryNotificationSettings::new(true, false, true, false, 123);
        let json = serde_json::to_string(&settings).unwrap();
        let deserialized: StoryNotificationSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(settings, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_all_values() {
        let settings = [
            StoryNotificationSettings::new(false, false, false, false, 0),
            StoryNotificationSettings::new(true, false, true, false, 123),
            StoryNotificationSettings::new(true, true, true, true, 999),
        ];
        for settings in &settings {
            let json = serde_json::to_string(&settings).unwrap();
            let deserialized: StoryNotificationSettings = serde_json::from_str(&json).unwrap();
            assert_eq!(settings, &deserialized);
        }
    }
}
