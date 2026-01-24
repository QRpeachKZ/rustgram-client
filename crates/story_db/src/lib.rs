// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Story Database
//!
//! Story database interface for Telegram.
//!
//! ## Overview
//!
//! This module provides stubbed database interfaces for story storage.
//! Full implementation will be added when the database layer is complete.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_story_db::StoryDb;
//!
//! // This is a stub implementation
//! // Full database functionality will be implemented later
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use rustgram_dialog_id::DialogId;
use rustgram_story_full_id::StoryFullId;

/// Buffer slice stub.
///
/// TODO: Replace with actual buffer implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferSlice {
    /// Inner data stub.
    #[cfg_attr(test, allow(dead_code))]
    inner: Vec<u8>,
}

impl BufferSlice {
    /// Creates a new buffer slice from bytes.
    #[must_use]
    pub fn new(data: Vec<u8>) -> Self {
        Self { inner: data }
    }
}

/// Story database entry.
///
/// Represents a story stored in the database.
///
/// Based on TDLib's `StoryDbStory` struct.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoryDbStory {
    /// The full story ID.
    story_full_id: StoryFullId,

    /// The serialized story data.
    data: BufferSlice,
}

impl StoryDbStory {
    /// Creates a new story database entry.
    ///
    /// # Arguments
    ///
    /// * `story_full_id` - The full story ID
    /// * `data` - The serialized story data
    #[must_use]
    pub fn new(story_full_id: StoryFullId, data: BufferSlice) -> Self {
        Self {
            story_full_id,
            data,
        }
    }

    /// Returns the full story ID.
    #[must_use]
    pub const fn story_full_id(&self) -> &StoryFullId {
        &self.story_full_id
    }

    /// Returns the serialized data.
    #[must_use]
    pub const fn data(&self) -> &BufferSlice {
        &self.data
    }
}

/// Active story list result.
///
/// Result of querying active stories from the database.
///
/// Based on TDLib's `StoryDbGetActiveStoryListResult` struct.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ActiveStoryListResult {
    /// The active stories.
    active_stories: Vec<(DialogId, BufferSlice)>,

    /// The next order value for pagination.
    next_order: i64,

    /// The next dialog ID for pagination.
    next_dialog_id: DialogId,
}

impl ActiveStoryListResult {
    /// Creates a new active story list result.
    ///
    /// # Arguments
    ///
    /// * `active_stories` - The active stories
    /// * `next_order` - The next order value
    /// * `next_dialog_id` - The next dialog ID
    #[must_use]
    pub fn new(
        active_stories: Vec<(DialogId, BufferSlice)>,
        next_order: i64,
        next_dialog_id: DialogId,
    ) -> Self {
        Self {
            active_stories,
            next_order,
            next_dialog_id,
        }
    }

    /// Returns the active stories.
    #[must_use]
    pub fn active_stories(&self) -> &[(DialogId, BufferSlice)] {
        &self.active_stories
    }

    /// Returns the next order value.
    #[must_use]
    pub const fn next_order(&self) -> i64 {
        self.next_order
    }

    /// Returns the next dialog ID.
    #[must_use]
    pub const fn next_dialog_id(&self) -> DialogId {
        self.next_dialog_id
    }
}

/// Story database interface.
///
/// Stubbed interface for story database operations.
///
/// Based on TDLib's `StoryDbSyncInterface` class.
///
/// # TDLib Alignment
///
/// Aligns with TDLib's `StoryDbSyncInterface` class in `StoryDb.h`.
///
/// # TODO
///
/// Full implementation when database layer is available.
pub trait StoryDb {
    /// Adds a story to the database.
    fn add_story(
        &mut self,
        story_full_id: StoryFullId,
        expires_at: i32,
        notification_id: i32,
        data: BufferSlice,
    ) -> Result<(), StoryDbError>;

    /// Deletes a story from the database.
    fn delete_story(&mut self, story_full_id: StoryFullId) -> Result<(), StoryDbError>;

