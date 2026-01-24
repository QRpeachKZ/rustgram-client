// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Validation functions for Telegram MTProto client.
//!
//! This module provides validation functions for usernames, currency amounts,
//! language codes, colors, and more.

use crate::error::{MiscError, Result};
use crate::RESERVED_PREFIXES;

/// Maximum username length.
const MAX_USERNAME_LENGTH: usize = 32;

/// Minimum allowed username length.
const MIN_ALLOWED_USERNAME_LENGTH: usize = 5;

/// Maximum currency amount (absolute value).
const MAX_CURRENCY_AMOUNT: i64 = 9_999_999_999_999;

/// Checks whether a string could be a valid username.
///
/// A valid username must:
/// - Not be empty and not exceed 32 characters
/// - Start with a letter (a-z or A-Z)
/// - Contain only letters, digits, and underscores
/// - Not end with an underscore
/// - Not contain consecutive underscores
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:260-282`
///
/// # Arguments
///
/// * `username` - The username to validate
///
/// # Returns
///
/// `true` if the username is valid, `false` otherwise.
///
/// # Examples
///
/// ```
/// use rustgram_misc::is_valid_username;
///
/// assert!(is_valid_username("username"));
/// assert!(is_valid_username("user_name"));
/// assert!(is_valid_username("user123"));
/// assert!(!is_valid_username("1numberstart"));
/// assert!(!is_valid_username("invalid-username"));
/// assert!(!is_valid_username("user__name"));
/// assert!(!is_valid_username("username_"));
/// ```
pub fn is_valid_username(username: &str) -> bool {
    if username.is_empty() || username.len() > MAX_USERNAME_LENGTH {
        return false;
    }

    let mut chars = username.chars();

    // First character must be a letter
    let first = match chars.next() {
        Some(c) => c,
        None => return false,
    };

    if !first.is_ascii_alphabetic() {
        return false;
    }

    let mut prev_char = first;
    let mut position = 1;

    for c in chars {
        position += 1;

        // Only letters, digits, and underscores allowed
        if !c.is_ascii_alphanumeric() && c != '_' {
            return false;
        }

        // No consecutive underscores
        if c == '_' && prev_char == '_' {
            return false;
        }

        // Cannot end with underscore
        if c == '_' && position == username.len() {
            return false;
        }

        prev_char = c;
    }

    true
}

/// Checks whether a string can be set as a username.
///
/// This is a stricter version of `is_valid_username` that also:
/// - Requires minimum length of 5 characters
/// - Excludes reserved username prefixes (admin, telegram, support, etc.)
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:284-299`
///
/// # Arguments
///
/// * `username` - The username to validate
///
/// # Returns
///
/// `true` if the username is allowed, `false` otherwise.
///
/// # Examples
///
/// ```
/// use rustgram_misc::is_allowed_username;
///
/// assert!(is_allowed_username("validuser123"));
/// assert!(is_allowed_username("myusername"));
/// assert!(!is_allowed_username("admin")); // Too short (5 chars min)
/// assert!(!is_allowed_username("telegram")); // Too short
/// assert!(!is_allowed_username("four")); // Less than 5 chars
/// assert!(!is_allowed_username("admin_user")); // Starts with reserved prefix
/// ```
pub fn is_allowed_username(username: &str) -> bool {
    if !is_valid_username(username) {
        return false;
    }

    if username.len() < MIN_ALLOWED_USERNAME_LENGTH {
        return false;
    }

    let lowercase = username.to_lowercase();

    // Check for reserved prefixes (TDLib rejects any username starting with these)
    for &prefix in RESERVED_PREFIXES {
        if lowercase.starts_with(prefix) {
            return false;
        }
    }

    true
}

/// Checks whether a currency amount is valid.
///
/// Valid currency amounts are in the range [-9999999999999, 9999999999999].
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:390-393`
///
/// # Arguments
///
/// * `amount` - The currency amount to validate
///
/// # Returns
///
/// `true` if the amount is valid, `false` otherwise.
///
/// # Examples
///
/// ```
/// use rustgram_misc::check_currency_amount;
///
/// assert!(check_currency_amount(0));
/// assert!(check_currency_amount(1_000_000));
/// assert!(check_currency_amount(-1_000_000));
/// assert!(check_currency_amount(9_999_999_999_999));
/// assert!(!check_currency_amount(10_000_000_000_000));
/// ```
pub fn check_currency_amount(amount: i64) -> bool {
    (-MAX_CURRENCY_AMOUNT..=MAX_CURRENCY_AMOUNT).contains(&amount)
}

