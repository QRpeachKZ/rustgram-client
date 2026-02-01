// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Message viewer types for Telegram MTProto client.
//!
//! This module implements TDLib's MessageViewer and MessageViewers
//! from `td/telegram/MessageViewer.h`.
//!
//! # Overview
//!
//! Message viewers track who has viewed a message in a chat. This is particularly
//! useful for group chats where multiple users may have read the same message.

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_types::UserId;
use std::fmt;

/// A viewer of a message.
///
/// Represents a single user who has viewed a message, along with the timestamp
/// of when they viewed it.
///
/// # Example
///
/// ```
/// use rustgram_message_viewer::MessageViewer;
/// use rustgram_types::UserId;
///
/// let user_id = UserId::new(123).unwrap();
/// let viewer = MessageViewer::new(user_id, 1672531200);
///
/// assert_eq!(viewer.user_id(), user_id);
/// assert_eq!(viewer.date(), 1672531200);
/// assert!(!viewer.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageViewer {
    /// The user who viewed the message.
    user_id: UserId,
    /// Unix timestamp when the message was viewed (clamped to >= 0).
    date: i32,
}

impl MessageViewer {
    /// Creates a new MessageViewer.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user who viewed the message
    /// * `date` - Unix timestamp when viewed (will be clamped to >= 0)
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_viewer::MessageViewer;
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(456).unwrap();
    /// let viewer = MessageViewer::new(user_id, 1672531200);
    /// ```
    pub fn new(user_id: UserId, date: i32) -> Self {
        // Clamp date to be non-negative, following TDLib's behavior
        let date = if date < 0 { 0 } else { date };
        Self { user_id, date }
    }

    /// Returns the user ID of the viewer.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_viewer::MessageViewer;
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(789).unwrap();
    /// let viewer = MessageViewer::new(user_id, 0);
    /// assert_eq!(viewer.user_id(), user_id);
    /// ```
    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    /// Returns the date when the message was viewed.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_viewer::MessageViewer;
    /// use rustgram_types::UserId;
    ///
    /// let user_id = UserId::new(1).unwrap();
    /// let viewer = MessageViewer::new(user_id, 1234567890);
    /// assert_eq!(viewer.date(), 1234567890);
    /// ```
    pub fn date(&self) -> i32 {
        self.date
    }

    /// Checks if this viewer is empty (invalid user and zero date).
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_viewer::MessageViewer;
    /// use rustgram_types::UserId;
    ///
    /// let valid = MessageViewer::new(UserId::new(1).unwrap(), 100);
    /// assert!(!valid.is_empty());
    ///
    /// let empty = MessageViewer::new(UserId::default(), 0);
    /// assert!(empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.user_id == UserId::default() && self.date == 0
    }

    /// Sets a new date for the viewer.
    ///
    /// # Arguments
    ///
    /// * `date` - The new date (will be clamped to >= 0)
    pub fn with_date(mut self, date: i32) -> Self {
        self.date = if date < 0 { 0 } else { date };
        self
    }
}

impl fmt::Display for MessageViewer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MessageViewer(user={}, date={})",
            self.user_id, self.date
        )
    }
}

/// A collection of message viewers.
///
/// Represents all users who have viewed a particular message.
///
/// # Example
///
/// ```
/// use rustgram_message_viewer::{MessageViewer, MessageViewers};
/// use rustgram_types::UserId;
///
/// let mut viewers = MessageViewers::new();
///
/// let user1 = UserId::new(1).unwrap();
/// let user2 = UserId::new(2).unwrap();
///
/// viewers.add_viewer(MessageViewer::new(user1, 100));
/// viewers.add_viewer(MessageViewer::new(user2, 101));
///
/// assert_eq!(viewers.len(), 2);
/// assert!(!viewers.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MessageViewers {
    /// The list of viewers.
    viewers: Vec<MessageViewer>,
}

