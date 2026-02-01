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

//! # Auction Bid Level
//!
//! Represents a bid level in a Telegram star gift auction.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_auction_bid_level::AuctionBidLevel;
//!
//! let bid = AuctionBidLevel::new(1, 100, 1234567890);
//! assert_eq!(bid.position(), 1);
//! assert_eq!(bid.star_count(), 100);
//! assert_eq!(bid.date(), 1234567890);
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// A bid level in a star gift auction.
///
/// Contains the position, star count, and date of a bid.
///
/// # Example
///
/// ```rust
/// use rustgram_auction_bid_level::AuctionBidLevel;
///
/// let bid = AuctionBidLevel::new(1, 500, 1234567890);
/// assert_eq!(bid.position(), 1);
/// assert_eq!(bid.star_count(), 500);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct AuctionBidLevel {
    /// Position in the auction (1 = first place)
    position: i32,
    /// Number of stars bid
    star_count: i64,
    /// Date of the bid as Unix timestamp
    date: i32,
}

impl AuctionBidLevel {
    /// Creates a new auction bid level.
    ///
    /// # Arguments
    ///
    /// * `position` - Position in the auction (1 = first place)
    /// * `star_count` - Number of stars bid
    /// * `date` - Date of the bid as Unix timestamp
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_auction_bid_level::AuctionBidLevel;
    ///
    /// let bid = AuctionBidLevel::new(1, 500, 1234567890);
    /// assert_eq!(bid.position(), 1);
    /// assert_eq!(bid.star_count(), 500);
    /// ```
    pub fn new(position: i32, star_count: i64, date: i32) -> Self {
        Self {
            position,
            star_count,
            date,
        }
    }

    /// Creates an auction bid level from a mock telegram_api::auctionBidLevel.
    ///
    /// This is a simplified version for testing.
    ///
    /// # Arguments
    ///
    /// * `position` - Position in the auction
    /// * `star_count` - Number of stars bid
    /// * `date` - Date of the bid as Unix timestamp
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_auction_bid_level::AuctionBidLevel;
    ///
    /// let bid = AuctionBidLevel::from_telegram_api(1, 500, 1234567890);
    /// assert_eq!(bid.position(), 1);
    /// ```
    pub fn from_telegram_api(position: i32, star_count: i64, date: i32) -> Self {
        Self {
            position,
            star_count,
            date,
        }
    }

    /// Returns the position in the auction.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_auction_bid_level::AuctionBidLevel;
    ///
    /// let bid = AuctionBidLevel::new(1, 500, 1234567890);
    /// assert_eq!(bid.position(), 1);
    /// ```
    pub fn position(&self) -> i32 {
        self.position
    }

    /// Returns the number of stars bid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_auction_bid_level::AuctionBidLevel;
    ///
    /// let bid = AuctionBidLevel::new(1, 500, 1234567890);
    /// assert_eq!(bid.star_count(), 500);
    /// ```
    pub fn star_count(&self) -> i64 {
        self.star_count
    }

    /// Returns the date of the bid as Unix timestamp.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_auction_bid_level::AuctionBidLevel;
    ///
    /// let bid = AuctionBidLevel::new(1, 500, 1234567890);
    /// assert_eq!(bid.date(), 1234567890);
    /// ```
    pub fn date(&self) -> i32 {
        self.date
    }

    /// Checks if this bid is before another bid in time.
    ///
    /// # Arguments
    ///
    /// * `other` - The other bid level to compare with
    ///
    /// # Returns
    ///
    /// Returns `true` if this bid's date is before the other's.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_auction_bid_level::AuctionBidLevel;
    ///
    /// let bid1 = AuctionBidLevel::new(1, 500, 1000000000);
    /// let bid2 = AuctionBidLevel::new(2, 400, 2000000000);
    /// assert!(bid1.is_before(&bid2));
    /// assert!(!bid2.is_before(&bid1));
    /// ```
    pub fn is_before(&self, other: &AuctionBidLevel) -> bool {
        self.date < other.date
    }

