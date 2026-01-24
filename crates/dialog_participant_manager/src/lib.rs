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

//! # Dialog Participant Manager
//!
//! Manager for dialog participants in group chats and channels.
//!
//! ## TDLib Alignment
//!
//! This module provides functionality similar to TDLib's `ChatManager::get_chat_participant`
//! and related methods, but as a simplified standalone manager.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_dialog_participant_manager::{DialogParticipantManager, ParticipantError};
//! use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
//! use rustgram_types::{ChatId, UserId};
//!
//! let mut manager = DialogParticipantManager::new();
//!
//! let chat_id = ChatId::new(123).unwrap();
//! let user_id = UserId::new(456).unwrap();
//!
//! let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
//! manager.add_participant(chat_id, participant).unwrap();
//!
//! assert!(manager.has_participant(chat_id, user_id));
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
use rustgram_types::{ChatId, UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub mod error;
pub use error::{ParticipantError, Result};

/// Manager for dialog participants.
///
/// Stores and manages participants for multiple dialogs (chats/channels).
#[derive(Debug, Clone)]
pub struct DialogParticipantManager {
    /// Map of chat_id -> (user_id -> DialogParticipant)
    participants: HashMap<ChatId, HashMap<UserId, DialogParticipant>>,
}

impl Default for DialogParticipantManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DialogParticipantManager {
    /// Creates a new participant manager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_manager::DialogParticipantManager;
    ///
    /// let manager = DialogParticipantManager::new();
    /// assert_eq!(manager.participant_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            participants: HashMap::new(),
        }
    }

    /// Adds a participant to a chat.
    ///
    /// # Errors
    ///
    /// Returns `ParticipantError::AlreadyExists` if the participant already exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_manager::{DialogParticipantManager, ParticipantError};
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::{ChatId, UserId};
    ///
    /// let mut manager = DialogParticipantManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    /// let user_id = UserId::new(456).unwrap();
    ///
    /// let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
    /// manager.add_participant(chat_id, participant).unwrap();
    /// ```
    pub fn add_participant(
        &mut self,
        chat_id: ChatId,
        participant: DialogParticipant,
    ) -> Result<()> {
        let chat_participants = self.participants.entry(chat_id).or_default();

        if let Some(user_id) = participant.user_id() {
            if chat_participants.contains_key(&user_id) {
                return Err(ParticipantError::AlreadyExists { chat_id, user_id });
            }
            chat_participants.insert(user_id, participant);
        }

        Ok(())
    }

    /// Removes a participant from a chat.
    ///
    /// # Errors
    ///
    /// Returns `ParticipantError::NotFound` if the participant doesn't exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_manager::{DialogParticipantManager, ParticipantError};
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::{ChatId, UserId};
    ///
    /// let mut manager = DialogParticipantManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    /// let user_id = UserId::new(456).unwrap();
    ///
    /// let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
    /// manager.add_participant(chat_id, participant.clone()).unwrap();
    ///
    /// manager.remove_participant(chat_id, user_id).unwrap();
    /// assert!(!manager.has_participant(chat_id, user_id));
    /// ```
    pub fn remove_participant(&mut self, chat_id: ChatId, user_id: UserId) -> Result<()> {
        let chat_participants = self
            .participants
            .get_mut(&chat_id)
            .ok_or(ParticipantError::ChatNotFound { chat_id })?;

        chat_participants
            .remove(&user_id)
            .ok_or(ParticipantError::NotFound { chat_id, user_id })?;

        Ok(())
    }

    /// Gets a participant from a chat.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_manager::DialogParticipantManager;
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::{ChatId, UserId};
    ///
    /// let mut manager = DialogParticipantManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    /// let user_id = UserId::new(456).unwrap();
    ///
    /// let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Creator);
    /// manager.add_participant(chat_id, participant.clone()).unwrap();
    ///
    /// let retrieved = manager.get_participant(chat_id, user_id).unwrap();
    /// assert!(retrieved.status().is_creator());
    /// ```
    pub fn get_participant(&self, chat_id: ChatId, user_id: UserId) -> Option<&DialogParticipant> {
        self.participants
            .get(&chat_id)
            .and_then(|chat| chat.get(&user_id))
    }

    /// Checks if a participant exists in a chat.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_manager::DialogParticipantManager;
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::{ChatId, UserId};
    ///
    /// let mut manager = DialogParticipantManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    /// let user_id = UserId::new(456).unwrap();
    ///
    /// let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
    /// manager.add_participant(chat_id, participant).unwrap();
    ///
    /// assert!(manager.has_participant(chat_id, user_id));
    /// ```
    pub fn has_participant(&self, chat_id: ChatId, user_id: UserId) -> bool {
        self.get_participant(chat_id, user_id).is_some()
    }

    /// Gets all participants for a chat.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_manager::DialogParticipantManager;
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::{ChatId, UserId};
    ///
    /// let mut manager = DialogParticipantManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    ///
    /// for i in 1..=3 {
    ///     let user_id = UserId::new(i).unwrap();
    ///     let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
    ///     manager.add_participant(chat_id, participant).unwrap();
    /// }
    ///
    /// let participants = manager.get_participants(chat_id);
    /// assert_eq!(participants.len(), 3);
    /// ```
    pub fn get_participants(&self, chat_id: ChatId) -> Vec<&DialogParticipant> {
        self.participants
            .get(&chat_id)
            .map(|chat| chat.values().collect())
            .unwrap_or_default()
    }

    /// Gets the count of participants in a chat.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_manager::DialogParticipantManager;
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::{ChatId, UserId};
    ///
    /// let mut manager = DialogParticipantManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    ///
    /// let user_id = UserId::new(456).unwrap();
    /// let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
    /// manager.add_participant(chat_id, participant).unwrap();
    ///
    /// assert_eq!(manager.participant_count(chat_id), 1);
    /// ```
    pub fn participant_count(&self, chat_id: ChatId) -> usize {
        self.participants
            .get(&chat_id)
            .map(|chat| chat.len())
            .unwrap_or(0)
    }

    /// Updates a participant's status.
    ///
    /// # Errors
    ///
    /// Returns `ParticipantError::NotFound` if the participant doesn't exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_manager::DialogParticipantManager;
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::{ChatId, UserId};
    ///
    /// let mut manager = DialogParticipantManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    /// let user_id = UserId::new(456).unwrap();
    ///
    /// let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
    /// manager.add_participant(chat_id, participant).unwrap();
    ///
    /// manager.update_status(chat_id, user_id, ParticipantStatus::Administrator).unwrap();
    /// let updated = manager.get_participant(chat_id, user_id).unwrap();
    /// assert!(updated.status().is_administrator());
    /// ```
    pub fn update_status(
        &mut self,
        chat_id: ChatId,
        user_id: UserId,
        status: ParticipantStatus,
    ) -> Result<()> {
        let chat_participants = self
            .participants
            .get_mut(&chat_id)
            .ok_or(ParticipantError::ChatNotFound { chat_id })?;

        let participant = chat_participants
            .get_mut(&user_id)
            .ok_or(ParticipantError::NotFound { chat_id, user_id })?;

        participant.set_status(status);
        Ok(())
    }

    /// Removes all participants for a chat.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_manager::DialogParticipantManager;
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::{ChatId, UserId};
    ///
    /// let mut manager = DialogParticipantManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    ///
    /// let user_id = UserId::new(456).unwrap();
    /// let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
    /// manager.add_participant(chat_id, participant).unwrap();
    ///
    /// manager.clear_chat(chat_id);
    /// assert_eq!(manager.participant_count(chat_id), 0);
    /// ```
    pub fn clear_chat(&mut self, chat_id: ChatId) {
        self.participants.remove(&chat_id);
    }

    /// Gets the total number of chats with participants.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_manager::DialogParticipantManager;
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::{ChatId, UserId};
    ///
    /// let mut manager = DialogParticipantManager::new();
    ///
    /// for i in 1..=3 {
    ///     let chat_id = ChatId::new(i).unwrap();
    ///     let user_id = UserId::new(i + 100).unwrap();
    ///     let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
    ///     manager.add_participant(chat_id, participant).unwrap();
    /// }
    ///
    /// assert_eq!(manager.chat_count(), 3);
    /// ```
    pub fn chat_count(&self) -> usize {
        self.participants.len()
    }

    /// Checks if a chat has any participants.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_participant_manager::DialogParticipantManager;
    /// use rustgram_dialog_participant::{DialogParticipant, ParticipantStatus};
    /// use rustgram_types::{ChatId, UserId};
    ///
    /// let mut manager = DialogParticipantManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    ///
    /// assert!(!manager.has_chat(chat_id));
    ///
    /// let user_id = UserId::new(456).unwrap();
    /// let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
    /// manager.add_participant(chat_id, participant).unwrap();
    ///
    /// assert!(manager.has_chat(chat_id));
    /// ```
    pub fn has_chat(&self, chat_id: ChatId) -> bool {
        self.participants.contains_key(&chat_id)
    }
}

