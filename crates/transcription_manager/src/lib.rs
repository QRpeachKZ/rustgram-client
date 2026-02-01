// Copyright 2025 rustgram-client contributors
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

//! # Transcription Manager
//!
//! Manages audio/video message transcriptions.
//!
//! ## Overview
//!
//! This module provides functionality for managing speech recognition
//! and transcription of audio/video messages in Telegram.
//!
//! ## TDLib Correspondence
//!
//! Corresponds to `td/telegram/TranscriptionManager.h` in TDLib.
//!
//! ## Example
//!
//! ```rust
//! use rustgram-transcription_manager::{TranscriptionManager, TrialParameters};
//! use rustgram_file_id::FileId;
//! use rustgram_message_content_type::MessageContentType;
//! use rustgram_message_full_id::MessageFullId;
//!
//! let mut manager = TranscriptionManager::new();
//! manager.update_trial_parameters(10, 60, 0);
//! let file_id = FileId::new(123, 0);
//! let content_type = MessageContentType::VoiceNote;
//! let message_id = MessageFullId::new(
//!     rustgram_types::DialogId::from_user(
//!         rustgram_types::UserId::new(1).unwrap()
//!     ),
//!     rustgram_types::MessageId::from_server_id(1)
//! );
//! manager.register_voice(file_id, content_type, message_id, "test");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_file_id::FileId;
use rustgram_message_content_type::MessageContentType;
use rustgram_message_full_id::MessageFullId;
use rustgram_transcription_info::TranscriptionInfo;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, Ordering};
use std::sync::RwLock;
use thiserror::Error;
use tracing::{debug, info, warn};

/// Errors that can occur in the transcription manager.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TranscriptionError {
    /// Transcription not found for the given file ID
    #[error("Transcription not found for file {0:?}")]
    TranscriptionNotFound(FileId),

    /// Invalid file ID
    #[error("Invalid file ID: {0:?}")]
    InvalidFileId(FileId),

    /// Transcription failed
    #[error("Transcription failed: {0}")]
    TranscriptionFailed(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded: {0} tries remaining")]
    RateLimitExceeded(i32),

    /// Invalid content type for transcription
    #[error("Invalid content type for transcription: {0:?}")]
    InvalidContentType(MessageContentType),
}

/// Result type for transcription operations.
pub type TranscriptionResult<T> = Result<T, TranscriptionError>;

/// Trial parameters for speech recognition.
///
/// Tracks the weekly trial quota and usage for speech recognition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialParameters {
    /// Weekly number of transcriptions allowed
    weekly_number: i32,
    /// Maximum duration in seconds for each transcription
    duration_max: i32,
    /// Number of tries remaining this week
    left_tries: i32,
    /// Unix timestamp when the weekly quota resets
    next_reset_date: i32,
}

impl Default for TrialParameters {
    fn default() -> Self {
        Self {
            weekly_number: 0,
            duration_max: 0,
            left_tries: 0,
            next_reset_date: 0,
        }
    }
}

impl TrialParameters {
    /// Creates new trial parameters.
    ///
    /// # Arguments
    ///
    /// * `weekly_number` - Number of transcriptions allowed per week
    /// * `duration_max` - Maximum duration in seconds per transcription
    /// * `left_tries` - Number of tries remaining
    /// * `next_reset_date` - Unix timestamp when quota resets
    #[must_use]
    pub const fn new(
        weekly_number: i32,
        duration_max: i32,
        left_tries: i32,
        next_reset_date: i32,
    ) -> Self {
        Self {
            weekly_number,
            duration_max,
            left_tries,
            next_reset_date,
        }
    }

    /// Returns the weekly number of allowed transcriptions.
    #[must_use]
    pub const fn weekly_number(&self) -> i32 {
        self.weekly_number
    }

    /// Returns the maximum duration in seconds.
    #[must_use]
    pub const fn duration_max(&self) -> i32 {
        self.duration_max
    }

    /// Returns the number of tries remaining this week.
    #[must_use]
    pub const fn left_tries(&self) -> i32 {
        self.left_tries
    }

    /// Returns the next reset date as Unix timestamp.
    #[must_use]
    pub const fn next_reset_date(&self) -> i32 {
        self.next_reset_date
    }