    /// Checks if this bid is after another bid in time.
    ///
    /// # Arguments
    ///
    /// * `other` - The other bid level to compare with
    ///
    /// # Returns
    ///
    /// Returns `true` if this bid's date is after the other's.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_auction_bid_level::AuctionBidLevel;
    ///
    /// let bid1 = AuctionBidLevel::new(1, 500, 2000000000);
    /// let bid2 = AuctionBidLevel::new(2, 400, 1000000000);
    /// assert!(bid1.is_after(&bid2));
    /// assert!(!bid2.is_after(&bid1));
    /// ```
    pub fn is_after(&self, other: &AuctionBidLevel) -> bool {
        self.date > other.date
    }

    /// Creates a vector of auction bid levels from mock telegram_api objects.
    ///
    /// # Arguments
    ///
    /// * `bid_data` - Vector of tuples (position, star_count, date)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_auction_bid_level::AuctionBidLevel;
    ///
    /// let data = vec![
    ///     (1, 500, 1234567890),
    ///     (2, 400, 1234567891),
    /// ];
    /// let bids = AuctionBidLevel::get_auction_bid_levels(data);
    /// assert_eq!(bids.len(), 2);
    /// ```
    pub fn get_auction_bid_levels(bid_data: Vec<(i32, i64, i32)>) -> Vec<Self> {
        bid_data
            .into_iter()
            .map(|(position, star_count, date)| Self::from_telegram_api(position, star_count, date))
            .collect()
    }

    /// Returns a mock td_api::auctionBid object.
    ///
    /// This is a placeholder for the real implementation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_auction_bid_level::AuctionBidLevel;
    ///
    /// let bid = AuctionBidLevel::new(1, 500, 1234567890);
    /// let obj = bid.get_auction_bid_object();
    /// assert_eq!(obj.position, 1);
    /// assert_eq!(obj.star_count, 500);
    /// ```
    pub fn get_auction_bid_object(&self) -> AuctionBidObject {
        AuctionBidObject {
            position: self.position,
            star_count: self.star_count,
            date: self.date,
        }
    }

    /// Checks if this is a winning bid (position 1).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_auction_bid_level::AuctionBidLevel;
    ///
    /// let winner = AuctionBidLevel::new(1, 500, 1234567890);
    /// assert!(winner.is_winning());
    ///
    /// let loser = AuctionBidLevel::new(2, 400, 1234567890);
    /// assert!(!loser.is_winning());
    /// ```
    pub fn is_winning(&self) -> bool {
        self.position == 1
    }

    /// Checks if the star count is valid (positive).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_auction_bid_level::AuctionBidLevel;
    ///
    /// let valid = AuctionBidLevel::new(1, 500, 1234567890);
    /// assert!(valid.has_valid_amount());
    ///
    /// let invalid = AuctionBidLevel::new(1, -1, 1234567890);
    /// assert!(!invalid.has_valid_amount());
    /// ```
    pub fn has_valid_amount(&self) -> bool {
        self.star_count > 0
    }
}

/// A mock TDLib API object for auction bid.
///
/// This is a placeholder for the real td_api::auctionBid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuctionBidObject {
    pub position: i32,
    pub star_count: i64,
    pub date: i32,
}

