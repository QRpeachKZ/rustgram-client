//! # Group Call Message Limit
//!
//! Message limits for group voice/video chats.
//!
//! ## Overview
//!
//! `GroupCallMessageLimit` defines the message sending limits for group calls,
//! including star costs, pin duration, text length, and emoji count.
//!
//! ## Usage
//!
//! ```
//! use rustgram_group_call_message_limit::{GroupCallMessageLimit, GroupCallMessageLimits};
//!
//! let limit = GroupCallMessageLimit::basic();
//! assert!(limit.is_valid());
//!
//! let limits = GroupCallMessageLimits::basic();
//! let level = limits.get_level(100);
//! ```

use std::cmp::Ordering;
use std::vec::Vec;

/// Message sending limits for a specific level in a group call.
///
/// Users can send messages in group calls based on their star balance.
/// Higher star counts unlock higher message limits.
///
/// # Examples
///
/// ```
/// use rustgram_group_call_message_limit::GroupCallMessageLimit;
///
/// let limit = GroupCallMessageLimit::basic();
/// assert!(limit.is_valid());
/// assert_eq!(limit.star_count(), 0);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupCallMessageLimit {
    star_count: i64,
    pin_duration: i32,
    max_text_length: i32,
    max_emoji_count: i32,
    color1: i32,
    color2: i32,
    color_bg: i32,
}

impl GroupCallMessageLimit {
    /// Creates a new message limit with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `star_count` - Required star count for this level
    /// * `pin_duration` - How long messages stay pinned (seconds)
    /// * `max_text_length` - Maximum text length for messages
    /// * `max_emoji_count` - Maximum number of emojis allowed
    /// * `color1` - Primary color
    /// * `color2` - Secondary color
    /// * `color_bg` - Background color
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message_limit::GroupCallMessageLimit;
    ///
    /// let limit = GroupCallMessageLimit::new(100, 300, 500, 10, 0xFF0000, 0x00FF00, 0x0000FF);
    /// assert_eq!(limit.star_count(), 100);
    /// ```
    #[inline]
    pub const fn new(
        star_count: i64,
        pin_duration: i32,
        max_text_length: i32,
        max_emoji_count: i32,
        color1: i32,
        color2: i32,
        color_bg: i32,
    ) -> Self {
        Self {
            star_count,
            pin_duration,
            max_text_length,
            max_emoji_count,
            color1,
            color2,
            color_bg,
        }
    }

    /// Returns the star count required for this level.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message_limit::GroupCallMessageLimit;
    ///
    /// let limit = GroupCallMessageLimit::new(100, 300, 500, 10, 0, 0, 0);
    /// assert_eq!(limit.star_count(), 100);
    /// ```
    #[inline]
    pub const fn star_count(&self) -> i64 {
        self.star_count
    }

    /// Returns the pin duration in seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message_limit::GroupCallMessageLimit;
    ///
    /// let limit = GroupCallMessageLimit::new(0, 300, 500, 10, 0, 0, 0);
    /// assert_eq!(limit.pin_duration(), 300);
    /// ```
    #[inline]
    pub const fn pin_duration(&self) -> i32 {
        self.pin_duration
    }

    /// Returns the maximum text length.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message_limit::GroupCallMessageLimit;
    ///
    /// let limit = GroupCallMessageLimit::new(0, 300, 500, 10, 0, 0, 0);
    /// assert_eq!(limit.max_text_length(), 500);
    /// ```
    #[inline]
    pub const fn max_text_length(&self) -> i32 {
        self.max_text_length
    }

    /// Returns the maximum emoji count.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message_limit::GroupCallMessageLimit;
    ///
    /// let limit = GroupCallMessageLimit::new(0, 300, 500, 10, 0, 0, 0);
    /// assert_eq!(limit.max_emoji_count(), 10);
    /// ```
    #[inline]
    pub const fn max_emoji_count(&self) -> i32 {
        self.max_emoji_count
    }

