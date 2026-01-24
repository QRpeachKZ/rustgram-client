// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Contact
//!
//! Contact type for Telegram.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_types::UserId;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Contact information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contact {
    /// Phone number.
    phone_number: String,
    /// First name.
    first_name: String,
    /// Last name.
    last_name: String,
    /// User ID.
    user_id: UserId,
}

impl Default for Contact {
    fn default() -> Self {
        Self {
            phone_number: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            user_id: UserId::default(),
        }
    }
}

impl Contact {
    /// Creates a new contact.
    #[must_use]
    pub fn new(
        phone_number: String,
        first_name: String,
        last_name: String,
        user_id: UserId,
    ) -> Self {
        Self {
            phone_number,
            first_name,
            last_name,
            user_id,
        }
    }

    /// Returns the phone number.
    #[must_use]
    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    /// Returns the first name.
    #[must_use]
    pub fn first_name(&self) -> &str {
        &self.first_name
    }

    /// Returns the last name.
    #[must_use]
    pub fn last_name(&self) -> &str {
        &self.last_name
    }

    /// Returns the user ID.
    #[must_use]
    pub const fn user_id(&self) -> UserId {
        self.user_id
    }

    /// Sets the user ID.
    pub fn set_user_id(&mut self, user_id: UserId) {
        self.user_id = user_id;
    }

    /// Returns the full name.
    #[must_use]
    pub fn full_name(&self) -> String {
        if self.last_name.is_empty() {
            self.first_name.clone()
        } else {
            format!("{} {}", self.first_name, self.last_name)
        }
    }
}

impl Hash for Contact {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.phone_number.hash(state);
        self.first_name.hash(state);
        self.last_name.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let user_id = UserId::new(123).unwrap();
        let contact = Contact::new(
            "+1234567890".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            user_id,
        );
        assert_eq!(contact.phone_number(), "+1234567890");
        assert_eq!(contact.first_name(), "John");
        assert_eq!(contact.last_name(), "Doe");
        assert_eq!(contact.user_id(), user_id);
    }

    #[test]
    fn test_full_name() {
        let contact = Contact::new(
            "123".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            UserId::new(123).unwrap(),
        );
        assert_eq!(contact.full_name(), "John Doe");
    }
}
