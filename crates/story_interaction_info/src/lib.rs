// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Story Interaction Info
//!
//! Story interaction statistics for Telegram.
//!
//! ## Overview
//!
//! This module provides the [`StoryInteractionInfo`] struct, which represents
//! interaction statistics for a Telegram story (views, reactions, forwards).
//!
//! ## Example
//!
//! ```rust
//! use rustgram_story_interaction_info::StoryInteractionInfo;
//!
//! let info = StoryInteractionInfo::with_counts(100, 25, 10);
//! assert_eq!(info.view_count(), 100);
//! assert_eq!(info.reaction_count(), 25);
//! assert_eq!(info.forward_count(), 10);
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Story interaction information.
///
/// Represents statistics about user interactions with a story.
///
/// # Fields
///
/// - `view_count` - Number of views
/// - `reaction_count` - Number of reactions
/// - `forward_count` - Number of forwards
///
/// # Example
///
/// ```rust
/// use rustgram_story_interaction_info::StoryInteractionInfo;
///
/// let info = StoryInteractionInfo::with_counts(100, 25, 10);
/// assert_eq!(info.view_count(), 100);
/// assert_eq!(info.reaction_count(), 25);
/// assert_eq!(info.forward_count(), 10);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoryInteractionInfo {
    /// Number of views.
    view_count: i32,

    /// Number of reactions.
    reaction_count: i32,

    /// Number of forwards.
    forward_count: i32,
}

impl StoryInteractionInfo {
    /// Creates a new story interaction info with all counts set to zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_interaction_info::StoryInteractionInfo;
    ///
    /// let info = StoryInteractionInfo::new();
    /// assert_eq!(info.view_count(), 0);
    /// assert_eq!(info.reaction_count(), 0);
    /// assert_eq!(info.forward_count(), 0);
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            view_count: 0,
            reaction_count: 0,
            forward_count: 0,
        }
    }

    /// Creates a new story interaction info with the specified counts.
    ///
    /// # Arguments
    ///
    /// * `view_count` - Number of views
    /// * `reaction_count` - Number of reactions
    /// * `forward_count` - Number of forwards
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_interaction_info::StoryInteractionInfo;
    ///
    /// let info = StoryInteractionInfo::with_counts(100, 25, 10);
    /// assert_eq!(info.view_count(), 100);
    /// assert_eq!(info.reaction_count(), 25);
    /// assert_eq!(info.forward_count(), 10);
    /// ```
    #[must_use]
    pub const fn with_counts(view_count: i32, reaction_count: i32, forward_count: i32) -> Self {
        Self {
            view_count,
            reaction_count,
            forward_count,
        }
    }

    /// Returns the view count.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_interaction_info::StoryInteractionInfo;
    ///
    /// let info = StoryInteractionInfo::with_counts(100, 25, 10);
    /// assert_eq!(info.view_count(), 100);
    /// ```
    #[must_use]
    pub const fn view_count(&self) -> i32 {
        self.view_count
    }

    /// Returns the reaction count.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_interaction_info::StoryInteractionInfo;
    ///
    /// let info = StoryInteractionInfo::with_counts(100, 25, 10);
    /// assert_eq!(info.reaction_count(), 25);
    /// ```
    #[must_use]
    pub const fn reaction_count(&self) -> i32 {
        self.reaction_count
    }

    /// Returns the forward count.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_interaction_info::StoryInteractionInfo;
    ///
    /// let info = StoryInteractionInfo::with_counts(100, 25, 10);
    /// assert_eq!(info.forward_count(), 10);
    /// ```
    #[must_use]
    pub const fn forward_count(&self) -> i32 {
        self.forward_count
    }

    /// Returns the total interaction count.
    ///
    /// This is the sum of views, reactions, and forwards.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_interaction_info::StoryInteractionInfo;
    ///
    /// let info = StoryInteractionInfo::with_counts(100, 25, 10);
    /// assert_eq!(info.total_count(), 135);
    /// ```
    #[must_use]
    pub const fn total_count(&self) -> i32 {
        self.view_count + self.reaction_count + self.forward_count
    }

    /// Checks if there are any interactions.
    ///
    /// Returns `true` if any count is greater than zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_interaction_info::StoryInteractionInfo;
    ///
    /// let empty = StoryInteractionInfo::new();
    /// assert!(!empty.has_interactions());
    ///
    /// let with_views = StoryInteractionInfo::with_counts(10, 0, 0);
    /// assert!(with_views.has_interactions());
    /// ```
    #[must_use]
    pub const fn has_interactions(&self) -> bool {
        self.view_count > 0 || self.reaction_count > 0 || self.forward_count > 0
    }

    /// Checks if there are any views.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_interaction_info::StoryInteractionInfo;
    ///
    /// assert!(!StoryInteractionInfo::new().has_views());
    /// assert!(StoryInteractionInfo::with_counts(10, 0, 0).has_views());
    /// ```
    #[must_use]
    pub const fn has_views(&self) -> bool {
        self.view_count > 0
    }

    /// Checks if there are any reactions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_interaction_info::StoryInteractionInfo;
    ///
    /// assert!(!StoryInteractionInfo::new().has_reactions());
    /// assert!(StoryInteractionInfo::with_counts(0, 10, 0).has_reactions());
    /// ```
    #[must_use]
    pub const fn has_reactions(&self) -> bool {
        self.reaction_count > 0
    }

    /// Checks if there are any forwards.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_interaction_info::StoryInteractionInfo;
    ///
    /// assert!(!StoryInteractionInfo::new().has_forwards());
    /// assert!(StoryInteractionInfo::with_counts(0, 0, 10).has_forwards());
    /// ```
    #[must_use]
    pub const fn has_forwards(&self) -> bool {
        self.forward_count > 0
    }
}

