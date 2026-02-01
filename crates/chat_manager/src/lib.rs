//! # Chat Manager
//!
//! Manages Telegram chat operations (basic groups, megagroups, channels).
//!
//! This module provides functionality for:
//! - Chat/Channel ID extraction from TL objects
//! - Input peer construction for API calls
//! - Chat metadata access (title, photo, permissions, etc.)
//! - Dialog participant management
//!
//! # Overview
//!
//! The ChatManager is responsible for managing all chat-related operations in Telegram.
//! It handles basic groups (small group chats), megagroups (large group chats),
//! and channels (broadcast channels).
//!
//! # Main Components
//!
//! - [`AccessRights`]: Permission levels for accessing chat data
//! - [`ChatMetadata`]: Metadata for basic group chats
//! - [`ChannelMetadata`]: Metadata for channels and megagroups
//! - [`ChatManager`]: Main manager for chat operations
//!
//! # Examples
//!
//! ```rust
//! use rustgram_chat_manager::{ChatManager, AccessRights};
//! use rustgram_types::{ChatId, ChannelId, UserId};
//!
//! // Create a manager
//! let mut manager = ChatManager::new();
//!
//! // Add a chat
//! let chat_id = ChatId::new(12345678).unwrap();
//! manager.add_chat(chat_id, "My Group".to_string());
//!
//! // Check if we have input peer for a chat
//! let has_read = manager.have_input_peer_chat(chat_id, AccessRights::Read);
//! ```
//!
//! # Access Rights
//!
//! Access rights control what operations can be performed on a chat:
//!
//! - `Know`: Basic knowledge that the chat exists
//! - `Read`: Can read messages in the chat
//! - `Edit`: Can edit chat metadata and settings
//! - `Write`: Can send messages and modify the chat
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

use rustgram_types::{ChannelId, ChatId, DialogId, InputPeer};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

pub mod error;
pub mod metadata;

pub use error::{ChatError, Result};
pub use metadata::{ChannelMetadata, ChatMetadata};

/// Access rights for chat operations.
///
/// Controls what level of access is required for various operations.
/// These correspond to the TDLib AccessRights enum.
///
/// # TDLib Alignment
///
/// From TDLib: AccessRights is used in `have_input_peer_chat` and `have_input_peer_channel`
/// to determine if we have sufficient access to perform operations on a chat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AccessRights {
    /// Know - Basic knowledge that the chat exists.
    ///
    /// This is the minimum access level, allowing us to know that
    /// a chat exists but not necessarily interact with it.
    Know,

    /// Read - Can read messages in the chat.
    ///
    /// This level allows reading messages and basic metadata.
    Read,

    /// Edit - Can edit chat metadata and settings.
    ///
    /// This level allows modifying chat information, settings,
    /// and administrative functions.
    Edit,

    /// Write - Full access including sending messages and modifying the chat.
    ///
    /// This is the highest access level, allowing all operations
    /// including sending messages and modifying the chat.
    Write,
}

impl Default for AccessRights {
    fn default() -> Self {
        Self::Know
    }
}

impl fmt::Display for AccessRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Know => write!(f, "Know"),
            Self::Read => write!(f, "Read"),
            Self::Edit => write!(f, "Edit"),
            Self::Write => write!(f, "Write"),
        }
    }
}

impl AccessRights {
    /// Checks if this access level meets or exceeds the required level.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::AccessRights;
    ///
    /// let read = AccessRights::Read;
    /// assert!(read.meets(AccessRights::Know));
    /// assert!(!read.meets(AccessRights::Edit));
    /// ```
    #[inline]
    #[must_use]
    pub const fn meets(self, required: Self) -> bool {
        self as u8 >= required as u8
    }
}

/// Chat manager state.
///
/// Internal state managed by the ChatManager.
/// Uses Arc<RwLock<T>> for thread-safe shared access.
#[derive(Debug, Clone)]
pub struct ChatManagerState {
    /// Map of chat IDs to their metadata.
    pub chats: HashMap<ChatId, ChatMetadata>,
    /// Map of channel IDs to their metadata.
    pub channels: HashMap<ChannelId, ChannelMetadata>,
}

impl Default for ChatManagerState {
    fn default() -> Self {
        Self {
            chats: HashMap::new(),
            channels: HashMap::new(),
        }
    }
}

/// Chat manager for Telegram chat operations.
///
/// Manages basic groups, megagroups, and channels.
/// Provides methods for accessing chat metadata and constructing input peers for API calls.
///
/// # Thread Safety
///
/// This manager uses `Arc<RwLock<T>>` internally, making it safe to share across threads.
/// However, the manager itself is not thread-safe - you should clone it or use Arc
/// if sharing across threads is needed.
///
/// # Examples
///
/// ```
/// use rustgram_chat_manager::{ChatManager, AccessRights};
/// use rustgram_types::{ChatId, ChannelId};
///
/// let mut manager = ChatManager::new();
///
/// let chat_id = ChatId::new(123).unwrap();
/// manager.add_chat(chat_id, "Test Group".to_string());
///
/// assert!(manager.have_chat(chat_id));
/// ```
#[derive(Debug, Clone)]
pub struct ChatManager {
    /// Internal state protected by RwLock for thread-safe access.
    state: Arc<std::sync::RwLock<ChatManagerState>>,
}

