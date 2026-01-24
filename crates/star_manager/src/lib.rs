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

//! # Star Manager
//!
//! Manages Telegram Stars and TON balance for the client.
//!
//! ## Overview
//!
//! The StarManager tracks owned stars/TON, pending amounts, and provides
//! utility methods for working with star transactions.
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_star_manager::StarManager;
//!
//! let manager = StarManager::new();
//!
//! // Update owned star amount
//! manager.update_owned_star_count(100_500_000_000);
//!
//! // Check if we have enough stars
//! assert!(manager.has_owned_star_count(50_000_000_000));
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_dialog_id::DialogId;
use rustgram_star_amount::StarAmount;
use rustgram_ton_amount::TonAmount;
use rustgram_types::UserId;

/// Star Manager for tracking stars/TON balance.
///
/// Based on TDLib's StarManager from `td/telegram/StarManager.h`.
/// Manages owned and pending amounts for both Stars and TON.
///
/// # Examples
///
/// ```
/// use rustgram_star_manager::StarManager;
///
/// let manager = StarManager::new();
/// assert_eq!(manager.owned_star_count(), 0);
/// ```
#[derive(Debug, Clone)]
pub struct StarManager {
    /// Whether owned star count is initialized.
    is_owned_star_count_inited: bool,
    /// Owned star count (in nanostars, 1 star = 1,000,000,000 nanostars).
    owned_star_count: i64,
    /// Owned nanostar count (fractional part, 0-999,999,999).
    owned_nanostar_count: i32,
    /// Pending owned star count (not yet confirmed).
    pending_owned_star_count: i64,
    /// Sent star count (in flight).
    sent_star_count: i64,
    /// Sent nanostar count (in flight).
    sent_nanostar_count: i32,

    /// Whether owned TON count is initialized.
    is_owned_ton_count_inited: bool,
    /// Owned TON count (in nanotons).
    owned_ton_count: i64,
    /// Pending owned TON count (not yet confirmed).
    pending_owned_ton_count: i64,
    /// Sent TON count (in flight).
    sent_ton_count: i64,
}

impl Default for StarManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StarManager {
    /// Creates a new StarManager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_star_manager::StarManager;
    ///
    /// let manager = StarManager::new();
    /// ```
    pub fn new() -> Self {
        Self {
            is_owned_star_count_inited: false,
            owned_star_count: 0,
            owned_nanostar_count: 0,
            pending_owned_star_count: 0,
            sent_star_count: 0,
            sent_nanostar_count: 0,

            is_owned_ton_count_inited: false,
            owned_ton_count: 0,
            pending_owned_ton_count: 0,
            sent_ton_count: 0,
        }
    }

    /// Updates the owned star amount.
    ///
    /// # Arguments
    ///
    /// * `star_amount` - The new star amount
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_star_manager::StarManager;
    /// use rustgram_star_amount::StarAmount;
    ///
    /// let mut manager = StarManager::new();
    /// let amount = StarAmount::from_parts(100, 500_000_000).unwrap();
    /// manager.update_owned_star_amount(amount);
    /// ```
    pub fn update_owned_star_amount(&mut self, star_amount: StarAmount) {
        self.is_owned_star_count_inited = true;
        self.owned_star_count = star_amount.star_count();
        self.owned_nanostar_count = star_amount.nanostar_count();
    }

    /// Adds pending owned star count.
    ///
    /// # Arguments
    ///
    /// * `star_count` - Stars to add (in nanostars)
    /// * `move_to_owned` - Whether to move directly to owned amount
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_star_manager::StarManager;
    ///
    /// let mut manager = StarManager::new();
    /// manager.add_pending_owned_star_count(50_000_000_000, false);
    /// ```
    pub fn add_pending_owned_star_count(&mut self, star_count: i64, move_to_owned: bool) {
        if move_to_owned {
            self.owned_star_count += star_count;
        } else {
            self.pending_owned_star_count += star_count;
        }
    }

