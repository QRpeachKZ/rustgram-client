// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Suggested Post Price
//!
//! Pricing information for suggested posts in Telegram.
//!
//! ## Overview
//!
//! This module provides the [`SuggestedPostPrice`] type, which represents
//! the price for publishing a suggested post. Prices can be in Telegram Stars
//! or Toncoin.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_suggested_post_price::SuggestedPostPrice;
//!
//! // Create a price in Stars
//! let stars_price = SuggestedPostPrice::stars(100);
//! assert_eq!(stars_price.amount(), 100);
//! assert!(!stars_price.is_empty());
//!
//! // Create a price in Toncoin (in cents)
//! let ton_price = SuggestedPostPrice::ton(5000);
//! assert_eq!(ton_price.amount(), 5000);
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Price type for suggested posts.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib's `SuggestedPostPrice` class.
///
/// # Example
///
/// ```rust
/// use rustgram_suggested_post_price::SuggestedPostPrice;
///
/// let price = SuggestedPostPrice::stars(100);
/// assert_eq!(price.amount(), 100);
/// assert!(matches!(price, SuggestedPostPrice::Star(_)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SuggestedPostPrice {
    /// Free (no price).
    None,
    /// Price in Telegram Stars.
    Star(i64),
    /// Price in Toncoin cents (1/100 of a Toncoin).
    Ton(i64),
}

impl SuggestedPostPrice {
    /// TON multiplier: 1 TON = 10,000,000 units in TDLib.
    pub const TON_MULTIPLIER: i64 = 10_000_000;

    /// Creates a free (no price) value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let price = SuggestedPostPrice::none();
    /// assert!(price.is_empty());
    /// ```
    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }

    /// Creates a price in Telegram Stars.
    ///
    /// # Arguments
    ///
    /// * `amount` - Number of Stars
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let price = SuggestedPostPrice::stars(100);
    /// assert_eq!(price.amount(), 100);
    /// ```
    #[must_use]
    pub const fn stars(amount: i64) -> Self {
        Self::Star(amount)
    }

    /// Creates a price in Toncoin cents.
    ///
    /// # Arguments
    ///
    /// * `cents` - Amount in Toncoin cents (1/100 of a Toncoin)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let price = SuggestedPostPrice::ton(5000); // 50 Toncoins
    /// assert_eq!(price.amount(), 5000);
    /// ```
    #[must_use]
    pub const fn ton(cents: i64) -> Self {
        Self::Ton(cents)
    }

    /// Returns the price amount.
    ///
    /// Returns 0 if the price is [`None`](SuggestedPostPrice::None).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let price = SuggestedPostPrice::stars(100);
    /// assert_eq!(price.amount(), 100);
    ///
    /// let none = SuggestedPostPrice::none();
    /// assert_eq!(none.amount(), 0);
    /// ```
    #[must_use]
    pub const fn amount(&self) -> i64 {
        match *self {
            Self::None => 0,
            Self::Star(amount) => amount,
            Self::Ton(amount) => amount,
        }
    }

    /// Checks if this is an empty (free) price.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// assert!(SuggestedPostPrice::none().is_empty());
    /// assert!(!SuggestedPostPrice::stars(100).is_empty());
    /// assert!(!SuggestedPostPrice::ton(5000).is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Checks if this is a Stars price.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// assert!(SuggestedPostPrice::stars(100).is_stars());
    /// assert!(!SuggestedPostPrice::ton(5000).is_stars());
    /// ```
    #[must_use]
    pub const fn is_stars(&self) -> bool {
        matches!(self, Self::Star(_))
    }

    /// Checks if this is a Toncoin price.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// assert!(SuggestedPostPrice::ton(5000).is_ton());
    /// assert!(!SuggestedPostPrice::stars(100).is_ton());
    /// ```
    #[must_use]
    pub const fn is_ton(&self) -> bool {
        matches!(self, Self::Ton(_))
    }
}

impl Default for SuggestedPostPrice {
    /// Returns a free (no price) value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let default = SuggestedPostPrice::default();
    /// assert!(default.is_empty());
    /// ```
    fn default() -> Self {
        Self::None
    }
}

