//! # Dialog Filter Manager
//!
//! Manages Telegram dialog filters (chat folders).
//!
//! This module provides functionality for:
//! - Creating, editing, and deleting dialog filters
//! - Managing pinned dialogs in filters
//! - Adding and removing dialogs from filters
//! - Synchronizing filters with server
//!
//! # Overview
//!
//! Dialog filters (also known as chat folders) allow users to organize their
//! chats into custom folders based on various criteria. Each filter can have
//! a name, icon, and set of included/excluded chats.
//!
//! # Main Components
//!
//! - [`DialogFilterManager`]: Main manager for dialog filter operations
//! - [`DialogFilter`]: Represents a single dialog filter
//! - [`DialogFilterId`]: Identifier for dialog filters
//!
//! # Examples
//!
//! ```rust
//! use rustgram_dialog_filter_manager::DialogFilterManager;
//! use rustgram_types::{ChatId, DialogId};
//!
//! let mut manager = DialogFilterManager::new();
//!
//! // Create a new filter
//! let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
//!
//! // Add a dialog to the filter
//! let chat_id = ChatId::new(123).unwrap();
//! manager.add_dialog_to_filter(filter_id, DialogId::from_chat(chat_id)).unwrap();
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

use rustgram_types::{ChatId, DialogId};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

pub mod error;
pub use error::{DialogFilterError, Result};

/// Maximum number of dialog filters allowed.
///
/// # TDLib Alignment
///
/// Corresponds to the server-side limit for dialog filters.
pub const MAX_FILTER_COUNT: usize = 10;

/// Maximum dialog filter name length.
///
/// # TDLib Alignment
///
/// Server-side limit for filter names.
pub const MAX_FILTER_NAME_LENGTH: usize = 12;

/// Minimum valid dialog filter ID.
///
/// # TDLib Alignment
///
/// From TDLib: DialogFilterId::min() returns 2.
pub const MIN_FILTER_ID: i32 = 2;

/// Maximum valid dialog filter ID.
///
/// # TDLib Alignment
///
/// From TDLib: DialogFilterId::max() returns 255.
pub const MAX_FILTER_ID: i32 = 255;

/// Dialog filter identifier.
///
/// Valid filter IDs are in the range [2, 255].
///
/// # TDLib Alignment
///
/// Corresponds to TDLib's `DialogFilterId` class.
/// Filter IDs start at 2 (ID 1 is reserved for the main folder).
///
/// # Examples
///
/// ```
/// use rustgram_dialog_filter_manager::DialogFilterId;
///
/// let filter_id = DialogFilterId::new(5).unwrap();
/// assert!(filter_id.is_valid());
/// assert_eq!(filter_id.get(), 5);
///
/// // Invalid IDs
/// assert!(DialogFilterId::new(1).is_err());
/// assert!(DialogFilterId::new(0).is_err());
/// assert!(DialogFilterId::new(256).is_err());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DialogFilterId(i32);

impl DialogFilterId {
    /// Creates a new dialog filter ID.
    ///
    /// Returns an error if the ID is not in the valid range [2, 255].
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterId;
    ///
    /// let id = DialogFilterId::new(10).unwrap();
    /// assert_eq!(id.get(), 10);
    /// ```
    #[inline]
    pub fn new(id: i32) -> Result<Self> {
        if (MIN_FILTER_ID..=MAX_FILTER_ID).contains(&id) {
            Ok(Self(id))
        } else {
            Err(DialogFilterError::InvalidFilterId(id))
        }
    }

    /// Returns the inner ID value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterId;
    ///
    /// let id = DialogFilterId::new(5).unwrap();
    /// assert_eq!(id.get(), 5);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Checks if this is a valid dialog filter ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterId;
    ///
    /// let id = DialogFilterId::new(5).unwrap();
    /// assert!(id.is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 >= MIN_FILTER_ID && self.0 <= MAX_FILTER_ID
    }
}

impl fmt::Display for DialogFilterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "filter {}", self.0)
    }
}

impl TryFrom<i32> for DialogFilterId {
    type Error = DialogFilterError;

    fn try_from(value: i32) -> Result<Self> {
        Self::new(value)
    }
}

impl From<DialogFilterId> for i32 {
    fn from(id: DialogFilterId) -> Self {
        id.0
    }
}

