//! # Rustgram StoryForwardInfo
//!
//! Story forward information tracking for Telegram MTProto client.
//!
//! This crate provides types for tracking forward/repost information for stories.
//! When a story is forwarded or reposted, this structure tracks the original
//! story source and modification state.
//!
//! ## Overview
//!
//! - [`StoryForwardInfo`] - Forward information for a story
//!
//! ## Forward Info
//!
//! Story forward information contains:
//!
//! - **Dialog ID** - The dialog (user/chat) where the original story was posted
//! - **Story ID** - The identifier of the original story
//! - **Sender Name** - Name of the sender (may be hidden for privacy)
//! - **Is Modified** - Whether the story was modified when forwarded
//!
//! ## Examples
//!
//! Basic forward info:
//!
//! ```
//! use rustgram_story_forward_info::StoryForwardInfo;
//! use rustgram_types::DialogId;
//! use rustgram_active_story_state::StoryId;
//!
//! let info = StoryForwardInfo::new();
//! assert!(info.is_empty());
//!
//! let dialog_id = DialogId::from_user(rustgram_types::UserId(123));
//! let story_id = StoryId::new(456);
//! let info_with_data = StoryForwardInfo::with_data(dialog_id, story_id, "Alice".to_string(), false);
//! assert!(!info_with_data.is_empty());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use rustgram_active_story_state::StoryId;
use rustgram_types::DialogId;
use std::fmt;

/// Story forward information.
///
/// Tracks information about a story that has been forwarded or reposted.
/// This includes the original story source and whether it was modified.
///
/// # Examples
///
/// ```
/// use rustgram_story_forward_info::StoryForwardInfo;
/// use rustgram_types::DialogId;
///
/// let info = StoryForwardInfo::new();
/// assert!(info.is_empty());
/// assert_eq!(info.dialog_id(), DialogId::default());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StoryForwardInfo {
    /// The dialog where the original story was posted
    dialog_id: DialogId,
    /// The identifier of the original story
    story_id: StoryId,
    /// Name of the sender (may be empty for privacy)
    sender_name: String,
    /// Whether the story was modified when forwarded
    is_modified: bool,
}

