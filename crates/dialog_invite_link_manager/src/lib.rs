//! # Dialog Invite Link Manager
//!
//! Manages Telegram dialog invite links.
//!
//! This module provides functionality for:
//! - Creating and managing invite links for chats/channels
//! - Exporting and editing invite links
//! - Tracking invite link usage statistics
//! - Managing join requests from invite links
//!
//! # Overview
//!
//! Invite links allow users to join chats and channels without being manually added.
//! This manager tracks invite links, their usage, and handles join requests.
//!
//! # Main Components
//!
//! - [`DialogInviteLinkManager`]: Main manager for invite link operations
//! - [`InviteLink`]: Represents a single invite link
//! - [`InviteLinkInfo`]: Cached information about an invite link
//!
//! # Examples
//!
//! ```rust
//! use rustgram_dialog_invite_link_manager::DialogInviteLinkManager;
//! use rustgram_types::{ChatId, DialogId};
//!
//! let mut manager = DialogInviteLinkManager::new();
//!
//! // Create a permanent invite link
//! let chat_id = ChatId::new(123).unwrap();
//! let dialog_id = DialogId::from_chat(chat_id);
//! let link = manager.create_permanent_link(dialog_id).unwrap();
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

use rustgram_types::{DialogId, UserId};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

pub mod error;
pub use error::{InviteLinkError, Result};

/// Maximum invite link title length.
///
/// # TDLib Alignment
///
/// Corresponds to TDLib's `DialogInviteLinkManager::MAX_INVITE_LINK_TITLE_LENGTH`.
pub const MAX_INVITE_LINK_TITLE_LENGTH: usize = 32;

/// Default expiration time for temporary invite links (in seconds).
///
/// 24 hours = 86400 seconds
pub const DEFAULT_EXPIRE_TIME: i32 = 86400;

/// Default usage limit for temporary invite links.
pub const DEFAULT_USAGE_LIMIT: i32 = 100;

/// Invite link for a dialog.
///
/// # TDLib Alignment
///
/// Corresponds to TDLib's `DialogInviteLink` class.
/// Simplified implementation with essential fields.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_invite_link_manager::InviteLink;
/// use rustgram_types::{ChatId, DialogId, UserId};
///
/// let link = InviteLink::new(
///     "https://t.me/+abcdef".to_string(),
///     UserId::new(123).unwrap(),
/// );
///
/// assert!(!link.is_revoked);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InviteLink {
    /// The invite link URL.
    pub link: String,
    /// User ID of the link creator.
    pub creator_id: UserId,
    /// Unix timestamp when the link was created.
    pub create_date: i32,
    /// Unix timestamp when the link expires, or 0 for never.
    pub expire_date: i32,
    /// Maximum number of uses, or 0 for unlimited.
    pub usage_limit: i32,
    /// Current number of times the link was used.
    pub usage_count: i32,
    /// Pending join request count.
    pub pending_join_request_count: i32,
    /// Link title.
    pub title: String,
    /// Whether this link requests users to join via approval.
    pub creates_join_request: bool,
    /// Whether the link is revoked.
    pub is_revoked: bool,
    /// Whether this is a permanent link.
    pub is_permanent: bool,
}

