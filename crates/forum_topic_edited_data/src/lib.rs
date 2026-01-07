// rustgram_forum_topic_edited_data
// Copyright (C) 2025 rustgram-client contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! # Forum Topic Edited Data
//!
//! Represents data about edits made to a forum topic.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_forum_topic_edited_data::ForumTopicEditedData;
//!
//! let data = ForumTopicEditedData::new();
//! assert!(data.is_empty());
//!
//! let data_with_title = ForumTopicEditedData::with_title("New Title".to_string());
//! assert!(!data_with_title.is_empty());
//! assert_eq!(data_with_title.title(), "New Title");
//! ```

use std::fmt;

/// Custom emoji identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomEmojiId(i64);

impl CustomEmojiId {
    /// Creates a new custom emoji ID.
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the underlying i64 value.
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }
}

/// Data about edits made to a forum topic.
///
/// Tracks changes to title, icon, closed state, and hidden state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForumTopicEditedData {
    /// New title (empty if not edited)
    title: String,
    /// Icon custom emoji ID
    icon_custom_emoji_id: CustomEmojiId,
    /// Whether the icon was edited
    edit_icon_custom_emoji_id: bool,
    /// Whether the closed state was edited
    edit_is_closed: bool,
    /// New closed state
    is_closed: bool,
    /// Whether the hidden state was edited
    edit_is_hidden: bool,
    /// New hidden state
    is_hidden: bool,
}

impl Default for ForumTopicEditedData {
    fn default() -> Self {
        Self::new()
    }
}

