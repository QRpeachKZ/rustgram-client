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

//! # Star Subscription Pricing
//!
//! Pricing information for Telegram Star subscriptions.
//!
//! Based on TDLib's StarSubscriptionPricing implementation.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use rustgram_star_amount::StarAmount;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Maximum allowed star count for a subscription (1 billion).
const MAX_AMOUNT: i64 = 1_000_000_000;

/// Star subscription pricing information.
///
/// Represents the pricing for a Telegram Stars subscription, including
/// the period (in days) and the amount of stars required.
///
/// # Example
///
/// ```rust
/// use rustgram_star_subscription_pricing::StarSubscriptionPricing;
///
/// // Create a 30-day subscription for 100 stars
/// let pricing = StarSubscriptionPricing::new(30, 100);
/// assert_eq!(pricing.period(), 30);
/// assert_eq!(pricing.amount(), 100);
/// assert!(!pricing.is_empty());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct StarSubscriptionPricing {
    /// Subscription period in days
    period: i32,
    /// Amount of stars required
    amount: i64,
}

impl StarSubscriptionPricing {
    /// Creates a new star subscription pricing.
    ///
    /// # Arguments
    ///
    /// * `period` - Subscription period in days
    /// * `amount` - Amount of stars required
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_subscription_pricing::StarSubscriptionPricing;
    ///
    /// let pricing = StarSubscriptionPricing::new(30, 100);
    /// assert_eq!(pricing.period(), 30);
    /// assert_eq!(pricing.amount(), 100);
    /// ```
    pub fn new(period: i32, amount: i64) -> Self {
        Self {
            period: period.max(0),
            amount: amount.clamp(0, MAX_AMOUNT),
        }
    }

    /// Creates a star subscription pricing from a star amount.
    ///
    /// # Arguments
    ///
    /// * `period` - Subscription period in days
    /// * `star_amount` - Star amount
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_subscription_pricing::StarSubscriptionPricing;
    /// use rustgram_star_amount::StarAmount;
    ///
    /// let stars = StarAmount::from_parts(100, 0).unwrap();
    /// let pricing = StarSubscriptionPricing::from_star_amount(30, &stars);
    /// assert_eq!(pricing.amount(), 100);
    /// ```
    pub fn from_star_amount(period: i32, star_amount: &StarAmount) -> Self {
        Self::new(period, star_amount.star_count())
    }

    /// Returns the subscription period in days.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_subscription_pricing::StarSubscriptionPricing;
    ///
    /// let pricing = StarSubscriptionPricing::new(30, 100);
    /// assert_eq!(pricing.period(), 30);
    /// ```
    pub fn period(&self) -> i32 {
        self.period
    }

    /// Returns the amount of stars required.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_subscription_pricing::StarSubscriptionPricing;
    ///
    /// let pricing = StarSubscriptionPricing::new(30, 100);
    /// assert_eq!(pricing.amount(), 100);
    /// ```
    pub fn amount(&self) -> i64 {
        self.amount
    }

    /// Returns `true` if the pricing is empty (invalid period or amount).
    ///
    /// A pricing is considered empty if the period is <= 0 or the amount is <= 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_subscription_pricing::StarSubscriptionPricing;
    ///
    /// let pricing = StarSubscriptionPricing::new(30, 100);
    /// assert!(!pricing.is_empty());
    ///
    /// let empty = StarSubscriptionPricing::new(0, 100);
    /// assert!(empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.period <= 0 || self.amount <= 0
    }

    /// Converts this pricing to a star amount.
    ///
    /// Returns `None` if the pricing is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_subscription_pricing::StarSubscriptionPricing;
    ///
    /// let pricing = StarSubscriptionPricing::new(30, 100);
    /// let stars = pricing.to_star_amount().unwrap();
    /// assert_eq!(stars.star_count(), 100);
    /// ```
    pub fn to_star_amount(&self) -> Option<StarAmount> {
        if self.is_empty() {
            return None;
        }
        StarAmount::from_parts(self.amount, 0)
    }
}

