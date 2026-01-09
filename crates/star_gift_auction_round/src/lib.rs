// Copyright 2024 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Star Gift Auction Round
//!
//! Represents an auction round for Telegram star gifts.
//!
//! Auction rounds can be either standard (with num and duration) or extendable
//! (with additional extend_top and extend_window parameters).

use serde::{Deserialize, Serialize};

/// Represents an auction round for star gifts.
///
/// # Example
///
/// ```rust
/// use rustgram_star_gift_auction_round::StarGiftAuctionRound;
///
/// // Standard round
/// let standard = StarGiftAuctionRound::with_standard(1, 3600);
/// assert_eq!(standard.num(), 1);
/// assert_eq!(standard.duration(), 3600);
/// assert!(!standard.is_extendable());
///
/// // Extendable round
/// let extendable = StarGiftAuctionRound::with_extendable(2, 7200, 5, 300);
/// assert_eq!(extendable.num(), 2);
/// assert_eq!(extendable.duration(), 7200);
/// assert_eq!(extendable.extend_top(), Some(5));
/// assert_eq!(extendable.extend_window(), Some(300));
/// assert!(extendable.is_extendable());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct StarGiftAuctionRound {
    num: i32,
    duration: i32,
    extend_top: Option<i32>,
    extend_window: Option<i32>,
}

impl StarGiftAuctionRound {
    /// Creates a new standard auction round.
    ///
    /// # Arguments
    ///
    /// * `num` - Round number
    /// * `duration` - Round duration in seconds
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_auction_round::StarGiftAuctionRound;
    ///
    /// let round = StarGiftAuctionRound::with_standard(1, 3600);
    /// ```
    pub fn with_standard(num: i32, duration: i32) -> Self {
        Self {
            num,
            duration,
            extend_top: None,
            extend_window: None,
        }
    }

    /// Creates a new extendable auction round.
    ///
    /// # Arguments
    ///
    /// * `num` - Round number
    /// * `duration` - Round duration in seconds
    /// * `extend_top` - Number of top bidders who can extend
    /// * `extend_window` - Time window for extending (seconds)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_auction_round::StarGiftAuctionRound;
    ///
    /// let round = StarGiftAuctionRound::with_extendable(1, 3600, 5, 300);
    /// ```
    pub fn with_extendable(num: i32, duration: i32, extend_top: i32, extend_window: i32) -> Self {
        Self {
            num,
            duration,
            extend_top: Some(extend_top),
            extend_window: Some(extend_window),
        }
    }

    /// Returns the round number.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_auction_round::StarGiftAuctionRound;
    ///
    /// let round = StarGiftAuctionRound::with_standard(3, 3600);
    /// assert_eq!(round.num(), 3);
    /// ```
    pub fn num(&self) -> i32 {
        self.num
    }

    /// Returns the round duration in seconds.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_auction_round::StarGiftAuctionRound;
    ///
    /// let round = StarGiftAuctionRound::with_standard(1, 7200);
    /// assert_eq!(round.duration(), 7200);
    /// ```
    pub fn duration(&self) -> i32 {
        self.duration
    }

    /// Returns the number of top bidders who can extend, if applicable.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_auction_round::StarGiftAuctionRound;
    ///
    /// let standard = StarGiftAuctionRound::with_standard(1, 3600);
    /// assert_eq!(standard.extend_top(), None);
    ///
    /// let extendable = StarGiftAuctionRound::with_extendable(1, 3600, 5, 300);
    /// assert_eq!(extendable.extend_top(), Some(5));
    /// ```
    pub fn extend_top(&self) -> Option<i32> {
        self.extend_top
    }

    /// Returns the time window for extending, if applicable.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_auction_round::StarGiftAuctionRound;
    ///
    /// let standard = StarGiftAuctionRound::with_standard(1, 3600);
    /// assert_eq!(standard.extend_window(), None);
    ///
    /// let extendable = StarGiftAuctionRound::with_extendable(1, 3600, 5, 300);
    /// assert_eq!(extendable.extend_window(), Some(300));
    /// ```
    pub fn extend_window(&self) -> Option<i32> {
        self.extend_window
    }

