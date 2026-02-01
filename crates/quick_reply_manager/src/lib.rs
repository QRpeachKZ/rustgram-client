// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Quick Reply Manager
//!
//! Manages quick reply shortcuts and messages for Telegram MTProto.
//!
//! Based on TDLib's `QuickReplyManager` from `td/telegram/QuickReplyManager.h`.
//!
//! # Overview
//!
//! The `QuickReplyManager` handles quick reply shortcuts - predefined message
//! templates that users can quickly send in conversations. It manages shortcuts,
//! their messages, and provides methods for creating, editing, and sending quick replies.
//!
//! # Example
//!
//! ```rust
//! use rustgram_quick_reply_manager::QuickReplyManager;
//!
//! let mut manager = QuickReplyManager::new();
//! manager.init();
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use rustgram_quick_reply_message_full_id::QuickReplyMessageFullId;
use rustgram_quick_reply_shortcut_id::QuickReplyShortcutId;
use rustgram_types::MessageId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

/// Maximum number of messages in a quick reply shortcut group (server-side limit).
pub const MAX_GROUPED_MESSAGES: usize = 10;

/// Error type for QuickReplyManager operations.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum QuickReplyManagerError {
    /// Manager not initialized.
    #[error("manager not initialized")]
    NotInitialized,

    /// Shortcut not found.
    #[error("shortcut not found: {0}")]
    ShortcutNotFound(QuickReplyShortcutId),

    /// Message not found.
    #[error("message not found")]
    MessageNotFound,

    /// Invalid shortcut name.
    #[error("invalid shortcut name: {0}")]
    InvalidShortcutName(String),

    /// Shortcut already exists.
    #[error("shortcut already exists: {0}")]
    ShortcutAlreadyExists(String),

    /// Too many messages in group.
    #[error("too many messages in group (max {0})")]
    TooManyMessages(usize),

    /// Network error.
    #[error("network error: {0}")]
    NetworkError(String),
}

/// Result type for QuickReplyManager operations.
pub type Result<T> = std::result::Result<T, QuickReplyManagerError>;

/// Checks if a shortcut name is valid.
///
/// A valid shortcut name contains only alphanumeric characters and underscores,
/// and is between 1 and 32 characters long.
///
/// # Arguments
///
/// * `name` - The shortcut name to validate
///
/// # Example
///
/// ```rust
/// use rustgram_quick_reply_manager::check_shortcut_name;
///
/// assert!(check_shortcut_name("hello").is_ok());
/// assert!(check_shortcut_name("hello_world").is_ok());
/// assert!(check_shortcut_name("").is_err());
/// assert!(check_shortcut_name("hello-world").is_err());
/// ```
pub fn check_shortcut_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(QuickReplyManagerError::InvalidShortcutName(
            "name is empty".to_string(),
        ));
    }

    if name.len() > 32 {
        return Err(QuickReplyManagerError::InvalidShortcutName(
            "name is too long (max 32 characters)".to_string(),
        ));
    }

    // Check that name contains only valid characters
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(QuickReplyManagerError::InvalidShortcutName(
            "name contains invalid characters".to_string(),
        ));
    }

    Ok(())
}

/// A quick reply message.
///
/// Represents a single message within a quick reply shortcut.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickReplyMessage {
    /// The message ID.
    pub message_id: MessageId,
    /// The shortcut ID this message belongs to.
    pub shortcut_id: QuickReplyShortcutId,
    /// When the message was last edited (Unix timestamp).
    pub edit_date: i32,
    /// Random ID for sending.
    pub random_id: i64,
    /// The message this is replying to, if any.
    pub reply_to_message_id: Option<MessageId>,
    /// ID of the bot via which this message was sent, if any.
    pub via_bot_user_id: Option<i64>,
    /// Whether sending this message failed.
    pub is_failed_to_send: bool,
    /// Whether to disable notification for this message.
    pub disable_notification: bool,
    /// Whether to invert the media position.
    pub invert_media: bool,
    /// Whether to disable web page preview.
    pub disable_web_page_preview: bool,
}

