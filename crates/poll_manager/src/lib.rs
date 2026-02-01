// Copyright (c) 2025 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Poll Manager
//!
//! Manages polls and voting for Telegram.
//!
//! ## Overview
//!
//! The `PollManager` handles poll operations including:
//! - Creating new polls
//! - Registering/unregistering polls with messages
//! - Setting poll answers (voting)
//! - Getting poll voters
//! - Stopping polls
//!
//! ## Architecture
//!
//! This is a simplified version of TDLib's `PollManager` that focuses
//! on the core data structures and state management. The full TDLib
//! implementation includes network queries, database persistence, and
//! real-time updates.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_poll_manager::{PollManager, PollId};
//! use rustgram_formatted_text::FormattedText;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let manager = PollManager::new();
//!
//! let question = FormattedText::new("What's your favorite color?");
//! let options = vec![
//!     FormattedText::new("Red"),
//!     FormattedText::new("Blue"),
//!     FormattedText::new("Green"),
//! ];
//!
//! let poll_id = manager.create_poll(question, options, false, false, false, -1, FormattedText::new(""), 0, 0, false).await;
//! assert!(manager.has_poll(poll_id).await);
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_dialog_id::DialogId;
use rustgram_formatted_text::FormattedText;
use rustgram_types::UserId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

pub use error::{Error, Result};
pub use poll_id::PollId;

mod error;
mod poll_id;

/// Maximum number of poll voters to retrieve per request.
pub const MAX_GET_POLL_VOTERS: i32 = 50;

/// Delay before unloading a poll from memory (in seconds).
pub const UNLOAD_POLL_DELAY: i32 = 600;

/// A single poll option.
///
/// Based on TDLib's `PollOption`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PollOption {
    /// The option text.
    text: FormattedText,
    /// Option data for serialization.
    data: String,
    /// Number of voters for this option.
    voter_count: i32,
    /// Whether the current user voted for this option.
    is_chosen: bool,
}

impl PollOption {
    /// Creates a new poll option.
    #[must_use]
    pub fn new(text: FormattedText) -> Self {
        Self {
            text,
            data: String::new(),
            voter_count: 0,
            is_chosen: false,
        }
    }

    /// Returns the option text.
    #[must_use]
    pub fn text(&self) -> &FormattedText {
        &self.text
    }

    /// Returns the option data.
    #[must_use]
    pub fn data(&self) -> &str {
        &self.data
    }

    /// Returns the voter count.
    #[must_use]
    pub fn voter_count(&self) -> i32 {
        self.voter_count
    }

    /// Returns whether this option was chosen.
    #[must_use]
    pub fn is_chosen(&self) -> bool {
        self.is_chosen
    }
}

/// A poll in Telegram.
///
/// Based on TDLib's `Poll` structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Poll {
    /// The poll question.
    question: FormattedText,
    /// The poll options.
    options: Vec<PollOption>,
    /// Recent voter dialog IDs.
    recent_voter_dialog_ids: Vec<DialogId>,
    /// Explanation for quiz polls.
    explanation: FormattedText,
    /// Total voter count.
    total_voter_count: i32,
    /// Correct option ID for quiz polls.
    correct_option_id: i32,
    /// Open period in seconds.
    open_period: i32,
    /// Close date timestamp.
    close_date: i32,
    /// Whether the poll is anonymous.
    is_anonymous: bool,
    /// Whether multiple answers are allowed.
    allow_multiple_answers: bool,
    /// Whether this is a quiz poll.
    is_quiz: bool,
    /// Whether the poll is closed.
    is_closed: bool,
}

impl Poll {
    /// Creates a new poll.
    #[must_use]
    pub fn new(question: FormattedText, options: Vec<PollOption>) -> Self {
        Self {
            question,
            options,
            recent_voter_dialog_ids: Vec::new(),
            explanation: FormattedText::new(""),
            total_voter_count: 0,
            correct_option_id: -1,
            open_period: 0,
            close_date: 0,
            is_anonymous: true,
            allow_multiple_answers: false,
            is_quiz: false,
            is_closed: false,
        }
    }

