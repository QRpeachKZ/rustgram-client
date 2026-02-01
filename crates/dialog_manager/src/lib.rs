//! # Dialog Manager
//!
//! Central manager for all Telegram dialogs (chats, channels, users).
//!
//! This module provides functionality for:
//! - Unified dialog management across all dialog types
//! - Input peer construction for API calls
//! - Dialog access checking and validation
//! - Message TTL management
//! - Dialog metadata operations
//!
//! # Overview
//!
//! The DialogManager is the central point for managing all types of dialogs in Telegram.
//! It provides a unified interface for working with users, basic groups, channels, and secret chats.
//!
//! # Main Components
//!
//! - [`DialogManager`]: Main manager for dialog operations
//! - [`DialogAccess`]: Access control for dialog operations
//! - [`InputDialogId`]: Input dialog ID for API calls
//!
//! # Examples
//!
//! ```rust
//! use rustgram_dialog_manager::DialogManager;
//! use rustgram_types::{ChatId, DialogId};
//!
//! let mut manager = DialogManager::new();
//!
//! // Register a dialog
//! let chat_id = ChatId::new(123).unwrap();
//! let dialog_id = DialogId::from_chat(chat_id);
//!
//! manager.register_dialog(dialog_id, "Test Group".to_string()).unwrap();
//!
//! // Check if dialog exists
//! assert!(manager.has_dialog(dialog_id));
//! ```
//!
//! For input peer operations, use `rustgram_chat_manager::AccessRights`:
//!
//! ```rust
//! # use rustgram_chat_manager::AccessRights;
//! # use rustgram_dialog_manager::DialogManager;
//! # use rustgram_types::{ChatId, DialogId};
//! #
//! # let mut manager = DialogManager::new();
//!
//! let chat_id = ChatId::new(123).unwrap();
//! let dialog_id = DialogId::from_chat(chat_id);
//!
//! manager.register_dialog(dialog_id, "Test".to_string()).unwrap();
//!
//! // Note: get_input_peer returns None if chat not registered in ChatManager
//! let peer = manager.get_input_peer(dialog_id, AccessRights::Read);
//! // assert!(peer.is_some()); // Requires ChatManager setup
//! ```
//!
//! # Thread Safety
//!
//! The manager uses `Arc<RwLock<T>>` for shared state, allowing safe concurrent access.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use rustgram_chat_manager::ChatManager;
use rustgram_types::{ChannelId, ChatId, DialogId, InputPeer, SecretChatId, UserId};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

pub mod cache;
pub mod error;
pub mod network;
pub mod participants;
pub mod tl_types;
pub use cache::{DialogCache, DialogMetadata as CachedDialogMetadata};
pub use error::{DialogError, Result};
pub use network::{generate_query_id, NetworkClient};
pub use participants::{
    ChannelFull, ChatFull, ChatPhoto, FileLocation, GetChannelParticipantsRequest,
    GetChannelParticipantsResponse, GetFullChatRequest, GetFullChatResponse, Participant,
    ParticipantManager, CHANNELS_GET_FULL_CHANNEL, CHANNELS_GET_PARTICIPANTS,
    DEFAULT_PARTICIPANTS_LIMIT, MESSAGES_GET_FULL_CHAT,
};
pub use tl_types::{
    CreateChatRequest, Dialog, DialogPagination, GetDialogsRequest, GetDialogsResponse,
    UpdatePhotoRequest, UpdateTitleRequest, Updates,
};

/// Maximum dialog title length.
///
/// # TDLib Alignment
///
/// Corresponds to TDLib's `DialogManager::MAX_TITLE_LENGTH`.
pub const MAX_TITLE_LENGTH: usize = 128;

/// Message time-to-live (auto-delete) in seconds.
///
/// # TDLib Alignment
///
/// Corresponds to TDLib's message TTL settings.
pub type MessageTtl = i32;

/// No auto-delete (messages stay forever).
pub const MESSAGE_TTL_FOREVER: MessageTtl = 0;

/// Input dialog ID for API calls.
///
/// Represents a dialog identifier that can be used in API calls.
///
/// # TDLib Alignment
///
/// Corresponds to TDLib's `InputDialogId` class.
/// Simplified implementation without access hash support.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_manager::InputDialogId;
/// use rustgram_types::{ChatId, DialogId};
///
/// let chat_id = ChatId::new(123).unwrap();
/// let dialog_id = DialogId::from_chat(chat_id);
///
/// let input = InputDialogId::new(dialog_id);
/// assert_eq!(input.get_dialog_id(), dialog_id);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputDialogId {
    /// The dialog ID.
    dialog_id: DialogId,
}

impl InputDialogId {
    /// Creates a new input dialog ID.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::InputDialogId;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
    /// let input = InputDialogId::new(dialog_id);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(dialog_id: DialogId) -> Self {
        Self { dialog_id }
    }

    /// Gets the dialog ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::InputDialogId;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
    /// let input = InputDialogId::new(dialog_id);
    ///
    /// assert_eq!(input.get_dialog_id(), dialog_id);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get_dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Checks if this input dialog ID is valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::InputDialogId;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let dialog_id = DialogId::from_chat(ChatId::new(123).unwrap());
    /// let input = InputDialogId::new(dialog_id);
    ///
    /// assert!(input.is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.dialog_id.is_valid()
    }

    /// Converts to a dialog ID.
    #[inline]
    #[must_use]
    pub const fn to_dialog_id(&self) -> DialogId {
        self.dialog_id
    }
}