    /// Updates the left tries based on current time and reset date.
    pub fn update_left_tries(&mut self, current_time: i32) {
        if current_time >= self.next_reset_date {
            self.left_tries = self.weekly_number;
        }
    }

    /// Returns `true` if transcription is allowed (tries remaining > 0).
    #[must_use]
    pub const fn can_transcribe(&self) -> bool {
        self.left_tries > 0
    }
}

/// Manager for audio/video message transcriptions.
///
/// This manager handles:
/// - Registration/unregistration of voice messages for transcription
/// - Starting and managing transcription operations
/// - Rating transcription results
/// - Managing trial parameters and rate limiting
///
/// # Thread Safety
///
/// This manager uses `RwLock` for internal state and can be safely
/// shared across threads.
#[derive(Debug)]
pub struct TranscriptionManager {
    /// Trial parameters for speech recognition
    trial_parameters: RwLock<TrialParameters>,
    /// Voice messages registered for transcription
    /// Maps FileId -> Set of MessageFullId
    voice_messages: RwLock<HashMap<FileId, HashSet<MessageFullId>>>,
    /// Maps MessageFullId -> (FileId, MessageContentType)
    message_file_ids: RwLock<HashMap<MessageFullId, (FileId, MessageContentType)>>,
    /// Transcription info for each file
    transcriptions: RwLock<HashMap<FileId, TranscriptionInfo>>,
    /// Next transcription ID
    next_transcription_id: AtomicI64,
    /// Whether get_difference is running
    running_get_difference: AtomicBool,
}

impl Default for TranscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TranscriptionManager {
    /// Audio transcription timeout in seconds.
    pub const AUDIO_TRANSCRIPTION_TIMEOUT: i32 = 60;

    /// Creates a new transcription manager.
    #[must_use]
    pub fn new() -> Self {
        info!("Creating new TranscriptionManager");
        Self {
            trial_parameters: RwLock::new(TrialParameters::default()),
            voice_messages: RwLock::new(HashMap::new()),
            message_file_ids: RwLock::new(HashMap::new()),
            transcriptions: RwLock::new(HashMap::new()),
            next_transcription_id: AtomicI64::new(1),
            running_get_difference: AtomicBool::new(false),
        }
    }

    /// Updates trial parameters for speech recognition.
    ///
    /// # Arguments
    ///
    /// * `weekly_number` - Number of transcriptions allowed per week
    /// * `duration_max` - Maximum duration in seconds per transcription
    /// * `cooldown_until` - Unix timestamp when cooldown ends
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram-transcription_manager::TranscriptionManager;
    ///
    /// let manager = TranscriptionManager::new();
    /// manager.update_trial_parameters(10, 60, 0);
    /// ```
    pub fn update_trial_parameters(
        &self,
        weekly_number: i32,
        duration_max: i32,
        cooldown_until: i32,
    ) {
        let mut params = self.trial_parameters.write().unwrap();
        *params = TrialParameters::new(weekly_number, duration_max, weekly_number, cooldown_until);
        debug!(
            "Updated trial parameters: weekly={}, max_duration={}, reset_at={}",
            weekly_number, duration_max, cooldown_until
        );
    }

    /// Registers a voice message for transcription.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file ID to register
    /// * `content_type` - The message content type (must be VoiceNote or Audio)
    /// * `message_full_id` - The full message ID
    /// * `source` - Source of the registration (for logging)
    ///
    /// # Errors
    ///
    /// Returns an error if the content type is not valid for transcription.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram-transcription_manager::TranscriptionManager;
    /// use rustgram_file_id::FileId;
    /// use rustgram_message_content_type::MessageContentType;
    /// use rustgram_message_full_id::MessageFullId;
    ///
    /// let manager = TranscriptionManager::new();
    /// let file_id = FileId::new(123, 0);
    /// let content_type = MessageContentType::VoiceNote;
    /// let message_id = MessageFullId::new(
    ///     rustgram_types::DialogId::from_user(
    ///         rustgram_types::UserId::new(1).unwrap()
    ///     ),
    ///     rustgram_types::MessageId::from_server_id(1)
    /// );
    /// manager.register_voice(file_id, content_type, message_id, "test");
    /// ```
    pub fn register_voice(
        &self,
        file_id: FileId,
        content_type: MessageContentType,
        message_full_id: MessageFullId,
        source: &str,
    ) -> TranscriptionResult<()> {
        if !file_id.is_valid() {
            return Err(TranscriptionError::InvalidFileId(file_id));
        }

        if !matches!(
            content_type,
            MessageContentType::VoiceNote | MessageContentType::Audio
        ) {
            return Err(TranscriptionError::InvalidContentType(content_type));
        }

        {
            let mut voice_messages = self.voice_messages.write().unwrap();
            voice_messages
                .entry(file_id)
                .or_insert_with(HashSet::new)
                .insert(message_full_id);
        }

        {
            let mut message_file_ids = self.message_file_ids.write().unwrap();
            message_file_ids.insert(message_full_id, (file_id, content_type));
        }

        debug!(
            "Registered voice message: file_id={:?}, message_full_id={:?}, source={}",
            file_id, message_full_id, source
        );
        Ok(())
    }

