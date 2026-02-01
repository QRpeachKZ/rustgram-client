//! # DialogFilterDialogInfo
//!
//! Information about a dialog within a dialog filter.
//!
//! This module implements TDLib's DialogFilterDialogInfo structure which
//! holds information about whether a specific dialog should be included in
//! a filter and its current state.
//!
//! ## Overview
//!
//! - [`DialogFilterDialogInfo`] - Dialog information for filtering
//!
//! ## Usage
//!
//! ```
//! use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
//! use rustgram_dialog_id::DialogId;
//! use rustgram_folder_id::FolderId;
//!
//! let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_dialog_id::DialogId;
use rustgram_folder_id::FolderId;
use serde::{Deserialize, Serialize};

/// Information about a dialog within a dialog filter.
///
/// This structure holds the dialog ID, folder ID, and various boolean flags
/// that determine if and how a dialog appears in a filter.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
/// use rustgram_dialog_id::DialogId;
/// use rustgram_folder_id::FolderId;
///
/// let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
/// assert_eq!(info.dialog_id(), DialogId::from(42));
/// assert!(!info.has_unread_mentions());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DialogFilterDialogInfo {
    /// The dialog ID
    dialog_id: DialogId,

    /// The folder ID this dialog belongs to
    folder_id: FolderId,

    /// Whether this dialog has unread mentions
    has_unread_mentions: bool,

    /// Whether this dialog is muted
    is_muted: bool,

    /// Whether this dialog has unread messages
    has_unread_messages: bool,
}

impl DialogFilterDialogInfo {
    /// Creates a new dialog filter info with default flags.
    ///
    /// All boolean flags default to `false`.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    /// * `folder_id` - The folder ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
    /// assert!(!info.has_unread_mentions());
    /// assert!(!info.is_muted());
    /// assert!(!info.has_unread_messages());
    /// ```
    #[must_use]
    pub const fn new(dialog_id: DialogId, folder_id: FolderId) -> Self {
        Self {
            dialog_id,
            folder_id,
            has_unread_mentions: false,
            is_muted: false,
            has_unread_messages: false,
        }
    }

    /// Creates a new dialog filter info with all flags specified.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    /// * `folder_id` - The folder ID
    /// * `has_unread_mentions` - Whether the dialog has unread mentions
    /// * `is_muted` - Whether the dialog is muted
    /// * `has_unread_messages` - Whether the dialog has unread messages
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let info = DialogFilterDialogInfo::with_flags(
    ///     DialogId::from(42),
    ///     FolderId::new(1),
    ///     true,  // has_unread_mentions
    ///     false, // is_muted
    ///     true,  // has_unread_messages
    /// );
    /// assert!(info.has_unread_mentions());
    /// assert!(info.has_unread_messages());
    /// ```
    #[must_use]
    pub const fn with_flags(
        dialog_id: DialogId,
        folder_id: FolderId,
        has_unread_mentions: bool,
        is_muted: bool,
        has_unread_messages: bool,
    ) -> Self {
        Self {
            dialog_id,
            folder_id,
            has_unread_mentions,
            is_muted,
            has_unread_messages,
        }
    }

    /// Returns the dialog ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
    /// assert_eq!(info.dialog_id(), DialogId::from(42));
    /// ```
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the folder ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
    /// assert_eq!(info.folder_id(), FolderId::new(1));
    /// ```
    #[must_use]
    pub const fn folder_id(&self) -> FolderId {
        self.folder_id
    }

    /// Returns whether this dialog has unread mentions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1))
    ///     .with_unread_mentions(true);
    /// assert!(info.has_unread_mentions());
    /// ```
    #[must_use]
    pub const fn has_unread_mentions(&self) -> bool {
        self.has_unread_mentions
    }

    /// Returns whether this dialog is muted.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1))
    ///     .with_muted(true);
    /// assert!(info.is_muted());
    /// ```
    #[must_use]
    pub const fn is_muted(&self) -> bool {
        self.is_muted
    }

    /// Returns whether this dialog has unread messages.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1))
    ///     .with_unread_messages(true);
    /// assert!(info.has_unread_messages());
    /// ```
    #[must_use]
    pub const fn has_unread_messages(&self) -> bool {
        self.has_unread_messages
    }

    /// Sets the dialog ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let mut info = DialogFilterDialogInfo::new(DialogId::from(1), FolderId::new(1));
    /// info.set_dialog_id(DialogId::from(99));
    /// assert_eq!(info.dialog_id(), DialogId::from(99));
    /// ```
    pub fn set_dialog_id(&mut self, dialog_id: DialogId) {
        self.dialog_id = dialog_id;
    }

    /// Sets the folder ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let mut info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
    /// info.set_folder_id(FolderId::new(2));
    /// assert_eq!(info.folder_id(), FolderId::new(2));
    /// ```
    pub fn set_folder_id(&mut self, folder_id: FolderId) {
        self.folder_id = folder_id;
    }

    /// Sets the unread mentions flag.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let mut info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
    /// info.set_unread_mentions(true);
    /// assert!(info.has_unread_mentions());
    /// ```
    pub fn set_unread_mentions(&mut self, value: bool) {
        self.has_unread_mentions = value;
    }

    /// Sets the muted flag.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let mut info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
    /// info.set_muted(true);
    /// assert!(info.is_muted());
    /// ```
    pub fn set_muted(&mut self, value: bool) {
        self.is_muted = value;
    }

    /// Sets the unread messages flag.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let mut info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
    /// info.set_unread_messages(true);
    /// assert!(info.has_unread_messages());
    /// ```
    pub fn set_unread_messages(&mut self, value: bool) {
        self.has_unread_messages = value;
    }

    /// Builder-style method to set unread mentions flag.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1))
    ///     .with_unread_mentions(true);
    /// assert!(info.has_unread_mentions());
    /// ```
    #[must_use]
    pub const fn with_unread_mentions(mut self, value: bool) -> Self {
        self.has_unread_mentions = value;
        self
    }

    /// Builder-style method to set muted flag.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1))
    ///     .with_muted(true);
    /// assert!(info.is_muted());
    /// ```
    #[must_use]
    pub const fn with_muted(mut self, value: bool) -> Self {
        self.is_muted = value;
        self
    }

    /// Builder-style method to set unread messages flag.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_filter_dialog_info::DialogFilterDialogInfo;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_folder_id::FolderId;
    ///
    /// let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1))
    ///     .with_unread_messages(true);
    /// assert!(info.has_unread_messages());
    /// ```
    #[must_use]
    pub const fn with_unread_messages(mut self, value: bool) -> Self {
        self.has_unread_messages = value;
        self
    }
}

impl Default for DialogFilterDialogInfo {
    fn default() -> Self {
        Self {
            dialog_id: DialogId::from(0),
            folder_id: FolderId::new(0),
            has_unread_mentions: false,
            is_muted: false,
            has_unread_messages: false,
        }
    }
}

// ========== Tests ==========

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Constructor Tests ==========

