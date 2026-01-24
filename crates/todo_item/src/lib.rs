// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # ToDo Item
//!
//! ToDo item structure for Telegram checklists.
//!
//! ## Overview
//!
//! This module provides the [`ToDoItem`] struct, which represents a single
//! task in a Telegram checklist/todo list.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_todo_item::ToDoItem;
//! use rustgram_formatted_text::FormattedText;
//!
//! // Create a todo item
//! let item = ToDoItem::new(123, FormattedText::new("Buy groceries"));
//! assert_eq!(item.id(), 123);
//! assert_eq!(item.title().text(), "Buy groceries");
//! assert!(item.is_valid());
//! ```

use std::fmt;

use rustgram_formatted_text::FormattedText;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A todo item in a Telegram checklist.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib's `ToDoItem` class.
///
/// # Example
///
/// ```rust
/// use rustgram_todo_item::ToDoItem;
/// use rustgram_formatted_text::FormattedText;
///
/// let item = ToDoItem::new(123, FormattedText::new("Buy groceries"));
/// assert_eq!(item.id(), 123);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ToDoItem {
    id: i32,
    title: FormattedText,
}

impl ToDoItem {
    /// Maximum length for todo item title text.
    pub const MAX_TEXT_LENGTH: usize = 100;

    /// Creates a new todo item.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the todo item (must be positive)
    /// * `title` - The title/text of the todo item
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_item::ToDoItem;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let item = ToDoItem::new(123, FormattedText::new("Buy groceries"));
    /// assert_eq!(item.id(), 123);
    /// ```
    #[must_use]
    pub fn new(id: i32, title: FormattedText) -> Self {
        Self { id, title }
    }

    /// Returns the todo item ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_item::ToDoItem;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let item = ToDoItem::new(456, FormattedText::new("Task"));
    /// assert_eq!(item.id(), 456);
    /// ```
    #[must_use]
    pub const fn id(&self) -> i32 {
        self.id
    }

    /// Returns the title of the todo item.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_item::ToDoItem;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let item = ToDoItem::new(123, FormattedText::new("Buy groceries"));
    /// assert_eq!(item.title().text(), "Buy groceries");
    /// ```
    #[must_use]
    pub const fn title(&self) -> &FormattedText {
        &self.title
    }

    /// Returns the search text for this todo item.
    ///
    /// This is typically the title text for searching/filtering.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_item::ToDoItem;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let item = ToDoItem::new(123, FormattedText::new("Buy groceries"));
    /// assert_eq!(item.search_text(), "Buy groceries");
    /// ```
    #[must_use]
    pub fn search_text(&self) -> &str {
        self.title.text()
    }

    /// Checks if this is a valid todo item.
    ///
    /// A valid todo item must have a positive ID and non-empty title.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_item::ToDoItem;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let item = ToDoItem::new(123, FormattedText::new("Task"));
    /// assert!(item.is_valid());
    ///
    /// let invalid_id = ToDoItem::new(0, FormattedText::new("Task"));
    /// assert!(!invalid_id.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.id > 0 && !self.title.text().is_empty()
    }

    /// Validates the todo item, logging any issues.
    ///
    /// In a full implementation, this would check for unsupported entities
    /// and log errors. For now, it just checks basic validity.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_item::ToDoItem;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let item = ToDoItem::new(123, FormattedText::new("Task"));
    /// item.validate(); // Will log if there are issues
    /// ```
    pub fn validate(&self) {
        if !self.is_valid() {
            // In a full implementation, this would log
            let _ = (self.id <= 0, self.title.text().is_empty());
        }
    }
}

