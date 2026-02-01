// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Rustgram DialogActionManager
//!
//! Dialog action (typing indicators) management for Telegram MTProto client.
//!
//! This crate provides types and utilities for managing chat actions like
//! typing indicators, recording voice notes, uploading files, etc.
//!
//! ## Overview
//!
//! - [`DialogAction`] - Represents a chat action (typing, recording, uploading, etc.)
//! - [`ActiveDialogAction`] - Tracks an active action with timeout info
//! - [`DialogActionManager`] - Manages active dialog actions
//!
//! ## Examples
//!
//! Creating a typing action:
//!
//! ```
//! use rustgram_dialog_action_manager::{DialogAction, DialogActionManager};
//!
//! let mut manager = DialogActionManager::new();
//! let action = DialogAction::typing();
//! manager.on_dialog_action(1, 2, 3, action, 0);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::collections::HashMap;
use std::time::{Duration, Instant};
use thiserror::Error;

/// Default timeout for dialog actions in seconds.
pub const DIALOG_ACTION_TIMEOUT: Duration = Duration::from_secs(6);

/// Errors that can occur in dialog action operations.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum DialogActionError {
    /// Invalid dialog ID
    #[error("Invalid dialog ID: {0}")]
    InvalidDialogId(i64),

    /// Invalid action type
    #[error("Invalid action type: {0}")]
    InvalidActionType(String),

    /// Action expired
    #[error("Action expired for dialog: {0}")]
    ActionExpired(i64),
}

/// Dialog action type.
///
/// Represents the current state/action of a user in a dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum DialogActionType {
    /// No action (cancels previous action)
    Cancel = 0,

    /// Typing a text message
    Typing = 1,

    /// Recording a video
    RecordingVideo = 2,

    /// Uploading a video
    UploadingVideo = 3,

    /// Recording a voice note
    RecordingVoiceNote = 4,

    /// Uploading a voice note
    UploadingVoiceNote = 5,

    /// Uploading a photo
    UploadingPhoto = 6,

    /// Uploading a document
    UploadingDocument = 7,

    /// Choosing a location
    ChoosingLocation = 8,

    /// Choosing a contact
    ChoosingContact = 9,

    /// Starting to play a game
    StartPlayingGame = 10,

    /// Recording a video note
    RecordingVideoNote = 11,

    /// Uploading a video note
    UploadingVideoNote = 12,

    /// Speaking in a voice chat
    SpeakingInVoiceChat = 13,

    /// Importing messages
    ImportingMessages = 14,

    /// Choosing a sticker
    ChoosingSticker = 15,

    /// Watching animations
    WatchingAnimations = 16,

    /// Clicking an animated emoji
    ClickingAnimatedEmoji = 17,

    /// Text draft (typing with preview)
    TextDraft = 18,
}

impl DialogActionType {
    /// Gets all action types.
    #[must_use]
    pub const fn all() -> [DialogActionType; 19] {
        [
            DialogActionType::Cancel,
            DialogActionType::Typing,
            DialogActionType::RecordingVideo,
            DialogActionType::UploadingVideo,
            DialogActionType::RecordingVoiceNote,
            DialogActionType::UploadingVoiceNote,
            DialogActionType::UploadingPhoto,
            DialogActionType::UploadingDocument,
            DialogActionType::ChoosingLocation,
            DialogActionType::ChoosingContact,
            DialogActionType::StartPlayingGame,
            DialogActionType::RecordingVideoNote,
            DialogActionType::UploadingVideoNote,
            DialogActionType::SpeakingInVoiceChat,
            DialogActionType::ImportingMessages,
            DialogActionType::ChoosingSticker,
            DialogActionType::WatchingAnimations,
            DialogActionType::ClickingAnimatedEmoji,
            DialogActionType::TextDraft,
        ]
    }

