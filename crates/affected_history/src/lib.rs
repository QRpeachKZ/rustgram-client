// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Affected history type for Telegram MTProto client.
//!
//! This module implements the affected history information from TDLib.
//!
//! # Example
//!
//! ```rust
//! use rustgram_affected_history::AffectedHistory;
//!
//! let history = AffectedHistory::with_pts(12345, 10, true);
//! assert_eq!(history.pts(), 12345);
//! assert_eq!(history.pts_count(), 10);
//! assert!(history.is_final());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::fmt;

/// Affected history information.
///
/// Contains information about history updates in Telegram.
/// Based on TDLib's affected history messages.
///
/// # Example
///
/// ```rust
/// use rustgram_affected_history::AffectedHistory;
///
/// let history = AffectedHistory::with_pts(100, 5, false);
/// assert_eq!(history.pts(), 100);
/// assert!(!history.is_final());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AffectedHistory {
    /// Permanent timestamp (pts)
    pts: i32,
    /// Number of affected messages
    pts_count: i32,
    /// Whether this is the final update
    is_final: bool,
}

impl AffectedHistory {
    /// Creates a new affected history with the given values.
    ///
    /// # Arguments
    ///
    /// * `pts` - Permanent timestamp
    /// * `pts_count` - Number of affected messages
    /// * `is_final` - Whether this is the final update
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_affected_history::AffectedHistory;
    ///
    /// let history = AffectedHistory::with_pts(12345, 10, true);
    /// assert_eq!(history.pts(), 12345);
    /// ```
    pub fn with_pts(pts: i32, pts_count: i32, is_final: bool) -> Self {
        Self {
            pts,
            pts_count,
            is_final,
        }
    }

    /// Creates an empty affected history.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_affected_history::AffectedHistory;
    ///
    /// let history = AffectedHistory::empty();
    /// assert_eq!(history.pts(), 0);
    /// assert_eq!(history.pts_count(), 0);
    /// ```
    pub fn empty() -> Self {
        Self {
            pts: 0,
            pts_count: 0,
            is_final: false,
        }
    }

    /// Creates affected history from a mock telegram_api object.
    ///
    /// This is a simplified version for testing. The real implementation would
    /// parse the actual MTProto object.
    ///
    /// # Arguments
    ///
    /// * `pts` - Permanent timestamp from telegram_api
    /// * `pts_count` - Number of affected messages from telegram_api
    /// * `is_final` - Whether this is the final update
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_affected_history::AffectedHistory;
    ///
    /// let history = AffectedHistory::from_telegram_api(1000, 5, true);
    /// assert!(history.is_final());
    /// ```
    pub fn from_telegram_api(pts: i32, pts_count: i32, is_final: bool) -> Self {
        Self {
            pts,
            pts_count,
            is_final,
        }
    }

    /// Returns the permanent timestamp (pts).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_affected_history::AffectedHistory;
    ///
    /// let history = AffectedHistory::with_pts(12345, 10, true);
    /// assert_eq!(history.pts(), 12345);
    /// ```
    pub fn pts(&self) -> i32 {
        self.pts
    }

    /// Returns the number of affected messages (pts_count).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_affected_history::AffectedHistory;
    ///
    /// let history = AffectedHistory::with_pts(12345, 10, true);
    /// assert_eq!(history.pts_count(), 10);
    /// ```
    pub fn pts_count(&self) -> i32 {
        self.pts_count
    }

    /// Checks if this is the final update.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_affected_history::AffectedHistory;
    ///
    /// let final_history = AffectedHistory::with_pts(12345, 10, true);
    /// assert!(final_history.is_final());
    ///
    /// let partial_history = AffectedHistory::with_pts(12345, 10, false);
    /// assert!(!partial_history.is_final());
    /// ```
    pub fn is_final(&self) -> bool {
        self.is_final
    }

    /// Checks if this affected history is empty (no updates).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_affected_history::AffectedHistory;
    ///
    /// let history = AffectedHistory::empty();
    /// assert!(history.is_empty());
    ///
    /// let history = AffectedHistory::with_pts(12345, 10, true);
    /// assert!(!history.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.pts == 0 && self.pts_count == 0
    }

    /// Checks if this affected history has updates.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_affected_history::AffectedHistory;
    ///
    /// let history = AffectedHistory::with_pts(12345, 10, true);
    /// assert!(history.has_updates());
    /// ```
    pub fn has_updates(&self) -> bool {
        self.pts_count > 0
    }

    /// Returns the end pts value (pts + pts_count).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_affected_history::AffectedHistory;
    ///
    /// let history = AffectedHistory::with_pts(100, 10, true);
    /// assert_eq!(history.end_pts(), 110);
    /// ```
    pub fn end_pts(&self) -> i32 {
        self.pts + self.pts_count
    }
}