impl InviteLink {
    /// Creates a new invite link.
    ///
    /// # Arguments
    ///
    /// * `link` - The invite link URL
    /// * `creator_id` - User ID of the creator
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::InviteLink;
    /// use rustgram_types::UserId;
    ///
    /// let link = InviteLink::new(
    ///     "https://t.me/+abcdef".to_string(),
    ///     UserId::new(123).unwrap(),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn new(link: String, creator_id: UserId) -> Self {
        Self {
            link,
            creator_id,
            create_date: 0,
            expire_date: 0,
            usage_limit: 0,
            usage_count: 0,
            pending_join_request_count: 0,
            title: String::new(),
            creates_join_request: false,
            is_revoked: false,
            is_permanent: false,
        }
    }

    /// Creates a permanent invite link.
    ///
    /// # Arguments
    ///
    /// * `link` - The invite link URL
    /// * `creator_id` - User ID of the creator
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::InviteLink;
    /// use rustgram_types::UserId;
    ///
    /// let link = InviteLink::permanent(
    ///     "https://t.me/+abcdef".to_string(),
    ///     UserId::new(123).unwrap(),
    /// );
    ///
    /// assert!(link.is_permanent);
    /// ```
    #[inline]
    #[must_use]
    pub fn permanent(link: String, creator_id: UserId) -> Self {
        Self {
            link,
            creator_id,
            create_date: 0,
            expire_date: 0,
            usage_limit: 0,
            usage_count: 0,
            pending_join_request_count: 0,
            title: String::new(),
            creates_join_request: false,
            is_revoked: false,
            is_permanent: true,
        }
    }

    /// Checks if the link is expired.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::InviteLink;
    /// use rustgram_types::UserId;
    ///
    /// let mut link = InviteLink::new(
    ///     "https://t.me/+abcdef".to_string(),
    ///     UserId::new(123).unwrap(),
    /// );
    /// link.expire_date = 1234567890;
    ///
    /// // Whether expired depends on current time
    /// ```
    #[inline]
    #[must_use]
    pub fn is_expired(&self) -> bool {
        self.expire_date > 0 && self.expire_date < Self::current_time()
    }

    /// Checks if the link has reached its usage limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::InviteLink;
    /// use rustgram_types::UserId;
    ///
    /// let mut link = InviteLink::new(
    ///     "https://t.me/+abcdef".to_string(),
    ///     UserId::new(123).unwrap(),
    /// );
    /// link.usage_limit = 10;
    /// link.usage_count = 10;
    ///
    /// assert!(link.is_usage_limit_reached());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_usage_limit_reached(&self) -> bool {
        self.usage_limit > 0 && self.usage_count >= self.usage_limit
    }

    /// Checks if the link can still be used.
    ///
    /// A link can be used if it's not revoked, not expired, and hasn't reached its usage limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::InviteLink;
    /// use rustgram_types::UserId;
    ///
    /// let link = InviteLink::new(
    ///     "https://t.me/+abcdef".to_string(),
    ///     UserId::new(123).unwrap(),
    /// );
    ///
    /// assert!(link.is_active());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_active(&self) -> bool {
        !self.is_revoked && !self.is_expired() && !self.is_usage_limit_reached()
    }

    #[cfg(test)]
    fn current_time() -> i32 {
        // In tests, return a fixed time
        1_600_000_000
    }

    #[cfg(not(test))]
    fn current_time() -> i32 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0)
    }
}

/// Cached invite link information.
///
/// # TDLib Alignment
///
/// Corresponds to TDLib's `InviteLinkInfo` struct.
#[derive(Debug, Clone)]
pub struct InviteLinkInfo {
    /// Dialog ID if known.
    pub dialog_id: Option<DialogId>,
    /// Chat title.
    pub title: String,
    /// Whether this is a channel (vs group).
    pub is_channel: bool,
    /// Whether the channel is a megagroup.
    pub is_megagroup: bool,
    /// Participant count.
    pub participant_count: i32,
    /// List of participant user IDs.
    pub participant_user_ids: Vec<UserId>,
    /// Whether joining requires approval.
    pub creates_join_request: bool,
}

impl Default for InviteLinkInfo {
    fn default() -> Self {
        Self {
            dialog_id: None,
            title: String::new(),
            is_channel: false,
            is_megagroup: false,
            participant_count: 0,
            participant_user_ids: Vec::new(),
            creates_join_request: false,
        }
    }
}

/// Dialog invite link manager state.
///
/// Internal state managed by the DialogInviteLinkManager.
/// Uses Arc<RwLock<T>> for thread-safe shared access.
#[derive(Debug, Clone)]
pub struct DialogInviteLinkManagerState {
    /// Map of dialog IDs to their permanent invite links.
    pub permanent_links: HashMap<DialogId, InviteLink>,
    /// Map of dialog IDs to their invite links (including temporary).
    pub all_links: HashMap<DialogId, Vec<InviteLink>>,
    /// Cached invite link information.
    pub invite_link_infos: HashMap<String, InviteLinkInfo>,
}

impl Default for DialogInviteLinkManagerState {
    fn default() -> Self {
        Self {
            permanent_links: HashMap::new(),
            all_links: HashMap::new(),
            invite_link_infos: HashMap::new(),
        }
    }
}

