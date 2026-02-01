// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Notification settings manager for Telegram MTProto client.
//!
//! This module implements TDLib's NotificationSettingsManager.
//!
//! # Overview
//!
//! The NotificationSettingsManager manages notification settings for different
//! scopes (users, chats, channels) and reaction notifications.
//!
//! # Example
//!
//! ```rust
//! use rustgram_notification_settings_manager::NotificationSettingsManager;
//!
//! let manager = NotificationSettingsManager::new();
//! let mute_until = manager.get_scope_mute_until(scope);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_dialog_id::DialogId;
use rustgram_scope_notification_settings::ScopeNotificationSettings;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

/// Notification settings scope.
///
/// Based on TDLib's `NotificationSettingsScope` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotificationSettingsScope {
    /// Private chats with users.
    Users,
    /// Group chats.
    Chats,
    /// Channels and supergroups.
    Channels,
}

/// Notification settings manager.
///
/// Based on TDLib's `NotificationSettingsManager` class.
///
/// Manages notification settings for different scopes including:
/// - Scope-specific settings (users, chats, channels)
/// - Reaction notification settings
/// - Saved ringtones
///
/// # Example
///
/// ```rust
/// use rustgram_notification_settings_manager::{
///     NotificationSettingsManager, NotificationSettingsScope
/// };
///
/// let manager = NotificationSettingsManager::new();
/// let mute_until = manager.get_scope_mute_until(NotificationSettingsScope::Users);
/// ```
#[derive(Debug, Clone)]
pub struct NotificationSettingsManager {
    /// Shared settings state.
    settings: Arc<SettingsState>,
}

/// Shared settings state.
#[derive(Debug)]
struct SettingsState {
    /// Users notification settings.
    users_settings: RwLock<ScopeNotificationSettings>,
    /// Chats notification settings.
    chats_settings: RwLock<ScopeNotificationSettings>,
    /// Channels notification settings.
    channels_settings: RwLock<ScopeNotificationSettings>,
    /// Reaction notification settings (mute_until, show_preview).
    reaction_settings: RwLock<(i32, bool)>,
    /// Saved ringtones (ringtone_id -> FileId mapping).
    /// TODO: Replace with proper FileId when available.
    saved_ringtones: RwLock<HashMap<i64, String>>,
}

