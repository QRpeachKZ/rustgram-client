//! # Rustgram ActiveStoryState
//!
//! Active story state tracking for Telegram MTProto client.
//!
//! This crate provides types for tracking the read/unread state of stories
//! in Telegram. It implements the story state management from TDLib with
//! support for live stories, unread tracking, and TD API object conversion.
//!
//! ## Overview
//!
//! - [`StoryId`] - Story identifier with validation and server/local detection
//! - [`ActiveStoryState`] - Tracks active and read story IDs with live story detection
//! - [`ActiveStoryStateObject`] - TD API object representation variants
//!
//! ## Story ID Validation
//!
//! Story IDs can be either server stories or local stories:
//!
//! - **Server stories**: IDs in range `1..=MAX_SERVER_STORY_ID` (1,999,999,999)
//! - **Local stories**: IDs <= 0 or > MAX_SERVER_STORY_ID
//! - **Invalid**: ID == 0
//!
//! ## Story States
//!
//! The [`ActiveStoryStateObject`] represents different story states:
//!
//! - `Live` - Currently broadcasting live story
//! - `Unread` - Has unread stories (active > read)
//! - `Read` - All stories read (active <= read)
//! - `None` - Non-server story (local or invalid)
//!
//! ## Examples
//!
//! Basic story ID usage:
//!
//! ```
//! use rustgram_active_story_state::StoryId;
//!
//! let server_story = StoryId::new(123);
//! assert!(server_story.is_server());
//! assert!(server_story.is_valid());
//! assert_eq!(server_story.get(), 123);
//!
//! let local_story = StoryId::new(-1);
//! assert!(!local_story.is_server());
//! assert!(local_story.is_valid());
//!
//! let invalid_story = StoryId::new(0);
//! assert!(!invalid_story.is_valid());
//! ```
//!
//! Tracking unread stories:
//!
//! ```
//! use rustgram_active_story_state::{ActiveStoryState, StoryId};
//!
//! let state = ActiveStoryState::new(
//!     StoryId::new(100), // max active
//!     StoryId::new(50),  // max read
//!     false,            // no live story
//! );
//!
//! assert!(state.has_unread_stories());
//! assert_eq!(state.max_active_story_id().get(), 100);
//! assert_eq!(state.max_read_story_id().get(), 50);
//! ```
//!
//! Converting to TD API object:
//!
//! ```
//! use rustgram_active_story_state::{ActiveStoryState, ActiveStoryStateObject, StoryId};
//!
//! // Live story
//! let live_state = ActiveStoryState::new(
//!     StoryId::new(100),
//!     StoryId::new(50),
//!     true, // has live story
//! );
//! assert!(matches!(
//!     live_state.get_active_story_state_object(),
//!     ActiveStoryStateObject::Live(_)
//! ));
//!
//! // Unread story
//! let unread_state = ActiveStoryState::new(
//!     StoryId::new(100),
//!     StoryId::new(50),
//!     false,
//! );
//! assert_eq!(
//!     unread_state.get_active_story_state_object(),
//!     ActiveStoryStateObject::Unread
//! );
//!
//! // Read story
//! let read_state = ActiveStoryState::new(
//!     StoryId::new(50),
//!     StoryId::new(100),
//!     false,
//! );
//! assert_eq!(
//!     read_state.get_active_story_state_object(),
//!     ActiveStoryStateObject::Read
//! );
//!
//! // Non-server story
//! let local_state = ActiveStoryState::new(
//!     StoryId::new(-1),
//!     StoryId::new(0),
//!     false,
//! );
//! assert_eq!(
//!     local_state.get_active_story_state_object(),
//!     ActiveStoryStateObject::None
//! );
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Maximum server story ID as defined by Telegram.
///
/// Story IDs greater than this value are considered local stories.
pub const MAX_SERVER_STORY_ID: i32 = 1_999_999_999;

/// Story identifier with validation.
///
/// Story IDs can represent either server stories (valid range) or local stories.
/// This type provides validation methods to distinguish between them.
///
/// # Examples
///
/// ```
/// use rustgram_active_story_state::StoryId;
///
/// let server_id = StoryId::new(123);
/// assert!(server_id.is_server());
///
/// let local_id = StoryId::new(-1);
/// assert!(!local_id.is_server());
///
/// let invalid = StoryId::new(0);
/// assert!(!invalid.is_valid());
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct StoryId(i32);