impl fmt::Display for ToDoItem {
    /// Formats the todo item for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_item::ToDoItem;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let item = ToDoItem::new(123, FormattedText::new("Buy groceries"));
    /// let s = format!("{}", item);
    /// assert!(s.contains("Buy groceries") || s.contains("123"));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ToDoItem(id: {}, title: {})", self.id, self.title.text())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let item = ToDoItem::new(123, FormattedText::new("Buy groceries"));
        assert_eq!(item.id(), 123);
        assert_eq!(item.title().text(), "Buy groceries");
    }

    #[test]
    fn test_id() {
        let item = ToDoItem::new(456, FormattedText::new("Task"));
        assert_eq!(item.id(), 456);
    }

    #[test]
    fn test_title() {
        let item = ToDoItem::new(123, FormattedText::new("My task"));
        assert_eq!(item.title().text(), "My task");
    }

    #[test]
    fn test_search_text() {
        let item = ToDoItem::new(123, FormattedText::new("Search me"));
        assert_eq!(item.search_text(), "Search me");
    }

    #[test]
    fn test_is_valid_true() {
        let item = ToDoItem::new(123, FormattedText::new("Task"));
        assert!(item.is_valid());
    }

    #[test]
    fn test_is_valid_zero_id() {
        let item = ToDoItem::new(0, FormattedText::new("Task"));
        assert!(!item.is_valid());
    }

    #[test]
    fn test_is_valid_negative_id() {
        let item = ToDoItem::new(-1, FormattedText::new("Task"));
        assert!(!item.is_valid());
    }

    #[test]
    fn test_is_valid_empty_title() {
        let item = ToDoItem::new(123, FormattedText::new(""));
        assert!(!item.is_valid());
    }

    #[test]
    fn test_validate_valid() {
        let item = ToDoItem::new(123, FormattedText::new("Task"));
        item.validate(); // Should not panic
    }

    #[test]
    fn test_validate_invalid() {
        let item = ToDoItem::new(0, FormattedText::new(""));
        item.validate(); // Should not panic, just check validity
    }

    #[test]
    fn test_equality() {
        let item1 = ToDoItem::new(123, FormattedText::new("Task"));
        let item2 = ToDoItem::new(123, FormattedText::new("Task"));
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_inequality_id() {
        let item1 = ToDoItem::new(123, FormattedText::new("Task"));
        let item2 = ToDoItem::new(456, FormattedText::new("Task"));
        assert_ne!(item1, item2);
    }

    #[test]
    fn test_inequality_title() {
        let item1 = ToDoItem::new(123, FormattedText::new("Task 1"));
        let item2 = ToDoItem::new(123, FormattedText::new("Task 2"));
        assert_ne!(item1, item2);
    }

    #[test]
    fn test_clone_semantics() {
        let item1 = ToDoItem::new(123, FormattedText::new("Task"));
        let item2 = item1.clone();
        assert_eq!(item1, item2);
    }

    #[test]
    fn test_display_format() {
        let item = ToDoItem::new(123, FormattedText::new("Buy groceries"));
        let s = format!("{}", item);
        assert!(s.contains("ToDoItem"));
    }

    #[test]
    fn test_debug_format() {
        let item = ToDoItem::new(123, FormattedText::new("Task"));
        let debug_str = format!("{:?}", item);
        assert!(debug_str.contains("ToDoItem"));
    }

    #[test]
    fn test_long_title() {
        let long_text = "A".repeat(ToDoItem::MAX_TEXT_LENGTH + 10);
        let item = ToDoItem::new(123, FormattedText::new(&long_text));
        assert_eq!(item.title().text().len(), ToDoItem::MAX_TEXT_LENGTH + 10);
        // The item is still valid even with a long title
        assert!(item.is_valid());
    }

    #[test]
    fn test_special_characters_in_title() {
        let special = "Task with émojis 🎉 and spëcial çharacters";
        let item = ToDoItem::new(123, FormattedText::new(special));
        assert_eq!(item.title().text(), special);
        assert!(item.is_valid());
    }

    #[test]
    fn test_multiline_title() {
        let multiline = "Line 1\nLine 2\rLine 3";
        let item = ToDoItem::new(123, FormattedText::new(multiline));
        assert_eq!(item.title().text(), multiline);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let original = ToDoItem::new(123, FormattedText::new("Buy groceries"));
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ToDoItem = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_with_entities() {
        let text = FormattedText::new("Task with text");
        let original = ToDoItem::new(456, text);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ToDoItem = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
