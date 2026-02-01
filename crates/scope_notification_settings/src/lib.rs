// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! Notification settings for scopes in Telegram.
//!
//! This module provides the [`ScopeNotificationSettings`] struct, which
//! represents notification preferences for different scopes (private chats,
//! groups, and channels).
//!
//! ## Overview
//!
//! Telegram allows users to customize notification settings for different
//! types of chats:
//!
//! - **Private chats**: One-on-one conversations with users
//! - **Group chats**: Group conversations
//! - **Channels**: Broadcast channels
//!
//! The [`ScopeNotificationSettings`] struct captures all notification
//! preferences for a specific scope.
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_scope_notification_settings::ScopeNotificationSettings;
//!
//! // Create default settings
//! let settings = ScopeNotificationSettings::default();
//! assert_eq!(settings.mute_until(), 0);
//! assert!(settings.show_preview());
//! ```
//!
//! ## TDLib Alignment
//!
//! This struct aligns with TDLib's `ScopeNotificationSettings` class:
//! - Field names match TDLib exactly
//! - All notification settings are supported
//! - NotificationSound is stubbed (full implementation pending)

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::fmt;

/// Stub for NotificationSound.
///
/// TODO: Full implementation when rustgram-notification-sound is available.
///
/// This stub provides the minimal structure needed for
/// ScopeNotificationSettings. The full implementation would contain
/// sound ID and duration information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationSound {
    /// Sound identifier.
    pub id: String,
}

impl NotificationSound {
    /// Create a new NotificationSound stub.
    #[must_use]
    pub fn new(id: String) -> Self {
        Self { id }
    }

    /// Get the sound ID.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Default for NotificationSound {
    fn default() -> Self {
        Self::new(String::from("default"))
    }
}

/// Builder for constructing [`ScopeNotificationSettings`].
///
/// Provides a fluent interface for creating notification settings with
/// all optional parameters.
///
/// # Example
///
/// ```rust
/// use rustgram_scope_notification_settings::{ScopeNotificationSettings, NotificationSound};
///
/// let sound = NotificationSound::new(String::from("custom"));
/// let settings = ScopeNotificationSettings::builder()
///     .with_mute_until(1234567890)
///     .with_sound(Some(sound))
///     .with_show_preview(false)
///     .build();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeNotificationSettingsBuilder {
    /// Timestamp until which notifications are muted.
    mute_until: i32,
    /// Notification sound setting.
    sound: Option<NotificationSound>,
    /// Story notification sound setting.
    story_sound: Option<NotificationSound>,
    /// Whether to show message preview.
    show_preview: bool,
    /// Whether to use default mute stories setting.
    use_default_mute_stories: bool,
    /// Whether to mute story notifications.
    mute_stories: bool,
    /// Whether to hide story sender.
    hide_story_sender: bool,
    /// Whether settings are synchronized with server.
    is_synchronized: bool,
    /// Local: disable pinned message notifications.
    disable_pinned_message_notifications: bool,
    /// Local: disable mention notifications.
    disable_mention_notifications: bool,
}

impl Default for ScopeNotificationSettingsBuilder {
    fn default() -> Self {
        Self {
            mute_until: 0,
            sound: None,
            story_sound: None,
            show_preview: true,
            use_default_mute_stories: true,
            mute_stories: false,
            hide_story_sender: false,
            is_synchronized: false,
            disable_pinned_message_notifications: false,
            disable_mention_notifications: false,
        }
    }
}

impl ScopeNotificationSettingsBuilder {
    /// Creates a new builder with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the mute_until timestamp.
    #[must_use]
    pub const fn with_mute_until(mut self, mute_until: i32) -> Self {
        self.mute_until = mute_until;
        self
    }

    /// Sets the notification sound.
    #[must_use]
    pub fn with_sound(mut self, sound: Option<NotificationSound>) -> Self {
        self.sound = sound;
        self
    }