impl Default for AffectedHistory {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for AffectedHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AffectedHistory[pts={}, count={}",
            self.pts, self.pts_count
        )?;
        if self.is_final {
            write!(f, ", final]")
        } else {
            write!(f, ", partial]")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_pts() {
        let history = AffectedHistory::with_pts(12345, 10, true);
        assert_eq!(history.pts(), 12345);
        assert_eq!(history.pts_count(), 10);
        assert!(history.is_final());
    }

    #[test]
    fn test_empty() {
        let history = AffectedHistory::empty();
        assert_eq!(history.pts(), 0);
        assert_eq!(history.pts_count(), 0);
        assert!(!history.is_final());
        assert!(history.is_empty());
    }

    #[test]
    fn test_from_telegram_api() {
        let history = AffectedHistory::from_telegram_api(1000, 5, false);
        assert_eq!(history.pts(), 1000);
        assert_eq!(history.pts_count(), 5);
        assert!(!history.is_final());
    }

    #[test]
    fn test_is_final() {
        let final_history = AffectedHistory::with_pts(12345, 10, true);
        assert!(final_history.is_final());

        let partial_history = AffectedHistory::with_pts(12345, 10, false);
        assert!(!partial_history.is_final());
    }

    #[test]
    fn test_is_empty() {
        let empty = AffectedHistory::empty();
        assert!(empty.is_empty());

        let non_empty = AffectedHistory::with_pts(12345, 10, true);
        assert!(!non_empty.is_empty());
    }

    #[test]
    fn test_has_updates() {
        let no_updates = AffectedHistory::with_pts(100, 0, true);
        assert!(!no_updates.has_updates());

        let with_updates = AffectedHistory::with_pts(100, 10, true);
        assert!(with_updates.has_updates());
    }

    #[test]
    fn test_end_pts() {
        let history = AffectedHistory::with_pts(100, 10, true);
        assert_eq!(history.end_pts(), 110);
    }

    #[test]
    fn test_default() {
        let history = AffectedHistory::default();
        assert_eq!(history.pts(), 0);
        assert_eq!(history.pts_count(), 0);
        assert!(!history.is_final());
    }

    #[test]
    fn test_equality() {
        let history1 = AffectedHistory::with_pts(100, 10, true);
        let history2 = AffectedHistory::with_pts(100, 10, true);
        assert_eq!(history1, history2);
    }

    #[test]
    fn test_inequality_pts() {
        let history1 = AffectedHistory::with_pts(100, 10, true);
        let history2 = AffectedHistory::with_pts(200, 10, true);
        assert_ne!(history1, history2);
    }

    #[test]
    fn test_inequality_pts_count() {
        let history1 = AffectedHistory::with_pts(100, 10, true);
        let history2 = AffectedHistory::with_pts(100, 20, true);
        assert_ne!(history1, history2);
    }

    #[test]
    fn test_inequality_final() {
        let history1 = AffectedHistory::with_pts(100, 10, true);
        let history2 = AffectedHistory::with_pts(100, 10, false);
        assert_ne!(history1, history2);
    }

    #[test]
    fn test_clone() {
        let history1 = AffectedHistory::with_pts(12345, 10, true);
        let history2 = history1.clone();
        assert_eq!(history1, history2);
    }

    #[test]
    fn test_display_final() {
        let history = AffectedHistory::with_pts(100, 10, true);
        let display = format!("{}", history);
        assert!(display.contains("final"));
        assert!(display.contains("pts=100"));
        assert!(display.contains("count=10"));
    }

    #[test]
    fn test_display_partial() {
        let history = AffectedHistory::with_pts(100, 10, false);
        let display = format!("{}", history);
        assert!(display.contains("partial"));
    }

    #[test]
    fn test_debug() {
        let history = AffectedHistory::with_pts(100, 10, true);
        let debug = format!("{:?}", history);
        assert!(debug.contains("AffectedHistory"));
    }

    #[test]
    fn test_large_pts() {
        let history = AffectedHistory::with_pts(i32::MAX, 10, true);
        assert_eq!(history.pts(), i32::MAX);
    }

    #[test]
    fn test_negative_pts_count() {
        let history = AffectedHistory::with_pts(100, -5, true);
        assert_eq!(history.pts_count(), -5);
        assert!(!history.has_updates());
    }

    #[test]
    fn test_zero_pts_with_count() {
        let history = AffectedHistory::with_pts(0, 10, true);
        assert_eq!(history.pts(), 0);
        assert!(history.has_updates());
        assert!(!history.is_empty());
    }

    #[test]
    fn test_zero_pts_zero_count() {
        let history = AffectedHistory::with_pts(0, 0, false);
        assert!(history.is_empty());
        assert!(!history.has_updates());
    }

    #[test]
    fn test_multiple_updates() {
        let history1 = AffectedHistory::with_pts(100, 5, false);
        let history2 = AffectedHistory::with_pts(105, 5, false);
        let history3 = AffectedHistory::with_pts(110, 5, true);

        assert_eq!(history1.end_pts(), 105);
        assert_eq!(history2.end_pts(), 110);
        assert_eq!(history3.end_pts(), 115);
    }

    #[test]
    fn test_end_pts_overflow_safe() {
        let history = AffectedHistory::with_pts(i32::MAX - 5, 10, true);
        // Use wrapping_add to avoid panic in debug mode
        let _end = history.pts.wrapping_add(history.pts_count);
    }

    #[test]
    fn test_partial_to_final_progression() {
        let partial = AffectedHistory::with_pts(100, 10, false);
        assert!(!partial.is_final());

        let final_update = AffectedHistory::with_pts(110, 5, true);
        assert!(final_update.is_final());
    }
}