impl StoryId {
    /// Creates a new story ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The raw story ID value
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_active_story_state::StoryId;
    ///
    /// let id = StoryId::new(42);
    /// assert_eq!(id.get(), 42);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(id: i32) -> Self {
        Self(id)
    }

    /// Returns the raw story ID value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_active_story_state::StoryId;
    ///
    /// let id = StoryId::new(123);
    /// assert_eq!(id.get(), 123);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get(&self) -> i32 {
        self.0
    }

    /// Checks if this is a server story ID.
    ///
    /// Server stories have IDs in the range `1..=MAX_SERVER_STORY_ID`.
    ///
    /// # Returns
    ///
    /// `true` if this is a valid server story ID, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_active_story_state::StoryId;
    ///
    /// assert!(StoryId::new(1).is_server());
    /// assert!(StoryId::new(1_999_999_999).is_server());
    /// assert!(!StoryId::new(0).is_server());
    /// assert!(!StoryId::new(-1).is_server());
    /// assert!(!StoryId::new(2_000_000_000).is_server());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_server(&self) -> bool {
        self.0 > 0 && self.0 <= MAX_SERVER_STORY_ID
    }

    /// Checks if this story ID is valid (non-zero).
    ///
    /// # Returns
    ///
    /// `true` if the ID is non-zero, `false` if it's zero
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_active_story_state::StoryId;
    ///
    /// assert!(StoryId::new(1).is_valid());
    /// assert!(StoryId::new(-1).is_valid());
    /// assert!(!StoryId::new(0).is_valid());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.0 != 0
    }
}

impl fmt::Display for StoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Hash for StoryId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Ord for StoryId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for StoryId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Active story state tracking.
///
/// Tracks the maximum active story ID, maximum read story ID, and whether
/// there's a live story. This structure is used to determine if there are
/// unread stories and to convert to TD API objects.
///
/// # Examples
///
/// ```
/// use rustgram_active_story_state::{ActiveStoryState, StoryId};
///
/// let state = ActiveStoryState::new(
///     StoryId::new(100),
///     StoryId::new(50),
///     false,
/// );
///
/// assert!(state.has_unread_stories());
/// assert_eq!(state.max_active_story_id().get(), 100);
/// assert!(!state.has_live_story());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ActiveStoryState {
    max_active_story_id: StoryId,
    max_read_story_id: StoryId,
    has_live_story: bool,
}

impl ActiveStoryState {
    /// Creates a new active story state.
    ///
    /// # Arguments
    ///
    /// * `max_active_story_id` - The maximum active story ID
    /// * `max_read_story_id` - The maximum read story ID
    /// * `has_live_story` - Whether there's currently a live story
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_active_story_state::{ActiveStoryState, StoryId};
    ///
    /// let state = ActiveStoryState::new(
    ///     StoryId::new(100),
    ///     StoryId::new(50),
    ///     true,
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(
        max_active_story_id: StoryId,
        max_read_story_id: StoryId,
        has_live_story: bool,
    ) -> Self {
        Self {
            max_active_story_id,
            max_read_story_id,
            has_live_story,
        }
    }

    /// Checks if there are unread stories.
    ///
    /// Stories are considered unread if the maximum active story ID
    /// is greater than the maximum read story ID.
    ///
    /// # Returns
    ///
    /// `true` if there are unread stories, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_active_story_state::{ActiveStoryState, StoryId};
    ///
    /// let unread = ActiveStoryState::new(
    ///     StoryId::new(100),
    ///     StoryId::new(50),
    ///     false,
    /// );
    /// assert!(unread.has_unread_stories());
    ///
    /// let read = ActiveStoryState::new(
    ///     StoryId::new(50),
    ///     StoryId::new(100),
    ///     false,
    /// );
    /// assert!(!read.has_unread_stories());
    /// ```
    #[inline]
    #[must_use]
    pub fn has_unread_stories(&self) -> bool {
        self.max_active_story_id.get() > self.max_read_story_id.get()
    }

    /// Converts this state to a TD API object.
    ///
    /// Returns the appropriate [`ActiveStoryStateObject`] variant based on:
    /// - Non-server stories return `None`
    /// - Live stories return `Live(id)`
    /// - Unread stories return `Unread`
    /// - Read stories return `Read`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_active_story_state::{ActiveStoryState, ActiveStoryStateObject, StoryId};
    ///
    /// // Live story
    /// let live = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), true);
    /// assert!(matches!(
    ///     live.get_active_story_state_object(),
    ///     ActiveStoryStateObject::Live(_)
    /// ));
    ///
    /// // Unread
    /// let unread = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), false);
    /// assert_eq!(unread.get_active_story_state_object(), ActiveStoryStateObject::Unread);
    ///
    /// // Read
    /// let read = ActiveStoryState::new(StoryId::new(50), StoryId::new(100), false);
    /// assert_eq!(read.get_active_story_state_object(), ActiveStoryStateObject::Read);
    ///
    /// // Non-server
    /// let local = ActiveStoryState::new(StoryId::new(-1), StoryId::new(0), false);
    /// assert_eq!(local.get_active_story_state_object(), ActiveStoryStateObject::None);
    /// ```
    #[inline]
    #[must_use]
    pub fn get_active_story_state_object(&self) -> ActiveStoryStateObject {
        if !self.max_active_story_id.is_server() {
            return ActiveStoryStateObject::None;
        }
        if self.has_live_story {
            return ActiveStoryStateObject::Live(self.max_active_story_id);
        }
        if self.has_unread_stories() {
            return ActiveStoryStateObject::Unread;
        }
        ActiveStoryStateObject::Read
    }