    /// Checks if we have at least the specified star count.
    ///
    /// # Arguments
    ///
    /// * `star_count` - Stars to check (in nanostars)
    ///
    /// # Returns
    ///
    /// `true` if we have at least the specified amount
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_star_manager::StarManager;
    ///
    /// let mut manager = StarManager::new();
    /// manager.update_owned_star_count(100_000_000_001);
    /// assert!(manager.has_owned_star_count(100_000_000_000));
    /// ```
    pub fn has_owned_star_count(&self, star_count: i64) -> bool {
        if !self.is_owned_star_count_inited {
            return false;
        }
        let total_star_count = self.owned_star_count;
        if total_star_count > star_count {
            return true;
        }
        if total_star_count < star_count {
            return false;
        }
        // Equal star counts, check nanostars
        self.owned_nanostar_count >= 0
    }

    /// Updates the owned TON amount.
    ///
    /// # Arguments
    ///
    /// * `ton_amount` - The new TON amount
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_star_manager::StarManager;
    /// use rustgram_ton_amount::TonAmount;
    ///
    /// let mut manager = StarManager::new();
    /// let amount = TonAmount::from_nanotons(1_000_000_000, false);
    /// manager.update_owned_ton_amount(amount);
    /// ```
    pub fn update_owned_ton_amount(&mut self, ton_amount: TonAmount) {
        self.is_owned_ton_count_inited = true;
        self.owned_ton_count = ton_amount.amount();
    }

    /// Adds pending owned TON count.
    ///
    /// # Arguments
    ///
    /// * `ton_count` - TON to add (in nanotons)
    /// * `move_to_owned` - Whether to move directly to owned amount
    pub fn add_pending_owned_ton_count(&mut self, ton_count: i64, move_to_owned: bool) {
        if move_to_owned {
            self.owned_ton_count += ton_count;
        } else {
            self.pending_owned_ton_count += ton_count;
        }
    }

    /// Checks if we have at least the specified TON count.
    ///
    /// # Arguments
    ///
    /// * `ton_count` - TON to check (in nanotons)
    ///
    /// # Returns
    ///
    /// `true` if we have at least the specified amount
    pub fn has_owned_ton_count(&self, ton_count: i64) -> bool {
        if !self.is_owned_ton_count_inited {
            return false;
        }
        self.owned_ton_count >= ton_count
    }

    /// Gets the owned star count (in nanostars).
    ///
    /// # Returns
    ///
    /// The owned star count
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_star_manager::StarManager;
    ///
    /// let mut manager = StarManager::new();
    /// manager.update_owned_star_count(100_000_000_001);
    /// assert_eq!(manager.owned_star_count(), 100_000_000_001);
    /// ```
    pub fn owned_star_count(&self) -> i64 {
        self.owned_star_count
    }

    /// Gets the owned nanostar count (fractional part).
    ///
    /// # Returns
    ///
    /// The owned nanostar count (0-999,999,999)
    pub fn owned_nanostar_count(&self) -> i32 {
        self.owned_nanostar_count
    }

    /// Gets the owned TON count (in nanotons).
    ///
    /// # Returns
    ///
    /// The owned TON count
    pub fn owned_ton_count(&self) -> i64 {
        self.owned_ton_count
    }

    /// Gets the pending owned star count.
    ///
    /// # Returns
    ///
    /// The pending owned star count
    pub fn pending_owned_star_count(&self) -> i64 {
        self.pending_owned_star_count
    }

    /// Gets the pending owned TON count.
    ///
    /// # Returns
    ///
    /// The pending owned TON count
    pub fn pending_owned_ton_count(&self) -> i64 {
        self.pending_owned_ton_count
    }

    /// Gets whether the owned star count is initialized.
    ///
    /// # Returns
    ///
    /// `true` if initialized
    pub fn is_owned_star_count_inited(&self) -> bool {
        self.is_owned_star_count_inited
    }

    /// Gets whether the owned TON count is initialized.
    ///
    /// # Returns
    ///
    /// `true` if initialized
    pub fn is_owned_ton_count_inited(&self) -> bool {
        self.is_owned_ton_count_inited
    }

