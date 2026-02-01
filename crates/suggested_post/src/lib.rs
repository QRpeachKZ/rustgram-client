// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Suggested Post
//!
//! Suggested post information for Telegram.
//!
//! ## Overview
//!
//! This module provides the [`SuggestedPost`] struct, which represents
//! a suggested post with pricing and state information.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_suggested_post::{SuggestedPost, SuggestedPostState};
//! use rustgram_suggested_post_price::SuggestedPostPrice;
//!
//! // Create a suggested post
//! let post = SuggestedPost::new(SuggestedPostPrice::stars(100), 1704067200);
//! assert!(post.is_pending());
//! ```

use std::fmt;

use rustgram_suggested_post_price::SuggestedPostPrice;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// State of a suggested post.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib's `suggestedPostState*` types.
///
/// # Example
///
/// ```rust
/// use rustgram_suggested_post::SuggestedPostState;
///
/// let state = SuggestedPostState::Pending;
/// assert!(state.is_pending());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SuggestedPostState {
    /// Post is pending approval/rejection.
    #[default]
    Pending,
    /// Post has been approved.
    Approved,
    /// Post has been declined.
    Declined,
}

impl SuggestedPostState {
    /// Checks if the post is pending.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::SuggestedPostState;
    ///
    /// assert!(SuggestedPostState::Pending.is_pending());
    /// assert!(!SuggestedPostState::Approved.is_pending());
    /// ```
    #[must_use]
    pub const fn is_pending(&self) -> bool {
        matches!(self, Self::Pending)
    }

    /// Checks if the post is approved.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::SuggestedPostState;
    ///
    /// assert!(SuggestedPostState::Approved.is_approved());
    /// assert!(!SuggestedPostState::Pending.is_approved());
    /// ```
    #[must_use]
    pub const fn is_approved(&self) -> bool {
        matches!(self, Self::Approved)
    }

    /// Checks if the post is declined.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::SuggestedPostState;
    ///
    /// assert!(SuggestedPostState::Declined.is_declined());
    /// assert!(!SuggestedPostState::Pending.is_declined());
    /// ```
    #[must_use]
    pub const fn is_declined(&self) -> bool {
        matches!(self, Self::Declined)
    }
}

/// Suggested post information.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib's `SuggestedPost` class.
///
/// # Example
///
/// ```rust
/// use rustgram_suggested_post::SuggestedPost;
/// use rustgram_suggested_post_price::SuggestedPostPrice;
///
/// let post = SuggestedPost::new(SuggestedPostPrice::stars(100), 1704067200);
/// assert!(post.is_pending());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SuggestedPost {
    price: SuggestedPostPrice,
    schedule_date: i32,
    state: SuggestedPostState,
}

impl SuggestedPost {
    /// Creates a new suggested post.
    ///
    /// # Arguments
    ///
    /// * `price` - The price for the suggested post
    /// * `schedule_date` - The scheduled date (Unix timestamp)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::SuggestedPost;
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let post = SuggestedPost::new(SuggestedPostPrice::stars(100), 1704067200);
    /// assert_eq!(post.price(), &SuggestedPostPrice::stars(100));
    /// ```
    #[must_use]
    pub const fn new(price: SuggestedPostPrice, schedule_date: i32) -> Self {
        Self {
            price,
            schedule_date,
            state: SuggestedPostState::Pending,
        }
    }

    /// Returns the price for the suggested post.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::SuggestedPost;
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let post = SuggestedPost::new(SuggestedPostPrice::stars(100), 0);
    /// assert_eq!(post.price(), &SuggestedPostPrice::stars(100));
    /// ```
    #[must_use]
    pub const fn price(&self) -> &SuggestedPostPrice {
        &self.price
    }

    /// Returns the schedule date.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::SuggestedPost;
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let post = SuggestedPost::new(SuggestedPostPrice::stars(100), 1704067200);
    /// assert_eq!(post.schedule_date(), 1704067200);
    /// ```
    #[must_use]
    pub const fn schedule_date(&self) -> i32 {
        self.schedule_date
    }

    /// Returns the state of the suggested post.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::{SuggestedPost, SuggestedPostState};
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let post = SuggestedPost::new(SuggestedPostPrice::stars(100), 0);
    /// assert_eq!(post.state(), SuggestedPostState::Pending);
    /// ```
    #[must_use]
    pub const fn state(&self) -> SuggestedPostState {
        self.state
    }