/// Validates a bot language code.
///
/// Bot language codes must be either empty or exactly 2 lowercase letters (a-z).
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:395-404`
///
/// # Arguments
///
/// * `language_code` - The language code to validate
///
/// # Returns
///
/// `Ok(())` if valid, `Err(MiscError::InvalidBotLanguageCode)` otherwise.
///
/// # Examples
///
/// ```
/// use rustgram_misc::validate_bot_language_code;
///
/// assert!(validate_bot_language_code("").is_ok());
/// assert!(validate_bot_language_code("en").is_ok());
/// assert!(validate_bot_language_code("ru").is_ok());
/// assert!(validate_bot_language_code("INVALID").is_err());
/// assert!(validate_bot_language_code("eng").is_err());
/// ```
pub fn validate_bot_language_code(language_code: &str) -> Result<()> {
    if language_code.is_empty() {
        return Ok(());
    }

    if language_code.len() == 2 {
        let bytes = language_code.as_bytes();
        if bytes[0].is_ascii_lowercase() && bytes[1].is_ascii_lowercase() {
            return Ok(());
        }
    }

    Err(MiscError::InvalidBotLanguageCode(language_code.to_string()))
}

/// Searches strings by prefix and returns matching indices.
///
/// This function searches through a list of strings and returns the indices
/// of strings that start with the given query prefix.
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:406-417`
///
/// # Arguments
///
/// * `strings` - The list of strings to search
/// * `query` - The prefix query to search for
/// * `limit` - Maximum number of results to return
/// * `return_all_for_empty_query` - If true, returns all indices when query is empty
///
/// # Returns
///
/// A vector of indices of matching strings.
///
/// # Examples
///
/// ```
/// use rustgram_misc::search_strings_by_prefix;
///
/// let strings = vec![
///     "apple".to_string(),
///     "application".to_string(),
///     "banana".to_string(),
///     "apricot".to_string(),
/// ];
///
/// let results = search_strings_by_prefix(&strings, "app", 10, false);
/// assert_eq!(results, vec![0, 1]); // apple, application
/// ```
pub fn search_strings_by_prefix(
    strings: &[String],
    query: &str,
    limit: usize,
    return_all_for_empty_query: bool,
) -> Vec<usize> {
    if query.is_empty() {
        if return_all_for_empty_query {
            return strings
                .iter()
                .enumerate()
                .take(limit)
                .map(|(i, _)| i)
                .collect();
        } else {
            return Vec::new();
        }
    }

    let query_lower = query.to_lowercase();
    let mut results = Vec::new();

    for (i, s) in strings.iter().enumerate() {
        if results.len() >= limit {
            break;
        }

        if s.to_lowercase().starts_with(&query_lower) {
            results.push(i);
        }
    }

    results
}

/// Converts Premium duration in days to approximate duration in months.
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:419-421`
///
/// # Arguments
///
/// * `day_count` - Number of days
///
/// # Returns
///
/// Approximate number of months (minimum 1).
///
/// # Examples
///
/// ```
/// use rustgram_misc::get_premium_duration_month_count;
///
/// assert_eq!(get_premium_duration_month_count(30), 1);
/// assert_eq!(get_premium_duration_month_count(60), 2);
/// assert_eq!(get_premium_duration_month_count(90), 3);
/// ```
pub fn get_premium_duration_month_count(day_count: i32) -> i32 {
    if day_count <= 0 {
        return 0;
    }
    (day_count / 30).max(1)
}

/// Converts Premium duration in months to duration in days.
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:423-428`
///
/// # Arguments
///
/// * `month_count` - Number of months
///
/// # Returns
///
/// Number of days (returns 7 for invalid values).
///
/// # Examples
///
/// ```
/// use rustgram_misc::get_premium_duration_day_count;
///
/// assert_eq!(get_premium_duration_day_count(1), 30);
/// assert_eq!(get_premium_duration_day_count(12), 365);
/// assert_eq!(get_premium_duration_day_count(0), 7);
/// ```
pub fn get_premium_duration_day_count(month_count: i32) -> i32 {
    if month_count <= 0 || month_count > 10_000_000 {
        return 7;
    }
    month_count * 30 + month_count / 3 + month_count / 12
}

/// Checks that an integer represents a valid RGB color.
///
/// Valid colors are in the range [0, 0xFFFFFF] (24-bit RGB).
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:430-432`
///
/// # Arguments
///
/// * `color` - The color value to validate
///
/// # Returns
///
/// `true` if the color is valid, `false` otherwise.
///
/// # Examples
///
/// ```
/// use rustgram_misc::is_valid_color;
///
/// assert!(is_valid_color(0x000000));
/// assert!(is_valid_color(0xFFFFFF));
/// assert!(is_valid_color(0xFF0000)); // Red
/// assert!(is_valid_color(0x00FF00)); // Green
/// assert!(is_valid_color(0x0000FF)); // Blue
/// assert!(!is_valid_color(-1));
/// assert!(!is_valid_color(0x1000000));
/// ```
pub fn is_valid_color(color: i32) -> bool {
    (0..=0xFFFFFF).contains(&color)
}