impl Default for StoryForwardInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl StoryForwardInfo {
    /// Creates a new empty forward info.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    ///
    /// let info = StoryForwardInfo::new();
    /// assert!(info.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            dialog_id: DialogId::default(),
            story_id: StoryId::new(0),
            sender_name: String::new(),
            is_modified: false,
        }
    }

    /// Creates forward info with the given data.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog where the original story was posted
    /// * `story_id` - The identifier of the original story
    /// * `sender_name` - Name of the sender
    /// * `is_modified` - Whether the story was modified
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    /// use rustgram_types::DialogId;
    /// use rustgram_active_story_state::StoryId;
    ///
    /// let dialog_id = DialogId::from_user(rustgram_types::UserId(123));
    /// let story_id = StoryId::new(456);
    /// let info = StoryForwardInfo::with_data(dialog_id, story_id, "Alice".to_string(), true);
    /// assert!(info.is_modified());
    /// ```
    #[inline]
    #[must_use]
    pub fn with_data(
        dialog_id: DialogId,
        story_id: StoryId,
        sender_name: String,
        is_modified: bool,
    ) -> Self {
        Self {
            dialog_id,
            story_id,
            sender_name,
            is_modified,
        }
    }

    /// Creates forward info from a full story ID.
    ///
    /// This is a convenience constructor when you have the complete story identifier.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog where the original story was posted
    /// * `story_id` - The identifier of the original story
    /// * `is_modified` - Whether the story was modified when forwarded
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    /// use rustgram_types::DialogId;
    /// use rustgram_active_story_state::StoryId;
    ///
    /// let dialog_id = DialogId::from_user(rustgram_types::UserId(123));
    /// let story_id = StoryId::new(789);
    /// let info = StoryForwardInfo::from_story(dialog_id, story_id, false);
    /// assert!(info.has_story());
    /// ```
    #[inline]
    #[must_use]
    pub fn from_story(dialog_id: DialogId, story_id: StoryId, is_modified: bool) -> Self {
        Self {
            dialog_id,
            story_id,
            sender_name: String::new(),
            is_modified,
        }
    }

    /// Checks if the forward info is empty.
    ///
    /// # Returns
    ///
    /// `true` if there's no valid story to forward
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    ///
    /// assert!(StoryForwardInfo::new().is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        !self.story_id.is_valid()
    }

    /// Checks if there's a valid story to forward.
    ///
    /// # Returns
    ///
    /// `true` if the story ID is valid (non-zero)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    /// use rustgram_types::DialogId;
    /// use rustgram_active_story_state::StoryId;
    ///
    /// let info = StoryForwardInfo::from_story(DialogId::default(), StoryId::new(123), false);
    /// assert!(info.has_story());
    /// ```
    #[inline]
    #[must_use]
    pub fn has_story(&self) -> bool {
        self.story_id.is_valid()
    }

    /// Returns the dialog ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    /// use rustgram_types::DialogId;
    /// use rustgram_active_story_state::StoryId;
    ///
    /// let dialog_id = DialogId::from_user(rustgram_types::UserId(123));
    /// let info = StoryForwardInfo::from_story(dialog_id, StoryId::new(1), false);
    /// assert_eq!(info.dialog_id(), dialog_id);
    /// ```
    #[inline]
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the story ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    /// use rustgram_types::DialogId;
    /// use rustgram_active_story_state::StoryId;
    ///
    /// let info = StoryForwardInfo::from_story(DialogId::default(), StoryId::new(456), false);
    /// assert_eq!(info.story_id().get(), 456);
    /// ```
    #[inline]
    #[must_use]
    pub const fn story_id(&self) -> StoryId {
        self.story_id
    }

    /// Returns the sender name.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    /// use rustgram_types::DialogId;
    /// use rustgram_active_story_state::StoryId;
    ///
    /// let info = StoryForwardInfo::with_data(
    ///     DialogId::default(),
    ///     StoryId::new(1),
    ///     "Alice".to_string(),
    ///     false,
    /// );
    /// assert_eq!(info.sender_name(), "Alice");
    /// ```
    #[inline]
    #[must_use]
    pub fn sender_name(&self) -> &str {
        &self.sender_name
    }

    /// Checks if the story was modified when forwarded.
    ///
    /// # Returns
    ///
    /// `true` if the story was modified
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    /// use rustgram_types::DialogId;
    /// use rustgram_active_story_state::StoryId;
    ///
    /// let info = StoryForwardInfo::from_story(DialogId::default(), StoryId::new(1), true);
    /// assert!(info.is_modified());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_modified(&self) -> bool {
        self.is_modified
    }

    /// Checks if the sender name is visible.
    ///
    /// # Returns
    ///
    /// `true` if there's a non-empty sender name
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    ///
    /// let info = StoryForwardInfo::with_data(
    ///     rustgram_types::DialogId::default(),
    ///     rustgram_active_story_state::StoryId::new(1),
    ///     "Alice".to_string(),
    ///     false,
    /// );
    /// assert!(info.has_sender_name());
    ///
    /// let info_no_name = StoryForwardInfo::from_story(
    ///     rustgram_types::DialogId::default(),
    ///     rustgram_active_story_state::StoryId::new(1),
    ///     false,
    /// );
    /// assert!(!info_no_name.has_sender_name());
    /// ```
    #[inline]
    #[must_use]
    pub fn has_sender_name(&self) -> bool {
        !self.sender_name.is_empty()
    }

    /// Hides the sender name if needed for privacy.
    ///
    /// This clears the sender name, typically used when privacy settings
    /// require hiding the original sender.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    /// use rustgram_types::DialogId;
    /// use rustgram_active_story_state::StoryId;
    ///
    /// let mut info = StoryForwardInfo::with_data(
    ///     DialogId::default(),
    ///     StoryId::new(1),
    ///     "Alice".to_string(),
    ///     false,
    /// );
    /// assert!(info.has_sender_name());
    ///
    /// info.hide_sender();
    /// assert!(!info.has_sender_name());
    /// assert_eq!(info.sender_name(), "");
    /// ```
    pub fn hide_sender(&mut self) {
        self.sender_name.clear();
    }

    /// Sets the sender name.
    ///
    /// # Arguments
    ///
    /// * `name` - The new sender name
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    ///
    /// let mut info = StoryForwardInfo::new();
    /// info.set_sender_name("Bob".to_string());
    /// assert_eq!(info.sender_name(), "Bob");
    /// ```
    pub fn set_sender_name(&mut self, name: String) {
        self.sender_name = name;
    }

    /// Sets the modified state.
    ///
    /// # Arguments
    ///
    /// * `is_modified` - Whether the story was modified
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    ///
    /// let mut info = StoryForwardInfo::new();
    /// assert!(!info.is_modified());
    ///
    /// info.set_modified(true);
    /// assert!(info.is_modified());
    /// ```
    pub fn set_modified(&mut self, is_modified: bool) {
        self.is_modified = is_modified;
    }

    /// Converts to TD API representation.
    ///
    /// Returns data suitable for `td_api::storyRepostInfo`.
    ///
    /// # Returns
    ///
    /// A tuple of (dialog_id, story_id, sender_name, is_modified)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    /// use rustgram_types::DialogId;
    /// use rustgram_active_story_state::StoryId;
    ///
    /// let info = StoryForwardInfo::with_data(
    ///     DialogId::from_user(rustgram_types::UserId(123)),
    ///     StoryId::new(456),
    ///     "Alice".to_string(),
    ///     true,
    /// );
    /// let (dialog_id, story_id, name, modified) = info.to_td_api();
    /// assert_eq!(story_id.get(), 456);
    /// assert_eq!(name, "Alice");
    /// assert!(modified);
    /// ```
    #[must_use]
    pub fn to_td_api(&self) -> (DialogId, StoryId, String, bool) {
        (
            self.dialog_id,
            self.story_id,
            self.sender_name.clone(),
            self.is_modified,
        )
    }

    /// Creates from TD API representation.
    ///
    /// Creates from `telegram_api::storyFwdHeader`.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog where the original story was posted
    /// * `story_id` - The identifier of the original story
    /// * `sender_name` - Name of the sender
    /// * `is_modified` - Whether the story was modified
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_story_forward_info::StoryForwardInfo;
    /// use rustgram_types::DialogId;
    /// use rustgram_active_story_state::StoryId;
    ///
    /// let info = StoryForwardInfo::from_td_api(
    ///     DialogId::from_user(rustgram_types::UserId(123)),
    ///     StoryId::new(456),
    ///     "Alice".to_string(),
    ///     false,
    /// );
    /// assert_eq!(info.sender_name(), "Alice");
    /// assert!(!info.is_modified());
    /// ```
    #[must_use]
    pub fn from_td_api(
        dialog_id: DialogId,
        story_id: StoryId,
        sender_name: String,
        is_modified: bool,
    ) -> Self {
        Self {
            dialog_id,
            story_id,
            sender_name,
            is_modified,
        }
    }
}

