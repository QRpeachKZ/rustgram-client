// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Star Gift Resale Price
//!
//! Star gift resale price for Telegram.
//!
//! Based on TDLib's `StarGiftResalePrice` from `td/telegram/StarGiftResalePrice.h`.
//!
//! # Overview
//!
//! A `StarGiftResalePrice` represents the price of a star gift when being resold.
//! The price can be in Stars or TON (Telegram Open Network).
//!
//! # Example
//!
//! ```rust
//! use rustgram_star_gift_resale_price::StarGiftResalePrice;
//!
//! let price = StarGiftResalePrice::stars(100);
//! assert!(price.is_star());
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// TON multiplier for converting between star count and ton count.
pub const TON_MULTIPLIER: i64 = 10_000_000;

/// Star gift resale price type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
enum Type {
    /// No price
    #[default]
    None = 0,
    /// Price in Stars
    Star = 1,
    /// Price in TON
    Ton = 2,
}

/// Star gift resale price.
///
/// Represents the price of a star gift when being resold.
/// Can be either in Stars or TON.
///
/// # TDLib Mapping
///
/// - `StarGiftResalePrice::stars(amount)` → TDLib: `StarsAmount`
/// - `StarGiftResalePrice::tons(amount)` → TDLib: `StarsAmount` with TON
///
/// # Example
///
/// ```rust
/// use rustgram_star_gift_resale_price::StarGiftResalePrice;
///
/// let star_price = StarGiftResalePrice::stars(100);
/// assert_eq!(star_price.get_star_count(), 100);
///
/// let ton_price = StarGiftResalePrice::tons(5);
/// assert_eq!(ton_price.get_ton_count(), 50000000);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StarGiftResalePrice {
    type_: Type,
    amount: i64,
}

impl StarGiftResalePrice {
    /// Creates a new empty resale price.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_resale_price::StarGiftResalePrice;
    ///
    /// let price = StarGiftResalePrice::new();
    /// assert!(price.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a resale price in Stars.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount in stars
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_resale_price::StarGiftResalePrice;
    ///
    /// let price = StarGiftResalePrice::stars(100);
    /// assert!(price.is_star());
    /// assert_eq!(price.get_star_count(), 100);
    /// ```
    #[must_use]
    pub fn stars(amount: i64) -> Self {
        Self {
            type_: Type::Star,
            amount,
        }
    }

    /// Creates a resale price in TON.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount in TON (multiplied by TON_MULTIPLIER internally)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_resale_price::StarGiftResalePrice;
    ///
    /// let price = StarGiftResalePrice::tons(5);
    /// assert!(price.is_ton());
    /// assert_eq!(price.get_ton_count(), 50000000);
    /// ```
    #[must_use]
    pub fn tons(amount: i64) -> Self {
        Self {
            type_: Type::Ton,
            amount,
        }
    }

    /// Checks if this price is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_resale_price::StarGiftResalePrice;
    ///
    /// assert!(StarGiftResalePrice::new().is_empty());
    /// assert!(!StarGiftResalePrice::stars(100).is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        matches!(self.type_, Type::None)
    }

    /// Checks if this price is in Stars.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_resale_price::StarGiftResalePrice;
    ///
    /// assert!(StarGiftResalePrice::stars(100).is_star());
    /// assert!(!StarGiftResalePrice::tons(5).is_star());
    /// ```
    #[must_use]
    pub fn is_star(&self) -> bool {
        matches!(self.type_, Type::Star)
    }

    /// Checks if this price is in TON.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_resale_price::StarGiftResalePrice;
    ///
    /// assert!(StarGiftResalePrice::tons(5).is_ton());
    /// assert!(!StarGiftResalePrice::stars(100).is_ton());
    /// ```
    #[must_use]
    pub fn is_ton(&self) -> bool {
        matches!(self.type_, Type::Ton)
    }

    /// Returns the star count.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_resale_price::StarGiftResalePrice;
    ///
    /// let price = StarGiftResalePrice::stars(100);
    /// assert_eq!(price.get_star_count(), 100);
    /// ```
    #[must_use]
    pub fn get_star_count(&self) -> i64 {
        self.amount
    }

    /// Returns the TON count (multiplied by TON_MULTIPLIER).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_resale_price::StarGiftResalePrice;
    ///
    /// let price = StarGiftResalePrice::tons(5);
    /// assert_eq!(price.get_ton_count(), 50000000);
    /// ```
    #[must_use]
    pub fn get_ton_count(&self) -> i64 {
        self.amount * TON_MULTIPLIER
    }
}

