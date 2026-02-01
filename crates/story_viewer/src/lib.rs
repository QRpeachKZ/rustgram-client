// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Story Viewer
//!
//! Story viewer types for Telegram.
//!
//! ## Overview
//!
//! This module provides the [`StoryViewer`] type, which represents information
//! about who viewed a story and how they interacted with it.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_story_viewer::{StoryViewer, ViewerType};
//! use rustgram_dialog_id::DialogId;
//!
//! let viewer = StoryViewer::new_view(DialogId::new(123), 456);
//! assert_eq!(viewer.actor_dialog_id().get(), 123);
//! assert_eq!(viewer.date(), 456);
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use rustgram_dialog_id::DialogId;
use rustgram_reaction_type::ReactionType;
use rustgram_story_id::StoryId;

/// Type of story viewer interaction.
///
/// Represents how a user interacted with a story.
///
/// Based on TDLib's `StoryViewer::Type` enum.
///
/// # TDLib Alignment
///
/// Aligns with TDLib's `enum class Type` in `StoryViewer.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ViewerType {
    /// No specific interaction type.
    #[default]
    None,
    /// User viewed the story.
    View,
    /// Story was forwarded.
    Forward,
    /// Story was reposted.
    Repost,
}

impl ViewerType {
    /// Returns `true` if this is a view interaction.
    #[must_use]
    pub const fn is_view(&self) -> bool {
        matches!(self, Self::View)
    }

    /// Returns `true` if this is a forward interaction.
    #[must_use]
    pub const fn is_forward(&self) -> bool {
        matches!(self, Self::Forward)
    }

    /// Returns `true` if this is a repost interaction.
    #[must_use]
    pub const fn is_repost(&self) -> bool {
        matches!(self, Self::Repost)
    }
}

/// Story viewer information.
///
/// Contains information about who viewed a story and how they interacted with it.
///
/// Based on TDLib's `StoryViewer` class.
///
/// # TDLib Alignment
///
/// Aligns with TDLib's `StoryViewer` class in `StoryViewer.h`.
///
/// # Example
///
/// ```rust
/// use rustgram_story_viewer::{StoryViewer, ViewerType};
/// use rustgram_dialog_id::DialogId;
///
/// let viewer = StoryViewer::new_view(DialogId::new(123), 456);
/// assert_eq!(viewer.actor_dialog_id().get(), 123);
/// assert_eq!(viewer.date(), 456);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoryViewer {
    /// The type of viewer interaction.
    viewer_type: ViewerType,

    /// The dialog ID of the viewer.
    actor_dialog_id: DialogId,

    /// The date when the story was viewed.
    date: i32,

    /// Whether the viewer is blocked.
    is_blocked: bool,

    /// Whether the viewer is blocked for stories.
    is_blocked_for_stories: bool,

    /// The reaction type (for View interactions).
    reaction_type: Option<ReactionType>,

    /// The story ID (for Repost interactions).
    story_id: Option<StoryId>,

    /// The message ID (for Forward interactions) - stubbed as i64.
    message_id: Option<i64>,
}

impl StoryViewer {
    /// Creates a new story viewer for a view interaction.
    ///
    /// # Arguments
    ///
    /// * `actor_dialog_id` - The dialog ID of the viewer
    /// * `date` - The date when the story was viewed
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_viewer::StoryViewer;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let viewer = StoryViewer::new_view(DialogId::new(123), 456);
    /// assert_eq!(viewer.actor_dialog_id().get(), 123);
    /// ```
    #[must_use]
    pub fn new_view(actor_dialog_id: DialogId, date: i32) -> Self {
        Self {
            viewer_type: ViewerType::View,
            actor_dialog_id,
            date,
            is_blocked: false,
            is_blocked_for_stories: false,
            reaction_type: None,
            story_id: None,
            message_id: None,
        }
    }

    /// Creates a new story viewer for a forward interaction.
    ///
    /// # Arguments
    ///
    /// * `actor_dialog_id` - The dialog ID of the viewer
    /// * `date` - The date when the story was viewed
    /// * `message_id` - The message ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_viewer::StoryViewer;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let viewer = StoryViewer::new_forward(DialogId::new(123), 456, 789);
    /// assert!(viewer.viewer_type().is_forward());
    /// ```
    #[must_use]
    pub fn new_forward(actor_dialog_id: DialogId, date: i32, message_id: i64) -> Self {
        Self {
            viewer_type: ViewerType::Forward,
            actor_dialog_id,
            date,
            is_blocked: false,
            is_blocked_for_stories: false,
            reaction_type: None,
            story_id: None,
            message_id: Some(message_id),
        }
    }

