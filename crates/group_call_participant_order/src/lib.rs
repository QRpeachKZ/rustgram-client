//! # Group Call Participant Order
//!
//! Ordering criteria for group call participants.
//!
//! ## Overview
//!
//! `GroupCallParticipantOrder` defines the ordering criteria used to sort
//! participants in a group call. Participants are ordered by several factors:
//! - Whether they have video (highest priority)
//! - Their active date (recent activity)
//! - Their raised hand rating
//! - Their joined date
//!
//! ## Usage
//!
//! ```
//! use rustgram_group_call_participant_order::GroupCallParticipantOrder;
//!
//! let order = GroupCallParticipantOrder::new(true, 1000, 500, 900);
//! assert!(order.has_video());
//! ```

use core::cmp::Ordering;
use core::fmt;

/// Ordering criteria for sorting group call participants.
///
/// Participants in group calls are ordered based on multiple criteria:
/// 1. Whether they have video (participants with video come first)
/// 2. Their active date (more recently active participants come first)
/// 3. Their raised hand rating (higher rating comes first)
/// 4. Their joined date (more recently joined participants come first)
///
/// # Examples
///
/// ```
/// use rustgram_group_call_participant_order::GroupCallParticipantOrder;
///
/// let order = GroupCallParticipantOrder::new(true, 1000, 500, 900);
/// assert!(order.has_video());
/// assert!(order.is_valid());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupCallParticipantOrder {
    has_video: bool,
    active_date: i32,
    raise_hand_rating: i64,
    joined_date: i32,
}

impl GroupCallParticipantOrder {
    /// Creates a new `GroupCallParticipantOrder`.
    ///
    /// # Arguments
    ///
    /// * `has_video` - Whether the participant has video
    /// * `active_date` - Timestamp of last activity
    /// * `raise_hand_rating` - Rating from raising hand
    /// * `joined_date` - Timestamp when participant joined
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant_order::GroupCallParticipantOrder;
    ///
    /// let order = GroupCallParticipantOrder::new(true, 1000, 500, 900);
    /// ```
    #[inline]
    pub const fn new(has_video: bool, active_date: i32, raise_hand_rating: i64, joined_date: i32) -> Self {
        Self {
            has_video,
            active_date,
            raise_hand_rating,
            joined_date,
        }
    }

    /// Returns `true` if the participant has video.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant_order::GroupCallParticipantOrder;
    ///
    /// let order = GroupCallParticipantOrder::new(true, 1000, 500, 900);
    /// assert!(order.has_video());
    /// ```
    #[inline]
    pub const fn has_video(&self) -> bool {
        self.has_video
    }

    /// Returns the active date timestamp.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant_order::GroupCallParticipantOrder;
    ///
    /// let order = GroupCallParticipantOrder::new(true, 12345, 0, 9000);
    /// assert_eq!(order.active_date(), 12345);
    /// ```
    #[inline]
    pub const fn active_date(&self) -> i32 {
        self.active_date
    }

    /// Returns the raise hand rating.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant_order::GroupCallParticipantOrder;
    ///
    /// let order = GroupCallParticipantOrder::new(true, 1000, 500, 900);
    /// assert_eq!(order.raise_hand_rating(), 500);
    /// ```
    #[inline]
    pub const fn raise_hand_rating(&self) -> i64 {
        self.raise_hand_rating
    }

    /// Returns the joined date timestamp.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant_order::GroupCallParticipantOrder;
    ///
    /// let order = GroupCallParticipantOrder::new(true, 1000, 500, 900);
    /// assert_eq!(order.joined_date(), 900);
    /// ```
    #[inline]
    pub const fn joined_date(&self) -> i32 {
        self.joined_date
    }

    /// Returns `true` if this is a valid order.
    ///
    /// A valid order must have non-negative timestamps.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant_order::GroupCallParticipantOrder;
    ///
    /// let order = GroupCallParticipantOrder::new(true, 1000, 500, 900);
    /// assert!(order.is_valid());
    /// ```
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.active_date >= 0 && self.joined_date >= 0
    }

    /// Creates the minimum possible order (lowest priority).
    ///
    /// This represents a participant without video who has never been active.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant_order::GroupCallParticipantOrder;
    ///
    /// let min = GroupCallParticipantOrder::min();
    /// assert!(!min.has_video());
    /// assert_eq!(min.active_date(), 0);
    /// ```
    #[inline]
    pub const fn min() -> Self {
        Self {
            has_video: false,
            active_date: 0,
            raise_hand_rating: 0,
            joined_date: 0,
        }
    }

    /// Creates the maximum possible order (highest priority).
    ///
    /// This represents a participant with video, recent activity,
    /// high raise hand rating, and recent join time.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_group_call_participant_order::GroupCallParticipantOrder;
    ///
    /// let max = GroupCallParticipantOrder::max();
    /// assert!(max.has_video());
    /// assert!(max.active_date() > 0);
    /// ```
    #[inline]
    pub const fn max() -> Self {
        Self {
            has_video: true,
            active_date: i32::MAX,
            raise_hand_rating: i64::MAX,
            joined_date: i32::MAX,
        }
    }
}

impl Default for GroupCallParticipantOrder {
    fn default() -> Self {
        Self::min()
    }
}