    /// Gets the integer representation.
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Gets action type from integer.
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(DialogActionType::Cancel),
            1 => Some(DialogActionType::Typing),
            2 => Some(DialogActionType::RecordingVideo),
            3 => Some(DialogActionType::UploadingVideo),
            4 => Some(DialogActionType::RecordingVoiceNote),
            5 => Some(DialogActionType::UploadingVoiceNote),
            6 => Some(DialogActionType::UploadingPhoto),
            7 => Some(DialogActionType::UploadingDocument),
            8 => Some(DialogActionType::ChoosingLocation),
            9 => Some(DialogActionType::ChoosingContact),
            10 => Some(DialogActionType::StartPlayingGame),
            11 => Some(DialogActionType::RecordingVideoNote),
            12 => Some(DialogActionType::UploadingVideoNote),
            13 => Some(DialogActionType::SpeakingInVoiceChat),
            14 => Some(DialogActionType::ImportingMessages),
            15 => Some(DialogActionType::ChoosingSticker),
            16 => Some(DialogActionType::WatchingAnimations),
            17 => Some(DialogActionType::ClickingAnimatedEmoji),
            18 => Some(DialogActionType::TextDraft),
            _ => None,
        }
    }

    /// Gets the name of the action type.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            DialogActionType::Cancel => "Cancel",
            DialogActionType::Typing => "Typing",
            DialogActionType::RecordingVideo => "RecordingVideo",
            DialogActionType::UploadingVideo => "UploadingVideo",
            DialogActionType::RecordingVoiceNote => "RecordingVoiceNote",
            DialogActionType::UploadingVoiceNote => "UploadingVoiceNote",
            DialogActionType::UploadingPhoto => "UploadingPhoto",
            DialogActionType::UploadingDocument => "UploadingDocument",
            DialogActionType::ChoosingLocation => "ChoosingLocation",
            DialogActionType::ChoosingContact => "ChoosingContact",
            DialogActionType::StartPlayingGame => "StartPlayingGame",
            DialogActionType::RecordingVideoNote => "RecordingVideoNote",
            DialogActionType::UploadingVideoNote => "UploadingVideoNote",
            DialogActionType::SpeakingInVoiceChat => "SpeakingInVoiceChat",
            DialogActionType::ImportingMessages => "ImportingMessages",
            DialogActionType::ChoosingSticker => "ChoosingSticker",
            DialogActionType::WatchingAnimations => "WatchingAnimations",
            DialogActionType::ClickingAnimatedEmoji => "ClickingAnimatedEmoji",
            DialogActionType::TextDraft => "TextDraft",
        }
    }

    /// Checks if this action cancels other actions.
    #[must_use]
    pub fn is_cancel(self) -> bool {
        matches!(self, DialogActionType::Cancel)
    }
}

/// Dialog action.
///
/// Represents a user's current action in a dialog, such as typing,
/// recording, uploading, etc.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_action_manager::{DialogAction, DialogActionType};
///
/// let typing = DialogAction::typing();
/// assert_eq!(typing.action_type(), DialogActionType::Typing);
///
/// let uploading = DialogAction::uploading_video(75);
/// assert_eq!(uploading.progress(), Some(75));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogAction {
    /// Type of action
    action_type: DialogActionType,

    /// Progress for uploading actions (0-100)
    progress: Option<i32>,

    /// Emoji for certain actions (watching animations, clicking animated emoji)
    emoji: Option<String>,

    /// Random ID for text draft actions
    random_id: Option<i64>,
}

impl DialogAction {
    /// Creates a new dialog action.
    #[must_use]
    pub const fn new(action_type: DialogActionType) -> Self {
        Self {
            action_type,
            progress: None,
            emoji: None,
            random_id: None,
        }
    }

    /// Gets the action type.
    #[must_use]
    pub const fn action_type(&self) -> DialogActionType {
        self.action_type
    }

    /// Gets the progress (for uploading actions).
    #[must_use]
    pub const fn progress(&self) -> Option<i32> {
        self.progress
    }