    #[test]
    fn test_new_creates_info_with_default_flags() {
        let dialog_id = DialogId::from(42);
        let folder_id = FolderId::new(1);

        let info = DialogFilterDialogInfo::new(dialog_id, folder_id);

        assert_eq!(info.dialog_id(), dialog_id);
        assert_eq!(info.folder_id(), folder_id);
        assert!(!info.has_unread_mentions());
        assert!(!info.is_muted());
        assert!(!info.has_unread_messages());
    }

    #[test]
    fn test_with_flags_creates_info_with_specified_flags() {
        let dialog_id = DialogId::from(42);
        let folder_id = FolderId::new(1);

        let info = DialogFilterDialogInfo::with_flags(
            dialog_id,
            folder_id,
            true,  // has_unread_mentions
            true,  // is_muted
            false, // has_unread_messages
        );

        assert_eq!(info.dialog_id(), dialog_id);
        assert_eq!(info.folder_id(), folder_id);
        assert!(info.has_unread_mentions());
        assert!(info.is_muted());
        assert!(!info.has_unread_messages());
    }

    #[test]
    fn test_default_creates_info() {
        let info = DialogFilterDialogInfo::default();

        assert_eq!(info.dialog_id(), DialogId::from(0));
        assert_eq!(info.folder_id(), FolderId::new(0));
        assert!(!info.has_unread_mentions());
        assert!(!info.is_muted());
        assert!(!info.has_unread_messages());
    }

    // ========== Getter Tests ==========

    #[test]
    fn test_dialog_id_returns_correct_value() {
        let info = DialogFilterDialogInfo::new(DialogId::from(123), FolderId::new(1));
        assert_eq!(info.dialog_id(), DialogId::from(123));
    }

