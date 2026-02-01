// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram Misc - Miscellaneous utility functions for Telegram MTProto client.
//!
//! This module provides various utility functions used throughout the Telegram client,
//! including string validation, cleaning, username validation, hash functions, and more.
//!
//! ## Overview
//!
//! The misc module provides utility functions for:
//!
//! - String cleaning and validation (names, usernames, phone numbers)
//! - Username validation
//! - Hash computation (MD5 string hash, vector hash)
//! - Currency amount validation
//! - Bot language code validation
//! - Premium duration conversion
//! - Color validation
//! - Emoji fingerprint generation
//! - String prefix search
//!
//! ## TDLib Correspondence
//!
//! | Rust function | TDLib function | File |
//! |---------------|----------------|------|
//! | [`clean_name`] | `td::clean_name` | `misc.cpp:24-50` |
//! | [`clean_username`] | `td::clean_username` | `misc.cpp:52-56` |
//! | [`clean_phone_number`] | `td::clean_phone_number` | `misc.cpp:58-60` |
//! | [`replace_offending_characters`] | `td::replace_offending_characters` | `misc.cpp:62-74` |
//! | [`clean_input_string`] | `td::clean_input_string` | `misc.cpp:76-161` (reused from venue) |
//! | [`strip_empty_characters`] | `td::strip_empty_characters` | `misc.cpp:163-254` |
//! | [`is_empty_string`] | `td::is_empty_string` | `misc.cpp:256-258` |
//! | [`is_valid_username`] | `td::is_valid_username` | `misc.cpp:260-282` |
//! | [`is_allowed_username`] | `td::is_allowed_username` | `misc.cpp:284-299` |
//! | [`get_md5_string_hash`] | `td::get_md5_string_hash` | `misc.cpp:301-309` |
//! | [`get_vector_hash`] | `td::get_vector_hash` | `misc.cpp:311-320` |
//! | [`get_emoji_fingerprints`] | `td::get_emoji_fingerprints` | `misc.cpp:380-388` |
//! | [`check_currency_amount`] | `td::check_currency_amount` | `misc.cpp:390-393` |
//! | [`validate_bot_language_code`] | `td::validate_bot_language_code` | `misc.cpp:395-404` |
//! | [`search_strings_by_prefix`] | `td::search_strings_by_prefix` | `misc.cpp:406-417` |
//! | [`get_premium_duration_month_count`] | `td::get_premium_duration_month_count` | `misc.cpp:419-421` |
//! | [`get_premium_duration_day_count`] | `td::get_premium_duration_day_count` | `misc.cpp:423-428` |
//! | [`is_valid_color`] | `td::is_valid_color` | `misc.cpp:430-432` |
//!
//! ## Examples
//!
//! ### Cleaning Strings
//!
//! ```
//! use rustgram_misc::{clean_name, clean_username, clean_phone_number};
//!
//! // Clean a name (collapses whitespace, removes special spaces)
//! let name = clean_name("Hello\u{A0}World", 100);
//! assert_eq!(name, "Hello World");
//!
//! // Clean a username (remove dots, lowercase)
//! let username = clean_username("Test.Username");
//! assert_eq!(username, "testusername");
//!
//! // Clean a phone number (keep only digits)
//! let phone = clean_phone_number("+1 (555) 123-4567");
//! assert_eq!(phone, "15551234567");
//! ```
//!
//! ### Validating Usernames
//!
//! ```
//! use rustgram_misc::{is_valid_username, is_allowed_username};
//!
//! assert!(is_valid_username("username"));
//! assert!(!is_valid_username("invalid-username"));
//! assert!(!is_valid_username("1numberstart"));
//!
//! assert!(is_allowed_username("validuser"));
//! assert!(!is_allowed_username("admin")); // Reserved prefix
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

pub mod error;
pub mod hash;
pub mod string;
pub mod validation;

// Re-export public API
pub use error::{MiscError, Result};
pub use hash::{get_emoji_fingerprints, get_md5_string_hash, get_vector_hash};
pub use string::{
    clean_name, clean_phone_number, clean_username, replace_offending_characters,
    strip_empty_characters,
};
pub use validation::{
    check_currency_amount, get_premium_duration_day_count, get_premium_duration_month_count,
    is_allowed_username, is_empty_string, is_valid_color, is_valid_username,
    search_strings_by_prefix, validate_bot_language_code,
};

// Re-export clean_input_string from venue crate
pub use rustgram_venue::clean_input_string;

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-misc";

