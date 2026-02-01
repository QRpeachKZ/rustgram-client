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

//! # Currency Amount
//!
//! Enum wrapper for Telegram currency amounts (Stars and TON).
//!
//! This type provides a unified representation for different currency types
//! used in Telegram payments, including Telegram Stars and TON cryptocurrency.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use rustgram_star_amount::StarAmount;
use rustgram_ton_amount::TonAmount;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Currency amount type.
///
/// Represents either a free (zero) amount, Stars, or TON cryptocurrency.
/// Based on TDLib's CurrencyAmount implementation.
///
/// # Example
///
/// ```rust
/// use rustgram_currency_amount::CurrencyAmount;
///
/// // Create a free (zero) amount
/// let free = CurrencyAmount::none();
/// assert!(free.is_none());
///
/// // Create a Stars amount
/// let stars = CurrencyAmount::stars(100);
/// assert!(stars.is_stars());
/// assert_eq!(stars.as_stars().unwrap().star_count(), 100);
///
/// // Create a TON amount
/// let ton = CurrencyAmount::ton(1_000_000_000, false);
/// assert!(ton.is_ton());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum CurrencyAmount {
    /// No currency / free
    #[default]
    None,
    /// Telegram Stars amount
    Star(StarAmount),
    /// TON cryptocurrency amount (in nanotons)
    Ton(TonAmount),
}

impl CurrencyAmount {
    /// Creates a `CurrencyAmount::None` (free/zero amount).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_currency_amount::CurrencyAmount;
    ///
    /// let amount = CurrencyAmount::none();
    /// assert!(amount.is_none());
    /// ```
    pub fn none() -> Self {
        Self::None
    }

    /// Creates a `CurrencyAmount::Star` from star count.
    ///
    /// # Arguments
    ///
    /// * `star_count` - Number of stars
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_currency_amount::CurrencyAmount;
    ///
    /// let amount = CurrencyAmount::stars(100);
    /// assert!(amount.is_stars());
    /// ```
    pub fn stars(star_count: i64) -> Self {
        Self::Star(StarAmount::from_parts(star_count, 0).unwrap_or_default())
    }

    /// Creates a `CurrencyAmount::Star` from star and nanostar parts.
    ///
    /// # Arguments
    ///
    /// * `star_count` - Whole stars
    /// * `nanostar_count` - Fractional part (0-999,999,999)
    ///
    /// # Returns
    ///
    /// Returns `None` if the amount cannot be created.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_currency_amount::CurrencyAmount;
    ///
    /// let amount = CurrencyAmount::stars_with_nanos(100, 500_000_000).unwrap();
    /// assert!(amount.is_stars());
    /// ```
    pub fn stars_with_nanos(star_count: i64, nanostar_count: i32) -> Option<Self> {
        Some(Self::Star(StarAmount::from_parts(
            star_count,
            nanostar_count,
        )?))
    }

    /// Creates a `CurrencyAmount::Ton` from nanotons.
    ///
    /// # Arguments
    ///
    /// * `nanotons` - Amount in nanotons
    /// * `allow_negative` - Whether negative amounts are allowed
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_currency_amount::CurrencyAmount;
    ///
    /// let amount = CurrencyAmount::ton(1_000_000_000, false);
    /// assert!(amount.is_ton());
    /// ```
    pub fn ton(nanotons: i64, allow_negative: bool) -> Self {
        Self::Ton(TonAmount::from_nanotons(nanotons, allow_negative))
    }

    /// Returns `true` if this is a `None` (free) amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_currency_amount::CurrencyAmount;
    ///
    /// let amount = CurrencyAmount::none();
    /// assert!(amount.is_none());
    /// ```
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` if this is a `Star` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_currency_amount::CurrencyAmount;
    ///
    /// let amount = CurrencyAmount::stars(100);
    /// assert!(amount.is_stars());
    /// ```
    pub fn is_stars(&self) -> bool {
        matches!(self, Self::Star(_))
    }

    /// Returns `true` if this is a `Ton` amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_currency_amount::CurrencyAmount;
    ///
    /// let amount = CurrencyAmount::ton(1_000_000_000, false);
    /// assert!(amount.is_ton());
    /// ```
    pub fn is_ton(&self) -> bool {
        matches!(self, Self::Ton(_))
    }

    /// Returns the `StarAmount` if this is a stars amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_currency_amount::CurrencyAmount;
    ///
    /// let amount = CurrencyAmount::stars(100);
    /// assert!(amount.as_stars().is_some());
    /// ```
    pub fn as_stars(&self) -> Option<&StarAmount> {
        match self {
            Self::Star(amount) => Some(amount),
            _ => None,
        }
    }

    /// Returns the `TonAmount` if this is a TON amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_currency_amount::CurrencyAmount;
    ///
    /// let amount = CurrencyAmount::ton(1_000_000_000, false);
    /// assert!(amount.as_ton().is_some());
    /// ```
    pub fn as_ton(&self) -> Option<&TonAmount> {
        match self {
            Self::Ton(amount) => Some(amount),
            _ => None,
        }
    }

    /// Returns `true` if the amount is positive.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_currency_amount::CurrencyAmount;
    ///
    /// let amount = CurrencyAmount::stars(100);
    /// assert!(amount.is_positive());
    ///
    /// let free = CurrencyAmount::none();
    /// assert!(!free.is_positive());
    /// ```
    pub fn is_positive(&self) -> bool {
        match self {
            Self::None => false,
            Self::Star(amount) => amount.is_positive(),
            Self::Ton(amount) => amount.is_positive(),
        }
    }

