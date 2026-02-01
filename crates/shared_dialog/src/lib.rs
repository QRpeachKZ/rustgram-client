// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Shared Dialog
//!
//! Shared dialog information for Telegram.
//!
//! Based on TDLib's `SharedDialog` from `td/telegram/SharedDialog.h`.
//!
//! # Overview
//!
//! A `SharedDialog` represents a dialog that has been shared with the user,
//! such as when someone shares a chat or user profile.
//!
//! # Example
//!
//! ```rust
//! use rustgram_shared_dialog::SharedDialog;
//! use rustgram_dialog_id::DialogId;
//!
//! let dialog = SharedDialog::new(DialogId::new(1234567890));
//! assert!(dialog.is_valid());
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_dialog_id::{DialogId, DialogType};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Shared dialog information.
///
/// Represents a dialog that has been shared with the user.
/// Contains the dialog ID and optional metadata like name and photo.
///
/// # TDLib Mapping
///
/// - `SharedDialog::new(dialog_id)` → TDLib: `SharedDialog(DialogId)`
/// - `is_valid()` → TDLib: Checks if `dialog_id_.is_valid()`
///
/// # Example
///
/// ```rust
/// use rustgram_shared_dialog::SharedDialog;
/// use rustgram_dialog_id::DialogId;
///
/// let dialog = SharedDialog::new(DialogId::new(1234567890));
/// assert!(dialog.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedDialog {
    /// Dialog ID
    dialog_id: DialogId,
    /// First name
    first_name: Option<String>,
    /// Last name
    last_name: Option<String>,
    /// Username
    username: Option<String>,
}

impl SharedDialog {
    /// Creates a new shared dialog from a dialog ID.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_shared_dialog::SharedDialog;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog = SharedDialog::new(DialogId::new(1234567890));
    /// assert_eq!(dialog.dialog_id(), DialogId::new(1234567890));
    /// ```
    #[must_use]
    pub fn new(dialog_id: DialogId) -> Self {
        Self {
            dialog_id,
            first_name: None,
            last_name: None,
            username: None,
        }
    }

    /// Creates a new shared dialog with full details.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    /// * `first_name` - First name
    /// * `last_name` - Last name
    /// * `username` - Username
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_shared_dialog::SharedDialog;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog = SharedDialog::with_details(
    ///     DialogId::new(1234567890),
    ///     Some("John"),
    ///     Some("Doe"),
    ///     Some("johndoe")
    /// );
    /// assert_eq!(dialog.first_name(), Some("John"));
    /// ```
    #[must_use]
    pub fn with_details(
        dialog_id: DialogId,
        first_name: Option<impl Into<String>>,
        last_name: Option<impl Into<String>>,
        username: Option<impl Into<String>>,
    ) -> Self {
        Self {
            dialog_id,
            first_name: first_name.map(|s| s.into()),
            last_name: last_name.map(|s| s.into()),
            username: username.map(|s| s.into()),
        }
    }

    /// Returns the dialog ID.
    #[must_use]
    pub fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the first name.
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        self.first_name.as_deref()
    }

    /// Returns the last name.
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        self.last_name.as_deref()
    }

    /// Returns the username.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    /// Checks if this is a valid shared dialog.
    ///
    /// A valid shared dialog has a valid dialog ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_shared_dialog::SharedDialog;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// assert!(SharedDialog::new(DialogId::new(1234567890)).is_valid());
    /// assert!(!SharedDialog::new(DialogId::new(0)).is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.dialog_id.is_valid()
    }

    /// Checks if this is a user dialog.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_shared_dialog::SharedDialog;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog = SharedDialog::new(DialogId::new(1234567890));
    /// assert!(dialog.is_user());
    /// ```
    #[must_use]
    pub fn is_user(&self) -> bool {
        self.dialog_id.get_type() == DialogType::User
    }

    /// Checks if this is a chat or channel dialog.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_shared_dialog::SharedDialog;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog = SharedDialog::new(DialogId::new(-1234567890));
    /// assert!(dialog.is_dialog());
    /// ```
    #[must_use]
    pub fn is_dialog(&self) -> bool {
        matches!(
            self.dialog_id.get_type(),
            DialogType::Chat | DialogType::Channel
        )
    }
}

impl Default for SharedDialog {
    fn default() -> Self {
        Self {
            dialog_id: DialogId::new(0),
            first_name: None,
            last_name: None,
            username: None,
        }
    }
}

