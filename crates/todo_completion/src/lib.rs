// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # To-Do Completion
//!
//! Completion status for todo items in Telegram.
//!
//! ## Overview
//!
//! This module provides the [`ToDoCompletion`] struct, which represents
//! the completion status of a todo item in Telegram. It includes
//! the todo ID, the dialog ID of who completed it, and the completion date.
//! It mirrors TDLib's `ToDoCompletion` struct.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_todo_completion::ToDoCompletion;
//!
//! // Create a todo completion
//! let completion = ToDoCompletion::new(123, 456789, 1704067200);
//! assert!(completion.is_valid());
//! assert_eq!(completion.id(), 123);
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Completion status for a todo item.
///
/// This type represents when and by whom a todo item was completed.
///
/// # Fields
///
/// - `id` - The todo item ID
/// - `completed_by_dialog_id` - The dialog ID that completed the todo
/// - `date` - The completion date (Unix timestamp)
///
/// # Example
///
/// ```rust
/// use rustgram_todo_completion::ToDoCompletion;
///
/// // Create a valid todo completion
/// let completion = ToDoCompletion::new(123, 456789, 1704067200);
/// assert!(completion.is_valid());
///
/// // Create an invalid one (zero dialog ID)
/// let invalid = ToDoCompletion::new(123, 0, 1704067200);
/// assert!(!invalid.is_valid());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ToDoCompletion {
    /// The todo item ID.
    id: i32,

    /// The dialog ID that completed the todo.
    completed_by_dialog_id: i64,

    /// The completion date (Unix timestamp).
    date: i32,
}

impl ToDoCompletion {
    /// Creates a new todo completion.
    ///
    /// # Arguments
    ///
    /// * `id` - The todo item ID
    /// * `completed_by_dialog_id` - The dialog ID that completed the todo
    /// * `date` - The completion date (Unix timestamp)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_completion::ToDoCompletion;
    ///
    /// let completion = ToDoCompletion::new(123, 456789, 1704067200);
    /// assert_eq!(completion.id(), 123);
    /// ```
    #[must_use]
    pub const fn new(id: i32, completed_by_dialog_id: i64, date: i32) -> Self {
        Self {
            id,
            completed_by_dialog_id,
            date,
        }
    }

    /// Returns the todo item ID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_completion::ToDoCompletion;
    ///
    /// let completion = ToDoCompletion::new(123, 456789, 1704067200);
    /// assert_eq!(completion.id(), 123);
    /// ```
    #[must_use]
    pub const fn id(&self) -> i32 {
        self.id
    }

    /// Returns the dialog ID that completed the todo.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_completion::ToDoCompletion;
    ///
    /// let completion = ToDoCompletion::new(123, 456789, 1704067200);
    /// assert_eq!(completion.completed_by_dialog_id(), 456789);
    /// ```
    #[must_use]
    pub const fn completed_by_dialog_id(&self) -> i64 {
        self.completed_by_dialog_id
    }

    /// Returns the completion date.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_completion::ToDoCompletion;
    ///
    /// let completion = ToDoCompletion::new(123, 456789, 1704067200);
    /// assert_eq!(completion.date(), 1704067200);
    /// ```
    #[must_use]
    pub const fn date(&self) -> i32 {
        self.date
    }

    /// Returns all values as a tuple.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_completion::ToDoCompletion;
    ///
    /// let completion = ToDoCompletion::new(123, 456789, 1704067200);
    /// assert_eq!(completion.get(), (123, 456789, 1704067200));
    /// ```
    #[must_use]
    pub const fn get(&self) -> (i32, i64, i32) {
        (self.id, self.completed_by_dialog_id, self.date)
    }

    /// Checks if this is a valid todo completion.
    ///
    /// A valid completion must have a non-zero dialog ID and a positive date.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_completion::ToDoCompletion;
    ///
    /// let valid = ToDoCompletion::new(123, 456789, 1704067200);
    /// assert!(valid.is_valid());
    ///
    /// let invalid_dialog = ToDoCompletion::new(123, 0, 1704067200);
    /// assert!(!invalid_dialog.is_valid());
    ///
    /// let invalid_date = ToDoCompletion::new(123, 456789, 0);
    /// assert!(!invalid_date.is_valid());
    /// ```
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.completed_by_dialog_id != 0 && self.date > 0
    }
}

impl Default for ToDoCompletion {
    /// Creates a default todo completion with zeros.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_completion::ToDoCompletion;
    ///
    /// let default = ToDoCompletion::default();
    /// assert_eq!(default.id(), 0);
    /// assert_eq!(default.completed_by_dialog_id(), 0);
    /// assert_eq!(default.date(), 0);
    /// assert!(!default.is_valid());
    /// ```
    fn default() -> Self {
        Self {
            id: 0,
            completed_by_dialog_id: 0,
            date: 0,
        }
    }
}

impl fmt::Display for ToDoCompletion {
    /// Formats the todo completion for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_completion::ToDoCompletion;
    ///
    /// let completion = ToDoCompletion::new(123, 456789, 1704067200);
    /// assert_eq!(
    ///     format!("{}", completion),
    ///     "ToDoCompletion(id: 123, by: 456789, date: 1704067200)"
    /// );
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ToDoCompletion(id: {}, by: {}, date: {})",
            self.id, self.completed_by_dialog_id, self.date
        )
    }
}