impl Default for NotificationSettingsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationSettingsManager {
    /// Creates a new NotificationSettingsManager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::NotificationSettingsManager;
    ///
    /// let manager = NotificationSettingsManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            settings: Arc::new(SettingsState {
                users_settings: RwLock::new(ScopeNotificationSettings::defaults()),
                chats_settings: RwLock::new(ScopeNotificationSettings::defaults()),
                channels_settings: RwLock::new(ScopeNotificationSettings::defaults()),
                reaction_settings: RwLock::new((0, true)),
                saved_ringtones: RwLock::new(HashMap::new()),
            }),
        }
    }

    /// Initializes the manager.
    ///
    /// This method should be called after creating the manager to set up
    /// any necessary internal state.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::NotificationSettingsManager;
    ///
    /// let manager = NotificationSettingsManager::new();
    /// manager.init();
    /// ```
    pub fn init(&self) {
        // TODO: Load settings from database when storage module is available
    }

    /// Gets the mute_until timestamp for a scope.
    ///
    /// # Arguments
    ///
    /// * `scope` - The notification settings scope
    ///
    /// # Returns
    ///
    /// The Unix timestamp until which notifications are muted.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::{
    ///     NotificationSettingsManager, NotificationSettingsScope
    /// };
    ///
    /// let manager = NotificationSettingsManager::new();
    /// let mute_until = manager.get_scope_mute_until(NotificationSettingsScope::Users);
    /// assert_eq!(mute_until, 0);
    /// ```
    #[must_use]
    pub fn get_scope_mute_until(&self, scope: NotificationSettingsScope) -> i32 {
        match scope {
            NotificationSettingsScope::Users => {
                self.settings.users_settings.read().unwrap().mute_until()
            }
            NotificationSettingsScope::Chats => {
                self.settings.chats_settings.read().unwrap().mute_until()
            }
            NotificationSettingsScope::Channels => {
                self.settings.channels_settings.read().unwrap().mute_until()
            }
        }
    }

    /// Gets the mute stories settings for a scope.
    ///
    /// # Arguments
    ///
    /// * `scope` - The notification settings scope
    ///
    /// # Returns
    ///
    /// A tuple of (use_default_mute_stories, mute_stories).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::{
    ///     NotificationSettingsManager, NotificationSettingsScope
    /// };
    ///
    /// let manager = NotificationSettingsManager::new();
    /// let (use_default, mute) = manager.get_scope_mute_stories(NotificationSettingsScope::Users);
    /// assert!(use_default);
    /// assert!(!mute);
    /// ```
    #[must_use]
    pub fn get_scope_mute_stories(&self, scope: NotificationSettingsScope) -> (bool, bool) {
        match scope {
            NotificationSettingsScope::Users => {
                let settings = self.settings.users_settings.read().unwrap();
                (settings.use_default_mute_stories(), settings.mute_stories())
            }
            NotificationSettingsScope::Chats => {
                let settings = self.settings.chats_settings.read().unwrap();
                (settings.use_default_mute_stories(), settings.mute_stories())
            }
            NotificationSettingsScope::Channels => {
                let settings = self.settings.channels_settings.read().unwrap();
                (settings.use_default_mute_stories(), settings.mute_stories())
            }
        }
    }

    /// Gets whether to show preview for a scope.
    ///
    /// # Arguments
    ///
    /// * `scope` - The notification settings scope
    ///
    /// # Returns
    ///
    /// Whether to show message previews.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::{
    ///     NotificationSettingsManager, NotificationSettingsScope
    /// };
    ///
    /// let manager = NotificationSettingsManager::new();
    /// assert!(manager.get_scope_show_preview(NotificationSettingsScope::Users));
    /// ```
    #[must_use]
    pub fn get_scope_show_preview(&self, scope: NotificationSettingsScope) -> bool {
        match scope {
            NotificationSettingsScope::Users => {
                self.settings.users_settings.read().unwrap().show_preview()
            }
            NotificationSettingsScope::Chats => {
                self.settings.chats_settings.read().unwrap().show_preview()
            }
            NotificationSettingsScope::Channels => self
                .settings
                .channels_settings
                .read()
                .unwrap()
                .show_preview(),
        }
    }

    /// Gets whether to hide story sender for a scope.
    ///
    /// # Arguments
    ///
    /// * `scope` - The notification settings scope
    ///
    /// # Returns
    ///
    /// Whether to hide story senders.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::{
    ///     NotificationSettingsManager, NotificationSettingsScope
    /// };
    ///
    /// let manager = NotificationSettingsManager::new();
    /// assert!(!manager.get_scope_hide_story_sender(NotificationSettingsScope::Users));
    /// ```
    #[must_use]
    pub fn get_scope_hide_story_sender(&self, scope: NotificationSettingsScope) -> bool {
        match scope {
            NotificationSettingsScope::Users => self
                .settings
                .users_settings
                .read()
                .unwrap()
                .hide_story_sender(),
            NotificationSettingsScope::Chats => self
                .settings
                .chats_settings
                .read()
                .unwrap()
                .hide_story_sender(),
            NotificationSettingsScope::Channels => self
                .settings
                .channels_settings
                .read()
                .unwrap()
                .hide_story_sender(),
        }
    }

    /// Gets whether to disable pinned message notifications for a scope.
    ///
    /// # Arguments
    ///
    /// * `scope` - The notification settings scope
    ///
    /// # Returns
    ///
    /// Whether to disable pinned message notifications.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::{
    ///     NotificationSettingsManager, NotificationSettingsScope
    /// };
    ///
    /// let manager = NotificationSettingsManager::new();
    /// assert!(!manager.get_scope_disable_pinned_message_notifications(NotificationSettingsScope::Users));
    /// ```
    #[must_use]
    pub fn get_scope_disable_pinned_message_notifications(
        &self,
        scope: NotificationSettingsScope,
    ) -> bool {
        match scope {
            NotificationSettingsScope::Users => self
                .settings
                .users_settings
                .read()
                .unwrap()
                .disable_pinned_message_notifications(),
            NotificationSettingsScope::Chats => self
                .settings
                .chats_settings
                .read()
                .unwrap()
                .disable_pinned_message_notifications(),
            NotificationSettingsScope::Channels => self
                .settings
                .channels_settings
                .read()
                .unwrap()
                .disable_pinned_message_notifications(),
        }
    }

    /// Gets whether to disable mention notifications for a scope.
    ///
    /// # Arguments
    ///
    /// * `scope` - The notification settings scope
    ///
    /// # Returns
    ///
    /// Whether to disable mention notifications.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::{
    ///     NotificationSettingsManager, NotificationSettingsScope
    /// };
    ///
    /// let manager = NotificationSettingsManager::new();
    /// assert!(!manager.get_scope_disable_mention_notifications(NotificationSettingsScope::Users));
    /// ```
    #[must_use]
    pub fn get_scope_disable_mention_notifications(
        &self,
        scope: NotificationSettingsScope,
    ) -> bool {
        match scope {
            NotificationSettingsScope::Users => self
                .settings
                .users_settings
                .read()
                .unwrap()
                .disable_mention_notifications(),
            NotificationSettingsScope::Chats => self
                .settings
                .chats_settings
                .read()
                .unwrap()
                .disable_mention_notifications(),
            NotificationSettingsScope::Channels => self
                .settings
                .channels_settings
                .read()
                .unwrap()
                .disable_mention_notifications(),
        }
    }

    /// Gets the notification settings for a scope.
    ///
    /// # Arguments
    ///
    /// * `scope` - The notification settings scope
    ///
    /// # Returns
    ///
    /// A clone of the scope notification settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::{
    ///     NotificationSettingsManager, NotificationSettingsScope
    /// };
    ///
    /// let manager = NotificationSettingsManager::new();
    /// let settings = manager.get_scope_settings(NotificationSettingsScope::Users);
    /// assert!(settings.show_preview());
    /// ```
    #[must_use]
    pub fn get_scope_settings(
        &self,
        scope: NotificationSettingsScope,
    ) -> ScopeNotificationSettings {
        match scope {
            NotificationSettingsScope::Users => {
                self.settings.users_settings.read().unwrap().clone()
            }
            NotificationSettingsScope::Chats => {
                self.settings.chats_settings.read().unwrap().clone()
            }
            NotificationSettingsScope::Channels => {
                self.settings.channels_settings.read().unwrap().clone()
            }
        }
    }

    /// Sets the notification settings for a scope.
    ///
    /// # Arguments
    ///
    /// * `scope` - The notification settings scope
    /// * `settings` - The new settings to apply
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::{
    ///     NotificationSettingsManager, NotificationSettingsScope
    /// };
    /// use rustgram_scope_notification_settings::ScopeNotificationSettings;
    ///
    /// let manager = NotificationSettingsManager::new();
    /// let settings = ScopeNotificationSettings::defaults()
    ///     .with_mute_until(1234567890)
    ///     .with_show_preview(false);
    /// manager.set_scope_settings(NotificationSettingsScope::Users, settings);
    /// ```
    pub fn set_scope_settings(
        &self,
        scope: NotificationSettingsScope,
        settings: ScopeNotificationSettings,
    ) {
        match scope {
            NotificationSettingsScope::Users => {
                *self.settings.users_settings.write().unwrap() = settings;
            }
            NotificationSettingsScope::Chats => {
                *self.settings.chats_settings.write().unwrap() = settings;
            }
            NotificationSettingsScope::Channels => {
                *self.settings.channels_settings.write().unwrap() = settings;
            }
        }
        // TODO: Send update to server when net module is available
    }

    /// Gets the reaction notification mute_until timestamp.
    ///
    /// # Returns
    ///
    /// The Unix timestamp until which reaction notifications are muted.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::NotificationSettingsManager;
    ///
    /// let manager = NotificationSettingsManager::new();
    /// assert_eq!(manager.get_reaction_mute_until(), 0);
    /// ```
    #[must_use]
    pub fn get_reaction_mute_until(&self) -> i32 {
        self.settings.reaction_settings.read().unwrap().0
    }

    /// Gets the reaction notification show_preview setting.
    ///
    /// # Returns
    ///
    /// Whether to show reaction notifications.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::NotificationSettingsManager;
    ///
    /// let manager = NotificationSettingsManager::new();
    /// assert!(manager.get_reaction_show_preview());
    /// ```
    #[must_use]
    pub fn get_reaction_show_preview(&self) -> bool {
        self.settings.reaction_settings.read().unwrap().1
    }

    /// Sets the reaction notification settings.
    ///
    /// # Arguments
    ///
    /// * `mute_until` - The Unix timestamp until which to mute reactions
    /// * `show_preview` - Whether to show reaction notifications
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::NotificationSettingsManager;
    ///
    /// let manager = NotificationSettingsManager::new();
    /// manager.set_reaction_settings(1234567890, false);
    /// assert_eq!(manager.get_reaction_mute_until(), 1234567890);
    /// assert!(!manager.get_reaction_show_preview());
    /// ```
    pub fn set_reaction_settings(&self, mute_until: i32, show_preview: bool) {
        *self.settings.reaction_settings.write().unwrap() = (mute_until, show_preview);
        // TODO: Send update to server when net module is available
    }

    /// Gets all saved ringtone IDs.
    ///
    /// # Returns
    ///
    /// A vector of saved ringtone IDs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::NotificationSettingsManager;
    ///
    /// let manager = NotificationSettingsManager::new();
    /// let ringtones = manager.get_saved_ringtones();
    /// assert!(ringtones.is_empty());
    /// ```
    #[must_use]
    pub fn get_saved_ringtones(&self) -> Vec<i64> {
        self.settings
            .saved_ringtones
            .read()
            .unwrap()
            .keys()
            .copied()
            .collect()
    }

    /// Adds a saved ringtone.
    ///
    /// # Arguments
    ///
    /// * `ringtone_id` - The ringtone ID
    /// * `file_id` - The file identifier (stub implementation uses String)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::NotificationSettingsManager;
    ///
    /// let manager = NotificationSettingsManager::new();
    /// manager.add_saved_ringtone(123, "file_123".to_string());
    /// assert!(manager.get_saved_ringtones().contains(&123));
    /// ```
    pub fn add_saved_ringtone(&self, ringtone_id: i64, file_id: String) {
        self.settings
            .saved_ringtones
            .write()
            .unwrap()
            .insert(ringtone_id, file_id);
        // TODO: Send update to server when net module is available
    }

    /// Removes a saved ringtone.
    ///
    /// # Arguments
    ///
    /// * `ringtone_id` - The ringtone ID to remove
    ///
    /// # Returns
    ///
    /// `true` if the ringtone was removed, `false` if it didn't exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::NotificationSettingsManager;
    ///
    /// let manager = NotificationSettingsManager::new();
    /// manager.add_saved_ringtone(123, "file_123".to_string());
    /// assert!(manager.remove_saved_ringtone(123));
    /// assert!(!manager.remove_saved_ringtone(456));
    /// ```
    pub fn remove_saved_ringtone(&self, ringtone_id: i64) -> bool {
        self.settings
            .saved_ringtones
            .write()
            .unwrap()
            .remove(&ringtone_id)
            .is_some()
    }

    /// Resets all notification settings to default.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_notification_settings_manager::NotificationSettingsManager;
    ///
    /// let manager = NotificationSettingsManager::new();
    /// manager.reset_all_settings();
    /// ```
    pub fn reset_all_settings(&self) {
        *self.settings.users_settings.write().unwrap() = ScopeNotificationSettings::defaults();
        *self.settings.chats_settings.write().unwrap() = ScopeNotificationSettings::defaults();
        *self.settings.channels_settings.write().unwrap() = ScopeNotificationSettings::defaults();
        *self.settings.reaction_settings.write().unwrap() = (0, true);
        self.settings.saved_ringtones.write().unwrap().clear();
        // TODO: Send update to server when net module is available
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let manager = NotificationSettingsManager::new();
        assert_eq!(
            manager.get_scope_mute_until(NotificationSettingsScope::Users),
            0
        );
    }

    #[test]
    fn test_default() {
        let manager = NotificationSettingsManager::default();
        assert_eq!(
            manager.get_scope_mute_until(NotificationSettingsScope::Users),
            0
        );
    }

    #[test]
    fn test_init() {
        let manager = NotificationSettingsManager::new();
        manager.init(); // Should not panic
    }

    #[test]
    fn test_get_scope_mute_until() {
        let manager = NotificationSettingsManager::new();
        assert_eq!(
            manager.get_scope_mute_until(NotificationSettingsScope::Users),
            0
        );
        assert_eq!(
            manager.get_scope_mute_until(NotificationSettingsScope::Chats),
            0
        );
        assert_eq!(
            manager.get_scope_mute_until(NotificationSettingsScope::Channels),
            0
        );
    }

    #[test]
    fn test_get_scope_mute_stories() {
        let manager = NotificationSettingsManager::new();
        let (use_default, mute) = manager.get_scope_mute_stories(NotificationSettingsScope::Users);
        assert!(use_default);
        assert!(!mute);
    }

    #[test]
    fn test_get_scope_show_preview() {
        let manager = NotificationSettingsManager::new();
        assert!(manager.get_scope_show_preview(NotificationSettingsScope::Users));
        assert!(manager.get_scope_show_preview(NotificationSettingsScope::Chats));
        assert!(manager.get_scope_show_preview(NotificationSettingsScope::Channels));
    }

    #[test]
    fn test_get_scope_hide_story_sender() {
        let manager = NotificationSettingsManager::new();
        assert!(!manager.get_scope_hide_story_sender(NotificationSettingsScope::Users));
        assert!(!manager.get_scope_hide_story_sender(NotificationSettingsScope::Chats));
        assert!(!manager.get_scope_hide_story_sender(NotificationSettingsScope::Channels));
    }

    #[test]
    fn test_get_scope_disable_pinned_message_notifications() {
        let manager = NotificationSettingsManager::new();
        assert!(!manager
            .get_scope_disable_pinned_message_notifications(NotificationSettingsScope::Users));
        assert!(!manager
            .get_scope_disable_pinned_message_notifications(NotificationSettingsScope::Chats));
        assert!(!manager
            .get_scope_disable_pinned_message_notifications(NotificationSettingsScope::Channels));
    }

    #[test]
    fn test_get_scope_disable_mention_notifications() {
        let manager = NotificationSettingsManager::new();
        assert!(!manager.get_scope_disable_mention_notifications(NotificationSettingsScope::Users));
        assert!(!manager.get_scope_disable_mention_notifications(NotificationSettingsScope::Chats));
        assert!(
            !manager.get_scope_disable_mention_notifications(NotificationSettingsScope::Channels)
        );
    }

    #[test]
    fn test_set_scope_settings() {
        let manager = NotificationSettingsManager::new();
        let settings = ScopeNotificationSettings::defaults()
            .with_mute_until(1234567890)
            .with_show_preview(false);

        manager.set_scope_settings(NotificationSettingsScope::Users, settings.clone());
        assert_eq!(
            manager.get_scope_mute_until(NotificationSettingsScope::Users),
            1234567890
        );
        assert!(!manager.get_scope_show_preview(NotificationSettingsScope::Users));
    }

    #[test]
    fn test_get_reaction_mute_until() {
        let manager = NotificationSettingsManager::new();
        assert_eq!(manager.get_reaction_mute_until(), 0);
    }

    #[test]
    fn test_get_reaction_show_preview() {
        let manager = NotificationSettingsManager::new();
        assert!(manager.get_reaction_show_preview());
    }

    #[test]
    fn test_set_reaction_settings() {
        let manager = NotificationSettingsManager::new();
        manager.set_reaction_settings(1234567890, false);
        assert_eq!(manager.get_reaction_mute_until(), 1234567890);
        assert!(!manager.get_reaction_show_preview());
    }

    #[test]
    fn test_get_saved_ringtones_empty() {
        let manager = NotificationSettingsManager::new();
        assert!(manager.get_saved_ringtones().is_empty());
    }

    #[test]
    fn test_add_saved_ringtone() {
        let manager = NotificationSettingsManager::new();
        manager.add_saved_ringtone(123, "file_123".to_string());
        let ringtones = manager.get_saved_ringtones();
        assert_eq!(ringtones.len(), 1);
        assert!(ringtones.contains(&123));
    }

    #[test]
    fn test_add_multiple_saved_ringtones() {
        let manager = NotificationSettingsManager::new();
        manager.add_saved_ringtone(123, "file_123".to_string());
        manager.add_saved_ringtone(456, "file_456".to_string());
        manager.add_saved_ringtone(789, "file_789".to_string());

        let ringtones = manager.get_saved_ringtones();
        assert_eq!(ringtones.len(), 3);
        assert!(ringtones.contains(&123));
        assert!(ringtones.contains(&456));
        assert!(ringtones.contains(&789));
    }

    #[test]
    fn test_remove_saved_ringtone() {
        let manager = NotificationSettingsManager::new();
        manager.add_saved_ringtone(123, "file_123".to_string());
        assert!(manager.remove_saved_ringtone(123));
        assert!(!manager.remove_saved_ringtone(123));
        assert!(manager.get_saved_ringtones().is_empty());
    }

    #[test]
    fn test_remove_saved_ringtone_nonexistent() {
        let manager = NotificationSettingsManager::new();
        assert!(!manager.remove_saved_ringtone(999));
    }

    #[test]
    fn test_reset_all_settings() {
        let manager = NotificationSettingsManager::new();
        let settings = ScopeNotificationSettings::defaults().with_mute_until(1234567890);
        manager.set_scope_settings(NotificationSettingsScope::Users, settings);
        manager.set_reaction_settings(999, false);
        manager.add_saved_ringtone(123, "file_123".to_string());

        manager.reset_all_settings();

        assert_eq!(
            manager.get_scope_mute_until(NotificationSettingsScope::Users),
            0
        );
        assert_eq!(manager.get_reaction_mute_until(), 0);
        assert!(manager.get_reaction_show_preview());
        assert!(manager.get_saved_ringtones().is_empty());
    }

    #[test]
    fn test_clone() {
        let manager1 = NotificationSettingsManager::new();
        manager1.set_scope_settings(
            NotificationSettingsScope::Users,
            ScopeNotificationSettings::defaults().with_mute_until(111),
        );

        let manager2 = manager1.clone();
        assert_eq!(
            manager2.get_scope_mute_until(NotificationSettingsScope::Users),
            111
        );
    }

    #[test]
    fn test_independent_managers() {
        let manager1 = NotificationSettingsManager::new();
        let manager2 = NotificationSettingsManager::new();

        manager1.set_scope_settings(
            NotificationSettingsScope::Users,
            ScopeNotificationSettings::defaults().with_mute_until(111),
        );

        assert_eq!(
            manager1.get_scope_mute_until(NotificationSettingsScope::Users),
            111
        );
        assert_eq!(
            manager2.get_scope_mute_until(NotificationSettingsScope::Users),
            0
        );
    }

    #[test]
    fn test_all_scopes_independent() {
        let manager = NotificationSettingsManager::new();

        manager.set_scope_settings(
            NotificationSettingsScope::Users,
            ScopeNotificationSettings::defaults().with_mute_until(111),
        );
        manager.set_scope_settings(
            NotificationSettingsScope::Chats,
            ScopeNotificationSettings::defaults().with_mute_until(222),
        );
        manager.set_scope_settings(
            NotificationSettingsScope::Channels,
            ScopeNotificationSettings::defaults().with_mute_until(333),
        );

        assert_eq!(
            manager.get_scope_mute_until(NotificationSettingsScope::Users),
            111
        );
        assert_eq!(
            manager.get_scope_mute_until(NotificationSettingsScope::Chats),
            222
        );
        assert_eq!(
            manager.get_scope_mute_until(NotificationSettingsScope::Channels),
            333
        );
    }

    #[test]
    fn test_debug_format() {
        let manager = NotificationSettingsManager::new();
        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("NotificationSettingsManager"));
    }
}
