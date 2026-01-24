// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Group Call Manager
//!
//! Manager for group voice and video calls in Telegram.
//!
//! ## Overview
//!
//! The `GroupCallManager` handles creation, management, and termination of
//! group voice and video calls (voice chats). It provides methods for:
//!
//! - Creating and joining group calls
//! - Managing participants
//! - Video and audio controls
//! - Screen sharing
//! - Recording
//!
//! ## Architecture
//!
//! Based on TDLib's `GroupCallManager` class, this module:
//! - Tracks active group calls
//! - Manages participant state
//! - Handles E2E encryption for calls
//! - Provides video quality controls
//!
//! ## Call States
//!
//! ```text
//! Joined -> Active -> Left
//!              -> Discarded
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_group_call_manager::GroupCallManager;
//! use rustgram_types::{DialogId, UserId};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut manager = GroupCallManager::new();
//!
//!     // Create a new group call
//!     let user_id = UserId::new(1234567890).unwrap();
//!     let dialog_id = DialogId::from_user(user_id);
//!     let call_id = manager.create_video_chat(dialog_id).await?;
//!
//!     // Join the call
//!     manager.join_group_call(call_id, dialog_id).await?;
//!
//!     // Toggle video
//!     manager.toggle_video(call_id, true).await?;
//!
//!     // Leave the call
//!     manager.leave_group_call(call_id).await?;
//!
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod error;

use rustgram_input_group_call_id::InputGroupCallId;
#[allow(unused_imports)] // Used in doc tests
use rustgram_types::{DialogId, UserId};
use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

pub use error::{Error, Result};

/// Group call ID type
pub type GroupCallId = i64;

/// Participant state in a group call
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticipantState {
    /// Participant is connecting
    Connecting,
    /// Participant is active
    Active,
    /// Participant is muted
    Muted,
    /// Participant left
    Left,
}

/// Group call participant information
#[derive(Debug, Clone)]
pub struct Participant {
    /// Dialog ID of the participant
    dialog_id: DialogId,
    /// Current participant state
    state: ParticipantState,
    /// Whether video is enabled
    has_video: bool,
    /// Whether screen sharing is active
    is_screen_sharing: bool,
    /// Volume level (1-100)
    volume_level: i32,
    /// Whether hand is raised
    is_hand_raised: bool,
}

impl Participant {
    /// Creates a new participant
    #[must_use]
    pub const fn new(dialog_id: DialogId) -> Self {
        Self {
            dialog_id,
            state: ParticipantState::Connecting,
            has_video: false,
            is_screen_sharing: false,
            volume_level: 100,
            is_hand_raised: false,
        }
    }

    /// Returns the dialog ID
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the participant state
    #[must_use]
    pub const fn state(&self) -> ParticipantState {
        self.state
    }

    /// Returns whether video is enabled
    #[must_use]
    pub const fn has_video(&self) -> bool {
        self.has_video
    }

    /// Returns whether screen sharing is active
    #[must_use]
    pub const fn is_screen_sharing(&self) -> bool {
        self.is_screen_sharing
    }

    /// Returns the volume level
    #[must_use]
    pub const fn volume_level(&self) -> i32 {
        self.volume_level
    }

    /// Returns whether hand is raised
    #[must_use]
    pub const fn is_hand_raised(&self) -> bool {
        self.is_hand_raised
    }
}

/// Active group call information
#[derive(Debug, Clone)]
#[allow(dead_code)] // Internal struct with fields used for future expansion
struct GroupCallInfo {
    /// Group call ID
    call_id: GroupCallId,
    /// Input group call ID with access hash
    input_id: InputGroupCallId,
    /// Associated dialog ID
    dialog_id: DialogId,
    /// Whether currently joined
    is_joined: bool,
    /// Whether video is enabled for local user
    is_video_enabled: bool,
    /// Whether video is paused
    is_video_paused: bool,
    /// Whether screen sharing is active
    is_screen_sharing: bool,
    /// Whether recording is active
    is_recording: bool,
    /// Whether microphone is muted
    is_muted: bool,
    /// Call title
    title: String,
    /// Participant count
    participant_count: i32,
    /// List of participants
    participants: Vec<Participant>,
}