    /// Returns the primary color.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message_limit::GroupCallMessageLimit;
    ///
    /// let limit = GroupCallMessageLimit::new(0, 300, 500, 10, 0xFF0000, 0, 0);
    /// assert_eq!(limit.color1(), 0xFF0000);
    /// ```
    #[inline]
    pub const fn color1(&self) -> i32 {
        self.color1
    }

    /// Returns the secondary color.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message_limit::GroupCallMessageLimit;
    ///
    /// let limit = GroupCallMessageLimit::new(0, 300, 500, 10, 0, 0x00FF00, 0);
    /// assert_eq!(limit.color2(), 0x00FF00);
    /// ```
    #[inline]
    pub const fn color2(&self) -> i32 {
        self.color2
    }

    /// Returns the background color.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message_limit::GroupCallMessageLimit;
    ///
    /// let limit = GroupCallMessageLimit::new(0, 300, 500, 10, 0, 0, 0x0000FF);
    /// assert_eq!(limit.color_bg(), 0x0000FF);
    /// ```
    #[inline]
    pub const fn color_bg(&self) -> i32 {
        self.color_bg
    }

    /// Returns `true` if this is a valid limit configuration.
    ///
    /// A valid limit must have non-negative values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message_limit::GroupCallMessageLimit;
    ///
    /// let limit = GroupCallMessageLimit::basic();
    /// assert!(limit.is_valid());
    /// ```
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.star_count >= 0
            && self.pin_duration >= 0
            && self.max_text_length >= 0
            && self.max_emoji_count >= 0
    }

    /// Creates a basic message limit configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message_limit::GroupCallMessageLimit;
    ///
    /// let limit = GroupCallMessageLimit::basic();
    /// assert!(limit.is_valid());
    /// ```
    #[inline]
    pub const fn basic() -> Self {
        Self {
            star_count: 0,
            pin_duration: 0,
            max_text_length: 250,
            max_emoji_count: 10,
            color1: 0,
            color2: 0,
            color_bg: 0,
        }
    }
}

impl Default for GroupCallMessageLimit {
    fn default() -> Self {
        Self::basic()
    }
}

impl PartialOrd for GroupCallMessageLimit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GroupCallMessageLimit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.star_count.cmp(&other.star_count)
    }
}

/// Collection of message limits for different star count levels.
///
/// # Examples
///
/// ```
/// use rustgram_group_call_message_limit::GroupCallMessageLimits;
///
/// let limits = GroupCallMessageLimits::basic();
/// let level = limits.get_level(100);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupCallMessageLimits {
    limits: Vec<GroupCallMessageLimit>,
}

impl GroupCallMessageLimits {
    /// Creates a new collection of message limits.
    ///
    /// # Arguments
    ///
    /// * `limits` - The limit configurations for different levels
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message_limit::{GroupCallMessageLimit, GroupCallMessageLimits};
    ///
    /// let limits = GroupCallMessageLimits::new(vec![
    ///     GroupCallMessageLimit::basic(),
    /// ]);
    /// ```
    #[inline]
    pub fn new(limits: Vec<GroupCallMessageLimit>) -> Self {
        Self { limits }
    }

    /// Returns the message level for a given star count.
    ///
    /// The level is determined by finding the highest limit level
    /// that the star count qualifies for.
    ///
    /// # Arguments
    ///
    /// * `star_count` - The user's star balance
    ///
    /// # Returns
    ///
    /// The level index (0-based) that the user qualifies for.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message_limit::GroupCallMessageLimits;
    ///
    /// let limits = GroupCallMessageLimits::basic();
    /// let level = limits.get_level(100);
    /// assert!(level >= 0);
    /// ```
    pub fn get_level(&self, star_count: i64) -> i32 {
        let mut level = 0;
        for (i, limit) in self.limits.iter().enumerate() {
            if star_count >= limit.star_count() {
                level = i as i32;
            } else {
                break;
            }
        }
        level
    }

    /// Returns the limits slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message_limit::GroupCallMessageLimits;
    ///
    /// let limits = GroupCallMessageLimits::basic();
    /// assert!(!limits.limits().is_empty());
    /// ```
    #[inline]
    pub fn limits(&self) -> &[GroupCallMessageLimit] {
        &self.limits
    }