impl MessageViewers {
    /// Creates a new empty MessageViewers collection.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_viewer::MessageViewers;
    ///
    /// let viewers = MessageViewers::new();
    /// assert!(viewers.is_empty());
    /// assert_eq!(viewers.len(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            viewers: Vec::new(),
        }
    }

    /// Creates a new MessageViewers with the specified viewers.
    ///
    /// # Arguments
    ///
    /// * `viewers` - The initial list of viewers
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_viewer::{MessageViewer, MessageViewers};
    /// use rustgram_types::UserId;
    ///
    /// let viewers_list = vec![
    ///     MessageViewer::new(UserId::new(1).unwrap(), 100),
    ///     MessageViewer::new(UserId::new(2).unwrap(), 101),
    /// ];
    ///
    /// let viewers = MessageViewers::with_viewers(viewers_list);
    /// assert_eq!(viewers.len(), 2);
    /// ```
    pub fn with_viewers(viewers: Vec<MessageViewer>) -> Self {
        Self { viewers }
    }

    /// Adds a viewer to the collection.
    ///
    /// # Arguments
    ///
    /// * `viewer` - The viewer to add
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_viewer::{MessageViewer, MessageViewers};
    /// use rustgram_types::UserId;
    ///
    /// let mut viewers = MessageViewers::new();
    /// viewers.add_viewer(MessageViewer::new(UserId::new(1).unwrap(), 100));
    /// assert_eq!(viewers.len(), 1);
    /// ```
    pub fn add_viewer(&mut self, viewer: MessageViewer) {
        self.viewers.push(viewer);
    }

    /// Returns a slice of all viewers.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_viewer::{MessageViewer, MessageViewers};
    /// use rustgram_types::UserId;
    ///
    /// let viewers = MessageViewers::with_viewers(vec![
    ///     MessageViewer::new(UserId::new(1).unwrap(), 100),
    ///     MessageViewer::new(UserId::new(2).unwrap(), 101),
    /// ]);
    ///
    /// assert_eq!(viewers.viewers().len(), 2);
    /// ```
    pub fn viewers(&self) -> &[MessageViewer] {
        &self.viewers
    }

    /// Returns the user IDs of all viewers.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_viewer::{MessageViewer, MessageViewers};
    /// use rustgram_types::UserId;
    ///
    /// let viewers = MessageViewers::with_viewers(vec![
    ///     MessageViewer::new(UserId::new(1).unwrap(), 100),
    ///     MessageViewer::new(UserId::new(2).unwrap(), 101),
    /// ]);
    ///
    /// let user_ids = viewers.get_user_ids();
    /// assert_eq!(user_ids.len(), 2);
    /// assert!(user_ids.contains(&UserId::new(1).unwrap()));
    /// ```
    pub fn get_user_ids(&self) -> Vec<UserId> {
        self.viewers.iter().map(|v| v.user_id).collect()
    }

    /// Checks if the collection is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_viewer::MessageViewers;
    ///
    /// let empty = MessageViewers::new();
    /// assert!(empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.viewers.is_empty()
    }

    /// Returns the number of viewers.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_viewer::{MessageViewer, MessageViewers};
    /// use rustgram_types::UserId;
    ///
    /// let viewers = MessageViewers::with_viewers(vec![
    ///     MessageViewer::new(UserId::new(1).unwrap(), 100),
    /// ]);
    /// assert_eq!(viewers.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.viewers.len()
    }

    /// Clears all viewers from the collection.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_viewer::{MessageViewer, MessageViewers};
    /// use rustgram_types::UserId;
    ///
    /// let mut viewers = MessageViewers::with_viewers(vec![
    ///     MessageViewer::new(UserId::new(1).unwrap(), 100),
    /// ]);
    /// viewers.clear();
    /// assert!(viewers.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.viewers.clear();
    }

    /// Removes the viewer at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the viewer to remove
    ///
    /// # Returns
    ///
    /// * `Some(viewer)` - If the index was valid
    /// * `None` - If the index was out of bounds
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_viewer::{MessageViewer, MessageViewers};
    /// use rustgram_types::UserId;
    ///
    /// let mut viewers = MessageViewers::with_viewers(vec![
    ///     MessageViewer::new(UserId::new(1).unwrap(), 100),
    /// ]);
    ///
    /// let removed = viewers.remove(0);
    /// assert!(removed.is_some());
    /// assert!(viewers.is_empty());
    /// ```
    pub fn remove(&mut self, index: usize) -> Option<MessageViewer> {
        if index < self.viewers.len() {
            Some(self.viewers.remove(index))
        } else {
            None
        }
    }

    /// Returns an iterator over the viewers.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_message_viewer::{MessageViewer, MessageViewers};
    /// use rustgram_types::UserId;
    ///
    /// let viewers = MessageViewers::with_viewers(vec![
    ///     MessageViewer::new(UserId::new(1).unwrap(), 100),
    ///     MessageViewer::new(UserId::new(2).unwrap(), 101),
    /// ]);
    ///
    /// let mut iter = viewers.iter();
    /// assert!(iter.next().is_some());
    /// assert!(iter.next().is_some());
    /// assert!(iter.next().is_none());
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &MessageViewer> {
        self.viewers.iter()
    }
}

impl fmt::Display for MessageViewers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MessageViewers(count={})", self.viewers.len())
    }
}

