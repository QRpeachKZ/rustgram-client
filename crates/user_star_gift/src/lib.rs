// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # User Star Gift
//!
//! Star gifts for Telegram users.
//!
//! ## Overview
//!
//! Represents star gifts that can be sent to users.
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_user_star_gift::StarGift;
//! use rustgram_types::UserId;
//!
//! let gift = StarGift::new(UserId::new(123).unwrap(), 5, 100);
//! assert_eq!(gift.stars(), 100);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_types::UserId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Star gift for a user
///
/// Represents a gift sent using Telegram Stars.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarGift {
    /// User who sent the gift
    sender_id: UserId,
    /// Gift ID
    gift_id: i64,
    /// Number of stars
    stars: i64,
    /// Message associated with the gift
    message: Option<String>,
}

impl StarGift {
    /// Creates a new star gift
    #[must_use]
    pub const fn new(sender_id: UserId, gift_id: i64, stars: i64) -> Self {
        Self {
            sender_id,
            gift_id,
            stars,
            message: None,
        }
    }

    /// Returns the sender user ID
    #[must_use]
    pub const fn sender_id(&self) -> UserId {
        self.sender_id
    }

    /// Returns the gift ID
    #[must_use]
    pub const fn gift_id(&self) -> i64 {
        self.gift_id
    }

    /// Returns the number of stars
    #[must_use]
    pub const fn stars(&self) -> i64 {
        self.stars
    }

    /// Returns the message
    #[must_use]
    pub const fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    /// Sets a message for the gift
    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }
}

impl fmt::Display for StarGift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StarGift(id={}, stars={}, from={})",
            self.gift_id, self.stars, self.sender_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_gift_new() {
        let sender = UserId::new(123).unwrap();
        let gift = StarGift::new(sender, 5, 100);
        assert_eq!(gift.sender_id(), sender);
        assert_eq!(gift.gift_id(), 5);
        assert_eq!(gift.stars(), 100);
    }

    #[test]
    fn test_star_gift_message() {
        let mut gift = StarGift::new(UserId::new(123).unwrap(), 1, 50);
        assert!(gift.message().is_none());

        gift.set_message("Happy birthday!".to_string());
        assert_eq!(gift.message(), Some(&"Happy birthday!".to_string()));
    }

    #[test]
    fn test_star_gift_display() {
        let gift = StarGift::new(UserId::new(123).unwrap(), 5, 100);
        assert!(format!("{}", gift).contains("100"));
    }
}