    /// Gets the emoji (for animation-related actions).
    #[must_use]
    pub fn emoji(&self) -> Option<&str> {
        self.emoji.as_deref()
    }

    /// Gets the random ID (for text draft actions).
    #[must_use]
    pub const fn random_id(&self) -> Option<i64> {
        self.random_id
    }

    /// Creates a cancel action.
    #[must_use]
    pub const fn cancel() -> Self {
        Self::new(DialogActionType::Cancel)
    }

    /// Creates a typing action.
    #[must_use]
    pub const fn typing() -> Self {
        Self::new(DialogActionType::Typing)
    }

    /// Creates an uploading video action with progress.
    #[must_use]
    pub const fn uploading_video(progress: i32) -> Self {
        Self {
            action_type: DialogActionType::UploadingVideo,
            progress: Some(progress),
            emoji: None,
            random_id: None,
        }
    }

    /// Creates an uploading photo action with progress.
    #[must_use]
    pub const fn uploading_photo(progress: i32) -> Self {
        Self {
            action_type: DialogActionType::UploadingPhoto,
            progress: Some(progress),
            emoji: None,
            random_id: None,
        }
    }

    /// Creates an uploading document action with progress.
    #[must_use]
    pub const fn uploading_document(progress: i32) -> Self {
        Self {
            action_type: DialogActionType::UploadingDocument,
            progress: Some(progress),
            emoji: None,
            random_id: None,
        }
    }

    /// Creates a recording voice note action.
    #[must_use]
    pub const fn recording_voice_note() -> Self {
        Self::new(DialogActionType::RecordingVoiceNote)
    }

    /// Creates an uploading voice note action with progress.
    #[must_use]
    pub const fn uploading_voice_note(progress: i32) -> Self {
        Self {
            action_type: DialogActionType::UploadingVoiceNote,
            progress: Some(progress),
            emoji: None,
            random_id: None,
        }
    }

    /// Creates a choosing location action.
    #[must_use]
    pub const fn choosing_location() -> Self {
        Self::new(DialogActionType::ChoosingLocation)
    }

    /// Creates a choosing contact action.
    #[must_use]
    pub const fn choosing_contact() -> Self {
        Self::new(DialogActionType::ChoosingContact)
    }

    /// Creates a watching animations action with emoji.
    #[must_use]
    pub fn watching_animations(emoji: String) -> Self {
        Self {
            action_type: DialogActionType::WatchingAnimations,
            progress: None,
            emoji: Some(emoji),
            random_id: None,
        }
    }

    /// Creates a clicking animated emoji action with message ID and emoji.
    #[must_use]
    pub fn clicking_animated_emoji(emoji: String) -> Self {
        Self {
            action_type: DialogActionType::ClickingAnimatedEmoji,
            progress: None,
            emoji: Some(emoji),
            random_id: None,
        }
    }

    /// Creates a text draft action with random ID.
    #[must_use]
    pub const fn text_draft(random_id: i64) -> Self {
        Self {
            action_type: DialogActionType::TextDraft,
            progress: None,
            emoji: None,
            random_id: Some(random_id),
        }
    }

    /// Creates a choosing sticker action.
    #[must_use]
    pub const fn choosing_sticker() -> Self {
        Self::new(DialogActionType::ChoosingSticker)
    }

    /// Checks if this action cancels other actions.
    #[must_use]
    pub fn is_cancel(&self) -> bool {
        matches!(self.action_type, DialogActionType::Cancel)
    }
}

impl Default for DialogAction {
    fn default() -> Self {
        Self::cancel()
    }
}

/// Active dialog action with timeout information.
///
/// Tracks a dialog action with its start time for timeout management.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveDialogAction {
    /// Top thread message ID (for forum topics)
    top_thread_message_id: i64,

    /// Dialog ID of the user performing the action
    typing_dialog_id: i64,

    /// The action being performed
    action: DialogAction,

    /// When the action started
    start_time: Instant,
}