/// Dialog invite link manager for Telegram.
///
/// Manages invite links for chats and channels.
///
/// # Thread Safety
///
/// This manager uses `Arc<RwLock<T>>` internally, making it safe to share across threads.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_invite_link_manager::DialogInviteLinkManager;
/// use rustgram_types::{ChatId, DialogId};
///
/// let mut manager = DialogInviteLinkManager::new();
///
/// let chat_id = ChatId::new(123).unwrap();
/// let dialog_id = DialogId::from_chat(chat_id);
///
/// let link = manager.create_permanent_link(dialog_id).unwrap();
/// assert!(!link.link.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct DialogInviteLinkManager {
    /// Internal state protected by RwLock for thread-safe access.
    state: Arc<std::sync::RwLock<DialogInviteLinkManagerState>>,
}

impl Default for DialogInviteLinkManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DialogInviteLinkManager {
    /// Creates a new dialog invite link manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::DialogInviteLinkManager;
    ///
    /// let manager = DialogInviteLinkManager::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: Arc::new(std::sync::RwLock::new(
                DialogInviteLinkManagerState::default(),
            )),
        }
    }

    /// Creates a permanent invite link for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Returns
    ///
    /// `Ok(link)` if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the lock cannot be acquired.
    ///
    /// # TDLib Alignment
    ///
    /// Corresponds to TDLib's `export_dialog_invite_link` with `is_permanent = true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::DialogInviteLinkManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogInviteLinkManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// let link = manager.create_permanent_link(dialog_id).unwrap();
    /// assert!(link.is_permanent);
    /// ```
    pub fn create_permanent_link(&mut self, dialog_id: DialogId) -> Result<InviteLink> {
        let mut state = self.state.write().map_err(|_| InviteLinkError::LockError)?;

        // Generate a fake invite link URL
        let link_url = format!("https://t.me/+{:x}", dialog_id.to_encoded());
        let creator_id = UserId::new(1).unwrap_or(UserId(0));

        let link = InviteLink::permanent(link_url, creator_id);

        state.permanent_links.insert(dialog_id, link.clone());

        // Also add to all_links
        state
            .all_links
            .entry(dialog_id)
            .or_insert_with(Vec::new)
            .push(link.clone());

        Ok(link)
    }

    /// Gets the permanent invite link for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Returns
    ///
    /// `Some(link)` if found, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::DialogInviteLinkManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogInviteLinkManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// assert!(manager.get_permanent_link(dialog_id).is_none());
    ///
    /// manager.create_permanent_link(dialog_id).unwrap();
    /// assert!(manager.get_permanent_link(dialog_id).is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn get_permanent_link(&self, dialog_id: DialogId) -> Option<InviteLink> {
        self.state
            .read()
            .ok()?
            .permanent_links
            .get(&dialog_id)
            .cloned()
    }

    /// Creates a temporary invite link for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `title` - Optional title for the link
    /// * `expire_date` - Expiration Unix timestamp, or 0 for default
    /// * `usage_limit` - Maximum uses, or 0 for default
    /// * `creates_join_request` - Whether users need approval to join
    ///
    /// # Returns
    ///
    /// `Ok(link)` if successful
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The title is too long
    /// - The expire date or usage limit is invalid
    /// - The lock cannot be acquired
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::DialogInviteLinkManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogInviteLinkManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// let link = manager.create_temporary_link(
    ///     dialog_id,
    ///     Some("Temporary link".to_string()),
    ///     0,
    ///     10,
    ///     false,
    /// ).unwrap();
    ///
    /// assert!(!link.is_permanent);
    /// ```
    pub fn create_temporary_link(
        &mut self,
        dialog_id: DialogId,
        title: Option<String>,
        expire_date: i32,
        usage_limit: i32,
        creates_join_request: bool,
    ) -> Result<InviteLink> {
        if let Some(t) = &title {
            if t.len() > MAX_INVITE_LINK_TITLE_LENGTH {
                return Err(InviteLinkError::TitleTooLong {
                    max: MAX_INVITE_LINK_TITLE_LENGTH,
                    len: t.len(),
                });
            }
        }

        if expire_date < 0 {
            return Err(InviteLinkError::InvalidExpireDate(expire_date));
        }

        if usage_limit < 0 {
            return Err(InviteLinkError::InvalidUsageLimit(usage_limit));
        }

        let mut state = self.state.write().map_err(|_| InviteLinkError::LockError)?;

        // Generate a fake invite link URL
        let link_url = format!(
            "https://t.me/+{:x}{:x}",
            dialog_id.to_encoded(),
            state.all_links.len() as i64
        );
        let creator_id = UserId::new(1).unwrap_or(UserId(0));

        let mut link = InviteLink::new(link_url, creator_id);
        link.title = title.unwrap_or_default();
        link.expire_date = if expire_date > 0 {
            expire_date
        } else {
            InviteLink::current_time() + DEFAULT_EXPIRE_TIME
        };
        link.usage_limit = if usage_limit > 0 {
            usage_limit
        } else {
            DEFAULT_USAGE_LIMIT
        };
        link.creates_join_request = creates_join_request;

        state
            .all_links
            .entry(dialog_id)
            .or_insert_with(Vec::new)
            .push(link.clone());

        Ok(link)
    }

    /// Revokes an invite link.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `link_url` - URL of the link to revoke
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful
    ///
    /// # Errors
    ///
    /// Returns an error if the link is not found or lock cannot be acquired.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::DialogInviteLinkManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogInviteLinkManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// let link = manager.create_temporary_link(
    ///     dialog_id,
    ///     None,
    ///     0,
    ///     10,
    ///     false,
    /// ).unwrap();
    ///
    /// manager.revoke_link(dialog_id, &link.link).unwrap();
    /// ```
    pub fn revoke_link(&mut self, dialog_id: DialogId, link_url: &str) -> Result<()> {
        let mut state = self.state.write().map_err(|_| InviteLinkError::LockError)?;

        let links = state
            .all_links
            .get_mut(&dialog_id)
            .ok_or(InviteLinkError::DialogNotFound(dialog_id))?;

        let link = links
            .iter_mut()
            .find(|l| l.link == link_url)
            .ok_or_else(|| InviteLinkError::LinkNotFound(link_url.to_string()))?;

        link.is_revoked = true;
        Ok(())
    }

    /// Gets all invite links for a dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    ///
    /// # Returns
    ///
    /// `Ok(links)` if successful
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::DialogInviteLinkManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogInviteLinkManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.create_permanent_link(dialog_id).unwrap();
    ///
    /// let links = manager.get_all_links(dialog_id).unwrap();
    /// assert!(!links.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn get_all_links(&self, dialog_id: DialogId) -> Option<Vec<InviteLink>> {
        self.state.read().ok()?.all_links.get(&dialog_id).cloned()
    }

    /// Gets a specific invite link.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    /// * `link_url` - URL of the link
    ///
    /// # Returns
    ///
    /// `Some(link)` if found
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::DialogInviteLinkManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogInviteLinkManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// let created = manager.create_permanent_link(dialog_id).unwrap();
    /// let found = manager.get_link(dialog_id, &created.link);
    ///
    /// assert!(found.is_some());
    /// ```
    #[must_use]
    pub fn get_link(&self, dialog_id: DialogId, link_url: &str) -> Option<InviteLink> {
        self.state
            .read()
            .ok()?
            .all_links
            .get(&dialog_id)?
            .iter()
            .find(|l| l.link == link_url)
            .cloned()
    }

    /// Checks an invite link and caches its information.
    ///
    /// # Arguments
    ///
    /// * `link_url` - URL of the invite link
    /// * `info` - Information about the link
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::{DialogInviteLinkManager, InviteLinkInfo};
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogInviteLinkManager::new();
    ///
    /// let mut info = InviteLinkInfo::default();
    /// info.title = "Test Chat".to_string();
    ///
    /// manager.check_invite_link("https://t.me/+abc".to_string(), info);
    /// ```
    pub fn check_invite_link(&mut self, link_url: String, info: InviteLinkInfo) -> Result<()> {
        let mut state = self.state.write().map_err(|_| InviteLinkError::LockError)?;
        state.invite_link_infos.insert(link_url, info);
        Ok(())
    }

    /// Gets cached invite link information.
    ///
    /// # Arguments
    ///
    /// * `link_url` - URL of the invite link
    ///
    /// # Returns
    ///
    /// `Some(info)` if cached
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::{DialogInviteLinkManager, InviteLinkInfo};
    ///
    /// let mut manager = DialogInviteLinkManager::new();
    ///
    /// let mut info = InviteLinkInfo::default();
    /// info.title = "Test".to_string();
    ///
    /// manager.check_invite_link("https://t.me/+abc".to_string(), info.clone());
    ///
    /// let cached = manager.get_invite_link_info("https://t.me/+abc");
    /// assert!(cached.is_some());
    /// ```
    #[inline]
    #[must_use]
    pub fn get_invite_link_info(&self, link_url: &str) -> Option<InviteLinkInfo> {
        self.state
            .read()
            .ok()?
            .invite_link_infos
            .get(link_url)
            .cloned()
    }

    /// Invalidates cached invite link information.
    ///
    /// # Arguments
    ///
    /// * `link_url` - URL of the invite link
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::{DialogInviteLinkManager, InviteLinkInfo};
    ///
    /// let mut manager = DialogInviteLinkManager::new();
    ///
    /// let info = InviteLinkInfo::default();
    /// manager.check_invite_link("https://t.me/+abc".to_string(), info);
    ///
    /// manager.invalidate_invite_link("https://t.me/+abc");
    ///
    /// assert!(manager.get_invite_link_info("https://t.me/+abc").is_none());
    /// ```
    pub fn invalidate_invite_link(&mut self, link_url: &str) -> Result<()> {
        let mut state = self.state.write().map_err(|_| InviteLinkError::LockError)?;
        state.invite_link_infos.remove(link_url);
        Ok(())
    }

    /// Gets the number of invite links for a dialog.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::DialogInviteLinkManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogInviteLinkManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// assert_eq!(manager.link_count(dialog_id), 0);
    ///
    /// manager.create_permanent_link(dialog_id).unwrap();
    /// assert_eq!(manager.link_count(dialog_id), 1);
    /// ```
    #[inline]
    #[must_use]
    pub fn link_count(&self, dialog_id: DialogId) -> usize {
        self.state
            .read()
            .ok()
            .and_then(|s| s.all_links.get(&dialog_id).map(|v| v.len()))
            .unwrap_or(0)
    }

    /// Clears all invite links for a dialog.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_invite_link_manager::DialogInviteLinkManager;
    /// use rustgram_types::{ChatId, DialogId};
    ///
    /// let mut manager = DialogInviteLinkManager::new();
    ///
    /// let chat_id = ChatId::new(123).unwrap();
    /// let dialog_id = DialogId::from_chat(chat_id);
    ///
    /// manager.create_permanent_link(dialog_id).unwrap();
    /// assert_eq!(manager.link_count(dialog_id), 1);
    ///
    /// manager.clear_dialog_links(dialog_id).unwrap();
    /// assert_eq!(manager.link_count(dialog_id), 0);
    /// ```
    pub fn clear_dialog_links(&mut self, dialog_id: DialogId) -> Result<()> {
        let mut state = self.state.write().map_err(|_| InviteLinkError::LockError)?;
        state.permanent_links.remove(&dialog_id);
        state.all_links.remove(&dialog_id);
        Ok(())
    }
}