impl fmt::Display for SharedDialog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SharedDialog({})", self.dialog_id)?;
        if let Some(name) = self.first_name() {
            write!(f, " {}", name)?;
        }
        if let Some(username) = self.username() {
            write!(f, " (@{})", username)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dialog_id = DialogId::new(1234567890);
        let dialog = SharedDialog::new(dialog_id);
        assert_eq!(dialog.dialog_id(), dialog_id);
        assert!(dialog.first_name.is_none());
        assert!(dialog.last_name.is_none());
        assert!(dialog.username.is_none());
    }

    #[test]
    fn test_default() {
        let dialog = SharedDialog::default();
        assert_eq!(dialog.dialog_id(), DialogId::new(0));
        assert!(!dialog.is_valid());
    }

    #[test]
    fn test_with_details() {
        let dialog = SharedDialog::with_details(
            DialogId::new(1234567890),
            Some("John"),
            Some("Doe"),
            Some("johndoe"),
        );
        assert_eq!(dialog.first_name(), Some("John"));
        assert_eq!(dialog.last_name(), Some("Doe"));
        assert_eq!(dialog.username(), Some("johndoe"));
    }

    #[test]
    fn test_with_details_none() {
        let dialog = SharedDialog::with_details(
            DialogId::new(1234567890),
            None::<String>,
            None::<String>,
            None::<String>,
        );
        assert_eq!(dialog.first_name(), None);
        assert_eq!(dialog.last_name(), None);
        assert_eq!(dialog.username(), None);
    }

    #[test]
    fn test_is_valid() {
        let valid = SharedDialog::new(DialogId::new(1234567890));
        assert!(valid.is_valid());

        let invalid = SharedDialog::new(DialogId::new(0));
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_is_user() {
        let user_dialog = SharedDialog::new(DialogId::new(1234567890));
        assert!(user_dialog.is_user());
        assert!(!user_dialog.is_dialog());
    }

    #[test]
    fn test_is_dialog_chat() {
        let chat_dialog = SharedDialog::new(DialogId::new(-1234567890));
        assert!(!chat_dialog.is_user());
        assert!(chat_dialog.is_dialog());
    }

    #[test]
    fn test_equality() {
        let dialog1 = SharedDialog::new(DialogId::new(1234567890));
        let dialog2 = SharedDialog::new(DialogId::new(1234567890));
        let dialog3 = SharedDialog::new(DialogId::new(9876543210));

        assert_eq!(dialog1, dialog2);
        assert_ne!(dialog1, dialog3);
    }

    #[test]
    fn test_equality_with_details() {
        let dialog1 = SharedDialog::with_details(
            DialogId::new(1234567890),
            Some("John"),
            Some("Doe"),
            Some("johndoe"),
        );
        let dialog2 = SharedDialog::with_details(
            DialogId::new(1234567890),
            Some("John"),
            Some("Doe"),
            Some("johndoe"),
        );
        let dialog3 = SharedDialog::with_details(
            DialogId::new(1234567890),
            Some("Jane"),
            Some("Smith"),
            Some("janesmith"),
        );

        assert_eq!(dialog1, dialog2);
        assert_ne!(dialog1, dialog3);
    }

    #[test]
    fn test_clone() {
        let dialog1 = SharedDialog::with_details(
            DialogId::new(1234567890),
            Some("John"),
            Some("Doe"),
            Some("johndoe"),
        );
        let dialog2 = dialog1.clone();
        assert_eq!(dialog1, dialog2);
    }

    #[test]
    fn test_display() {
        let dialog = SharedDialog::with_details(
            DialogId::new(1234567890),
            Some("John"),
            Some("Doe"),
            Some("johndoe"),
        );
        let display = format!("{dialog}");
        assert!(display.contains("SharedDialog"));
        assert!(display.contains("John"));
        assert!(display.contains("@johndoe"));
    }

    #[test]
    fn test_display_minimal() {
        let dialog = SharedDialog::new(DialogId::new(1234567890));
        let display = format!("{dialog}");
        assert!(display.contains("SharedDialog"));
    }

    #[test]
    fn test_serialization() {
        let dialog = SharedDialog::with_details(
            DialogId::new(1234567890),
            Some("John"),
            Some("Doe"),
            Some("johndoe"),
        );
        let json = serde_json::to_string(&dialog).expect("Failed to serialize");

        let deserialized: SharedDialog =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, dialog);
    }

    #[test]
    fn test_serialization_minimal() {
        let dialog = SharedDialog::new(DialogId::new(1234567890));
        let json = serde_json::to_string(&dialog).expect("Failed to serialize");

        let deserialized: SharedDialog =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, dialog);
    }

    #[test]
    fn test_from_string_options() {
        let dialog = SharedDialog::with_details(
            DialogId::new(1234567890),
            Some("John".to_string()),
            Some("Doe".to_string()),
            Some("johndoe".to_string()),
        );
        assert_eq!(dialog.first_name(), Some("John"));
        assert_eq!(dialog.last_name(), Some("Doe"));
        assert_eq!(dialog.username(), Some("johndoe"));
    }

    #[test]
    fn test_empty_strings() {
        let dialog =
            SharedDialog::with_details(DialogId::new(1234567890), Some(""), Some(""), Some(""));
        assert_eq!(dialog.first_name(), Some(""));
        assert_eq!(dialog.last_name(), Some(""));
        assert_eq!(dialog.username(), Some(""));
    }
}