impl ActiveDialogAction {
    /// Creates a new active dialog action.
    #[must_use]
    pub const fn new(
        top_thread_message_id: i64,
        typing_dialog_id: i64,
        action: DialogAction,
        start_time: Instant,
    ) -> Self {
        Self {
            top_thread_message_id,
            typing_dialog_id,
            action,
            start_time,
        }
    }

    /// Gets the top thread message ID.
    #[must_use]
    pub const fn top_thread_message_id(&self) -> i64 {
        self.top_thread_message_id
    }

    /// Gets the typing dialog ID.
    #[must_use]
    pub const fn typing_dialog_id(&self) -> i64 {
        self.typing_dialog_id
    }

    /// Gets the action.
    #[must_use]
    pub const fn action(&self) -> &DialogAction {
        &self.action
    }

    /// Gets the start time.
    #[must_use]
    pub const fn start_time(&self) -> Instant {
        self.start_time
    }

    /// Checks if the action has expired.
    #[must_use]
    pub fn is_expired(&self, timeout: Duration) -> bool {
        self.start_time.elapsed() > timeout
    }
}

/// Dialog action manager.
///
/// Manages active dialog actions (typing indicators) for multiple dialogs.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_action_manager::{DialogActionManager, DialogAction};
/// use std::time::Duration;
///
/// let mut manager = DialogActionManager::new();
///
/// // Register a typing action
/// let action = DialogAction::typing();
/// manager.on_dialog_action(1, 0, 2, action, 0);
///
/// // Get active actions for a dialog
/// let actions = manager.get_active_actions(1);
/// assert!(!actions.is_empty());
///
/// // Clear actions for a dialog
/// manager.clear_active_actions(1);
/// ```
#[derive(Debug, Default)]
pub struct DialogActionManager {
    /// Active dialog actions by dialog ID
    active_actions: HashMap<i64, Vec<ActiveDialogAction>>,
}

impl DialogActionManager {
    /// Creates a new dialog action manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_action_manager::DialogActionManager;
    ///
    /// let manager = DialogActionManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles a dialog action event.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    /// * `top_thread_message_id` - Top thread message ID (0 for non-thread)
    /// * `typing_dialog_id` - Dialog ID of the user performing the action
    /// * `action` - The action being performed
    /// * `date` - Message date (unused in current implementation)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_action_manager::{DialogActionManager, DialogAction};
    ///
    /// let mut manager = DialogActionManager::new();
    /// manager.on_dialog_action(1, 0, 2, DialogAction::typing(), 0);
    /// ```
    pub fn on_dialog_action(
        &mut self,
        dialog_id: i64,
        top_thread_message_id: i64,
        typing_dialog_id: i64,
        action: DialogAction,
        _date: i32,
    ) {
        // If action is cancel, remove all actions for this typing_dialog_id
        if action.is_cancel() {
            self.remove_actions_for_typing_dialog(dialog_id, typing_dialog_id);
            return;
        }

        // Remove existing action from same typing_dialog_id
        self.remove_actions_for_typing_dialog(dialog_id, typing_dialog_id);

        // Add new action
        let active_action = ActiveDialogAction::new(
            top_thread_message_id,
            typing_dialog_id,
            action,
            Instant::now(),
        );

        self.active_actions
            .entry(dialog_id)
            .or_insert_with(Vec::new)
            .push(active_action);
    }

    /// Removes all actions from a specific typing dialog.
    fn remove_actions_for_typing_dialog(&mut self, dialog_id: i64, typing_dialog_id: i64) {
        if let Some(actions) = self.active_actions.get_mut(&dialog_id) {
            actions.retain(|a| a.typing_dialog_id() != typing_dialog_id);
        }
    }