    /// Checks if the post is pending.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::SuggestedPost;
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let post = SuggestedPost::new(SuggestedPostPrice::stars(100), 0);
    /// assert!(post.is_pending());
    /// ```
    #[must_use]
    pub const fn is_pending(&self) -> bool {
        self.state.is_pending()
    }

    /// Checks if the post is approved.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::{SuggestedPost, SuggestedPostState};
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let mut post = SuggestedPost::new(SuggestedPostPrice::stars(100), 0);
    /// post.set_state(SuggestedPostState::Approved);
    /// assert!(post.is_approved());
    /// ```
    #[must_use]
    pub const fn is_approved(&self) -> bool {
        self.state.is_approved()
    }

    /// Checks if the post is declined.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::{SuggestedPost, SuggestedPostState};
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let mut post = SuggestedPost::new(SuggestedPostPrice::stars(100), 0);
    /// post.set_state(SuggestedPostState::Declined);
    /// assert!(post.is_declined());
    /// ```
    #[must_use]
    pub const fn is_declined(&self) -> bool {
        self.state.is_declined()
    }

    /// Sets the state of the suggested post.
    ///
    /// # Arguments
    ///
    /// * `state` - The new state
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::{SuggestedPost, SuggestedPostState};
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let mut post = SuggestedPost::new(SuggestedPostPrice::stars(100), 0);
    /// post.set_state(SuggestedPostState::Approved);
    /// assert!(post.is_approved());
    /// ```
    pub fn set_state(&mut self, state: SuggestedPostState) {
        self.state = state;
    }

    /// Checks if this post can be accepted.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::SuggestedPost;
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let post = SuggestedPost::new(SuggestedPostPrice::stars(100), 0);
    /// assert!(post.can_be_accepted());
    /// ```
    #[must_use]
    pub const fn can_be_accepted(&self) -> bool {
        self.state.is_pending()
    }

    /// Checks if this post can be rejected.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::SuggestedPost;
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let post = SuggestedPost::new(SuggestedPostPrice::stars(100), 0);
    /// assert!(post.can_be_rejected());
    /// ```
    #[must_use]
    pub const fn can_be_rejected(&self) -> bool {
        self.state.is_pending()
    }
}

impl Default for SuggestedPost {
    /// Creates a default suggested post (no price, no date, pending).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::SuggestedPost;
    ///
    /// let post = SuggestedPost::default();
    /// assert!(post.price().is_empty());
    /// assert_eq!(post.schedule_date(), 0);
    /// ```
    fn default() -> Self {
        Self {
            price: SuggestedPostPrice::none(),
            schedule_date: 0,
            state: SuggestedPostState::Pending,
        }
    }
}