    /// Creates a basic message limits configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_message_limit::GroupCallMessageLimits;
    ///
    /// let limits = GroupCallMessageLimits::basic();
    /// assert!(!limits.limits().is_empty());
    /// ```
    #[inline]
    pub fn basic() -> Self {
        Self {
            limits: vec![GroupCallMessageLimit::basic()],
        }
    }
}

impl Default for GroupCallMessageLimits {
    fn default() -> Self {
        Self::basic()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_new() {
        let limit = GroupCallMessageLimit::new(100, 300, 500, 10, 0, 0, 0);
        assert_eq!(limit.star_count(), 100);
        assert_eq!(limit.pin_duration(), 300);
        assert_eq!(limit.max_text_length(), 500);
        assert_eq!(limit.max_emoji_count(), 10);
    }

    #[test]
    fn test_basic() {
        let limit = GroupCallMessageLimit::basic();
        assert!(limit.is_valid());
        assert_eq!(limit.star_count(), 0);
    }

    #[rstest]
    #[case(100, 300, 500, 10, 0, 0, 0, true)]
    #[case(-1, 300, 500, 10, 0, 0, 0, false)]
    #[case(100, -1, 500, 10, 0, 0, 0, false)]
    fn test_is_valid(
        #[case] star_count: i64,
        #[case] pin_duration: i32,
        #[case] max_text_length: i32,
        #[case] max_emoji_count: i32,
        #[case] color1: i32,
        #[case] color2: i32,
        #[case] color_bg: i32,
        #[case] expected: bool,
    ) {
        let limit = GroupCallMessageLimit::new(
            star_count, pin_duration, max_text_length, max_emoji_count, color1, color2, color_bg,
        );
        assert_eq!(limit.is_valid(), expected);
    }

    #[test]
    fn test_ordering() {
        let low = GroupCallMessageLimit::new(100, 0, 0, 0, 0, 0, 0);
        let high = GroupCallMessageLimit::new(500, 0, 0, 0, 0, 0, 0);
        assert!(low < high);
    }

    #[test]
    fn test_limits_get_level() {
        let limits = GroupCallMessageLimits::new(vec![
            GroupCallMessageLimit::new(0, 0, 100, 5, 0, 0, 0),
            GroupCallMessageLimit::new(100, 300, 250, 10, 0, 0, 0),
            GroupCallMessageLimit::new(500, 600, 500, 20, 0, 0, 0),
        ]);

        assert_eq!(limits.get_level(0), 0);
        assert_eq!(limits.get_level(50), 0);
        assert_eq!(limits.get_level(100), 1);
        assert_eq!(limits.get_level(250), 1);
        assert_eq!(limits.get_level(500), 2);
        assert_eq!(limits.get_level(1000), 2);
    }

    #[test]
    fn test_limits_basic() {
        let limits = GroupCallMessageLimits::basic();
        assert_eq!(limits.get_level(0), 0);
    }

    #[test]
    fn test_limits_empty() {
        let limits = GroupCallMessageLimits::new(vec![]);
        assert_eq!(limits.get_level(100), 0);
    }

    #[test]
    fn test_equality() {
        let limit1 = GroupCallMessageLimit::new(100, 300, 500, 10, 0, 0, 0);
        let limit2 = GroupCallMessageLimit::new(100, 300, 500, 10, 0, 0, 0);
        let limit3 = GroupCallMessageLimit::new(200, 300, 500, 10, 0, 0, 0);
        assert_eq!(limit1, limit2);
        assert_ne!(limit1, limit3);
    }

    #[test]
    fn test_clone() {
        let limit = GroupCallMessageLimit::basic();
        let cloned = limit.clone();
        assert_eq!(limit, cloned);
    }

    #[test]
    fn test_default() {
        let limit = GroupCallMessageLimit::default();
        assert_eq!(limit, GroupCallMessageLimit::basic());
    }
}