    /// Unregisters a voice message from transcription.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file ID to unregister
    /// * `content_type` - The message content type
    /// * `message_full_id` - The full message ID
    /// * `source` - Source of the unregistration (for logging)
    pub fn unregister_voice(
        &self,
        file_id: FileId,
        content_type: MessageContentType,
        message_full_id: MessageFullId,
        source: &str,
    ) {
        {
            let mut voice_messages = self.voice_messages.write().unwrap();
            if let Some(messages) = voice_messages.get_mut(&file_id) {
                messages.remove(&message_full_id);
                if messages.is_empty() {
                    voice_messages.remove(&file_id);
                }
            }
        }

        {
            let mut message_file_ids = self.message_file_ids.write().unwrap();
            message_file_ids.remove(&message_full_id);
        }

        debug!(
            "Unregistered voice message: file_id={:?}, message_full_id={:?}, source={}",
            file_id, message_full_id, source
        );
    }

    /// Recognizes speech for a message.
    ///
    /// # Arguments
    ///
    /// * `message_full_id` - The full message ID to transcribe
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Rate limit is exceeded
    /// - File ID is not found
    /// - Transcription fails
    pub fn recognize_speech(&self, message_full_id: MessageFullId) -> TranscriptionResult<i64> {
        // Check rate limit
        {
            let params = self.trial_parameters.read().unwrap();
            if !params.can_transcribe() {
                return Err(TranscriptionError::RateLimitExceeded(params.left_tries()));
            }
        }

        // Get file ID
        let (file_id, content_type) = {
            let message_file_ids = self.message_file_ids.read().unwrap();
            match message_file_ids.get(&message_full_id) {
                Some(&data) => data,
                None => return Err(TranscriptionError::TranscriptionNotFound(FileId::empty())),
            }
        };

        // Generate transcription ID
        let transcription_id = self.next_transcription_id.fetch_add(1, Ordering::SeqCst);

        // Initialize transcription
        {
            let mut transcriptions = self.transcriptions.write().unwrap();
            transcriptions
                .entry(file_id)
                .or_insert_with(TranscriptionInfo::new)
                .start_transcription(transcription_id);
        }

        // Decrement tries
        {
            let mut params = self.trial_parameters.write().unwrap();
            if params.left_tries > 0 {
                params.left_tries -= 1;
            }
        }

        info!(
            "Started speech recognition: message_full_id={:?}, transcription_id={}, content_type={:?}",
            message_full_id, transcription_id, content_type
        );

        Ok(transcription_id)
    }

    /// Marks transcription as completed.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file ID
    /// * `text` - The transcribed text
    /// * `transcription_id` - The transcription ID
    pub fn on_transcription_completed(&self, file_id: FileId, text: String, transcription_id: i64) {
        let mut transcriptions = self.transcriptions.write().unwrap();
        if let Some(info) = transcriptions.get_mut(&file_id) {
            info.complete_transcription(text, transcription_id);
            info!(
                "Transcription completed: file_id={:?}, transcription_id={}",
                file_id, transcription_id
            );
        }
    }