impl ForumTopicEditedData {
    /// Creates a new empty forum topic edited data.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_edited_data::ForumTopicEditedData;
    ///
    /// let data = ForumTopicEditedData::new();
    /// assert!(data.is_empty());
    /// assert_eq!(data.title(), "");
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: String::new(),
            icon_custom_emoji_id: CustomEmojiId::new(0),
            edit_icon_custom_emoji_id: false,
            edit_is_closed: false,
            is_closed: false,
            edit_is_hidden: false,
            is_hidden: false,
        }
    }

    /// Creates a new forum topic edited data with only a title change.
    ///
    /// # Arguments
    ///
    /// * `title` - New title for the topic
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_edited_data::ForumTopicEditedData;
    ///
    /// let data = ForumTopicEditedData::with_title("New Title".to_string());
    /// assert_eq!(data.title(), "New Title");
    /// assert!(!data.is_empty());
    /// ```
    #[must_use]
    pub fn with_title(title: String) -> Self {
        Self {
            title,
            ..Default::default()
        }
    }

    /// Creates a new forum topic edited data with all fields.
    ///
    /// # Arguments
    ///
    /// * `title` - New title
    /// * `edit_icon_custom_emoji_id` - Whether icon is being edited
    /// * `icon_custom_emoji_id` - New icon custom emoji ID
    /// * `edit_is_closed` - Whether closed state is being edited
    /// * `is_closed` - New closed state
    /// * `edit_is_hidden` - Whether hidden state is being edited
    /// * `is_hidden` - New hidden state
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_edited_data::ForumTopicEditedData;
    ///
    /// let data = ForumTopicEditedData::with_all_fields(
    ///     "Title".to_string(),
    ///     true,
    ///     12345,
    ///     true,
    ///     false,
    ///     false,
    ///     false,
    /// );
    /// assert_eq!(data.title(), "Title");
    /// assert!(data.editing_icon_custom_emoji_id());
    /// assert!(data.editing_is_closed());
    /// ```
    #[must_use]
    pub fn with_all_fields(
        title: String,
        edit_icon_custom_emoji_id: bool,
        icon_custom_emoji_id: i64,
        edit_is_closed: bool,
        is_closed: bool,
        edit_is_hidden: bool,
        is_hidden: bool,
    ) -> Self {
        Self {
            title,
            icon_custom_emoji_id: CustomEmojiId::new(icon_custom_emoji_id),
            edit_icon_custom_emoji_id,
            edit_is_closed,
            is_closed,
            edit_is_hidden,
            is_hidden,
        }
    }

    /// Returns the title.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_edited_data::ForumTopicEditedData;
    ///
    /// let data = ForumTopicEditedData::with_title("Test".to_string());
    /// assert_eq!(data.title(), "Test");
    /// ```
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the icon custom emoji ID if valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_edited_data::ForumTopicEditedData;
    ///
    /// let data = ForumTopicEditedData::with_all_fields(
    ///     String::new(), true, 12345, false, false, false, false
    /// );
    /// assert_eq!(data.icon_custom_emoji_id(), Some(12345));
    /// ```
    #[must_use]
    pub fn icon_custom_emoji_id(&self) -> Option<i64> {
        let id = self.icon_custom_emoji_id.get();
        if id != 0 {
            Some(id)
        } else {
            None
        }
    }

    /// Returns whether the icon custom emoji ID is being edited.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_edited_data::ForumTopicEditedData;
    ///
    /// let data = ForumTopicEditedData::with_all_fields(
    ///     String::new(), true, 12345, false, false, false, false
    /// );
    /// assert!(data.editing_icon_custom_emoji_id());
    /// ```
    #[must_use]
    pub const fn editing_icon_custom_emoji_id(&self) -> bool {
        self.edit_icon_custom_emoji_id
    }

    /// Returns whether the closed state is being edited.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_edited_data::ForumTopicEditedData;
    ///
    /// let data = ForumTopicEditedData::with_all_fields(
    ///     String::new(), false, 0, true, true, false, false
    /// );
    /// assert!(data.editing_is_closed());
    /// ```
    #[must_use]
    pub const fn editing_is_closed(&self) -> bool {
        self.edit_is_closed
    }

    /// Returns the new closed state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_edited_data::ForumTopicEditedData;
    ///
    /// let data = ForumTopicEditedData::with_all_fields(
    ///     String::new(), false, 0, true, true, false, false
    /// );
    /// assert!(data.is_closed());
    /// ```
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        self.is_closed
    }

    /// Returns whether the hidden state is being edited.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_edited_data::ForumTopicEditedData;
    ///
    /// let data = ForumTopicEditedData::with_all_fields(
    ///     String::new(), false, 0, false, false, true, true
    /// );
    /// assert!(data.editing_is_hidden());
    /// ```
    #[must_use]
    pub const fn editing_is_hidden(&self) -> bool {
        self.edit_is_hidden
    }

    /// Returns the new hidden state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_edited_data::ForumTopicEditedData;
    ///
    /// let data = ForumTopicEditedData::with_all_fields(
    ///     String::new(), false, 0, false, false, true, true
    /// );
    /// assert!(data.is_hidden());
    /// ```
    #[must_use]
    pub const fn is_hidden(&self) -> bool {
        self.is_hidden
    }

    /// Checks if no edits were made.
    ///
    /// Returns `true` if:
    /// - Title is empty
    /// - Icon is not being edited
    /// - Closed state is not being edited
    /// - Hidden state is not being edited
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_edited_data::ForumTopicEditedData;
    ///
    /// let empty = ForumTopicEditedData::new();
    /// assert!(empty.is_empty());
    ///
    /// let with_title = ForumTopicEditedData::with_title("Title".to_string());
    /// assert!(!with_title.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.title.is_empty()
            && !self.edit_icon_custom_emoji_id
            && !self.edit_is_closed
            && !self.edit_is_hidden
    }

    /// Determines the message content type based on the edited data.
    ///
    /// Returns a string representing the message type:
    /// - `"hidden_toggled"` if editing hidden with specific condition
    /// - `"closed_toggled"` if editing closed
    /// - `"edited"` otherwise
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_forum_topic_edited_data::ForumTopicEditedData;
    ///
    /// // Hidden toggle case
    /// let hidden = ForumTopicEditedData::with_all_fields(
    ///     String::new(), false, 0, false, false, true, true
    /// );
    /// assert_eq!(hidden.message_content_type(), "hidden_toggled");
    ///
    /// // Closed toggle case
    /// let closed = ForumTopicEditedData::with_all_fields(
    ///     String::new(), false, 0, true, true, false, false
    /// );
    /// assert_eq!(closed.message_content_type(), "closed_toggled");
    ///
    /// // Regular edit case
    /// let edited = ForumTopicEditedData::with_title("Title".to_string());
    /// assert_eq!(edited.message_content_type(), "edited");
    /// ```
    #[must_use]
    #[allow(clippy::overly_complex_bool_expr)]
    pub fn message_content_type(&self) -> &'static str {
        // Condition 1: edit_is_hidden && !(!is_hidden && edit_is_closed && !is_closed)
        // Note: This is based on TDLib logic where the condition checks if we're NOT
        // unhiding the topic while closing it
        if self.edit_is_hidden && !(self.is_hidden && self.edit_is_closed && !self.is_closed) {
            return "hidden_toggled";
        }

        // Condition 2: edit_is_closed
        if self.edit_is_closed {
            return "closed_toggled";
        }

        // Default: edited
        "edited"
    }
}