impl fmt::Display for StarGiftResalePrice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.type_ {
            Type::None => write!(f, "no price"),
            Type::Star => write!(f, "{} stars", self.amount),
            Type::Ton => write!(f, "{} TON", self.amount),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let price = StarGiftResalePrice::new();
        assert!(price.is_empty());
    }

    #[test]
    fn test_default() {
        let price = StarGiftResalePrice::default();
        assert!(price.is_empty());
    }

    #[test]
    fn test_stars() {
        let price = StarGiftResalePrice::stars(100);
        assert!(!price.is_empty());
        assert!(price.is_star());
        assert!(!price.is_ton());
        assert_eq!(price.get_star_count(), 100);
    }

    #[test]
    fn test_tons() {
        let price = StarGiftResalePrice::tons(5);
        assert!(!price.is_empty());
        assert!(!price.is_star());
        assert!(price.is_ton());
        assert_eq!(price.get_ton_count(), 50000000);
    }

    #[test]
    fn test_is_empty() {
        assert!(StarGiftResalePrice::new().is_empty());
        assert!(!StarGiftResalePrice::stars(100).is_empty());
        assert!(!StarGiftResalePrice::tons(5).is_empty());
    }

    #[test]
    fn test_is_star() {
        assert!(!StarGiftResalePrice::new().is_star());
        assert!(StarGiftResalePrice::stars(100).is_star());
        assert!(!StarGiftResalePrice::tons(5).is_star());
    }

    #[test]
    fn test_is_ton() {
        assert!(!StarGiftResalePrice::new().is_ton());
        assert!(!StarGiftResalePrice::stars(100).is_ton());
        assert!(StarGiftResalePrice::tons(5).is_ton());
    }

    #[test]
    fn test_get_star_count() {
        let price = StarGiftResalePrice::stars(1000);
        assert_eq!(price.get_star_count(), 1000);
    }

    #[test]
    fn test_get_ton_count() {
        let price = StarGiftResalePrice::tons(10);
        assert_eq!(price.get_ton_count(), 100000000);
    }

    #[test]
    fn test_ton_multiplier() {
        assert_eq!(TON_MULTIPLIER, 10_000_000);
    }

    #[test]
    fn test_display_none() {
        let price = StarGiftResalePrice::new();
        assert_eq!(format!("{price}"), "no price");
    }

    #[test]
    fn test_display_star() {
        let price = StarGiftResalePrice::stars(100);
        assert_eq!(format!("{price}"), "100 stars");
    }

    #[test]
    fn test_display_ton() {
        let price = StarGiftResalePrice::tons(5);
        assert_eq!(format!("{price}"), "5 TON");
    }

    #[test]
    fn test_equality() {
        let price1 = StarGiftResalePrice::stars(100);
        let price2 = StarGiftResalePrice::stars(100);
        let price3 = StarGiftResalePrice::stars(200);

        assert_eq!(price1, price2);
        assert_ne!(price1, price3);
    }

    #[test]
    fn test_clone() {
        let price1 = StarGiftResalePrice::stars(100);
        let price2 = price1.clone();
        assert_eq!(price1, price2);
    }

    #[test]
    fn test_serialization() {
        let price = StarGiftResalePrice::stars(100);
        let json = serde_json::to_string(&price).expect("Failed to serialize");
        let deserialized: StarGiftResalePrice =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, price);
    }

    #[test]
    fn test_zero_star_price() {
        let price = StarGiftResalePrice::stars(0);
        assert!(price.is_star());
        assert_eq!(price.get_star_count(), 0);
    }

    #[test]
    fn test_zero_ton_price() {
        let price = StarGiftResalePrice::tons(0);
        assert!(price.is_ton());
        assert_eq!(price.get_ton_count(), 0);
    }

    #[test]
    fn test_large_star_amount() {
        let price = StarGiftResalePrice::stars(i64::MAX);
        assert_eq!(price.get_star_count(), i64::MAX);
    }

    #[test]
    fn test_large_ton_amount() {
        let price = StarGiftResalePrice::tons(i64::MAX / TON_MULTIPLIER);
        assert!(price.is_ton());
    }

    #[test]
    fn test_types_are_mutually_exclusive() {
        let star_price = StarGiftResalePrice::stars(100);
        let ton_price = StarGiftResalePrice::tons(5);
        let empty_price = StarGiftResalePrice::new();

        assert!(star_price.is_star());
        assert!(!star_price.is_ton());
        assert!(!star_price.is_empty());

        assert!(!ton_price.is_star());
        assert!(ton_price.is_ton());
        assert!(!ton_price.is_empty());

        assert!(!empty_price.is_star());
        assert!(!empty_price.is_ton());
        assert!(empty_price.is_empty());
    }
}
