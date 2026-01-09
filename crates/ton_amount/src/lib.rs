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

//! # TON Amount
//!
//! Represents TON cryptocurrency amounts in nanotons (smallest unit).
//!
//! 1 TON = 1,000,000,000 nanotons

use serde::{Deserialize, Serialize};

/// Maximum allowed TON amount (2^51 nanotons).
pub const MAX_AMOUNT: i64 = 1 << 51;

/// Represents a TON amount in nanotons.
///
/// # Example
///
/// ```rust
/// use rustgram_ton_amount::TonAmount;
///
/// // Create from nanotons
/// let amount = TonAmount::from_nanotons(1_000_000_000, false);
/// assert_eq!(amount.amount(), 1_000_000_000);
/// assert!(amount.is_positive());
/// assert!(!amount.is_negative());
///
/// // Negative amount (if allowed)
/// let negative = TonAmount::from_nanotons(-500_000_000, true);
/// assert!(negative.is_negative());
/// assert!(!negative.is_positive());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct TonAmount {
    amount: i64,
}

impl TonAmount {
    /// Creates a new `TonAmount` from nanotons.
    ///
    /// # Arguments
    ///
    /// * `amount` - Amount in nanotons
    /// * `allow_negative` - Whether negative amounts are allowed
    ///
    /// # Returns
    ///
    /// Returns a validated `TonAmount`. If the amount exceeds bounds,
    /// it will be clamped to the maximum/minimum allowed value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_ton_amount::TonAmount;
    ///
    /// let amount = TonAmount::from_nanotons(1_000_000_000, false);
    /// assert_eq!(amount.amount(), 1_000_000_000);
    /// ```
    pub fn from_nanotons(amount: i64, allow_negative: bool) -> Self {
        Self {
            amount: Self::validate_amount(amount, allow_negative),
        }
    }

    /// Validates and clamps the amount to valid bounds.
    ///
    /// # Arguments
    ///
    /// * `amount` - Raw amount to validate
    /// * `allow_negative` - Whether negative amounts are allowed
    ///
    /// # Returns
    ///
    /// Returns the validated amount, clamped to valid range.
    fn validate_amount(amount: i64, allow_negative: bool) -> i64 {
        if amount < 0 {
            if !allow_negative {
                return 0;
            }
            if amount < -MAX_AMOUNT {
                return -MAX_AMOUNT;
            }
        } else if amount > MAX_AMOUNT {
            return MAX_AMOUNT;
        }
        amount
    }

    /// Returns the amount in nanotons.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_ton_amount::TonAmount;
    ///
    /// let amount = TonAmount::from_nanotons(1_500_000_000, false);
    /// assert_eq!(amount.amount(), 1_500_000_000);
    /// ```
    pub fn amount(&self) -> i64 {
        self.amount
    }

    /// Returns `true` if the amount is positive (> 0).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_ton_amount::TonAmount;
    ///
    /// let amount = TonAmount::from_nanotons(100, false);
    /// assert!(amount.is_positive());
    ///
    /// let zero = TonAmount::from_nanotons(0, false);
    /// assert!(!zero.is_positive());
    /// ```
    pub fn is_positive(&self) -> bool {
        self.amount > 0
    }

    /// Returns `true` if the amount is negative (< 0).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_ton_amount::TonAmount;
    ///
    /// let amount = TonAmount::from_nanotons(-100, true);
    /// assert!(amount.is_negative());
    ///
    /// let positive = TonAmount::from_nanotons(100, false);
    /// assert!(!positive.is_negative());
    /// ```
    pub fn is_negative(&self) -> bool {
        self.amount < 0
    }

    /// Returns `true` if the amount is zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_ton_amount::TonAmount;
    ///
    /// let amount = TonAmount::from_nanotons(0, false);
    /// assert!(amount.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.amount == 0
    }
}