impl QuickReplyMessage {
    /// Creates a new quick reply message.
    #[must_use]
    pub fn new(message_id: MessageId, shortcut_id: QuickReplyShortcutId) -> Self {
        Self {
            message_id,
            shortcut_id,
            edit_date: 0,
            random_id: 0,
            reply_to_message_id: None,
            via_bot_user_id: None,
            is_failed_to_send: false,
            disable_notification: false,
            invert_media: false,
            disable_web_page_preview: false,
        }
    }

    /// Checks if this message can be edited.
    #[must_use]
    pub fn can_edit(&self) -> bool {
        // In TDLib, messages can be edited if they haven't failed to send
        !self.is_failed_to_send
    }

    /// Checks if this message can be resent.
    #[must_use]
    pub fn can_resend(&self) -> bool {
        // Only failed messages can be resent
        self.is_failed_to_send
    }
}

/// A quick reply shortcut.
///
/// Represents a named collection of quick reply messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickReplyShortcut {
    /// The shortcut name.
    pub name: String,
    /// The shortcut ID.
    pub shortcut_id: QuickReplyShortcutId,
    /// Total server message count.
    pub server_total_count: i32,
    /// Total local message count.
    pub local_total_count: i32,
    /// Messages in this shortcut.
    pub messages: Vec<QuickReplyMessage>,
    /// Last assigned message ID.
    pub last_assigned_message_id: MessageId,
}

impl QuickReplyShortcut {
    /// Creates a new quick reply shortcut.
    #[must_use]
    pub fn new(name: String, shortcut_id: QuickReplyShortcutId) -> Self {
        Self {
            name,
            shortcut_id,
            server_total_count: 0,
            local_total_count: 0,
            messages: Vec::new(),
            last_assigned_message_id: MessageId(0),
        }
    }

    /// Returns the number of messages in this shortcut.
    #[must_use]
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Checks if all messages for this shortcut have been loaded.
    #[must_use]
    pub fn has_all_messages(&self) -> bool {
        self.message_count() == (self.server_total_count + self.local_total_count) as usize
    }

    /// Gets a message by its ID.
    #[must_use]
    pub fn get_message(&self, message_id: MessageId) -> Option<&QuickReplyMessage> {
        self.messages.iter().find(|m| m.message_id == message_id)
    }
}

/// Quick reply shortcuts collection.
///
/// Manages the state of all quick reply shortcuts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickReplyShortcuts {
    /// All shortcuts.
    pub shortcuts: Vec<QuickReplyShortcut>,
    /// Whether shortcuts have been initialized.
    pub are_inited: bool,
    /// Whether shortcuts have been loaded from database.
    pub are_loaded_from_database: bool,
}

impl Default for QuickReplyShortcuts {
    fn default() -> Self {
        Self {
            shortcuts: Vec::new(),
            are_inited: false,
            are_loaded_from_database: false,
        }
    }
}

impl QuickReplyShortcuts {
    /// Creates a new empty shortcuts collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            shortcuts: Vec::new(),
            are_inited: false,
            are_loaded_from_database: false,
        }
    }

    /// Gets a shortcut by ID.
    #[must_use]
    pub fn get_shortcut(&self, shortcut_id: QuickReplyShortcutId) -> Option<&QuickReplyShortcut> {
        self.shortcuts.iter().find(|s| s.shortcut_id == shortcut_id)
    }

    /// Gets a shortcut by name.
    #[must_use]
    pub fn get_shortcut_by_name(&self, name: &str) -> Option<&QuickReplyShortcut> {
        self.shortcuts.iter().find(|s| s.name == name)
    }

    /// Gets a mutable shortcut by ID.
    #[must_use]
    pub fn get_shortcut_mut(
        &mut self,
        shortcut_id: QuickReplyShortcutId,
    ) -> Option<&mut QuickReplyShortcut> {
        self.shortcuts
            .iter_mut()
            .find(|s| s.shortcut_id == shortcut_id)
    }

    /// Adds a shortcut to the collection.
    pub fn add_shortcut(&mut self, shortcut: QuickReplyShortcut) {
        self.shortcuts.push(shortcut);
    }

    /// Removes a shortcut from the collection.
    pub fn remove_shortcut(
        &mut self,
        shortcut_id: QuickReplyShortcutId,
    ) -> Option<QuickReplyShortcut> {
        let pos = self
            .shortcuts
            .iter()
            .position(|s| s.shortcut_id == shortcut_id)?;
        Some(self.shortcuts.remove(pos))
    }

    /// Returns all shortcut IDs.
    #[must_use]
    pub fn shortcut_ids(&self) -> Vec<QuickReplyShortcutId> {
        self.shortcuts.iter().map(|s| s.shortcut_id).collect()
    }
}