    /// Creates a new story viewer for a repost interaction.
    ///
    /// # Arguments
    ///
    /// * `actor_dialog_id` - The dialog ID of the viewer
    /// * `date` - The date when the story was viewed
    /// * `story_id` - The story ID
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_story_viewer::StoryViewer;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_story_id::StoryId;
    ///
    /// let viewer = StoryViewer::new_repost(DialogId::new(123), 456, StoryId::new(789));
    /// assert!(viewer.viewer_type().is_repost());
    /// ```
    #[must_use]
    pub fn new_repost(actor_dialog_id: DialogId, date: i32, story_id: StoryId) -> Self {
        Self {
            viewer_type: ViewerType::Repost,
            actor_dialog_id,
            date,
            is_blocked: false,
            is_blocked_for_stories: false,
            reaction_type: None,
            message_id: None,
            story_id: Some(story_id),
        }
    }

    /// Returns the viewer type.
    #[must_use]
    pub const fn viewer_type(&self) -> ViewerType {
        self.viewer_type
    }

    /// Returns the dialog ID of the viewer.
    #[must_use]
    pub const fn actor_dialog_id(&self) -> DialogId {
        self.actor_dialog_id
    }

    /// Returns the date when the story was viewed.
    #[must_use]
    pub const fn date(&self) -> i32 {
        self.date
    }

    /// Returns whether the viewer is blocked.
    #[must_use]
    pub const fn is_blocked(&self) -> bool {
        self.is_blocked
    }

    /// Returns whether the viewer is blocked for stories.
    #[must_use]
    pub const fn is_blocked_for_stories(&self) -> bool {
        self.is_blocked_for_stories
    }

    /// Returns the reaction type (if available).
    #[must_use]
    pub const fn reaction_type(&self) -> Option<&ReactionType> {
        self.reaction_type.as_ref()
    }

    /// Returns the message ID (if available).
    #[must_use]
    pub const fn message_id(&self) -> Option<i64> {
        self.message_id
    }

    /// Returns the story ID (if available).
    #[must_use]
    pub const fn story_id(&self) -> Option<StoryId> {
        match self.story_id {
            Some(id) => Some(id),
            None => None,
        }
    }

    /// Sets the reaction type.
    pub fn set_reaction_type(&mut self, reaction_type: ReactionType) {
        self.reaction_type = Some(reaction_type);
    }

    /// Sets the blocked status.
    pub fn set_blocked(&mut self, is_blocked: bool) {
        self.is_blocked = is_blocked;
    }

    /// Sets the blocked for stories status.
    pub fn set_blocked_for_stories(&mut self, is_blocked_for_stories: bool) {
        self.is_blocked_for_stories = is_blocked_for_stories;
    }

    /// Checks if this viewer is valid.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.actor_dialog_id.is_valid()
    }
}