impl From<(i32, i64, i32)> for ToDoCompletion {
    /// Creates a todo completion from a tuple of (id, completed_by_dialog_id, date).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_todo_completion::ToDoCompletion;
    ///
    /// let completion: ToDoCompletion = (123, 456789, 1704067200).into();
    /// assert_eq!(completion.id(), 123);
    /// assert_eq!(completion.completed_by_dialog_id(), 456789);
    /// assert_eq!(completion.date(), 1704067200);
    /// ```
    fn from((id, completed_by_dialog_id, date): (i32, i64, i32)) -> Self {
        Self::new(id, completed_by_dialog_id, date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let completion = ToDoCompletion::new(123, 456789, 1704067200);
        assert_eq!(completion.id(), 123);
        assert_eq!(completion.completed_by_dialog_id(), 456789);
        assert_eq!(completion.date(), 1704067200);
    }

    #[test]
    fn test_id() {
        let completion = ToDoCompletion::new(999, 456789, 1704067200);
        assert_eq!(completion.id(), 999);
    }

    #[test]
    fn test_completed_by_dialog_id() {
        let completion = ToDoCompletion::new(123, 999999, 1704067200);
        assert_eq!(completion.completed_by_dialog_id(), 999999);
    }

    #[test]
    fn test_date() {
        let completion = ToDoCompletion::new(123, 456789, 999999);
        assert_eq!(completion.date(), 999999);
    }

    #[test]
    fn test_get() {
        let completion = ToDoCompletion::new(123, 456789, 1704067200);
        assert_eq!(completion.get(), (123, 456789, 1704067200));
    }

    #[test]
    fn test_is_valid_true() {
        let completion = ToDoCompletion::new(123, 456789, 1704067200);
        assert!(completion.is_valid());

        let completion2 = ToDoCompletion::new(0, 1, 1);
        assert!(completion2.is_valid());
    }

    #[test]
    fn test_is_valid_false_zero_dialog_id() {
        let completion = ToDoCompletion::new(123, 0, 1704067200);
        assert!(!completion.is_valid());
    }

    #[test]
    fn test_is_valid_false_zero_date() {
        let completion = ToDoCompletion::new(123, 456789, 0);
        assert!(!completion.is_valid());
    }

    #[test]
    fn test_is_valid_false_negative_date() {
        let completion = ToDoCompletion::new(123, 456789, -1);
        assert!(!completion.is_valid());
    }

    #[test]
    fn test_default() {
        let default = ToDoCompletion::default();
        assert_eq!(default.id(), 0);
        assert_eq!(default.completed_by_dialog_id(), 0);
        assert_eq!(default.date(), 0);
        assert!(!default.is_valid());
    }

    #[test]
    fn test_equality() {
        let completion1 = ToDoCompletion::new(123, 456789, 1704067200);
        let completion2 = ToDoCompletion::new(123, 456789, 1704067200);
        assert_eq!(completion1, completion2);
    }

    #[test]
    fn test_inequality() {
        let completion1 = ToDoCompletion::new(123, 456789, 1704067200);
        let completion2 = ToDoCompletion::new(999, 456789, 1704067200);
        assert_ne!(completion1, completion2);

        let completion3 = ToDoCompletion::new(123, 999, 1704067200);
        assert_ne!(completion1, completion3);

        let completion4 = ToDoCompletion::new(123, 456789, 999);
        assert_ne!(completion1, completion4);
    }

    #[test]
    fn test_copy_semantics() {
        let completion1 = ToDoCompletion::new(123, 456789, 1704067200);
        let completion2 = completion1;
        assert_eq!(completion1, completion2);
        assert_eq!(completion1.id(), 123);
    }

    #[test]
    fn test_clone_semantics() {
        let completion1 = ToDoCompletion::new(123, 456789, 1704067200);
        let completion2 = completion1.clone();
        assert_eq!(completion1, completion2);
    }

    #[test]
    fn test_display_format() {
        let completion = ToDoCompletion::new(123, 456789, 1704067200);
        assert_eq!(
            format!("{}", completion),
            "ToDoCompletion(id: 123, by: 456789, date: 1704067200)"
        );
    }

    #[test]
    fn test_from_tuple() {
        let completion: ToDoCompletion = (123, 456789, 1704067200).into();
        assert_eq!(completion.id(), 123);
        assert_eq!(completion.completed_by_dialog_id(), 456789);
        assert_eq!(completion.date(), 1704067200);
    }

    #[test]
    fn test_debug_format() {
        let completion = ToDoCompletion::new(123, 456789, 1704067200);
        let debug_str = format!("{:?}", completion);
        assert!(debug_str.contains("ToDoCompletion"));
        assert!(debug_str.contains("123"));
    }

    #[test]
    fn test_negative_dialog_id() {
        let completion = ToDoCompletion::new(123, -999, 1704067200);
        assert!(completion.is_valid());
        assert_eq!(completion.completed_by_dialog_id(), -999);
    }

    #[test]
    fn test_zero_id_valid() {
        let completion = ToDoCompletion::new(0, 456789, 1704067200);
        assert!(completion.is_valid());
        assert_eq!(completion.id(), 0);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let original = ToDoCompletion::new(123, 456789, 1704067200);

        // Test JSON serialization
        let json = serde_json::to_string(&original).unwrap();
        assert_eq!(
            json,
            r#"{"id":123,"completed_by_dialog_id":456789,"date":1704067200}"#
        );

        let deserialized: ToDoCompletion = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);

        // Test binary serialization
        let encoded = bincode::serialize(&original).unwrap();
        let decoded: ToDoCompletion = bincode::deserialize(&encoded).unwrap();
        assert_eq!(original, decoded);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_default() {
        let original = ToDoCompletion::default();

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ToDoCompletion = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_negative() {
        let original = ToDoCompletion::new(123, -999, 1704067200);

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: ToDoCompletion = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
