// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Autosave settings manager for Telegram MTProto client.
//!
//! This module implements TDLib's AutosaveManager class.
//!
//! # Example
//!
//! ```rust
//! use rustgram_autosave_manager::{DialogAutosaveSettings, AutosaveSettings, DEFAULT_MAX_VIDEO_FILE_SIZE};
//!
//! let settings = DialogAutosaveSettings::default();
//! assert_eq!(settings.max_video_file_size(), DEFAULT_MAX_VIDEO_FILE_SIZE);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_dialog_id::DialogId;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

/// Minimum allowed max video file size (512 KB).
pub const MIN_MAX_VIDEO_FILE_SIZE: i64 = 512 * 1024;

/// Default max video file size (100 MB).
pub const DEFAULT_MAX_VIDEO_FILE_SIZE: i64 = 100 * 1024 * 1024;

/// Maximum allowed max video file size (4 GB).
pub const MAX_MAX_VIDEO_FILE_SIZE: i64 = (4000_i64) * 1024 * 1024;

/// Dialog autosave settings.
///
/// Based on TDLib's `AutosaveManager::DialogAutosaveSettings` struct.
///
/// Contains settings for automatic media saving for a specific dialog scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogAutosaveSettings {
    /// Whether settings have been initialized.
    pub are_inited: bool,

    /// Whether to autosave photos.
    pub autosave_photos: bool,

    /// Whether to autosave videos.
    pub autosave_videos: bool,

    /// Maximum video file size to autosave (in bytes).
    pub max_video_file_size: i64,
}

impl DialogAutosaveSettings {
    /// Creates a new DialogAutosaveSettings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::DialogAutosaveSettings;
    ///
    /// let settings = DialogAutosaveSettings::new(true, false, 10 * 1024 * 1024);
    /// assert!(settings.autosave_photos);
    /// assert!(!settings.autosave_videos);
    /// ```
    pub fn new(autosave_photos: bool, autosave_videos: bool, max_video_file_size: i64) -> Self {
        Self {
            are_inited: true,
            autosave_photos,
            autosave_videos,
            max_video_file_size: max_video_file_size.clamp(
                MIN_MAX_VIDEO_FILE_SIZE,
                MAX_MAX_VIDEO_FILE_SIZE,
            ),
        }
    }

    /// Returns the maximum video file size.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::{DialogAutosaveSettings, DEFAULT_MAX_VIDEO_FILE_SIZE};
    ///
    /// let settings = DialogAutosaveSettings::default();
    /// assert_eq!(settings.max_video_file_size(), DEFAULT_MAX_VIDEO_FILE_SIZE);
    /// ```
    pub const fn max_video_file_size(&self) -> i64 {
        self.max_video_file_size
    }

    /// Checks if settings are initialized.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::DialogAutosaveSettings;
    ///
    /// let settings = DialogAutosaveSettings::default();
    /// assert!(!settings.is_initialized());
    /// ```
    pub const fn is_initialized(&self) -> bool {
        self.are_inited
    }

    /// Sets whether to autosave photos.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::DialogAutosaveSettings;
    ///
    /// let mut settings = DialogAutosaveSettings::default();
    /// settings.set_autosave_photos(true);
    /// assert!(settings.autosave_photos);
    /// ```
    pub fn set_autosave_photos(&mut self, value: bool) {
        self.autosave_photos = value;
        self.are_inited = true;
    }

    /// Sets whether to autosave videos.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::DialogAutosaveSettings;
    ///
    /// let mut settings = DialogAutosaveSettings::default();
    /// settings.set_autosave_videos(true);
    /// assert!(settings.autosave_videos);
    /// ```
    pub fn set_autosave_videos(&mut self, value: bool) {
        self.autosave_videos = value;
        self.are_inited = true;
    }