    /// Returns the maximum active story ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_active_story_state::{ActiveStoryState, StoryId};
    ///
    /// let state = ActiveStoryState::new(
    ///     StoryId::new(100),
    ///     StoryId::new(50),
    ///     false,
    /// );
    /// assert_eq!(state.max_active_story_id().get(), 100);
    /// ```
    #[inline]
    #[must_use]
    pub const fn max_active_story_id(&self) -> StoryId {
        self.max_active_story_id
    }

    /// Returns the maximum read story ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_active_story_state::{ActiveStoryState, StoryId};
    ///
    /// let state = ActiveStoryState::new(
    ///     StoryId::new(100),
    ///     StoryId::new(50),
    ///     false,
    /// );
    /// assert_eq!(state.max_read_story_id().get(), 50);
    /// ```
    #[inline]
    #[must_use]
    pub const fn max_read_story_id(&self) -> StoryId {
        self.max_read_story_id
    }

    /// Checks if there's a live story.
    ///
    /// # Returns
    ///
    /// `true` if there's currently a live story, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_active_story_state::{ActiveStoryState, StoryId};
    ///
    /// let live = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), true);
    /// assert!(live.has_live_story());
    ///
    /// let not_live = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), false);
    /// assert!(!not_live.has_live_story());
    /// ```
    #[inline]
    #[must_use]
    pub const fn has_live_story(&self) -> bool {
        self.has_live_story
    }
}

impl fmt::Display for ActiveStoryState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ActiveStoryState {{ active: {}, read: {}, live: {} }}",
            self.max_active_story_id, self.max_read_story_id, self.has_live_story
        )
    }
}

