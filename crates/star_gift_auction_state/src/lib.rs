// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Star Gift Auction State
//!
//! Star gift auction state for Telegram.
//!
//! Based on TDLib's `StarGiftAuctionState` from `td/telegram/StarGiftAuctionState.h`.
//!
//! # Overview
//!
//! A `StarGiftAuctionState` represents the state of a star gift auction.
//!
//! # Example
//!
//! ```rust
//! use rustgram_star_gift_auction_state::StarGiftAuctionState;
//!
//! let state = StarGiftAuctionState::new();
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_auction_bid_level::AuctionBidLevel;
use rustgram_star_gift_auction_round::StarGiftAuctionRound;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Star gift auction state.
///
/// Represents the state of a star gift auction.
///
/// # TDLib Mapping
///
/// TDLib: `StarGiftAuctionState`
///
/// # Example
///
/// ```rust
/// use rustgram_star_gift_auction_state::StarGiftAuctionState;
///
/// let state = StarGiftAuctionState::new();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StarGiftAuctionState {
    /// Is not modified
    is_not_modified: bool,
    /// Is active
    is_active: bool,
    /// Start date
    start_date: Option<i32>,
    /// End date
    end_date: Option<i32>,
    /// Version (active)
    version: Option<i32>,
    /// Min bid amount (active)
    min_bid_amount: Option<i64>,
    /// Bid levels (active)
    bid_levels: Option<Vec<AuctionBidLevel>>,
    /// Rounds (active)
    rounds: Option<Vec<StarGiftAuctionRound>>,
    /// Next round at (active)
    next_round_at: Option<i32>,
    /// Current round (active)
    current_round: Option<i32>,
    /// Total rounds (active)
    total_rounds: Option<i32>,
    /// Average price (finished)
    average_price: Option<i64>,
    /// Listed count (finished)
    listed_count: Option<i32>,
    /// Fragment listed count (finished)
    fragment_listed_count: Option<i32>,
    /// Fragment listed URL (finished)
    fragment_listed_url: Option<String>,
}

impl StarGiftAuctionState {
    /// Creates a new auction state.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_auction_state::StarGiftAuctionState;
    ///
    /// let state = StarGiftAuctionState::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if this is not modified.
    #[must_use]
    pub fn is_not_modified(&self) -> bool {
        self.is_not_modified
    }

    /// Checks if the auction is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Returns the version.
    #[must_use]
    pub fn version(&self) -> Option<i32> {
        self.version
    }
}

impl fmt::Display for StarGiftAuctionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_active {
            write!(f, "ActiveAuction")?;
        } else {
            write!(f, "FinishedAuction")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let state = StarGiftAuctionState::new();
        assert!(!state.is_active());
    }

    #[test]
    fn test_default() {
        let state = StarGiftAuctionState::default();
        assert!(!state.is_active());
    }

    #[test]
    fn test_is_not_modified() {
        let state = StarGiftAuctionState::new();
        assert!(!state.is_not_modified());
    }

    #[test]
    fn test_is_active() {
        let state = StarGiftAuctionState::new();
        assert!(!state.is_active());
    }

    #[test]
    fn test_version() {
        let state = StarGiftAuctionState::new();
        assert_eq!(state.version(), None);
    }

    #[test]
    fn test_display() {
        let state = StarGiftAuctionState::new();
        assert!(format!("{state}").contains("FinishedAuction"));
    }

    #[test]
    fn test_equality() {
        let state1 = StarGiftAuctionState::new();
        let state2 = StarGiftAuctionState::new();
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_clone() {
        let state1 = StarGiftAuctionState::new();
        let state2 = state1.clone();
        assert_eq!(state1, state2);
    }

    #[test]
    fn test_serialization() {
        let state = StarGiftAuctionState::new();
        let json = serde_json::to_string(&state).expect("Failed to serialize");
        let deserialized: StarGiftAuctionState =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, state);
    }
}