    /// Sets the maximum video file size.
    ///
    /// # Arguments
    ///
    /// * `size` - Maximum file size in bytes (will be clamped to valid range)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::DialogAutosaveSettings;
    ///
    /// let mut settings = DialogAutosaveSettings::default();
    /// settings.set_max_video_file_size(50 * 1024 * 1024);
    /// assert_eq!(settings.max_video_file_size(), 50 * 1024 * 1024);
    /// ```
    pub fn set_max_video_file_size(&mut self, size: i64) {
        self.max_video_file_size = size.clamp(MIN_MAX_VIDEO_FILE_SIZE, MAX_MAX_VIDEO_FILE_SIZE);
        self.are_inited = true;
    }
}

impl Default for DialogAutosaveSettings {
    fn default() -> Self {
        Self {
            are_inited: false,
            autosave_photos: false,
            autosave_videos: false,
            max_video_file_size: DEFAULT_MAX_VIDEO_FILE_SIZE,
        }
    }
}

impl Display for DialogAutosaveSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DialogAutosaveSettings(photos={}, videos={}, max_size={})",
            self.autosave_photos,
            self.autosave_videos,
            self.max_video_file_size
        )
    }
}

/// Autosave settings for all scopes.
///
/// Based on TDLib's `AutosaveManager::AutosaveSettings` struct.
///
/// Contains autosave settings for different dialog types (users, chats, broadcasts)
/// and per-dialog exceptions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutosaveSettings {
    /// Whether settings have been initialized.
    pub are_inited: bool,

    /// Whether settings are being reloaded.
    pub are_being_reloaded: bool,

    /// Whether settings need to be reloaded.
    pub need_reload: bool,

    /// Settings for private user chats.
    pub user_settings: DialogAutosaveSettings,

    /// Settings for group chats.
    pub chat_settings: DialogAutosaveSettings,

    /// Settings for broadcast channels.
    pub broadcast_settings: DialogAutosaveSettings,

    /// Per-dialog exceptions.
    pub exceptions: HashMap<DialogId, DialogAutosaveSettings>,
}

impl AutosaveSettings {
    /// Creates a new AutosaveSettings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::AutosaveSettings;
    ///
    /// let settings = AutosaveSettings::new();
    /// assert!(!settings.are_inited);
    /// ```
    pub fn new() -> Self {
        Self {
            are_inited: false,
            are_being_reloaded: false,
            need_reload: false,
            user_settings: DialogAutosaveSettings::default(),
            chat_settings: DialogAutosaveSettings::default(),
            broadcast_settings: DialogAutosaveSettings::default(),
            exceptions: HashMap::new(),
        }
    }

    /// Returns true if settings are initialized.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::AutosaveSettings;
    ///
    /// let settings = AutosaveSettings::new();
    /// assert!(!settings.is_initialized());
    /// ```
    pub const fn is_initialized(&self) -> bool {
        self.are_inited
    }

    /// Marks settings as initialized.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::AutosaveSettings;
    ///
    /// let mut settings = AutosaveSettings::new();
    /// settings.mark_initialized();
    /// assert!(settings.is_initialized());
    /// ```
    pub fn mark_initialized(&mut self) {
        self.are_inited = true;
    }

    /// Returns the user settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::AutosaveSettings;
    ///
    /// let settings = AutosaveSettings::new();
    /// let user_settings = settings.user_settings();
    /// assert!(!user_settings.is_initialized());
    /// ```
    pub const fn user_settings(&self) -> &DialogAutosaveSettings {
        &self.user_settings
    }

    /// Returns the chat settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::AutosaveSettings;
    ///
    /// let settings = AutosaveSettings::new();
    /// let chat_settings = settings.chat_settings();
    /// assert!(!chat_settings.is_initialized());
    /// ```
    pub const fn chat_settings(&self) -> &DialogAutosaveSettings {
        &self.chat_settings
    }