impl fmt::Display for StoryForwardInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StoryForwardInfo {{ dialog_id: {:?}, story_id: {}, sender: {}, modified: {} }}",
            self.dialog_id,
            self.story_id.get(),
            if self.sender_name.is_empty() {
                "(hidden)"
            } else {
                &self.sender_name
            },
            self.is_modified
        )
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-story-forward-info";

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::UserId;

    // ========== Constructor Tests ==========

    #[test]
    fn test_new_creates_empty() {
        let info = StoryForwardInfo::new();
        assert!(info.is_empty());
        assert!(!info.has_story());
        assert_eq!(info.dialog_id(), DialogId::default());
        assert_eq!(info.story_id().get(), 0);
    }

    #[test]
    fn test_default_creates_empty() {
        let info = StoryForwardInfo::default();
        assert!(info.is_empty());
    }

    #[test]
    fn test_with_data_sets_values() {
        let dialog_id = DialogId::from_user(UserId(123));
        let story_id = StoryId::new(456);
        let info = StoryForwardInfo::with_data(dialog_id, story_id, "Alice".to_string(), true);

        assert_eq!(info.dialog_id(), dialog_id);
        assert_eq!(info.story_id(), story_id);
        assert_eq!(info.sender_name(), "Alice");
        assert!(info.is_modified());
    }

    #[test]
    fn test_from_story_creates_valid() {
        let dialog_id = DialogId::from_user(UserId(123));
        let story_id = StoryId::new(789);
        let info = StoryForwardInfo::from_story(dialog_id, story_id, false);

        assert_eq!(info.dialog_id(), dialog_id);
        assert_eq!(info.story_id(), story_id);
        assert!(!info.is_modified());
        assert!(!info.has_sender_name());
    }

    // ========== is_empty Tests ==========

    #[test]
    fn test_is_empty_when_story_zero() {
        let info = StoryForwardInfo::new();
        assert!(info.is_empty());
    }

    #[test]
    fn test_is_empty_when_story_valid() {
        let info = StoryForwardInfo::from_story(DialogId::default(), StoryId::new(1), false);
        assert!(!info.is_empty());
    }

    #[test]
    fn test_is_empty_with_negative_story() {
        let info = StoryForwardInfo::from_story(DialogId::default(), StoryId::new(-1), false);
        assert!(!info.is_empty()); // Negative is still valid
    }

    // ========== has_story Tests ==========

    #[test]
    fn test_has_story_when_zero() {
        let info = StoryForwardInfo::new();
        assert!(!info.has_story());
    }

    #[test]
    fn test_has_story_when_positive() {
        let info = StoryForwardInfo::from_story(DialogId::default(), StoryId::new(100), false);
        assert!(info.has_story());
    }

    #[test]
    fn test_has_story_when_negative() {
        let info = StoryForwardInfo::from_story(DialogId::default(), StoryId::new(-1), false);
        assert!(info.has_story());
    }

    // ========== Accessor Tests ==========

    #[test]
    fn test_dialog_id_returns_value() {
        let dialog_id = DialogId::from_user(UserId(999));
        let info = StoryForwardInfo::from_story(dialog_id, StoryId::new(1), false);
        assert_eq!(info.dialog_id(), dialog_id);
    }

    #[test]
    fn test_story_id_returns_value() {
        let story_id = StoryId::new(12345);
        let info = StoryForwardInfo::from_story(DialogId::default(), story_id, false);
        assert_eq!(info.story_id(), story_id);
    }

    #[test]
    fn test_sender_name_returns_value() {
        let info = StoryForwardInfo::with_data(
            DialogId::default(),
            StoryId::new(1),
            "Bob".to_string(),
            false,
        );
        assert_eq!(info.sender_name(), "Bob");
    }

    #[test]
    fn test_sender_name_empty_when_not_set() {
        let info = StoryForwardInfo::from_story(DialogId::default(), StoryId::new(1), false);
        assert_eq!(info.sender_name(), "");
    }

    #[test]
    fn test_is_modified_returns_value() {
        let info = StoryForwardInfo::from_story(DialogId::default(), StoryId::new(1), true);
        assert!(info.is_modified());
    }

    #[test]
    fn test_is_modified_false_by_default() {
        let info = StoryForwardInfo::new();
        assert!(!info.is_modified());
    }

    // ========== has_sender_name Tests ==========

    #[test]
    fn test_has_sender_name_when_set() {
        let info = StoryForwardInfo::with_data(
            DialogId::default(),
            StoryId::new(1),
            "Alice".to_string(),
            false,
        );
        assert!(info.has_sender_name());
    }

    #[test]
    fn test_has_sender_name_when_empty() {
        let info = StoryForwardInfo::from_story(DialogId::default(), StoryId::new(1), false);
        assert!(!info.has_sender_name());
    }

    #[test]
    fn test_has_sender_name_when_whitespace_only() {
        let info = StoryForwardInfo::with_data(
            DialogId::default(),
            StoryId::new(1),
            "   ".to_string(),
            false,
        );
        assert!(info.has_sender_name()); // Whitespace is still content
    }

    // ========== Mutator Tests ==========

    #[test]
    fn test_hide_sender_clears_name() {
        let mut info = StoryForwardInfo::with_data(
            DialogId::default(),
            StoryId::new(1),
            "Alice".to_string(),
            false,
        );
        assert!(info.has_sender_name());

        info.hide_sender();
        assert!(!info.has_sender_name());
        assert_eq!(info.sender_name(), "");
    }

    #[test]
    fn test_set_sender_name() {
        let mut info = StoryForwardInfo::new();
        info.set_sender_name("Charlie".to_string());
        assert_eq!(info.sender_name(), "Charlie");
    }

    #[test]
    fn test_set_sender_name_overwrites() {
        let mut info = StoryForwardInfo::with_data(
            DialogId::default(),
            StoryId::new(1),
            "Alice".to_string(),
            false,
        );
        info.set_sender_name("Bob".to_string());
        assert_eq!(info.sender_name(), "Bob");
    }

    #[test]
    fn test_set_modified() {
        let mut info = StoryForwardInfo::new();
        assert!(!info.is_modified());

        info.set_modified(true);
        assert!(info.is_modified());

        info.set_modified(false);
        assert!(!info.is_modified());
    }

    // ========== TD API Conversion Tests ==========

    #[test]
    fn test_to_td_api_returns_values() {
        let dialog_id = DialogId::from_user(UserId(123));
        let story_id = StoryId::new(456);
        let info = StoryForwardInfo::with_data(dialog_id, story_id, "Alice".to_string(), true);

        let (ret_dialog, ret_story, ret_name, ret_modified) = info.to_td_api();
        assert_eq!(ret_dialog, dialog_id);
        assert_eq!(ret_story, story_id);
        assert_eq!(ret_name, "Alice");
        assert!(ret_modified);
    }

    #[test]
    fn test_from_td_api_creates_instance() {
        let dialog_id = DialogId::from_user(UserId(789));
        let story_id = StoryId::new(999);
        let info = StoryForwardInfo::from_td_api(dialog_id, story_id, "Sender".to_string(), false);

        assert_eq!(info.dialog_id(), dialog_id);
        assert_eq!(info.story_id(), story_id);
        assert_eq!(info.sender_name(), "Sender");
        assert!(!info.is_modified());
    }

    #[test]
    fn test_td_api_round_trip() {
        let original = StoryForwardInfo::with_data(
            DialogId::from_user(UserId(123)),
            StoryId::new(456),
            "Original".to_string(),
            true,
        );

        let (dialog_id, story_id, sender_name, is_modified) = original.to_td_api();
        let restored = StoryForwardInfo::from_td_api(dialog_id, story_id, sender_name, is_modified);

        assert_eq!(original, restored);
    }

    // ========== Equality Tests ==========

    #[test]
    fn test_equality_same_values() {
        let dialog_id = DialogId::from_user(UserId(1));
        let story_id = StoryId::new(2);
        let info1 = StoryForwardInfo::with_data(dialog_id, story_id, "Name".to_string(), false);
        let info2 = StoryForwardInfo::with_data(dialog_id, story_id, "Name".to_string(), false);
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_equality_different_dialog() {
        let info1 =
            StoryForwardInfo::from_story(DialogId::from_user(UserId(1)), StoryId::new(2), false);
        let info2 =
            StoryForwardInfo::from_story(DialogId::from_user(UserId(3)), StoryId::new(2), false);
        assert_ne!(info1, info2);
    }

    #[test]
    fn test_equality_different_story() {
        let info1 =
            StoryForwardInfo::from_story(DialogId::from_user(UserId(1)), StoryId::new(2), false);
        let info2 =
            StoryForwardInfo::from_story(DialogId::from_user(UserId(1)), StoryId::new(3), false);
        assert_ne!(info1, info2);
    }

    #[test]
    fn test_equality_different_name() {
        let info1 = StoryForwardInfo::with_data(
            DialogId::from_user(UserId(1)),
            StoryId::new(2),
            "Alice".to_string(),
            false,
        );
        let info2 = StoryForwardInfo::with_data(
            DialogId::from_user(UserId(1)),
            StoryId::new(2),
            "Bob".to_string(),
            false,
        );
        assert_ne!(info1, info2);
    }

    #[test]
    fn test_equality_different_modified() {
        let info1 =
            StoryForwardInfo::from_story(DialogId::from_user(UserId(1)), StoryId::new(2), true);
        let info2 =
            StoryForwardInfo::from_story(DialogId::from_user(UserId(1)), StoryId::new(2), false);
        assert_ne!(info1, info2);
    }

    // ========== Clone Tests ==========

    #[test]
    fn test_clone() {
        let info = StoryForwardInfo::with_data(
            DialogId::from_user(UserId(123)),
            StoryId::new(456),
            "Alice".to_string(),
            true,
        );
        let cloned = info.clone();
        assert_eq!(info, cloned);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_format_with_name() {
        let info = StoryForwardInfo::with_data(
            DialogId::from_user(UserId(123)),
            StoryId::new(456),
            "Alice".to_string(),
            false,
        );
        let display = format!("{}", info);
        assert!(display.contains("Alice"));
        assert!(display.contains("456"));
    }

    #[test]
    fn test_display_format_without_name() {
        let info = StoryForwardInfo::from_story(
            DialogId::from_user(UserId(123)),
            StoryId::new(456),
            false,
        );
        let display = format!("{}", info);
        assert!(display.contains("(hidden)"));
        assert!(display.contains("456"));
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-story-forward-info");
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_with_empty_sender_name() {
        let info =
            StoryForwardInfo::with_data(DialogId::default(), StoryId::new(1), String::new(), false);
        assert!(!info.has_sender_name());
        assert_eq!(info.sender_name(), "");
    }

    #[test]
    fn test_with_large_story_id() {
        let story_id = StoryId::new(i32::MAX);
        let info = StoryForwardInfo::from_story(DialogId::default(), story_id, false);
        assert_eq!(info.story_id().get(), i32::MAX);
    }

    #[test]
    fn test_hide_sender_when_already_empty() {
        let mut info = StoryForwardInfo::new();
        assert!(!info.has_sender_name());
        info.hide_sender();
        assert!(!info.has_sender_name());
    }

    #[test]
    fn test_set_modified_multiple_times() {
        let mut info = StoryForwardInfo::new();
        info.set_modified(true);
        assert!(info.is_modified());
        info.set_modified(false);
        assert!(!info.is_modified());
        info.set_modified(true);
        assert!(info.is_modified());
    }

    #[test]
    fn test_dialog_id_none() {
        let info = StoryForwardInfo::new();
        assert_eq!(info.dialog_id(), DialogId::default());
    }
}