/// Quick reply manager for Telegram.
///
/// Manages quick reply shortcuts and their associated messages.
///
/// # Example
///
/// ```rust
/// use rustgram_quick_reply_manager::QuickReplyManager;
///
/// let mut manager = QuickReplyManager::new();
/// manager.init();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickReplyManager {
    /// Quick reply shortcuts collection.
    #[serde(default)]
    pub shortcuts: QuickReplyShortcuts,
    /// Next local shortcut ID to assign.
    #[serde(default)]
    pub next_local_shortcut_id: i32,
    /// Deleted shortcut IDs.
    #[serde(default)]
    pub deleted_shortcut_ids: Vec<QuickReplyShortcutId>,
    /// Shortcut name to ID mapping.
    #[serde(default)]
    pub shortcut_names: HashMap<String, QuickReplyShortcutId>,
}

impl Default for QuickReplyManager {
    fn default() -> Self {
        Self::new()
    }
}

impl QuickReplyManager {
    /// Creates a new `QuickReplyManager`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_quick_reply_manager::QuickReplyManager;
    ///
    /// let manager = QuickReplyManager::new();
    /// assert!(!manager.shortcuts.are_inited);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            shortcuts: QuickReplyShortcuts::new(),
            next_local_shortcut_id: QuickReplyShortcutId::MAX_SERVER_SHORTCUT_ID + 1,
            deleted_shortcut_ids: Vec::new(),
            shortcut_names: HashMap::new(),
        }
    }

    /// Initializes the manager.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_quick_reply_manager::QuickReplyManager;
    ///
    /// let mut manager = QuickReplyManager::new();
    /// manager.init();
    /// ```
    pub fn init(&mut self) {
        self.shortcuts.are_inited = true;
    }

    /// Gets all quick reply shortcuts.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_quick_reply_manager::QuickReplyManager;
    ///
    /// let manager = QuickReplyManager::new();
    /// let shortcuts = manager.get_quick_reply_shortcuts();
    /// assert!(shortcuts.is_empty());
    /// ```
    #[must_use]
    pub fn get_quick_reply_shortcuts(&self) -> &[QuickReplyShortcut] {
        &self.shortcuts.shortcuts
    }

    /// Sets the name of a quick reply shortcut.
    ///
    /// # Arguments
    ///
    /// * `shortcut_id` - The shortcut to rename
    /// * `name` - The new name
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_quick_reply_manager::QuickReplyManager;
    /// use rustgram_quick_reply_shortcut_id::QuickReplyShortcutId;
    ///
    /// let mut manager = QuickReplyManager::new();
    /// manager.init();
    ///
    /// let shortcut_id = QuickReplyShortcutId::new(1);
    /// // After adding a shortcut...
    /// let result = manager.set_shortcut_name(shortcut_id, "new_name".to_string());
    /// ```
    pub fn set_shortcut_name(
        &mut self,
        shortcut_id: QuickReplyShortcutId,
        name: String,
    ) -> Result<()> {
        check_shortcut_name(&name)?;

        // Check if name already exists
        if let Some(&existing_id) = self.shortcut_names.get(&name) {
            if existing_id != shortcut_id {
                return Err(QuickReplyManagerError::ShortcutAlreadyExists(name));
            }
        }

        let shortcut = self
            .shortcuts
            .get_shortcut_mut(shortcut_id)
            .ok_or_else(|| QuickReplyManagerError::ShortcutNotFound(shortcut_id))?;

        // Remove old name mapping
        self.shortcut_names.remove(&shortcut.name);

        // Update name
        shortcut.name = name.clone();
        self.shortcut_names.insert(name, shortcut_id);

        Ok(())
    }

    /// Deletes a quick reply shortcut.
    ///
    /// # Arguments
    ///
    /// * `shortcut_id` - The shortcut to delete
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_quick_reply_manager::QuickReplyManager;
    /// use rustgram_quick_reply_shortcut_id::QuickReplyShortcutId;
    ///
    /// let mut manager = QuickReplyManager::new();
    /// manager.init();
    ///
    /// let shortcut_id = QuickReplyShortcutId::new(1);
    /// let result = manager.delete_shortcut(shortcut_id);
    /// ```
    pub fn delete_shortcut(&mut self, shortcut_id: QuickReplyShortcutId) -> Result<()> {
        let shortcut = self
            .shortcuts
            .remove_shortcut(shortcut_id)
            .ok_or_else(|| QuickReplyManagerError::ShortcutNotFound(shortcut_id))?;

        // Remove from name mapping
        self.shortcut_names.remove(&shortcut.name);

        // Track as deleted
        self.deleted_shortcut_ids.push(shortcut_id);

        Ok(())
    }

    /// Reorders quick reply shortcuts.
    ///
    /// # Arguments
    ///
    /// * `shortcut_ids` - New order for shortcuts
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_quick_reply_manager::QuickReplyManager;
    /// use rustgram_quick_reply_shortcut_id::QuickReplyShortcutId;
    ///
    /// let mut manager = QuickReplyManager::new();
    /// manager.init();
    ///
    /// let id1 = QuickReplyShortcutId::new(1);
    /// let id2 = QuickReplyShortcutId::new(2);
    /// let result = manager.reorder_shortcuts(&[id2, id1]);
    /// ```
    pub fn reorder_shortcuts(&mut self, shortcut_ids: &[QuickReplyShortcutId]) -> Result<()> {
        // Create a new ordered shortcuts vector
        let mut new_shortcuts = Vec::new();

        for &shortcut_id in shortcut_ids {
            let pos = self
                .shortcuts
                .shortcuts
                .iter()
                .position(|s| s.shortcut_id == shortcut_id)
                .ok_or_else(|| QuickReplyManagerError::ShortcutNotFound(shortcut_id))?;

            let shortcut = self.shortcuts.shortcuts.remove(pos);
            new_shortcuts.push(shortcut);
        }

        // Add any remaining shortcuts
        self.shortcuts.shortcuts = new_shortcuts;

        Ok(())
    }

    /// Gets messages for a specific shortcut.
    ///
    /// # Arguments
    ///
    /// * `shortcut_id` - The shortcut to get messages for
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_quick_reply_manager::QuickReplyManager;
    /// use rustgram_quick_reply_shortcut_id::QuickReplyShortcutId;
    ///
    /// let manager = QuickReplyManager::new();
    /// let messages = manager.get_shortcut_messages(QuickReplyShortcutId::new(1));
    /// assert!(messages.is_err()); // Shortcut doesn't exist
    /// ```
    #[must_use]
    pub fn get_shortcut_messages(
        &self,
        shortcut_id: QuickReplyShortcutId,
    ) -> Result<&[QuickReplyMessage]> {
        let shortcut = self
            .shortcuts
            .get_shortcut(shortcut_id)
            .ok_or_else(|| QuickReplyManagerError::ShortcutNotFound(shortcut_id))?;

        Ok(&shortcut.messages)
    }

    /// Gets a specific message.
    ///
    /// # Arguments
    ///
    /// * `message_full_id` - The full message identifier
    #[must_use]
    pub fn get_message(
        &self,
        message_full_id: QuickReplyMessageFullId,
    ) -> Result<&QuickReplyMessage> {
        let shortcut = self
            .shortcuts
            .get_shortcut(message_full_id.shortcut_id())
            .ok_or_else(|| {
                QuickReplyManagerError::ShortcutNotFound(message_full_id.shortcut_id())
            })?;

        shortcut
            .get_message(message_full_id.message_id())
            .ok_or(QuickReplyManagerError::MessageNotFound)
    }

    /// Creates a new local shortcut.
    ///
    /// # Arguments
    ///
    /// * `name` - The shortcut name
    /// * `new_message_count` - Number of new messages to create
    pub fn create_new_local_shortcut(
        &mut self,
        name: String,
        new_message_count: i32,
    ) -> Result<QuickReplyShortcutId> {
        check_shortcut_name(&name)?;

        // Check if name already exists
        if self.shortcut_names.contains_key(&name) {
            return Err(QuickReplyManagerError::ShortcutAlreadyExists(name));
        }

        let shortcut_id = QuickReplyShortcutId::new(self.next_local_shortcut_id);
        self.next_local_shortcut_id += 1;

        let mut shortcut = QuickReplyShortcut::new(name.clone(), shortcut_id);
        shortcut.local_total_count = new_message_count;

        self.shortcut_names.insert(name, shortcut_id);
        self.shortcuts.add_shortcut(shortcut);

        Ok(shortcut_id)
    }

    /// Deletes messages from a shortcut.
    ///
    /// # Arguments
    ///
    /// * `shortcut_id` - The shortcut containing messages
    /// * `message_ids` - The message IDs to delete
    pub fn delete_messages(
        &mut self,
        shortcut_id: QuickReplyShortcutId,
        message_ids: &[MessageId],
    ) -> Result<()> {
        let shortcut = self
            .shortcuts
            .get_shortcut_mut(shortcut_id)
            .ok_or_else(|| QuickReplyManagerError::ShortcutNotFound(shortcut_id))?;

        // Remove specified messages
        shortcut
            .messages
            .retain(|m| !message_ids.contains(&m.message_id));

        Ok(())
    }

    /// Updates a quick reply message.
    ///
    /// # Arguments
    ///
    /// * `message_full_id` - The message to update
    /// * `message` - The updated message data
    pub fn update_message(
        &mut self,
        message_full_id: QuickReplyMessageFullId,
        message: QuickReplyMessage,
    ) -> Result<()> {
        let shortcut = self
            .shortcuts
            .get_shortcut_mut(message_full_id.shortcut_id())
            .ok_or_else(|| {
                QuickReplyManagerError::ShortcutNotFound(message_full_id.shortcut_id())
            })?;

        let pos = shortcut
            .messages
            .iter()
            .position(|m| m.message_id == message_full_id.message_id())
            .ok_or(QuickReplyManagerError::MessageNotFound)?;

        shortcut.messages[pos] = message;

        Ok(())
    }

    /// Returns the current state of the manager.
    #[must_use]
    pub fn is_initialized(&self) -> bool {
        self.shortcuts.are_inited
    }

    /// Resets the manager to uninitialized state.
    pub fn reset(&mut self) {
        self.shortcuts = QuickReplyShortcuts::new();
        self.next_local_shortcut_id = QuickReplyShortcutId::MAX_SERVER_SHORTCUT_ID + 1;
        self.deleted_shortcut_ids.clear();
        self.shortcut_names.clear();
    }
}