    /// Returns the broadcast settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::AutosaveSettings;
    ///
    /// let settings = AutosaveSettings::new();
    /// let broadcast_settings = settings.broadcast_settings();
    /// assert!(!broadcast_settings.is_initialized());
    /// ```
    pub const fn broadcast_settings(&self) -> &DialogAutosaveSettings {
        &self.broadcast_settings
    }

    /// Sets the user settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::{AutosaveSettings, DialogAutosaveSettings};
    ///
    /// let mut settings = AutosaveSettings::new();
    /// let new_user = DialogAutosaveSettings::new(true, false, 10 * 1024 * 1024);
    /// settings.set_user_settings(new_user);
    /// ```
    pub fn set_user_settings(&mut self, settings: DialogAutosaveSettings) {
        self.user_settings = settings;
    }

    /// Sets the chat settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::{AutosaveSettings, DialogAutosaveSettings};
    ///
    /// let mut settings = AutosaveSettings::new();
    /// let new_chat = DialogAutosaveSettings::new(false, true, 50 * 1024 * 1024);
    /// settings.set_chat_settings(new_chat);
    /// ```
    pub fn set_chat_settings(&mut self, settings: DialogAutosaveSettings) {
        self.chat_settings = settings;
    }

    /// Sets the broadcast settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::{AutosaveSettings, DialogAutosaveSettings};
    ///
    /// let mut settings = AutosaveSettings::new();
    /// let new_broadcast = DialogAutosaveSettings::new(true, true, 100 * 1024 * 1024);
    /// settings.set_broadcast_settings(new_broadcast);
    /// ```
    pub fn set_broadcast_settings(&mut self, settings: DialogAutosaveSettings) {
        self.broadcast_settings = settings;
    }

    /// Adds or updates an exception for a specific dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `settings` - Exception settings for this dialog
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::{AutosaveSettings, DialogAutosaveSettings};
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let mut settings = AutosaveSettings::new();
    /// let exception = DialogAutosaveSettings::new(true, true, 200 * 1024 * 1024);
    /// settings.add_exception(DialogId::new(100), exception);
    /// assert_eq!(settings.exceptions_count(), 1);
    /// ```
    pub fn add_exception(&mut self, dialog_id: DialogId, exception_settings: DialogAutosaveSettings) {
        self.exceptions.insert(dialog_id, exception_settings);
    }

    /// Gets the exception settings for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Returns
    ///
    /// Option containing the exception settings if found.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::{AutosaveSettings, DialogAutosaveSettings};
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let mut settings = AutosaveSettings::new();
    /// let exception = DialogAutosaveSettings::new(true, true, 200 * 1024 * 1024);
    /// settings.add_exception(DialogId::new(100), exception.clone());
    ///
    /// let retrieved = settings.get_exception(DialogId::new(100));
    /// assert!(retrieved.is_some());
    /// assert_eq!(retrieved.unwrap(), &exception);
    /// ```
    pub fn get_exception(&self, dialog_id: DialogId) -> Option<&DialogAutosaveSettings> {
        self.exceptions.get(&dialog_id)
    }

    /// Removes an exception for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Returns
    ///
    /// Option containing the removed exception settings if it existed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::{AutosaveSettings, DialogAutosaveSettings};
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let mut settings = AutosaveSettings::new();
    /// let exception = DialogAutosaveSettings::new(true, true, 200 * 1024 * 1024);
    /// settings.add_exception(DialogId::new(100), exception);
    ///
    /// let removed = settings.remove_exception(DialogId::new(100));
    /// assert!(removed.is_some());
    /// assert_eq!(settings.exceptions_count(), 0);
    /// ```
    pub fn remove_exception(&mut self, dialog_id: DialogId) -> Option<DialogAutosaveSettings> {
        self.exceptions.remove(&dialog_id)
    }