/// TD API object representation of active story state.
///
/// This enum represents the different states that can be returned
/// by the TD API when querying story state.
///
/// # Variants
///
/// * `Live` - Currently broadcasting live story (contains story ID)
/// * `Unread` - Has unread stories
/// * `Read` - All stories are read
/// * `None` - Non-server story (local or invalid)
///
/// # Examples
///
/// ```
/// use rustgram_active_story_state::{ActiveStoryState, ActiveStoryStateObject, StoryId};
///
/// let state = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), false);
/// match state.get_active_story_state_object() {
///     ActiveStoryStateObject::Live(id) => println!("Live story: {}", id),
///     ActiveStoryStateObject::Unread => println!("Has unread stories"),
///     ActiveStoryStateObject::Read => println!("All read"),
///     ActiveStoryStateObject::None => println!("Non-server story"),
/// }
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ActiveStoryStateObject {
    /// Currently broadcasting live story with the story ID.
    Live(StoryId),
    /// Has unread stories.
    Unread,
    /// All stories are read.
    Read,
    /// Non-server story (local or invalid).
    None,
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-active-story-state";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== StoryId Tests ==========

    #[test]
    fn test_story_id_new_positive() {
        let id = StoryId::new(42);
        assert_eq!(id.get(), 42);
    }

    #[test]
    fn test_story_id_new_zero() {
        let id = StoryId::new(0);
        assert_eq!(id.get(), 0);
    }

    #[test]
    fn test_story_id_new_negative() {
        let id = StoryId::new(-5);
        assert_eq!(id.get(), -5);
    }

    #[test]
    fn test_story_id_get_returns_value() {
        let id = StoryId::new(123);
        assert_eq!(id.get(), 123);
    }

    #[test]
    fn test_story_id_is_server_for_valid_range() {
        assert!(StoryId::new(1).is_server());
        assert!(StoryId::new(100).is_server());
        assert!(StoryId::new(1_999_999_999).is_server());
    }

    #[test]
    fn test_story_id_is_server_for_local_stories() {
        assert!(!StoryId::new(0).is_server());
        assert!(!StoryId::new(-1).is_server());
        assert!(!StoryId::new(2_000_000_000).is_server());
        assert!(!StoryId::new(i32::MAX).is_server());
    }

    #[test]
    fn test_story_id_is_valid_for_nonzero() {
        assert!(StoryId::new(1).is_valid());
        assert!(StoryId::new(-1).is_valid());
        assert!(StoryId::new(i32::MAX).is_valid());
    }

    #[test]
    fn test_story_id_is_valid_for_zero() {
        assert!(!StoryId::new(0).is_valid());
    }

    #[test]
    fn test_story_id_max_server_constant() {
        assert_eq!(MAX_SERVER_STORY_ID, 1_999_999_999);
        assert!(StoryId::new(MAX_SERVER_STORY_ID).is_server());
        assert!(!StoryId::new(MAX_SERVER_STORY_ID + 1).is_server());
    }

    #[test]
    fn test_story_id_ordering() {
        let id1 = StoryId::new(10);
        let id2 = StoryId::new(20);
        let id3 = StoryId::new(10);

        assert!(id1 < id2);
        assert!(id2 > id1);
        assert_eq!(id1, id3);
        assert_eq!(id1.partial_cmp(&id2), Some(std::cmp::Ordering::Less));
    }

    // ========== ActiveStoryState Tests ==========

    #[test]
    fn test_active_story_state_new_with_valid_ids() {
        let state = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), false);
        assert_eq!(state.max_active_story_id().get(), 100);
        assert_eq!(state.max_read_story_id().get(), 50);
        assert!(!state.has_live_story());
    }

    #[test]
    fn test_active_story_state_new_with_zero_ids() {
        let state = ActiveStoryState::new(StoryId::new(0), StoryId::new(0), false);
        assert_eq!(state.max_active_story_id().get(), 0);
        assert_eq!(state.max_read_story_id().get(), 0);
    }

    #[test]
    fn test_has_unread_stories_when_active_greater() {
        let state = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), false);
        assert!(state.has_unread_stories());
    }

    #[test]
    fn test_has_unread_stories_when_equal() {
        let state = ActiveStoryState::new(StoryId::new(100), StoryId::new(100), false);
        assert!(!state.has_unread_stories());
    }

    #[test]
    fn test_has_unread_stories_when_active_less() {
        let state = ActiveStoryState::new(StoryId::new(50), StoryId::new(100), false);
        assert!(!state.has_unread_stories());
    }

    #[test]
    fn test_get_active_story_state_object_returns_live() {
        let state = ActiveStoryState::new(
            StoryId::new(100),
            StoryId::new(50),
            true, // has live story
        );
        match state.get_active_story_state_object() {
            ActiveStoryStateObject::Live(id) => assert_eq!(id.get(), 100),
            _ => panic!("Expected Live state"),
        }
    }

    #[test]
    fn test_get_active_story_state_object_returns_unread() {
        let state = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), false);
        assert_eq!(
            state.get_active_story_state_object(),
            ActiveStoryStateObject::Unread
        );
    }

    #[test]
    fn test_get_active_story_state_object_returns_read() {
        let state = ActiveStoryState::new(StoryId::new(50), StoryId::new(100), false);
        assert_eq!(
            state.get_active_story_state_object(),
            ActiveStoryStateObject::Read
        );
    }

    #[test]
    fn test_get_active_story_state_object_returns_none_for_local() {
        let state = ActiveStoryState::new(
            StoryId::new(-1), // local story
            StoryId::new(0),
            false,
        );
        assert_eq!(
            state.get_active_story_state_object(),
            ActiveStoryStateObject::None
        );
    }

    #[test]
    fn test_max_active_story_id_accessor() {
        let state = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), false);
        assert_eq!(state.max_active_story_id().get(), 100);
    }

    #[test]
    fn test_max_read_story_id_accessor() {
        let state = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), false);
        assert_eq!(state.max_read_story_id().get(), 50);
    }

    #[test]
    fn test_has_live_story_accessor() {
        let live_state = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), true);
        assert!(live_state.has_live_story());

        let non_live_state = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), false);
        assert!(!non_live_state.has_live_story());
    }

    // ========== Integration Tests ==========

    #[test]
    fn test_display_story_id() {
        let id = StoryId::new(42);
        assert_eq!(format!("{}", id), "42");
    }

    #[test]
    fn test_display_active_story_state() {
        let state = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), true);
        let display = format!("{}", state);
        assert!(display.contains("100"));
        assert!(display.contains("50"));
        assert!(display.contains("true"));
    }

    #[test]
    fn test_story_id_equality() {
        let id1 = StoryId::new(42);
        let id2 = StoryId::new(42);
        let id3 = StoryId::new(43);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_active_story_state_equality() {
        let state1 = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), true);
        let state2 = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), true);
        let state3 = ActiveStoryState::new(StoryId::new(100), StoryId::new(50), false);

        assert_eq!(state1, state2);
        assert_ne!(state1, state3);
    }

    #[test]
    fn test_active_story_state_object_equality() {
        let obj1 = ActiveStoryStateObject::Unread;
        let obj2 = ActiveStoryStateObject::Unread;
        let obj3 = ActiveStoryStateObject::Read;

        assert_eq!(obj1, obj2);
        assert_ne!(obj1, obj3);
    }

    #[test]
    fn test_crate_metadata() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-active-story-state");
    }
}