    /// Sets the story notification sound.
    #[must_use]
    pub fn with_story_sound(mut self, story_sound: Option<NotificationSound>) -> Self {
        self.story_sound = story_sound;
        self
    }

    /// Sets whether to show message preview.
    #[must_use]
    pub const fn with_show_preview(mut self, show_preview: bool) -> Self {
        self.show_preview = show_preview;
        self
    }

    /// Sets whether to use default mute stories setting.
    #[must_use]
    pub const fn with_use_default_mute_stories(mut self, use_default: bool) -> Self {
        self.use_default_mute_stories = use_default;
        self
    }

    /// Sets whether to mute story notifications.
    #[must_use]
    pub const fn with_mute_stories(mut self, mute_stories: bool) -> Self {
        self.mute_stories = mute_stories;
        self
    }

    /// Sets whether to hide story sender.
    #[must_use]
    pub const fn with_hide_story_sender(mut self, hide: bool) -> Self {
        self.hide_story_sender = hide;
        self
    }

    /// Sets whether settings are synchronized with server.
    #[must_use]
    pub const fn with_synchronized(mut self, is_synchronized: bool) -> Self {
        self.is_synchronized = is_synchronized;
        self
    }

    /// Sets whether to disable pinned message notifications.
    #[must_use]
    pub const fn with_disable_pinned_message_notifications(mut self, disable: bool) -> Self {
        self.disable_pinned_message_notifications = disable;
        self
    }

    /// Sets whether to disable mention notifications.
    #[must_use]
    pub const fn with_disable_mention_notifications(mut self, disable: bool) -> Self {
        self.disable_mention_notifications = disable;
        self
    }

    /// Builds the settings.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // Intentional for potential future complexity
    pub fn build(self) -> ScopeNotificationSettings {
        ScopeNotificationSettings {
            mute_until: self.mute_until,
            sound: self.sound,
            story_sound: self.story_sound,
            show_preview: self.show_preview,
            use_default_mute_stories: self.use_default_mute_stories,
            mute_stories: self.mute_stories,
            hide_story_sender: self.hide_story_sender,
            is_synchronized: self.is_synchronized,
            disable_pinned_message_notifications: self.disable_pinned_message_notifications,
            disable_mention_notifications: self.disable_mention_notifications,
        }
    }
}

/// Notification settings for a scope (private, group, channel).
///
/// Contains notification preferences including mute status, sound settings,
/// story notification settings, and local overrides.
///
/// # Example
///
/// ```rust
/// use rustgram_scope_notification_settings::ScopeNotificationSettings;
///
/// // Create default settings
/// let settings = ScopeNotificationSettings::new();
/// assert_eq!(settings.mute_until(), 0);
/// assert!(settings.show_preview());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeNotificationSettings {
    /// Timestamp until which notifications are muted.
    mute_until: i32,
    /// Notification sound setting.
    sound: Option<NotificationSound>,
    /// Story notification sound setting.
    story_sound: Option<NotificationSound>,
    /// Whether to show message preview.
    show_preview: bool,
    /// Whether to use default mute stories setting.
    use_default_mute_stories: bool,
    /// Whether to mute story notifications.
    mute_stories: bool,
    /// Whether to hide story sender.
    hide_story_sender: bool,
    /// Whether settings are synchronized with server.
    is_synchronized: bool,
    /// Local: disable pinned message notifications.
    disable_pinned_message_notifications: bool,
    /// Local: disable mention notifications.
    disable_mention_notifications: bool,
}

impl ScopeNotificationSettings {
    /// Creates a builder for constructing notification settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scope_notification_settings::ScopeNotificationSettings;
    ///
    /// let settings = ScopeNotificationSettings::builder()
    ///     .with_mute_until(1234567890)
    ///     .with_show_preview(false)
    ///     .build();
    /// assert_eq!(settings.mute_until(), 1234567890);
    /// ```
    #[must_use]
    pub fn builder() -> ScopeNotificationSettingsBuilder {
        ScopeNotificationSettingsBuilder::new()
    }