    /// Clears all exceptions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::{AutosaveSettings, DialogAutosaveSettings};
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let mut settings = AutosaveSettings::new();
    /// settings.add_exception(DialogId::new(100), DialogAutosaveSettings::default());
    /// settings.add_exception(DialogId::new(200), DialogAutosaveSettings::default());
    /// assert_eq!(settings.exceptions_count(), 2);
    ///
    /// settings.clear_exceptions();
    /// assert_eq!(settings.exceptions_count(), 0);
    /// ```
    pub fn clear_exceptions(&mut self) {
        self.exceptions.clear();
    }

    /// Returns the number of exceptions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::{AutosaveSettings, DialogAutosaveSettings};
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let mut settings = AutosaveSettings::new();
    /// assert_eq!(settings.exceptions_count(), 0);
    ///
    /// settings.add_exception(DialogId::new(100), DialogAutosaveSettings::default());
    /// assert_eq!(settings.exceptions_count(), 1);
    /// ```
    pub fn exceptions_count(&self) -> usize {
        self.exceptions.len()
    }
}

impl Default for AutosaveSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for AutosaveSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AutosaveSettings(inited={}, reloading={}, exceptions={})",
            self.are_inited,
            self.are_being_reloaded,
            self.exceptions_count()
        )
    }
}

/// Autosave settings manager.
///
/// Based on TDLib's `AutosaveManager` class.
///
/// Manages autosave settings for the application.
#[derive(Debug, Clone)]
pub struct AutosaveManager {
    /// Settings storage.
    settings: AutosaveSettings,
}

impl AutosaveManager {
    /// Creates a new AutosaveManager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::AutosaveManager;
    ///
    /// let manager = AutosaveManager::new();
    /// assert!(!manager.settings().is_initialized());
    /// ```
    pub fn new() -> Self {
        Self {
            settings: AutosaveSettings::new(),
        }
    }

    /// Returns a reference to the settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::AutosaveManager;
    ///
    /// let manager = AutosaveManager::new();
    /// let settings = manager.settings();
    /// assert!(!settings.is_initialized());
    /// ```
    pub const fn settings(&self) -> &AutosaveSettings {
        &self.settings
    }

    /// Returns a mutable reference to the settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::AutosaveManager;
    ///
    /// let mut manager = AutosaveManager::new();
    /// let settings = manager.settings_mut();
    /// settings.mark_initialized();
    /// ```
    pub fn settings_mut(&mut self) -> &mut AutosaveSettings {
        &mut self.settings
    }

    /// Reloads autosave settings (stub implementation).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::AutosaveManager;
    ///
    /// let mut manager = AutosaveManager::new();
    /// // TODO: Implement actual reload from server/database
    /// ```
    pub fn reload_settings(&mut self) {
        self.settings.are_being_reloaded = true;
        // TODO: Implement actual reload logic
        self.settings.are_being_reloaded = false;
    }

    /// Gets current autosave settings.
    ///
    /// # Returns
    ///
    /// Clone of the current settings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::AutosaveManager;
    ///
    /// let manager = AutosaveManager::new();
    /// let settings = manager.get_settings();
    /// assert!(!settings.is_initialized());
    /// ```
    pub fn get_settings(&self) -> AutosaveSettings {
        self.settings.clone()
    }

    /// Sets autosave settings.
    ///
    /// # Arguments
    ///
    /// * `settings` - New settings to apply
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_autosave_manager::{AutosaveManager, AutosaveSettings};
    ///
    /// let mut manager = AutosaveManager::new();
    /// let new_settings = AutosaveSettings::new();
    /// manager.set_settings(new_settings);
    /// ```
    pub fn set_settings(&mut self, settings: AutosaveSettings) {
        self.settings = settings;
    }
}