impl fmt::Display for ForumTopicEditedData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if !self.title.is_empty() {
            parts.push(format!("set title to \"{}\"", self.title));
        }

        if self.edit_icon_custom_emoji_id {
            if let Some(emoji_id) = self.icon_custom_emoji_id() {
                parts.push(format!("set icon to {}", emoji_id));
            }
        }

        if self.edit_is_closed {
            parts.push(format!("set is_closed to {}", self.is_closed));
        }

        if self.edit_is_hidden {
            parts.push(format!("set is_hidden to {}", self.is_hidden));
        }

        if parts.is_empty() {
            write!(f, "(no changes)")
        } else {
            write!(f, "{}", parts.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let data = ForumTopicEditedData::default();
        assert!(data.is_empty());
        assert_eq!(data.title(), "");
        assert!(!data.editing_icon_custom_emoji_id());
        assert!(!data.editing_is_closed());
        assert!(!data.editing_is_hidden());
    }

    #[test]
    fn test_new() {
        let data = ForumTopicEditedData::new();
        assert!(data.is_empty());
        assert_eq!(data.title(), "");
    }

    #[test]
    fn test_with_title() {
        let data = ForumTopicEditedData::with_title("New Title".to_string());
        assert_eq!(data.title(), "New Title");
        assert!(!data.is_empty());
    }

    #[test]
    fn test_with_all_fields() {
        let data = ForumTopicEditedData::with_all_fields(
            "Title".to_string(),
            true,
            12345,
            true,
            false,
            false,
            false,
        );

        assert_eq!(data.title(), "Title");
        assert!(data.editing_icon_custom_emoji_id());
        assert_eq!(data.icon_custom_emoji_id(), Some(12345));
        assert!(data.editing_is_closed());
        assert!(!data.is_closed());
        assert!(!data.editing_is_hidden());
    }

    #[test]
    fn test_is_empty() {
        let empty = ForumTopicEditedData::new();
        assert!(empty.is_empty());

        let with_title = ForumTopicEditedData::with_title("Title".to_string());
        assert!(!with_title.is_empty());

        let with_icon = ForumTopicEditedData::with_all_fields(
            String::new(),
            true,
            12345,
            false,
            false,
            false,
            false,
        );
        assert!(!with_icon.is_empty());

        let with_closed = ForumTopicEditedData::with_all_fields(
            String::new(),
            false,
            0,
            true,
            true,
            false,
            false,
        );
        assert!(!with_closed.is_empty());

        let with_hidden = ForumTopicEditedData::with_all_fields(
            String::new(),
            false,
            0,
            false,
            false,
            true,
            true,
        );
        assert!(!with_hidden.is_empty());
    }

    #[test]
    fn test_message_content_type_hidden_toggled() {
        // edit_is_hidden && specific condition
        let data = ForumTopicEditedData::with_all_fields(
            String::new(),
            false,
            0,
            false,
            false,
            true,
            true,
        );
        assert_eq!(data.message_content_type(), "hidden_toggled");

        // With title but still hidden toggle priority
        let data2 = ForumTopicEditedData::with_all_fields(
            "Title".to_string(),
            false,
            0,
            false,
            false,
            true,
            true,
        );
        assert_eq!(data2.message_content_type(), "hidden_toggled");
    }

    #[test]
    fn test_message_content_type_closed_toggled() {
        let data = ForumTopicEditedData::with_all_fields(
            String::new(),
            false,
            0,
            true,
            true,
            false,
            false,
        );
        assert_eq!(data.message_content_type(), "closed_toggled");

        let data2 = ForumTopicEditedData::with_all_fields(
            String::new(),
            false,
            0,
            true,
            false,
            false,
            false,
        );
        assert_eq!(data2.message_content_type(), "closed_toggled");
    }

    #[test]
    fn test_message_content_type_edited() {
        let data = ForumTopicEditedData::with_title("Title".to_string());
        assert_eq!(data.message_content_type(), "edited");

        let data2 = ForumTopicEditedData::with_all_fields(
            "Title".to_string(),
            true,
            12345,
            false,
            false,
            false,
            false,
        );
        assert_eq!(data2.message_content_type(), "edited");
    }

    #[test]
    fn test_display_no_changes() {
        let data = ForumTopicEditedData::new();
        assert_eq!(format!("{}", data), "(no changes)");
    }

    #[test]
    fn test_display_title() {
        let data = ForumTopicEditedData::with_title("New Title".to_string());
        assert_eq!(format!("{}", data), "set title to \"New Title\"");
    }

    #[test]
    fn test_display_icon() {
        let data = ForumTopicEditedData::with_all_fields(
            String::new(),
            true,
            12345,
            false,
            false,
            false,
            false,
        );
        assert_eq!(format!("{}", data), "set icon to 12345");
    }

    #[test]
    fn test_display_closed() {
        let data = ForumTopicEditedData::with_all_fields(
            String::new(),
            false,
            0,
            true,
            true,
            false,
            false,
        );
        assert_eq!(format!("{}", data), "set is_closed to true");
    }

    #[test]
    fn test_display_hidden() {
        let data = ForumTopicEditedData::with_all_fields(
            String::new(),
            false,
            0,
            false,
            false,
            true,
            false,
        );
        assert_eq!(format!("{}", data), "set is_hidden to false");
    }

    #[test]
    fn test_display_multiple() {
        let data = ForumTopicEditedData::with_all_fields(
            "Title".to_string(),
            true,
            12345,
            true,
            true,
            true,
            false,
        );
        let display = format!("{}", data);
        assert!(display.contains("set title to \"Title\""));
        assert!(display.contains("set icon to 12345"));
        assert!(display.contains("set is_closed to true"));
        assert!(display.contains("set is_hidden to false"));
    }

    #[test]
    fn test_equality() {
        let a = ForumTopicEditedData::with_title("Title".to_string());
        let b = ForumTopicEditedData::with_title("Title".to_string());
        let c = ForumTopicEditedData::with_title("Other".to_string());

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_icon_custom_emoji_id_none() {
        let data = ForumTopicEditedData::new();
        assert_eq!(data.icon_custom_emoji_id(), None);
    }

    #[test]
    fn test_icon_custom_emoji_id_some() {
        let data = ForumTopicEditedData::with_all_fields(
            String::new(),
            true,
            12345,
            false,
            false,
            false,
            false,
        );
        assert_eq!(data.icon_custom_emoji_id(), Some(12345));
    }

    #[test]
    fn test_cloning() {
        let data1 = ForumTopicEditedData::with_all_fields(
            "Title".to_string(),
            true,
            12345,
            true,
            true,
            true,
            false,
        );
        let data2 = data1.clone();
        assert_eq!(data1, data2);
    }

    #[test]
    fn test_getters() {
        let data = ForumTopicEditedData::with_all_fields(
            "Test".to_string(),
            true,
            999,
            true,
            false,
            true,
            true,
        );

        assert_eq!(data.title(), "Test");
        assert_eq!(data.icon_custom_emoji_id(), Some(999));
        assert!(data.editing_icon_custom_emoji_id());
        assert!(data.editing_is_closed());
        assert!(!data.is_closed());
        assert!(data.editing_is_hidden());
        assert!(data.is_hidden());
    }
}
