// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # To-Do List
//!
//! To-do list functionality for Telegram.
//!
//! ## Overview
//!
//! Represents a checklist with items that can be completed.
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_todo_list::{ToDoList, ToDoItem};
//!
//! let mut list = ToDoList::new("My Checklist".to_string());
//! list.add_item(ToDoItem::new("Task 1".to_string()));
//! list.add_item(ToDoItem::new("Task 2".to_string()));
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// A single to-do item
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToDoItem {
    /// Item ID
    id: i32,
    /// Item title/text
    title: String,
    /// Whether the item is completed
    is_completed: bool,
}

impl ToDoItem {
    /// Creates a new to-do item
    #[must_use]
    pub fn new(title: String) -> Self {
        Self {
            id: 0,
            title,
            is_completed: false,
        }
    }

    /// Returns the item ID
    #[must_use]
    pub const fn id(&self) -> i32 {
        self.id
    }

    /// Returns the item title
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns whether the item is completed
    #[must_use]
    pub const fn is_completed(&self) -> bool {
        self.is_completed
    }

    /// Sets the completion status
    pub fn set_completed(&mut self, completed: bool) {
        self.is_completed = completed;
    }

    /// Toggles the completion status
    pub fn toggle(&mut self) {
        self.is_completed = !self.is_completed;
    }
}

impl fmt::Display for ToDoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.is_completed { "✓" } else { " " };
        write!(f, "[{}] {}", status, self.title)
    }
}

/// To-do list
///
/// Represents a checklist with items.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToDoList {
    /// List title
    title: String,
    /// List items
    items: Vec<ToDoItem>,
    /// Whether others can append items
    others_can_append: bool,
    /// Whether others can complete items
    others_can_complete: bool,
}

impl ToDoList {
    /// Creates a new to-do list
    #[must_use]
    pub fn new(title: String) -> Self {
        Self {
            title,
            items: Vec::new(),
            others_can_append: false,
            others_can_complete: false,
        }
    }

    /// Returns the list title
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Sets the list title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Returns the items
    #[must_use]
    pub fn items(&self) -> &[ToDoItem] {
        &self.items
    }

    /// Returns mutable reference to items
    #[must_use]
    pub fn items_mut(&mut self) -> &mut [ToDoItem] {
        &mut self.items
    }

    /// Adds an item to the list
    pub fn add_item(&mut self, mut item: ToDoItem) {
        item.id = self.items.len() as i32 + 1;
        self.items.push(item);
    }

    /// Removes an item by ID
    pub fn remove_item(&mut self, id: i32) -> bool {
        if let Some(pos) = self.items.iter().position(|item| item.id == id) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    /// Returns whether others can append
    #[must_use]
    pub const fn others_can_append(&self) -> bool {
        self.others_can_append
    }

    /// Sets whether others can append
    pub fn set_others_can_append(&mut self, value: bool) {
        self.others_can_append = value;
    }

    /// Returns whether others can complete
    #[must_use]
    pub const fn others_can_complete(&self) -> bool {
        self.others_can_complete
    }

    /// Sets whether others can complete
    pub fn set_others_can_complete(&mut self, value: bool) {
        self.others_can_complete = value;
    }

    /// Returns the number of items
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the list is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns the number of completed items
    #[must_use]
    pub fn completed_count(&self) -> usize {
        self.items.iter().filter(|item| item.is_completed()).count()
    }
}

impl fmt::Display for ToDoList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.title)?;
        for item in &self.items {
            writeln!(f, "  {}", item)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_item_new() {
        let item = ToDoItem::new("Test task".to_string());
        assert_eq!(item.title(), "Test task");
        assert!(!item.is_completed());
    }

    #[test]
    fn test_todo_item_toggle() {
        let mut item = ToDoItem::new("Task".to_string());
        assert!(!item.is_completed());
        item.toggle();
        assert!(item.is_completed());
        item.toggle();
        assert!(!item.is_completed());
    }

    #[test]
    fn test_todo_list_new() {
        let list = ToDoList::new("My List".to_string());
        assert_eq!(list.title(), "My List");
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_todo_list_add_item() {
        let mut list = ToDoList::new("List".to_string());
        list.add_item(ToDoItem::new("Task 1".to_string()));
        list.add_item(ToDoItem::new("Task 2".to_string()));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_todo_list_remove_item() {
        let mut list = ToDoList::new("List".to_string());
        list.add_item(ToDoItem::new("Task 1".to_string()));
        list.add_item(ToDoItem::new("Task 2".to_string()));
        assert!(list.remove_item(1));
        assert_eq!(list.len(), 1);
        assert!(!list.remove_item(999));
    }

    #[test]
    fn test_todo_list_completed_count() {
        let mut list = ToDoList::new("List".to_string());
        list.add_item(ToDoItem::new("Task 1".to_string()));
        list.add_item(ToDoItem::new("Task 2".to_string()));
        list.add_item(ToDoItem::new("Task 3".to_string()));
        assert_eq!(list.completed_count(), 0);

        list.items_mut()[0].set_completed(true);
        list.items_mut()[2].set_completed(true);
        assert_eq!(list.completed_count(), 2);
    }

    #[test]
    fn test_todo_list_permissions() {
        let mut list = ToDoList::new("List".to_string());
        assert!(!list.others_can_append());
        assert!(!list.others_can_complete());

        list.set_others_can_append(true);
        list.set_others_can_complete(true);
        assert!(list.others_can_append());
        assert!(list.others_can_complete());
    }

    #[test]
    fn test_todo_item_display() {
        let item = ToDoItem::new("Test".to_string());
        assert!(format!("{}", item).contains("Test"));

        let mut completed = ToDoItem::new("Done".to_string());
        completed.set_completed(true);
        assert!(format!("{}", completed).contains("✓"));
    }
}
