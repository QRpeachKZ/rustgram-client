// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Star Gift ID
//!
//! Star gift identifier for Telegram.
//!
//! Based on TDLib's `StarGiftId` from `td/telegram/StarGiftId.h`.
//!
//! # Overview
//!
//! A `StarGiftId` uniquely identifies a star gift in Telegram.
//! Star gifts can be identified in different ways:
//! - By server message ID (ForUser)
//! - By dialog ID and saved message ID (ForDialog)
//! - By a slug string (Slug)
//!
//! # Example
//!
//! ```rust
//! use rustgram_star_gift_id::StarGiftId;
//!
//! let id = StarGiftId::from_slug("my_gift_slug");
//! assert!(id.is_valid());
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_dialog_id::DialogId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Star gift type identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
enum Type {
    /// Empty/invalid gift ID
    #[default]
    Empty = 0,
    /// Gift from a user (identified by server message ID)
    ForUser = 1,
    /// Gift in a dialog (identified by dialog ID and saved message ID)
    ForDialog = 2,
    /// Gift identified by a slug string
    Slug = 3,
}

/// Star gift identifier.
///
/// Represents a unique identifier for a Telegram star gift.
/// Can be one of three types: ForUser, ForDialog, or Slug.
///
/// # TDLib Mapping
///
/// - `StarGiftId::for_user(message_id)` → TDLib: `StarGiftId(ServerMessageId)`
/// - `StarGiftId::for_dialog(dialog_id, saved_id)` → TDLib: `StarGiftId(DialogId, int64)`
/// - `StarGiftId::from_slug(slug)` → TDLib: `StarGiftId::from_slug(string)`
///
/// # Example
///
/// ```rust
/// use rustgram_star_gift_id::StarGiftId;
///
/// let slug_id = StarGiftId::from_slug("my_gift");
/// assert!(slug_id.is_valid());
/// assert!(slug_id.is_slug());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StarGiftId {
    type_: Type,
    server_message_id: Option<i32>,
    dialog_id: Option<DialogId>,
    saved_id: Option<i64>,
    slug: Option<String>,
}

impl StarGiftId {
    /// Creates a new empty `StarGiftId`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_id::StarGiftId;
    ///
    /// let id = StarGiftId::new();
    /// assert!(!id.is_valid());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `StarGiftId` from a server message ID (ForUser type).
    ///
    /// # Arguments
    ///
    /// * `server_message_id` - The server message ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_id::StarGiftId;
    ///
    /// let id = StarGiftId::for_user(12345);
    /// assert!(id.is_valid());
    /// assert!(id.is_for_user());
    /// ```
    #[must_use]
    pub fn for_user(server_message_id: i32) -> Self {
        Self {
            type_: Type::ForUser,
            server_message_id: Some(server_message_id),
            dialog_id: None,
            saved_id: None,
            slug: None,
        }
    }

    /// Creates a `StarGiftId` from a dialog ID and saved message ID (ForDialog type).
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    /// * `saved_id` - The saved message ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_id::StarGiftId;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog_id = DialogId::new(1234567890);
    /// let id = StarGiftId::for_dialog(dialog_id, 98765);
    /// assert!(id.is_valid());
    /// assert!(id.is_for_dialog());
    /// ```
    #[must_use]
    pub fn for_dialog(dialog_id: DialogId, saved_id: i64) -> Self {
        Self {
            type_: Type::ForDialog,
            server_message_id: None,
            dialog_id: Some(dialog_id),
            saved_id: Some(saved_id),
            slug: None,
        }
    }

    /// Creates a `StarGiftId` from a slug string (Slug type).
    ///
    /// # Arguments
    ///
    /// * `slug` - The slug string
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_id::StarGiftId;
    ///
    /// let id = StarGiftId::from_slug("my_gift_slug");
    /// assert!(id.is_valid());
    /// assert!(id.is_slug());
    /// ```
    #[must_use]
    pub fn from_slug(slug: impl Into<String>) -> Self {
        Self {
            type_: Type::Slug,
            server_message_id: None,
            dialog_id: None,
            saved_id: None,
            slug: Some(slug.into()),
        }
    }