impl fmt::Display for SuggestedPost {
    /// Formats the suggested post for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post::SuggestedPost;
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let post = SuggestedPost::new(SuggestedPostPrice::stars(100), 1704067200);
    /// let s = format!("{}", post);
    /// assert!(s.contains("SuggestedPost"));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SuggestedPost(price: {}, date: {}, state: {:?})",
            self.price, self.schedule_date, self.state
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let post = SuggestedPost::new(SuggestedPostPrice::stars(100), 1704067200);
        assert_eq!(post.price(), &SuggestedPostPrice::stars(100));
        assert_eq!(post.schedule_date(), 1704067200);
        assert_eq!(post.state(), SuggestedPostState::Pending);
    }

    #[test]
    fn test_price() {
        let post = SuggestedPost::new(SuggestedPostPrice::ton(5000), 0);
        assert_eq!(post.price(), &SuggestedPostPrice::ton(5000));
    }

    #[test]
    fn test_schedule_date() {
        let post = SuggestedPost::new(SuggestedPostPrice::none(), 12345);
        assert_eq!(post.schedule_date(), 12345);
    }

    #[test]
    fn test_state() {
        let post = SuggestedPost::new(SuggestedPostPrice::none(), 0);
        assert_eq!(post.state(), SuggestedPostState::Pending);
    }

    #[test]
    fn test_is_pending() {
        let post = SuggestedPost::new(SuggestedPostPrice::none(), 0);
        assert!(post.is_pending());
    }

    #[test]
    fn test_is_approved() {
        let mut post = SuggestedPost::new(SuggestedPostPrice::none(), 0);
        post.set_state(SuggestedPostState::Approved);
        assert!(post.is_approved());
    }

    #[test]
    fn test_is_declined() {
        let mut post = SuggestedPost::new(SuggestedPostPrice::none(), 0);
        post.set_state(SuggestedPostState::Declined);
        assert!(post.is_declined());
    }

    #[test]
    fn test_set_state() {
        let mut post = SuggestedPost::new(SuggestedPostPrice::none(), 0);
        post.set_state(SuggestedPostState::Approved);
        assert_eq!(post.state(), SuggestedPostState::Approved);
    }

    #[test]
    fn test_can_be_accepted() {
        let mut post = SuggestedPost::new(SuggestedPostPrice::none(), 0);
        assert!(post.can_be_accepted());

        post.set_state(SuggestedPostState::Approved);
        assert!(!post.can_be_accepted());
    }

    #[test]
    fn test_can_be_rejected() {
        let mut post = SuggestedPost::new(SuggestedPostPrice::none(), 0);
        assert!(post.can_be_rejected());

        post.set_state(SuggestedPostState::Declined);
        assert!(!post.can_be_rejected());
    }

    #[test]
    fn test_default() {
        let post = SuggestedPost::default();
        assert!(post.price().is_empty());
        assert_eq!(post.schedule_date(), 0);
        assert_eq!(post.state(), SuggestedPostState::Pending);
    }

    #[test]
    fn test_equality() {
        let post1 = SuggestedPost::new(SuggestedPostPrice::stars(100), 1704067200);
        let post2 = SuggestedPost::new(SuggestedPostPrice::stars(100), 1704067200);
        assert_eq!(post1, post2);
    }

    #[test]
    fn test_inequality_price() {
        let post1 = SuggestedPost::new(SuggestedPostPrice::stars(100), 0);
        let post2 = SuggestedPost::new(SuggestedPostPrice::stars(200), 0);
        assert_ne!(post1, post2);
    }

    #[test]
    fn test_inequality_date() {
        let post1 = SuggestedPost::new(SuggestedPostPrice::none(), 100);
        let post2 = SuggestedPost::new(SuggestedPostPrice::none(), 200);
        assert_ne!(post1, post2);
    }

    #[test]
    fn test_inequality_state() {
        let mut post1 = SuggestedPost::new(SuggestedPostPrice::none(), 0);
        post1.set_state(SuggestedPostState::Approved);
        let post2 = SuggestedPost::new(SuggestedPostPrice::none(), 0);
        assert_ne!(post1, post2);
    }

    #[test]
    fn test_clone_semantics() {
        let post1 = SuggestedPost::new(SuggestedPostPrice::stars(100), 1704067200);
        let post2 = post1.clone();
        assert_eq!(post1, post2);
    }

    #[test]
    fn test_display_format() {
        let post = SuggestedPost::new(SuggestedPostPrice::stars(100), 1704067200);
        let s = format!("{}", post);
        assert!(s.contains("SuggestedPost"));
    }

    #[test]
    fn test_debug_format() {
        let post = SuggestedPost::new(SuggestedPostPrice::stars(100), 1704067200);
        let debug_str = format!("{:?}", post);
        assert!(debug_str.contains("SuggestedPost"));
    }

    #[test]
    fn test_state_pending() {
        assert!(SuggestedPostState::Pending.is_pending());
        assert!(!SuggestedPostState::Pending.is_approved());
        assert!(!SuggestedPostState::Pending.is_declined());
    }

    #[test]
    fn test_state_approved() {
        assert!(!SuggestedPostState::Approved.is_pending());
        assert!(SuggestedPostState::Approved.is_approved());
        assert!(!SuggestedPostState::Approved.is_declined());
    }

    #[test]
    fn test_state_declined() {
        assert!(!SuggestedPostState::Declined.is_pending());
        assert!(!SuggestedPostState::Declined.is_approved());
        assert!(SuggestedPostState::Declined.is_declined());
    }

    #[test]
    fn test_state_default() {
        let state = SuggestedPostState::default();
        assert_eq!(state, SuggestedPostState::Pending);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let original = SuggestedPost::new(SuggestedPostPrice::stars(100), 1704067200);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: SuggestedPost = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_state() {
        let original = SuggestedPostState::Approved;
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: SuggestedPostState = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