impl PartialOrd for GroupCallParticipantOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GroupCallParticipantOrder {
    fn cmp(&self, other: &Self) -> Ordering {
        // Video participants come first
        match self.has_video.cmp(&other.has_video) {
            Ordering::Equal => {}
            other => return other,
        }
        // Then by active date (more recent = higher)
        match self.active_date.cmp(&other.active_date) {
            Ordering::Equal => {}
            other => return other,
        }
        // Then by raise hand rating (higher = higher priority)
        match self.raise_hand_rating.cmp(&other.raise_hand_rating) {
            Ordering::Equal => {}
            other => return other,
        }
        // Finally by joined date (more recent = higher)
        self.joined_date.cmp(&other.joined_date)
    }
}

impl fmt::Display for GroupCallParticipantOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GroupCallParticipantOrder {{ has_video: {}, active_date: {}, raise_hand_rating: {}, joined_date: {} }}",
            self.has_video, self.active_date, self.raise_hand_rating, self.joined_date
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_new() {
        let order = GroupCallParticipantOrder::new(true, 1000, 500, 900);
        assert!(order.has_video());
        assert_eq!(order.active_date(), 1000);
        assert_eq!(order.raise_hand_rating(), 500);
        assert_eq!(order.joined_date(), 900);
    }

    #[test]
    fn test_default() {
        let order = GroupCallParticipantOrder::default();
        assert!(!order.has_video());
        assert_eq!(order.active_date(), 0);
        assert_eq!(order.raise_hand_rating(), 0);
        assert_eq!(order.joined_date(), 0);
    }

    #[test]
    fn test_min() {
        let order = GroupCallParticipantOrder::min();
        assert!(!order.has_video());
        assert_eq!(order.active_date(), 0);
        assert_eq!(order.raise_hand_rating(), 0);
        assert_eq!(order.joined_date(), 0);
    }

    #[test]
    fn test_max() {
        let order = GroupCallParticipantOrder::max();
        assert!(order.has_video());
        assert_eq!(order.active_date(), i32::MAX);
        assert_eq!(order.raise_hand_rating(), i64::MAX);
        assert_eq!(order.joined_date(), i32::MAX);
    }

    #[rstest]
    #[case(true, 1000, 500, 900, true)]
    #[case(true, 1000, 500, -1, false)]
    #[case(true, -1, 500, 900, false)]
    fn test_is_valid(
        #[case] has_video: bool,
        #[case] active_date: i32,
        #[case] raise_hand_rating: i64,
        #[case] joined_date: i32,
        #[case] expected: bool,
    ) {
        let order = GroupCallParticipantOrder::new(has_video, active_date, raise_hand_rating, joined_date);
        assert_eq!(order.is_valid(), expected);
    }

    #[test]
    fn test_ordering_video_first() {
        let with_video = GroupCallParticipantOrder::new(true, 100, 0, 100);
        let without_video = GroupCallParticipantOrder::new(false, 1000, 0, 1000);
        assert!(with_video > without_video);
    }

    #[test]
    fn test_ordering_active_date() {
        let recent = GroupCallParticipantOrder::new(false, 1000, 0, 100);
        let older = GroupCallParticipantOrder::new(false, 100, 0, 100);
        assert!(recent > older);
    }

    #[test]
    fn test_ordering_raise_hand_rating() {
        let high_rating = GroupCallParticipantOrder::new(false, 100, 500, 100);
        let low_rating = GroupCallParticipantOrder::new(false, 100, 100, 100);
        assert!(high_rating > low_rating);
    }

    #[test]
    fn test_ordering_joined_date() {
        let recent = GroupCallParticipantOrder::new(false, 100, 0, 1000);
        let older = GroupCallParticipantOrder::new(false, 100, 0, 100);
        assert!(recent > older);
    }

    #[test]
    fn test_equality() {
        let order1 = GroupCallParticipantOrder::new(true, 1000, 500, 900);
        let order2 = GroupCallParticipantOrder::new(true, 1000, 500, 900);
        let order3 = GroupCallParticipantOrder::new(false, 1000, 500, 900);

        assert_eq!(order1, order2);
        assert_ne!(order1, order3);
    }

    #[test]
    fn test_comparison_operators() {
        let order1 = GroupCallParticipantOrder::new(true, 1000, 500, 900);
        let order2 = GroupCallParticipantOrder::new(true, 1000, 500, 800);
        let order3 = GroupCallParticipantOrder::new(true, 1000, 500, 900);

        assert!(order1 > order2);
        assert!(order2 < order1);
        assert!(order1 >= order3);
        assert!(order1 <= order3);
    }

    #[test]
    fn test_copy() {
        let order1 = GroupCallParticipantOrder::new(true, 1000, 500, 900);
        let order2 = order1;
        assert_eq!(order1, order2);
    }

    #[test]
    fn test_clone() {
        let order1 = GroupCallParticipantOrder::new(true, 1000, 500, 900);
        let order2 = order1.clone();
        assert_eq!(order1, order2);
    }

    #[test]
    fn test_display() {
        let order = GroupCallParticipantOrder::new(true, 1000, 500, 900);
        let s = format!("{}", order);
        assert!(s.contains("has_video: true"));
        assert!(s.contains("active_date: 1000"));
    }

    #[rstest]
    #[case(true, false)]
    #[case(false, true)]
    fn test_has_video(#[case] has_video: bool, #[case] expected: bool) {
        let order = GroupCallParticipantOrder::new(has_video, 1000, 500, 900);
        assert_eq!(order.has_video(), expected);
    }
}