    /// Creates new notification settings with specified values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scope_notification_settings::{ScopeNotificationSettings, NotificationSound};
    ///
    /// let sound = NotificationSound::new(String::from("custom"));
    /// let settings = ScopeNotificationSettings::builder()
    ///     .with_mute_until(0)
    ///     .with_sound(Some(sound.clone()))
    ///     .with_show_preview(true)
    ///     .with_use_default_mute_stories(true)
    ///     .with_mute_stories(false)
    ///     .with_hide_story_sender(false)
    ///     .with_disable_pinned_message_notifications(false)
    ///     .with_disable_mention_notifications(false)
    ///     .build();
    /// assert!(settings.sound().is_some());
    /// ```
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        mute_until: i32,
        sound: Option<NotificationSound>,
        story_sound: Option<NotificationSound>,
        show_preview: bool,
        use_default_mute_stories: bool,
        mute_stories: bool,
        hide_story_sender: bool,
        disable_pinned_message_notifications: bool,
        disable_mention_notifications: bool,
    ) -> Self {
        Self {
            mute_until,
            sound,
            story_sound,
            show_preview,
            use_default_mute_stories,
            mute_stories,
            hide_story_sender,
            is_synchronized: true,
            disable_pinned_message_notifications,
            disable_mention_notifications,
        }
    }

    /// Creates default notification settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scope_notification_settings::ScopeNotificationSettings;
    ///
    /// let settings = ScopeNotificationSettings::defaults();
    /// assert_eq!(settings.mute_until(), 0);
    /// assert!(settings.show_preview());
    /// assert!(!settings.disable_pinned_message_notifications());
    /// ```
    #[must_use]
    pub fn defaults() -> Self {
        Self {
            mute_until: 0,
            sound: None,
            story_sound: None,
            show_preview: true,
            use_default_mute_stories: true,
            mute_stories: false,
            hide_story_sender: false,
            is_synchronized: false,
            disable_pinned_message_notifications: false,
            disable_mention_notifications: false,
        }
    }

    /// Marks settings as synchronized with the server.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scope_notification_settings::ScopeNotificationSettings;
    ///
    /// let settings = ScopeNotificationSettings::new()
    ///     .with_synchronized(true);
    /// assert!(settings.is_synchronized());
    /// ```
    #[must_use]
    pub const fn with_synchronized(mut self, is_synchronized: bool) -> Self {
        self.is_synchronized = is_synchronized;
        self
    }

    /// Sets the mute_until timestamp.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scope_notification_settings::ScopeNotificationSettings;
    ///
    /// let settings = ScopeNotificationSettings::new()
    ///     .with_mute_until(1234567890);
    /// assert_eq!(settings.mute_until(), 1234567890);
    /// ```
    #[must_use]
    pub const fn with_mute_until(mut self, mute_until: i32) -> Self {
        self.mute_until = mute_until;
        self
    }

    /// Sets the notification sound.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scope_notification_settings::{ScopeNotificationSettings, NotificationSound};
    ///
    /// let sound = NotificationSound::new(String::from("custom"));
    /// let settings = ScopeNotificationSettings::new()
    ///     .with_sound(Some(sound));
    /// assert!(settings.sound().is_some());
    /// ```
    #[must_use]
    pub fn with_sound(mut self, sound: Option<NotificationSound>) -> Self {
        self.sound = sound;
        self
    }

    /// Sets the story notification sound.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scope_notification_settings::{ScopeNotificationSettings, NotificationSound};
    ///
    /// let sound = NotificationSound::new(String::from("story"));
    /// let settings = ScopeNotificationSettings::new()
    ///     .with_story_sound(Some(sound));
    /// assert!(settings.story_sound().is_some());
    /// ```
    #[must_use]
    pub fn with_story_sound(mut self, story_sound: Option<NotificationSound>) -> Self {
        self.story_sound = story_sound;
        self
    }

    /// Sets whether to show message preview.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scope_notification_settings::ScopeNotificationSettings;
    ///
    /// let settings = ScopeNotificationSettings::new()
    ///     .with_show_preview(false);
    /// assert!(!settings.show_preview());
    /// ```
    #[must_use]
    pub const fn with_show_preview(mut self, show_preview: bool) -> Self {
        self.show_preview = show_preview;
        self
    }

    /// Sets whether to use default mute stories setting.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scope_notification_settings::ScopeNotificationSettings;
    ///
    /// let settings = ScopeNotificationSettings::new()
    ///     .with_use_default_mute_stories(false);
    /// assert!(!settings.use_default_mute_stories());
    /// ```
    #[must_use]
    pub const fn with_use_default_mute_stories(mut self, use_default: bool) -> Self {
        self.use_default_mute_stories = use_default;
        self
    }

    /// Sets whether to mute story notifications.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scope_notification_settings::ScopeNotificationSettings;
    ///
    /// let settings = ScopeNotificationSettings::new()
    ///     .with_mute_stories(true);
    /// assert!(settings.mute_stories());
    /// ```
    #[must_use]
    pub const fn with_mute_stories(mut self, mute_stories: bool) -> Self {
        self.mute_stories = mute_stories;
        self
    }

    /// Sets whether to hide story sender.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scope_notification_settings::ScopeNotificationSettings;
    ///
    /// let settings = ScopeNotificationSettings::new()
    ///     .with_hide_story_sender(true);
    /// assert!(settings.hide_story_sender());
    /// ```
    #[must_use]
    pub const fn with_hide_story_sender(mut self, hide: bool) -> Self {
        self.hide_story_sender = hide;
        self
    }

    /// Sets whether to disable pinned message notifications.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scope_notification_settings::ScopeNotificationSettings;
    ///
    /// let settings = ScopeNotificationSettings::new()
    ///     .with_disable_pinned_message_notifications(true);
    /// assert!(settings.disable_pinned_message_notifications());
    /// ```
    #[must_use]
    pub const fn with_disable_pinned_message_notifications(mut self, disable: bool) -> Self {
        self.disable_pinned_message_notifications = disable;
        self
    }

    /// Sets whether to disable mention notifications.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scope_notification_settings::ScopeNotificationSettings;
    ///
    /// let settings = ScopeNotificationSettings::new()
    ///     .with_disable_mention_notifications(true);
    /// assert!(settings.disable_mention_notifications());
    /// ```
    #[must_use]
    pub const fn with_disable_mention_notifications(mut self, disable: bool) -> Self {
        self.disable_mention_notifications = disable;
        self
    }

    /// Returns the mute_until timestamp.
    #[must_use]
    pub const fn mute_until(&self) -> i32 {
        self.mute_until
    }

    /// Returns the notification sound setting.
    #[must_use]
    pub const fn sound(&self) -> Option<&NotificationSound> {
        self.sound.as_ref()
    }

    /// Returns the story notification sound setting.
    #[must_use]
    pub const fn story_sound(&self) -> Option<&NotificationSound> {
        self.story_sound.as_ref()
    }

    /// Returns whether to show message preview.
    #[must_use]
    pub const fn show_preview(&self) -> bool {
        self.show_preview
    }

    /// Returns whether to use default mute stories setting.
    #[must_use]
    pub const fn use_default_mute_stories(&self) -> bool {
        self.use_default_mute_stories
    }

    /// Returns whether to mute story notifications.
    #[must_use]
    pub const fn mute_stories(&self) -> bool {
        self.mute_stories
    }

    /// Returns whether to hide story sender.
    #[must_use]
    pub const fn hide_story_sender(&self) -> bool {
        self.hide_story_sender
    }

    /// Returns whether settings are synchronized with server.
    #[must_use]
    pub const fn is_synchronized(&self) -> bool {
        self.is_synchronized
    }

    /// Returns whether pinned message notifications are disabled.
    #[must_use]
    pub const fn disable_pinned_message_notifications(&self) -> bool {
        self.disable_pinned_message_notifications
    }

    /// Returns whether mention notifications are disabled.
    #[must_use]
    pub const fn disable_mention_notifications(&self) -> bool {
        self.disable_mention_notifications
    }

    /// Returns whether notifications are currently muted.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_scope_notification_settings::ScopeNotificationSettings;
    ///
    /// let settings = ScopeNotificationSettings::new()
    ///     .with_mute_until(9999999999);
    /// assert!(settings.is_muted());
    /// ```
    #[must_use]
    pub fn is_muted(&self) -> bool {
        self.mute_until > 0
    }
}