    /// Rates a speech recognition result.
    ///
    /// # Arguments
    ///
    /// * `message_full_id` - The full message ID
    /// * `is_good` - Whether the transcription was good
    ///
    /// # Errors
    ///
    /// Returns an error if the transcription is not found.
    pub fn rate_speech_recognition(
        &self,
        message_full_id: MessageFullId,
        is_good: bool,
    ) -> TranscriptionResult<()> {
        let (file_id, _) = {
            let message_file_ids = self.message_file_ids.read().unwrap();
            match message_file_ids.get(&message_full_id) {
                Some(&data) => data,
                None => return Err(TranscriptionError::TranscriptionNotFound(FileId::empty())),
            }
        };

        {
            let transcriptions = self.transcriptions.read().unwrap();
            if transcriptions.contains_key(&file_id) {
                debug!(
                    "Rated speech recognition: message_full_id={:?}, is_good={}",
                    message_full_id, is_good
                );
                Ok(())
            } else {
                Err(TranscriptionError::TranscriptionNotFound(file_id))
            }
        }
    }

    /// Gets trial parameters.
    #[must_use]
    pub fn get_trial_parameters(&self) -> TrialParameters {
        let params = self.trial_parameters.read().unwrap();
        params.clone()
    }

    /// Gets transcription info for a file.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The file ID
    ///
    /// # Returns
    ///
    /// `None` if no transcription exists for the file.
    #[must_use]
    pub fn get_transcription_info(&self, file_id: FileId) -> Option<TranscriptionInfo> {
        let transcriptions = self.transcriptions.read().unwrap();
        transcriptions.get(&file_id).cloned()
    }

    /// Returns whether get_difference is currently running.
    #[must_use]
    pub fn running_get_difference(&self) -> bool {
        self.running_get_difference.load(Ordering::Acquire)
    }

    /// Sets whether get_difference is running.
    pub fn set_running_get_difference(&self, running: bool) {
        self.running_get_difference
            .store(running, Ordering::Release);
    }

    /// Gets the number of pending transcriptions.
    #[must_use]
    pub fn get_pending_transcription_count(&self) -> usize {
        let transcriptions = self.transcriptions.read().unwrap();
        transcriptions
            .values()
            .filter(|t| !t.is_transcribed() && !t.has_error())
            .count()
    }

    /// Gets all registered file IDs.
    #[must_use]
    pub fn get_registered_file_ids(&self) -> Vec<FileId> {
        let voice_messages = self.voice_messages.read().unwrap();
        voice_messages.keys().copied().collect()
    }