impl fmt::Display for QuickReplyManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "QuickReplyManager(shortcuts={}, inited={})",
            self.shortcuts.shortcuts.len(),
            self.shortcuts.are_inited
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_shortcut_name_valid() {
        assert!(check_shortcut_name("hello").is_ok());
        assert!(check_shortcut_name("hello_world").is_ok());
        assert!(check_shortcut_name("hello123").is_ok());
        assert!(check_shortcut_name("a").is_ok());
    }

    #[test]
    fn test_check_shortcut_name_invalid() {
        assert!(check_shortcut_name("").is_err());
        assert!(check_shortcut_name("hello-world").is_err());
        assert!(check_shortcut_name("hello world").is_err());
        assert!(check_shortcut_name("hello.world").is_err());
    }

    #[test]
    fn test_check_shortcut_name_too_long() {
        let long_name = "a".repeat(33);
        assert!(check_shortcut_name(&long_name).is_err());

        let max_name = "a".repeat(32);
        assert!(check_shortcut_name(&max_name).is_ok());
    }

    #[test]
    fn test_quick_reply_message_new() {
        let shortcut_id = QuickReplyShortcutId::new(1);
        let message_id = MessageId((10 << 20) | 2);
        let message = QuickReplyMessage::new(message_id, shortcut_id);

        assert_eq!(message.message_id, message_id);
        assert_eq!(message.shortcut_id, shortcut_id);
        assert!(message.can_edit());
        assert!(!message.can_resend());
    }

    #[test]
    fn test_quick_reply_message_failed() {
        let mut message =
            QuickReplyMessage::new(MessageId((10 << 20) | 2), QuickReplyShortcutId::new(1));
        message.is_failed_to_send = true;

        assert!(!message.can_edit());
        assert!(message.can_resend());
    }

    #[test]
    fn test_quick_reply_shortcut_new() {
        let shortcut_id = QuickReplyShortcutId::new(1);
        let shortcut = QuickReplyShortcut::new("test".to_string(), shortcut_id);

        assert_eq!(shortcut.name, "test");
        assert_eq!(shortcut.shortcut_id, shortcut_id);
        assert_eq!(shortcut.message_count(), 0);
        // An empty shortcut (0 of 0 messages) has "all" messages loaded
        assert!(shortcut.has_all_messages());
    }

    #[test]
    fn test_quick_reply_shortcut_get_message() {
        let mut shortcut =
            QuickReplyShortcut::new("test".to_string(), QuickReplyShortcutId::new(1));

        let message_id = MessageId((10 << 20) | 2);
        let message = QuickReplyMessage::new(message_id, shortcut.shortcut_id);
        shortcut.messages.push(message);

        assert!(shortcut.get_message(message_id).is_some());
        assert!(shortcut.get_message(MessageId((99 << 20) | 2)).is_none());
    }

    #[test]
    fn test_quick_reply_shortcuts_new() {
        let shortcuts = QuickReplyShortcuts::new();

        assert!(!shortcuts.are_inited);
        assert!(!shortcuts.are_loaded_from_database);
        assert_eq!(shortcuts.shortcuts.len(), 0);
    }

    #[test]
    fn test_quick_reply_shortcuts_default() {
        let shortcuts = QuickReplyShortcuts::default();

        assert!(!shortcuts.are_inited);
    }

    #[test]
    fn test_quick_reply_shortcuts_add_get() {
        let mut shortcuts = QuickReplyShortcuts::new();
        let shortcut_id = QuickReplyShortcutId::new(1);
        let shortcut = QuickReplyShortcut::new("test".to_string(), shortcut_id);

        shortcuts.add_shortcut(shortcut);

        assert_eq!(shortcuts.shortcuts.len(), 1);
        assert!(shortcuts.get_shortcut(shortcut_id).is_some());
        assert!(shortcuts.get_shortcut_by_name("test").is_some());
    }

    #[test]
    fn test_quick_reply_shortcuts_remove() {
        let mut shortcuts = QuickReplyShortcuts::new();
        let shortcut_id = QuickReplyShortcutId::new(1);
        let shortcut = QuickReplyShortcut::new("test".to_string(), shortcut_id);

        shortcuts.add_shortcut(shortcut);

        let removed = shortcuts.remove_shortcut(shortcut_id);
        assert!(removed.is_some());
        assert_eq!(shortcuts.shortcuts.len(), 0);
    }

    #[test]
    fn test_quick_reply_manager_new() {
        let manager = QuickReplyManager::new();

        assert!(!manager.shortcuts.are_inited);
        assert_eq!(
            manager.next_local_shortcut_id,
            QuickReplyShortcutId::MAX_SERVER_SHORTCUT_ID + 1
        );
    }

    #[test]
    fn test_quick_reply_manager_default() {
        let manager = QuickReplyManager::default();

        assert!(!manager.shortcuts.are_inited);
    }

    #[test]
    fn test_quick_reply_manager_init() {
        let mut manager = QuickReplyManager::new();
        manager.init();

        assert!(manager.shortcuts.are_inited);
    }

    #[test]
    fn test_quick_reply_manager_create_shortcut() {
        let mut manager = QuickReplyManager::new();
        manager.init();

        let result = manager.create_new_local_shortcut("test".to_string(), 0);

        assert!(result.is_ok());
        let shortcut_id = result.unwrap();
        assert!(shortcut_id.is_local());
    }

    #[test]
    fn test_quick_reply_manager_create_shortcut_invalid_name() {
        let mut manager = QuickReplyManager::new();
        manager.init();

        let result = manager.create_new_local_shortcut("test-name".to_string(), 0);

        assert!(result.is_err());
    }

    #[test]
    fn test_quick_reply_manager_create_shortcut_duplicate() {
        let mut manager = QuickReplyManager::new();
        manager.init();

        let _ = manager.create_new_local_shortcut("test".to_string(), 0);
        let result = manager.create_new_local_shortcut("test".to_string(), 0);

        assert!(matches!(
            result,
            Err(QuickReplyManagerError::ShortcutAlreadyExists(_))
        ));
    }

    #[test]
    fn test_quick_reply_manager_delete_shortcut() {
        let mut manager = QuickReplyManager::new();
        manager.init();

        let shortcut_id = manager
            .create_new_local_shortcut("test".to_string(), 0)
            .unwrap();

        let result = manager.delete_shortcut(shortcut_id);

        assert!(result.is_ok());
        assert!(manager.shortcuts.get_shortcut(shortcut_id).is_none());
    }

    #[test]
    fn test_quick_reply_manager_delete_shortcut_not_found() {
        let mut manager = QuickReplyManager::new();
        manager.init();

        let result = manager.delete_shortcut(QuickReplyShortcutId::new(999));

        assert!(matches!(
            result,
            Err(QuickReplyManagerError::ShortcutNotFound(_))
        ));
    }

    #[test]
    fn test_quick_reply_manager_set_shortcut_name() {
        let mut manager = QuickReplyManager::new();
        manager.init();

        let shortcut_id = manager
            .create_new_local_shortcut("test".to_string(), 0)
            .unwrap();

        let result = manager.set_shortcut_name(shortcut_id, "new_name".to_string());

        assert!(result.is_ok());
        let shortcut = manager.shortcuts.get_shortcut(shortcut_id).unwrap();
        assert_eq!(shortcut.name, "new_name");
    }

    #[test]
    fn test_quick_reply_manager_set_shortcut_name_invalid() {
        let mut manager = QuickReplyManager::new();
        manager.init();

        let shortcut_id = manager
            .create_new_local_shortcut("test".to_string(), 0)
            .unwrap();

        let result = manager.set_shortcut_name(shortcut_id, "invalid-name".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_quick_reply_manager_reset() {
        let mut manager = QuickReplyManager::new();
        manager.init();
        let _ = manager.create_new_local_shortcut("test".to_string(), 0);

        assert!(manager.shortcuts.are_inited);
        assert!(!manager.shortcut_names.is_empty());

        manager.reset();

        assert!(!manager.shortcuts.are_inited);
        assert!(manager.shortcut_names.is_empty());
    }

    #[test]
    fn test_quick_reply_manager_display() {
        let manager = QuickReplyManager::new();
        let display = format!("{manager}");
        assert!(display.contains("QuickReplyManager"));
    }

    #[test]
    fn test_serialization() {
        let manager = QuickReplyManager::new();
        let json = serde_json::to_string(&manager).expect("Failed to serialize");
        assert!(json.contains("shortcuts"));

        let deserialized: QuickReplyManager =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(
            deserialized.next_local_shortcut_id,
            manager.next_local_shortcut_id
        );
    }
}