impl From<DialogId> for InputDialogId {
    #[inline]
    fn from(dialog_id: DialogId) -> Self {
        Self::new(dialog_id)
    }
}

impl From<UserId> for InputDialogId {
    #[inline]
    fn from(user_id: UserId) -> Self {
        Self::new(DialogId::from_user(user_id))
    }
}

impl From<ChatId> for InputDialogId {
    #[inline]
    fn from(chat_id: ChatId) -> Self {
        Self::new(DialogId::from_chat(chat_id))
    }
}

impl From<ChannelId> for InputDialogId {
    #[inline]
    fn from(channel_id: ChannelId) -> Self {
        Self::new(DialogId::from_channel(channel_id))
    }
}

impl From<SecretChatId> for InputDialogId {
    #[inline]
    fn from(secret_chat_id: SecretChatId) -> Self {
        Self::new(DialogId::from_secret_chat(secret_chat_id))
    }
}

/// Dialog metadata stored in the manager.
#[derive(Debug, Clone)]
pub struct DialogManagerMetadata {
    /// Dialog title.
    pub title: String,
    /// Message TTL (auto-delete timer).
    pub message_ttl: MessageTtl,
    /// Whether the dialog is pinned.
    pub is_pinned: bool,
    /// Whether the dialog is marked as unread.
    #[allow(dead_code)]
    pub is_marked_unread: bool,
    /// Notification settings (simplified).
    #[allow(dead_code)]
    pub muted: bool,
}

impl Default for DialogManagerMetadata {
    fn default() -> Self {
        Self {
            title: String::new(),
            message_ttl: MESSAGE_TTL_FOREVER,
            is_pinned: false,
            is_marked_unread: false,
            muted: false,
        }
    }
}

/// Dialog manager state.
///
/// Internal state managed by the DialogManager.
/// Uses Arc<RwLock<T>> for thread-safe shared access.
#[derive(Debug, Clone)]
pub struct DialogManagerState {
    /// Map of dialog IDs to their metadata.
    pub dialogs: HashMap<DialogId, DialogManagerMetadata>,
    /// Set of dialogs the user has access to.
    pub accessible_dialogs: HashSet<DialogId>,
    /// Recently opened dialogs.
    pub recently_opened: Vec<DialogId>,
    /// Recently found dialogs.
    pub recently_found: Vec<DialogId>,
    /// Username to dialog ID mapping.
    pub username_cache: HashMap<String, DialogId>,
}

impl Default for DialogManagerState {
    fn default() -> Self {
        Self {
            dialogs: HashMap::new(),
            accessible_dialogs: HashSet::new(),
            recently_opened: Vec::new(),
            recently_found: Vec::new(),
            username_cache: HashMap::new(),
        }
    }
}

/// Dialog manager for Telegram.
///
/// Central manager for all dialog operations including users, chats, channels, and secret chats.
///
/// # Thread Safety
///
/// This manager uses `Arc<RwLock<T>>` internally, making it safe to share across threads.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_manager::DialogManager;
/// use rustgram_types::{ChatId, DialogId};
///
/// let mut manager = DialogManager::new();
///
/// let chat_id = ChatId::new(123).unwrap();
/// let dialog_id = DialogId::from_chat(chat_id);
///
/// manager.register_dialog(dialog_id, "Test Group".to_string()).unwrap();
/// assert!(manager.has_dialog(dialog_id));
/// ```
#[derive(Debug, Clone)]
pub struct DialogManager {
    /// Internal state protected by RwLock for thread-safe access.
    state: Arc<std::sync::RwLock<DialogManagerState>>,
    /// Chat manager for channel/chat operations.
    chat_manager: ChatManager,
}