impl Default for ChatManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatManager {
    /// Creates a new chat manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChatManager;
    ///
    /// let manager = ChatManager::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(std::sync::RwLock::new(ChatManagerState::default())),
        }
    }

    // ========== Chat Operations ==========

    /// Adds a chat to the manager.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat ID
    /// * `title` - The chat title
    ///
    /// # Returns
    ///
    /// `Ok(())` if the chat was added successfully
    ///
    /// # Errors
    ///
    /// Returns an error if the write lock cannot be acquired.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChatManager;
    /// use rustgram_types::ChatId;
    ///
    /// let mut manager = ChatManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    ///
    /// manager.add_chat(chat_id, "My Group".to_string()).unwrap();
    /// assert!(manager.have_chat(chat_id));
    /// ```
    pub fn add_chat(&mut self, chat_id: ChatId, title: String) -> Result<()> {
        let mut state = self.state.write().map_err(|_| ChatError::LockError)?;
        let metadata = ChatMetadata::new(title);
        state.chats.insert(chat_id, metadata);
        Ok(())
    }

    /// Checks if a chat exists in the manager.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat ID to check
    ///
    /// # Returns
    ///
    /// `true` if the chat exists
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChatManager;
    /// use rustgram_types::ChatId;
    ///
    /// let mut manager = ChatManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    ///
    /// assert!(!manager.have_chat(chat_id));
    /// manager.add_chat(chat_id, "Test".to_string()).unwrap();
    /// assert!(manager.have_chat(chat_id));
    /// ```
    #[inline]
    #[must_use]
    pub fn have_chat(&self, chat_id: ChatId) -> bool {
        self.state
            .read()
            .map(|s| s.chats.contains_key(&chat_id))
            .unwrap_or(false)
    }

    /// Gets the title of a chat.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat ID
    ///
    /// # Returns
    ///
    /// `Some(title)` if the chat exists, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChatManager;
    /// use rustgram_types::ChatId;
    ///
    /// let mut manager = ChatManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    /// manager.add_chat(chat_id, "My Group".to_string()).unwrap();
    ///
    /// assert_eq!(manager.get_chat_title(chat_id), Some("My Group".to_string()));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_chat_title(&self, chat_id: ChatId) -> Option<String> {
        self.state
            .read()
            .ok()?
            .chats
            .get(&chat_id)
            .map(|m| m.title.clone())
    }

    /// Checks if we have input peer access for a chat.
    ///
    /// This determines if we have sufficient access rights to construct
    /// an InputPeer for API calls.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat ID
    /// * `access_rights` - The required access level
    ///
    /// # Returns
    ///
    /// `true` if we have input peer access at the specified level
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `ChatManager::have_input_peer_chat`.
    /// Basic groups (chats) don't require access hashes, so we just need
    /// to know about the chat.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::{ChatManager, AccessRights};
    /// use rustgram_types::ChatId;
    ///
    /// let mut manager = ChatManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    /// manager.add_chat(chat_id, "Test".to_string()).unwrap();
    ///
    /// assert!(manager.have_input_peer_chat(chat_id, AccessRights::Read));
    /// ```
    #[inline]
    #[must_use]
    pub fn have_input_peer_chat(&self, chat_id: ChatId, _access_rights: AccessRights) -> bool {
        // Basic groups don't require access hash, just need to know about the chat
        self.have_chat(chat_id)
    }

    /// Gets an input peer for a chat.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat ID
    /// * `access_rights` - The required access level
    ///
    /// # Returns
    ///
    /// `Some(InputPeer)` if we have access, `None` otherwise
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `ChatManager::get_input_peer_chat`.
    /// For basic groups, this returns `InputPeer::Chat` without an access hash.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::{ChatManager, AccessRights};
    /// use rustgram_types::{ChatId, InputPeer};
    ///
    /// let mut manager = ChatManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    /// manager.add_chat(chat_id, "Test".to_string()).unwrap();
    ///
    /// let peer = manager.get_input_peer_chat(chat_id, AccessRights::Read);
    /// assert!(peer.is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn get_input_peer_chat(
        &self,
        chat_id: ChatId,
        _access_rights: AccessRights,
    ) -> Option<InputPeer> {
        if self.have_input_peer_chat(chat_id, AccessRights::Know) {
            Some(InputPeer::Chat(chat_id))
        } else {
            None
        }
    }

    /// Gets chat metadata.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat ID
    ///
    /// # Returns
    ///
    /// `Some(&ChatMetadata)` if the chat exists, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChatManager;
    /// use rustgram_types::ChatId;
    ///
    /// let mut manager = ChatManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    /// manager.add_chat(chat_id, "Test".to_string()).unwrap();
    ///
    /// let metadata = manager.get_chat(chat_id);
    /// assert!(metadata.is_some());
    /// ```
    #[inline]
    pub fn get_chat(&self, chat_id: ChatId) -> Option<ChatMetadata> {
        self.state.read().ok()?.chats.get(&chat_id).cloned()
    }

    // ========== Channel Operations ==========

    /// Adds a channel to the manager.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `metadata` - The channel metadata
    ///
    /// # Returns
    ///
    /// `Ok(())` if the channel was added successfully
    ///
    /// # Errors
    ///
    /// Returns an error if the write lock cannot be acquired.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::{ChatManager, ChannelMetadata};
    /// use rustgram_types::{access::AccessHash, ChannelId};
    ///
    /// let mut manager = ChatManager::new();
    /// let channel_id = ChannelId::new(123).unwrap();
    /// let metadata = ChannelMetadata::new("My Channel".to_string(), AccessHash::new(456));
    ///
    /// manager.add_channel(channel_id, metadata).unwrap();
    /// assert!(manager.have_channel(channel_id));
    /// ```
    pub fn add_channel(&mut self, channel_id: ChannelId, metadata: ChannelMetadata) -> Result<()> {
        let mut state = self.state.write().map_err(|_| ChatError::LockError)?;
        state.channels.insert(channel_id, metadata);
        Ok(())
    }

    /// Checks if a channel exists in the manager.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID to check
    ///
    /// # Returns
    ///
    /// `true` if the channel exists
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::{ChatManager, ChannelMetadata};
    /// use rustgram_types::{access::AccessHash, ChannelId};
    ///
    /// let mut manager = ChatManager::new();
    /// let channel_id = ChannelId::new(123).unwrap();
    ///
    /// assert!(!manager.have_channel(channel_id));
    ///
    /// let metadata = ChannelMetadata::new("Test".to_string(), AccessHash::new(789));
    /// manager.add_channel(channel_id, metadata).unwrap();
    /// assert!(manager.have_channel(channel_id));
    /// ```
    #[inline]
    #[must_use]
    pub fn have_channel(&self, channel_id: ChannelId) -> bool {
        self.state
            .read()
            .map(|s| s.channels.contains_key(&channel_id))
            .unwrap_or(false)
    }

    /// Gets the title of a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// `Some(title)` if the channel exists, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::{ChatManager, ChannelMetadata};
    /// use rustgram_types::{access::AccessHash, ChannelId};
    ///
    /// let mut manager = ChatManager::new();
    /// let channel_id = ChannelId::new(123).unwrap();
    /// let metadata = ChannelMetadata::new("My Channel".to_string(), AccessHash::new(456));
    /// manager.add_channel(channel_id, metadata).unwrap();
    ///
    /// assert_eq!(manager.get_channel_title(channel_id), Some("My Channel".to_string()));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_channel_title(&self, channel_id: ChannelId) -> Option<String> {
        self.state
            .read()
            .ok()?
            .channels
            .get(&channel_id)
            .map(|m| m.title.clone())
    }

    /// Checks if we have input peer access for a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `access_rights` - The required access level
    ///
    /// # Returns
    ///
    /// `true` if we have input peer access at the specified level
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `ChatManager::have_input_peer_channel`.
    /// Channels require both that we know about the channel and have a valid access hash.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::{ChatManager, ChannelMetadata, AccessRights};
    /// use rustgram_types::{access::AccessHash, ChannelId};
    ///
    /// let mut manager = ChatManager::new();
    /// let channel_id = ChannelId::new(123).unwrap();
    /// let metadata = ChannelMetadata::new("Test".to_string(), AccessHash::new(456));
    /// manager.add_channel(channel_id, metadata).unwrap();
    ///
    /// assert!(manager.have_input_peer_channel(channel_id, AccessRights::Read));
    /// ```
    #[inline]
    #[must_use]
    pub fn have_input_peer_channel(
        &self,
        channel_id: ChannelId,
        _access_rights: AccessRights,
    ) -> bool {
        self.state
            .read()
            .ok()
            .and_then(|s| {
                s.channels
                    .get(&channel_id)
                    .map(|m| m.access_hash.is_valid())
            })
            .unwrap_or(false)
    }

    /// Gets an input peer for a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    /// * `access_rights` - The required access level
    ///
    /// # Returns
    ///
    /// `Some(InputPeer)` if we have access, `None` otherwise
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `ChatManager::get_input_peer_channel`.
    /// Returns `InputPeer::Channel` with the access hash if available.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::{ChatManager, ChannelMetadata, AccessRights};
    /// use rustgram_types::{access::AccessHash, ChannelId};
    ///
    /// let mut manager = ChatManager::new();
    /// let channel_id = ChannelId::new(123).unwrap();
    /// let metadata = ChannelMetadata::new("Test".to_string(), AccessHash::new(456));
    /// manager.add_channel(channel_id, metadata).unwrap();
    ///
    /// let peer = manager.get_input_peer_channel(channel_id, AccessRights::Read);
    /// assert!(peer.is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn get_input_peer_channel(
        &self,
        channel_id: ChannelId,
        _access_rights: AccessRights,
    ) -> Option<InputPeer> {
        let state = self.state.read().ok()?;
        let metadata = state.channels.get(&channel_id)?;
        if metadata.access_hash.is_valid() {
            Some(InputPeer::Channel {
                channel_id,
                access_hash: metadata.access_hash,
            })
        } else {
            None
        }
    }

    /// Gets channel metadata.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// `Some(&ChannelMetadata)` if the channel exists, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::{ChatManager, ChannelMetadata};
    /// use rustgram_types::{access::AccessHash, ChannelId};
    ///
    /// let mut manager = ChatManager::new();
    /// let channel_id = ChannelId::new(123).unwrap();
    /// let metadata = ChannelMetadata::new("Test".to_string(), AccessHash::new(456));
    /// manager.add_channel(channel_id, metadata).unwrap();
    ///
    /// let retrieved = manager.get_channel(channel_id);
    /// assert!(retrieved.is_some());
    /// ```
    #[inline]
    pub fn get_channel(&self, channel_id: ChannelId) -> Option<ChannelMetadata> {
        self.state.read().ok()?.channels.get(&channel_id).cloned()
    }

    // ========== Dialog Operations ==========

    /// Extracts the DialogId from a chat.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat ID
    ///
    /// # Returns
    ///
    /// `Some(DialogId)` if the chat is valid, `None` otherwise
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `ChatManager::get_dialog_id` for chat objects.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChatManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = ChatManager::get_dialog_id_from_chat(chat_id);
    ///
    /// assert_eq!(dialog_id, Some(DialogId::from_chat(chat_id)));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_dialog_id_from_chat(chat_id: ChatId) -> Option<DialogId> {
        Some(DialogId::from_chat(chat_id))
    }

    /// Extracts the DialogId from a channel.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID
    ///
    /// # Returns
    ///
    /// `Some(DialogId)` if the channel is valid, `None` otherwise
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `ChatManager::get_dialog_id` for channel objects.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChatManager;
    /// use rustgram_types::{ChannelId, DialogId};
    ///
    /// let channel_id = ChannelId::new(123).unwrap();
    /// let dialog_id = ChatManager::get_dialog_id_from_channel(channel_id);
    ///
    /// assert_eq!(dialog_id, Some(DialogId::from_channel(channel_id)));
    /// ```
    #[inline]
    #[must_use]
    pub fn get_dialog_id_from_channel(channel_id: ChannelId) -> Option<DialogId> {
        Some(DialogId::from_channel(channel_id))
    }

    /// Gets a simple input peer for a dialog ID.
    ///
    /// This is a simplified version that doesn't consider access rights.
    /// Returns the appropriate InputPeer based on the dialog type.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    ///
    /// # Returns
    ///
    /// `Some(InputPeer)` if we have the dialog, `None` otherwise
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `ChatManager::get_simple_input_peer`.
    /// This is a stub implementation that only returns peers we know about.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChatManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = ChatManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    /// manager.add_chat(chat_id, "Test".to_string()).unwrap();
    ///
    /// let dialog_id = DialogId::from_chat(chat_id);
    /// let peer = manager.get_simple_input_peer(dialog_id);
    ///
    /// assert!(peer.is_some());
    /// ```
    #[must_use]
    pub fn get_simple_input_peer(&self, dialog_id: DialogId) -> Option<InputPeer> {
        match dialog_id {
            DialogId::User(_user_id) => {
                // TODO: Implement user lookup when UserManager is available
                None
            }
            DialogId::Chat(chat_id) => self.get_input_peer_chat(chat_id, AccessRights::Know),
            DialogId::Channel(channel_id) => {
                self.get_input_peer_channel(channel_id, AccessRights::Know)
            }
            DialogId::SecretChat(_secret_chat_id) => {
                // TODO: Implement secret chat lookup when SecretChatManager is available
                None
            }
        }
    }

    // ========== State Management ==========

    /// Gets the number of chats in the manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChatManager;
    /// use rustgram_types::ChatId;
    ///
    /// let mut manager = ChatManager::new();
    /// assert_eq!(manager.chat_count(), 0);
    ///
    /// manager.add_chat(ChatId::new(1).unwrap(), "Test1".to_string()).unwrap();
    /// manager.add_chat(ChatId::new(2).unwrap(), "Test2".to_string()).unwrap();
    ///
    /// assert_eq!(manager.chat_count(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub fn chat_count(&self) -> usize {
        self.state.read().map(|s| s.chats.len()).unwrap_or(0)
    }

    /// Gets the number of channels in the manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::{ChatManager, ChannelMetadata};
    /// use rustgram_types::{access::AccessHash, ChannelId};
    ///
    /// let mut manager = ChatManager::new();
    /// assert_eq!(manager.channel_count(), 0);
    ///
    /// let metadata = ChannelMetadata::new("Test".to_string(), AccessHash::new(123));
    /// manager.add_channel(ChannelId::new(1).unwrap(), metadata).unwrap();
    ///
    /// assert_eq!(manager.channel_count(), 1);
    /// ```
    #[inline]
    #[must_use]
    pub fn channel_count(&self) -> usize {
        self.state.read().map(|s| s.channels.len()).unwrap_or(0)
    }

    /// Clears all chats and channels from the manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChatManager;
    /// use rustgram_types::ChatId;
    ///
    /// let mut manager = ChatManager::new();
    /// manager.add_chat(ChatId::new(1).unwrap(), "Test".to_string()).unwrap();
    ///
    /// manager.clear().unwrap();
    /// assert_eq!(manager.chat_count(), 0);
    /// ```
    pub fn clear(&mut self) -> Result<()> {
        let mut state = self.state.write().map_err(|_| ChatError::LockError)?;
        state.chats.clear();
        state.channels.clear();
        Ok(())
    }

    /// Removes a chat from the manager.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - The chat ID to remove
    ///
    /// # Returns
    ///
    /// `Ok(true)` if the chat was removed, `Ok(false)` if it didn't exist
    ///
    /// # Errors
    ///
    /// Returns an error if the write lock cannot be acquired.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::ChatManager;
    /// use rustgram_types::ChatId;
    ///
    /// let mut manager = ChatManager::new();
    /// let chat_id = ChatId::new(123).unwrap();
    /// manager.add_chat(chat_id, "Test".to_string()).unwrap();
    ///
    /// let removed = manager.remove_chat(chat_id).unwrap();
    /// assert!(removed);
    /// assert!(!manager.have_chat(chat_id));
    /// ```
    pub fn remove_chat(&mut self, chat_id: ChatId) -> Result<bool> {
        let mut state = self.state.write().map_err(|_| ChatError::LockError)?;
        Ok(state.chats.remove(&chat_id).is_some())
    }

    /// Removes a channel from the manager.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - The channel ID to remove
    ///
    /// # Returns
    ///
    /// `Ok(true)` if the channel was removed, `Ok(false)` if it didn't exist
    ///
    /// # Errors
    ///
    /// Returns an error if the write lock cannot be acquired.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_chat_manager::{ChatManager, ChannelMetadata};
    /// use rustgram_types::{access::AccessHash, ChannelId};
    ///
    /// let mut manager = ChatManager::new();
    /// let channel_id = ChannelId::new(123).unwrap();
    /// let metadata = ChannelMetadata::new("Test".to_string(), AccessHash::new(456));
    /// manager.add_channel(channel_id, metadata).unwrap();
    ///
    /// let removed = manager.remove_channel(channel_id).unwrap();
    /// assert!(removed);
    /// assert!(!manager.have_channel(channel_id));
    /// ```
    pub fn remove_channel(&mut self, channel_id: ChannelId) -> Result<bool> {
        let mut state = self.state.write().map_err(|_| ChatError::LockError)?;
        Ok(state.channels.remove(&channel_id).is_some())
    }
}