    /// Checks if this is a valid star gift ID.
    ///
    /// # Returns
    ///
    /// Returns `true` if the ID is valid (not Empty), `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_id::StarGiftId;
    ///
    /// assert!(!StarGiftId::new().is_valid());
    /// assert!(StarGiftId::from_slug("gift").is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !matches!(self.type_, Type::Empty)
    }

    /// Checks if this is an empty gift ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_id::StarGiftId;
    ///
    /// assert!(StarGiftId::new().is_empty());
    /// assert!(!StarGiftId::from_slug("gift").is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self.type_, Type::Empty)
    }

    /// Checks if this is a ForUser gift ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_id::StarGiftId;
    ///
    /// assert!(StarGiftId::for_user(123).is_for_user());
    /// assert!(!StarGiftId::from_slug("gift").is_for_user());
    /// ```
    #[must_use]
    pub fn is_for_user(&self) -> bool {
        matches!(self.type_, Type::ForUser)
    }

    /// Checks if this is a ForDialog gift ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_id::StarGiftId;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog_id = DialogId::new(1234567890);
    /// assert!(StarGiftId::for_dialog(dialog_id, 123).is_for_dialog());
    /// assert!(!StarGiftId::from_slug("gift").is_for_dialog());
    /// ```
    #[must_use]
    pub fn is_for_dialog(&self) -> bool {
        matches!(self.type_, Type::ForDialog)
    }

    /// Checks if this is a Slug gift ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_id::StarGiftId;
    ///
    /// assert!(StarGiftId::from_slug("gift").is_slug());
    /// assert!(!StarGiftId::for_user(123).is_slug());
    /// ```
    #[must_use]
    pub fn is_slug(&self) -> bool {
        matches!(self.type_, Type::Slug)
    }

    /// Returns the server message ID if this is a ForUser gift.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_id::StarGiftId;
    ///
    /// let id = StarGiftId::for_user(12345);
    /// assert_eq!(id.server_message_id(), Some(12345));
    /// ```
    #[must_use]
    pub fn server_message_id(&self) -> Option<i32> {
        self.server_message_id
    }

    /// Returns the dialog ID if this is a ForDialog gift.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_id::StarGiftId;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog_id = DialogId::new(1234567890);
    /// let id = StarGiftId::for_dialog(dialog_id, 123);
    /// assert_eq!(id.dialog_id(), Some(dialog_id));
    /// ```
    #[must_use]
    pub fn dialog_id(&self) -> Option<DialogId> {
        self.dialog_id
    }

    /// Returns the saved message ID if this is a ForDialog gift.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_id::StarGiftId;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog_id = DialogId::new(1234567890);
    /// let id = StarGiftId::for_dialog(dialog_id, 98765);
    /// assert_eq!(id.saved_id(), Some(98765));
    /// ```
    #[must_use]
    pub fn saved_id(&self) -> Option<i64> {
        self.saved_id
    }

    /// Returns the slug if this is a Slug gift.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_id::StarGiftId;
    ///
    /// let id = StarGiftId::from_slug("my_gift");
    /// assert_eq!(id.slug(), Some("my_gift"));
    /// ```
    #[must_use]
    pub fn slug(&self) -> Option<&str> {
        self.slug.as_deref()
    }
}

impl fmt::Display for StarGiftId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.type_ {
            Type::Empty => write!(f, "unknown gift"),
            Type::ForUser => write!(f, "user gift from msg {}", self.server_message_id.unwrap_or(0)),
            Type::ForDialog => {
                write!(
                    f,
                    "dialog gift {} msg {}",
                    self.dialog_id.unwrap_or_default(),
                    self.saved_id.unwrap_or(0)
                )
            }
            Type::Slug => write!(f, "gift {}", self.slug.as_deref().unwrap_or("")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = StarGiftId::new();
        assert!(!id.is_valid());
        assert!(id.is_empty());
    }

    #[test]
    fn test_default() {
        let id = StarGiftId::default();
        assert!(!id.is_valid());
        assert!(id.is_empty());
    }

    #[test]
    fn test_for_user() {
        let id = StarGiftId::for_user(12345);
        assert!(id.is_valid());
        assert!(id.is_for_user());
        assert!(!id.is_for_dialog());
        assert!(!id.is_slug());
        assert_eq!(id.server_message_id(), Some(12345));
        assert_eq!(id.dialog_id(), None);
        assert_eq!(id.saved_id(), None);
        assert_eq!(id.slug(), None);
    }

    #[test]
    fn test_for_dialog() {
        let dialog_id = DialogId::new(1234567890);
        let id = StarGiftId::for_dialog(dialog_id, 98765);
        assert!(id.is_valid());
        assert!(!id.is_for_user());
        assert!(id.is_for_dialog());
        assert!(!id.is_slug());
        assert_eq!(id.server_message_id(), None);
        assert_eq!(id.dialog_id(), Some(dialog_id));
        assert_eq!(id.saved_id(), Some(98765));
        assert_eq!(id.slug(), None);
    }