impl Default for DialogManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DialogManager {
    /// Creates a new dialog manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    ///
    /// let manager = DialogManager::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(std::sync::RwLock::new(DialogManagerState::default())),
            chat_manager: ChatManager::new(),
        }
    }

    // ========== Dialog Registration ==========

    /// Registers a dialog with the manager.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `title` - Dialog title
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The title is empty or too long
    /// - The lock cannot be acquired
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.register_dialog(dialog_id, "Test Group".to_string()).unwrap();
    /// assert!(manager.has_dialog(dialog_id));
    /// ```
    pub fn register_dialog(&mut self, dialog_id: DialogId, title: String) -> Result<()> {
        if title.is_empty() {
            return Err(DialogError::EmptyTitle);
        }
        if title.len() > MAX_TITLE_LENGTH {
            return Err(DialogError::TitleTooLong {
                max: MAX_TITLE_LENGTH,
                len: title.len(),
            });
        }

        let mut state = self.state.write().map_err(|_| DialogError::LockError)?;

        let metadata = DialogManagerMetadata {
            title,
            ..Default::default()
        };

        state.dialogs.insert(dialog_id, metadata);
        state.accessible_dialogs.insert(dialog_id);
        Ok(())
    }

    /// Checks if a dialog exists in the manager.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID to check
    ///
    /// # Returns
    ///
    /// `true` if the dialog exists
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// assert!(!manager.has_dialog(dialog_id));
    ///
    /// manager.register_dialog(dialog_id, "Test".to_string()).unwrap();
    /// assert!(manager.has_dialog(dialog_id));
    /// ```
    #[inline]
    #[must_use]
    pub fn has_dialog(&self, dialog_id: DialogId) -> bool {
        self.state
            .read()
            .map(|s| s.dialogs.contains_key(&dialog_id))
            .unwrap_or(false)
    }

    /// Checks if we have access to a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Returns
    ///
    /// `true` if we have access
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `have_dialog_info`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// assert!(!manager.have_dialog_access(dialog_id));
    ///
    /// manager.register_dialog(dialog_id, "Test".to_string()).unwrap();
    /// assert!(manager.have_dialog_access(dialog_id));
    /// ```
    #[inline]
    #[must_use]
    pub fn have_dialog_access(&self, dialog_id: DialogId) -> bool {
        self.state
            .read()
            .map(|s| s.accessible_dialogs.contains(&dialog_id))
            .unwrap_or(false)
    }

    /// Gets a dialog's title.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Returns
    ///
    /// `Some(title)` if the dialog exists
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.register_dialog(dialog_id, "Test Group".to_string()).unwrap();
    ///
    /// assert_eq!(manager.get_dialog_title(dialog_id), Some("Test Group".to_string()));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_dialog_title(&self, dialog_id: DialogId) -> Option<String> {
        self.state
            .read()
            .ok()?
            .dialogs
            .get(&dialog_id)
            .map(|m| m.title.clone())
    }

    // ========== Input Peer Operations ==========

    /// Gets an input peer for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `access_rights` - Required access rights
    ///
    /// # Returns
    ///
    /// `Some(InputPeer)` if we have access
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `get_input_peer` method.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustgram_chat_manager::AccessRights;
    /// # use rustgram_dialog_manager::DialogManager;
    /// # use rustgram_types::{ChatId, DialogId};
    /// #
    /// # let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.register_dialog(dialog_id, "Test".to_string()).unwrap();
    ///
    /// // Note: Returns None if chat not registered in ChatManager
    /// let peer = manager.get_input_peer(dialog_id, AccessRights::Read);
    /// // assert!(peer.is_some()); // Requires ChatManager setup
    /// ```
    #[must_use]
    pub fn get_input_peer(
        &self,
        dialog_id: DialogId,
        access_rights: rustgram_chat_manager::AccessRights,
    ) -> Option<InputPeer> {
        if !self.have_dialog_access(dialog_id) {
            return None;
        }

        match dialog_id {
            DialogId::User(_user_id) => {
                // TODO: Implement user input peer when UserManager is available
                None
            }
            DialogId::Chat(chat_id) => self
                .chat_manager
                .get_input_peer_chat(chat_id, access_rights),
            DialogId::Channel(channel_id) => self
                .chat_manager
                .get_input_peer_channel(channel_id, access_rights),
            DialogId::SecretChat(_secret_chat_id) => {
                // TODO: Implement secret chat input peer when SecretChatManager is available
                None
            }
        }
    }

    /// Checks if we have an input peer for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `access_rights` - Required access rights
    ///
    /// # Returns
    ///
    /// `true` if we can construct an input peer
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `have_input_peer` method.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rustgram_chat_manager::AccessRights;
    /// # use rustgram_dialog_manager::DialogManager;
    /// # use rustgram_types::{ChatId, DialogId};
    /// #
    /// # let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.register_dialog(dialog_id, "Test".to_string()).unwrap();
    ///
    /// // Note: Returns false if chat not registered in ChatManager
    /// // assert!(manager.have_input_peer(dialog_id, AccessRights::Read));
    /// ```
    #[inline]
    #[must_use]
    pub fn have_input_peer(
        &self,
        dialog_id: DialogId,
        access_rights: rustgram_chat_manager::AccessRights,
    ) -> bool {
        self.get_input_peer(dialog_id, access_rights).is_some()
    }

    // ========== Message TTL Operations ==========

    /// Sets the message TTL for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `ttl` - Time-to-live in seconds (0 = forever)
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the dialog doesn't exist or lock cannot be acquired.
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `set_dialog_message_ttl_on_server`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.register_dialog(dialog_id, "Test".to_string()).unwrap();
    ///
    /// // Set auto-delete to 1 day
    /// manager.set_message_ttl(dialog_id, 86400).unwrap();
    /// ```
    pub fn set_message_ttl(&mut self, dialog_id: DialogId, ttl: MessageTtl) -> Result<()> {
        let mut state = self.state.write().map_err(|_| DialogError::LockError)?;

        let metadata = state
            .dialogs
            .get_mut(&dialog_id)
            .ok_or(DialogError::DialogNotFound(dialog_id))?;

        metadata.message_ttl = ttl;
        Ok(())
    }

    /// Gets the message TTL for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Returns
    ///
    /// `Some(ttl)` if the dialog exists
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.register_dialog(dialog_id, "Test".to_string()).unwrap();
    ///
    /// assert_eq!(manager.get_message_ttl(dialog_id), Some(0)); // Default = forever
    ///
    /// manager.set_message_ttl(dialog_id, 86400).unwrap();
    /// assert_eq!(manager.get_message_ttl(dialog_id), Some(86400));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_message_ttl(&self, dialog_id: DialogId) -> Option<MessageTtl> {
        self.state
            .read()
            .ok()?
            .dialogs
            .get(&dialog_id)
            .map(|m| m.message_ttl)
    }

    // ========== Pin/Unpin Operations ==========

    /// Pins a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `toggle_dialog_is_pinned_on_server`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.register_dialog(dialog_id, "Test".to_string()).unwrap();
    /// manager.pin_dialog(dialog_id).unwrap();
    ///
    /// assert!(manager.is_dialog_pinned(dialog_id));
    /// ```
    pub fn pin_dialog(&mut self, dialog_id: DialogId) -> Result<()> {
        let mut state = self.state.write().map_err(|_| DialogError::LockError)?;

        let metadata = state
            .dialogs
            .get_mut(&dialog_id)
            .ok_or(DialogError::DialogNotFound(dialog_id))?;

        metadata.is_pinned = true;
        Ok(())
    }

    /// Unpins a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.register_dialog(dialog_id, "Test".to_string()).unwrap();
    /// manager.pin_dialog(dialog_id).unwrap();
    ///
    /// manager.unpin_dialog(dialog_id).unwrap();
    ///
    /// assert!(!manager.is_dialog_pinned(dialog_id));
    /// ```
    pub fn unpin_dialog(&mut self, dialog_id: DialogId) -> Result<()> {
        let mut state = self.state.write().map_err(|_| DialogError::LockError)?;

        let metadata = state
            .dialogs
            .get_mut(&dialog_id)
            .ok_or(DialogError::DialogNotFound(dialog_id))?;

        metadata.is_pinned = false;
        Ok(())
    }

    /// Checks if a dialog is pinned.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Returns
    ///
    /// `true` if pinned
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.register_dialog(dialog_id, "Test".to_string()).unwrap();
    ///
    /// assert!(!manager.is_dialog_pinned(dialog_id));
    ///
    /// manager.pin_dialog(dialog_id).unwrap();
    /// assert!(manager.is_dialog_pinned(dialog_id));
    /// ```
    #[inline]
    #[must_use]
    pub fn is_dialog_pinned(&self, dialog_id: DialogId) -> bool {
        self.state
            .read()
            .map(|s| {
                s.dialogs
                    .get(&dialog_id)
                    .map(|m| m.is_pinned)
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    }

    // ========== Recently Opened/Found Dialogs ==========

    /// Adds a dialog to recently opened.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `on_dialog_opened`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.on_dialog_opened(dialog_id);
    ///
    /// let opened = manager.get_recently_opened(10);
    /// assert!(opened.contains(&dialog_id));
    /// ```
    pub fn on_dialog_opened(&mut self, dialog_id: DialogId) {
        if let Ok(mut state) = self.state.write() {
            state.recently_opened.retain(|&id| id != dialog_id);
            state.recently_opened.insert(0, dialog_id);
        }
    }

    /// Gets recently opened dialogs.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number to return
    ///
    /// # Returns
    ///
    /// Vector of dialog IDs
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.on_dialog_opened(dialog_id);
    ///
    /// let opened = manager.get_recently_opened(10);
    /// assert_eq!(opened.len(), 1);
    /// ```
    #[must_use]
    pub fn get_recently_opened(&self, limit: usize) -> Vec<DialogId> {
        self.state
            .read()
            .ok()
            .map(|s| s.recently_opened.iter().copied().take(limit).collect())
            .unwrap_or_default()
    }

    /// Adds a dialog to recently found.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.add_recently_found(dialog_id);
    ///
    /// let found = manager.get_recently_found(10);
    /// assert!(found.contains(&dialog_id));
    /// ```
    pub fn add_recently_found(&mut self, dialog_id: DialogId) {
        if let Ok(mut state) = self.state.write() {
            state.recently_found.retain(|&id| id != dialog_id);
            state.recently_found.insert(0, dialog_id);
        }
    }

    /// Removes a dialog from recently found.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.add_recently_found(dialog_id);
    /// manager.remove_recently_found(dialog_id);
    ///
    /// let found = manager.get_recently_found(10);
    /// assert!(!found.contains(&dialog_id));
    /// ```
    pub fn remove_recently_found(&mut self, dialog_id: DialogId) {
        if let Ok(mut state) = self.state.write() {
            state.recently_found.retain(|&id| id != dialog_id);
        }
    }

    /// Gets recently found dialogs.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number to return
    ///
    /// # Returns
    ///
    /// Vector of dialog IDs
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.add_recently_found(dialog_id);
    ///
    /// let found = manager.get_recently_found(10);
    /// assert_eq!(found.len(), 1);
    /// ```
    #[must_use]
    pub fn get_recently_found(&self, limit: usize) -> Vec<DialogId> {
        self.state
            .read()
            .ok()
            .map(|s| s.recently_found.iter().copied().take(limit).collect())
            .unwrap_or_default()
    }

    // ========== Utility Methods ==========

    /// Gets the number of dialogs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// assert_eq!(manager.dialog_count(), 0);
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.register_dialog(dialog_id, "Test".to_string()).unwrap();
    /// assert_eq!(manager.dialog_count(), 1);
    /// ```
    #[inline]
    #[must_use]
    pub fn dialog_count(&self) -> usize {
        self.state.read().map(|s| s.dialogs.len()).unwrap_or(0)
    }

    /// Clears all dialogs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::DialogManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.register_dialog(dialog_id, "Test".to_string()).unwrap();
    ///
    /// manager.clear().unwrap();
    /// assert_eq!(manager.dialog_count(), 0);
    /// ```
    pub fn clear(&mut self) -> Result<()> {
        let mut state = self.state.write().map_err(|_| DialogError::LockError)?;
        state.dialogs.clear();
        state.accessible_dialogs.clear();
        state.recently_opened.clear();
        state.recently_found.clear();
        state.username_cache.clear();
        Ok(())
    }

    // ========== Network Operations ==========

    /// Loads dialogs from the server.
    ///
    /// # Arguments
    ///
    /// * `client` - Network client for making requests
    /// * `pagination` - Pagination parameters
    /// * `limit` - Maximum dialogs to return (1-100)
    ///
    /// # Returns
    ///
    /// Paginated dialog list with continuation token
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Not authenticated
    /// - Network request fails
    /// - TL data is invalid
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustgram_dialog_manager::{DialogManager, DialogPagination, NetworkClient};
    /// use rustgram_net::NetQueryDispatcher;
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = DialogManager::new();
    /// let dispatcher = Arc::new(NetQueryDispatcher::new());
    /// let client = NetworkClient::new(dispatcher);
    ///
    /// // Load first page
    /// let (dialogs, pagination) = manager.load_dialogs(&client, None, 20).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load_dialogs(
        &self,
        client: &NetworkClient,
        pagination: Option<DialogPagination>,
        limit: usize,
    ) -> Result<(Vec<Dialog>, Option<DialogPagination>)> {
        // Validate limit
        let limit = limit.clamp(1, 100);

        // Build request
        let (offset_date, offset_id, offset_peer) = match pagination {
            Some(p) => (p.offset_date, p.offset_id, p.offset_peer),
            None => (0, 0, InputPeer::empty()),
        };

        let request = GetDialogsRequest {
            offset_date,
            offset_id,
            offset_peer,
            limit: limit as i32,
            hash: 0,
        };

        request.validate().map_err(DialogError::InvalidTlData)?;

        // Send request via network client with TL serialization
        let response: GetDialogsResponse = client
            .send_typed_query(&request, GetDialogsRequest::CONSTRUCTOR_ID)
            .await?;

        // Extract pagination info for next page
        let next_pagination = if response.dialogs.is_empty() {
            None
        } else {
            // Use last dialog for pagination
            let last_dialog = response
                .dialogs
                .last()
                .ok_or_else(|| DialogError::NetworkError("No dialogs in response".to_string()))?;

            let offset_peer = match last_dialog.peer {
                tl_types::Peer::User { user_id } => InputPeer::User {
                    user_id,
                    access_hash: rustgram_types::AccessHash::new(0),
                },
                tl_types::Peer::Chat { chat_id } => InputPeer::Chat(chat_id),
                tl_types::Peer::Channel { channel_id } => {
                    // Channel ID is u64, need to convert to i64 for ChannelId::new
                    let channel_id_i64 = channel_id as i64;
                    let channel_id_obj =
                        rustgram_types::ChannelId::new(channel_id_i64).map_err(|_| {
                            DialogError::NetworkError("Failed to create channel ID".to_string())
                        })?;
                    InputPeer::Channel {
                        channel_id: channel_id_obj,
                        access_hash: rustgram_types::AccessHash::new(0),
                    }
                }
                tl_types::Peer::Empty => InputPeer::empty(),
            };

            Some(DialogPagination {
                offset_date: 0, // Would need to extract from message
                offset_id: last_dialog.top_message,
                offset_peer,
            })
        };

        Ok((response.dialogs, next_pagination))
    }

    /// Creates a new group chat.
    ///
    /// # Arguments
    ///
    /// * `client` - Network client for making requests
    /// * `user_ids` - Users to add to chat
    /// * `title` - Chat title
    ///
    /// # Returns
    ///
    /// ID of created dialog
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Network request fails
    /// - Title is invalid
    /// - User IDs are empty
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustgram_dialog_manager::{DialogManager, NetworkClient};
    /// use rustgram_net::NetQueryDispatcher;
    /// use rustgram_types::UserId;
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = DialogManager::new();
    /// let dispatcher = Arc::new(NetQueryDispatcher::new());
    /// let client = NetworkClient::new(dispatcher);
    ///
    /// let user_ids = vec![UserId::new(123).unwrap()];
    /// let dialog_id = manager.create_dialog(&client, user_ids, "Test Group".to_string()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_dialog(
        &self,
        client: &NetworkClient,
        user_ids: Vec<UserId>,
        title: String,
    ) -> Result<DialogId> {
        // Validate title length
        if title.is_empty() {
            return Err(DialogError::EmptyTitle);
        }
        if title.len() > MAX_TITLE_LENGTH {
            return Err(DialogError::TitleTooLong {
                max: MAX_TITLE_LENGTH,
                len: title.len(),
            });
        }

        // Build request
        let request = CreateChatRequest {
            user_ids: user_ids.clone(),
            title: title.clone(),
            ttl_period: None,
        };

        request.validate().map_err(DialogError::InvalidTlData)?;

        // Send request via network client with TL serialization
        let _response: tl_types::Updates = client
            .send_typed_query(&request, CreateChatRequest::CONSTRUCTOR_ID)
            .await?;

        // TODO: Extract actual dialog ID from response
        // For now, return a placeholder
        let chat_id = ChatId::new(1)
            .map_err(|_| DialogError::NetworkError("Failed to create chat ID".to_string()))?;
        Ok(DialogId::from_chat(chat_id))
    }

    /// Updates dialog title on server.
    ///
    /// # Arguments
    ///
    /// * `client` - Network client for making requests
    /// * `dialog_id` - Dialog to update
    /// * `title` - New title
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Network request fails
    /// - Dialog not found
    /// - Title is invalid
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustgram_dialog_manager::{DialogManager, NetworkClient};
    /// use rustgram_net::NetQueryDispatcher;
    /// use rustgram_types::{ChatId, DialogId};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = DialogManager::new();
    /// let dispatcher = Arc::new(NetQueryDispatcher::new());
    /// let client = NetworkClient::new(dispatcher);
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.register_dialog(dialog_id, "Old Title".to_string())?;
    /// manager.update_dialog_title(&client, dialog_id, "New Title".to_string()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_dialog_title(
        &mut self,
        client: &NetworkClient,
        dialog_id: DialogId,
        title: String,
    ) -> Result<()> {
        // Check if dialog exists
        if !self.has_dialog(dialog_id) {
            return Err(DialogError::DialogNotFound(dialog_id));
        }

        // Validate title
        if title.is_empty() {
            return Err(DialogError::EmptyTitle);
        }
        if title.len() > MAX_TITLE_LENGTH {
            return Err(DialogError::TitleTooLong {
                max: MAX_TITLE_LENGTH,
                len: title.len(),
            });
        }

        // Extract chat_id from dialog_id
        let chat_id = match dialog_id {
            DialogId::Chat(cid) => cid,
            _ => {
                return Err(DialogError::InvalidInputPeer(dialog_id));
            }
        };

        // Build request
        let request = UpdateTitleRequest {
            chat_id,
            title: title.clone(),
        };

        request.validate().map_err(DialogError::InvalidTlData)?;

        // Send request via network client with TL serialization
        let _response: tl_types::Updates = client
            .send_typed_query(&request, UpdateTitleRequest::CONSTRUCTOR_ID)
            .await?;

        // Update local cache
        let mut state = self.state.write().map_err(|_| DialogError::LockError)?;

        if let Some(metadata) = state.dialogs.get_mut(&dialog_id) {
            metadata.title = title;
        }

        Ok(())
    }

    /// Updates dialog photo on server.
    ///
    /// # Arguments
    ///
    /// * `client` - Network client for making requests
    /// * `dialog_id` - Dialog to update
    /// * `photo` - New photo
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Network request fails
    /// - Dialog not found
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustgram_dialog_manager::{DialogManager, NetworkClient};
    /// use rustgram_net::NetQueryDispatcher;
    /// use rustgram_types::{ChatId, DialogId, InputPeer};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut manager = DialogManager::new();
    /// let dispatcher = Arc::new(NetQueryDispatcher::new());
    /// let client = NetworkClient::new(dispatcher);
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.update_dialog_photo(&client, dialog_id, InputPeer::empty()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_dialog_photo(
        &mut self,
        client: &NetworkClient,
        dialog_id: DialogId,
        photo: InputPeer,
    ) -> Result<()> {
        // Check if dialog exists
        if !self.has_dialog(dialog_id) {
            return Err(DialogError::DialogNotFound(dialog_id));
        }

        // Extract chat_id from dialog_id
        let chat_id = match dialog_id {
            DialogId::Chat(cid) => cid,
            _ => {
                return Err(DialogError::InvalidInputPeer(dialog_id));
            }
        };

        // Build request
        let request = UpdatePhotoRequest { chat_id, photo };

        // Send request via network client with TL serialization
        let _response: tl_types::Updates = client
            .send_typed_query(&request, UpdatePhotoRequest::CONSTRUCTOR_ID)
            .await?;

        Ok(())
    }

    /// Reloads a dialog from server (cache refresh).
    ///
    /// # Arguments
    ///
    /// * `client` - Network client for making requests
    /// * `dialog_id` - Dialog to reload
    ///
    /// # Returns
    ///
    /// Updated dialog information
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Network request fails
    /// - Dialog not found on server
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustgram_dialog_manager::{DialogManager, NetworkClient};
    /// use rustgram_net::NetQueryDispatcher;
    /// use rustgram_types::{ChatId, DialogId};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = DialogManager::new();
    /// let dispatcher = Arc::new(NetQueryDispatcher::new());
    /// let client = NetworkClient::new(dispatcher);
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// let dialog = manager.reload_dialog(&client, dialog_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn reload_dialog(
        &self,
        _client: &NetworkClient,
        dialog_id: DialogId,
    ) -> Result<Dialog> {
        // TODO: Implement reload from server
        // For now, return error
        Err(DialogError::DialogNotFound(dialog_id))
    }
}