    /// Returns `true` if this is an extendable auction round.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_auction_round::StarGiftAuctionRound;
    ///
    /// let standard = StarGiftAuctionRound::with_standard(1, 3600);
    /// assert!(!standard.is_extendable());
    ///
    /// let extendable = StarGiftAuctionRound::with_extendable(1, 3600, 5, 300);
    /// assert!(extendable.is_extendable());
    /// ```
    pub fn is_extendable(&self) -> bool {
        self.extend_top.is_some() && self.extend_window.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_round() {
        let round = StarGiftAuctionRound::with_standard(1, 3600);
        assert_eq!(round.num(), 1);
        assert_eq!(round.duration(), 3600);
        assert_eq!(round.extend_top(), None);
        assert_eq!(round.extend_window(), None);
        assert!(!round.is_extendable());
    }

    #[test]
    fn test_extendable_round() {
        let round = StarGiftAuctionRound::with_extendable(2, 7200, 5, 300);
        assert_eq!(round.num(), 2);
        assert_eq!(round.duration(), 7200);
        assert_eq!(round.extend_top(), Some(5));
        assert_eq!(round.extend_window(), Some(300));
        assert!(round.is_extendable());
    }

    #[test]
    fn test_default() {
        let round = StarGiftAuctionRound::default();
        assert_eq!(round.num(), 0);
        assert_eq!(round.duration(), 0);
        assert!(!round.is_extendable());
    }

    #[test]
    fn test_equality() {
        let round1 = StarGiftAuctionRound::with_standard(1, 3600);
        let round2 = StarGiftAuctionRound::with_standard(1, 3600);
        assert_eq!(round1, round2);

        let round3 = StarGiftAuctionRound::with_extendable(1, 3600, 5, 300);
        assert_ne!(round1, round3);

        let round4 = StarGiftAuctionRound::with_extendable(1, 3600, 5, 300);
        assert_eq!(round3, round4);
    }

    #[test]
    fn test_round_number_variants() {
        for num in 0..10 {
            let round = StarGiftAuctionRound::with_standard(num, 3600);
            assert_eq!(round.num(), num);
        }
    }

    #[test]
    fn test_duration_variants() {
        let durations = [60, 300, 600, 1800, 3600, 7200];
        for duration in durations {
            let round = StarGiftAuctionRound::with_standard(1, duration);
            assert_eq!(round.duration(), duration);
        }
    }

    #[test]
    fn test_extendable_params() {
        let round = StarGiftAuctionRound::with_extendable(1, 3600, 10, 600);
        assert_eq!(round.extend_top(), Some(10));
        assert_eq!(round.extend_window(), Some(600));
        assert!(round.is_extendable());
    }

    #[test]
    fn test_zero_extend_params() {
        // Round with 0 extend params is still extendable
        let round = StarGiftAuctionRound::with_extendable(1, 3600, 0, 0);
        assert_eq!(round.extend_top(), Some(0));
        assert_eq!(round.extend_window(), Some(0));
        assert!(round.is_extendable());
    }

    #[test]
    fn test_serialization() {
        let round = StarGiftAuctionRound::with_extendable(1, 3600, 5, 300);
        let json = serde_json::to_string(&round).unwrap();
        let parsed: StarGiftAuctionRound = serde_json::from_str(&json).unwrap();
        assert_eq!(round, parsed);
    }

    #[test]
    fn test_serialization_standard() {
        let round = StarGiftAuctionRound::with_standard(1, 3600);
        let json = serde_json::to_string(&round).unwrap();
        let parsed: StarGiftAuctionRound = serde_json::from_str(&json).unwrap();
        assert_eq!(round, parsed);
    }

    #[test]
    fn test_clone() {
        let round = StarGiftAuctionRound::with_extendable(1, 3600, 5, 300);
        let cloned = round.clone();
        assert_eq!(round, cloned);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let round1 = StarGiftAuctionRound::with_standard(1, 3600);
        let round2 = StarGiftAuctionRound::with_standard(1, 3600);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        round1.hash(&mut hasher1);
        round2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_debug_format() {
        let round = StarGiftAuctionRound::with_extendable(1, 3600, 5, 300);
        let debug_str = format!("{:?}", round);
        assert!(debug_str.contains("StarGiftAuctionRound"));
        assert!(debug_str.contains("1"));
    }
}