impl fmt::Display for DialogInviteLinkManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dialog_count = self.state.read().map(|s| s.all_links.len()).unwrap_or(0);
        write!(
            f,
            "DialogInviteLinkManager(dialogs_with_links: {})",
            dialog_count
        )
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-dialog-invite-link-manager";

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::ChatId;

    // ========== InviteLink Tests ==========

    #[test]
    fn test_invite_link_new() {
        let link = InviteLink::new("https://t.me/+test".to_string(), UserId::new(123).unwrap());

        assert!(!link.is_permanent);
        assert!(!link.is_revoked);
        assert!(link.is_active());
    }

    #[test]
    fn test_invite_link_permanent() {
        let link =
            InviteLink::permanent("https://t.me/+test".to_string(), UserId::new(123).unwrap());

        assert!(link.is_permanent);
        assert!(link.is_active());
    }

    #[test]
    fn test_invite_link_expired() {
        let mut link = InviteLink::new("https://t.me/+test".to_string(), UserId::new(123).unwrap());
        link.expire_date = 1000000000; // Past timestamp

        assert!(link.is_expired());
        assert!(!link.is_active());
    }

    #[test]
    fn test_invite_link_usage_limit_reached() {
        let mut link = InviteLink::new("https://t.me/+test".to_string(), UserId::new(123).unwrap());
        link.usage_limit = 10;
        link.usage_count = 10;

        assert!(link.is_usage_limit_reached());
        assert!(!link.is_active());
    }

    #[test]
    fn test_invite_link_revoked() {
        let mut link = InviteLink::new("https://t.me/+test".to_string(), UserId::new(123).unwrap());
        link.is_revoked = true;

        assert!(!link.is_active());
    }

    // ========== InviteLinkInfo Tests ==========

    #[test]
    fn test_invite_link_info_default() {
        let info = InviteLinkInfo::default();

        assert!(info.title.is_empty());
        assert!(!info.is_channel);
        assert_eq!(info.participant_count, 0);
    }

    // ========== Manager Constructor Tests ==========

    #[test]
    fn test_manager_new() {
        let manager = DialogInviteLinkManager::new();
        assert_eq!(
            manager.link_count(DialogId::from_chat(ChatId::new(1).unwrap())),
            0
        );
    }

    #[test]
    fn test_manager_default() {
        let manager = DialogInviteLinkManager::default();
        assert_eq!(
            manager.link_count(DialogId::from_chat(ChatId::new(1).unwrap())),
            0
        );
    }

    // ========== Create Permanent Link Tests ==========

    #[test]
    fn test_create_permanent_link() {
        let mut manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let link = manager.create_permanent_link(dialog_id).unwrap();

        assert!(link.is_permanent);
        assert!(link.is_active());

        let retrieved = manager.get_permanent_link(dialog_id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().link, link.link);
    }

    #[test]
    fn test_create_permanent_link_twice() {
        let mut manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let _link1 = manager.create_permanent_link(dialog_id).unwrap();
        let _link2 = manager.create_permanent_link(dialog_id).unwrap();

        // Second creation should replace the first
        let retrieved = manager.get_permanent_link(dialog_id).unwrap();
        assert_eq!(retrieved.is_permanent, true);
    }

    // ========== Create Temporary Link Tests ==========

    #[test]
    fn test_create_temporary_link() {
        let mut manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let link = manager
            .create_temporary_link(dialog_id, Some("Test".to_string()), 0, 10, false)
            .unwrap();

        assert!(!link.is_permanent);
        assert_eq!(link.title, "Test");
        assert_eq!(link.usage_limit, 10);
        assert!(!link.creates_join_request);
    }

    #[test]
    fn test_create_temporary_link_title_too_long() {
        let mut manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let long_title = "a".repeat(MAX_INVITE_LINK_TITLE_LENGTH + 1);
        let result = manager.create_temporary_link(dialog_id, Some(long_title), 0, 10, false);

        assert!(result.is_err());
    }

    #[test]
    fn test_create_temporary_link_invalid_expire_date() {
        let mut manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let result = manager.create_temporary_link(dialog_id, None, -1, 10, false);

        assert!(result.is_err());
    }

    #[test]
    fn test_create_temporary_link_invalid_usage_limit() {
        let mut manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let result = manager.create_temporary_link(dialog_id, None, 0, -1, false);

        assert!(result.is_err());
    }

    #[test]
    fn test_create_temporary_link_with_join_request() {
        let mut manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let link = manager
            .create_temporary_link(dialog_id, None, 0, 10, true)
            .unwrap();

        assert!(link.creates_join_request);
    }

    // ========== Get Link Tests ==========

    #[test]
    fn test_get_link() {
        let mut manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let created = manager.create_permanent_link(dialog_id).unwrap();
        let found = manager.get_link(dialog_id, &created.link);

        assert!(found.is_some());
        assert_eq!(found.unwrap().link, created.link);
    }

    #[test]
    fn test_get_link_not_found() {
        let manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let found = manager.get_link(dialog_id, "https://t.me/+nonexistent");

        assert!(found.is_none());
    }

    // ========== Get All Links Tests ==========

    #[test]
    fn test_get_all_links() {
        let mut manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager.create_permanent_link(dialog_id).unwrap();
        manager
            .create_temporary_link(dialog_id, None, 0, 10, false)
            .unwrap();

        let links = manager.get_all_links(dialog_id).unwrap();

        assert_eq!(links.len(), 2);
    }

    #[test]
    fn test_get_all_links_empty() {
        let manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let links = manager.get_all_links(dialog_id);

        assert!(links.is_none());
    }

    // ========== Revoke Link Tests ==========

    #[test]
    fn test_revoke_link() {
        let mut manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let link = manager
            .create_temporary_link(dialog_id, None, 0, 10, false)
            .unwrap();

        manager.revoke_link(dialog_id, &link.link).unwrap();

        let retrieved = manager.get_link(dialog_id, &link.link).unwrap();
        assert!(retrieved.is_revoked);
        assert!(!retrieved.is_active());
    }

    #[test]
    fn test_revoke_link_not_found() {
        let mut manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        let result = manager.revoke_link(dialog_id, "https://t.me/+nonexistent");

        assert!(result.is_err());
    }

    // ========== Check Invite Link Tests ==========

    #[test]
    fn test_check_invite_link() {
        let mut manager = DialogInviteLinkManager::new();

        let mut info = InviteLinkInfo::default();
        info.title = "Test Chat".to_string();

        manager
            .check_invite_link("https://t.me/+abc".to_string(), info)
            .unwrap();

        let cached = manager.get_invite_link_info("https://t.me/+abc");

        assert!(cached.is_some());
        assert_eq!(cached.unwrap().title, "Test Chat");
    }

    #[test]
    fn test_invalidate_invite_link() {
        let mut manager = DialogInviteLinkManager::new();

        let info = InviteLinkInfo::default();
        manager
            .check_invite_link("https://t.me/+abc".to_string(), info)
            .unwrap();

        manager.invalidate_invite_link("https://t.me/+abc").unwrap();

        let cached = manager.get_invite_link_info("https://t.me/+abc");
        assert!(cached.is_none());
    }

    // ========== Link Count Tests ==========

    #[test]
    fn test_link_count() {
        let mut manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        assert_eq!(manager.link_count(dialog_id), 0);

        manager.create_permanent_link(dialog_id).unwrap();
        assert_eq!(manager.link_count(dialog_id), 1);

        manager
            .create_temporary_link(dialog_id, None, 0, 10, false)
            .unwrap();
        assert_eq!(manager.link_count(dialog_id), 2);
    }

    // ========== Clear Dialog Links Tests ==========

    #[test]
    fn test_clear_dialog_links() {
        let mut manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        manager.create_permanent_link(dialog_id).unwrap();
        manager
            .create_temporary_link(dialog_id, None, 0, 10, false)
            .unwrap();

        assert_eq!(manager.link_count(dialog_id), 2);

        manager.clear_dialog_links(dialog_id).unwrap();

        assert_eq!(manager.link_count(dialog_id), 0);
        assert!(manager.get_permanent_link(dialog_id).is_none());
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_format() {
        let manager = DialogInviteLinkManager::new();
        let display = format!("{}", manager);

        assert!(display.contains("DialogInviteLinkManager"));
        assert!(display.contains("dialogs_with_links: 0"));
    }

    // ========== Constants Tests ==========

    #[test]
    fn test_constants() {
        assert_eq!(MAX_INVITE_LINK_TITLE_LENGTH, 32);
        assert_eq!(DEFAULT_EXPIRE_TIME, 86400);
        assert_eq!(DEFAULT_USAGE_LIMIT, 100);
    }

    // ========== Integration Tests ==========

    #[test]
    fn test_full_workflow() {
        let mut manager = DialogInviteLinkManager::new();

        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        // Create permanent link
        let permanent = manager.create_permanent_link(dialog_id).unwrap();
        assert!(permanent.is_permanent);

        // Create temporary link
        let temporary = manager
            .create_temporary_link(dialog_id, Some("Temp".to_string()), 0, 5, true)
            .unwrap();
        assert!(!temporary.is_permanent);
        assert_eq!(temporary.title, "Temp");

        // Verify both links exist
        assert_eq!(manager.link_count(dialog_id), 2);

        // Revoke temporary link
        manager.revoke_link(dialog_id, &temporary.link).unwrap();

        let revoked = manager.get_link(dialog_id, &temporary.link).unwrap();
        assert!(revoked.is_revoked);

        // Get all links
        let all_links = manager.get_all_links(dialog_id).unwrap();
        assert_eq!(all_links.len(), 2);
    }
}
