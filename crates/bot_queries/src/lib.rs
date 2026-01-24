// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Bot Queries
//!
//! Bot query types for Telegram.
//!
//! ## Overview
//!
//! Types related to bot queries and interactions.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_types::{DialogId, UserId};
use serde::{Deserialize, Serialize};

/// Bot query state.
///
/// Represents the state of a bot query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BotQueryState {
    /// Query is pending.
    Pending,
    /// Query succeeded.
    Succeeded,
    /// Query failed.
    Failed,
}

impl Default for BotQueryState {
    fn default() -> Self {
        Self::Pending
    }
}

/// Bot query information.
///
/// Contains information about a bot query.
/// Based on TDLib bot query handling patterns.
///
/// # Example
///
/// ```rust
/// use rustgram_bot_queries::{BotQuery, BotQueryState};
/// use rustgram_types::{UserId, DialogId};
///
/// let user_id = UserId::new(123).unwrap();
/// let dialog_id = DialogId::from_user(user_id);
/// let query = BotQuery::new(1, dialog_id, "/start".to_string());
/// assert_eq!(query.query_id(), 1);
/// assert_eq!(query.text(), "/start");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotQuery {
    /// Query ID.
    query_id: i64,
    /// Dialog ID where the query was made.
    dialog_id: DialogId,
    /// Query text.
    text: String,
    /// Query state.
    state: BotQueryState,
}

impl Default for BotQuery {
    fn default() -> Self {
        Self {
            query_id: 0,
            dialog_id: DialogId::default(),
            text: String::new(),
            state: BotQueryState::default(),
        }
    }
}

impl BotQuery {
    /// Creates a new bot query.
    ///
    /// # Arguments
    ///
    /// * `query_id` - Query ID
    /// * `dialog_id` - Dialog ID
    /// * `text` - Query text
    #[must_use]
    pub const fn new(query_id: i64, dialog_id: DialogId, text: String) -> Self {
        Self {
            query_id,
            dialog_id,
            text,
            state: BotQueryState::Pending,
        }
    }

    /// Returns the query ID.
    #[must_use]
    pub const fn query_id(&self) -> i64 {
        self.query_id
    }

    /// Returns the dialog ID.
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the query text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the query state.
    #[must_use]
    pub const fn state(&self) -> BotQueryState {
        self.state
    }

    /// Sets the query state.
    pub fn set_state(&mut self, state: BotQueryState) {
        self.state = state;
    }

    /// Checks if the query is pending.
    #[must_use]
    pub const fn is_pending(&self) -> bool {
        matches!(self.state, BotQueryState::Pending)
    }

    /// Checks if the query succeeded.
    #[must_use]
    pub const fn is_succeeded(&self) -> bool {
        matches!(self.state, BotQueryState::Succeeded)
    }

    /// Checks if the query failed.
    #[must_use]
    pub const fn is_failed(&self) -> bool {
        matches!(self.state, BotQueryState::Failed)
    }
}

/// Bot command query.
///
/// Represents a bot command query (e.g., /start, /help).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotCommandQuery {
    /// Command (e.g., "start", "help").
    command: String,
    /// Arguments.
    args: String,
    /// User who sent the command.
    user_id: UserId,
}

impl Default for BotCommandQuery {
    fn default() -> Self {
        Self {
            command: String::new(),
            args: String::new(),
            user_id: UserId::default(),
        }
    }
}

impl BotCommandQuery {
    /// Creates a new bot command query.
    #[must_use]
    pub const fn new(command: String, args: String, user_id: UserId) -> Self {
        Self {
            command,
            args,
            user_id,
        }
    }

    /// Returns the command.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Returns the arguments.
    #[must_use]
    pub fn args(&self) -> &str {
        &self.args
    }

    /// Returns the user ID.
    #[must_use]
    pub const fn user_id(&self) -> UserId {
        self.user_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_query_new() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let query = BotQuery::new(1, dialog_id, "/start".to_string());
        assert_eq!(query.query_id(), 1);
        assert_eq!(query.text(), "/start");
        assert!(query.is_pending());
    }

    #[test]
    fn test_bot_query_state() {
        let mut query = BotQuery::new(1, DialogId::default(), "test".to_string());
        assert!(query.is_pending());

        query.set_state(BotQueryState::Succeeded);
        assert!(query.is_succeeded());

        query.set_state(BotQueryState::Failed);
        assert!(query.is_failed());
    }

    #[test]
    fn test_bot_query_default() {
        let query = BotQuery::default();
        assert_eq!(query.query_id(), 0);
        assert!(query.text().is_empty());
    }

    #[test]
    fn test_bot_command_query_new() {
        let user_id = UserId::new(123).unwrap();
        let cmd = BotCommandQuery::new("start".to_string(), "arg".to_string(), user_id);
        assert_eq!(cmd.command(), "start");
        assert_eq!(cmd.args(), "arg");
        assert_eq!(cmd.user_id(), user_id);
    }

    #[test]
    fn test_bot_query_equality() {
        let user_id = UserId::new(123).unwrap();
        let dialog_id = DialogId::from_user(user_id);
        let q1 = BotQuery::new(1, dialog_id, "test".to_string());
        let q2 = BotQuery::new(1, dialog_id, "test".to_string());
        let q3 = BotQuery::new(2, dialog_id, "test".to_string());
        assert_eq!(q1, q2);
        assert_ne!(q1, q3);
    }
}