// Reserved username prefixes (from misc.cpp:291-297)
const RESERVED_PREFIXES: &[&str] = &[
    "admin",
    "telegram",
    "support",
    "security",
    "settings",
    "contacts",
    "service",
    "telegraph",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-misc");
    }

    #[test]
    fn test_clean_username() {
        assert_eq!(clean_username("Test.Username"), "testusername");
        assert_eq!(clean_username("Simple"), "simple");
    }

    #[test]
    fn test_clean_phone_number() {
        assert_eq!(clean_phone_number("+1 (555) 123-4567"), "15551234567");
        assert_eq!(clean_phone_number("123-456-7890"), "1234567890");
    }

    #[test]
    fn test_is_valid_username() {
        assert!(is_valid_username("username"));
        assert!(is_valid_username("user_name"));
        assert!(is_valid_username("user123"));
        assert!(is_valid_username("A")); // Single letter is valid
        assert!(!is_valid_username("A_")); // Ends with underscore - invalid
        assert!(!is_valid_username("1numberstart"));
        assert!(!is_valid_username("invalid-username"));
        assert!(!is_valid_username("user__name"));
        assert!(!is_valid_username("username_"));
    }

    #[test]
    fn test_is_allowed_username() {
        assert!(is_allowed_username("validuser123"));
        assert!(!is_allowed_username("admin")); // Too short (5 chars min)
        assert!(!is_allowed_username("telegram")); // Too short
        assert!(!is_allowed_username("admin_user")); // Starts with "admin_"
        assert!(!is_allowed_username("telegram_bot")); // Starts with "telegram_"
        assert!(!is_allowed_username("support_user")); // Starts with "support_"
    }

    #[test]
    fn test_check_currency_amount() {
        assert!(check_currency_amount(0));
        assert!(check_currency_amount(1_000_000));
        assert!(check_currency_amount(-1_000_000));
        assert!(!check_currency_amount(10_000_000_000_000));
        assert!(!check_currency_amount(-10_000_000_000_000));
    }

    #[test]
    fn test_validate_bot_language_code() {
        assert!(validate_bot_language_code("").is_ok());
        assert!(validate_bot_language_code("en").is_ok());
        assert!(validate_bot_language_code("ru").is_ok());
        assert!(validate_bot_language_code("INVALID").is_err());
        assert!(validate_bot_language_code("eng").is_err());
    }

    #[test]
    fn test_get_premium_duration_month_count() {
        assert_eq!(get_premium_duration_month_count(30), 1);
        assert_eq!(get_premium_duration_month_count(60), 2);
        assert_eq!(get_premium_duration_month_count(90), 3);
        assert_eq!(get_premium_duration_month_count(0), 0);
    }

    #[test]
    fn test_get_premium_duration_day_count() {
        assert_eq!(get_premium_duration_day_count(1), 30); // 1 * 30 + 1/3 + 1/12 = 30
        assert_eq!(get_premium_duration_day_count(12), 365); // 12 * 30 + 4 + 1 = 365
        assert_eq!(get_premium_duration_day_count(0), 7); // Invalid returns 7
    }

    #[test]
    fn test_is_valid_color() {
        assert!(is_valid_color(0x000000));
        assert!(is_valid_color(0xFFFFFF));
        assert!(is_valid_color(0xFF0000));
        assert!(!is_valid_color(-1));
        assert!(!is_valid_color(0x1000000));
    }

    #[test]
    fn test_get_md5_string_hash() {
        let hash1 = get_md5_string_hash("test");
        let hash2 = get_md5_string_hash("test");
        assert_eq!(hash1, hash2);

        let hash3 = get_md5_string_hash("different");
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_get_vector_hash() {
        let vec1 = vec![1u64, 2, 3, 4, 5];
        let hash1 = get_vector_hash(&vec1);
        assert_eq!(hash1, get_vector_hash(&vec1));

        let vec2 = vec![1u64, 2, 3, 4, 6];
        assert_ne!(hash1, get_vector_hash(&vec2));
    }

    #[test]
    fn test_is_empty_string() {
        assert!(is_empty_string(""));
        assert!(is_empty_string("   "));
        assert!(!is_empty_string("test"));
    }

    #[test]
    fn test_reserved_prefixes() {
        assert_eq!(RESERVED_PREFIXES.len(), 8);
        assert!(RESERVED_PREFIXES.contains(&"admin"));
        assert!(RESERVED_PREFIXES.contains(&"telegram"));
    }
}