impl std::fmt::Display for TonAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} TON", self.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_nanotons_positive() {
        let amount = TonAmount::from_nanotons(1_000_000_000, false);
        assert_eq!(amount.amount(), 1_000_000_000);
        assert!(amount.is_positive());
        assert!(!amount.is_negative());
    }

    #[test]
    fn test_from_nanotons_zero() {
        let amount = TonAmount::from_nanotons(0, false);
        assert_eq!(amount.amount(), 0);
        assert!(!amount.is_positive());
        assert!(!amount.is_negative());
        assert!(amount.is_zero());
    }

    #[test]
    fn test_from_nanotons_negative_allowed() {
        let amount = TonAmount::from_nanotons(-500_000_000, true);
        assert_eq!(amount.amount(), -500_000_000);
        assert!(!amount.is_positive());
        assert!(amount.is_negative());
    }

    #[test]
    fn test_from_nanotons_negative_rejected() {
        let amount = TonAmount::from_nanotons(-500_000_000, false);
        assert_eq!(amount.amount(), 0);
        assert!(!amount.is_negative());
    }

    #[test]
    fn test_max_amount_clamp() {
        let amount = TonAmount::from_nanotons(MAX_AMOUNT + 1, false);
        assert_eq!(amount.amount(), MAX_AMOUNT);
    }

    #[test]
    fn test_min_amount_clamp() {
        let amount = TonAmount::from_nanotons(-(MAX_AMOUNT + 1), true);
        assert_eq!(amount.amount(), -MAX_AMOUNT);
    }

    #[test]
    fn test_exact_max_amount() {
        let amount = TonAmount::from_nanotons(MAX_AMOUNT, false);
        assert_eq!(amount.amount(), MAX_AMOUNT);
    }

    #[test]
    fn test_exact_min_amount() {
        let amount = TonAmount::from_nanotons(-MAX_AMOUNT, true);
        assert_eq!(amount.amount(), -MAX_AMOUNT);
    }

    #[test]
    fn test_equality() {
        let amount1 = TonAmount::from_nanotons(1_000_000_000, false);
        let amount2 = TonAmount::from_nanotons(1_000_000_000, false);
        assert_eq!(amount1, amount2);

        let amount3 = TonAmount::from_nanotons(500_000_000, false);
        assert_ne!(amount1, amount3);
    }

    #[test]
    fn test_default() {
        let amount = TonAmount::default();
        assert_eq!(amount.amount(), 0);
        assert!(amount.is_zero());
    }

    #[test]
    fn test_copy() {
        let amount1 = TonAmount::from_nanotons(1_000_000_000, false);
        let amount2 = amount1;
        assert_eq!(amount1.amount(), amount2.amount());
    }

    #[test]
    fn test_clone() {
        let amount1 = TonAmount::from_nanotons(1_000_000_000, false);
        let amount2 = amount1.clone();
        assert_eq!(amount1, amount2);
    }

    #[test]
    fn test_display() {
        let amount = TonAmount::from_nanotons(1_000_000_000, false);
        assert_eq!(format!("{}", amount), "1000000000 TON");
    }

    #[test]
    fn test_serialization() {
        let amount = TonAmount::from_nanotons(1_000_000_000, false);
        let json = serde_json::to_string(&amount).unwrap();
        let parsed: TonAmount = serde_json::from_str(&json).unwrap();
        assert_eq!(amount, parsed);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let amount1 = TonAmount::from_nanotons(1_000_000_000, false);
        let amount2 = TonAmount::from_nanotons(1_000_000_000, false);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        amount1.hash(&mut hasher1);
        amount2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_small_amounts() {
        let amounts = [1, 10, 100, 1000, 1_000_000];
        for raw in amounts {
            let amount = TonAmount::from_nanotons(raw, false);
            assert_eq!(amount.amount(), raw);
        }
    }

    #[test]
    fn test_large_amounts() {
        let amounts = [1_000_000_000, 10_000_000_000, 1_000_000_000_000, MAX_AMOUNT];
        for raw in amounts {
            let amount = TonAmount::from_nanotons(raw, false);
            assert_eq!(amount.amount(), raw);
        }
    }

    #[test]
    fn test_negative_small_amounts() {
        let amounts = [-1, -10, -100, -1000];
        for raw in amounts {
            let amount = TonAmount::from_nanotons(raw, true);
            assert_eq!(amount.amount(), raw);
            assert!(amount.is_negative());
        }
    }

    #[test]
    fn test_zero_edge_case() {
        // Test zero with both allow_negative values
        let amount1 = TonAmount::from_nanotons(0, false);
        let amount2 = TonAmount::from_nanotons(0, true);
        assert_eq!(amount1.amount(), 0);
        assert_eq!(amount2.amount(), 0);
        assert_eq!(amount1, amount2);
    }
}