impl fmt::Display for AuctionBidLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AuctionBid[position={}, stars={}, date={}]",
            self.position, self.star_count, self.date
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bid = AuctionBidLevel::new(1, 500, 1234567890);

        assert_eq!(bid.position(), 1);
        assert_eq!(bid.star_count(), 500);
        assert_eq!(bid.date(), 1234567890);
    }

    #[test]
    fn test_from_telegram_api() {
        let bid = AuctionBidLevel::from_telegram_api(1, 500, 1234567890);

        assert_eq!(bid.position(), 1);
        assert_eq!(bid.star_count(), 500);
        assert_eq!(bid.date(), 1234567890);
    }

    #[test]
    fn test_is_before() {
        let bid1 = AuctionBidLevel::new(1, 500, 1000000000);
        let bid2 = AuctionBidLevel::new(2, 400, 2000000000);

        assert!(bid1.is_before(&bid2));
        assert!(!bid2.is_before(&bid1));
    }

    #[test]
    fn test_is_before_same_date() {
        let bid1 = AuctionBidLevel::new(1, 500, 1234567890);
        let bid2 = AuctionBidLevel::new(2, 400, 1234567890);

        assert!(!bid1.is_before(&bid2));
        assert!(!bid2.is_before(&bid1));
    }

    #[test]
    fn test_is_after() {
        let bid1 = AuctionBidLevel::new(1, 500, 2000000000);
        let bid2 = AuctionBidLevel::new(2, 400, 1000000000);

        assert!(bid1.is_after(&bid2));
        assert!(!bid2.is_after(&bid1));
    }

    #[test]
    fn test_get_auction_bid_levels() {
        let data = vec![
            (1, 500, 1234567890),
            (2, 400, 1234567891),
            (3, 300, 1234567892),
        ];

        let bids = AuctionBidLevel::get_auction_bid_levels(data);

        assert_eq!(bids.len(), 3);
        assert_eq!(bids[0].position(), 1);
        assert_eq!(bids[1].position(), 2);
        assert_eq!(bids[2].position(), 3);
    }

    #[test]
    fn test_get_auction_bid_levels_empty() {
        let bids = AuctionBidLevel::get_auction_bid_levels(vec![]);
        assert!(bids.is_empty());
    }

    #[test]
    fn test_get_auction_bid_object() {
        let bid = AuctionBidLevel::new(1, 500, 1234567890);
        let obj = bid.get_auction_bid_object();

        assert_eq!(obj.position, 1);
        assert_eq!(obj.star_count, 500);
        assert_eq!(obj.date, 1234567890);
    }

    #[test]
    fn test_is_winning() {
        let winner = AuctionBidLevel::new(1, 500, 1234567890);
        assert!(winner.is_winning());

        let loser = AuctionBidLevel::new(2, 400, 1234567890);
        assert!(!loser.is_winning());

        let third = AuctionBidLevel::new(3, 300, 1234567890);
        assert!(!third.is_winning());
    }

    #[test]
    fn test_has_valid_amount() {
        let valid = AuctionBidLevel::new(1, 500, 1234567890);
        assert!(valid.has_valid_amount());

        let invalid = AuctionBidLevel::new(1, -1, 1234567890);
        assert!(!invalid.has_valid_amount());

        let zero = AuctionBidLevel::new(1, 0, 1234567890);
        assert!(!zero.has_valid_amount());
    }

    #[test]
    fn test_default() {
        let bid = AuctionBidLevel::default();

        assert_eq!(bid.position(), 0);
        assert_eq!(bid.star_count(), 0);
        assert_eq!(bid.date(), 0);
    }

    #[test]
    fn test_equality() {
        let bid1 = AuctionBidLevel::new(1, 500, 1234567890);
        let bid2 = AuctionBidLevel::new(1, 500, 1234567890);

        assert_eq!(bid1, bid2);
    }

    #[test]
    fn test_inequality() {
        let bid1 = AuctionBidLevel::new(1, 500, 1234567890);
        let bid2 = AuctionBidLevel::new(2, 500, 1234567890);

        assert_ne!(bid1, bid2);
    }

    #[test]
    fn test_copy() {
        let bid1 = AuctionBidLevel::new(1, 500, 1234567890);
        let bid2 = bid1;

        assert_eq!(bid1, bid2);
        assert_eq!(bid1.position(), 1);
    }

    #[test]
    fn test_clone() {
        let bid1 = AuctionBidLevel::new(1, 500, 1234567890);
        let bid2 = bid1;

        assert_eq!(bid1, bid2);
    }

    #[test]
    fn test_display() {
        let bid = AuctionBidLevel::new(1, 500, 1234567890);
        let display = format!("{}", bid);

        assert!(display.contains("position=1"));
        assert!(display.contains("stars=500"));
        assert!(display.contains("date=1234567890"));
    }

    #[test]
    fn test_serialization() {
        let bid = AuctionBidLevel::new(1, 500, 1234567890);

        let json = serde_json::to_string(&bid).unwrap();
        let parsed: AuctionBidLevel = serde_json::from_str(&json).unwrap();

        assert_eq!(bid, parsed);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let bid1 = AuctionBidLevel::new(1, 500, 1234567890);
        let bid2 = AuctionBidLevel::new(1, 500, 1234567890);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        bid1.hash(&mut hasher1);
        bid2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_negative_position() {
        let bid = AuctionBidLevel::new(-1, 500, 1234567890);
        assert_eq!(bid.position(), -1);
    }

    #[test]
    fn test_negative_star_count() {
        let bid = AuctionBidLevel::new(1, -500, 1234567890);
        assert_eq!(bid.star_count(), -500);
    }

    #[test]
    fn test_negative_date() {
        let bid = AuctionBidLevel::new(1, 500, -1234567890);
        assert_eq!(bid.date(), -1234567890);
    }

    #[test]
    fn test_large_star_count() {
        let bid = AuctionBidLevel::new(1, i64::MAX, 1234567890);
        assert_eq!(bid.star_count(), i64::MAX);
    }

    #[test]
    fn test_chronological_ordering() {
        let bids = [
            AuctionBidLevel::new(3, 300, 2000000000),
            AuctionBidLevel::new(1, 500, 1000000000),
            AuctionBidLevel::new(2, 400, 1500000000),
        ];

        // Check ordering by date
        assert!(bids[1].is_before(&bids[2]));
        assert!(bids[2].is_before(&bids[0]));
    }

    #[test]
    fn test_auction_bid_object_equality() {
        let obj1 = AuctionBidObject {
            position: 1,
            star_count: 500,
            date: 1234567890,
        };
        let obj2 = AuctionBidObject {
            position: 1,
            star_count: 500,
            date: 1234567890,
        };

        assert_eq!(obj1, obj2);
    }

    #[test]
    fn test_multiple_bids_different_positions() {
        let data = vec![
            (1, 1000, 1000),
            (2, 900, 2000),
            (3, 800, 3000),
            (4, 700, 4000),
            (5, 600, 5000),
        ];

        let bids = AuctionBidLevel::get_auction_bid_levels(data);

        assert_eq!(bids.len(), 5);
        for (i, bid) in bids.iter().enumerate() {
            assert_eq!(bid.position(), (i + 1) as i32);
        }
    }

    #[test]
    fn test_zero_values() {
        let bid = AuctionBidLevel::new(0, 0, 0);

        assert_eq!(bid.position(), 0);
        assert_eq!(bid.star_count(), 0);
        assert_eq!(bid.date(), 0);
        assert!(!bid.has_valid_amount());
        assert!(!bid.is_winning());
    }

    #[test]
    fn test_future_date() {
        let future = 2147483647; // Max i32
        let bid = AuctionBidLevel::new(1, 500, future);
        assert_eq!(bid.date(), future);
    }

    #[test]
    fn test_max_star_count() {
        let bid = AuctionBidLevel::new(1, i64::MAX, 1234567890);
        assert_eq!(bid.star_count(), i64::MAX);
        assert!(bid.has_valid_amount());
    }

    #[test]
    fn test_bid_position_ordering() {
        let bid1 = AuctionBidLevel::new(1, 500, 1000);
        let bid2 = AuctionBidLevel::new(2, 400, 1000);
        let bid3 = AuctionBidLevel::new(3, 300, 1000);

        assert!(bid1.is_winning());
        assert!(!bid2.is_winning());
        assert!(!bid3.is_winning());
    }

    #[test]
    fn test_bid_with_same_date() {
        let bid1 = AuctionBidLevel::new(1, 500, 1234567890);
        let bid2 = AuctionBidLevel::new(2, 400, 1234567890);

        assert!(!bid1.is_before(&bid2));
        assert!(!bid2.is_before(&bid1));
        assert!(!bid1.is_after(&bid2));
        assert!(!bid2.is_after(&bid1));
    }
}