impl fmt::Display for ChatManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chat_count = self.chat_count();
        let channel_count = self.channel_count();
        write!(
            f,
            "ChatManager(chats: {}, channels: {})",
            chat_count, channel_count
        )
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-chat-manager";

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::{access::AccessHash, UserId};

    // ========== AccessRights Tests ==========

    #[test]
    fn test_access_rights_ordering() {
        assert!(AccessRights::Know < AccessRights::Read);
        assert!(AccessRights::Read < AccessRights::Edit);
        assert!(AccessRights::Edit < AccessRights::Write);
    }

    #[test]
    fn test_access_rights_meets() {
        assert!(AccessRights::Know.meets(AccessRights::Know));
        assert!(AccessRights::Read.meets(AccessRights::Know));
        assert!(AccessRights::Edit.meets(AccessRights::Read));
        assert!(AccessRights::Write.meets(AccessRights::Edit));

        assert!(!AccessRights::Know.meets(AccessRights::Read));
        assert!(!AccessRights::Read.meets(AccessRights::Edit));
        assert!(!AccessRights::Edit.meets(AccessRights::Write));
    }

    #[test]
    fn test_access_rights_display() {
        assert_eq!(format!("{}", AccessRights::Know), "Know");
        assert_eq!(format!("{}", AccessRights::Read), "Read");
        assert_eq!(format!("{}", AccessRights::Edit), "Edit");
        assert_eq!(format!("{}", AccessRights::Write), "Write");
    }

    #[test]
    fn test_access_rights_default() {
        assert_eq!(AccessRights::default(), AccessRights::Know);
    }

    // ========== Constructor Tests ==========

    #[test]
    fn test_manager_new() {
        let manager = ChatManager::new();
        assert_eq!(manager.chat_count(), 0);
        assert_eq!(manager.channel_count(), 0);
    }

    #[test]
    fn test_manager_default() {
        let manager = ChatManager::default();
        assert_eq!(manager.chat_count(), 0);
        assert_eq!(manager.channel_count(), 0);
    }

    // ========== Chat Operations Tests ==========

    #[test]
    fn test_add_chat() {
        let mut manager = ChatManager::new();
        let chat_id = ChatId::new(123).unwrap();

        let result = manager.add_chat(chat_id, "Test Group".to_string());
        assert!(result.is_ok());
        assert!(manager.have_chat(chat_id));
        assert_eq!(manager.chat_count(), 1);
    }

    #[test]
    fn test_add_multiple_chats() {
        let mut manager = ChatManager::new();

        for i in 1..=10 {
            let chat_id = ChatId::new(i).unwrap();
            manager.add_chat(chat_id, format!("Chat {}", i)).unwrap();
        }

        assert_eq!(manager.chat_count(), 10);
    }

    #[test]
    fn test_have_chat() {
        let mut manager = ChatManager::new();
        let chat_id = ChatId::new(123).unwrap();

        assert!(!manager.have_chat(chat_id));
        manager.add_chat(chat_id, "Test".to_string()).unwrap();
        assert!(manager.have_chat(chat_id));
    }

    #[test]
    fn test_get_chat_title() {
        let mut manager = ChatManager::new();
        let chat_id = ChatId::new(123).unwrap();

        assert_eq!(manager.get_chat_title(chat_id), None);

        manager.add_chat(chat_id, "My Group".to_string()).unwrap();
        assert_eq!(
            manager.get_chat_title(chat_id),
            Some("My Group".to_string())
        );
    }

    #[test]
    fn test_have_input_peer_chat() {
        let mut manager = ChatManager::new();
        let chat_id = ChatId::new(123).unwrap();

        assert!(!manager.have_input_peer_chat(chat_id, AccessRights::Read));

        manager.add_chat(chat_id, "Test".to_string()).unwrap();
        assert!(manager.have_input_peer_chat(chat_id, AccessRights::Know));
        assert!(manager.have_input_peer_chat(chat_id, AccessRights::Read));
        assert!(manager.have_input_peer_chat(chat_id, AccessRights::Edit));
        assert!(manager.have_input_peer_chat(chat_id, AccessRights::Write));
    }

    #[test]
    fn test_get_input_peer_chat() {
        let mut manager = ChatManager::new();
        let chat_id = ChatId::new(123).unwrap();

        assert_eq!(
            manager.get_input_peer_chat(chat_id, AccessRights::Read),
            None
        );

        manager.add_chat(chat_id, "Test".to_string()).unwrap();
        let peer = manager.get_input_peer_chat(chat_id, AccessRights::Read);
        assert!(peer.is_some());
        match peer {
            Some(InputPeer::Chat(id)) => assert_eq!(id, chat_id),
            _ => panic!("Expected Chat peer"),
        }
    }

    #[test]
    fn test_get_chat() {
        let mut manager = ChatManager::new();
        let chat_id = ChatId::new(123).unwrap();

        assert_eq!(manager.get_chat(chat_id), None);

        manager.add_chat(chat_id, "Test Group".to_string()).unwrap();
        let metadata = manager.get_chat(chat_id);
        assert!(metadata.is_some());
        assert_eq!(metadata.unwrap().title, "Test Group");
    }

    #[test]
    fn test_remove_chat() {
        let mut manager = ChatManager::new();
        let chat_id = ChatId::new(123).unwrap();

        manager.add_chat(chat_id, "Test".to_string()).unwrap();
        assert!(manager.have_chat(chat_id));

        let removed = manager.remove_chat(chat_id).unwrap();
        assert!(removed);
        assert!(!manager.have_chat(chat_id));
        assert_eq!(manager.chat_count(), 0);
    }

    #[test]
    fn test_remove_nonexistent_chat() {
        let mut manager = ChatManager::new();
        let chat_id = ChatId::new(123).unwrap();

        let removed = manager.remove_chat(chat_id).unwrap();
        assert!(!removed);
    }

    // ========== Channel Operations Tests ==========

    #[test]
    fn test_add_channel() {
        let mut manager = ChatManager::new();
        let channel_id = ChannelId::new(123).unwrap();
        let metadata = ChannelMetadata::new("Test Channel".to_string(), AccessHash::new(456));

        let result = manager.add_channel(channel_id, metadata);
        assert!(result.is_ok());
        assert!(manager.have_channel(channel_id));
        assert_eq!(manager.channel_count(), 1);
    }

    #[test]
    fn test_add_multiple_channels() {
        let mut manager = ChatManager::new();

        for i in 1..=10 {
            let channel_id = ChannelId::new(i).unwrap();
            let metadata =
                ChannelMetadata::new(format!("Channel {}", i), AccessHash::new(i as i64 * 1000));
            manager.add_channel(channel_id, metadata).unwrap();
        }

        assert_eq!(manager.channel_count(), 10);
    }

    #[test]
    fn test_have_channel() {
        let mut manager = ChatManager::new();
        let channel_id = ChannelId::new(123).unwrap();
        let metadata = ChannelMetadata::new("Test".to_string(), AccessHash::new(456));

        assert!(!manager.have_channel(channel_id));
        manager.add_channel(channel_id, metadata).unwrap();
        assert!(manager.have_channel(channel_id));
    }

    #[test]
    fn test_get_channel_title() {
        let mut manager = ChatManager::new();
        let channel_id = ChannelId::new(123).unwrap();
        let metadata = ChannelMetadata::new("My Channel".to_string(), AccessHash::new(456));

        assert_eq!(manager.get_channel_title(channel_id), None);

        manager.add_channel(channel_id, metadata).unwrap();
        assert_eq!(
            manager.get_channel_title(channel_id),
            Some("My Channel".to_string())
        );
    }

    #[test]
    fn test_have_input_peer_channel() {
        let mut manager = ChatManager::new();
        let channel_id = ChannelId::new(123).unwrap();
        let metadata = ChannelMetadata::new("Test".to_string(), AccessHash::new(456));

        assert!(!manager.have_input_peer_channel(channel_id, AccessRights::Read));

        manager.add_channel(channel_id, metadata).unwrap();
        assert!(manager.have_input_peer_channel(channel_id, AccessRights::Know));
        assert!(manager.have_input_peer_channel(channel_id, AccessRights::Read));
        assert!(manager.have_input_peer_channel(channel_id, AccessRights::Edit));
        assert!(manager.have_input_peer_channel(channel_id, AccessRights::Write));
    }

    #[test]
    fn test_get_input_peer_channel() {
        let mut manager = ChatManager::new();
        let channel_id = ChannelId::new(123).unwrap();
        let access_hash = AccessHash::new(456);
        let metadata = ChannelMetadata::new("Test".to_string(), access_hash);

        assert_eq!(
            manager.get_input_peer_channel(channel_id, AccessRights::Read),
            None
        );

        manager.add_channel(channel_id, metadata).unwrap();
        let peer = manager.get_input_peer_channel(channel_id, AccessRights::Read);
        assert!(peer.is_some());
        match peer {
            Some(InputPeer::Channel {
                channel_id: id,
                access_hash: hash,
            }) => {
                assert_eq!(id, channel_id);
                assert_eq!(hash, access_hash);
            }
            _ => panic!("Expected Channel peer"),
        }
    }

    #[test]
    fn test_get_channel() {
        let mut manager = ChatManager::new();
        let channel_id = ChannelId::new(123).unwrap();
        let metadata = ChannelMetadata::new("Test Channel".to_string(), AccessHash::new(456));

        assert_eq!(manager.get_channel(channel_id), None);

        manager.add_channel(channel_id, metadata.clone()).unwrap();
        let retrieved = manager.get_channel(channel_id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().title, "Test Channel");
    }

    #[test]
    fn test_remove_channel() {
        let mut manager = ChatManager::new();
        let channel_id = ChannelId::new(123).unwrap();
        let metadata = ChannelMetadata::new("Test".to_string(), AccessHash::new(456));

        manager.add_channel(channel_id, metadata).unwrap();
        assert!(manager.have_channel(channel_id));

        let removed = manager.remove_channel(channel_id).unwrap();
        assert!(removed);
        assert!(!manager.have_channel(channel_id));
        assert_eq!(manager.channel_count(), 0);
    }

    #[test]
    fn test_remove_nonexistent_channel() {
        let mut manager = ChatManager::new();
        let channel_id = ChannelId::new(123).unwrap();

        let removed = manager.remove_channel(channel_id).unwrap();
        assert!(!removed);
    }

    // ========== Dialog Operations Tests ==========

    #[test]
    fn test_get_dialog_id_from_chat() {
        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = ChatManager::get_dialog_id_from_chat(chat_id);

        assert_eq!(dialog_id, Some(DialogId::from_chat(chat_id)));
    }

    #[test]
    fn test_get_dialog_id_from_channel() {
        let channel_id = ChannelId::new(123).unwrap();
        let dialog_id = ChatManager::get_dialog_id_from_channel(channel_id);

        assert_eq!(dialog_id, Some(DialogId::from_channel(channel_id)));
    }

    #[test]
    fn test_get_simple_input_peer_for_chat() {
        let mut manager = ChatManager::new();
        let chat_id = ChatId::new(123).unwrap();
        manager.add_chat(chat_id, "Test".to_string()).unwrap();

        let dialog_id = DialogId::from_chat(chat_id);
        let peer = manager.get_simple_input_peer(dialog_id);

        assert!(peer.is_some());
        match peer {
            Some(InputPeer::Chat(id)) => assert_eq!(id, chat_id),
            _ => panic!("Expected Chat peer"),
        }
    }

    #[test]
    fn test_get_simple_input_peer_for_channel() {
        let mut manager = ChatManager::new();
        let channel_id = ChannelId::new(123).unwrap();
        let metadata = ChannelMetadata::new("Test".to_string(), AccessHash::new(456));
        manager.add_channel(channel_id, metadata).unwrap();

        let dialog_id = DialogId::from_channel(channel_id);
        let peer = manager.get_simple_input_peer(dialog_id);

        assert!(peer.is_some());
        match peer {
            Some(InputPeer::Channel { channel_id: id, .. }) => assert_eq!(id, channel_id),
            _ => panic!("Expected Channel peer"),
        }
    }

    #[test]
    fn test_get_simple_input_peer_for_unknown_user() {
        let manager = ChatManager::new();
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);

        let peer = manager.get_simple_input_peer(dialog_id);
        // User lookup not implemented yet
        assert!(peer.is_none());
    }

    // ========== State Management Tests ==========

    #[test]
    fn test_chat_count() {
        let mut manager = ChatManager::new();
        assert_eq!(manager.chat_count(), 0);

        for i in 1..=5 {
            manager
                .add_chat(ChatId::new(i).unwrap(), format!("Chat {}", i))
                .unwrap();
        }

        assert_eq!(manager.chat_count(), 5);
    }

    #[test]
    fn test_channel_count() {
        let mut manager = ChatManager::new();
        assert_eq!(manager.channel_count(), 0);

        for i in 1..=5 {
            let metadata =
                ChannelMetadata::new(format!("Channel {}", i), AccessHash::new(i as i64));
            manager
                .add_channel(ChannelId::new(i).unwrap(), metadata)
                .unwrap();
        }

        assert_eq!(manager.channel_count(), 5);
    }

    #[test]
    fn test_clear() {
        let mut manager = ChatManager::new();

        manager
            .add_chat(ChatId::new(1).unwrap(), "Chat1".to_string())
            .unwrap();
        manager
            .add_chat(ChatId::new(2).unwrap(), "Chat2".to_string())
            .unwrap();

        let metadata = ChannelMetadata::new("Channel1".to_string(), AccessHash::new(123));
        manager
            .add_channel(ChannelId::new(1).unwrap(), metadata)
            .unwrap();

        assert_eq!(manager.chat_count(), 2);
        assert_eq!(manager.channel_count(), 1);

        manager.clear().unwrap();

        assert_eq!(manager.chat_count(), 0);
        assert_eq!(manager.channel_count(), 0);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_format() {
        let mut manager = ChatManager::new();
        let display = format!("{}", manager);
        assert!(display.contains("ChatManager"));
        assert!(display.contains("chats: 0"));
        assert!(display.contains("channels: 0"));

        manager
            .add_chat(ChatId::new(1).unwrap(), "Test".to_string())
            .unwrap();
        let display = format!("{}", manager);
        assert!(display.contains("chats: 1"));
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_chat_metadata_new() {
        let metadata = ChatMetadata::new("Test Group".to_string());
        assert_eq!(metadata.title, "Test Group");
        assert_eq!(metadata.participant_count, 0);
        assert!(metadata.is_active);
    }

    #[test]
    fn test_channel_metadata_new() {
        let access_hash = AccessHash::new(12345);
        let metadata = ChannelMetadata::new("Test Channel".to_string(), access_hash);
        assert_eq!(metadata.title, "Test Channel");
        assert_eq!(metadata.access_hash, access_hash);
        assert!(!metadata.is_broadcast);
        assert!(!metadata.is_megagroup);
    }

    #[test]
    fn test_channel_metadata_with_flags() {
        let access_hash = AccessHash::new(12345);
        let mut metadata = ChannelMetadata::new("Test Channel".to_string(), access_hash);

        metadata.is_broadcast = true;
        metadata.is_megagroup = false;

        assert!(metadata.is_broadcast);
        assert!(!metadata.is_megagroup);
    }

    // ========== Integration Tests ==========

    #[test]
    fn test_mixed_chats_and_channels() {
        let mut manager = ChatManager::new();

        // Add chats
        for i in 1..=3 {
            let chat_id = ChatId::new(i).unwrap();
            manager.add_chat(chat_id, format!("Chat {}", i)).unwrap();
        }

        // Add channels
        for i in 1..=3 {
            let channel_id = ChannelId::new(i + 100).unwrap();
            let metadata =
                ChannelMetadata::new(format!("Channel {}", i), AccessHash::new(i as i64 * 1000));
            manager.add_channel(channel_id, metadata).unwrap();
        }

        assert_eq!(manager.chat_count(), 3);
        assert_eq!(manager.channel_count(), 3);

        // Verify all chats exist
        for i in 1..=3 {
            let chat_id = ChatId::new(i).unwrap();
            assert!(manager.have_chat(chat_id));
        }

        // Verify all channels exist
        for i in 1..=3 {
            let channel_id = ChannelId::new(i + 100).unwrap();
            assert!(manager.have_channel(channel_id));
        }
    }

    #[test]
    fn test_dialog_id_extraction() {
        let chat_id = ChatId::new(123).unwrap();
        let channel_id = ChannelId::new(456).unwrap();

        let chat_dialog_id = ChatManager::get_dialog_id_from_chat(chat_id).unwrap();
        let channel_dialog_id = ChatManager::get_dialog_id_from_channel(channel_id).unwrap();

        assert_eq!(chat_dialog_id.get_chat_id(), Some(chat_id));
        assert_eq!(channel_dialog_id.get_channel_id(), Some(channel_id));
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_and_crate_name() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-chat-manager");
    }

    #[test]
    fn test_access_rights_hash_and_eq() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(AccessRights::Read);
        set.insert(AccessRights::Read); // Duplicate
        set.insert(AccessRights::Write);

        assert_eq!(set.len(), 2);
        assert!(set.contains(&AccessRights::Read));
        assert!(set.contains(&AccessRights::Write));
        assert!(!set.contains(&AccessRights::Edit));
    }
}