    /// Returns `true` if the amount is zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_currency_amount::CurrencyAmount;
    ///
    /// let amount = CurrencyAmount::stars(0);
    /// assert!(amount.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        match self {
            Self::None => true,
            Self::Star(amount) => amount.is_zero(),
            Self::Ton(amount) => amount.is_zero(),
        }
    }
}

impl fmt::Display for CurrencyAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "[Free]"),
            Self::Star(amount) => write!(f, "[{} Stars]", amount.as_string()),
            Self::Ton(amount) => write!(f, "[{} nanotoncoins]", amount.amount()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none() {
        let amount = CurrencyAmount::none();
        assert!(amount.is_none());
        assert!(!amount.is_stars());
        assert!(!amount.is_ton());
        assert!(!amount.is_positive());
        assert!(amount.is_zero());
        assert_eq!(format!("{}", amount), "[Free]");
    }

    #[test]
    fn test_stars() {
        let amount = CurrencyAmount::stars(100);
        assert!(!amount.is_none());
        assert!(amount.is_stars());
        assert!(!amount.is_ton());
        assert!(amount.is_positive());
        assert!(!amount.is_zero());

        let star_amount = amount.as_stars();
        assert!(star_amount.is_some());
        assert_eq!(star_amount.unwrap().star_count(), 100);
    }

    #[test]
    fn test_stars_with_nanos() {
        let amount = CurrencyAmount::stars_with_nanos(100, 500_000_000).unwrap();
        assert!(amount.is_stars());

        let star_amount = amount.as_stars().unwrap();
        assert_eq!(star_amount.star_count(), 100);
        assert_eq!(star_amount.nanostar_count(), 500_000_000);
    }

    #[test]
    fn test_stars_with_nanos_invalid() {
        let result = CurrencyAmount::stars_with_nanos(100, -1);
        assert!(result.is_none());
    }

    #[test]
    fn test_ton() {
        let amount = CurrencyAmount::ton(1_000_000_000, false);
        assert!(!amount.is_none());
        assert!(!amount.is_stars());
        assert!(amount.is_ton());
        assert!(amount.is_positive());
        assert!(!amount.is_zero());

        let ton_amount = amount.as_ton();
        assert!(ton_amount.is_some());
        assert_eq!(ton_amount.unwrap().amount(), 1_000_000_000);
    }

    #[test]
    fn test_ton_negative_allowed() {
        let amount = CurrencyAmount::ton(-500_000_000, true);
        assert!(amount.is_ton());
        assert!(!amount.is_positive());

        let ton_amount = amount.as_ton().unwrap();
        assert!(ton_amount.is_negative());
        assert_eq!(ton_amount.amount(), -500_000_000);
    }

    #[test]
    fn test_ton_negative_rejected() {
        let amount = CurrencyAmount::ton(-500_000_000, false);
        assert!(amount.is_ton());

        let ton_amount = amount.as_ton().unwrap();
        assert!(!ton_amount.is_negative());
        assert_eq!(ton_amount.amount(), 0);
    }

    #[test]
    fn test_zero_stars() {
        let amount = CurrencyAmount::stars(0);
        assert!(amount.is_stars());
        assert!(!amount.is_positive());
        assert!(amount.is_zero());
    }

    #[test]
    fn test_zero_ton() {
        let amount = CurrencyAmount::ton(0, false);
        assert!(amount.is_ton());
        assert!(!amount.is_positive());
        assert!(amount.is_zero());
    }

    #[test]
    fn test_equality() {
        let amount1 = CurrencyAmount::stars(100);
        let amount2 = CurrencyAmount::stars(100);
        assert_eq!(amount1, amount2);

        let amount3 = CurrencyAmount::stars(200);
        assert_ne!(amount1, amount3);

        let amount4 = CurrencyAmount::ton(1_000_000_000, false);
        assert_ne!(amount1, amount4);
    }

    #[test]
    fn test_default() {
        let amount = CurrencyAmount::default();
        assert!(amount.is_none());
    }

    #[test]
    fn test_clone() {
        let amount1 = CurrencyAmount::stars(100);
        let amount2 = amount1.clone();
        assert_eq!(amount1, amount2);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let amount1 = CurrencyAmount::stars(100);
        let amount2 = CurrencyAmount::stars(100);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        amount1.hash(&mut hasher1);
        amount2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_serialization() {
        let amount = CurrencyAmount::stars(100);
        let json = serde_json::to_string(&amount).unwrap();
        let parsed: CurrencyAmount = serde_json::from_str(&json).unwrap();
        assert_eq!(amount, parsed);
    }

    #[test]
    fn test_display_stars() {
        let amount = CurrencyAmount::stars(100);
        let display = format!("{}", amount);
        assert!(display.contains("100"));
        assert!(display.contains("Stars"));
    }

    #[test]
    fn test_display_ton() {
        let amount = CurrencyAmount::ton(1_500_000_000, false);
        let display = format!("{}", amount);
        assert!(display.contains("1500000000"));
        assert!(display.contains("nanotoncoins"));
    }

    #[test]
    fn test_display_none() {
        let amount = CurrencyAmount::none();
        assert_eq!(format!("{}", amount), "[Free]");
    }

    #[test]
    fn test_as_stars_none() {
        let amount = CurrencyAmount::none();
        assert!(amount.as_stars().is_none());
    }

    #[test]
    fn test_as_ton_none() {
        let amount = CurrencyAmount::none();
        assert!(amount.as_ton().is_none());
    }

    #[test]
    fn test_as_stars_ton() {
        let amount = CurrencyAmount::ton(1_000_000_000, false);
        assert!(amount.as_stars().is_none());
    }

    #[test]
    fn test_as_ton_stars() {
        let amount = CurrencyAmount::stars(100);
        assert!(amount.as_ton().is_none());
    }
}