impl Default for AutosaveManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // DialogAutosaveSettings tests

    #[test]
    fn test_dialog_autosave_settings_new() {
        let settings = DialogAutosaveSettings::new(true, false, 10 * 1024 * 1024);
        assert!(settings.are_inited);
        assert!(settings.autosave_photos);
        assert!(!settings.autosave_videos);
        assert_eq!(settings.max_video_file_size, 10 * 1024 * 1024);
    }

    #[test]
    fn test_dialog_autosave_settings_default() {
        let settings = DialogAutosaveSettings::default();
        assert!(!settings.are_inited);
        assert!(!settings.autosave_photos);
        assert!(!settings.autosave_videos);
        assert_eq!(settings.max_video_file_size, DEFAULT_MAX_VIDEO_FILE_SIZE);
    }

    #[test]
    fn test_dialog_autosave_settings_constants() {
        assert_eq!(MIN_MAX_VIDEO_FILE_SIZE, 512 * 1024);
        assert_eq!(DEFAULT_MAX_VIDEO_FILE_SIZE, 100 * 1024 * 1024);
        assert_eq!(MAX_MAX_VIDEO_FILE_SIZE, 4000 * 1024 * 1024);
    }

    #[test]
    fn test_max_video_file_size() {
        let settings = DialogAutosaveSettings::default();
        assert_eq!(settings.max_video_file_size(), DEFAULT_MAX_VIDEO_FILE_SIZE);
    }

    #[test]
    fn test_is_initialized() {
        let settings = DialogAutosaveSettings::default();
        assert!(!settings.is_initialized());

        let settings = DialogAutosaveSettings::new(true, false, 10 * 1024 * 1024);
        assert!(settings.is_initialized());
    }

    #[test]
    fn test_set_autosave_photos() {
        let mut settings = DialogAutosaveSettings::default();
        settings.set_autosave_photos(true);
        assert!(settings.autosave_photos);
        assert!(settings.is_initialized());
    }

    #[test]
    fn test_set_autosave_videos() {
        let mut settings = DialogAutosaveSettings::default();
        settings.set_autosave_videos(true);
        assert!(settings.autosave_videos);
        assert!(settings.is_initialized());
    }

    #[test]
    fn test_set_max_video_file_size() {
        let mut settings = DialogAutosaveSettings::default();
        settings.set_max_video_file_size(50 * 1024 * 1024);
        assert_eq!(settings.max_video_file_size, 50 * 1024 * 1024);
    }

    #[test]
    fn test_set_max_video_file_size_clamping() {
        let mut settings = DialogAutosaveSettings::default();

        // Test lower bound
        settings.set_max_video_file_size(100);
        assert_eq!(settings.max_video_file_size, MIN_MAX_VIDEO_FILE_SIZE);

        // Test upper bound
        settings.set_max_video_file_size(10_000 * 1024 * 1024);
        assert_eq!(settings.max_video_file_size, MAX_MAX_VIDEO_FILE_SIZE);
    }

    #[test]
    fn test_dialog_autosave_settings_equality() {
        let settings1 = DialogAutosaveSettings::new(true, false, 10 * 1024 * 1024);
        let settings2 = DialogAutosaveSettings::new(true, false, 10 * 1024 * 1024);
        assert_eq!(settings1, settings2);

        let settings3 = DialogAutosaveSettings::new(false, true, 20 * 1024 * 1024);
        assert_ne!(settings1, settings3);
    }

    #[test]
    fn test_dialog_autosave_settings_clone() {
        let settings1 = DialogAutosaveSettings::new(true, false, 10 * 1024 * 1024);
        let settings2 = settings1.clone();
        assert_eq!(settings1, settings2);
    }

    #[test]
    fn test_dialog_autosave_settings_display() {
        let settings = DialogAutosaveSettings::new(true, false, 10 * 1024 * 1024);
        let display = format!("{}", settings);
        assert!(display.contains("photos=true"));
        assert!(display.contains("videos=false"));
    }

    // AutosaveSettings tests

    #[test]
    fn test_autosave_settings_new() {
        let settings = AutosaveSettings::new();
        assert!(!settings.are_inited);
        assert!(!settings.are_being_reloaded);
        assert!(!settings.need_reload);
        assert_eq!(settings.exceptions_count(), 0);
    }

    #[test]
    fn test_autosave_settings_default() {
        let settings = AutosaveSettings::default();
        assert!(!settings.are_inited);
        assert_eq!(settings.exceptions_count(), 0);
    }

    #[test]
    fn test_autosave_settings_is_initialized() {
        let settings = AutosaveSettings::new();
        assert!(!settings.is_initialized());

        let mut settings = AutosaveSettings::new();
        settings.mark_initialized();
        assert!(settings.is_initialized());
    }

    #[test]
    fn test_mark_initialized() {
        let mut settings = AutosaveSettings::new();
        settings.mark_initialized();
        assert!(settings.is_initialized());
    }

    #[test]
    fn test_user_settings() {
        let settings = AutosaveSettings::new();
        let user_settings = settings.user_settings();
        assert!(!user_settings.is_initialized());
    }

    #[test]
    fn test_chat_settings() {
        let settings = AutosaveSettings::new();
        let chat_settings = settings.chat_settings();
        assert!(!chat_settings.is_initialized());
    }

    #[test]
    fn test_broadcast_settings() {
        let settings = AutosaveSettings::new();
        let broadcast_settings = settings.broadcast_settings();
        assert!(!broadcast_settings.is_initialized());
    }

    #[test]
    fn test_set_user_settings() {
        let mut settings = AutosaveSettings::new();
        let new_user = DialogAutosaveSettings::new(true, false, 10 * 1024 * 1024);
        settings.set_user_settings(new_user);
        assert!(settings.user_settings.autosave_photos);
    }

    #[test]
    fn test_set_chat_settings() {
        let mut settings = AutosaveSettings::new();
        let new_chat = DialogAutosaveSettings::new(false, true, 50 * 1024 * 1024);
        settings.set_chat_settings(new_chat);
        assert!(settings.chat_settings.autosave_videos);
    }

    #[test]
    fn test_set_broadcast_settings() {
        let mut settings = AutosaveSettings::new();
        let new_broadcast = DialogAutosaveSettings::new(true, true, 100 * 1024 * 1024);
        settings.set_broadcast_settings(new_broadcast);
        assert!(settings.broadcast_settings.autosave_photos);
        assert!(settings.broadcast_settings.autosave_videos);
    }

    #[test]
    fn test_add_exception() {
        let mut settings = AutosaveSettings::new();
        let exception = DialogAutosaveSettings::new(true, true, 200 * 1024 * 1024);
        settings.add_exception(DialogId::new(100), exception);
        assert_eq!(settings.exceptions_count(), 1);
    }

    #[test]
    fn test_get_exception() {
        let mut settings = AutosaveSettings::new();
        let exception = DialogAutosaveSettings::new(true, true, 200 * 1024 * 1024);
        settings.add_exception(DialogId::new(100), exception.clone());

        let retrieved = settings.get_exception(DialogId::new(100));
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), &exception);

        let missing = settings.get_exception(DialogId::new(999));
        assert!(missing.is_none());
    }

    #[test]
    fn test_remove_exception() {
        let mut settings = AutosaveSettings::new();
        let exception = DialogAutosaveSettings::new(true, true, 200 * 1024 * 1024);
        settings.add_exception(DialogId::new(100), exception);

        let removed = settings.remove_exception(DialogId::new(100));
        assert!(removed.is_some());
        assert_eq!(settings.exceptions_count(), 0);

        let removed_again = settings.remove_exception(DialogId::new(100));
        assert!(removed_again.is_none());
    }

    #[test]
    fn test_clear_exceptions() {
        let mut settings = AutosaveSettings::new();
        settings.add_exception(DialogId::new(100), DialogAutosaveSettings::default());
        settings.add_exception(DialogId::new(200), DialogAutosaveSettings::default());
        assert_eq!(settings.exceptions_count(), 2);

        settings.clear_exceptions();
        assert_eq!(settings.exceptions_count(), 0);
    }

    #[test]
    fn test_exceptions_count() {
        let mut settings = AutosaveSettings::new();
        assert_eq!(settings.exceptions_count(), 0);

        settings.add_exception(DialogId::new(100), DialogAutosaveSettings::default());
        assert_eq!(settings.exceptions_count(), 1);

        settings.add_exception(DialogId::new(200), DialogAutosaveSettings::default());
        assert_eq!(settings.exceptions_count(), 2);
    }

    #[test]
    fn test_autosave_settings_equality() {
        let settings1 = AutosaveSettings::new();
        let settings2 = AutosaveSettings::new();
        assert_eq!(settings1, settings2);

        let mut settings3 = AutosaveSettings::new();
        settings3.mark_initialized();
        assert_ne!(settings1, settings3);
    }

    #[test]
    fn test_autosave_settings_clone() {
        let mut settings1 = AutosaveSettings::new();
        settings1.mark_initialized();
        settings1.add_exception(DialogId::new(100), DialogAutosaveSettings::default());

        let settings2 = settings1.clone();
        assert_eq!(settings1, settings2);
        assert_eq!(settings2.exceptions_count(), 1);
    }

    #[test]
    fn test_autosave_settings_display() {
        let settings = AutosaveSettings::new();
        let display = format!("{}", settings);
        assert!(display.contains("inited=false"));
    }

    // AutosaveManager tests

    #[test]
    fn test_autosave_manager_new() {
        let manager = AutosaveManager::new();
        assert!(!manager.settings().is_initialized());
    }

    #[test]
    fn test_autosave_manager_default() {
        let manager = AutosaveManager::default();
        assert!(!manager.settings().is_initialized());
    }

    #[test]
    fn test_settings() {
        let manager = AutosaveManager::new();
        let settings = manager.settings();
        assert!(!settings.is_initialized());
    }

    #[test]
    fn test_settings_mut() {
        let mut manager = AutosaveManager::new();
        let settings = manager.settings_mut();
        settings.mark_initialized();
        assert!(manager.settings().is_initialized());
    }

    #[test]
    fn test_get_settings() {
        let manager = AutosaveManager::new();
        let settings = manager.get_settings();
        assert!(!settings.is_initialized());
    }

    #[test]
    fn test_set_settings() {
        let mut manager = AutosaveManager::new();
        let new_settings = AutosaveSettings::new();
        manager.set_settings(new_settings);
        assert_eq!(manager.settings().exceptions_count(), 0);
    }

    #[test]
    fn test_reload_settings() {
        let mut manager = AutosaveManager::new();
        manager.reload_settings();
        // TODO: Verify actual reload behavior when implemented
    }

    #[test]
    fn test_multiple_exceptions() {
        let mut settings = AutosaveSettings::new();
        for i in 1..=10 {
            settings.add_exception(DialogId::new(i), DialogAutosaveSettings::default());
        }
        assert_eq!(settings.exceptions_count(), 10);

        for i in 1..=10 {
            assert!(settings.get_exception(DialogId::new(i)).is_some());
        }
    }

    #[test]
    fn test_full_settings_workflow() {
        let mut settings = AutosaveSettings::new();
        settings.mark_initialized();

        settings.set_user_settings(DialogAutosaveSettings::new(true, false, 10 * 1024 * 1024));
        settings.set_chat_settings(DialogAutosaveSettings::new(false, true, 50 * 1024 * 1024));
        settings.set_broadcast_settings(DialogAutosaveSettings::new(true, true, 100 * 1024 * 1024));

        settings.add_exception(DialogId::new(100), DialogAutosaveSettings::new(true, true, 200 * 1024 * 1024));

        assert!(settings.is_initialized());
        assert!(settings.user_settings.autosave_photos);
        assert!(settings.chat_settings.autosave_videos);
        assert!(settings.broadcast_settings.autosave_photos);
        assert!(settings.broadcast_settings.autosave_videos);
        assert_eq!(settings.exceptions_count(), 1);
    }
}