impl fmt::Display for DialogManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dialog_count = self.dialog_count();
        write!(f, "DialogManager(dialogs: {})", dialog_count)
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-dialog-manager";

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_chat_manager::AccessRights;

    // ========== InputDialogId Tests ==========

    #[test]
    fn test_input_dialog_id_new() {
        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let input = InputDialogId::new(dialog_id);
        assert_eq!(input.get_dialog_id(), dialog_id);
    }

    #[test]
    fn test_input_dialog_id_is_valid() {
        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let input = InputDialogId::new(dialog_id);
        assert!(input.is_valid());
    }

    #[test]
    fn test_input_dialog_id_from_user_id() {
        let user_id = UserId::new(123).unwrap();
        let input = InputDialogId::from(user_id);

        assert_eq!(input.get_dialog_id().get_user_id(), Some(user_id));
    }

    #[test]
    fn test_input_dialog_id_from_chat_id() {
        let chat_id = ChatId::new(123).unwrap();
        let input = InputDialogId::from(chat_id);

        assert_eq!(input.get_dialog_id().get_chat_id(), Some(chat_id));
    }

    // ========== Manager Constructor Tests ==========

    #[test]
    fn test_manager_new() {
        let manager = DialogManager::new();
        assert_eq!(manager.dialog_count(), 0);
    }

    #[test]
    fn test_manager_default() {
        let manager = DialogManager::default();
        assert_eq!(manager.dialog_count(), 0);
    }

    // ========== Register Dialog Tests ==========

    #[test]
    fn test_register_dialog() {
        let mut manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager
            .register_dialog(dialog_id, "Test Group".to_string())
            .unwrap();

        assert!(manager.has_dialog(dialog_id));
        assert_eq!(manager.dialog_count(), 1);
    }

    #[test]
    fn test_register_dialog_empty_title() {
        let mut manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let result = manager.register_dialog(dialog_id, "".to_string());

        assert!(result.is_err());
        match result {
            Err(DialogError::EmptyTitle) => {}
            _ => panic!("Expected EmptyTitle error"),
        }
    }

    #[test]
    fn test_register_dialog_title_too_long() {
        let mut manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let long_title = "a".repeat(MAX_TITLE_LENGTH + 1);
        let result = manager.register_dialog(dialog_id, long_title);

        assert!(result.is_err());
        match result {
            Err(DialogError::TitleTooLong { max, len }) => {
                assert_eq!(max, MAX_TITLE_LENGTH);
                assert_eq!(len, MAX_TITLE_LENGTH + 1);
            }
            _ => panic!("Expected TitleTooLong error"),
        }
    }

    #[test]
    fn test_register_multiple_dialogs() {
        let mut manager = DialogManager::new();

        for i in 1..=5 {
            let chat_id = ChatId::new(i).unwrap();
            let dialog_id = DialogId::from_chat(chat_id);
            manager
                .register_dialog(dialog_id, format!("Chat {}", i))
                .unwrap();
        }

        assert_eq!(manager.dialog_count(), 5);
    }

    // ========== Has Dialog Tests ==========

    #[test]
    fn test_has_dialog() {
        let mut manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        assert!(!manager.has_dialog(dialog_id));

        manager
            .register_dialog(dialog_id, "Test".to_string())
            .unwrap();

        assert!(manager.has_dialog(dialog_id));
    }

    // ========== Get Dialog Title Tests ==========

    #[test]
    fn test_get_dialog_title() {
        let mut manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        assert_eq!(manager.get_dialog_title(dialog_id), None);

        manager
            .register_dialog(dialog_id, "Test Group".to_string())
            .unwrap();

        assert_eq!(
            manager.get_dialog_title(dialog_id),
            Some("Test Group".to_string())
        );
    }

    // ========== Have Dialog Access Tests ==========

    #[test]
    fn test_have_dialog_access() {
        let mut manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        assert!(!manager.have_dialog_access(dialog_id));

        manager
            .register_dialog(dialog_id, "Test".to_string())
            .unwrap();

        assert!(manager.have_dialog_access(dialog_id));
    }

    // ========== Get Input Peer Tests ==========

    #[test]
    fn test_get_input_peer_not_found() {
        let manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let peer = manager.get_input_peer(dialog_id, AccessRights::Read);
        assert!(peer.is_none());
    }

    // ========== Message TTL Tests ==========

    #[test]
    fn test_set_message_ttl() {
        let mut manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager
            .register_dialog(dialog_id, "Test".to_string())
            .unwrap();

        assert_eq!(
            manager.get_message_ttl(dialog_id),
            Some(MESSAGE_TTL_FOREVER)
        );

        manager.set_message_ttl(dialog_id, 86400).unwrap();

        assert_eq!(manager.get_message_ttl(dialog_id), Some(86400));
    }

    #[test]
    fn test_set_message_ttl_not_found() {
        let mut manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let result = manager.set_message_ttl(dialog_id, 86400);

        assert!(result.is_err());
    }

    // ========== Pin/Unpin Dialog Tests ==========

    #[test]
    fn test_pin_dialog() {
        let mut manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager
            .register_dialog(dialog_id, "Test".to_string())
            .unwrap();

        assert!(!manager.is_dialog_pinned(dialog_id));

        manager.pin_dialog(dialog_id).unwrap();

        assert!(manager.is_dialog_pinned(dialog_id));
    }

    #[test]
    fn test_unpin_dialog() {
        let mut manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager
            .register_dialog(dialog_id, "Test".to_string())
            .unwrap();
        manager.pin_dialog(dialog_id).unwrap();

        assert!(manager.is_dialog_pinned(dialog_id));

        manager.unpin_dialog(dialog_id).unwrap();

        assert!(!manager.is_dialog_pinned(dialog_id));
    }

    // ========== Recently Opened Tests ==========

    #[test]
    fn test_on_dialog_opened() {
        let mut manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager.on_dialog_opened(dialog_id);

        let opened = manager.get_recently_opened(10);
        assert!(opened.contains(&dialog_id));
    }

    #[test]
    fn test_get_recently_opened_limit() {
        let mut manager = DialogManager::new();

        for i in 1..=10 {
            let chat_id = ChatId::new(i).unwrap();
            let dialog_id = DialogId::from_chat(chat_id);
            manager.on_dialog_opened(dialog_id);
        }

        let opened = manager.get_recently_opened(5);
        assert_eq!(opened.len(), 5);
    }

    // ========== Recently Found Tests ==========

    #[test]
    fn test_add_recently_found() {
        let mut manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager.add_recently_found(dialog_id);

        let found = manager.get_recently_found(10);
        assert!(found.contains(&dialog_id));
    }

    #[test]
    fn test_remove_recently_found() {
        let mut manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager.add_recently_found(dialog_id);
        assert!(manager.get_recently_found(10).contains(&dialog_id));

        manager.remove_recently_found(dialog_id);
        assert!(!manager.get_recently_found(10).contains(&dialog_id));
    }

    // ========== Dialog Count Tests ==========

    #[test]
    fn test_dialog_count() {
        let mut manager = DialogManager::new();

        assert_eq!(manager.dialog_count(), 0);

        for i in 1..=5 {
            let chat_id = ChatId::new(i).unwrap();
            let dialog_id = DialogId::from_chat(chat_id);
            manager
                .register_dialog(dialog_id, format!("Chat {}", i))
                .unwrap();
        }

        assert_eq!(manager.dialog_count(), 5);
    }

    // ========== Clear Tests ==========

    #[test]
    fn test_clear() {
        let mut manager = DialogManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager
            .register_dialog(dialog_id, "Test".to_string())
            .unwrap();
        manager.on_dialog_opened(dialog_id);

        assert_eq!(manager.dialog_count(), 1);

        manager.clear().unwrap();

        assert_eq!(manager.dialog_count(), 0);
        assert!(manager.get_recently_opened(10).is_empty());
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_format() {
        let manager = DialogManager::new();
        let display = format!("{}", manager);

        assert!(display.contains("DialogManager"));
        assert!(display.contains("dialogs: 0"));
    }

    // ========== Constants Tests ==========

    #[test]
    fn test_constants() {
        assert_eq!(MAX_TITLE_LENGTH, 128);
        assert_eq!(MESSAGE_TTL_FOREVER, 0);
    }

    // ========== Integration Tests ==========

    #[test]
    fn test_full_workflow() {
        let mut manager = DialogManager::new();

        // Register dialogs
        let chat1 = ChatId::new(1).unwrap();
        let dialog1 = DialogId::from_chat(chat1);
        let chat2 = ChatId::new(2).unwrap();
        let dialog2 = DialogId::from_chat(chat2);

        manager
            .register_dialog(dialog1, "Chat 1".to_string())
            .unwrap();
        manager
            .register_dialog(dialog2, "Chat 2".to_string())
            .unwrap();

        assert_eq!(manager.dialog_count(), 2);

        // Pin first dialog
        manager.pin_dialog(dialog1).unwrap();
        assert!(manager.is_dialog_pinned(dialog1));
        assert!(!manager.is_dialog_pinned(dialog2));

        // Set TTL
        manager.set_message_ttl(dialog1, 3600).unwrap();
        assert_eq!(manager.get_message_ttl(dialog1), Some(3600));

        // Open dialog
        manager.on_dialog_opened(dialog1);
        assert!(manager.get_recently_opened(10).contains(&dialog1));

        // Add to recently found
        manager.add_recently_found(dialog2);
        assert!(manager.get_recently_found(10).contains(&dialog2));

        // Clear
        manager.clear().unwrap();
        assert_eq!(manager.dialog_count(), 0);
    }

    #[test]
    fn test_input_dialog_id_conversions() {
        let user_id = UserId::new(1).unwrap();
        let chat_id = ChatId::new(2).unwrap();
        let channel_id = ChannelId::new(3).unwrap();
        let secret_id = SecretChatId::new(4).unwrap();

        let from_user: InputDialogId = user_id.into();
        let from_chat: InputDialogId = chat_id.into();
        let from_channel: InputDialogId = channel_id.into();
        let from_secret: InputDialogId = secret_id.into();

        assert_eq!(from_user.get_dialog_id().get_user_id(), Some(user_id));
        assert_eq!(from_chat.get_dialog_id().get_chat_id(), Some(chat_id));
        assert_eq!(
            from_channel.get_dialog_id().get_channel_id(),
            Some(channel_id)
        );
        assert_eq!(
            from_secret.get_dialog_id().get_secret_chat_id(),
            Some(secret_id)
        );
    }
}