    #[test]
    fn test_folder_id_returns_correct_value() {
        let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(5));
        assert_eq!(info.folder_id(), FolderId::new(5));
    }

    #[test]
    fn test_has_unread_mentions_returns_correct_value() {
        let info = DialogFilterDialogInfo::with_flags(
            DialogId::from(42),
            FolderId::new(1),
            true,
            false,
            false,
        );
        assert!(info.has_unread_mentions());
    }

    #[test]
    fn test_is_muted_returns_correct_value() {
        let info = DialogFilterDialogInfo::with_flags(
            DialogId::from(42),
            FolderId::new(1),
            false,
            true,
            false,
        );
        assert!(info.is_muted());
    }

    #[test]
    fn test_has_unread_messages_returns_correct_value() {
        let info = DialogFilterDialogInfo::with_flags(
            DialogId::from(42),
            FolderId::new(1),
            false,
            false,
            true,
        );
        assert!(info.has_unread_messages());
    }

    // ========== Setter Tests ==========

    #[test]
    fn test_set_dialog_id_updates_value() {
        let mut info = DialogFilterDialogInfo::new(DialogId::from(1), FolderId::new(1));
        info.set_dialog_id(DialogId::from(99));
        assert_eq!(info.dialog_id(), DialogId::from(99));
    }

    #[test]
    fn test_set_folder_id_updates_value() {
        let mut info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
        info.set_folder_id(FolderId::new(2));
        assert_eq!(info.folder_id(), FolderId::new(2));
    }

    #[test]
    fn test_set_unread_mentions_updates_value() {
        let mut info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
        assert!(!info.has_unread_mentions());

        info.set_unread_mentions(true);
        assert!(info.has_unread_mentions());

        info.set_unread_mentions(false);
        assert!(!info.has_unread_mentions());
    }

    #[test]
    fn test_set_muted_updates_value() {
        let mut info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
        assert!(!info.is_muted());

        info.set_muted(true);
        assert!(info.is_muted());

        info.set_muted(false);
        assert!(!info.is_muted());
    }

    #[test]
    fn test_set_unread_messages_updates_value() {
        let mut info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
        assert!(!info.has_unread_messages());

        info.set_unread_messages(true);
        assert!(info.has_unread_messages());

        info.set_unread_messages(false);
        assert!(!info.has_unread_messages());
    }

    // ========== Builder Tests ==========

    #[test]
    fn test_with_unread_mentions_builder() {
        let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1))
            .with_unread_mentions(true);

        assert!(info.has_unread_mentions());
        assert!(!info.is_muted());
        assert!(!info.has_unread_messages());
    }

    #[test]
    fn test_with_muted_builder() {
        let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1))
            .with_muted(true);

        assert!(!info.has_unread_mentions());
        assert!(info.is_muted());
        assert!(!info.has_unread_messages());
    }

    #[test]
    fn test_with_unread_messages_builder() {
        let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1))
            .with_unread_messages(true);

        assert!(!info.has_unread_mentions());
        assert!(!info.is_muted());
        assert!(info.has_unread_messages());
    }

    #[test]
    fn test_chained_builders() {
        let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1))
            .with_unread_mentions(true)
            .with_muted(true)
            .with_unread_messages(true);

        assert!(info.has_unread_mentions());
        assert!(info.is_muted());
        assert!(info.has_unread_messages());
    }

    // ========== PartialEq Tests ==========

    #[test]
    fn test_eq_same_values() {
        let info1 = DialogFilterDialogInfo::with_flags(
            DialogId::from(42),
            FolderId::new(1),
            true,
            false,
            true,
        );
        let info2 = DialogFilterDialogInfo::with_flags(
            DialogId::from(42),
            FolderId::new(1),
            true,
            false,
            true,
        );
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_eq_different_dialog_id() {
        let info1 = DialogFilterDialogInfo::new(DialogId::from(1), FolderId::new(1));
        let info2 = DialogFilterDialogInfo::new(DialogId::from(2), FolderId::new(1));
        assert_ne!(info1, info2);
    }

    #[test]
    fn test_eq_different_folder_id() {
        let info1 = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
        let info2 = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(2));
        assert_ne!(info1, info2);
    }

    #[test]
    fn test_eq_different_flags() {
        let info1 = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1))
            .with_unread_mentions(true);
        let info2 = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1));
        assert_ne!(info1, info2);
    }

    // ========== Clone Tests ==========

    #[test]
    fn test_clone_creates_copy() {
        let info1 = DialogFilterDialogInfo::with_flags(
            DialogId::from(42),
            FolderId::new(1),
            true,
            true,
            true,
        );
        let info2 = info1.clone();

        assert_eq!(info1, info2);
        assert_eq!(info1.dialog_id(), info2.dialog_id());
        assert_eq!(info1.folder_id(), info2.folder_id());
        assert_eq!(info1.has_unread_mentions(), info2.has_unread_mentions());
    }

    // ========== Hash Tests ==========

    #[test]
    fn test_hash_same_for_same_values() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let info1 = DialogFilterDialogInfo::with_flags(
            DialogId::from(42),
            FolderId::new(1),
            true,
            false,
            true,
        );
        let info2 = DialogFilterDialogInfo::with_flags(
            DialogId::from(42),
            FolderId::new(1),
            true,
            false,
            true,
        );

        let mut hasher1 = DefaultHasher::new();
        info1.hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        info2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_hash_different_for_different_values() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let info1 = DialogFilterDialogInfo::new(DialogId::from(1), FolderId::new(1));
        let info2 = DialogFilterDialogInfo::new(DialogId::from(2), FolderId::new(1));

        let mut hasher1 = DefaultHasher::new();
        info1.hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        info2.hash(&mut hasher2);

        assert_ne!(hasher1.finish(), hasher2.finish());
    }

    // ========== Serde Tests ==========

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let info = DialogFilterDialogInfo::with_flags(
            DialogId::from(42),
            FolderId::new(1),
            true,
            false,
            true,
        );

        let json = serde_json::to_string(&info).unwrap();
        let deserialized: DialogFilterDialogInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(info, deserialized);
    }

    #[test]
    fn test_serialize_contains_expected_fields() {
        let info = DialogFilterDialogInfo::new(DialogId::from(42), FolderId::new(1))
            .with_unread_mentions(true);

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("has_unread_mentions"));
        assert!(json.contains("true"));
    }
}