    /// Gets message IDs for a file.
    #[must_use]
    pub fn get_message_ids_for_file(&self, file_id: FileId) -> Vec<MessageFullId> {
        let voice_messages = self.voice_messages.read().unwrap();
        voice_messages
            .get(&file_id)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::{DialogId, MessageId, UserId};

    fn create_test_message_full_id(user_id: i64, message_id: i32) -> MessageFullId {
        MessageFullId::new(
            DialogId::from_user(UserId::new(user_id).unwrap()),
            MessageId::from_server_id(message_id),
        )
    }

    #[test]
    fn test_manager_creation() {
        let manager = TranscriptionManager::new();
        assert!(!manager.running_get_difference());
        assert_eq!(manager.get_pending_transcription_count(), 0);
    }

    #[test]
    fn test_trial_parameters_default() {
        let params = TrialParameters::default();
        assert_eq!(params.weekly_number(), 0);
        assert_eq!(params.duration_max(), 0);
        assert_eq!(params.left_tries(), 0);
        assert!(!params.can_transcribe());
    }

    #[test]
    fn test_trial_parameters_new() {
        let params = TrialParameters::new(10, 60, 5, 100);
        assert_eq!(params.weekly_number(), 10);
        assert_eq!(params.duration_max(), 60);
        assert_eq!(params.left_tries(), 5);
        assert!(params.can_transcribe());
    }

    #[test]
    fn test_update_trial_parameters() {
        let manager = TranscriptionManager::new();
        manager.update_trial_parameters(10, 60, 0);

        let params = manager.get_trial_parameters();
        assert_eq!(params.weekly_number(), 10);
        assert_eq!(params.duration_max(), 60);
        assert!(params.can_transcribe());
    }

    #[test]
    fn test_update_left_tries() {
        let mut params = TrialParameters::new(10, 60, 5, 100);
        params.update_left_tries(50); // Before reset
        assert_eq!(params.left_tries(), 5);

        params.update_left_tries(150); // After reset
        assert_eq!(params.left_tries(), 10);
    }

    #[test]
    fn test_register_voice_valid() {
        let manager = TranscriptionManager::new();
        let file_id = FileId::new(123, 0);
        let content_type = MessageContentType::VoiceNote;
        let message_id = create_test_message_full_id(1, 1);

        let result = manager.register_voice(file_id, content_type, message_id, "test");
        assert!(result.is_ok());

        let file_ids = manager.get_registered_file_ids();
        assert_eq!(file_ids.len(), 1);
        assert_eq!(file_ids[0], file_id);
    }

    #[test]
    fn test_register_voice_invalid_file_id() {
        let manager = TranscriptionManager::new();
        let file_id = FileId::empty();
        let content_type = MessageContentType::VoiceNote;
        let message_id = create_test_message_full_id(1, 1);

        let result = manager.register_voice(file_id, content_type, message_id, "test");
        assert!(matches!(result, Err(TranscriptionError::InvalidFileId(_))));
    }

    #[test]
    fn test_register_voice_invalid_content_type() {
        let manager = TranscriptionManager::new();
        let file_id = FileId::new(123, 0);
        let content_type = MessageContentType::Text;
        let message_id = create_test_message_full_id(1, 1);

        let result = manager.register_voice(file_id, content_type, message_id, "test");
        assert!(matches!(
            result,
            Err(TranscriptionError::InvalidContentType(_))
        ));
    }

    #[test]
    fn test_register_audio() {
        let manager = TranscriptionManager::new();
        let file_id = FileId::new(123, 0);
        let content_type = MessageContentType::Audio;
        let message_id = create_test_message_full_id(1, 1);

        let result = manager.register_voice(file_id, content_type, message_id, "test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_unregister_voice() {
        let manager = TranscriptionManager::new();
        let file_id = FileId::new(123, 0);
        let content_type = MessageContentType::VoiceNote;
        let message_id = create_test_message_full_id(1, 1);

        manager
            .register_voice(file_id, content_type, message_id, "test")
            .unwrap();
        manager.unregister_voice(file_id, content_type, message_id, "test");

        let file_ids = manager.get_registered_file_ids();
        assert_eq!(file_ids.len(), 0);
    }

    #[test]
    fn test_recognize_speech_success() {
        let manager = TranscriptionManager::new();
        manager.update_trial_parameters(10, 60, 0);

        let file_id = FileId::new(123, 0);
        let content_type = MessageContentType::VoiceNote;
        let message_id = create_test_message_full_id(1, 1);

        manager
            .register_voice(file_id, content_type, message_id, "test")
            .unwrap();

        let result = manager.recognize_speech(message_id);
        assert!(result.is_ok());

        let transcription_id = result.unwrap();
        assert!(transcription_id > 0);

        let params = manager.get_trial_parameters();
        assert_eq!(params.left_tries(), 9); // Decremented by 1
    }

    #[test]
    fn test_recognize_speech_rate_limited() {
        let manager = TranscriptionManager::new();
        manager.update_trial_parameters(10, 60, 0);

        let file_id = FileId::new(123, 0);
        let content_type = MessageContentType::VoiceNote;
        let message_id = create_test_message_full_id(1, 1);

        manager
            .register_voice(file_id, content_type, message_id, "test")
            .unwrap();

        // Use all tries
        for _ in 0..10 {
            let _ = manager.recognize_speech(message_id);
        }

        let result = manager.recognize_speech(message_id);
        assert!(matches!(
            result,
            Err(TranscriptionError::RateLimitExceeded(0))
        ));
    }

    #[test]
    fn test_recognize_speech_not_found() {
        let manager = TranscriptionManager::new();
        manager.update_trial_parameters(10, 60, 0);

        let message_id = create_test_message_full_id(1, 1);

        let result = manager.recognize_speech(message_id);
        assert!(matches!(
            result,
            Err(TranscriptionError::TranscriptionNotFound(_))
        ));
    }

    #[test]
    fn test_on_transcription_completed() {
        let manager = TranscriptionManager::new();
        let file_id = FileId::new(123, 0);
        let content_type = MessageContentType::VoiceNote;
        let message_id = create_test_message_full_id(1, 1);

        manager
            .register_voice(file_id, content_type, message_id, "test")
            .unwrap();
        let transcription_id = manager.recognize_speech(message_id).unwrap();

        manager.on_transcription_completed(file_id, "Hello world".to_string(), transcription_id);

        let info = manager.get_transcription_info(file_id);
        assert!(info.is_some());
        assert!(info.unwrap().is_transcribed());
    }

    #[test]
    fn test_rate_speech_recognition() {
        let manager = TranscriptionManager::new();
        let file_id = FileId::new(123, 0);
        let content_type = MessageContentType::VoiceNote;
        let message_id = create_test_message_full_id(1, 1);

        manager
            .register_voice(file_id, content_type, message_id, "test")
            .unwrap();
        let transcription_id = manager.recognize_speech(message_id).unwrap();
        manager.on_transcription_completed(file_id, "Test".to_string(), transcription_id);

        let result = manager.rate_speech_recognition(message_id, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_rate_speech_recognition_not_found() {
        let manager = TranscriptionManager::new();
        let message_id = create_test_message_full_id(1, 1);

        let result = manager.rate_speech_recognition(message_id, true);
        assert!(matches!(
            result,
            Err(TranscriptionError::TranscriptionNotFound(_))
        ));
    }

    #[test]
    fn test_get_message_ids_for_file() {
        let manager = TranscriptionManager::new();
        let file_id = FileId::new(123, 0);
        let content_type = MessageContentType::VoiceNote;

        let message_id1 = create_test_message_full_id(1, 1);
        let message_id2 = create_test_message_full_id(1, 2);

        manager
            .register_voice(file_id, content_type, message_id1, "test")
            .unwrap();
        manager
            .register_voice(file_id, content_type, message_id2, "test")
            .unwrap();

        let message_ids = manager.get_message_ids_for_file(file_id);
        assert_eq!(message_ids.len(), 2);
    }

    #[test]
    fn test_get_pending_transcription_count() {
        let manager = TranscriptionManager::new();
        manager.update_trial_parameters(10, 60, 0);

        let file_id1 = FileId::new(123, 0);
        let file_id2 = FileId::new(124, 0);
        let content_type = MessageContentType::VoiceNote;
        let message_id1 = create_test_message_full_id(1, 1);
        let message_id2 = create_test_message_full_id(1, 2);

        manager
            .register_voice(file_id1, content_type, message_id1, "test")
            .unwrap();
        manager
            .register_voice(file_id2, content_type, message_id2, "test")
            .unwrap();

        let id1 = manager.recognize_speech(message_id1).unwrap();
        let id2 = manager.recognize_speech(message_id2).unwrap();

        assert_eq!(manager.get_pending_transcription_count(), 2);

        manager.on_transcription_completed(file_id1, "Done".to_string(), id1);

        assert_eq!(manager.get_pending_transcription_count(), 1);

        manager.on_transcription_completed(file_id2, "Done".to_string(), id2);

        assert_eq!(manager.get_pending_transcription_count(), 0);
    }

    #[test]
    fn test_running_get_difference() {
        let manager = TranscriptionManager::new();
        assert!(!manager.running_get_difference());

        manager.set_running_get_difference(true);
        assert!(manager.running_get_difference());

        manager.set_running_get_difference(false);
        assert!(!manager.running_get_difference());
    }

    #[test]
    fn test_multiple_messages_same_file() {
        let manager = TranscriptionManager::new();
        let file_id = FileId::new(123, 0);
        let content_type = MessageContentType::VoiceNote;

        let message_id1 = create_test_message_full_id(1, 1);
        let message_id2 = create_test_message_full_id(1, 2);
        let message_id3 = create_test_message_full_id(1, 3);

        manager
            .register_voice(file_id, content_type, message_id1, "test")
            .unwrap();
        manager
            .register_voice(file_id, content_type, message_id2, "test")
            .unwrap();
        manager
            .register_voice(file_id, content_type, message_id3, "test")
            .unwrap();

        let message_ids = manager.get_message_ids_for_file(file_id);
        assert_eq!(message_ids.len(), 3);

        // Unregister one message
        manager.unregister_voice(file_id, content_type, message_id2, "test");

        let message_ids = manager.get_message_ids_for_file(file_id);
        assert_eq!(message_ids.len(), 2);

        // File still registered
        let file_ids = manager.get_registered_file_ids();
        assert_eq!(file_ids.len(), 1);
    }

    #[test]
    fn test_audio_transcription_timeout_const() {
        assert_eq!(TranscriptionManager::AUDIO_TRANSCRIPTION_TIMEOUT, 60);
    }
}