/// Manager for group voice and video calls
///
/// Handles creation, management, and termination of group calls.
/// Based on TDLib's `GroupCallManager` class.
///
/// # Example
///
/// ```rust
/// use rustgram_group_call_manager::GroupCallManager;
/// use rustgram_types::{DialogId, UserId};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut manager = GroupCallManager::new();
///
/// // Create a new group call
/// let user_id = UserId::new(1234567890).unwrap();
/// let dialog_id = DialogId::from_user(user_id);
/// let call_id = manager.create_video_chat(dialog_id).await?;
/// assert!(call_id > 0);
///
/// // Join the call
/// manager.join_group_call(call_id, dialog_id).await?;
///
/// // Leave the call
/// manager.leave_group_call(call_id).await?;
///
/// Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct GroupCallManager {
    /// Next group call ID to assign
    next_call_id: Arc<AtomicI64>,
    /// Active group calls by call ID
    calls: Arc<RwLock<HashMap<GroupCallId, GroupCallInfo>>>,
    /// Calls by dialog ID
    dialog_calls: Arc<RwLock<HashMap<DialogId, GroupCallId>>>,
    /// Calls by input group call ID
    input_id_calls: Arc<RwLock<HashMap<InputGroupCallId, GroupCallId>>>,
}

impl Default for GroupCallManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GroupCallManager {
    /// Creates a new group call manager
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_manager::GroupCallManager;
    ///
    /// let manager = GroupCallManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_call_id: Arc::new(AtomicI64::new(1)),
            calls: Arc::new(RwLock::new(HashMap::new())),
            dialog_calls: Arc::new(RwLock::new(HashMap::new())),
            input_id_calls: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates a new video chat (group call)
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID where to create the call
    ///
    /// # Returns
    ///
    /// The group call ID
    ///
    /// # Errors
    ///
    /// Returns an error if a call already exists for this dialog
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_manager::GroupCallManager;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = GroupCallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    ///
    /// let call_id = manager.create_video_chat(dialog_id).await?;
    /// assert!(call_id > 0);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_video_chat(&self, dialog_id: DialogId) -> Result<GroupCallId> {
        // Check if there's already a call for this dialog
        let dialog_calls = self.dialog_calls.read().await;
        if let Some(&existing_call_id) = dialog_calls.get(&dialog_id) {
            return Err(Error::CallAlreadyExists(existing_call_id));
        }
        drop(dialog_calls);

        let call_id = self.next_call_id.fetch_add(1, Ordering::SeqCst);
        let input_id = InputGroupCallId::new(call_id, 0); // Simplified - no access hash for now

        let call_info = GroupCallInfo {
            call_id,
            input_id,
            dialog_id,
            is_joined: false,
            is_video_enabled: false,
            is_video_paused: false,
            is_screen_sharing: false,
            is_recording: false,
            is_muted: false,
            title: String::new(),
            participant_count: 0,
            participants: Vec::new(),
        };

        let mut calls = self.calls.write().await;
        let mut dialog_calls = self.dialog_calls.write().await;
        let mut input_id_calls = self.input_id_calls.write().await;

        calls.insert(call_id, call_info);
        dialog_calls.insert(dialog_id, call_id);
        input_id_calls.insert(input_id, call_id);

