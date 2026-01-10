// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Dialog filter invite link types for Telegram MTProto client.
//!
//! This module implements TDLib's DialogFilterInviteLink from
//! `td/telegram/DialogFilterInviteLink.h`.
//!
//! # Overview
//!
//! Dialog filter invite links allow users to share invite links for chat folders
//! (also known as dialog filters or chat folders). These links can be used to
//! quickly add a set of chats to another user's Telegram client.

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_types::DialogId;
use std::fmt;

/// Dialog filter invite link for sharing chat folders.
///
/// Represents an invite link for a chat folder that can be shared with other users.
/// When a user clicks the link, they can add the folder with all its chats to
/// their Telegram client.
///
/// # Example
///
/// ```
/// use rustgram_dialog_filter_invite_link::DialogFilterInviteLink;
/// use rustgram_types::{DialogId, ChannelId};
///
/// let channel_id = ChannelId::new(100000000000).unwrap();
/// let dialog_ids = vec![
///     DialogId::from_channel(channel_id),
/// ];
///
/// let link = DialogFilterInviteLink::new(
///     "https://t.me/addlist/abc123".to_string(),
///     "My Folder".to_string(),
///     dialog_ids
/// );
///
/// assert!(link.is_valid());
/// assert_eq!(link.title(), "My Folder");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogFilterInviteLink {
    /// The invite link URL.
    invite_link: String,
    /// The title of the chat folder.
    title: String,
    /// The list of dialog IDs included in this folder.
    dialog_ids: Vec<DialogId>,
}

impl DialogFilterInviteLink {
    /// Creates a new DialogFilterInviteLink.
    ///
    /// # Arguments
    ///
    /// * `invite_link` - The invite link URL
    /// * `title` - The title of the chat folder
    /// * `dialog_ids` - List of dialog IDs included in the folder
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dialog_filter_invite_link::DialogFilterInviteLink;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// let link = DialogFilterInviteLink::new(
    ///     "https://t.me/addlist/xyz789".to_string(),
    ///     "Important Chats".to_string(),
    ///     vec![DialogId::from_user(UserId::new(123).unwrap())]
    /// );
    /// ```
    pub fn new(invite_link: String, title: String, dialog_ids: Vec<DialogId>) -> Self {
        Self {
            invite_link,
            title,
            dialog_ids,
        }
    }

    /// Returns the invite link URL.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dialog_filter_invite_link::DialogFilterInviteLink;
    ///
    /// let link = DialogFilterInviteLink::new(
    ///     "https://t.me/addlist/test".to_string(),
    ///     "Test".to_string(),
    ///     vec![]
    /// );
    /// assert_eq!(link.invite_link(), "https://t.me/addlist/test");
    /// ```
    pub fn invite_link(&self) -> &str {
        &self.invite_link
    }

    /// Returns the title of the chat folder.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dialog_filter_invite_link::DialogFilterInviteLink;
    ///
    /// let link = DialogFilterInviteLink::new(
    ///     "url".to_string(),
    ///     "Work Folder".to_string(),
    ///     vec![]
    /// );
    /// assert_eq!(link.title(), "Work Folder");
    /// ```
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the list of dialog IDs in this folder.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dialog_filter_invite_link::DialogFilterInviteLink;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// let dialog_id = DialogId::from_user(UserId::new(1).unwrap());
    /// let link = DialogFilterInviteLink::new(
    ///     "url".to_string(),
    ///     "Folder".to_string(),
    ///     vec![dialog_id]
    /// );
    ///
    /// assert_eq!(link.dialog_ids().len(), 1);
    /// assert_eq!(link.dialog_ids()[0], dialog_id);
    /// ```
    pub fn dialog_ids(&self) -> &[DialogId] {
        &self.dialog_ids
    }