impl fmt::Display for StarSubscriptionPricing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "no subscription")
        } else {
            write!(
                f,
                "subscription for {} days for {} stars",
                self.period, self.amount
            )
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::expect_used)]

    use super::*;

    #[test]
    fn test_new() {
        let pricing = StarSubscriptionPricing::new(30, 100);
        assert_eq!(pricing.period(), 30);
        assert_eq!(pricing.amount(), 100);
        assert!(!pricing.is_empty());
    }

    #[test]
    fn test_new_empty_period() {
        let pricing = StarSubscriptionPricing::new(0, 100);
        assert_eq!(pricing.period(), 0);
        assert!(pricing.is_empty());
    }

    #[test]
    fn test_new_empty_amount() {
        let pricing = StarSubscriptionPricing::new(30, 0);
        assert_eq!(pricing.amount(), 0);
        assert!(pricing.is_empty());
    }

    #[test]
    fn test_new_negative_period() {
        let pricing = StarSubscriptionPricing::new(-10, 100);
        assert_eq!(pricing.period(), 0);
        assert!(pricing.is_empty());
    }

    #[test]
    fn test_new_negative_amount() {
        let pricing = StarSubscriptionPricing::new(30, -100);
        assert_eq!(pricing.amount(), 0);
        assert!(pricing.is_empty());
    }

    #[test]
    fn test_new_large_amount() {
        let pricing = StarSubscriptionPricing::new(30, 2_000_000_000);
        assert_eq!(pricing.amount(), MAX_AMOUNT);
    }

    #[test]
    fn test_from_star_amount() {
        let stars = StarAmount::from_parts(100, 0).expect("valid star amount");
        let pricing = StarSubscriptionPricing::from_star_amount(30, &stars);
        assert_eq!(pricing.period(), 30);
        assert_eq!(pricing.amount(), 100);
    }

    #[test]
    fn test_from_star_amount_with_nanos() {
        let stars = StarAmount::from_parts(100, 500_000_000).expect("valid star amount");
        let pricing = StarSubscriptionPricing::from_star_amount(30, &stars);
        assert_eq!(pricing.amount(), 100); // Only whole stars
    }

    #[test]
    fn test_is_empty() {
        let pricing = StarSubscriptionPricing::new(30, 100);
        assert!(!pricing.is_empty());

        let empty_period = StarSubscriptionPricing::new(0, 100);
        assert!(empty_period.is_empty());

        let empty_amount = StarSubscriptionPricing::new(30, 0);
        assert!(empty_amount.is_empty());
    }

    #[test]
    fn test_to_star_amount() {
        let pricing = StarSubscriptionPricing::new(30, 100);
        let stars = pricing.to_star_amount();
        assert!(stars.is_some());
        let stars = stars.expect("valid star amount");
        assert_eq!(stars.star_count(), 100);
    }

    #[test]
    fn test_to_star_amount_empty() {
        let pricing = StarSubscriptionPricing::new(0, 100);
        assert!(pricing.to_star_amount().is_none());
    }

    #[test]
    fn test_equality() {
        let pricing1 = StarSubscriptionPricing::new(30, 100);
        let pricing2 = StarSubscriptionPricing::new(30, 100);
        assert_eq!(pricing1, pricing2);

        let pricing3 = StarSubscriptionPricing::new(60, 100);
        assert_ne!(pricing1, pricing3);

        let pricing4 = StarSubscriptionPricing::new(30, 200);
        assert_ne!(pricing1, pricing4);
    }

    #[test]
    fn test_default() {
        let pricing = StarSubscriptionPricing::default();
        assert_eq!(pricing.period(), 0);
        assert_eq!(pricing.amount(), 0);
        assert!(pricing.is_empty());
    }

    #[test]
    fn test_clone() {
        let pricing1 = StarSubscriptionPricing::new(30, 100);
        let pricing2 = pricing1.clone();
        assert_eq!(pricing1, pricing2);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let pricing1 = StarSubscriptionPricing::new(30, 100);
        let pricing2 = StarSubscriptionPricing::new(30, 100);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        pricing1.hash(&mut hasher1);
        pricing2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_serialization() {
        let pricing = StarSubscriptionPricing::new(30, 100);
        let json = serde_json::to_string(&pricing).unwrap();
        let parsed: StarSubscriptionPricing = serde_json::from_str(&json).unwrap();
        assert_eq!(pricing, parsed);
    }

    #[test]
    fn test_display() {
        let pricing = StarSubscriptionPricing::new(30, 100);
        let display = format!("{}", pricing);
        assert!(display.contains("30"));
        assert!(display.contains("days"));
        assert!(display.contains("100"));
        assert!(display.contains("stars"));
    }

    #[test]
    fn test_display_empty() {
        let pricing = StarSubscriptionPricing::new(0, 100);
        assert_eq!(format!("{}", pricing), "no subscription");
    }

    #[test]
    fn test_one_day_subscription() {
        let pricing = StarSubscriptionPricing::new(1, 10);
        assert!(!pricing.is_empty());
        assert_eq!(pricing.period(), 1);
    }

    #[test]
    fn test_yearly_subscription() {
        let pricing = StarSubscriptionPricing::new(365, 1000);
        assert!(!pricing.is_empty());
        assert_eq!(pricing.period(), 365);
    }

    #[test]
    fn test_max_amount_boundary() {
        let pricing = StarSubscriptionPricing::new(30, MAX_AMOUNT);
        assert_eq!(pricing.amount(), MAX_AMOUNT);
        assert!(!pricing.is_empty());
    }

    #[test]
    fn test_weekly_subscription() {
        let pricing = StarSubscriptionPricing::new(7, 50);
        assert!(!pricing.is_empty());
        assert_eq!(pricing.period(), 7);
        assert_eq!(pricing.amount(), 50);
    }

    #[test]
    fn test_half_year_subscription() {
        let pricing = StarSubscriptionPricing::new(182, 500);
        assert!(!pricing.is_empty());
        assert_eq!(pricing.period(), 182);
    }

    #[test]
    fn test_multiple_subscriptions() {
        let subscriptions = [
            StarSubscriptionPricing::new(7, 50),
            StarSubscriptionPricing::new(30, 100),
            StarSubscriptionPricing::new(90, 250),
            StarSubscriptionPricing::new(365, 1000),
        ];
        assert_eq!(subscriptions.len(), 4);
    }

    #[test]
    fn test_pricing_display_format() {
        let pricing = StarSubscriptionPricing::new(30, 100);
        let display = format!("{}", pricing);
        assert!(display.contains("30"));
        assert!(display.contains("days"));
        assert!(display.contains("100"));
    }
}