impl Default for ScopeNotificationSettings {
    fn default() -> Self {
        Self::defaults()
    }
}

impl fmt::Display for ScopeNotificationSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ScopeNotificationSettings {{ muted: {}, preview: {}, sync: {} }}",
            self.is_muted(),
            self.show_preview,
            self.is_synchronized
        )
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-scope-notification-settings";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-scope-notification-settings");
    }

    // NotificationSound stub tests
    #[test]
    fn test_notification_sound_new() {
        let sound = NotificationSound::new(String::from("test_sound"));
        assert_eq!(sound.id(), "test_sound");
    }

    #[test]
    fn test_notification_sound_default() {
        let sound = NotificationSound::default();
        assert_eq!(sound.id(), "default");
    }

    #[test]
    fn test_notification_sound_clone() {
        let sound1 = NotificationSound::new(String::from("sound1"));
        let sound2 = sound1.clone();
        assert_eq!(sound1.id(), sound2.id());
    }

    #[test]
    fn test_notification_sound_equality() {
        let sound1 = NotificationSound::new(String::from("sound"));
        let sound2 = NotificationSound::new(String::from("sound"));
        let sound3 = NotificationSound::new(String::from("other"));
        assert_eq!(sound1, sound2);
        assert_ne!(sound1, sound3);
    }

    // ScopeNotificationSettings constructor tests
    #[test]
    fn test_new() {
        let sound = NotificationSound::new(String::from("custom"));
        let settings = ScopeNotificationSettings::new(
            1234567890,
            Some(sound.clone()),
            None,
            false,
            false,
            true,
            false,
            true,
            false,
        );
        assert_eq!(settings.mute_until(), 1234567890);
        assert!(settings.sound().is_some());
        assert!(settings.sound().is_some_and(|s| s.id() == "custom"));
        assert!(settings.story_sound().is_none());
        assert!(!settings.show_preview());
        assert!(settings.mute_stories());
        assert!(settings.disable_pinned_message_notifications());
    }

    #[test]
    fn test_default_trait() {
        let settings = ScopeNotificationSettings::default();
        assert_eq!(settings.mute_until(), 0);
        assert!(settings.sound().is_none());
        assert!(settings.story_sound().is_none());
        assert!(settings.show_preview());
        assert!(settings.use_default_mute_stories());
        assert!(!settings.mute_stories());
        assert!(!settings.hide_story_sender());
        assert!(!settings.is_synchronized());
        assert!(!settings.disable_pinned_message_notifications());
        assert!(!settings.disable_mention_notifications());
    }

    #[test]
    fn test_builder() {
        let sound = NotificationSound::new(String::from("custom"));
        let settings = ScopeNotificationSettings::builder()
            .with_mute_until(1234567890)
            .with_sound(Some(sound))
            .with_show_preview(false)
            .build();

        assert_eq!(settings.mute_until(), 1234567890);
        assert!(settings.sound().is_some());
        assert!(!settings.show_preview());
    }

    // Builder method tests
    #[test]
    fn test_with_mute_until() {
        let settings = ScopeNotificationSettings::defaults().with_mute_until(999999999);
        assert_eq!(settings.mute_until(), 999999999);
    }

    #[test]
    fn test_with_sound() {
        let sound = NotificationSound::new(String::from("test"));
        let settings = ScopeNotificationSettings::defaults().with_sound(Some(sound.clone()));
        assert!(settings.sound().is_some());
        assert_eq!(settings.sound().unwrap().id(), "test");
    }

    #[test]
    fn test_with_sound_none() {
        let settings = ScopeNotificationSettings::defaults().with_sound(None);
        assert!(settings.sound().is_none());
    }

    #[test]
    fn test_with_story_sound() {
        let sound = NotificationSound::new(String::from("story"));
        let settings = ScopeNotificationSettings::defaults().with_story_sound(Some(sound));
        assert!(settings.story_sound().is_some());
        assert_eq!(settings.story_sound().unwrap().id(), "story");
    }

    #[test]
    fn test_with_show_preview() {
        let settings = ScopeNotificationSettings::defaults().with_show_preview(false);
        assert!(!settings.show_preview());
    }

    #[test]
    fn test_with_use_default_mute_stories() {
        let settings = ScopeNotificationSettings::defaults().with_use_default_mute_stories(false);
        assert!(!settings.use_default_mute_stories());
    }

    #[test]
    fn test_with_mute_stories() {
        let settings = ScopeNotificationSettings::defaults().with_mute_stories(true);
        assert!(settings.mute_stories());
    }

    #[test]
    fn test_with_hide_story_sender() {
        let settings = ScopeNotificationSettings::defaults().with_hide_story_sender(true);
        assert!(settings.hide_story_sender());
    }

    #[test]
    fn test_with_disable_pinned_message_notifications() {
        let settings = ScopeNotificationSettings::defaults().with_disable_pinned_message_notifications(true);
        assert!(settings.disable_pinned_message_notifications());
    }

    #[test]
    fn test_with_disable_mention_notifications() {
        let settings = ScopeNotificationSettings::defaults().with_disable_mention_notifications(true);
        assert!(settings.disable_mention_notifications());
    }

    #[test]
    fn test_with_synchronized() {
        let settings = ScopeNotificationSettings::defaults().with_synchronized(true);
        assert!(settings.is_synchronized());
    }

    // Chained builder tests
    #[test]
    fn test_chained_builder() {
        let sound = NotificationSound::new(String::from("custom"));
        let settings = ScopeNotificationSettings::defaults()
            .with_mute_until(1234567890)
            .with_sound(Some(sound))
            .with_show_preview(false)
            .with_mute_stories(true)
            .with_synchronized(true)
            .with_disable_pinned_message_notifications(true);

        assert_eq!(settings.mute_until(), 1234567890);
        assert!(settings.sound().is_some());
        assert!(!settings.show_preview());
        assert!(settings.mute_stories());
        assert!(settings.is_synchronized());
        assert!(settings.disable_pinned_message_notifications());
    }

    // Method tests
    #[test]
    fn test_is_muted_true() {
        let settings = ScopeNotificationSettings::defaults().with_mute_until(999999999);
        assert!(settings.is_muted());
    }

    #[test]
    fn test_is_muted_false() {
        let settings = ScopeNotificationSettings::defaults().with_mute_until(0);
        assert!(!settings.is_muted());
    }

    #[test]
    fn test_is_muted_negative() {
        let settings = ScopeNotificationSettings::defaults().with_mute_until(-1);
        assert!(!settings.is_muted());
    }

    // Trait tests
    #[test]
    fn test_clone() {
        let settings1 = ScopeNotificationSettings::new().with_mute_until(12345);
        let settings2 = settings1.clone();
        assert_eq!(settings1, settings2);
    }

    #[test]
    fn test_equality() {
        let settings1 = ScopeNotificationSettings::new();
        let settings2 = ScopeNotificationSettings::new();
        assert_eq!(settings1, settings2);
    }

    #[test]
    fn test_inequality_mute_until() {
        let settings1 = ScopeNotificationSettings::new().with_mute_until(100);
        let settings2 = ScopeNotificationSettings::new().with_mute_until(200);
        assert_ne!(settings1, settings2);
    }

    #[test]
    fn test_inequality_show_preview() {
        let settings1 = ScopeNotificationSettings::new().with_show_preview(true);
        let settings2 = ScopeNotificationSettings::new().with_show_preview(false);
        assert_ne!(settings1, settings2);
    }

    #[test]
    fn test_inequality_sound() {
        let sound = NotificationSound::new(String::from("custom"));
        let settings1 = ScopeNotificationSettings::new().with_sound(Some(sound.clone()));
        let settings2 = ScopeNotificationSettings::new().with_sound(None);
        assert_ne!(settings1, settings2);
    }

    #[test]
    fn test_debug() {
        let settings = ScopeNotificationSettings::defaults();
        let debug_str = format!("{:?}", settings);
        assert!(debug_str.contains("ScopeNotificationSettings"));
    }

    #[test]
    fn test_display() {
        let settings = ScopeNotificationSettings::defaults();
        let display_str = format!("{}", settings);
        assert!(display_str.contains("muted: false"));
        assert!(display_str.contains("preview: true"));
        assert!(display_str.contains("sync: false"));
    }

    // Edge case tests
    #[test]
    fn test_max_i32_mute_until() {
        let settings = ScopeNotificationSettings::defaults().with_mute_until(i32::MAX);
        assert_eq!(settings.mute_until(), i32::MAX);
        assert!(settings.is_muted());
    }

    #[test]
    fn test_min_i32_mute_until() {
        let settings = ScopeNotificationSettings::defaults().with_mute_until(i32::MIN);
        assert_eq!(settings.mute_until(), i32::MIN);
        assert!(!settings.is_muted());
    }

    #[test]
    fn test_all_false() {
        let settings = ScopeNotificationSettings::new(
            0,
            None,
            None,
            false,
            false,
            false,
            false,
            false,
            false,
        );
        assert!(!settings.show_preview());
        assert!(!settings.use_default_mute_stories());
        assert!(!settings.mute_stories());
        assert!(!settings.hide_story_sender());
        assert!(!settings.disable_pinned_message_notifications());
        assert!(!settings.disable_mention_notifications());
    }

    #[test]
    fn test_all_true() {
        let settings = ScopeNotificationSettings::new(
            1234567890,
            None,
            None,
            true,
            true,
            true,
            true,
            true,
            true,
        );
        assert!(settings.show_preview());
        assert!(settings.use_default_mute_stories());
        assert!(settings.mute_stories());
        assert!(settings.hide_story_sender());
        assert!(settings.disable_pinned_message_notifications());
        assert!(settings.disable_mention_notifications());
    }

    #[test]
    fn test_builder_default() {
        let settings = ScopeNotificationSettings::builder().build();
        assert_eq!(settings.mute_until(), 0);
        assert!(settings.show_preview());
    }

    #[test]
    fn test_builder_chained() {
        let sound = NotificationSound::new(String::from("sound"));
        let settings = ScopeNotificationSettings::builder()
            .with_mute_until(111)
            .with_sound(Some(sound))
            .with_show_preview(false)
            .with_use_default_mute_stories(false)
            .with_mute_stories(true)
            .with_hide_story_sender(true)
            .with_synchronized(true)
            .with_disable_pinned_message_notifications(true)
            .with_disable_mention_notifications(true)
            .build();

        assert_eq!(settings.mute_until(), 111);
        assert!(settings.sound().is_some());
        assert!(!settings.show_preview());
        assert!(!settings.use_default_mute_stories());
        assert!(settings.mute_stories());
        assert!(settings.hide_story_sender());
        assert!(settings.is_synchronized());
        assert!(settings.disable_pinned_message_notifications());
        assert!(settings.disable_mention_notifications());
    }
}