impl Serialize for DialogParticipantManager {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.participants.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DialogParticipantManager {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let participants = HashMap::deserialize(deserializer)?;
        Ok(Self { participants })
    }
}

impl fmt::Display for DialogParticipantManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DialogParticipantManager(chats={}, participants={})",
            self.chat_count(),
            self.participants
                .values()
                .map(|chat| chat.len())
                .sum::<usize>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_user(id: i64) -> UserId {
        UserId::new(id).unwrap()
    }

    fn create_test_chat(id: i64) -> ChatId {
        ChatId::new(id).unwrap()
    }

    // Creation tests (2)
    #[test]
    fn test_new() {
        let manager = DialogParticipantManager::new();
        assert_eq!(manager.chat_count(), 0);
        assert_eq!(manager.participant_count(create_test_chat(1)), 0);
    }

    #[test]
    fn test_default() {
        let manager = DialogParticipantManager::default();
        assert_eq!(manager.chat_count(), 0);
    }

    // Add participant tests (4)
    #[test]
    fn test_add_participant() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        let result = manager.add_participant(chat_id, participant);
        assert!(result.is_ok());
        assert!(manager.has_participant(chat_id, user_id));
    }

    #[test]
    fn test_add_participant_duplicate() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        manager
            .add_participant(chat_id, participant.clone())
            .unwrap();

        let result = manager.add_participant(chat_id, participant);
        assert!(matches!(
            result,
            Err(ParticipantError::AlreadyExists { .. })
        ));
    }

    #[test]
    fn test_add_multiple_participants_same_chat() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);

        for i in 1..=5 {
            let user_id = create_test_user(i);
            let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
            manager.add_participant(chat_id, participant).unwrap();
        }

        assert_eq!(manager.participant_count(chat_id), 5);
    }

    #[test]
    fn test_add_participants_different_chats() {
        let mut manager = DialogParticipantManager::new();

        for chat_i in 1..=3 {
            let chat_id = create_test_chat(chat_i);
            for user_i in 1..=2 {
                let user_id = create_test_user(chat_i * 100 + user_i);
                let participant =
                    DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
                manager.add_participant(chat_id, participant).unwrap();
            }
        }

        assert_eq!(manager.chat_count(), 3);
    }

    // Remove participant tests (3)
    #[test]
    fn test_remove_participant() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        manager.add_participant(chat_id, participant).unwrap();

        let result = manager.remove_participant(chat_id, user_id);
        assert!(result.is_ok());
        assert!(!manager.has_participant(chat_id, user_id));
    }

    #[test]
    fn test_remove_participant_not_found() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        let result = manager.remove_participant(chat_id, user_id);
        assert!(matches!(result, Err(ParticipantError::NotFound { .. })));
    }

    #[test]
    fn test_remove_participant_chat_not_found() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        // Add to different chat
        let other_chat = create_test_chat(789);
        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        manager.add_participant(other_chat, participant).unwrap();

        let result = manager.remove_participant(chat_id, user_id);
        assert!(matches!(result, Err(ParticipantError::ChatNotFound { .. })));
    }

    // Get participant tests (3)
    #[test]
    fn test_get_participant() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        let participant =
            DialogParticipant::new(user_id, None, 1234567890, ParticipantStatus::Creator);
        manager
            .add_participant(chat_id, participant.clone())
            .unwrap();

        let retrieved = manager.get_participant(chat_id, user_id);
        assert_eq!(retrieved.unwrap().joined_date(), 1234567890);
    }

    #[test]
    fn test_get_participant_not_found() {
        let manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        assert!(manager.get_participant(chat_id, user_id).is_none());
    }

    #[test]
    fn test_get_participant_wrong_chat() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        manager.add_participant(chat_id, participant).unwrap();

        let other_chat = create_test_chat(789);
        assert!(manager.get_participant(other_chat, user_id).is_none());
    }

    // Has participant tests (2)
    #[test]
    fn test_has_participant_true() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        manager.add_participant(chat_id, participant).unwrap();

        assert!(manager.has_participant(chat_id, user_id));
    }

    #[test]
    fn test_has_participant_false() {
        let manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        assert!(!manager.has_participant(chat_id, user_id));
    }

    // Get participants tests (2)
    #[test]
    fn test_get_participants_empty() {
        let manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);

        assert!(manager.get_participants(chat_id).is_empty());
    }

    #[test]
    fn test_get_participants_multiple() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);

        for i in 1..=3 {
            let user_id = create_test_user(i);
            let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
            manager.add_participant(chat_id, participant).unwrap();
        }

        let participants = manager.get_participants(chat_id);
        assert_eq!(participants.len(), 3);
    }

    // Participant count tests (2)
    #[test]
    fn test_participant_count_empty() {
        let manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);

        assert_eq!(manager.participant_count(chat_id), 0);
    }

    #[test]
    fn test_participant_count() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);

        for i in 1..=5 {
            let user_id = create_test_user(i);
            let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
            manager.add_participant(chat_id, participant).unwrap();
        }

        assert_eq!(manager.participant_count(chat_id), 5);
    }

    // Update status tests (3)
    #[test]
    fn test_update_status() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        manager.add_participant(chat_id, participant).unwrap();

        manager
            .update_status(chat_id, user_id, ParticipantStatus::Administrator)
            .unwrap();

        let updated = manager.get_participant(chat_id, user_id).unwrap();
        assert!(updated.status().is_administrator());
    }

    #[test]
    fn test_update_status_not_found() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        let result = manager.update_status(chat_id, user_id, ParticipantStatus::Creator);
        assert!(matches!(result, Err(ParticipantError::NotFound { .. })));
    }

    #[test]
    fn test_update_status_to_banned() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        manager.add_participant(chat_id, participant).unwrap();

        manager
            .update_status(
                chat_id,
                user_id,
                ParticipantStatus::Banned { until_date: 0 },
            )
            .unwrap();

        let updated = manager.get_participant(chat_id, user_id).unwrap();
        assert!(updated.status().is_banned());
    }

    // Clear chat tests (2)
    #[test]
    fn test_clear_chat() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);

        for i in 1..=5 {
            let user_id = create_test_user(i);
            let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
            manager.add_participant(chat_id, participant).unwrap();
        }

        manager.clear_chat(chat_id);
        assert_eq!(manager.participant_count(chat_id), 0);
        assert!(!manager.has_chat(chat_id));
    }

    #[test]
    fn test_clear_chat_nonexistent() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);

        // Should not panic
        manager.clear_chat(chat_id);
        assert_eq!(manager.chat_count(), 0);
    }

    // Chat count tests (2)
    #[test]
    fn test_chat_count_empty() {
        let manager = DialogParticipantManager::new();
        assert_eq!(manager.chat_count(), 0);
    }

    #[test]
    fn test_chat_count() {
        let mut manager = DialogParticipantManager::new();

        for i in 1..=3 {
            let chat_id = create_test_chat(i);
            let user_id = create_test_user(i + 100);
            let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
            manager.add_participant(chat_id, participant).unwrap();
        }

        assert_eq!(manager.chat_count(), 3);
    }

    // Has chat tests (2)
    #[test]
    fn test_has_chat_true() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
        manager.add_participant(chat_id, participant).unwrap();

        assert!(manager.has_chat(chat_id));
    }

    #[test]
    fn test_has_chat_false() {
        let manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);

        assert!(!manager.has_chat(chat_id));
    }

    // Display tests (1)
    #[test]
    fn test_display() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);

        for i in 1..=3 {
            let user_id = create_test_user(i);
            let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
            manager.add_participant(chat_id, participant).unwrap();
        }

        let display = format!("{}", manager);
        assert!(display.contains("DialogParticipantManager"));
        assert!(display.contains("chats=1"));
        assert!(display.contains("participants=3"));
    }

    // Clone tests (1)
    #[test]
    fn test_clone() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);
        let user_id = create_test_user(456);

        let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Creator);
        manager.add_participant(chat_id, participant).unwrap();

        let cloned = manager.clone();
        assert!(cloned.has_participant(chat_id, user_id));
    }

    // Serialization tests (2)
    #[test]
    fn test_serialize_deserialize() {
        let mut manager = DialogParticipantManager::new();
        let chat_id = create_test_chat(123);

        for i in 1..=3 {
            let user_id = create_test_user(i);
            let participant = DialogParticipant::new(user_id, None, 0, ParticipantStatus::Member);
            manager.add_participant(chat_id, participant).unwrap();
        }

        let json = serde_json::to_string(&manager).unwrap();
        let deserialized: DialogParticipantManager = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.chat_count(), 1);
        assert_eq!(deserialized.participant_count(chat_id), 3);
    }

    #[test]
    fn test_serialize_empty() {
        let manager = DialogParticipantManager::new();

        let json = serde_json::to_string(&manager).unwrap();
        let deserialized: DialogParticipantManager = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.chat_count(), 0);
    }
}