    #[test]
    fn test_from_slug() {
        let id = StarGiftId::from_slug("my_gift_slug");
        assert!(id.is_valid());
        assert!(!id.is_for_user());
        assert!(!id.is_for_dialog());
        assert!(id.is_slug());
        assert_eq!(id.server_message_id(), None);
        assert_eq!(id.dialog_id(), None);
        assert_eq!(id.saved_id(), None);
        assert_eq!(id.slug(), Some("my_gift_slug"));
    }

    #[test]
    fn test_from_slug_string() {
        let slug = String::from("test_slug");
        let id = StarGiftId::from_slug(slug.clone());
        assert_eq!(id.slug(), Some("test_slug"));
    }

    #[test]
    fn test_is_valid() {
        assert!(!StarGiftId::new().is_valid());
        assert!(StarGiftId::for_user(123).is_valid());
        assert!(StarGiftId::for_dialog(DialogId::new(1), 1).is_valid());
        assert!(StarGiftId::from_slug("gift").is_valid());
    }

    #[test]
    fn test_is_empty() {
        assert!(StarGiftId::new().is_empty());
        assert!(!StarGiftId::for_user(123).is_empty());
        assert!(!StarGiftId::from_slug("gift").is_empty());
    }

    #[test]
    fn test_equality() {
        let id1 = StarGiftId::for_user(12345);
        let id2 = StarGiftId::for_user(12345);
        let id3 = StarGiftId::for_user(54321);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_clone() {
        let id1 = StarGiftId::from_slug("my_gift");
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_display_for_user() {
        let id = StarGiftId::for_user(12345);
        let display = format!("{id}");
        assert!(display.contains("user gift"));
        assert!(display.contains("12345"));
    }

    #[test]
    fn test_display_for_dialog() {
        let dialog_id = DialogId::new(1234567890);
        let id = StarGiftId::for_dialog(dialog_id, 98765);
        let display = format!("{id}");
        assert!(display.contains("dialog gift"));
        assert!(display.contains("98765"));
    }

    #[test]
    fn test_display_slug() {
        let id = StarGiftId::from_slug("my_gift");
        let display = format!("{id}");
        assert!(display.contains("gift"));
        assert!(display.contains("my_gift"));
    }

    #[test]
    fn test_display_empty() {
        let id = StarGiftId::new();
        let display = format!("{id}");
        assert!(display.contains("unknown"));
    }

    #[test]
    fn test_serialization_for_user() {
        let id = StarGiftId::for_user(12345);
        let json = serde_json::to_string(&id).expect("Failed to serialize");
        let deserialized: StarGiftId = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, id);
    }

    #[test]
    fn test_serialization_for_dialog() {
        let dialog_id = DialogId::new(1234567890);
        let id = StarGiftId::for_dialog(dialog_id, 98765);
        let json = serde_json::to_string(&id).expect("Failed to serialize");
        let deserialized: StarGiftId = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, id);
    }

    #[test]
    fn test_serialization_slug() {
        let id = StarGiftId::from_slug("my_gift");
        let json = serde_json::to_string(&id).expect("Failed to serialize");
        let deserialized: StarGiftId = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, id);
    }

    #[test]
    fn test_empty_server_message_id() {
        let id = StarGiftId::for_user(0);
        assert_eq!(id.server_message_id(), Some(0));
        assert!(id.is_valid());
    }

    #[test]
    fn test_empty_saved_id() {
        let dialog_id = DialogId::new(1234567890);
        let id = StarGiftId::for_dialog(dialog_id, 0);
        assert_eq!(id.saved_id(), Some(0));
        assert!(id.is_valid());
    }

    #[test]
    fn test_empty_slug() {
        let id = StarGiftId::from_slug("");
        assert_eq!(id.slug(), Some(""));
        assert!(id.is_valid());
    }

    #[test]
    fn test_types_are_mutually_exclusive() {
        let user_id = StarGiftId::for_user(123);
        let dialog_id = StarGiftId::for_dialog(DialogId::new(1), 1);
        let slug_id = StarGiftId::from_slug("gift");

        assert!(user_id.is_for_user());
        assert!(!user_id.is_for_dialog());
        assert!(!user_id.is_slug());

        assert!(!dialog_id.is_for_user());
        assert!(dialog_id.is_for_dialog());
        assert!(!dialog_id.is_slug());

        assert!(!slug_id.is_for_user());
        assert!(!slug_id.is_for_dialog());
        assert!(slug_id.is_slug());
    }
}