/// Checks if a string is empty after stripping empty characters.
///
/// # TDLib Correspondence
///
/// TDLib reference: `misc.cpp:256-258`
///
/// # Arguments
///
/// * `str` - The string to check
///
/// # Returns
///
/// `true` if the string is empty after stripping, `false` otherwise.
///
/// # Examples
///
/// ```
/// use rustgram_misc::is_empty_string;
///
/// assert!(is_empty_string(""));
/// assert!(is_empty_string("   "));
/// assert!(is_empty_string("\u{A0}\u{2000}"));
/// assert!(!is_empty_string("test"));
/// assert!(!is_empty_string(" test "));
/// ```
pub fn is_empty_string(str: &str) -> bool {
    crate::string::strip_empty_characters(str, str.len(), false).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_username() {
        // Valid usernames
        assert!(is_valid_username("username"));
        assert!(is_valid_username("user_name"));
        assert!(is_valid_username("user123"));
        assert!(is_valid_username("a"));
        assert!(is_valid_username("Z"));
        assert!(is_valid_username("a1"));

        // Invalid usernames
        assert!(!is_valid_username("")); // Empty
        assert!(!is_valid_username("1numberstart")); // Starts with digit
        assert!(!is_valid_username("invalid-username")); // Contains hyphen
        assert!(!is_valid_username("user__name")); // Consecutive underscores
        assert!(!is_valid_username("username_")); // Ends with underscore
        assert!(!is_valid_username("A_")); // Ends with underscore
        assert!(!is_valid_username("a".repeat(33).as_str())); // Too long
        assert!(!is_valid_username("user@name")); // Special character
    }

    #[test]
    fn test_is_allowed_username() {
        // Allowed usernames (5+ chars, valid format, not starting with reserved prefix)
        assert!(is_allowed_username("validuser123"));
        assert!(is_allowed_username("my_username"));
        assert!(is_allowed_username("abcde")); // Exactly 5 chars
        assert!(is_allowed_username("myadmin")); // Doesn't start with "admin"
        assert!(is_allowed_username("youradmin")); // Doesn't start with "admin"
        assert!(is_allowed_username("yourtelegram")); // Doesn't start with "telegram"

        // Disallowed: reserved prefixes (any username starting with prefix)
        assert!(!is_allowed_username("admin")); // Reserved (also too short)
        assert!(!is_allowed_username("administrator")); // Starts with "admin"
        assert!(!is_allowed_username("admin_xyz")); // Starts with "admin"
        assert!(!is_allowed_username("telegram")); // Reserved (also too short)
        assert!(!is_allowed_username("telegram_news")); // Starts with "telegram"
        assert!(!is_allowed_username("support_user")); // Starts with "support"
        assert!(!is_allowed_username("security_team")); // Starts with "security"
        assert!(!is_allowed_username("settings_bot")); // Starts with "settings"
        assert!(!is_allowed_username("contacts_list")); // Starts with "contacts"
        assert!(!is_allowed_username("service_info")); // Starts with "service"
        assert!(!is_allowed_username("telegraph_news")); // Starts with "telegraph"

        // Disallowed: too short
        assert!(!is_allowed_username("Admin")); // Too short (4 chars)
        assert!(!is_allowed_username("abc")); // Too short

        // Disallowed: invalid format
        assert!(!is_allowed_username("1bad")); // Invalid format
        assert!(!is_allowed_username("bad-name")); // Invalid format
    }

    #[test]
    fn test_check_currency_amount() {
        // Valid amounts
        assert!(check_currency_amount(0));
        assert!(check_currency_amount(1));
        assert!(check_currency_amount(-1));
        assert!(check_currency_amount(1_000_000));
        assert!(check_currency_amount(-1_000_000));
        assert!(check_currency_amount(9_999_999_999_999));
        assert!(check_currency_amount(-9_999_999_999_999));

        // Invalid amounts
        assert!(!check_currency_amount(10_000_000_000_000));
        assert!(!check_currency_amount(-10_000_000_000_000));
        assert!(!check_currency_amount(i64::MAX));
        assert!(!check_currency_amount(i64::MIN));
    }

    #[test]
    fn test_validate_bot_language_code() {
        // Valid codes
        assert!(validate_bot_language_code("").is_ok());
        assert!(validate_bot_language_code("en").is_ok());
        assert!(validate_bot_language_code("ru").is_ok());
        assert!(validate_bot_language_code("de").is_ok());
        assert!(validate_bot_language_code("zh").is_ok());

        // Invalid codes
        assert!(validate_bot_language_code("INVALID").is_err());
        assert!(validate_bot_language_code("eng").is_err());
        assert!(validate_bot_language_code("e").is_err());
        assert!(validate_bot_language_code("123").is_err());
        assert!(validate_bot_language_code("e1").is_err());
    }

    #[test]
    fn test_search_strings_by_prefix() {
        let strings = vec![
            "apple".to_string(),
            "application".to_string(),
            "banana".to_string(),
            "apricot".to_string(),
            "Apple".to_string(), // Case insensitive
        ];

        // Normal search
        let results = search_strings_by_prefix(&strings, "app", 10, false);
        assert_eq!(results, vec![0, 1, 4]); // apple, application, Apple

        // With limit
        let results = search_strings_by_prefix(&strings, "app", 2, false);
        assert_eq!(results.len(), 2);

        // Empty query without return_all
        let results = search_strings_by_prefix(&strings, "", 10, false);
        assert_eq!(results, vec![]);

        // Empty query with return_all
        let results = search_strings_by_prefix(&strings, "", 10, true);
        assert_eq!(results.len(), 5);

        // No matches
        let results = search_strings_by_prefix(&strings, "xyz", 10, false);
        assert_eq!(results, vec![]);
    }

    #[test]
    fn test_get_premium_duration_month_count() {
        assert_eq!(get_premium_duration_month_count(30), 1);
        assert_eq!(get_premium_duration_month_count(60), 2);
        assert_eq!(get_premium_duration_month_count(90), 3);
        assert_eq!(get_premium_duration_month_count(120), 4);
        assert_eq!(get_premium_duration_month_count(0), 0);
        assert_eq!(get_premium_duration_month_count(-10), 0);
    }

    #[test]
    fn test_get_premium_duration_day_count() {
        // Formula: month_count * 30 + month_count / 3 + month_count / 12
        assert_eq!(get_premium_duration_day_count(1), 30); // 30 + 0 + 0 = 30
        assert_eq!(get_premium_duration_day_count(2), 60); // 60 + 0 + 0 = 60
        assert_eq!(get_premium_duration_day_count(3), 91); // 90 + 1 + 0 = 91
        assert_eq!(get_premium_duration_day_count(12), 365); // 360 + 4 + 1 = 365
        assert_eq!(get_premium_duration_day_count(0), 7); // Invalid returns 7
        assert_eq!(get_premium_duration_day_count(-1), 7); // Invalid returns 7
    }

    #[test]
    fn test_is_valid_color() {
        // Valid colors
        assert!(is_valid_color(0x000000));
        assert!(is_valid_color(0xFFFFFF));
        assert!(is_valid_color(0xFF0000)); // Red
        assert!(is_valid_color(0x00FF00)); // Green
        assert!(is_valid_color(0x0000FF)); // Blue
        assert!(is_valid_color(0x808080)); // Gray
        assert!(is_valid_color(0));

        // Invalid colors
        assert!(!is_valid_color(-1));
        assert!(!is_valid_color(0x1000000));
        assert!(!is_valid_color(i32::MIN));
        assert!(!is_valid_color(i32::MAX));
    }

    #[test]
    fn test_is_empty_string() {
        assert!(is_empty_string(""));
        assert!(is_empty_string("   "));
        assert!(is_empty_string("\t\n"));
        assert!(is_empty_string("\u{A0}\u{2000}"));
        assert!(!is_empty_string("test"));
        assert!(!is_empty_string(" test "));
        assert!(!is_empty_string("  a  "));
    }

    #[test]
    fn test_reserved_prefixes() {
        // All reserved prefixes should be rejected (also too short)
        assert!(!is_allowed_username("admin"));
        assert!(!is_allowed_username("telegram"));
        assert!(!is_allowed_username("support"));
        assert!(!is_allowed_username("security"));
        assert!(!is_allowed_username("settings"));
        assert!(!is_allowed_username("contacts"));
        assert!(!is_allowed_username("service"));
        assert!(!is_allowed_username("telegraph"));

        // Prefixes followed by underscore should also be rejected
        assert!(!is_allowed_username("admin_user"));
        assert!(!is_allowed_username("telegram_bot"));
        assert!(!is_allowed_username("support_team"));

        // Long usernames starting with reserved prefixes should be rejected
        // (the implementation uses starts_with, so ANY username starting with reserved prefix is rejected)
        assert!(!is_allowed_username("administrator123")); // Starts with "admin"
        assert!(!is_allowed_username("telegramxyz")); // Starts with "telegram"
        assert!(!is_allowed_username("supportuser")); // Starts with "support"
        assert!(!is_allowed_username("security123")); // Starts with "security"

        // Variations that don't start with reserved prefixes should be allowed
        assert!(is_allowed_username("myadmin"));
        assert!(is_allowed_username("myadmin123"));
        assert!(is_allowed_username("yourtelegram"));
        assert!(is_allowed_username("getsupport"));
    }
}