impl fmt::Display for StoryViewer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StoryViewer {{ type: {:?}, dialog: {}, date: {} }}",
            self.viewer_type,
            self.actor_dialog_id.get(),
            self.date
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== ViewerType Tests ==========

    #[test]
    fn test_viewer_type_default() {
        assert_eq!(ViewerType::default(), ViewerType::None);
    }

    #[test]
    fn test_viewer_type_is_view() {
        assert!(ViewerType::View.is_view());
        assert!(!ViewerType::Forward.is_view());
        assert!(!ViewerType::Repost.is_view());
    }

    #[test]
    fn test_viewer_type_is_forward() {
        assert!(!ViewerType::View.is_forward());
        assert!(ViewerType::Forward.is_forward());
        assert!(!ViewerType::Repost.is_forward());
    }

    #[test]
    fn test_viewer_type_is_repost() {
        assert!(!ViewerType::View.is_repost());
        assert!(!ViewerType::Forward.is_repost());
        assert!(ViewerType::Repost.is_repost());
    }

    // ========== new_view Tests ==========

    #[test]
    fn test_new_view() {
        let viewer = StoryViewer::new_view(DialogId::new(123), 456);
        assert_eq!(viewer.actor_dialog_id().get(), 123);
        assert_eq!(viewer.date(), 456);
        assert!(viewer.viewer_type().is_view());
    }

    #[test]
    fn test_new_view_defaults() {
        let viewer = StoryViewer::new_view(DialogId::new(123), 456);
        assert!(!viewer.is_blocked());
        assert!(!viewer.is_blocked_for_stories());
        assert!(viewer.reaction_type().is_none());
        assert!(viewer.message_id().is_none());
        assert!(viewer.story_id().is_none());
    }

    // ========== new_forward Tests ==========

    #[test]
    fn test_new_forward() {
        let viewer = StoryViewer::new_forward(DialogId::new(123), 456, 789);
        assert_eq!(viewer.actor_dialog_id().get(), 123);
        assert_eq!(viewer.date(), 456);
        assert!(viewer.viewer_type().is_forward());
        assert_eq!(viewer.message_id(), Some(789));
    }

    // ========== new_repost Tests ==========

    #[test]
    fn test_new_repost() {
        let viewer = StoryViewer::new_repost(DialogId::new(123), 456, StoryId::new(789));
        assert_eq!(viewer.actor_dialog_id().get(), 123);
        assert_eq!(viewer.date(), 456);
        assert!(viewer.viewer_type().is_repost());
        assert_eq!(viewer.story_id(), Some(StoryId::new(789)));
    }

    // ========== accessor Tests ==========

    #[test]
    fn test_viewer_type() {
        let viewer = StoryViewer::new_view(DialogId::new(123), 456);
        assert_eq!(viewer.viewer_type(), ViewerType::View);
    }

    #[test]
    fn test_actor_dialog_id() {
        let viewer = StoryViewer::new_view(DialogId::new(123), 456);
        assert_eq!(viewer.actor_dialog_id(), DialogId::new(123));
    }

    #[test]
    fn test_date() {
        let viewer = StoryViewer::new_view(DialogId::new(123), 456);
        assert_eq!(viewer.date(), 456);
    }

    #[test]
    fn test_is_blocked() {
        let mut viewer = StoryViewer::new_view(DialogId::new(123), 456);
        assert!(!viewer.is_blocked());
        viewer.set_blocked(true);
        assert!(viewer.is_blocked());
    }

    #[test]
    fn test_is_blocked_for_stories() {
        let mut viewer = StoryViewer::new_view(DialogId::new(123), 456);
        assert!(!viewer.is_blocked_for_stories());
        viewer.set_blocked_for_stories(true);
        assert!(viewer.is_blocked_for_stories());
    }

    #[test]
    fn test_reaction_type_none() {
        let viewer = StoryViewer::new_view(DialogId::new(123), 456);
        assert!(viewer.reaction_type().is_none());
    }

    #[test]
    fn test_set_reaction_type() {
        let mut viewer = StoryViewer::new_view(DialogId::new(123), 456);
        let reaction = ReactionType::emoji("👍");
        viewer.set_reaction_type(reaction.clone());
        assert!(viewer.reaction_type().is_some());
    }

    #[test]
    fn test_story_id_none() {
        let viewer = StoryViewer::new_view(DialogId::new(123), 456);
        assert_eq!(viewer.story_id(), None);
    }

    // ========== is_valid Tests ==========

    #[test]
    fn test_is_valid_true() {
        let viewer = StoryViewer::new_view(DialogId::new(123), 456);
        assert!(viewer.is_valid());
    }

    #[test]
    fn test_is_valid_false() {
        let viewer = StoryViewer::new_view(DialogId::new(0), 456);
        assert!(!viewer.is_valid());
    }

    // ========== equality Tests ==========

    #[test]
    fn test_equality_same() {
        let viewer1 = StoryViewer::new_view(DialogId::new(123), 456);
        let viewer2 = StoryViewer::new_view(DialogId::new(123), 456);
        assert_eq!(viewer1, viewer2);
    }

    #[test]
    fn test_equality_different() {
        let viewer1 = StoryViewer::new_view(DialogId::new(123), 456);
        let viewer2 = StoryViewer::new_view(DialogId::new(789), 456);
        assert_ne!(viewer1, viewer2);
    }

    // ========== display Tests ==========

    #[test]
    fn test_display() {
        let viewer = StoryViewer::new_view(DialogId::new(123), 456);
        let display = format!("{}", viewer);
        assert!(display.contains("StoryViewer"));
    }

    #[test]
    fn test_debug() {
        let viewer = StoryViewer::new_view(DialogId::new(123), 456);
        let debug_str = format!("{:?}", viewer);
        assert!(debug_str.contains("StoryViewer"));
    }

    // ========== Serialization Tests ==========

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let viewer = StoryViewer::new_view(DialogId::new(123), 456);
        let json = serde_json::to_string(&viewer).unwrap();
        let deserialized: StoryViewer = serde_json::from_str(&json).unwrap();
        assert_eq!(viewer, deserialized);
    }
}