    /// Converts star count to integer stars.
    ///
    /// This is a utility method for converting nanostars to stars.
    ///
    /// # Arguments
    ///
    /// * `amount` - Amount in nanostars
    /// * `allow_negative` - Whether to allow negative values
    ///
    /// # Returns
    ///
    /// Star count as i64, or `None` if invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_star_manager::StarManager;
    ///
    /// assert_eq!(StarManager::get_star_count(1_500_000_000, false), Some(1));
    /// assert_eq!(StarManager::get_star_count(-1_000_000_000, true), Some(-1));
    /// assert_eq!(StarManager::get_star_count(-1_000_000_000, false), None);
    /// ```
    pub fn get_star_count(amount: i64, allow_negative: bool) -> Option<i64> {
        if amount >= 0 {
            return Some(amount / 1_000_000_000);
        }
        if allow_negative {
            return Some(-((-amount + 999_999_999) / 1_000_000_000));
        }
        None
    }

    /// Gets the nanostar count from star and nanostar components.
    ///
    /// # Arguments
    ///
    /// * `star_count` - Star count (will be modified)
    /// * `nanostar_count` - Nanostar count
    ///
    /// # Returns
    ///
    /// Adjusted nanostar count (0-999,999,999)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_star_manager::StarManager;
    ///
    /// let mut stars = 100;
    /// let nanos = StarManager::get_nanostar_count(&mut stars, 1_500_000_000);
    /// assert_eq!(stars, 101);
    /// assert_eq!(nanos, 500_000_000);
    /// ```
    pub fn get_nanostar_count(star_count: &mut i64, nanostar_count: i32) -> i32 {
        let nanostar_count = nanostar_count % 1_000_000_000;
        if nanostar_count < 0 {
            *star_count -= 1;
            return nanostar_count + 1_000_000_000;
        }
        nanostar_count
    }

    /// Converts star count to approximate months.
    ///
    /// This is a utility method for estimating subscription duration.
    ///
    /// # Arguments
    ///
    /// * `star_count` - Star count in nanostars
    ///
    /// # Returns
    ///
    /// Approximate number of months
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_star_manager::StarManager;
    ///
    /// // Approximate monthly subscription cost
    /// assert_eq!(StarManager::get_months_by_star_count(10_000_000_000), 1);
    /// ```
    pub fn get_months_by_star_count(star_count: i64) -> i32 {
        let months = (star_count / 10_000_000_000) as i32;
        months.max(1).min(12)
    }

    /// Updates the owned star count directly (in nanostars).
    ///
    /// # Arguments
    ///
    /// * `star_count` - New star count in nanostars
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_star_manager::StarManager;
    ///
    /// let mut manager = StarManager::new();
    /// manager.update_owned_star_count(100_500_000_000);
    /// assert_eq!(manager.owned_star_count(), 100_500_000_000);
    /// ```
    pub fn update_owned_star_count(&mut self, star_count: i64) {
        self.is_owned_star_count_inited = true;
        self.owned_star_count = star_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let manager = StarManager::default();
        assert!(!manager.is_owned_star_count_inited());
        assert!(!manager.is_owned_ton_count_inited());
        assert_eq!(manager.owned_star_count(), 0);
        assert_eq!(manager.owned_ton_count(), 0);
    }

    #[test]
    fn test_new() {
        let manager = StarManager::new();
        assert!(!manager.is_owned_star_count_inited());
        assert!(!manager.is_owned_ton_count_inited());
        assert_eq!(manager.owned_star_count(), 0);
        assert_eq!(manager.owned_ton_count(), 0);
    }

    #[test]
    fn test_update_owned_star_count() {
        let mut manager = StarManager::new();
        manager.update_owned_star_count(100_500_000_001);
        assert!(manager.is_owned_star_count_inited());
        assert_eq!(manager.owned_star_count(), 100_500_000_001);
    }

    #[test]
    fn test_update_owned_ton_count() {
        let mut manager = StarManager::new();
        let amount = TonAmount::from_nanotons(1_000_000_000, false);
        manager.update_owned_ton_amount(amount);
        assert!(manager.is_owned_ton_count_inited());
        assert_eq!(manager.owned_ton_count(), 1_000_000_000);
    }