    /// Returns the question.
    #[must_use]
    pub fn question(&self) -> &FormattedText {
        &self.question
    }

    /// Returns the options.
    #[must_use]
    pub fn options(&self) -> &[PollOption] {
        &self.options
    }

    /// Returns the total voter count.
    #[must_use]
    pub fn total_voter_count(&self) -> i32 {
        self.total_voter_count
    }

    /// Returns whether the poll is closed.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.is_closed
    }

    /// Returns whether the poll is anonymous.
    #[must_use]
    pub fn is_anonymous(&self) -> bool {
        self.is_anonymous
    }

    /// Returns the correct option ID for quiz polls.
    #[must_use]
    pub fn correct_option_id(&self) -> i32 {
        self.correct_option_id
    }

    /// Sets the poll as closed.
    pub fn set_closed(&mut self) {
        self.is_closed = true;
    }

    /// Increments the voter count.
    pub fn increment_voter_count(&mut self) {
        self.total_voter_count += 1;
    }
}

/// Poll manager.
///
/// Manages polls and voting operations.
#[derive(Clone)]
pub struct PollManager {
    inner: Arc<RwLock<Inner>>,
}

struct Inner {
    polls: HashMap<PollId, Poll>,
    local_poll_id_counter: i64,
    server_poll_messages: HashMap<PollId, Vec<MessageFullId>>,
    other_poll_messages: HashMap<PollId, Vec<MessageFullId>>,
    reply_poll_counts: HashMap<PollId, i32>,
    loaded_from_database: HashMap<PollId, bool>,
}

impl fmt::Debug for PollManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PollManager")
            .field("poll_count", &self.try_poll_count())
            .finish()
    }
}

impl Default for PollManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Full identifier for a message containing a poll.
///
/// TODO: This is a stub - should use the proper MessageFullId from rustgram-message_full_id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageFullId {
    /// Dialog ID.
    dialog_id: DialogId,
    /// Server message ID.
    message_id: i32,
}

impl MessageFullId {
    /// Creates a new message full ID.
    #[must_use]
    pub const fn new(dialog_id: DialogId, message_id: i32) -> Self {
        Self {
            dialog_id,
            message_id,
        }
    }
}