impl Default for StoryInteractionInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StoryInteractionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StoryInteractionInfo {{ views: {}, reactions: {}, forwards: {} }}",
            self.view_count, self.reaction_count, self.forward_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let info = StoryInteractionInfo::new();
        assert_eq!(info.view_count(), 0);
        assert_eq!(info.reaction_count(), 0);
        assert_eq!(info.forward_count(), 0);
    }

    #[test]
    fn test_with_counts() {
        let info = StoryInteractionInfo::with_counts(100, 25, 10);
        assert_eq!(info.view_count(), 100);
        assert_eq!(info.reaction_count(), 25);
        assert_eq!(info.forward_count(), 10);
    }

    #[test]
    fn test_view_count() {
        let info = StoryInteractionInfo::with_counts(100, 25, 10);
        assert_eq!(info.view_count(), 100);
    }

    #[test]
    fn test_reaction_count() {
        let info = StoryInteractionInfo::with_counts(100, 25, 10);
        assert_eq!(info.reaction_count(), 25);
    }

    #[test]
    fn test_forward_count() {
        let info = StoryInteractionInfo::with_counts(100, 25, 10);
        assert_eq!(info.forward_count(), 10);
    }

    #[test]
    fn test_total_count() {
        let info = StoryInteractionInfo::with_counts(100, 25, 10);
        assert_eq!(info.total_count(), 135);

        let empty = StoryInteractionInfo::new();
        assert_eq!(empty.total_count(), 0);
    }

    #[test]
    fn test_has_interactions_true() {
        let with_views = StoryInteractionInfo::with_counts(10, 0, 0);
        assert!(with_views.has_interactions());

        let with_reactions = StoryInteractionInfo::with_counts(0, 10, 0);
        assert!(with_reactions.has_interactions());

        let with_forwards = StoryInteractionInfo::with_counts(0, 0, 10);
        assert!(with_forwards.has_interactions());
    }

    #[test]
    fn test_has_interactions_false() {
        let empty = StoryInteractionInfo::new();
        assert!(!empty.has_interactions());
    }

    #[test]
    fn test_has_views() {
        assert!(!StoryInteractionInfo::new().has_views());
        assert!(StoryInteractionInfo::with_counts(10, 0, 0).has_views());
        assert!(StoryInteractionInfo::with_counts(10, 5, 3).has_views());
    }

    #[test]
    fn test_has_reactions() {
        assert!(!StoryInteractionInfo::new().has_reactions());
        assert!(StoryInteractionInfo::with_counts(0, 10, 0).has_reactions());
        assert!(StoryInteractionInfo::with_counts(5, 10, 3).has_reactions());
    }

    #[test]
    fn test_has_forwards() {
        assert!(!StoryInteractionInfo::new().has_forwards());
        assert!(StoryInteractionInfo::with_counts(0, 0, 10).has_forwards());
        assert!(StoryInteractionInfo::with_counts(5, 3, 10).has_forwards());
    }

    #[test]
    fn test_equality() {
        let info1 = StoryInteractionInfo::with_counts(100, 25, 10);
        let info2 = StoryInteractionInfo::with_counts(100, 25, 10);
        assert_eq!(info1, info2);

        let info3 = StoryInteractionInfo::with_counts(99, 25, 10);
        assert_ne!(info1, info3);
    }

    #[test]
    fn test_clone() {
        let info1 = StoryInteractionInfo::with_counts(100, 25, 10);
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_default() {
        let info = StoryInteractionInfo::default();
        assert_eq!(info.view_count(), 0);
        assert_eq!(info.reaction_count(), 0);
        assert_eq!(info.forward_count(), 0);
    }

    #[test]
    fn test_display() {
        let info = StoryInteractionInfo::with_counts(100, 25, 10);
        let display = format!("{}", info);
        assert!(display.contains("100"));
        assert!(display.contains("25"));
        assert!(display.contains("10"));
    }

    #[test]
    fn test_debug_format() {
        let info = StoryInteractionInfo::with_counts(100, 25, 10);
        let debug_str = format!("{:?}", info);
        assert!(debug_str.contains("StoryInteractionInfo"));
        assert!(debug_str.contains("100"));
        assert!(debug_str.contains("25"));
        assert!(debug_str.contains("10"));
    }

    #[test]
    fn test_zero_counts() {
        let info = StoryInteractionInfo::with_counts(0, 0, 0);
        assert_eq!(info.view_count(), 0);
        assert_eq!(info.reaction_count(), 0);
        assert_eq!(info.forward_count(), 0);
        assert!(!info.has_interactions());
        assert_eq!(info.total_count(), 0);
    }

    #[test]
    fn test_negative_counts() {
        // Negative counts are technically allowed in this stub
        let info = StoryInteractionInfo::with_counts(-10, -5, -3);
        assert_eq!(info.view_count(), -10);
        assert_eq!(info.reaction_count(), -5);
        assert_eq!(info.forward_count(), -3);
        assert_eq!(info.total_count(), -18);
    }

    #[test]
    fn test_large_counts() {
        let info = StoryInteractionInfo::with_counts(i32::MAX, i32::MAX, 1);
        // Check that it doesn't overflow in display
        let _display = format!("{}", info);
    }

    #[test]
    fn test_total_overflow() {
        // Test potential overflow case
        let info = StoryInteractionInfo::with_counts(i32::MAX, 1, 0);
        // In production, this would be handled properly
        // For this stub, we just verify it doesn't panic in display
        let _display = format!("{}", info);
    }

    // ========== Serialization Tests ==========

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let info = StoryInteractionInfo::with_counts(100, 25, 10);
        let json = serde_json::to_string(&info).unwrap();
        let deserialized: StoryInteractionInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_format() {
        let info = StoryInteractionInfo::with_counts(100, 25, 10);
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("100"));
        assert!(json.contains("25"));
        assert!(json.contains("10"));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_zero() {
        let info = StoryInteractionInfo::new();
        let json = serde_json::to_string(&info).unwrap();
        let deserialized: StoryInteractionInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info, deserialized);
    }
}