    #[test]
    fn test_has_owned_star_count() {
        let mut manager = StarManager::new();
        assert!(!manager.has_owned_star_count(100));

        manager.update_owned_star_count(100_000_000_001);
        assert!(manager.has_owned_star_count(100_000_000_000));
        assert!(!manager.has_owned_star_count(101_000_000_000));
    }

    #[test]
    fn test_has_owned_ton_count() {
        let mut manager = StarManager::new();
        assert!(!manager.has_owned_ton_count(1000));

        manager.update_owned_ton_count(TonAmount::from_nanotons(1_000_000_000, false));
        assert!(manager.has_owned_ton_count(1_000_000_000));
        assert!(!manager.has_owned_ton_count(2_000_000_000));
    }

    #[test]
    fn test_add_pending_owned_star_count() {
        let mut manager = StarManager::new();
        manager.add_pending_owned_star_count(50_000_000_000, false);
        assert_eq!(manager.pending_owned_star_count(), 50_000_000_000);

        manager.add_pending_owned_star_count(50_000_000_000, true);
        assert_eq!(manager.owned_star_count(), 50_000_000_000);
    }

    #[test]
    fn test_add_pending_owned_ton_count() {
        let mut manager = StarManager::new();
        manager.add_pending_owned_ton_count(500_000_000, false);
        assert_eq!(manager.pending_owned_ton_count(), 500_000_000);

        manager.add_pending_owned_ton_count(500_000_000, true);
        assert_eq!(manager.owned_ton_count(), 500_000_000);
    }

    #[test]
    fn test_get_star_count() {
        assert_eq!(StarManager::get_star_count(1_500_000_000, false), Some(1));
        assert_eq!(StarManager::get_star_count(1_000_000_000, false), Some(1));
        assert_eq!(StarManager::get_star_count(500_000_000, false), Some(0));
        assert_eq!(StarManager::get_star_count(-1_000_000_000, true), Some(-1));
        assert_eq!(StarManager::get_star_count(-1_000_000_000, false), None);
    }

    #[test]
    fn test_get_nanostar_count() {
        let mut stars = 100;
        let nanos = StarManager::get_nanostar_count(&mut stars, 500_000_000);
        assert_eq!(stars, 100);
        assert_eq!(nanos, 500_000_000);

        let mut stars = 100;
        let nanos = StarManager::get_nanostar_count(&mut stars, 1_500_000_000);
        assert_eq!(stars, 101);
        assert_eq!(nanos, 500_000_000);
    }

    #[test]
    fn test_get_months_by_star_count() {
        assert_eq!(StarManager::get_months_by_star_count(5_000_000_000), 1);
        assert_eq!(StarManager::get_months_by_star_count(10_000_000_000), 1);
        assert_eq!(StarManager::get_months_by_star_count(30_000_000_000), 3);
        assert_eq!(StarManager::get_months_by_star_count(120_000_000_000), 12);
        assert_eq!(StarManager::get_months_by_star_count(150_000_000_000), 12);
    }

    #[test]
    fn test_star_manager_total() {
        let mut manager = StarManager::new();
        manager.update_owned_star_count(100_500_000_001);
        assert_eq!(manager.owned_star_count(), 100_500_000_001);

        manager.add_pending_owned_star_count(50_000_000_000, false);
        assert_eq!(manager.pending_owned_star_count(), 50_000_000_000);
    }

    #[test]
    fn test_ton_manager_total() {
        let mut manager = StarManager::new();
        manager.update_owned_ton_count(TonAmount::from_nanotons(1_000_000_000, false));
        assert_eq!(manager.owned_ton_count(), 1_000_000_000);

        manager.add_pending_owned_ton_count(500_000_000, false);
        assert_eq!(manager.pending_owned_ton_count(), 500_000_000);
    }

    #[test]
    fn test_clone() {
        let mut manager = StarManager::new();
        manager.update_owned_star_count(100_000_000_001);
        manager.update_owned_ton_count(TonAmount::from_nanotons(1_000_000_000, false));

        let cloned = manager.clone();
        assert_eq!(cloned.owned_star_count(), 100_000_000_001);
        assert_eq!(cloned.owned_ton_count(), 1_000_000_000);
    }
}