/// Dialog filter (chat folder).
///
/// Represents a user-created filter for organizing chats.
///
/// # TDLib Alignment
///
/// Corresponds to TDLib's `DialogFilter` class.
/// Simplified implementation with essential fields.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_filter_manager::{DialogFilter, DialogFilterId};
/// use rustgram_types::{ChatId, DialogId};
///
/// let filter = DialogFilter::new(
///     DialogFilterId::new(2).unwrap(),
///     "Work".to_string(),
///     vec![],
/// ).unwrap();
///
/// assert_eq!(filter.name(), "Work");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogFilter {
    /// Filter ID.
    pub id: DialogFilterId,
    /// Filter title (internal name).
    title: String,
    /// Included dialog IDs.
    pub included_dialogs: Vec<DialogId>,
    /// Pinned dialog IDs (in order).
    pub pinned_dialogs: Vec<DialogId>,
    /// Excluded dialog IDs.
    pub excluded_dialogs: Vec<DialogId>,
    /// Whether this filter is the main chat list.
    pub is_main: bool,
}

impl DialogFilter {
    /// Creates a new dialog filter.
    ///
    /// # Arguments
    ///
    /// * `id` - Filter ID
    /// * `name` - Filter name
    /// * `included_dialogs` - List of dialog IDs to include
    ///
    /// # Returns
    ///
    /// `Ok(filter)` if the name is valid
    ///
    /// # Errors
    ///
    /// Returns an error if the name is empty or too long.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::{DialogFilter, DialogFilterId};
    ///
    /// let filter = DialogFilter::new(
    ///     DialogFilterId::new(2).unwrap(),
    ///     "Work".to_string(),
    ///     vec![],
    /// ).unwrap();
    ///
    /// assert!(filter.is_valid());
    /// ```
    #[inline]
    pub fn new(id: DialogFilterId, name: String, included_dialogs: Vec<DialogId>) -> Result<Self> {
        if name.is_empty() {
            return Err(DialogFilterError::EmptyFilterName);
        }
        if name.len() > MAX_FILTER_NAME_LENGTH {
            return Err(DialogFilterError::FilterNameTooLong {
                max: MAX_FILTER_NAME_LENGTH,
                len: name.len(),
            });
        }

        Ok(Self {
            id,
            title: name,
            included_dialogs,
            pinned_dialogs: Vec::new(),
            excluded_dialogs: Vec::new(),
            is_main: false,
        })
    }

    /// Gets the filter name.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::{DialogFilter, DialogFilterId};
    ///
    /// let filter = DialogFilter::new(
    ///     DialogFilterId::new(2).unwrap(),
    ///     "Work".to_string(),
    ///     vec![],
    /// ).unwrap();
    ///
    /// assert_eq!(filter.name(), "Work");
    /// ```
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.title
    }

    /// Checks if this filter is valid.
    ///
    /// A filter is valid if it has a non-empty name.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::{DialogFilter, DialogFilterId};
    ///
    /// let filter = DialogFilter::new(
    ///     DialogFilterId::new(2).unwrap(),
    ///     "Work".to_string(),
    ///     vec![],
    /// ).unwrap();
    ///
    /// assert!(filter.is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.title.is_empty()
    }

    /// Checks if a dialog is included in this filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::{DialogFilter, DialogFilterId};
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    /// let filter = DialogFilter::new(
    ///     DialogFilterId::new(2).unwrap(),
    ///     "Work".to_string(),
    ///     vec![dialog_id],
    /// ).unwrap();
    ///
    /// assert!(filter.contains_dialog(dialog_id));
    /// ```
    #[inline]
    #[must_use]
    pub fn contains_dialog(&self, dialog_id: DialogId) -> bool {
        self.included_dialogs.contains(&dialog_id)
    }

    /// Checks if a dialog is pinned in this filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::{DialogFilter, DialogFilterId};
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    /// let mut filter = DialogFilter::new(
    ///     DialogFilterId::new(2).unwrap(),
    ///     "Work".to_string(),
    ///     vec![dialog_id],
    /// ).unwrap();
    ///
    /// filter.pinned_dialogs.push(dialog_id);
    /// assert!(filter.is_dialog_pinned(dialog_id));
    /// ```
    #[inline]
    #[must_use]
    pub fn is_dialog_pinned(&self, dialog_id: DialogId) -> bool {
        self.pinned_dialogs.contains(&dialog_id)
    }
}

/// Dialog filter manager state.
///
/// Internal state managed by the DialogFilterManager.
/// Uses Arc<RwLock<T>> for thread-safe shared access.
#[derive(Debug, Clone)]
pub struct DialogFilterManagerState {
    /// Map of filter IDs to their filters.
    pub filters: HashMap<DialogFilterId, DialogFilter>,
    /// Next available filter ID.
    pub next_filter_id: i32,
}