    /// Gets all active actions for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    ///
    /// # Returns
    ///
    /// Vector of active actions (expired actions filtered out)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_action_manager::{DialogActionManager, DialogAction};
    ///
    /// let mut manager = DialogActionManager::new();
    /// manager.on_dialog_action(1, 0, 2, DialogAction::typing(), 0);
    ///
    /// let actions = manager.get_active_actions(1);
    /// assert!(!actions.is_empty());
    /// ```
    #[must_use]
    pub fn get_active_actions(&mut self, dialog_id: i64) -> Vec<ActiveDialogAction> {
        self.cleanup_expired_actions(dialog_id);
        self.active_actions
            .get(&dialog_id)
            .map(|v| v.clone())
            .unwrap_or_default()
    }

    /// Cleans up expired actions for a dialog.
    fn cleanup_expired_actions(&mut self, dialog_id: i64) {
        if let Some(actions) = self.active_actions.get_mut(&dialog_id) {
            actions.retain(|a| !a.is_expired(DIALOG_ACTION_TIMEOUT));
        }
    }

    /// Clears all active actions for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_action_manager::{DialogActionManager, DialogAction};
    ///
    /// let mut manager = DialogActionManager::new();
    /// manager.on_dialog_action(1, 0, 2, DialogAction::typing(), 0);
    ///
    /// manager.clear_active_actions(1);
    /// assert!(manager.get_active_actions(1).is_empty());
    /// ```
    pub fn clear_active_actions(&mut self, dialog_id: i64) {
        self.active_actions.remove(&dialog_id);
    }

    /// Gets the count of active actions across all dialogs.
    ///
    /// # Returns
    ///
    /// Total number of active actions
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_action_manager::{DialogActionManager, DialogAction};
    ///
    /// let mut manager = DialogActionManager::new();
    /// assert_eq!(manager.active_action_count(), 0);
    ///
    /// manager.on_dialog_action(1, 0, 2, DialogAction::typing(), 0);
    /// assert_eq!(manager.active_action_count(), 1);
    /// ```
    #[must_use]
    pub fn active_action_count(&self) -> usize {
        self.active_actions.values().map(|v| v.len()).sum()
    }

    /// Cleans up all expired actions across all dialogs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_action_manager::{DialogActionManager, DialogAction};
    /// use std::time::Duration;
    ///
    /// let mut manager = DialogActionManager::new();
    /// manager.on_dialog_action(1, 0, 2, DialogAction::typing(), 0);
    ///
    /// // After timeout, cleanup will remove the action
    /// std::thread::sleep(Duration::from_secs(7));
    /// manager.cleanup_all_expired();
    /// ```
    pub fn cleanup_all_expired(&mut self) {
        for dialog_id in self.active_actions.keys().copied().collect::<Vec<_>>() {
            self.cleanup_expired_actions(dialog_id);
        }
    }