impl PollManager {
    /// Creates a new poll manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(Inner {
                polls: HashMap::new(),
                local_poll_id_counter: 0,
                server_poll_messages: HashMap::new(),
                other_poll_messages: HashMap::new(),
                reply_poll_counts: HashMap::new(),
                loaded_from_database: HashMap::new(),
            })),
        }
    }

    /// Checks if a poll ID is a local poll ID.
    ///
    /// Local poll IDs are negative, server poll IDs are positive.
    #[must_use]
    pub fn is_local_poll_id(poll_id: PollId) -> bool {
        poll_id.get() < 0
    }

    /// Returns the number of polls (synchronous).
    #[must_use]
    pub fn try_poll_count(&self) -> Option<usize> {
        self.inner.try_read().ok().map(|inner| inner.polls.len())
    }

    /// Creates a new poll.
    ///
    /// # Arguments
    ///
    /// * `question` - The poll question
    /// * `options` - The poll options
    /// * `is_anonymous` - Whether the poll is anonymous
    /// * `allow_multiple_answers` - Whether multiple answers are allowed
    /// * `is_quiz` - Whether this is a quiz poll
    /// * `correct_option_id` - The correct option ID for quizzes
    /// * `explanation` - Explanation for quiz answers
    /// * `open_period` - Open period in seconds
    /// * `close_date` - Close date timestamp
    /// * `is_closed` - Whether the poll is initially closed
    ///
    /// # Returns
    ///
    /// The ID of the created poll.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_poll_manager::PollManager;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let manager = PollManager::new();
    ///
    /// let question = FormattedText::new("Question?");
    /// let options = vec![
    ///     FormattedText::new("Option 1"),
    ///     FormattedText::new("Option 2"),
    /// ];
    ///
    /// let poll_id = manager.create_poll(
    ///     question, options, true, false, false, -1,
    ///     FormattedText::new(""), 0, 0, false
    /// ).await;
    ///
    /// assert!(manager.has_poll(poll_id).await);
    /// # }
    /// ```
    pub async fn create_poll(
        &self,
        question: FormattedText,
        options: Vec<FormattedText>,
        is_anonymous: bool,
        allow_multiple_answers: bool,
        is_quiz: bool,
        correct_option_id: i32,
        explanation: FormattedText,
        open_period: i32,
        close_date: i32,
        is_closed: bool,
    ) -> PollId {
        let mut inner = self.inner.write().await;

        inner.local_poll_id_counter -= 1;
        let poll_id = PollId::new(inner.local_poll_id_counter);

        let poll_options = options.into_iter().map(PollOption::new).collect();

        let mut poll = Poll::new(question, poll_options);
        poll.is_anonymous = is_anonymous;
        poll.allow_multiple_answers = allow_multiple_answers;
        poll.is_quiz = is_quiz;
        poll.correct_option_id = correct_option_id;
        poll.explanation = explanation;
        poll.open_period = open_period;
        poll.close_date = close_date;
        poll.is_closed = is_closed;

        inner.polls.insert(poll_id, poll);

        info!("Created poll: {:?}", poll_id);
        debug!("Poll is_anonymous={}, is_quiz={}, is_closed={}",
            is_anonymous, is_quiz, is_closed);

        poll_id
    }

    /// Registers a poll with a message.
    ///
    /// # Arguments
    ///
    /// * `poll_id` - The poll ID
    /// * `message_full_id` - The message full ID
    /// * `source` - Source description for logging
    pub async fn register_poll(&self, poll_id: PollId, message_full_id: MessageFullId, source: &str) {
        let mut inner = self.inner.write().await;

        let map = if Self::is_local_poll_id(poll_id) {
            &mut inner.other_poll_messages
        } else {
            &mut inner.server_poll_messages
        };

        map.entry(poll_id)
            .or_insert_with(Vec::new)
            .push(message_full_id);

        debug!("Registered poll {:?} with message from: {}", poll_id, source);
    }

    /// Unregisters a poll from a message.
    pub async fn unregister_poll(&self, poll_id: PollId, message_full_id: MessageFullId, source: &str) {
        let mut inner = self.inner.write().await;

        let map = if Self::is_local_poll_id(poll_id) {
            &mut inner.other_poll_messages
        } else {
            &mut inner.server_poll_messages
        };

        if let Some(messages) = map.get_mut(&poll_id) {
            messages.retain(|id| id != &message_full_id);
        }

        debug!("Unregistered poll {:?} from message from: {}", poll_id, source);
    }

    /// Registers a reply poll.
    pub async fn register_reply_poll(&self, poll_id: PollId) {
        let mut inner = self.inner.write().await;
        *inner.reply_poll_counts.entry(poll_id).or_insert(0) += 1;
    }

    /// Unregisters a reply poll.
    pub async fn unregister_reply_poll(&self, poll_id: PollId) {
        let mut inner = self.inner.write().await;
        if let Some(count) = inner.reply_poll_counts.get_mut(&poll_id) {
            if *count > 0 {
                *count -= 1;
            }
        }
    }

    /// Checks if a poll exists.
    #[must_use]
    pub async fn has_poll(&self, poll_id: PollId) -> bool {
        let inner = self.inner.read().await;
        inner.polls.contains_key(&poll_id)
    }

    /// Gets the poll.
    #[must_use]
    pub async fn get_poll(&self, poll_id: PollId) -> Option<Poll> {
        let inner = self.inner.read().await;
        inner.polls.get(&poll_id).cloned()
    }

    /// Checks if a poll is closed.
    #[must_use]
    pub async fn get_poll_is_closed(&self, poll_id: PollId) -> bool {
        let inner = self.inner.read().await;
        inner.polls.get(&poll_id).map(|p| p.is_closed).unwrap_or(false)
    }

    /// Checks if a poll is anonymous.
    #[must_use]
    pub async fn get_poll_is_anonymous(&self, poll_id: PollId) -> bool {
        let inner = self.inner.read().await;
        inner.polls.get(&poll_id).map(|p| p.is_anonymous).unwrap_or(true)
    }

    /// Gets the poll search text.
    #[must_use]
    pub async fn get_poll_search_text(&self, poll_id: PollId) -> String {
        let inner = self.inner.read().await;
        inner
            .polls
            .get(&poll_id)
            .map(|p| p.question().text().to_string())
            .unwrap_or_default()
    }

    /// Sets a poll answer (votes).
    ///
    /// # Arguments
    ///
    /// * `poll_id` - The poll ID
    /// * `message_full_id` - The message full ID
    /// * `option_ids` - The option IDs to vote for
    ///
    /// # Errors
    ///
    /// Returns an error if the poll doesn't exist or is closed.
    pub async fn set_poll_answer(
        &self,
        poll_id: PollId,
        message_full_id: MessageFullId,
        option_ids: Vec<i32>,
    ) -> Result<()> {
        let mut inner = self.inner.write().await;

        let poll = inner
            .polls
            .get_mut(&poll_id)
            .ok_or_else(|| Error::PollNotFound { poll_id })?;

        if poll.is_closed {
            return Err(Error::PollClosed);
        }

        info!("Setting poll answer for {:?}: {:?}", poll_id, option_ids);
        debug!("Message: {:?}", message_full_id);

        // Update voter counts
        for &option_id in &option_ids {
            if let Some(option) = poll.options.get_mut(option_id as usize) {
                option.voter_count += 1;
                option.is_chosen = true;
            }
        }

        poll.increment_voter_count();

        Ok(())
    }

    /// Gets poll voters for a specific option.
    ///
    /// # Arguments
    ///
    /// * `poll_id` - The poll ID
    /// * `option_id` - The option ID
    /// * `offset` - Offset for pagination
    /// * `limit` - Maximum number of voters to return
    ///
    /// # Returns
    ///
    /// A vector of user IDs who voted for this option.
    ///
    /// # Errors
    ///
    /// Returns an error if the poll doesn't exist.
    pub async fn get_poll_voters(
        &self,
        poll_id: PollId,
        option_id: i32,
        offset: String,
        limit: i32,
    ) -> Result<Vec<UserId>> {
        let inner = self.inner.read().await;

        let _poll = inner
            .polls
            .get(&poll_id)
            .ok_or_else(|| Error::PollNotFound { poll_id })?;

        // In the full implementation, this would query the database
        // For now, return an empty vector
        debug!(
            "Getting poll voters for {:?}, option {}, offset={}, limit={}",
            poll_id, option_id, offset, limit
        );

        Ok(Vec::new())
    }

    /// Stops a poll.
    ///
    /// # Arguments
    ///
    /// * `poll_id` - The poll ID
    /// * `message_full_id` - The message full ID
    ///
    /// # Errors
    ///
    /// Returns an error if the poll doesn't exist.
    pub async fn stop_poll(
        &self,
        poll_id: PollId,
        message_full_id: MessageFullId,
    ) -> Result<()> {
        let mut inner = self.inner.write().await;

        let poll = inner
            .polls
            .get_mut(&poll_id)
            .ok_or_else(|| Error::PollNotFound { poll_id })?;

        info!("Stopping poll {:?}", poll_id);
        debug!("Message: {:?}", message_full_id);

        poll.set_closed();

        Ok(())
    }

    /// Stops a local poll.
    pub async fn stop_local_poll(&self, poll_id: PollId) {
        let mut inner = self.inner.write().await;

        if let Some(poll) = inner.polls.get_mut(&poll_id) {
            poll.set_closed();
            debug!("Stopped local poll {:?}", poll_id);
        }
    }

    /// Duplicates a poll.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog ID
    /// * `poll_id` - The poll ID to duplicate
    ///
    /// # Returns
    ///
    /// The new poll ID.
    #[must_use]
    pub async fn dup_poll(&self, dialog_id: DialogId, poll_id: PollId) -> PollId {
        let mut inner = self.inner.write().await;

        let poll = inner.polls.get(&poll_id);

        if let Some(original) = poll {
            inner.local_poll_id_counter -= 1;
            let new_id = PollId::new(inner.local_poll_id_counter);

            let duplicated = original.clone();
            inner.polls.insert(new_id, duplicated);

            debug!("Duplicated poll {:?} to {:?} in dialog {:?}", poll_id, new_id, dialog_id);

            new_id
        } else {
            poll_id
        }
    }

    /// Calculates vote percentages.
    ///
    /// # Arguments
    ///
    /// * `voter_counts` - Vector of voter counts per option
    /// * `total_voter_count` - Total voter count
    ///
    /// # Returns
    ///
    /// Vector of percentages (0-100).
    #[must_use]
    pub fn get_vote_percentage(voter_counts: &[i32], total_voter_count: i32) -> Vec<i32> {
        if total_voter_count == 0 {
            return vec![0; voter_counts.len()];
        }

        voter_counts
            .iter()
            .map(|&count| (count * 100 / total_voter_count))
            .collect()
    }

    /// Returns the reply poll count.
    pub async fn get_reply_poll_count(&self, poll_id: PollId) -> i32 {
        let inner = self.inner.read().await;
        inner.reply_poll_counts.get(&poll_id).copied().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === PollOption Tests ===

    #[test]
    fn test_poll_option_new() {
        let text = FormattedText::new("Option");
        let option = PollOption::new(text.clone());

        assert_eq!(option.text(), &text);
        assert_eq!(option.data(), "");
        assert_eq!(option.voter_count(), 0);
        assert!(!option.is_chosen());
    }

    #[test]
    fn test_poll_option_clone() {
        let text = FormattedText::new("Option");
        let option1 = PollOption::new(text);
        let option2 = option1.clone();

        assert_eq!(option1, option2);
    }

    // === Poll Tests ===

    #[test]
    fn test_poll_new() {
        let question = FormattedText::new("Question?");
        let options = vec![
            PollOption::new(FormattedText::new("A")),
            PollOption::new(FormattedText::new("B")),
        ];

        let poll = Poll::new(question.clone(), options.clone());

        assert_eq!(poll.question(), &question);
        assert_eq!(poll.options().len(), 2);
        assert_eq!(poll.total_voter_count(), 0);
        assert!(!poll.is_closed());
        assert!(poll.is_anonymous());
    }

    #[test]
    fn test_poll_set_closed() {
        let question = FormattedText::new("Question?");
        let options = vec![PollOption::new(FormattedText::new("A"))];

        let mut poll = Poll::new(question, options);
        assert!(!poll.is_closed());

        poll.set_closed();
        assert!(poll.is_closed());
    }

    #[test]
    fn test_poll_increment_voter_count() {
        let question = FormattedText::new("Question?");
        let options = vec![PollOption::new(FormattedText::new("A"))];

        let mut poll = Poll::new(question, options);
        assert_eq!(poll.total_voter_count(), 0);

        poll.increment_voter_count();
        assert_eq!(poll.total_voter_count(), 1);

        poll.increment_voter_count();
        assert_eq!(poll.total_voter_count(), 2);
    }

    // === PollId Tests ===

    #[test]
    fn test_poll_id_new() {
        let id = PollId::new(123);
        assert_eq!(id.get(), 123);
    }

    #[test]
    fn test_poll_id_from_i64() {
        let id = PollId::from(456);
        assert_eq!(id.get(), 456);
    }

    #[test]
    fn test_poll_id_to_i64() {
        let id = PollId::new(789);
        let value: i64 = id.into();
        assert_eq!(value, 789);
    }

    #[test]
    fn test_poll_id_local() {
        let local = PollId::new(-1);
        let server = PollId::new(1);

        assert!(PollManager::is_local_poll_id(local));
        assert!(!PollManager::is_local_poll_id(server));
    }

    // === MessageFullId Tests ===

    #[test]
    fn test_message_full_id_new() {
        let dialog_id = DialogId::new(123);
        let full_id = MessageFullId::new(dialog_id, 456);

        assert_eq!(full_id.dialog_id, dialog_id);
        assert_eq!(full_id.message_id, 456);
    }

    // === PollManager Tests ===

    #[tokio::test]
    async fn test_manager_new() {
        let manager = PollManager::new();
        assert_eq!(manager.try_poll_count(), Some(0));
    }

    #[tokio::test]
    async fn test_manager_default() {
        let manager = PollManager::default();
        assert_eq!(manager.try_poll_count(), Some(0));
    }

    #[tokio::test]
    async fn test_create_poll() {
        let manager = PollManager::new();

        let question = FormattedText::new("Question?");
        let options = vec![
            FormattedText::new("A"),
            FormattedText::new("B"),
            FormattedText::new("C"),
        ];

        let poll_id = manager.create_poll(
            question.clone(),
            options.clone(),
            true,
            false,
            false,
            -1,
            FormattedText::new(""),
            0,
            0,
            false,
        )
        .await;

        assert!(manager.has_poll(poll_id).await);

        let poll = manager.get_poll(poll_id).await.unwrap();
        assert_eq!(poll.question(), &question);
        assert_eq!(poll.options().len(), 3);
    }

    #[tokio::test]
    async fn test_create_poll_multiple() {
        let manager = PollManager::new();

        let poll_id1 = manager
            .create_poll(
                FormattedText::new("Q1"),
                vec![FormattedText::new("A")],
                false,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        let poll_id2 = manager
            .create_poll(
                FormattedText::new("Q2"),
                vec![FormattedText::new("B")],
                false,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        assert_ne!(poll_id1, poll_id2);
        assert!(manager.has_poll(poll_id1).await);
        assert!(manager.has_poll(poll_id2).await);
    }

    #[tokio::test]
    async fn test_register_poll() {
        let manager = PollManager::new();

        let poll_id = manager
            .create_poll(
                FormattedText::new("Q"),
                vec![FormattedText::new("A")],
                false,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        let message_id = MessageFullId::new(DialogId::new(123), 456);
        manager
            .register_poll(poll_id, message_id, "test")
            .await;

        // Registration succeeds - the poll is tracked internally
        assert!(manager.has_poll(poll_id).await);
    }

    #[tokio::test]
    async fn test_unregister_poll() {
        let manager = PollManager::new();

        let poll_id = manager
            .create_poll(
                FormattedText::new("Q"),
                vec![FormattedText::new("A")],
                false,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        let message_id = MessageFullId::new(DialogId::new(123), 456);
        manager
            .register_poll(poll_id, message_id, "test")
            .await;
        manager
            .unregister_poll(poll_id, message_id, "test")
            .await;

        // Poll still exists, just unregistered from message
        assert!(manager.has_poll(poll_id).await);
    }

    #[tokio::test]
    async fn test_register_reply_poll() {
        let manager = PollManager::new();

        let poll_id = manager
            .create_poll(
                FormattedText::new("Q"),
                vec![FormattedText::new("A")],
                false,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        assert_eq!(manager.get_reply_poll_count(poll_id).await, 0);

        manager.register_reply_poll(poll_id).await;
        assert_eq!(manager.get_reply_poll_count(poll_id).await, 1);

        manager.register_reply_poll(poll_id).await;
        assert_eq!(manager.get_reply_poll_count(poll_id).await, 2);
    }

    #[tokio::test]
    async fn test_unregister_reply_poll() {
        let manager = PollManager::new();

        let poll_id = manager
            .create_poll(
                FormattedText::new("Q"),
                vec![FormattedText::new("A")],
                false,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        manager.register_reply_poll(poll_id).await;
        manager.register_reply_poll(poll_id).await;

        manager.unregister_reply_poll(poll_id).await;
        assert_eq!(manager.get_reply_poll_count(poll_id).await, 1);

        manager.unregister_reply_poll(poll_id).await;
        assert_eq!(manager.get_reply_poll_count(poll_id).await, 0);

        // Unregistering when count is 0 keeps it at 0
        manager.unregister_reply_poll(poll_id).await;
        assert_eq!(manager.get_reply_poll_count(poll_id).await, 0);
    }

    #[tokio::test]
    async fn test_has_poll() {
        let manager = PollManager::new();

        let poll_id = manager
            .create_poll(
                FormattedText::new("Q"),
                vec![FormattedText::new("A")],
                false,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        assert!(manager.has_poll(poll_id).await);
        assert!(!manager.has_poll(PollId::new(999999)).await);
    }

    #[tokio::test]
    async fn test_get_poll_is_closed() {
        let manager = PollManager::new();

        let poll_id = manager
            .create_poll(
                FormattedText::new("Q"),
                vec![FormattedText::new("A")],
                false,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        assert!(!manager.get_poll_is_closed(poll_id).await);

        manager
            .stop_poll(poll_id, MessageFullId::new(DialogId::new(123), 456))
            .await
            .unwrap();

        assert!(manager.get_poll_is_closed(poll_id).await);
    }

    #[tokio::test]
    async fn test_get_poll_is_anonymous() {
        let manager = PollManager::new();

        let poll_id1 = manager
            .create_poll(
                FormattedText::new("Q"),
                vec![FormattedText::new("A")],
                true,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        let poll_id2 = manager
            .create_poll(
                FormattedText::new("Q"),
                vec![FormattedText::new("A")],
                false,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        assert!(manager.get_poll_is_anonymous(poll_id1).await);
        assert!(!manager.get_poll_is_anonymous(poll_id2).await);
    }

    #[tokio::test]
    async fn test_get_poll_search_text() {
        let manager = PollManager::new();

        let poll_id = manager
            .create_poll(
                FormattedText::new("What is your favorite color?"),
                vec![FormattedText::new("Red")],
                false,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        let text = manager.get_poll_search_text(poll_id).await;
        assert_eq!(text, "What is your favorite color?");
    }

    #[tokio::test]
    async fn test_set_poll_answer() {
        let manager = PollManager::new();

        let poll_id = manager
            .create_poll(
                FormattedText::new("Q"),
                vec![FormattedText::new("A"), FormattedText::new("B")],
                false,
                true,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        let message_id = MessageFullId::new(DialogId::new(123), 456);

        let result = manager
            .set_poll_answer(poll_id, message_id, vec![0, 1])
            .await;

        assert!(result.is_ok());

        let poll = manager.get_poll(poll_id).await.unwrap();
        assert_eq!(poll.options()[0].voter_count(), 1);
        assert_eq!(poll.options()[1].voter_count(), 1);
        assert_eq!(poll.total_voter_count(), 1);
    }

    #[tokio::test]
    async fn test_set_poll_answer_nonexistent_poll() {
        let manager = PollManager::new();
        let message_id = MessageFullId::new(DialogId::new(123), 456);

        let result = manager
            .set_poll_answer(PollId::new(999999), message_id, vec![0])
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::PollNotFound { poll_id }) => {
                assert_eq!(poll_id.get(), 999999);
            }
            _ => panic!("Expected PollNotFound error"),
        }
    }

    #[tokio::test]
    async fn test_get_poll_voters() {
        let manager = PollManager::new();

        let poll_id = manager
            .create_poll(
                FormattedText::new("Q"),
                vec![FormattedText::new("A")],
                false,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        let result = manager
            .get_poll_voters(poll_id, 0, String::new(), 10)
            .await;

        assert!(result.is_ok());
        let voters = result.unwrap();
        assert!(voters.is_empty());
    }

    #[tokio::test]
    async fn test_get_poll_voters_nonexistent_poll() {
        let manager = PollManager::new();

        let result = manager
            .get_poll_voters(PollId::new(999999), 0, String::new(), 10)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_stop_poll() {
        let manager = PollManager::new();

        let poll_id = manager
            .create_poll(
                FormattedText::new("Q"),
                vec![FormattedText::new("A")],
                false,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        let message_id = MessageFullId::new(DialogId::new(123), 456);

        let result = manager.stop_poll(poll_id, message_id).await;
        assert!(result.is_ok());

        let poll = manager.get_poll(poll_id).await.unwrap();
        assert!(poll.is_closed());
    }

    #[tokio::test]
    async fn test_stop_local_poll() {
        let manager = PollManager::new();

        let poll_id = manager
            .create_poll(
                FormattedText::new("Q"),
                vec![FormattedText::new("A")],
                false,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        manager.stop_local_poll(poll_id).await;

        let poll = manager.get_poll(poll_id).await.unwrap();
        assert!(poll.is_closed());
    }

    #[tokio::test]
    async fn test_dup_poll() {
        let manager = PollManager::new();

        let poll_id = manager
            .create_poll(
                FormattedText::new("Q"),
                vec![FormattedText::new("A"), FormattedText::new("B")],
                true,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        let dialog_id = DialogId::new(123);
        let new_id = manager.dup_poll(dialog_id, poll_id).await;

        assert_ne!(poll_id, new_id);
        assert!(manager.has_poll(new_id).await);

        let original = manager.get_poll(poll_id).await.unwrap();
        let duplicate = manager.get_poll(new_id).await.unwrap();

        assert_eq!(original.question(), duplicate.question());
        assert_eq!(original.options().len(), duplicate.options().len());
    }

    #[tokio::test]
    async fn test_get_vote_percentage() {
        let counts = vec![5, 10, 15];
        let total = 30;

        let percentages = PollManager::get_vote_percentage(&counts, total);

        assert_eq!(percentages, vec![16, 33, 50]); // 5/30*100=16.6->16, 10/30*100=33.3->33, 15/30*100=50
    }

    #[tokio::test]
    async fn test_get_vote_percentage_zero_total() {
        let counts = vec![0, 0, 0];
        let total = 0;

        let percentages = PollManager::get_vote_percentage(&counts, total);

        assert_eq!(percentages, vec![0, 0, 0]);
    }

    #[tokio::test]
    async fn test_get_vote_percentage_empty() {
        let counts: Vec<i32> = vec![];
        let total = 10;

        let percentages = PollManager::get_vote_percentage(&counts, total);

        assert!(percentages.is_empty());
    }

    #[tokio::test]
    async fn test_manager_clone() {
        let manager1 = PollManager::new();
        let manager2 = manager1.clone();

        let poll_id = manager1
            .create_poll(
                FormattedText::new("Q"),
                vec![FormattedText::new("A")],
                false,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        assert!(manager1.has_poll(poll_id).await);
        assert!(manager2.has_poll(poll_id).await);
    }

    // === Integration Tests ===

    #[tokio::test]
    async fn test_full_poll_lifecycle() {
        let manager = PollManager::new();

        // Create poll
        let poll_id = manager
            .create_poll(
                FormattedText::new("Best programming language?"),
                vec![
                    FormattedText::new("Rust"),
                    FormattedText::new("C++"),
                    FormattedText::new("Python"),
                ],
                true,
                false,
                false,
                -1,
                FormattedText::new(""),
                0,
                0,
                false,
            )
            .await;

        assert!(manager.has_poll(poll_id).await);
        assert!(!manager.get_poll_is_closed(poll_id).await);

        // Register with message
        let message_id = MessageFullId::new(DialogId::new(123), 456);
        manager
            .register_poll(poll_id, message_id, "test")
            .await;

        // Vote
        let result = manager
            .set_poll_answer(poll_id, message_id, vec![0])
            .await;
        assert!(result.is_ok());

        let poll = manager.get_poll(poll_id).await.unwrap();
        assert_eq!(poll.total_voter_count(), 1);

        // Stop poll
        manager.stop_poll(poll_id, message_id).await.unwrap();
        assert!(manager.get_poll_is_closed(poll_id).await);
    }
}
