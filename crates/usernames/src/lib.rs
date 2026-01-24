// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Usernames
//!
//! Username management for Telegram users.
//!
//! ## Overview
//!
//! Handles multiple usernames including active and editable ones.
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_usernames::Usernames;
//!
//! let usernames = Usernames::with_primary("primary".to_string());
//! assert_eq!(usernames.first_username(), "primary");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Collection of usernames for a user
///
/// Manages active, disabled, and editable usernames.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Usernames {
    /// Active usernames
    active_usernames: Vec<String>,
    /// Disabled usernames
    disabled_usernames: Vec<String>,
    /// Editable username position in active list
    editable_username_pos: Option<usize>,
    /// Whether the editable username is currently disabled
    is_editable_username_disabled: bool,
}

impl Default for Usernames {
    fn default() -> Self {
        Self::new()
    }
}

impl Usernames {
    /// Creates a new empty usernames collection
    #[must_use]
    pub fn new() -> Self {
        Self {
            active_usernames: Vec::new(),
            disabled_usernames: Vec::new(),
            editable_username_pos: None,
            is_editable_username_disabled: false,
        }
    }

    /// Creates usernames with a primary username
    #[must_use]
    pub fn with_primary(primary: String) -> Self {
        Self {
            active_usernames: vec![primary],
            disabled_usernames: Vec::new(),
            editable_username_pos: Some(0),
            is_editable_username_disabled: false,
        }
    }

    /// Returns the first (primary) username
    #[must_use]
    pub fn first_username(&self) -> &str {
        if self.active_usernames.is_empty() {
            ""
        } else {
            &self.active_usernames[0]
        }
    }

    /// Returns whether there is a first username
    #[must_use]
    pub fn has_first_username(&self) -> bool {
        !self.active_usernames.is_empty()
    }

    /// Returns the editable username
    #[must_use]
    pub fn editable_username(&self) -> Option<&str> {
        if let Some(pos) = self.editable_username_pos {
            if self.is_editable_username_disabled {
                self.disabled_usernames.get(pos).map(|s| s.as_str())
            } else {
                self.active_usernames.get(pos).map(|s| s.as_str())
            }
        } else {
            None
        }
    }

    /// Returns whether there is an editable username
    #[must_use]
    pub fn has_editable_username(&self) -> bool {
        self.editable_username_pos.is_some()
    }

    /// Returns all active usernames
    #[must_use]
    pub fn active_usernames(&self) -> &[String] {
        &self.active_usernames
    }

    /// Returns all disabled usernames
    #[must_use]
    pub fn disabled_usernames(&self) -> &[String] {
        &self.disabled_usernames
    }

    /// Adds an active username
    pub fn add_active(&mut self, username: String) {
        if !self.active_usernames.contains(&username) {
            self.active_usernames.push(username);
        }
    }

    /// Adds a disabled username
    pub fn add_disabled(&mut self, username: String) {
        if !self.disabled_usernames.contains(&username) {
            self.disabled_usernames.push(username);
        }
    }

    /// Removes a username
    pub fn remove(&mut self, username: &str) -> bool {
        let mut removed = false;
        self.active_usernames.retain(|u| {
            if u == username {
                removed = true;
                false
            } else {
                true
            }
        });
        self.disabled_usernames.retain(|u| u != username);
        removed
    }

    /// Returns true if the collection is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.active_usernames.is_empty()
            && self.disabled_usernames.is_empty()
            && self.editable_username_pos.is_none()
    }

    /// Returns the total count of usernames
    #[must_use]
    pub fn count(&self) -> usize {
        self.active_usernames.len() + self.disabled_usernames.len()
    }
}

impl fmt::Display for Usernames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Usernames(")?;
        let mut first = true;
        for name in &self.active_usernames {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "@{}", name)?;
            first = false;
        }
        if let Some(editable) = self.editable_username() {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "[editable: @{}]", editable)?;
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let usernames = Usernames::new();
        assert!(usernames.is_empty());
        assert_eq!(usernames.count(), 0);
        assert!(!usernames.has_first_username());
    }

    #[test]
    fn test_with_primary() {
        let usernames = Usernames::with_primary("testuser".to_string());
        assert_eq!(usernames.first_username(), "testuser");
        assert!(usernames.has_first_username());
        assert_eq!(usernames.editable_username(), Some("testuser"));
    }

    #[test]
    fn test_add_active() {
        let mut usernames = Usernames::new();
        usernames.add_active("user1".to_string());
        usernames.add_active("user2".to_string());
        assert_eq!(usernames.count(), 2);
        assert_eq!(usernames.active_usernames().len(), 2);
    }

    #[test]
    fn test_add_disabled() {
        let mut usernames = Usernames::new();
        usernames.add_disabled("olduser".to_string());
        assert_eq!(usernames.count(), 1);
        assert_eq!(usernames.disabled_usernames().len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut usernames = Usernames::new();
        usernames.add_active("user1".to_string());
        usernames.add_active("user2".to_string());
        assert!(usernames.remove("user1"));
        assert_eq!(usernames.count(), 1);
        assert!(!usernames.remove("notexist"));
    }

    #[test]
    fn test_no_duplicate_active() {
        let mut usernames = Usernames::new();
        usernames.add_active("same".to_string());
        usernames.add_active("same".to_string());
        assert_eq!(usernames.active_usernames().len(), 1);
    }

    #[test]
    fn test_display() {
        let usernames = Usernames::with_primary("test".to_string());
        assert!(format!("{}", usernames).contains("@test"));
    }
}