    /// Checks if this invite link is valid.
    ///
    /// A valid invite link must have a non-empty invite link URL.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dialog_filter_invite_link::DialogFilterInviteLink;
    ///
    /// let valid = DialogFilterInviteLink::new(
    ///     "https://t.me/addlist/test".to_string(),
    ///     "Title".to_string(),
    ///     vec![]
    /// );
    /// assert!(valid.is_valid());
    ///
    /// let invalid = DialogFilterInviteLink::new(
    ///     "".to_string(),
    ///     "Title".to_string(),
    ///     vec![]
    /// );
    /// assert!(!invalid.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.invite_link.is_empty()
    }

    /// Checks if the given string is a valid invite link.
    ///
    /// Valid invite links must start with "https://t.me/addlist/" or "tg://addlist/".
    ///
    /// # Arguments
    ///
    /// * `invite_link` - The invite link to validate
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dialog_filter_invite_link::DialogFilterInviteLink;
    ///
    /// assert!(DialogFilterInviteLink::is_valid_invite_link("https://t.me/addlist/abc123"));
    /// assert!(DialogFilterInviteLink::is_valid_invite_link("tg://addlist/xyz789"));
    /// assert!(!DialogFilterInviteLink::is_valid_invite_link("https://example.com"));
    /// assert!(!DialogFilterInviteLink::is_valid_invite_link(""));
    /// ```
    pub fn is_valid_invite_link(invite_link: &str) -> bool {
        const HTTPS_PREFIX: &str = "https://t.me/addlist/";
        const TG_PREFIX: &str = "tg://addlist/";

        let link = invite_link.trim();

        if link.is_empty() {
            return false;
        }

        link.starts_with(HTTPS_PREFIX) || link.starts_with(TG_PREFIX)
    }

    /// Checks if this invite link is HTTPS (more secure than tg://).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dialog_filter_invite_link::DialogFilterInviteLink;
    ///
    /// let https_link = DialogFilterInviteLink::new(
    ///     "https://t.me/addlist/test".to_string(),
    ///     "Title".to_string(),
    ///     vec![]
    /// );
    /// assert!(https_link.is_https());
    ///
    /// let tg_link = DialogFilterInviteLink::new(
    ///     "tg://addlist/test".to_string(),
    ///     "Title".to_string(),
    ///     vec![]
    /// );
    /// assert!(!tg_link.is_https());
    /// ```
    pub fn is_https(&self) -> bool {
        self.invite_link.starts_with("https://")
    }

    /// Checks if this invite link uses the tg:// protocol scheme.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dialog_filter_invite_link::DialogFilterInviteLink;
    ///
    /// let tg_link = DialogFilterInviteLink::new(
    ///     "tg://addlist/test".to_string(),
    ///     "Title".to_string(),
    ///     vec![]
    /// );
    /// assert!(tg_link.is_tg_scheme());
    /// ```
    pub fn is_tg_scheme(&self) -> bool {
        self.invite_link.starts_with("tg://")
    }

    /// Returns the number of dialogs in this folder.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dialog_filter_invite_link::DialogFilterInviteLink;
    /// use rustgram_types::{DialogId, UserId};
    ///
    /// let link = DialogFilterInviteLink::new(
    ///     "url".to_string(),
    ///     "Folder".to_string(),
    ///     vec![
    ///         DialogId::from_user(UserId::new(1).unwrap()),
    ///         DialogId::from_user(UserId::new(2).unwrap()),
    ///     ]
    /// );
    ///
    /// assert_eq!(link.dialog_count(), 2);
    /// ```
    pub fn dialog_count(&self) -> usize {
        self.dialog_ids.len()
    }

    /// Checks if the folder is empty (contains no dialogs).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dialog_filter_invite_link::DialogFilterInviteLink;
    ///
    /// let empty = DialogFilterInviteLink::new(
    ///     "url".to_string(),
    ///     "Folder".to_string(),
    ///     vec![]
    /// );
    /// assert!(empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.dialog_ids.is_empty()
    }

    /// Sets a new invite link URL.
    ///
    /// # Arguments
    ///
    /// * `invite_link` - The new invite link URL
    pub fn with_invite_link(mut self, invite_link: String) -> Self {
        self.invite_link = invite_link;
        self
    }

    /// Sets a new title for the folder.
    ///
    /// # Arguments
    ///
    /// * `title` - The new title
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Sets new dialog IDs for the folder.
    ///
    /// # Arguments
    ///
    /// * `dialog_ids` - The new list of dialog IDs
    pub fn with_dialog_ids(mut self, dialog_ids: Vec<DialogId>) -> Self {
        self.dialog_ids = dialog_ids;
        self
    }

    /// Adds a dialog ID to the folder.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID to add
    pub fn add_dialog(&mut self, dialog_id: DialogId) {
        self.dialog_ids.push(dialog_id);
    }

    /// Removes a dialog ID from the folder at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the dialog to remove
    ///
    /// # Returns
    ///
    /// * `Some(dialog_id)` - If the index was valid
    /// * `None` - If the index was out of bounds
    pub fn remove_dialog(&mut self, index: usize) -> Option<DialogId> {
        if index < self.dialog_ids.len() {
            Some(self.dialog_ids.remove(index))
        } else {
            None
        }
    }

    /// Clears all dialogs from the folder.
    pub fn clear_dialogs(&mut self) {
        self.dialog_ids.clear();
    }
}