impl fmt::Display for SuggestedPostPrice {
    /// Formats the price for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// assert_eq!(format!("{}", SuggestedPostPrice::none()), "[Free]");
    /// assert_eq!(format!("{}", SuggestedPostPrice::stars(100)), "[100 Stars]");
    /// assert_eq!(format!("{}", SuggestedPostPrice::ton(5000)), "[5000 Toncoin cents]");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::None => write!(f, "[Free]"),
            Self::Star(amount) => write!(f, "[{amount} Stars]"),
            Self::Ton(amount) => write!(f, "[{amount} Toncoin cents]"),
        }
    }
}

impl From<i64> for SuggestedPostPrice {
    /// Creates a Stars price from an amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_suggested_post_price::SuggestedPostPrice;
    ///
    /// let price: SuggestedPostPrice = 100.into();
    /// assert_eq!(price, SuggestedPostPrice::stars(100));
    /// ```
    fn from(amount: i64) -> Self {
        if amount == 0 {
            Self::None
        } else {
            Self::Star(amount)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none() {
        let price = SuggestedPostPrice::none();
        assert!(price.is_empty());
        assert_eq!(price.amount(), 0);
        assert!(!price.is_stars());
        assert!(!price.is_ton());
    }

    #[test]
    fn test_stars() {
        let price = SuggestedPostPrice::stars(100);
        assert!(!price.is_empty());
        assert_eq!(price.amount(), 100);
        assert!(price.is_stars());
        assert!(!price.is_ton());
    }

    #[test]
    fn test_ton() {
        let price = SuggestedPostPrice::ton(5000);
        assert!(!price.is_empty());
        assert_eq!(price.amount(), 5000);
        assert!(!price.is_stars());
        assert!(price.is_ton());
    }

    #[test]
    fn test_amount_none() {
        assert_eq!(SuggestedPostPrice::none().amount(), 0);
    }

    #[test]
    fn test_amount_stars() {
        assert_eq!(SuggestedPostPrice::stars(999).amount(), 999);
    }

    #[test]
    fn test_amount_ton() {
        assert_eq!(SuggestedPostPrice::ton(777).amount(), 777);
    }

    #[test]
    fn test_is_empty_none() {
        assert!(SuggestedPostPrice::none().is_empty());
    }

    #[test]
    fn test_is_empty_stars() {
        assert!(!SuggestedPostPrice::stars(1).is_empty());
        assert!(!SuggestedPostPrice::stars(100).is_empty());
    }

    #[test]
    fn test_is_empty_ton() {
        assert!(!SuggestedPostPrice::ton(1).is_empty());
        assert!(!SuggestedPostPrice::ton(5000).is_empty());
    }

    #[test]
    fn test_is_stars_true() {
        assert!(SuggestedPostPrice::stars(1).is_stars());
        assert!(SuggestedPostPrice::stars(100).is_stars());
    }

    #[test]
    fn test_is_stars_false() {
        assert!(!SuggestedPostPrice::none().is_stars());
        assert!(!SuggestedPostPrice::ton(5000).is_stars());
    }

    #[test]
    fn test_is_ton_true() {
        assert!(SuggestedPostPrice::ton(1).is_ton());
        assert!(SuggestedPostPrice::ton(5000).is_ton());
    }

    #[test]
    fn test_is_ton_false() {
        assert!(!SuggestedPostPrice::none().is_ton());
        assert!(!SuggestedPostPrice::stars(100).is_ton());
    }

    #[test]
    fn test_default() {
        let default = SuggestedPostPrice::default();
        assert!(default.is_empty());
        assert_eq!(default, SuggestedPostPrice::none());
    }

    #[test]
    fn test_display_none() {
        assert_eq!(format!("{}", SuggestedPostPrice::none()), "[Free]");
    }

    #[test]
    fn test_display_stars() {
        assert_eq!(format!("{}", SuggestedPostPrice::stars(100)), "[100 Stars]");
        assert_eq!(format!("{}", SuggestedPostPrice::stars(1)), "[1 Stars]");
        assert_eq!(format!("{}", SuggestedPostPrice::stars(0)), "[0 Stars]");
    }

    #[test]
    fn test_display_ton() {
        assert_eq!(
            format!("{}", SuggestedPostPrice::ton(5000)),
            "[5000 Toncoin cents]"
        );
        assert_eq!(
            format!("{}", SuggestedPostPrice::ton(1)),
            "[1 Toncoin cents]"
        );
        assert_eq!(
            format!("{}", SuggestedPostPrice::ton(0)),
            "[0 Toncoin cents]"
        );
    }

    #[test]
    fn test_from_i64_zero() {
        let price: SuggestedPostPrice = 0.into();
        assert_eq!(price, SuggestedPostPrice::none());
    }

    #[test]
    fn test_from_i64_positive() {
        let price: SuggestedPostPrice = 100.into();
        assert_eq!(price, SuggestedPostPrice::stars(100));
    }

    #[test]
    fn test_from_i64_negative() {
        let price: SuggestedPostPrice = SuggestedPostPrice::from(-50);
        assert_eq!(price, SuggestedPostPrice::stars(-50));
    }

    #[test]
    fn test_equality_same_type() {
        assert_eq!(
            SuggestedPostPrice::stars(100),
            SuggestedPostPrice::stars(100)
        );
        assert_eq!(SuggestedPostPrice::ton(5000), SuggestedPostPrice::ton(5000));
        assert_eq!(SuggestedPostPrice::none(), SuggestedPostPrice::none());
    }

    #[test]
    fn test_equality_different_type() {
        assert_ne!(SuggestedPostPrice::stars(100), SuggestedPostPrice::ton(100));
        assert_ne!(SuggestedPostPrice::stars(100), SuggestedPostPrice::none());
    }

    #[test]
    fn test_equality_different_amount() {
        assert_ne!(
            SuggestedPostPrice::stars(100),
            SuggestedPostPrice::stars(200)
        );
        assert_ne!(SuggestedPostPrice::ton(5000), SuggestedPostPrice::ton(6000));
    }

    #[test]
    fn test_copy_semantics() {
        let price1 = SuggestedPostPrice::stars(100);
        let price2 = price1;
        assert_eq!(price1, price2);
        assert_eq!(price1.amount(), 100);
    }

    #[test]
    fn test_clone_semantics() {
        let price1 = SuggestedPostPrice::ton(5000);
        let price2 = price1.clone();
        assert_eq!(price1, price2);
    }

    #[test]
    fn test_debug_format() {
        let price = SuggestedPostPrice::stars(100);
        let debug_str = format!("{:?}", price);
        assert!(debug_str.contains("Star"));
        assert!(debug_str.contains("100"));
    }

    #[test]
    fn test_zero_stars_valid() {
        let price = SuggestedPostPrice::stars(0);
        assert!(!price.is_empty());
        assert!(price.is_stars());
        assert_eq!(price.amount(), 0);
    }

    #[test]
    fn test_zero_ton_valid() {
        let price = SuggestedPostPrice::ton(0);
        assert!(!price.is_empty());
        assert!(price.is_ton());
        assert_eq!(price.amount(), 0);
    }

    #[test]
    fn test_negative_stars() {
        let price = SuggestedPostPrice::stars(-100);
        assert!(!price.is_empty());
        assert!(price.is_stars());
        assert_eq!(price.amount(), -100);
    }

    #[test]
    fn test_negative_ton() {
        let price = SuggestedPostPrice::ton(-5000);
        assert!(!price.is_empty());
        assert!(price.is_ton());
        assert_eq!(price.amount(), -5000);
    }

    #[test]
    fn test_large_values() {
        let price = SuggestedPostPrice::stars(i64::MAX);
        assert_eq!(price.amount(), i64::MAX);

        let ton_price = SuggestedPostPrice::ton(i64::MIN);
        assert_eq!(ton_price.amount(), i64::MIN);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_none() {
        let original = SuggestedPostPrice::none();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: SuggestedPostPrice = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_stars() {
        let original = SuggestedPostPrice::stars(100);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: SuggestedPostPrice = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_ton() {
        let original = SuggestedPostPrice::ton(5000);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: SuggestedPostPrice = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_binary() {
        let original = SuggestedPostPrice::stars(100);
        let encoded = bincode::serialize(&original).unwrap();
        let decoded: SuggestedPostPrice = bincode::deserialize(&encoded).unwrap();
        assert_eq!(original, decoded);
    }
}
