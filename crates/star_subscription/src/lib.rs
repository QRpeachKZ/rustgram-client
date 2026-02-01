// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Star Subscription
//!
//! Star subscription for Telegram.
//!
//! Based on TDLib's `StarSubscription` from `td/telegram/StarSubscription.h`.
//!
//! # Overview
//!
//! A `StarSubscription` represents a Telegram Stars subscription.
//!
//! # Example
//!
//! ```rust
//! use rustgram_star_subscription::StarSubscription;
//!
//! let subscription = StarSubscription::new();
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_dialog_id::DialogId;
use rustgram_star_subscription_pricing::StarSubscriptionPricing;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Stub for Photo.
/// TODO: Replace with full Photo type when available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Photo;

/// Star subscription.
///
/// Represents a Telegram Stars subscription.
///
/// # TDLib Mapping
///
/// TDLib: `StarSubscription`
///
/// # Example
///
/// ```rust
/// use rustgram_star_subscription::StarSubscription;
///
/// let subscription = StarSubscription::new();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StarSubscription {
    /// Subscription ID
    id: Option<String>,
    /// Dialog ID
    dialog_id: Option<DialogId>,
    /// Until date
    until_date: Option<i32>,
    /// Can reuse
    can_reuse: bool,
    /// Is canceled
    is_canceled: bool,
    /// Is bot canceled
    is_bot_canceled: bool,
    /// Missing balance
    missing_balance: bool,
    /// Invite hash
    invite_hash: Option<String>,
    /// Title
    title: Option<String>,
    /// Photo
    photo: Option<Photo>,
    /// Invoice slug
    invoice_slug: Option<String>,
    /// Pricing
    pricing: Option<StarSubscriptionPricing>,
}

impl StarSubscription {
    /// Creates a new empty star subscription.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_subscription::StarSubscription;
    ///
    /// let subscription = StarSubscription::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if this subscription is valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_subscription::StarSubscription;
    ///
    /// let subscription = StarSubscription::new();
    /// assert!(!subscription.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.id.is_some()
            && self.dialog_id.is_some()
            && self.until_date.is_some()
            && self.pricing.is_some()
    }
}

impl fmt::Display for StarSubscription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StarSubscription")?;
        if let Some(title) = &self.title {
            write!(f, ": {}", title)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let subscription = StarSubscription::new();
        assert!(!subscription.is_valid());
    }

    #[test]
    fn test_default() {
        let subscription = StarSubscription::default();
        assert!(!subscription.is_valid());
    }

    #[test]
    fn test_is_valid() {
        let subscription = StarSubscription::new();
        assert!(!subscription.is_valid());
    }

    #[test]
    fn test_display() {
        let subscription = StarSubscription::new();
        assert_eq!(format!("{subscription}"), "StarSubscription");
    }

    #[test]
    fn test_display_with_title() {
        let mut subscription = StarSubscription::new();
        subscription.title = Some("Test Subscription".to_string());
        assert!(format!("{subscription}").contains("Test Subscription"));
    }

    #[test]
    fn test_equality() {
        let sub1 = StarSubscription::new();
        let sub2 = StarSubscription::new();
        assert_eq!(sub1, sub2);
    }

    #[test]
    fn test_clone() {
        let sub1 = StarSubscription::new();
        let sub2 = sub1.clone();
        assert_eq!(sub1, sub2);
    }

    #[test]
    fn test_serialization() {
        let subscription = StarSubscription::new();
        let json = serde_json::to_string(&subscription).expect("Failed to serialize");
        let deserialized: StarSubscription =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, subscription);
    }

    #[test]
    fn test_can_reuse_default() {
        let subscription = StarSubscription::new();
        assert!(!subscription.can_reuse);
    }

    #[test]
    fn test_is_canceled_default() {
        let subscription = StarSubscription::new();
        assert!(!subscription.is_canceled);
    }

    #[test]
    fn test_is_bot_canceled_default() {
        let subscription = StarSubscription::new();
        assert!(!subscription.is_bot_canceled);
    }

    #[test]
    fn test_missing_balance_default() {
        let subscription = StarSubscription::new();
        assert!(!subscription.missing_balance);
    }
}