impl IntoIterator for MessageViewers {
    type Item = MessageViewer;
    type IntoIter = std::vec::IntoIter<MessageViewer>;

    fn into_iter(self) -> Self::IntoIter {
        self.viewers.into_iter()
    }
}

impl<'a> IntoIterator for &'a MessageViewers {
    type Item = &'a MessageViewer;
    type IntoIter = std::slice::Iter<'a, MessageViewer>;

    fn into_iter(self) -> Self::IntoIter {
        self.viewers.iter()
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-message-viewer";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-message-viewer");
    }

    // MessageViewer tests
    #[test]
    fn test_viewer_new() {
        let user_id = UserId::new(123).unwrap();
        let viewer = MessageViewer::new(user_id, 1672531200);

        assert_eq!(viewer.user_id(), user_id);
        assert_eq!(viewer.date(), 1672531200);
    }

    #[test]
    fn test_viewer_date_clamping() {
        let user_id = UserId::new(1).unwrap();

        // Negative date should be clamped to 0
        let viewer = MessageViewer::new(user_id, -100);
        assert_eq!(viewer.date(), 0);

        // Zero date should remain zero
        let viewer = MessageViewer::new(user_id, 0);
        assert_eq!(viewer.date(), 0);

        // Positive date should remain unchanged
        let viewer = MessageViewer::new(user_id, 100);
        assert_eq!(viewer.date(), 100);
    }

    #[test]
    fn test_viewer_is_empty() {
        let valid_user = UserId::new(1).unwrap();
        let viewer = MessageViewer::new(valid_user, 100);
        assert!(!viewer.is_empty());

        let empty_viewer = MessageViewer::new(UserId::default(), 0);
        assert!(empty_viewer.is_empty());

        // Invalid user with non-zero date is not empty
        let invalid_user_viewer = MessageViewer::new(UserId::default(), 100);
        assert!(!invalid_user_viewer.is_empty());

        // Valid user with zero date is not empty
        let zero_date_viewer = MessageViewer::new(valid_user, 0);
        assert!(!zero_date_viewer.is_empty());
    }

    #[test]
    fn test_viewer_with_date() {
        let user_id = UserId::new(1).unwrap();
        let viewer = MessageViewer::new(user_id, 100).with_date(200);

        assert_eq!(viewer.date(), 200);
    }

    #[test]
    fn test_viewer_with_date_clamping() {
        let user_id = UserId::new(1).unwrap();
        let viewer = MessageViewer::new(user_id, 100).with_date(-50);

        assert_eq!(viewer.date(), 0);
    }

    #[test]
    fn test_viewer_display() {
        let user_id = UserId::new(1).unwrap();
        let viewer = MessageViewer::new(user_id, 1234567890);

        let display = format!("{viewer}");
        assert!(display.contains("user"));
        assert!(display.contains("1234567890"));
    }

    #[test]
    fn test_viewer_clone() {
        let user_id = UserId::new(1).unwrap();
        let viewer1 = MessageViewer::new(user_id, 100);
        let viewer2 = viewer1.clone();

        assert_eq!(viewer1, viewer2);
    }

    #[test]
    fn test_viewer_eq() {
        let user_id = UserId::new(1).unwrap();
        let viewer1 = MessageViewer::new(user_id, 100);
        let viewer2 = MessageViewer::new(user_id, 100);

        assert_eq!(viewer1, viewer2);

        let viewer3 = MessageViewer::new(user_id, 200);
        assert_ne!(viewer1, viewer3);
    }

    // MessageViewers tests
    #[test]
    fn test_viewers_new() {
        let viewers = MessageViewers::new();

        assert!(viewers.is_empty());
        assert_eq!(viewers.len(), 0);
        assert_eq!(viewers.viewers().len(), 0);
    }

    #[test]
    fn test_viewers_default() {
        let viewers = MessageViewers::default();

        assert!(viewers.is_empty());
        assert_eq!(viewers.len(), 0);
    }

    #[test]
    fn test_viewers_with_viewers() {
        let viewers_list = vec![
            MessageViewer::new(UserId::new(1).unwrap(), 100),
            MessageViewer::new(UserId::new(2).unwrap(), 101),
            MessageViewer::new(UserId::new(3).unwrap(), 102),
        ];

        let viewers = MessageViewers::with_viewers(viewers_list.clone());
        assert_eq!(viewers.len(), 3);
        assert_eq!(viewers.viewers(), &viewers_list);
    }

    #[test]
    fn test_viewers_add_viewer() {
        let mut viewers = MessageViewers::new();

        viewers.add_viewer(MessageViewer::new(UserId::new(1).unwrap(), 100));
        assert_eq!(viewers.len(), 1);

        viewers.add_viewer(MessageViewer::new(UserId::new(2).unwrap(), 101));
        assert_eq!(viewers.len(), 2);
    }

    #[test]
    fn test_viewers_get_user_ids() {
        let viewers = MessageViewers::with_viewers(vec![
            MessageViewer::new(UserId::new(1).unwrap(), 100),
            MessageViewer::new(UserId::new(2).unwrap(), 101),
            MessageViewer::new(UserId::new(3).unwrap(), 102),
        ]);

        let user_ids = viewers.get_user_ids();
        assert_eq!(user_ids.len(), 3);
        assert!(user_ids.contains(&UserId::new(1).unwrap()));
        assert!(user_ids.contains(&UserId::new(2).unwrap()));
        assert!(user_ids.contains(&UserId::new(3).unwrap()));
    }

    #[test]
    fn test_viewers_clear() {
        let mut viewers =
            MessageViewers::with_viewers(vec![MessageViewer::new(UserId::new(1).unwrap(), 100)]);

        assert_eq!(viewers.len(), 1);
        viewers.clear();
        assert!(viewers.is_empty());
        assert_eq!(viewers.len(), 0);
    }

    #[test]
    fn test_viewers_remove() {
        let mut viewers = MessageViewers::with_viewers(vec![
            MessageViewer::new(UserId::new(1).unwrap(), 100),
            MessageViewer::new(UserId::new(2).unwrap(), 101),
            MessageViewer::new(UserId::new(3).unwrap(), 102),
        ]);

        // Remove middle viewer
        let removed = viewers.remove(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().user_id(), UserId::new(2).unwrap());
        assert_eq!(viewers.len(), 2);

        // Try to remove out of bounds
        assert!(viewers.remove(10).is_none());

        // Remove first viewer
        let removed = viewers.remove(0);
        assert!(removed.is_some());
        assert_eq!(viewers.len(), 1);
    }

    #[test]
    fn test_viewers_iter() {
        let viewers = MessageViewers::with_viewers(vec![
            MessageViewer::new(UserId::new(1).unwrap(), 100),
            MessageViewer::new(UserId::new(2).unwrap(), 101),
        ]);

        let mut iter = viewers.iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_viewers_display() {
        let viewers =
            MessageViewers::with_viewers(vec![MessageViewer::new(UserId::new(1).unwrap(), 100)]);

        let display = format!("{viewers}");
        assert!(display.contains("count=1"));
    }

    #[test]
    fn test_viewers_clone() {
        let viewers1 =
            MessageViewers::with_viewers(vec![MessageViewer::new(UserId::new(1).unwrap(), 100)]);
        let viewers2 = viewers1.clone();

        assert_eq!(viewers1, viewers2);
    }

    #[test]
    fn test_viewers_eq() {
        let viewers_list = vec![
            MessageViewer::new(UserId::new(1).unwrap(), 100),
            MessageViewer::new(UserId::new(2).unwrap(), 101),
        ];

        let viewers1 = MessageViewers::with_viewers(viewers_list.clone());
        let viewers2 = MessageViewers::with_viewers(viewers_list);

        assert_eq!(viewers1, viewers2);
    }

    #[test]
    fn test_viewers_into_iter() {
        let viewers = MessageViewers::with_viewers(vec![
            MessageViewer::new(UserId::new(1).unwrap(), 100),
            MessageViewer::new(UserId::new(2).unwrap(), 101),
        ]);

        let count = viewers.into_iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_viewers_ref_iter() {
        let viewers = MessageViewers::with_viewers(vec![
            MessageViewer::new(UserId::new(1).unwrap(), 100),
            MessageViewer::new(UserId::new(2).unwrap(), 101),
        ]);

        let count = (&viewers).into_iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_viewers_for_loop() {
        let viewers = MessageViewers::with_viewers(vec![
            MessageViewer::new(UserId::new(1).unwrap(), 100),
            MessageViewer::new(UserId::new(2).unwrap(), 101),
            MessageViewer::new(UserId::new(3).unwrap(), 102),
        ]);

        let mut count = 0;
        for viewer in &viewers {
            assert!(viewer.user_id().is_valid());
            count += 1;
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn test_empty_viewers_user_ids() {
        let viewers = MessageViewers::new();
        let user_ids = viewers.get_user_ids();
        assert_eq!(user_ids.len(), 0);
    }

    #[test]
    fn test_message_viewer_i32_max_date() {
        let user_id = UserId::new(1).unwrap();
        let viewer = MessageViewer::new(user_id, i32::MAX);
        assert_eq!(viewer.date(), i32::MAX);
    }
}