    /// Clears all active actions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_action_manager::{DialogActionManager, DialogAction};
    ///
    /// let mut manager = DialogActionManager::new();
    /// manager.on_dialog_action(1, 0, 2, DialogAction::typing(), 0);
    /// manager.on_dialog_action(2, 0, 3, DialogAction::typing(), 0);
    ///
    /// manager.clear_all();
    /// assert_eq!(manager.active_action_count(), 0);
    /// ```
    pub fn clear_all(&mut self) {
        self.active_actions.clear();
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-dialog-action-manager";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== DialogActionType Tests ==========

    #[test]
    fn test_action_type_as_i32() {
        assert_eq!(DialogActionType::Cancel.as_i32(), 0);
        assert_eq!(DialogActionType::Typing.as_i32(), 1);
        assert_eq!(DialogActionType::UploadingVideo.as_i32(), 3);
    }

    #[test]
    fn test_action_type_from_i32() {
        assert_eq!(
            DialogActionType::from_i32(0),
            Some(DialogActionType::Cancel)
        );
        assert_eq!(
            DialogActionType::from_i32(1),
            Some(DialogActionType::Typing)
        );
        assert_eq!(DialogActionType::from_i32(999), None);
    }

    #[test]
    fn test_action_type_name() {
        assert_eq!(DialogActionType::Typing.name(), "Typing");
        assert_eq!(DialogActionType::UploadingVideo.name(), "UploadingVideo");
    }

    #[test]
    fn test_action_type_is_cancel() {
        assert!(DialogActionType::Cancel.is_cancel());
        assert!(!DialogActionType::Typing.is_cancel());
    }

    // ========== DialogAction Tests ==========

    #[test]
    fn test_dialog_action_new() {
        let action = DialogAction::new(DialogActionType::Typing);
        assert_eq!(action.action_type(), DialogActionType::Typing);
        assert!(action.progress().is_none());
        assert!(action.emoji().is_none());
    }

    #[test]
    fn test_dialog_action_cancel() {
        let action = DialogAction::cancel();
        assert!(action.is_cancel());
    }

    #[test]
    fn test_dialog_action_typing() {
        let action = DialogAction::typing();
        assert_eq!(action.action_type(), DialogActionType::Typing);
        assert!(!action.is_cancel());
    }

    #[test]
    fn test_dialog_action_uploading_video() {
        let action = DialogAction::uploading_video(75);
        assert_eq!(action.action_type(), DialogActionType::UploadingVideo);
        assert_eq!(action.progress(), Some(75));
    }

    #[test]
    fn test_dialog_action_uploading_photo() {
        let action = DialogAction::uploading_photo(50);
        assert_eq!(action.action_type(), DialogActionType::UploadingPhoto);
        assert_eq!(action.progress(), Some(50));
    }

    #[test]
    fn test_dialog_action_uploading_document() {
        let action = DialogAction::uploading_document(25);
        assert_eq!(action.action_type(), DialogActionType::UploadingDocument);
        assert_eq!(action.progress(), Some(25));
    }

    #[test]
    fn test_dialog_action_recording_voice_note() {
        let action = DialogAction::recording_voice_note();
        assert_eq!(action.action_type(), DialogActionType::RecordingVoiceNote);
    }

    #[test]
    fn test_dialog_action_uploading_voice_note() {
        let action = DialogAction::uploading_voice_note(90);
        assert_eq!(action.action_type(), DialogActionType::UploadingVoiceNote);
        assert_eq!(action.progress(), Some(90));
    }

    #[test]
    fn test_dialog_action_choosing_location() {
        let action = DialogAction::choosing_location();
        assert_eq!(action.action_type(), DialogActionType::ChoosingLocation);
    }

    #[test]
    fn test_dialog_action_choosing_contact() {
        let action = DialogAction::choosing_contact();
        assert_eq!(action.action_type(), DialogActionType::ChoosingContact);
    }

    #[test]
    fn test_dialog_action_watching_animations() {
        let action = DialogAction::watching_animations("😀".to_string());
        assert_eq!(action.action_type(), DialogActionType::WatchingAnimations);
        assert_eq!(action.emoji(), Some("😀"));
    }

    #[test]
    fn test_dialog_action_clicking_animated_emoji() {
        let action = DialogAction::clicking_animated_emoji("🎉".to_string());
        assert_eq!(
            action.action_type(),
            DialogActionType::ClickingAnimatedEmoji
        );
        assert_eq!(action.emoji(), Some("🎉"));
    }

    #[test]
    fn test_dialog_action_text_draft() {
        let action = DialogAction::text_draft(12345);
        assert_eq!(action.action_type(), DialogActionType::TextDraft);
        assert_eq!(action.random_id(), Some(12345));
    }

    #[test]
    fn test_dialog_action_choosing_sticker() {
        let action = DialogAction::choosing_sticker();
        assert_eq!(action.action_type(), DialogActionType::ChoosingSticker);
    }

    #[test]
    fn test_dialog_action_default() {
        let action = DialogAction::default();
        assert!(action.is_cancel());
    }

    // ========== ActiveDialogAction Tests ==========

    #[test]
    fn test_active_dialog_action_new() {
        let action = DialogAction::typing();
        let active = ActiveDialogAction::new(0, 2, action, Instant::now());
        assert_eq!(active.top_thread_message_id(), 0);
        assert_eq!(active.typing_dialog_id(), 2);
        assert_eq!(active.action().action_type(), DialogActionType::Typing);
    }

    #[test]
    fn test_active_dialog_action_is_expired() {
        let action = DialogAction::typing();
        let past_instant = Instant::now() - Duration::from_secs(10);
        let active = ActiveDialogAction::new(0, 2, action, past_instant);
        assert!(active.is_expired(DIALOG_ACTION_TIMEOUT));
    }

    // ========== DialogActionManager Tests ==========

    #[test]
    fn test_manager_new() {
        let manager = DialogActionManager::new();
        assert_eq!(manager.active_action_count(), 0);
    }

    #[test]
    fn test_manager_default() {
        let manager = DialogActionManager::default();
        assert_eq!(manager.active_action_count(), 0);
    }

    #[test]
    fn test_on_dialog_action_typing() {
        let mut manager = DialogActionManager::new();
        manager.on_dialog_action(1, 0, 2, DialogAction::typing(), 0);
        assert_eq!(manager.active_action_count(), 1);
    }

    #[test]
    fn test_on_dialog_action_cancel() {
        let mut manager = DialogActionManager::new();
        manager.on_dialog_action(1, 0, 2, DialogAction::typing(), 0);
        assert_eq!(manager.active_action_count(), 1);

        manager.on_dialog_action(1, 0, 2, DialogAction::cancel(), 0);
        assert_eq!(manager.active_action_count(), 0);
    }

    #[test]
    fn test_on_dialog_action_replace_same_typing_dialog() {
        let mut manager = DialogActionManager::new();
        manager.on_dialog_action(1, 0, 2, DialogAction::typing(), 0);
        manager.on_dialog_action(1, 0, 2, DialogAction::uploading_photo(50), 0);
        assert_eq!(manager.active_action_count(), 1);
    }

    #[test]
    fn test_on_dialog_action_multiple_typing_dialogs() {
        let mut manager = DialogActionManager::new();
        manager.on_dialog_action(1, 0, 2, DialogAction::typing(), 0);
        manager.on_dialog_action(1, 0, 3, DialogAction::typing(), 0);
        assert_eq!(manager.active_action_count(), 2);
    }

    #[test]
    fn test_get_active_actions() {
        let mut manager = DialogActionManager::new();
        manager.on_dialog_action(1, 0, 2, DialogAction::typing(), 0);

        let actions = manager.get_active_actions(1);
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].typing_dialog_id(), 2);
    }

    #[test]
    fn test_get_active_actions_empty() {
        let mut manager = DialogActionManager::new();
        let actions = manager.get_active_actions(1);
        assert!(actions.is_empty());
    }

    #[test]
    fn test_clear_active_actions() {
        let mut manager = DialogActionManager::new();
        manager.on_dialog_action(1, 0, 2, DialogAction::typing(), 0);
        manager.clear_active_actions(1);

        assert_eq!(manager.active_action_count(), 0);
    }

    #[test]
    fn test_active_action_count() {
        let mut manager = DialogActionManager::new();
        assert_eq!(manager.active_action_count(), 0);

        manager.on_dialog_action(1, 0, 2, DialogAction::typing(), 0);
        assert_eq!(manager.active_action_count(), 1);

        manager.on_dialog_action(2, 0, 3, DialogAction::typing(), 0);
        assert_eq!(manager.active_action_count(), 2);
    }

    #[test]
    fn test_clear_all() {
        let mut manager = DialogActionManager::new();
        manager.on_dialog_action(1, 0, 2, DialogAction::typing(), 0);
        manager.on_dialog_action(2, 0, 3, DialogAction::typing(), 0);

        manager.clear_all();
        assert_eq!(manager.active_action_count(), 0);
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-dialog-action-manager");
    }
}