impl Default for DialogFilterManagerState {
    fn default() -> Self {
        Self {
            filters: HashMap::new(),
            next_filter_id: MIN_FILTER_ID,
        }
    }
}

/// Dialog filter manager for Telegram.
///
/// Manages dialog filters (chat folders) for organizing chats.
///
/// # Thread Safety
///
/// This manager uses `Arc<RwLock<T>>` internally, making it safe to share across threads.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_filter_manager::DialogFilterManager;
/// use rustgram_types::{ChatId, DialogId};
///
/// let mut manager = DialogFilterManager::new();
///
/// // Create a new filter
/// let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
///
/// // Add a dialog to the filter
/// let chat_id = ChatId::new(123).unwrap();
/// manager.add_dialog_to_filter(filter_id, DialogId::from_chat(chat_id)).unwrap();
///
/// // Get the filter
/// let filter = manager.get_filter(filter_id).unwrap();
/// assert_eq!(filter.name(), "Work");
/// ```
#[derive(Debug, Clone)]
pub struct DialogFilterManager {
    /// Internal state protected by RwLock for thread-safe access.
    state: Arc<std::sync::RwLock<DialogFilterManagerState>>,
}

impl Default for DialogFilterManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DialogFilterManager {
    /// Creates a new dialog filter manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterManager;
    ///
    /// let manager = DialogFilterManager::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(std::sync::RwLock::new(DialogFilterManagerState::default())),
        }
    }

    /// Creates a new dialog filter.
    ///
    /// # Arguments
    ///
    /// * `name` - Filter name
    /// * `included_dialogs` - List of dialog IDs to include
    ///
    /// # Returns
    ///
    /// `Ok(filter_id)` if successful
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The name is empty or too long
    /// - The filter limit is reached
    /// - The lock cannot be acquired
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterManager;
    ///
    /// let mut manager = DialogFilterManager::new();
    ///
    /// let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
    /// assert!(filter_id.is_valid());
    /// ```
    pub fn create_filter(
        &mut self,
        name: String,
        included_dialogs: Vec<DialogId>,
    ) -> Result<DialogFilterId> {
        let mut state = self
            .state
            .write()
            .map_err(|_| DialogFilterError::LockError)?;

        if state.filters.len() >= MAX_FILTER_COUNT {
            return Err(DialogFilterError::LimitExceeded {
                max: MAX_FILTER_COUNT,
                requested: state.filters.len() + 1,
            });
        }

        let filter_id = DialogFilterId::new(state.next_filter_id)?;
        let filter = DialogFilter::new(filter_id, name, included_dialogs)?;

        state.filters.insert(filter_id, filter);

        // Find next available ID
        loop {
            state.next_filter_id += 1;
            if state.next_filter_id > MAX_FILTER_ID {
                state.next_filter_id = MIN_FILTER_ID;
            }
            if !DialogFilterId::new(state.next_filter_id)
                .ok()
                .map(|id| state.filters.contains_key(&id))
                .unwrap_or(false)
            {
                break;
            }
        }

        Ok(filter_id)
    }

    /// Gets a dialog filter by ID.
    ///
    /// # Arguments
    ///
    /// * `filter_id` - Filter ID
    ///
    /// # Returns
    ///
    /// `Some(&filter)` if found, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterManager;
    ///
    /// let mut manager = DialogFilterManager::new();
    ///
    /// let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
    /// let filter = manager.get_filter(filter_id);
    ///
    /// assert!(filter.is_some());
    /// assert_eq!(filter.unwrap().name(), "Work");
    /// ```
    #[inline]
    #[must_use]
    pub fn get_filter(&self, filter_id: DialogFilterId) -> Option<DialogFilter> {
        self.state.read().ok()?.filters.get(&filter_id).cloned()
    }

    /// Edits an existing dialog filter.
    ///
    /// # Arguments
    ///
    /// * `filter_id` - Filter ID
    /// * `name` - New filter name
    /// * `included_dialogs` - New list of included dialog IDs
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The filter doesn't exist
    /// - The name is invalid
    /// - The lock cannot be acquired
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterManager;
    ///
    /// let mut manager = DialogFilterManager::new();
    ///
    /// let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
    ///
    /// manager.edit_filter(filter_id, "Personal".to_string(), vec![]).unwrap();
    ///
    /// let filter = manager.get_filter(filter_id).unwrap();
    /// assert_eq!(filter.name(), "Personal");
    /// ```
    pub fn edit_filter(
        &mut self,
        filter_id: DialogFilterId,
        name: String,
        included_dialogs: Vec<DialogId>,
    ) -> Result<()> {
        let mut state = self
            .state
            .write()
            .map_err(|_| DialogFilterError::LockError)?;

        if !state.filters.contains_key(&filter_id) {
            return Err(DialogFilterError::FilterNotFound(filter_id.get()));
        }

        let filter = DialogFilter::new(filter_id, name, included_dialogs)?;
        state.filters.insert(filter_id, filter);
        Ok(())
    }

    /// Deletes a dialog filter.
    ///
    /// # Arguments
    ///
    /// * `filter_id` - Filter ID to delete
    ///
    /// # Returns
    ///
    /// `Ok(true)` if deleted, `Ok(false)` if not found
    ///
    /// # Errors
    ///
    /// Returns an error if the lock cannot be acquired.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterManager;
    ///
    /// let mut manager = DialogFilterManager::new();
    ///
    /// let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
    ///
    /// let deleted = manager.delete_filter(filter_id).unwrap();
    /// assert!(deleted);
    ///
    /// let deleted = manager.delete_filter(filter_id).unwrap();
    /// assert!(!deleted);
    /// ```
    pub fn delete_filter(&mut self, filter_id: DialogFilterId) -> Result<bool> {
        let mut state = self
            .state
            .write()
            .map_err(|_| DialogFilterError::LockError)?;
        Ok(state.filters.remove(&filter_id).is_some())
    }

    /// Adds a dialog to a filter.
    ///
    /// # Arguments
    ///
    /// * `filter_id` - Filter ID
    /// * `dialog_id` - Dialog ID to add
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The filter doesn't exist
    /// - The dialog is already in the filter
    /// - The lock cannot be acquired
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogFilterManager::new();
    ///
    /// let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.add_dialog_to_filter(filter_id, dialog_id).unwrap();
    ///
    /// let filter = manager.get_filter(filter_id).unwrap();
    /// assert!(filter.contains_dialog(dialog_id));
    /// ```
    pub fn add_dialog_to_filter(
        &mut self,
        filter_id: DialogFilterId,
        dialog_id: DialogId,
    ) -> Result<()> {
        let mut state = self
            .state
            .write()
            .map_err(|_| DialogFilterError::LockError)?;

        let filter = state
            .filters
            .get_mut(&filter_id)
            .ok_or_else(|| DialogFilterError::FilterNotFound(filter_id.get()))?;

        if filter.contains_dialog(dialog_id) {
            return Ok(()); // Already in filter, no-op
        }

        filter.included_dialogs.push(dialog_id);
        Ok(())
    }

    /// Removes a dialog from a filter.
    ///
    /// # Arguments
    ///
    /// * `filter_id` - Filter ID
    /// * `dialog_id` - Dialog ID to remove
    ///
    /// # Returns
    ///
    /// `Ok(true)` if removed, `Ok(false)` if not in filter
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The filter doesn't exist
    /// - The lock cannot be acquired
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogFilterManager::new();
    ///
    /// let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.add_dialog_to_filter(filter_id, dialog_id).unwrap();
    ///
    /// let removed = manager.remove_dialog_from_filter(filter_id, dialog_id).unwrap();
    /// assert!(removed);
    ///
    /// let removed = manager.remove_dialog_from_filter(filter_id, dialog_id).unwrap();
    /// assert!(!removed);
    /// ```
    pub fn remove_dialog_from_filter(
        &mut self,
        filter_id: DialogFilterId,
        dialog_id: DialogId,
    ) -> Result<bool> {
        let mut state = self
            .state
            .write()
            .map_err(|_| DialogFilterError::LockError)?;

        let filter = state
            .filters
            .get_mut(&filter_id)
            .ok_or_else(|| DialogFilterError::FilterNotFound(filter_id.get()))?;

        if let Some(pos) = filter
            .included_dialogs
            .iter()
            .position(|&id| id == dialog_id)
        {
            filter.included_dialogs.remove(pos);
            // Also remove from pinned if present
            filter.pinned_dialogs.retain(|&id| id != dialog_id);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Pins a dialog in a filter.
    ///
    /// # Arguments
    ///
    /// * `filter_id` - Filter ID
    /// * `dialog_id` - Dialog ID to pin
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The filter doesn't exist
    /// - The dialog is not in the filter
    /// - The lock cannot be acquired
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogFilterManager::new();
    ///
    /// let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.add_dialog_to_filter(filter_id, dialog_id).unwrap();
    /// manager.pin_dialog_in_filter(filter_id, dialog_id).unwrap();
    ///
    /// let filter = manager.get_filter(filter_id).unwrap();
    /// assert!(filter.is_dialog_pinned(dialog_id));
    /// ```
    pub fn pin_dialog_in_filter(
        &mut self,
        filter_id: DialogFilterId,
        dialog_id: DialogId,
    ) -> Result<()> {
        let mut state = self
            .state
            .write()
            .map_err(|_| DialogFilterError::LockError)?;

        let filter = state
            .filters
            .get_mut(&filter_id)
            .ok_or_else(|| DialogFilterError::FilterNotFound(filter_id.get()))?;

        if !filter.contains_dialog(dialog_id) {
            // Dialog must be a chat for this error, otherwise use a default chat ID
            let chat_id = dialog_id
                .get_chat_id()
                .unwrap_or_else(|| ChatId::new(1).unwrap_or(ChatId(0)));
            return Err(DialogFilterError::DialogNotInFilter(chat_id));
        }

        if !filter.is_dialog_pinned(dialog_id) {
            filter.pinned_dialogs.push(dialog_id);
        }

        Ok(())
    }

    /// Unpins a dialog in a filter.
    ///
    /// # Arguments
    ///
    /// * `filter_id` - Filter ID
    /// * `dialog_id` - Dialog ID to unpin
    ///
    /// # Returns
    ///
    /// `Ok(true)` if unpinned, `Ok(false)` if not pinned
    ///
    /// # Errors
    ///
    /// Returns an error if the filter doesn't exist or lock cannot be acquired.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogFilterManager::new();
    ///
    /// let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.add_dialog_to_filter(filter_id, dialog_id).unwrap();
    /// manager.pin_dialog_in_filter(filter_id, dialog_id).unwrap();
    ///
    /// let unpinned = manager.unpin_dialog_in_filter(filter_id, dialog_id).unwrap();
    /// assert!(unpinned);
    /// ```
    pub fn unpin_dialog_in_filter(
        &mut self,
        filter_id: DialogFilterId,
        dialog_id: DialogId,
    ) -> Result<bool> {
        let mut state = self
            .state
            .write()
            .map_err(|_| DialogFilterError::LockError)?;

        let filter = state
            .filters
            .get_mut(&filter_id)
            .ok_or_else(|| DialogFilterError::FilterNotFound(filter_id.get()))?;

        if let Some(pos) = filter.pinned_dialogs.iter().position(|&id| id == dialog_id) {
            filter.pinned_dialogs.remove(pos);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Gets the number of filters.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterManager;
    ///
    /// let mut manager = DialogFilterManager::new();
    /// assert_eq!(manager.filter_count(), 0);
    ///
    /// manager.create_filter("Work".to_string(), vec![]).unwrap();
    /// assert_eq!(manager.filter_count(), 1);
    /// ```
    #[inline]
    #[must_use]
    pub fn filter_count(&self) -> usize {
        self.state.read().map(|s| s.filters.len()).unwrap_or(0)
    }

    /// Gets all filter IDs.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterManager;
    ///
    /// let mut manager = DialogFilterManager::new();
    ///
    /// manager.create_filter("Work".to_string(), vec![]).unwrap();
    /// manager.create_filter("Personal".to_string(), vec![]).unwrap();
    ///
    /// let ids = manager.get_all_filter_ids();
    /// assert_eq!(ids.len(), 2);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_all_filter_ids(&self) -> Vec<DialogFilterId> {
        self.state
            .read()
            .ok()
            .map(|s| {
                let mut ids: Vec<_> = s.filters.keys().copied().collect();
                ids.sort_by_key(|&id| id.get());
                ids
            })
            .unwrap_or_default()
    }

    /// Clears all filters.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_manager::DialogFilterManager;
    ///
    /// let mut manager = DialogFilterManager::new();
    ///
    /// manager.create_filter("Work".to_string(), vec![]).unwrap();
    /// assert_eq!(manager.filter_count(), 1);
    ///
    /// manager.clear().unwrap();
    /// assert_eq!(manager.filter_count(), 0);
    /// ```
    pub fn clear(&mut self) -> Result<()> {
        let mut state = self
            .state
            .write()
            .map_err(|_| DialogFilterError::LockError)?;
        state.filters.clear();
        state.next_filter_id = MIN_FILTER_ID;
        Ok(())
    }
}

impl fmt::Display for DialogFilterManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let filter_count = self.filter_count();
        write!(f, "DialogFilterManager(filters: {})", filter_count)
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-dialog-filter-manager";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== DialogFilterId Tests ==========

    #[test]
    fn test_filter_id_valid() {
        let id = DialogFilterId::new(5).unwrap();
        assert!(id.is_valid());
        assert_eq!(id.get(), 5);
    }

    #[test]
    fn test_filter_id_minimum() {
        let id = DialogFilterId::new(MIN_FILTER_ID).unwrap();
        assert!(id.is_valid());
        assert_eq!(id.get(), MIN_FILTER_ID);
    }

    #[test]
    fn test_filter_id_maximum() {
        let id = DialogFilterId::new(MAX_FILTER_ID).unwrap();
        assert!(id.is_valid());
        assert_eq!(id.get(), MAX_FILTER_ID);
    }

    #[test]
    fn test_filter_id_too_low() {
        assert!(DialogFilterId::new(1).is_err());
        assert!(DialogFilterId::new(0).is_err());
        assert!(DialogFilterId::new(-1).is_err());
    }

    #[test]
    fn test_filter_id_too_high() {
        assert!(DialogFilterId::new(256).is_err());
        assert!(DialogFilterId::new(1000).is_err());
    }

    #[test]
    fn test_filter_id_equality() {
        let id1 = DialogFilterId::new(5).unwrap();
        let id2 = DialogFilterId::new(5).unwrap();
        let id3 = DialogFilterId::new(6).unwrap();

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_filter_id_ordering() {
        let id1 = DialogFilterId::new(5).unwrap();
        let id2 = DialogFilterId::new(6).unwrap();

        assert!(id1 < id2);
    }

    #[test]
    fn test_filter_id_display() {
        let id = DialogFilterId::new(5).unwrap();
        assert_eq!(format!("{}", id), "filter 5");
    }

    // ========== DialogFilter Tests ==========

    #[test]
    fn test_filter_new() {
        let filter_id = DialogFilterId::new(2).unwrap();
        let filter = DialogFilter::new(filter_id, "Work".to_string(), vec![]).unwrap();

        assert_eq!(filter.id, filter_id);
        assert_eq!(filter.name(), "Work");
        assert!(filter.included_dialogs.is_empty());
        assert!(filter.pinned_dialogs.is_empty());
    }

    #[test]
    fn test_filter_empty_name() {
        let filter_id = DialogFilterId::new(2).unwrap();
        let result = DialogFilter::new(filter_id, "".to_string(), vec![]);

        assert!(result.is_err());
        match result {
            Err(DialogFilterError::EmptyFilterName) => {}
            _ => panic!("Expected EmptyFilterName error"),
        }
    }

    #[test]
    fn test_filter_name_too_long() {
        let filter_id = DialogFilterId::new(2).unwrap();
        let long_name = "a".repeat(MAX_FILTER_NAME_LENGTH + 1);
        let result = DialogFilter::new(filter_id, long_name, vec![]);

        assert!(result.is_err());
        match result {
            Err(DialogFilterError::FilterNameTooLong { max, len }) => {
                assert_eq!(max, MAX_FILTER_NAME_LENGTH);
                assert_eq!(len, MAX_FILTER_NAME_LENGTH + 1);
            }
            _ => panic!("Expected FilterNameTooLong error"),
        }
    }

    #[test]
    fn test_filter_with_dialogs() {
        let filter_id = DialogFilterId::new(2).unwrap();
        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let mut filter = DialogFilter::new(filter_id, "Work".to_string(), vec![]).unwrap();
        filter.included_dialogs.push(dialog_id);

        assert!(filter.contains_dialog(dialog_id));
    }

    #[test]
    fn test_filter_pinned_dialogs() {
        let filter_id = DialogFilterId::new(2).unwrap();
        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let mut filter = DialogFilter::new(filter_id, "Work".to_string(), vec![]).unwrap();
        filter.included_dialogs.push(dialog_id);
        filter.pinned_dialogs.push(dialog_id);

        assert!(filter.is_dialog_pinned(dialog_id));
    }

    #[test]
    fn test_filter_is_valid() {
        let filter_id = DialogFilterId::new(2).unwrap();
        let filter = DialogFilter::new(filter_id, "Work".to_string(), vec![]).unwrap();

        assert!(filter.is_valid());
    }

    // ========== Manager Constructor Tests ==========

    #[test]
    fn test_manager_new() {
        let manager = DialogFilterManager::new();
        assert_eq!(manager.filter_count(), 0);
    }

    #[test]
    fn test_manager_default() {
        let manager = DialogFilterManager::default();
        assert_eq!(manager.filter_count(), 0);
    }

    // ========== Create Filter Tests ==========

    #[test]
    fn test_create_filter() {
        let mut manager = DialogFilterManager::new();

        let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();

        assert!(filter_id.is_valid());
        assert_eq!(manager.filter_count(), 1);
    }

    #[test]
    fn test_create_filter_empty_name() {
        let mut manager = DialogFilterManager::new();

        let result = manager.create_filter("".to_string(), vec![]);

        assert!(result.is_err());
    }

    #[test]
    fn test_create_multiple_filters() {
        let mut manager = DialogFilterManager::new();

        for i in 0..5 {
            let name = format!("Filter{}", i);
            manager.create_filter(name, vec![]).unwrap();
        }

        assert_eq!(manager.filter_count(), 5);
    }

    #[test]
    fn test_create_filter_limit() {
        let mut manager = DialogFilterManager::new();

        for i in 0..MAX_FILTER_COUNT {
            let name = format!("Filter{}", i);
            manager.create_filter(name, vec![]).unwrap();
        }

        let result = manager.create_filter("TooMany".to_string(), vec![]);

        assert!(result.is_err());
        match result {
            Err(DialogFilterError::LimitExceeded { max, .. }) => {
                assert_eq!(max, MAX_FILTER_COUNT);
            }
            _ => panic!("Expected LimitExceeded error"),
        }
    }

    // ========== Get Filter Tests ==========

    #[test]
    fn test_get_filter() {
        let mut manager = DialogFilterManager::new();

        let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
        let filter = manager.get_filter(filter_id);

        assert!(filter.is_some());
        assert_eq!(filter.unwrap().name(), "Work");
    }

    #[test]
    fn test_get_filter_not_found() {
        let manager = DialogFilterManager::new();
        let filter_id = DialogFilterId::new(5).unwrap();

        let filter = manager.get_filter(filter_id);

        assert!(filter.is_none());
    }

    // ========== Edit Filter Tests ==========

    #[test]
    fn test_edit_filter() {
        let mut manager = DialogFilterManager::new();

        let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
        manager
            .edit_filter(filter_id, "Personal".to_string(), vec![])
            .unwrap();

        let filter = manager.get_filter(filter_id).unwrap();
        assert_eq!(filter.name(), "Personal");
    }

    #[test]
    fn test_edit_filter_not_found() {
        let mut manager = DialogFilterManager::new();
        let filter_id = DialogFilterId::new(5).unwrap();

        let result = manager.edit_filter(filter_id, "Work".to_string(), vec![]);

        assert!(result.is_err());
        match result {
            Err(DialogFilterError::FilterNotFound(id)) => {
                assert_eq!(id, 5);
            }
            _ => panic!("Expected FilterNotFound error"),
        }
    }

    // ========== Delete Filter Tests ==========

    #[test]
    fn test_delete_filter() {
        let mut manager = DialogFilterManager::new();

        let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
        assert_eq!(manager.filter_count(), 1);

        let deleted = manager.delete_filter(filter_id).unwrap();
        assert!(deleted);
        assert_eq!(manager.filter_count(), 0);
    }

    #[test]
    fn test_delete_filter_not_found() {
        let mut manager = DialogFilterManager::new();
        let filter_id = DialogFilterId::new(5).unwrap();

        let deleted = manager.delete_filter(filter_id).unwrap();

        assert!(!deleted);
    }

    // ========== Add/Remove Dialog Tests ==========

    #[test]
    fn test_add_dialog_to_filter() {
        let mut manager = DialogFilterManager::new();

        let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager.add_dialog_to_filter(filter_id, dialog_id).unwrap();

        let filter = manager.get_filter(filter_id).unwrap();
        assert!(filter.contains_dialog(dialog_id));
    }

    #[test]
    fn test_add_dialog_to_filter_twice() {
        let mut manager = DialogFilterManager::new();

        let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager.add_dialog_to_filter(filter_id, dialog_id).unwrap();
        manager.add_dialog_to_filter(filter_id, dialog_id).unwrap(); // Should be no-op

        let filter = manager.get_filter(filter_id).unwrap();
        assert_eq!(filter.included_dialogs.len(), 1);
    }

    #[test]
    fn test_remove_dialog_from_filter() {
        let mut manager = DialogFilterManager::new();

        let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager.add_dialog_to_filter(filter_id, dialog_id).unwrap();

        let removed = manager
            .remove_dialog_from_filter(filter_id, dialog_id)
            .unwrap();
        assert!(removed);

        let filter = manager.get_filter(filter_id).unwrap();
        assert!(!filter.contains_dialog(dialog_id));
    }

    #[test]
    fn test_remove_dialog_not_in_filter() {
        let mut manager = DialogFilterManager::new();

        let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let removed = manager
            .remove_dialog_from_filter(filter_id, dialog_id)
            .unwrap();

        assert!(!removed);
    }

    // ========== Pin/Unpin Dialog Tests ==========

    #[test]
    fn test_pin_dialog_in_filter() {
        let mut manager = DialogFilterManager::new();

        let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager.add_dialog_to_filter(filter_id, dialog_id).unwrap();
        manager.pin_dialog_in_filter(filter_id, dialog_id).unwrap();

        let filter = manager.get_filter(filter_id).unwrap();
        assert!(filter.is_dialog_pinned(dialog_id));
    }

    #[test]
    fn test_pin_dialog_not_in_filter() {
        let mut manager = DialogFilterManager::new();

        let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let result = manager.pin_dialog_in_filter(filter_id, dialog_id);

        assert!(result.is_err());
    }

    #[test]
    fn test_unpin_dialog_in_filter() {
        let mut manager = DialogFilterManager::new();

        let filter_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager.add_dialog_to_filter(filter_id, dialog_id).unwrap();
        manager.pin_dialog_in_filter(filter_id, dialog_id).unwrap();

        let unpinned = manager
            .unpin_dialog_in_filter(filter_id, dialog_id)
            .unwrap();
        assert!(unpinned);

        let filter = manager.get_filter(filter_id).unwrap();
        assert!(!filter.is_dialog_pinned(dialog_id));
    }

    // ========== Get All Filter IDs Tests ==========

    #[test]
    fn test_get_all_filter_ids() {
        let mut manager = DialogFilterManager::new();

        manager.create_filter("C".to_string(), vec![]).unwrap();
        manager.create_filter("A".to_string(), vec![]).unwrap();
        manager.create_filter("B".to_string(), vec![]).unwrap();

        let ids = manager.get_all_filter_ids();
        assert_eq!(ids.len(), 3);
    }

    // ========== Clear Tests ==========

    #[test]
    fn test_clear() {
        let mut manager = DialogFilterManager::new();

        manager.create_filter("Work".to_string(), vec![]).unwrap();
        manager
            .create_filter("Personal".to_string(), vec![])
            .unwrap();

        assert_eq!(manager.filter_count(), 2);

        manager.clear().unwrap();

        assert_eq!(manager.filter_count(), 0);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_format() {
        let manager = DialogFilterManager::new();
        let display = format!("{}", manager);

        assert!(display.contains("DialogFilterManager"));
        assert!(display.contains("filters: 0"));
    }

    // ========== Constants Tests ==========

    #[test]
    fn test_constants() {
        assert_eq!(MAX_FILTER_COUNT, 10);
        assert_eq!(MAX_FILTER_NAME_LENGTH, 12);
        assert_eq!(MIN_FILTER_ID, 2);
        assert_eq!(MAX_FILTER_ID, 255);
    }

    // ========== Integration Tests ==========

    #[test]
    fn test_full_workflow() {
        let mut manager = DialogFilterManager::new();

        // Create filters
        let work_id = manager.create_filter("Work".to_string(), vec![]).unwrap();
        let personal_id = manager
            .create_filter("Personal".to_string(), vec![])
            .unwrap();

        assert_eq!(manager.filter_count(), 2);

        // Add dialogs to Work filter
        let chat1 = ChatId::new(1).unwrap();
        let chat2 = ChatId::new(2).unwrap();
        manager
            .add_dialog_to_filter(work_id, DialogId::from_chat(chat1))
            .unwrap();
        manager
            .add_dialog_to_filter(work_id, DialogId::from_chat(chat2))
            .unwrap();

        // Pin one dialog
        manager
            .pin_dialog_in_filter(work_id, DialogId::from_chat(chat1))
            .unwrap();

        // Verify
        let work_filter = manager.get_filter(work_id).unwrap();
        assert_eq!(work_filter.included_dialogs.len(), 2);
        assert!(work_filter.is_dialog_pinned(DialogId::from_chat(chat1)));

        // Delete one filter
        manager.delete_filter(personal_id).unwrap();
        assert_eq!(manager.filter_count(), 1);
    }

    #[test]
    fn test_filter_id_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(DialogFilterId::new(5).unwrap());
        set.insert(DialogFilterId::new(5).unwrap()); // Duplicate
        set.insert(DialogFilterId::new(6).unwrap());

        assert_eq!(set.len(), 2);
    }
}