impl fmt::Display for DialogFilterInviteLink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DialogFilterInviteLink(title={}, dialogs={})",
            self.title,
            self.dialog_ids.len()
        )
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-dialog-filter-invite-link";

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::{ChatId, UserId};

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-dialog-filter-invite-link");
    }

    #[test]
    fn test_invite_link_new() {
        let link = DialogFilterInviteLink::new(
            "https://t.me/addlist/test123".to_string(),
            "Test Folder".to_string(),
            vec![],
        );

        assert_eq!(link.invite_link(), "https://t.me/addlist/test123");
        assert_eq!(link.title(), "Test Folder");
        assert!(link.is_empty());
    }

    #[test]
    fn test_invite_link_with_dialogs() {
        let user_id = UserId::new(1).unwrap();
        let dialogs = vec![DialogId::from_user(user_id)];

        let link = DialogFilterInviteLink::new(
            "https://t.me/addlist/abc".to_string(),
            "Folder".to_string(),
            dialogs.clone(),
        );

        assert_eq!(link.dialog_ids(), &dialogs);
        assert_eq!(link.dialog_count(), 1);
        assert!(!link.is_empty());
    }

    #[test]
    fn test_invite_link_is_valid() {
        let valid = DialogFilterInviteLink::new(
            "https://t.me/addlist/test".to_string(),
            "Title".to_string(),
            vec![],
        );
        assert!(valid.is_valid());

        let invalid = DialogFilterInviteLink::new("".to_string(), "Title".to_string(), vec![]);
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_is_valid_invite_link_static() {
        // Valid HTTPS links
        assert!(DialogFilterInviteLink::is_valid_invite_link(
            "https://t.me/addlist/abc123"
        ));
        assert!(DialogFilterInviteLink::is_valid_invite_link(
            "https://t.me/addlist/XYZ-123_456"
        ));

        // Valid tg:// links
        assert!(DialogFilterInviteLink::is_valid_invite_link(
            "tg://addlist/test"
        ));

        // Invalid links
        assert!(!DialogFilterInviteLink::is_valid_invite_link(
            "https://example.com"
        ));
        assert!(!DialogFilterInviteLink::is_valid_invite_link(
            "t.me/addlist/test"
        ));
        assert!(!DialogFilterInviteLink::is_valid_invite_link(""));
        assert!(!DialogFilterInviteLink::is_valid_invite_link(
            "https://telegram.org"
        ));
    }

    #[test]
    fn test_is_valid_invite_link_whitespace() {
        // Should trim whitespace
        assert!(DialogFilterInviteLink::is_valid_invite_link(
            "  https://t.me/addlist/test  "
        ));

        // Only whitespace is invalid
        assert!(!DialogFilterInviteLink::is_valid_invite_link("   "));
    }

    #[test]
    fn test_is_https() {
        let https_link = DialogFilterInviteLink::new(
            "https://t.me/addlist/test".to_string(),
            "Title".to_string(),
            vec![],
        );
        assert!(https_link.is_https());

        let tg_link = DialogFilterInviteLink::new(
            "tg://addlist/test".to_string(),
            "Title".to_string(),
            vec![],
        );
        assert!(!tg_link.is_https());
    }

    #[test]
    fn test_is_tg_scheme() {
        let tg_link = DialogFilterInviteLink::new(
            "tg://addlist/test".to_string(),
            "Title".to_string(),
            vec![],
        );
        assert!(tg_link.is_tg_scheme());

        let https_link = DialogFilterInviteLink::new(
            "https://t.me/addlist/test".to_string(),
            "Title".to_string(),
            vec![],
        );
        assert!(!https_link.is_tg_scheme());
    }

    #[test]
    fn test_dialog_count() {
        let dialogs = vec![
            DialogId::from_user(UserId::new(1).unwrap()),
            DialogId::from_user(UserId::new(2).unwrap()),
            DialogId::from_user(UserId::new(3).unwrap()),
        ];

        let link = DialogFilterInviteLink::new("url".to_string(), "Folder".to_string(), dialogs);

        assert_eq!(link.dialog_count(), 3);
    }

    #[test]
    fn test_with_invite_link() {
        let link = DialogFilterInviteLink::new(
            "https://t.me/addlist/old".to_string(),
            "Title".to_string(),
            vec![],
        )
        .with_invite_link("https://t.me/addlist/new".to_string());

        assert_eq!(link.invite_link(), "https://t.me/addlist/new");
    }

    #[test]
    fn test_with_title() {
        let link = DialogFilterInviteLink::new("url".to_string(), "Old Title".to_string(), vec![])
            .with_title("New Title".to_string());

        assert_eq!(link.title(), "New Title");
    }

    #[test]
    fn test_with_dialog_ids() {
        let link = DialogFilterInviteLink::new(
            "url".to_string(),
            "Folder".to_string(),
            vec![DialogId::from_user(UserId::new(1).unwrap())],
        )
        .with_dialog_ids(vec![
            DialogId::from_user(UserId::new(2).unwrap()),
            DialogId::from_user(UserId::new(3).unwrap()),
        ]);

        assert_eq!(link.dialog_count(), 2);
    }

    #[test]
    fn test_add_dialog() {
        let mut link = DialogFilterInviteLink::new("url".to_string(), "Folder".to_string(), vec![]);

        link.add_dialog(DialogId::from_user(UserId::new(1).unwrap()));
        assert_eq!(link.dialog_count(), 1);

        link.add_dialog(DialogId::from_user(UserId::new(2).unwrap()));
        assert_eq!(link.dialog_count(), 2);
    }

    #[test]
    fn test_remove_dialog() {
        let mut link = DialogFilterInviteLink::new(
            "url".to_string(),
            "Folder".to_string(),
            vec![
                DialogId::from_user(UserId::new(1).unwrap()),
                DialogId::from_user(UserId::new(2).unwrap()),
                DialogId::from_user(UserId::new(3).unwrap()),
            ],
        );

        let removed = link.remove_dialog(1);
        assert!(removed.is_some());
        assert_eq!(
            removed.unwrap(),
            DialogId::from_user(UserId::new(2).unwrap())
        );
        assert_eq!(link.dialog_count(), 2);

        // Out of bounds
        assert!(link.remove_dialog(10).is_none());
    }

    #[test]
    fn test_clear_dialogs() {
        let mut link = DialogFilterInviteLink::new(
            "url".to_string(),
            "Folder".to_string(),
            vec![DialogId::from_user(UserId::new(1).unwrap())],
        );

        assert!(!link.is_empty());
        link.clear_dialogs();
        assert!(link.is_empty());
    }

    #[test]
    fn test_display() {
        let link = DialogFilterInviteLink::new(
            "https://t.me/addlist/test".to_string(),
            "Test Folder".to_string(),
            vec![],
        );

        let display = format!("{link}");
        assert!(display.contains("Test Folder"));
        assert!(display.contains("0"));
    }

    #[test]
    fn test_clone() {
        let link1 = DialogFilterInviteLink::new(
            "https://t.me/addlist/test".to_string(),
            "Test".to_string(),
            vec![],
        );
        let link2 = link1.clone();

        assert_eq!(link1, link2);
    }

    #[test]
    fn test_eq() {
        let dialogs = vec![DialogId::from_user(UserId::new(1).unwrap())];

        let link1 = DialogFilterInviteLink::new(
            "https://t.me/addlist/test".to_string(),
            "Test".to_string(),
            dialogs.clone(),
        );
        let link2 = DialogFilterInviteLink::new(
            "https://t.me/addlist/test".to_string(),
            "Test".to_string(),
            dialogs,
        );

        assert_eq!(link1, link2);
    }

    #[test]
    fn test_multiple_dialog_types() {
        let dialogs = vec![
            DialogId::from_user(UserId::new(1).unwrap()),
            DialogId::from_chat(ChatId::new(100).unwrap()),
        ];

        let link = DialogFilterInviteLink::new("url".to_string(), "Folder".to_string(), dialogs);

        assert_eq!(link.dialog_count(), 2);
    }

    #[test]
    fn test_builder_chain() {
        let link = DialogFilterInviteLink::new("url".to_string(), "Initial".to_string(), vec![])
            .with_title("Updated".to_string())
            .with_invite_link("https://t.me/addlist/new".to_string());

        assert_eq!(link.title(), "Updated");
        assert_eq!(link.invite_link(), "https://t.me/addlist/new");
    }

    #[test]
    fn test_empty_title() {
        let link = DialogFilterInviteLink::new(
            "https://t.me/addlist/test".to_string(),
            "".to_string(),
            vec![],
        );

        // Empty title is still valid
        assert!(link.is_valid());
        assert_eq!(link.title(), "");
    }

    #[test]
    fn test_mixed_dialog_types() {
        use rustgram_types::{ChannelId, SecretChatId};

        let dialogs = vec![
            DialogId::from_user(UserId::new(1).unwrap()),
            DialogId::from_chat(ChatId::new(100).unwrap()),
            DialogId::from_channel(ChannelId::new(100000000000).unwrap()),
            DialogId::from_secret_chat(SecretChatId::new(1).unwrap()),
        ];

        let link = DialogFilterInviteLink::new("url".to_string(), "All Types".to_string(), dialogs);

        assert_eq!(link.dialog_count(), 4);
    }
}