    /// Gets a story from the database.
    fn get_story(&self, story_full_id: StoryFullId) -> Result<Option<BufferSlice>, StoryDbError>;

    /// Gets expiring stories.
    fn get_expiring_stories(
        &self,
        expires_till: i32,
        limit: i32,
    ) -> Result<Vec<StoryDbStory>, StoryDbError>;
}

/// Story database error type.
///
/// Represents errors that can occur in story database operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoryDbError {
    /// Database not initialized.
    NotInitialized,

    /// Story not found.
    StoryNotFound,

    /// Invalid story data.
    InvalidData,

    /// Database error.
    DatabaseError(String),
}

impl fmt::Display for StoryDbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "Database not initialized"),
            Self::StoryNotFound => write!(f, "Story not found"),
            Self::InvalidData => write!(f, "Invalid story data"),
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for StoryDbError {}

/// Stub story database implementation.
///
/// TODO: Replace with actual database implementation.
#[derive(Debug, Default)]
pub struct StubStoryDb {
    /// Inner stub data.
    #[allow(dead_code)]
    inner: (),
}

impl StubStoryDb {
    /// Creates a new stub story database.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl StoryDb for StubStoryDb {
    fn add_story(
        &mut self,
        _story_full_id: StoryFullId,
        _expires_at: i32,
        _notification_id: i32,
        _data: BufferSlice,
    ) -> Result<(), StoryDbError> {
        Err(StoryDbError::NotInitialized)
    }

    fn delete_story(&mut self, _story_full_id: StoryFullId) -> Result<(), StoryDbError> {
        Err(StoryDbError::NotInitialized)
    }

    fn get_story(&self, _story_full_id: StoryFullId) -> Result<Option<BufferSlice>, StoryDbError> {
        Err(StoryDbError::NotInitialized)
    }

    fn get_expiring_stories(
        &self,
        _expires_till: i32,
        _limit: i32,
    ) -> Result<Vec<StoryDbStory>, StoryDbError> {
        Err(StoryDbError::NotInitialized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== StoryDbStory Tests ==========

    #[test]
    fn test_story_db_story_new() {
        let full_id = StoryFullId::new(DialogId::new(123), rustgram_story_id::StoryId::new(456));
        let data = BufferSlice::new(vec![1, 2, 3]);
        let story = StoryDbStory::new(full_id, data);
        assert_eq!(story.story_full_id().dialog_id().get(), 123);
    }

    // ========== ActiveStoryListResult Tests ==========

    #[test]
    fn test_active_story_list_result_new() {
        let result = ActiveStoryListResult::new(vec![], 0, DialogId::new(0));
        assert!(result.active_stories().is_empty());
        assert_eq!(result.next_order(), 0);
    }

    // ========== StoryDbError Tests ==========

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", StoryDbError::NotInitialized),
            "Database not initialized"
        );
        assert_eq!(
            format!("{}", StoryDbError::StoryNotFound),
            "Story not found"
        );
        assert_eq!(
            format!("{}", StoryDbError::DatabaseError("test".to_string())),
            "Database error: test"
        );
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(StoryDbError::NotInitialized, StoryDbError::NotInitialized);
        assert_ne!(StoryDbError::NotInitialized, StoryDbError::StoryNotFound);
    }

    // ========== StubStoryDb Tests ==========

    #[test]
    fn test_stub_db_new() {
        let mut db = StubStoryDb::new();
        let result = db.add_story(
            StoryFullId::new(DialogId::new(123), rustgram_story_id::StoryId::new(456)),
            0,
            0,
            BufferSlice::new(vec![]),
        );
        assert_eq!(result, Err(StoryDbError::NotInitialized));
    }

    #[test]
    fn test_stub_db_get_story() {
        let db = StubStoryDb::new();
        let result = db.get_story(StoryFullId::new(
            DialogId::new(123),
            rustgram_story_id::StoryId::new(456),
        ));
        assert_eq!(result, Err(StoryDbError::NotInitialized));
    }
}