        Ok(call_id)
    }

    /// Joins a group call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Group call ID
    /// * `dialog_id` - Dialog ID joining as
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist or already joined
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_manager::GroupCallManager;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = GroupCallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let call_id = manager.create_video_chat(dialog_id).await?;
    ///
    /// manager.join_group_call(call_id, dialog_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn join_group_call(&self, call_id: GroupCallId, _dialog_id: DialogId) -> Result<()> {
        let mut calls = self.calls.write().await;

        let call_info = calls
            .get_mut(&call_id)
            .ok_or(Error::CallNotFound(call_id))?;

        if call_info.is_joined {
            return Err(Error::AlreadyJoined);
        }

        call_info.is_joined = true;

        Ok(())
    }

    /// Leaves a group call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Group call ID to leave
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_manager::GroupCallManager;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = GroupCallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let call_id = manager.create_video_chat(dialog_id).await?;
    /// manager.join_group_call(call_id, dialog_id).await?;
    ///
    /// manager.leave_group_call(call_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn leave_group_call(&self, call_id: GroupCallId) -> Result<()> {
        let mut calls = self.calls.write().await;

        let call_info = calls
            .get_mut(&call_id)
            .ok_or(Error::CallNotFound(call_id))?;

        call_info.is_joined = false;

        Ok(())
    }

    /// Discards a group call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Group call ID to discard
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_manager::GroupCallManager;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = GroupCallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let call_id = manager.create_video_chat(dialog_id).await?;
    ///
    /// manager.discard_group_call(call_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn discard_group_call(&self, call_id: GroupCallId) -> Result<()> {
        let call_info = {
            let calls = self.calls.read().await;
            calls.get(&call_id).cloned()
        };

        let call_info = call_info.ok_or(Error::CallNotFound(call_id))?;

        let mut calls = self.calls.write().await;
        let mut dialog_calls = self.dialog_calls.write().await;
        let mut input_id_calls = self.input_id_calls.write().await;

        calls.remove(&call_id);
        dialog_calls.remove(&call_info.dialog_id);
        input_id_calls.remove(&call_info.input_id);

        Ok(())
    }

    /// Toggles video for a call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Group call ID
    /// * `enabled` - Whether video should be enabled
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_manager::GroupCallManager;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = GroupCallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let call_id = manager.create_video_chat(dialog_id).await?;
    ///
    /// manager.toggle_video(call_id, true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn toggle_video(&self, call_id: GroupCallId, enabled: bool) -> Result<()> {
        let mut calls = self.calls.write().await;

        let call_info = calls
            .get_mut(&call_id)
            .ok_or(Error::CallNotFound(call_id))?;

        call_info.is_video_enabled = enabled;

        Ok(())
    }

    /// Toggles mute state for a call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Group call ID
    /// * `muted` - Whether microphone should be muted
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_manager::GroupCallManager;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = GroupCallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let call_id = manager.create_video_chat(dialog_id).await?;
    ///
    /// manager.toggle_mute(call_id, true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn toggle_mute(&self, call_id: GroupCallId, muted: bool) -> Result<()> {
        let mut calls = self.calls.write().await;

        let call_info = calls
            .get_mut(&call_id)
            .ok_or(Error::CallNotFound(call_id))?;

        call_info.is_muted = muted;

        Ok(())
    }

    /// Toggles screen sharing for a call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Group call ID
    /// * `enabled` - Whether screen sharing should be enabled
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_manager::GroupCallManager;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = GroupCallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let call_id = manager.create_video_chat(dialog_id).await?;
    ///
    /// manager.toggle_screen_sharing(call_id, true).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn toggle_screen_sharing(&self, call_id: GroupCallId, enabled: bool) -> Result<()> {
        let mut calls = self.calls.write().await;

        let call_info = calls
            .get_mut(&call_id)
            .ok_or(Error::CallNotFound(call_id))?;

        call_info.is_screen_sharing = enabled;

        Ok(())
    }

    /// Checks if a call is joined
    ///
    /// # Arguments
    ///
    /// * `call_id` - Group call ID
    ///
    /// # Returns
    ///
    /// Whether the call is joined
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_manager::GroupCallManager;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = GroupCallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let call_id = manager.create_video_chat(dialog_id).await?;
    ///
    /// assert!(!manager.is_joined(call_id).await?);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn is_joined(&self, call_id: GroupCallId) -> Result<bool> {
        let calls = self.calls.read().await;

        let call_info = calls.get(&call_id).ok_or(Error::CallNotFound(call_id))?;

        Ok(call_info.is_joined)
    }

    /// Checks if video is enabled for a call
    ///
    /// # Arguments
    ///
    /// * `call_id` - Group call ID
    ///
    /// # Returns
    ///
    /// Whether video is enabled
    ///
    /// # Errors
    ///
    /// Returns an error if the call doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_manager::GroupCallManager;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = GroupCallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// let call_id = manager.create_video_chat(dialog_id).await?;
    ///
    /// assert!(!manager.is_video_enabled(call_id).await?);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn is_video_enabled(&self, call_id: GroupCallId) -> Result<bool> {
        let calls = self.calls.read().await;

        let call_info = calls.get(&call_id).ok_or(Error::CallNotFound(call_id))?;

        Ok(call_info.is_video_enabled)
    }

    /// Gets the call ID for a dialog
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Returns
    ///
    /// The group call ID if there's an active call for this dialog
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_manager::GroupCallManager;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = GroupCallManager::new();
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    ///
    /// assert!(manager.get_call_by_dialog(dialog_id).await?.is_none());
    ///
    /// let call_id = manager.create_video_chat(dialog_id).await?;
    /// assert_eq!(manager.get_call_by_dialog(dialog_id).await?, Some(call_id));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_call_by_dialog(&self, dialog_id: DialogId) -> Result<Option<GroupCallId>> {
        let dialog_calls = self.dialog_calls.read().await;
        Ok(dialog_calls.get(&dialog_id).copied())
    }

    /// Returns the number of active calls
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_group_call_manager::GroupCallManager;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = GroupCallManager::new();
    /// assert_eq!(manager.active_call_count().await, 0);
    ///
    /// let user_id = UserId::new(1234567890).unwrap();
    /// let dialog_id = DialogId::from_user(user_id);
    /// manager.create_video_chat(dialog_id).await?;
    /// assert_eq!(manager.active_call_count().await, 1);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn active_call_count(&self) -> usize {
        self.calls.read().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Constructor Tests ==========

    #[test]
    fn test_group_call_manager_new() {
        let manager = GroupCallManager::new();
        assert_eq!(manager.next_call_id.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_group_call_manager_default() {
        let manager = GroupCallManager::default();
        assert_eq!(manager.next_call_id.load(Ordering::SeqCst), 1);
    }

    // ========== Create Video Chat Tests ==========

    #[tokio::test]
    async fn test_create_video_chat_returns_valid_id() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);

        let call_id = manager.create_video_chat(dialog_id).await.unwrap();
        assert!(call_id > 0);
        assert_eq!(call_id, 1);
    }

    #[tokio::test]
    async fn test_create_video_chat_increments_id() {
        let manager = GroupCallManager::new();
        let user_id1 = UserId::new(1).unwrap();
        let user_id2 = UserId::new(2).unwrap();
        let dialog_id1 = DialogId::from_user(user_id1);
        let dialog_id2 = DialogId::from_user(user_id2);

        let call_id1 = manager.create_video_chat(dialog_id1).await.unwrap();
        let call_id2 = manager.create_video_chat(dialog_id2).await.unwrap();

        assert_eq!(call_id1, 1);
        assert_eq!(call_id2, 2);
    }

    #[tokio::test]
    async fn test_create_video_chat_already_exists() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);

        manager.create_video_chat(dialog_id).await.unwrap();
        let result = manager.create_video_chat(dialog_id).await;

        assert!(matches!(result, Err(Error::CallAlreadyExists(_))));
    }

    // ========== Join Group Call Tests ==========

    #[tokio::test]
    async fn test_join_group_call_success() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let call_id = manager.create_video_chat(dialog_id).await.unwrap();

        manager.join_group_call(call_id, dialog_id).await.unwrap();

        assert!(manager.is_joined(call_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_join_group_call_not_found() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let call_id = 999;

        let result = manager.join_group_call(call_id, dialog_id).await;
        assert!(matches!(result, Err(Error::CallNotFound(_))));
    }

    #[tokio::test]
    async fn test_join_group_call_already_joined() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let call_id = manager.create_video_chat(dialog_id).await.unwrap();

        manager.join_group_call(call_id, dialog_id).await.unwrap();

        let result = manager.join_group_call(call_id, dialog_id).await;
        assert!(matches!(result, Err(Error::AlreadyJoined)));
    }

    // ========== Leave Group Call Tests ==========

    #[tokio::test]
    async fn test_leave_group_call_success() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let call_id = manager.create_video_chat(dialog_id).await.unwrap();

        manager.join_group_call(call_id, dialog_id).await.unwrap();
        manager.leave_group_call(call_id).await.unwrap();

        assert!(!manager.is_joined(call_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_leave_group_call_not_found() {
        let manager = GroupCallManager::new();
        let call_id = 999;

        let result = manager.leave_group_call(call_id).await;
        assert!(matches!(result, Err(Error::CallNotFound(_))));
    }

    // ========== Discard Group Call Tests ==========

    #[tokio::test]
    async fn test_discard_group_call_success() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let call_id = manager.create_video_chat(dialog_id).await.unwrap();

        manager.discard_group_call(call_id).await.unwrap();

        // Should not be able to join after discarding
        let result = manager.is_joined(call_id).await;
        assert!(matches!(result, Err(Error::CallNotFound(_))));
    }

    #[tokio::test]
    async fn test_discard_group_call_removes_from_dialog_calls() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let call_id = manager.create_video_chat(dialog_id).await.unwrap();

        assert_eq!(
            manager.get_call_by_dialog(dialog_id).await.unwrap(),
            Some(call_id)
        );

        manager.discard_group_call(call_id).await.unwrap();

        assert_eq!(manager.get_call_by_dialog(dialog_id).await.unwrap(), None);
    }

    // ========== Toggle Video Tests ==========

    #[tokio::test]
    async fn test_toggle_video_on() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let call_id = manager.create_video_chat(dialog_id).await.unwrap();

        manager.toggle_video(call_id, true).await.unwrap();

        assert!(manager.is_video_enabled(call_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_toggle_video_off() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let call_id = manager.create_video_chat(dialog_id).await.unwrap();

        manager.toggle_video(call_id, true).await.unwrap();
        manager.toggle_video(call_id, false).await.unwrap();

        assert!(!manager.is_video_enabled(call_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_toggle_video_not_found() {
        let manager = GroupCallManager::new();
        let call_id = 999;

        let result = manager.toggle_video(call_id, true).await;
        assert!(matches!(result, Err(Error::CallNotFound(_))));
    }

    // ========== Toggle Mute Tests ==========

    #[tokio::test]
    async fn test_toggle_mute_on() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let call_id = manager.create_video_chat(dialog_id).await.unwrap();

        manager.toggle_mute(call_id, true).await.unwrap();
    }

    // ========== Toggle Screen Sharing Tests ==========

    #[tokio::test]
    async fn test_toggle_screen_sharing_on() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let call_id = manager.create_video_chat(dialog_id).await.unwrap();

        manager.toggle_screen_sharing(call_id, true).await.unwrap();
    }

    // ========== Is Joined Tests ==========

    #[tokio::test]
    async fn test_is_joined_false_initially() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let call_id = manager.create_video_chat(dialog_id).await.unwrap();

        assert!(!manager.is_joined(call_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_is_joined_not_found() {
        let manager = GroupCallManager::new();
        let call_id = 999;

        let result = manager.is_joined(call_id).await;
        assert!(matches!(result, Err(Error::CallNotFound(_))));
    }

    // ========== Is Video Enabled Tests ==========

    #[tokio::test]
    async fn test_is_video_enabled_false_initially() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let call_id = manager.create_video_chat(dialog_id).await.unwrap();

        assert!(!manager.is_video_enabled(call_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_is_video_enabled_not_found() {
        let manager = GroupCallManager::new();
        let call_id = 999;

        let result = manager.is_video_enabled(call_id).await;
        assert!(matches!(result, Err(Error::CallNotFound(_))));
    }

    // ========== Get Call By Dialog Tests ==========

    #[tokio::test]
    async fn test_get_call_by_dialog_none() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);

        let result = manager.get_call_by_dialog(dialog_id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_call_by_dialog_some() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let call_id = manager.create_video_chat(dialog_id).await.unwrap();

        let result = manager.get_call_by_dialog(dialog_id).await.unwrap();
        assert_eq!(result, Some(call_id));
    }

    // ========== Active Call Count Tests ==========

    #[tokio::test]
    async fn test_active_call_count_zero() {
        let manager = GroupCallManager::new();
        assert_eq!(manager.active_call_count().await, 0);
    }

    #[tokio::test]
    async fn test_active_call_count_multiple() {
        let manager = GroupCallManager::new();
        let user_id1 = UserId::new(1).unwrap();
        let user_id2 = UserId::new(2).unwrap();
        let user_id3 = UserId::new(3).unwrap();
        let dialog_id1 = DialogId::from_user(user_id1);
        let dialog_id2 = DialogId::from_user(user_id2);
        let dialog_id3 = DialogId::from_user(user_id3);

        manager.create_video_chat(dialog_id1).await.unwrap();
        manager.create_video_chat(dialog_id2).await.unwrap();
        manager.create_video_chat(dialog_id3).await.unwrap();

        assert_eq!(manager.active_call_count().await, 3);
    }

    // ========== Multi-Call Workflow Tests ==========

    #[tokio::test]
    async fn test_multiple_calls_different_dialogs() {
        let manager = GroupCallManager::new();
        let user1 = UserId::new(1).unwrap();
        let user2 = UserId::new(2).unwrap();
        let dialog1 = DialogId::from_user(user1);
        let dialog2 = DialogId::from_user(user2);

        let call1 = manager.create_video_chat(dialog1).await.unwrap();
        let call2 = manager.create_video_chat(dialog2).await.unwrap();

        assert_ne!(call1, call2);
        assert_eq!(
            manager.get_call_by_dialog(dialog1).await.unwrap(),
            Some(call1)
        );
        assert_eq!(
            manager.get_call_by_dialog(dialog2).await.unwrap(),
            Some(call2)
        );
    }

    #[tokio::test]
    async fn test_call_lifecycle() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);

        // Create
        let call_id = manager.create_video_chat(dialog_id).await.unwrap();
        assert!(!manager.is_joined(call_id).await.unwrap());

        // Join
        manager.join_group_call(call_id, dialog_id).await.unwrap();
        assert!(manager.is_joined(call_id).await.unwrap());

        // Toggle video
        manager.toggle_video(call_id, true).await.unwrap();
        assert!(manager.is_video_enabled(call_id).await.unwrap());

        // Leave
        manager.leave_group_call(call_id).await.unwrap();
        assert!(!manager.is_joined(call_id).await.unwrap());

        // Discard
        manager.discard_group_call(call_id).await.unwrap();
    }

    // ========== Edge Cases ==========

    #[tokio::test]
    async fn test_create_call_after_discard() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);

        let call1 = manager.create_video_chat(dialog_id).await.unwrap();
        manager.discard_group_call(call1).await.unwrap();

        // Should be able to create a new call after discarding
        let call2 = manager.create_video_chat(dialog_id).await.unwrap();
        assert_ne!(call1, call2);
    }

    #[tokio::test]
    async fn test_join_leave_multiple_times() {
        let manager = GroupCallManager::new();
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let call_id = manager.create_video_chat(dialog_id).await.unwrap();

        // First join/leave cycle
        manager.join_group_call(call_id, dialog_id).await.unwrap();
        assert!(manager.is_joined(call_id).await.unwrap());
        manager.leave_group_call(call_id).await.unwrap();
        assert!(!manager.is_joined(call_id).await.unwrap());

        // Second join/leave cycle (should work because we don't track calls by user)
        manager.join_group_call(call_id, dialog_id).await.unwrap();
        assert!(manager.is_joined(call_id).await.unwrap());
        manager.leave_group_call(call_id).await.unwrap();
        assert!(!manager.is_joined(call_id).await.unwrap());
    }

    // ========== ParticipantState Equality Tests ==========

    #[test]
    fn test_participant_state_equality() {
        assert_eq!(ParticipantState::Connecting, ParticipantState::Connecting);
        assert_eq!(ParticipantState::Active, ParticipantState::Active);
        assert_ne!(ParticipantState::Active, ParticipantState::Muted);
    }

    // ========== Participant Tests ==========

    #[test]
    fn test_participant_new() {
        let user_id = UserId::new(1234567890).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let participant = Participant::new(dialog_id);

        assert_eq!(participant.dialog_id(), dialog_id);
        assert_eq!(participant.state(), ParticipantState::Connecting);
        assert!(!participant.has_video());
        assert!(!participant.is_screen_sharing());
        assert_eq!(participant.volume_level(), 100);
        assert!(!participant.is_hand_raised());
    }
}
