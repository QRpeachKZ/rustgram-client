// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Rustgram CountryInfoManager
//!
//! Country information and phone number management for Telegram MTProto client.
//!
//! This crate provides types and utilities for:
//! - Country information (name, code, calling codes)
//! - Phone number validation and formatting
//! - International calling code detection
//!
//! ## Overview
//!
//! - [`CallingCodeInfo`] - Calling code with prefixes and patterns
//! - [`CountryInfo`] - Country information with name, code, calling codes
//! - [`PhoneNumberInfo`] - Phone number validation result
//! - [`CountryInfoManager`] - Manages country lists and phone info lookup
//!
//! ## Examples
//!
//! Basic phone number info lookup:
//!
//! ```no_run
//! use rustgram_country_info_manager::CountryInfoManager;
//!
//! let manager = CountryInfoManager::new();
//! let info = manager.get_phone_number_info_sync("en", "+14155552671");
//! println!("Country: {}", info.country.unwrap_or_default());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::bool_to_int_with_if)]
#![allow(clippy::collapsible_else_if)]

use std::collections::HashMap;
use std::sync::RwLock;
use thiserror::Error;

/// Errors that can occur in country info operations.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum CountryError {
    /// Invalid country code
    #[error("Invalid country code: {0}")]
    InvalidCountryCode(String),

    /// Invalid phone number format
    #[error("Invalid phone number format: {0}")]
    InvalidPhoneNumber(String),

    /// Country list not loaded
    #[error("Country list not loaded for language: {0}")]
    CountryListNotLoaded(String),

    /// Invalid language code
    #[error("Invalid language code: {0}")]
    InvalidLanguageCode(String),
}

/// Calling code information with prefixes and formatting patterns.
///
/// # Examples
///
/// ```
/// use rustgram_country_info_manager::CallingCodeInfo;
///
/// let info = CallingCodeInfo {
///     calling_code: "+1".to_string(),
///     prefixes: vec!["415".to_string(), "650".to_string()],
///     patterns: vec!["XXX XXX XXXX".to_string()],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallingCodeInfo {
    /// International calling code (e.g., "+1", "+44")
    pub calling_code: String,

    /// Valid prefixes for this calling code
    pub prefixes: Vec<String>,

    /// Formatting patterns (X = digit, other chars preserved)
    pub patterns: Vec<String>,
}

impl CallingCodeInfo {
    /// Creates a new calling code info.
    #[must_use]
    pub const fn new(calling_code: String, prefixes: Vec<String>, patterns: Vec<String>) -> Self {
        Self {
            calling_code,
            prefixes,
            patterns,
        }
    }

    /// Checks if a phone number starts with this calling code.
    #[must_use]
    pub fn matches(&self, phone_number: &str) -> bool {
        phone_number.starts_with(&self.calling_code)
    }

    /// Gets the length of the calling code.
    #[must_use]
    pub fn len(&self) -> usize {
        self.calling_code.len()
    }
}

/// Country information with name, code, and calling codes.
///
/// # Examples
///
/// ```
/// use rustgram_country_info_manager::{CountryInfo, CallingCodeInfo};
///
/// let country = CountryInfo {
///     country_code: "US".to_string(),
///     default_name: "United States".to_string(),
///     name: "United States".to_string(),
///     calling_codes: vec![
///         CallingCodeInfo::new(
///             "+1".to_string(),
///             vec!["415".to_string()],
///             vec!["XXX XXX XXXX".to_string()],
///         ),
///     ],
///     is_hidden: false,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CountryInfo {
    /// ISO 3166-1 alpha-2 country code
    pub country_code: String,

    /// Default (English) name
    pub default_name: String,

    /// Localized name (if available)
    pub name: String,

    /// Calling codes for this country
    pub calling_codes: Vec<CallingCodeInfo>,

    /// Whether this country should be hidden from lists
    pub is_hidden: bool,
}

impl CountryInfo {
    /// Creates a new country info.
    #[must_use]
    pub const fn new(
        country_code: String,
        default_name: String,
        name: String,
        calling_codes: Vec<CallingCodeInfo>,
        is_hidden: bool,
    ) -> Self {
        Self {
            country_code,
            default_name,
            name,
            calling_codes,
            is_hidden,
        }
    }

    /// Gets the display name (localized or default).
    #[must_use]
    pub fn display_name(&self) -> &str {
        if self.name.is_empty() {
            &self.default_name
        } else {
            &self.name
        }
    }

    /// Gets all calling codes as strings.
    #[must_use]
    pub fn calling_codes_strings(&self) -> Vec<&str> {
        self.calling_codes
            .iter()
            .map(|c| c.calling_code.as_str())
            .collect()
    }
}

/// Phone number validation and formatting result.
///
/// # Examples
///
/// ```
/// use rustgram_country_info_manager::PhoneNumberInfo;
///
/// let info = PhoneNumberInfo {
///     country: Some("US".to_string()),
///     country_code: Some("+1".to_string()),
///     formatted: "+1 (415) 555-2671".to_string(),
///     is_anonymous: false,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneNumberInfo {
    /// ISO country code (if identified)
    pub country: Option<String>,

    /// Calling code (if identified)
    pub country_code: Option<String>,

    /// Formatted phone number
    pub formatted: String,

    /// Whether this is an anonymous/fragment number
    pub is_anonymous: bool,
}

impl PhoneNumberInfo {
    /// Creates a new phone number info.
    #[must_use]
    pub const fn new(
        country: Option<String>,
        country_code: Option<String>,
        formatted: String,
        is_anonymous: bool,
    ) -> Self {
        Self {
            country,
            country_code,
            formatted,
            is_anonymous,
        }
    }

    /// Creates an unknown phone number info.
    #[must_use]
    pub fn unknown(phone_number: String, is_anonymous: bool) -> Self {
        Self {
            country: None,
            country_code: None,
            formatted: phone_number,
            is_anonymous,
        }
    }

    /// Checks if the country was identified.
    #[must_use]
    pub fn is_identified(&self) -> bool {
        self.country.is_some()
    }
}

impl Default for PhoneNumberInfo {
    fn default() -> Self {
        Self {
            country: None,
            country_code: None,
            formatted: String::new(),
            is_anonymous: false,
        }
    }
}

/// Country list for a specific language.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CountryList {
    /// List of countries
    countries: Vec<CountryInfo>,

    /// Hash for cache invalidation
    hash: i32,
}

impl CountryList {
    /// Creates a new country list.
    #[must_use]
    pub const fn new(countries: Vec<CountryInfo>, hash: i32) -> Self {
        Self { countries, hash }
    }
}

/// Country information manager.
///
/// Manages country lists and provides phone number validation.
///
/// # Examples
///
/// ```
/// use rustgram_country_info_manager::CountryInfoManager;
///
/// let manager = CountryInfoManager::new();
/// let info = manager.get_phone_number_info_sync("en", "+14155552671");
/// ```
#[derive(Debug)]
pub struct CountryInfoManager {
    /// Country lists by language code
    country_lists: RwLock<HashMap<String, CountryList>>,

    /// Fragment phone number prefixes (anonymous numbers)
    fragment_prefixes: Vec<String>,
}

impl Default for CountryInfoManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CountryInfoManager {
    /// Creates a new country info manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_country_info_manager::CountryInfoManager;
    ///
    /// let manager = CountryInfoManager::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            country_lists: RwLock::new(HashMap::new()),
            fragment_prefixes: vec!["888".to_string()],
        }
    }

    /// Gets a country flag emoji from a country code.
    ///
    /// # Arguments
    ///
    /// * `country_code` - ISO 3166-1 alpha-2 country code (e.g., "US", "GB")
    ///
    /// # Returns
    ///
    /// The flag emoji for the country, or empty string if not found
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_country_info_manager::CountryInfoManager;
    ///
    /// let flag = CountryInfoManager::get_country_flag_emoji("US");
    /// assert_eq!(flag, "🇺🇸");
    ///
    /// let flag = CountryInfoManager::get_country_flag_emoji("GB");
    /// assert_eq!(flag, "🇬🇧");
    /// ```
    #[must_use]
    pub fn get_country_flag_emoji(country_code: &str) -> String {
        if country_code.len() != 2 {
            return String::new();
        }

        let mut chars = country_code.chars();
        let first = chars.next().unwrap_or(' ');
        let second = chars.next().unwrap_or(' ');

        if !first.is_ascii_uppercase() || !second.is_ascii_uppercase() {
            return String::new();
        }

        // Convert A-Z to regional indicator symbols
        let flag_a = 0x1F1E6; // 🇦
        let base = flag_a - ('A' as u32);

        let first_flag = char::from_u32(base + (first as u32)).unwrap_or(' ');
        let second_flag = char::from_u32(base + (second as u32)).unwrap_or(' ');

        format!("{}{}", first_flag, second_flag)
    }

    /// Cleans a phone number (removes non-digit characters except leading +).
    ///
    /// # Arguments
    ///
    /// * `phone_number` - The phone number to clean
    ///
    /// # Returns
    ///
    /// The cleaned phone number
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_country_info_manager::CountryInfoManager;
    ///
    /// let cleaned = CountryInfoManager::clean_phone_number("+1 (415) 555-2671");
    /// assert_eq!(cleaned, "+14155552671");
    /// ```
    #[must_use]
    pub fn clean_phone_number(phone_number: &str) -> String {
        let mut result = String::new();
        let mut has_plus = false;

        for c in phone_number.chars() {
            if c == '+' && !has_plus && result.is_empty() {
                result.push(c);
                has_plus = true;
            } else if c.is_ascii_digit() {
                result.push(c);
            }
        }

        result
    }

    /// Checks if a phone number is a fragment (anonymous) number.
    ///
    /// # Arguments
    ///
    /// * `phone_number` - The phone number to check
    ///
    /// # Returns
    ///
    /// `true` if this is a fragment number
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_country_info_manager::CountryInfoManager;
    ///
    /// assert!(CountryInfoManager::is_fragment_phone_number("+888123456789"));
    /// assert!(!CountryInfoManager::is_fragment_phone_number("+14155552671"));
    /// ```
    #[must_use]
    pub fn is_fragment_phone_number(phone_number: &str) -> bool {
        if phone_number.is_empty() {
            return false;
        }

        let cleaned = Self::clean_phone_number(phone_number);
        if cleaned.is_empty() {
            return false;
        }

        // Check against fragment prefixes (default: 888)
        for prefix in &["888", "886", "887"] {
            if cleaned.starts_with(&format!("+{}", prefix)) {
                return true;
            }
        }

        false
    }

    /// Gets phone number info synchronously (using cached country list).
    ///
    /// # Arguments
    ///
    /// * `language_code` - Language code for country names (e.g., "en", "ru")
    /// * `phone_number_prefix` - Phone number to analyze
    ///
    /// # Returns
    ///
    /// Phone number information
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_country_info_manager::CountryInfoManager;
    ///
    /// let manager = CountryInfoManager::new();
    /// let info = manager.get_phone_number_info_sync("en", "+14155552671");
    /// ```
    #[must_use]
    pub fn get_phone_number_info_sync(
        &self,
        language_code: &str,
        phone_number_prefix: &str,
    ) -> PhoneNumberInfo {
        let phone_number = Self::clean_phone_number(phone_number_prefix);

        if phone_number.is_empty() {
            return PhoneNumberInfo::default();
        }

        let is_anonymous = Self::is_fragment_phone_number(&phone_number);

        // Get country list
        let lists = self.country_lists.read().unwrap();
        let list = lists.get(language_code).or_else(|| lists.get("en"));

        let Some(list) = list else {
            return PhoneNumberInfo::unknown(phone_number, is_anonymous);
        };

        self.get_phone_number_info_from_list(list, &phone_number, is_anonymous)
    }

    /// Internal: Gets phone number info from a country list.
    fn get_phone_number_info_from_list(
        &self,
        list: &CountryList,
        phone_number: &str,
        is_anonymous: bool,
    ) -> PhoneNumberInfo {
        let mut best_country: Option<&CountryInfo> = None;
        let mut best_calling_code: Option<&CallingCodeInfo> = None;
        let mut best_length = 0;
        let mut is_prefix = false;

        for country in &list.countries {
            for calling_code in &country.calling_codes {
                if phone_number.starts_with(&calling_code.calling_code) {
                    let code_size = calling_code.calling_code.len();

                    // Check if phone number without code matches a prefix
                    let after_code = &phone_number[code_size..];

                    for prefix in &calling_code.prefixes {
                        // Check if prefix is a prefix of after_code
                        if prefix.starts_with(after_code) {
                            is_prefix = true;
                        }

                        // Check if after_code starts with prefix
                        if after_code.starts_with(prefix) {
                            let match_length = code_size + prefix.len();
                            if match_length > best_length {
                                best_country = Some(country);
                                best_calling_code = Some(calling_code);
                                best_length = match_length;
                            }
                        }
                    }
                }

                // Check if calling code is a prefix of phone number
                if calling_code.calling_code.starts_with(phone_number) {
                    is_prefix = true;
                }
            }
        }

        let Some((country, calling_code)) = best_country.zip(best_calling_code) else {
            return PhoneNumberInfo::unknown(
                if is_prefix {
                    phone_number.to_string()
                } else {
                    String::new()
                },
                is_anonymous,
            );
        };

        // Format the phone number
        let after_code = &phone_number[calling_code.calling_code.len()..];
        let formatted = self.format_phone_number(calling_code, after_code);

        PhoneNumberInfo {
            country: Some(country.country_code.clone()),
            country_code: Some(calling_code.calling_code.clone()),
            formatted: format!("{}{}", calling_code.calling_code, formatted),
            is_anonymous,
        }
    }

    /// Formats a phone number using calling code patterns.
    fn format_phone_number(&self, calling_code: &CallingCodeInfo, number: &str) -> String {
        for pattern in &calling_code.patterns {
            if let Some(formatted) = self.apply_pattern(number, pattern) {
                return formatted;
            }
        }
        number.to_string()
    }

    /// Applies a formatting pattern to a phone number.
    fn apply_pattern(&self, number: &str, pattern: &str) -> Option<String> {
        let mut result = String::new();
        let mut pattern_chars = pattern.chars().peekable();
        let mut number_chars = number.chars();

        while let Some(pc) = pattern_chars.next() {
            if pc == 'X' {
                if let Some(nc) = number_chars.next() {
                    result.push(nc);
                } else {
                    break;
                }
            } else if pc.is_ascii_digit() {
                // Pattern has specific digit, must match
                if let Some(nc) = number_chars.next() {
                    if nc == pc {
                        result.push(nc);
                    } else {
                        return None; // Pattern doesn't match
                    }
                } else {
                    return None; // Number too short
                }
            } else {
                result.push(pc);
            }
        }

        // Add remaining digits
        for nc in number_chars {
            result.push(nc);
        }

        // Trim trailing whitespace
        Some(result.trim_end().to_string())
    }

    /// Adds a country list for a language.
    ///
    /// # Arguments
    ///
    /// * `language_code` - Language code (e.g., "en", "ru")
    /// * `countries` - List of countries
    /// * `hash` - Hash for cache invalidation
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_country_info_manager::{CountryInfoManager, CountryInfo, CallingCodeInfo};
    ///
    /// let mut manager = CountryInfoManager::new();
    /// manager.add_country_list("en".to_string(), vec![
    ///     CountryInfo::new(
    ///         "US".to_string(),
    ///         "United States".to_string(),
    ///         "United States".to_string(),
    ///         vec![CallingCodeInfo::new(
    ///             "+1".to_string(),
    ///             vec![String::new()], // Empty prefix matches any number
    ///             vec!["XXX XXX XXXX".to_string()],
    ///         )],
    ///         false,
    ///     ),
    /// ], 0);
    /// ```
    pub fn add_country_list(&self, language_code: String, countries: Vec<CountryInfo>, hash: i32) {
        let mut lists = self.country_lists.write().unwrap();
        lists.insert(language_code, CountryList::new(countries, hash));
    }

    /// Gets all countries for a language.
    ///
    /// # Arguments
    ///
    /// * `language_code` - Language code (e.g., "en", "ru")
    ///
    /// # Returns
    ///
    /// Vector of country information, or error if not loaded
    ///
    /// # Errors
    ///
    /// Returns `CountryError::CountryListNotLoaded` if the list isn't loaded
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_country_info_manager::CountryInfoManager;
    ///
    /// let manager = CountryInfoManager::new();
    /// match manager.get_countries("en") {
    ///     Ok(countries) => println!("Found {} countries", countries.len()),
    ///     Err(e) => eprintln!("Error: {}", e),
    /// }
    /// ```
    pub fn get_countries(&self, language_code: &str) -> Result<Vec<CountryInfo>, CountryError> {
        let lists = self.country_lists.read().unwrap();
        let list = lists
            .get(language_code)
            .ok_or_else(|| CountryError::CountryListNotLoaded(language_code.to_string()))?;

        Ok(list.countries.clone())
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-country-info-manager";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== CallingCodeInfo Tests ==========

    #[test]
    fn test_calling_code_info_new() {
        let info = CallingCodeInfo::new(
            "+1".to_string(),
            vec!["415".to_string()],
            vec!["XXX XXX XXXX".to_string()],
        );
        assert_eq!(info.calling_code, "+1");
        assert_eq!(info.prefixes.len(), 1);
        assert_eq!(info.patterns.len(), 1);
    }

    #[test]
    fn test_calling_code_info_matches() {
        let info = CallingCodeInfo::new("+1".to_string(), vec![], vec![]);
        assert!(info.matches("+14155552671"));
        assert!(!info.matches("+441234567890"));
    }

    #[test]
    fn test_calling_code_info_len() {
        let info = CallingCodeInfo::new("+1".to_string(), vec![], vec![]);
        assert_eq!(info.len(), 2);
    }

    // ========== CountryInfo Tests ==========

    #[test]
    fn test_country_info_new() {
        let country = CountryInfo::new(
            "US".to_string(),
            "United States".to_string(),
            "United States".to_string(),
            vec![],
            false,
        );
        assert_eq!(country.country_code, "US");
        assert_eq!(country.default_name, "United States");
        assert!(!country.is_hidden);
    }

    #[test]
    fn test_country_info_display_name() {
        let country_with_name = CountryInfo::new(
            "US".to_string(),
            "United States".to_string(),
            "Estados Unidos".to_string(),
            vec![],
            false,
        );
        assert_eq!(country_with_name.display_name(), "Estados Unidos");

        let country_without_name = CountryInfo::new(
            "US".to_string(),
            "United States".to_string(),
            String::new(),
            vec![],
            false,
        );
        assert_eq!(country_without_name.display_name(), "United States");
    }

    #[test]
    fn test_country_info_calling_codes_strings() {
        let codes = vec![
            CallingCodeInfo::new("+1".to_string(), vec![], vec![]),
            CallingCodeInfo::new("+44".to_string(), vec![], vec![]),
        ];
        let country = CountryInfo::new(
            "US".to_string(),
            "United States".to_string(),
            "United States".to_string(),
            codes,
            false,
        );
        assert_eq!(country.calling_codes_strings(), vec!["+1", "+44"]);
    }

    // ========== PhoneNumberInfo Tests ==========

    #[test]
    fn test_phone_number_info_new() {
        let info = PhoneNumberInfo::new(
            Some("US".to_string()),
            Some("+1".to_string()),
            "+14155552671".to_string(),
            false,
        );
        assert_eq!(info.country, Some("US".to_string()));
        assert_eq!(info.country_code, Some("+1".to_string()));
        assert!(!info.is_anonymous);
    }

    #[test]
    fn test_phone_number_info_unknown() {
        let info = PhoneNumberInfo::unknown("+12345".to_string(), true);
        assert!(info.country.is_none());
        assert!(info.country_code.is_none());
        assert!(info.is_anonymous);
    }

    #[test]
    fn test_phone_number_info_is_identified() {
        let identified = PhoneNumberInfo::new(
            Some("US".to_string()),
            Some("+1".to_string()),
            "+1".to_string(),
            false,
        );
        assert!(identified.is_identified());

        let unknown = PhoneNumberInfo::unknown("+12345".to_string(), false);
        assert!(!unknown.is_identified());
    }

    #[test]
    fn test_phone_number_info_default() {
        let info = PhoneNumberInfo::default();
        assert!(info.country.is_none());
        assert!(info.country_code.is_none());
        assert!(info.formatted.is_empty());
        assert!(!info.is_anonymous);
    }

    // ========== CountryInfoManager Tests ==========

    #[test]
    fn test_manager_new() {
        let manager = CountryInfoManager::new();
        // Just check it exists and doesn't panic
        assert_eq!(manager.fragment_prefixes.len(), 1);
    }

    #[test]
    fn test_manager_default() {
        let manager = CountryInfoManager::default();
        assert_eq!(manager.fragment_prefixes.len(), 1);
    }

    // ========== Country Flag Tests ==========

    #[test]
    fn test_get_country_flag_emoji_us() {
        assert_eq!(CountryInfoManager::get_country_flag_emoji("US"), "🇺🇸");
    }

    #[test]
    fn test_get_country_flag_emoji_gb() {
        assert_eq!(CountryInfoManager::get_country_flag_emoji("GB"), "🇬🇧");
    }

    #[test]
    fn test_get_country_flag_emoji_invalid_length() {
        assert_eq!(CountryInfoManager::get_country_flag_emoji("USA"), "");
        assert_eq!(CountryInfoManager::get_country_flag_emoji("U"), "");
    }

    #[test]
    fn test_get_country_flag_emoji_non_alpha() {
        assert_eq!(CountryInfoManager::get_country_flag_emoji("1A"), "");
        assert_eq!(CountryInfoManager::get_country_flag_emoji(" "), "");
    }

    // ========== Clean Phone Number Tests ==========

    #[test]
    fn test_clean_phone_number_basic() {
        assert_eq!(
            CountryInfoManager::clean_phone_number("+1 (415) 555-2671"),
            "+14155552671"
        );
    }

    #[test]
    fn test_clean_phone_number_with_spaces() {
        assert_eq!(
            CountryInfoManager::clean_phone_number("+1 415 555 2671"),
            "+14155552671"
        );
    }

    #[test]
    fn test_clean_phone_number_with_dashes() {
        assert_eq!(
            CountryInfoManager::clean_phone_number("+1-415-555-2671"),
            "+14155552671"
        );
    }

    #[test]
    fn test_clean_phone_number_empty() {
        assert_eq!(CountryInfoManager::clean_phone_number(""), "");
    }

    #[test]
    fn test_clean_phone_number_without_plus() {
        assert_eq!(
            CountryInfoManager::clean_phone_number("14155552671"),
            "14155552671"
        );
    }

    // ========== Fragment Phone Number Tests ==========

    #[test]
    fn test_is_fragment_phone_number_888() {
        assert!(CountryInfoManager::is_fragment_phone_number(
            "+888123456789"
        ));
    }

    #[test]
    fn test_is_fragment_phone_number_886() {
        assert!(CountryInfoManager::is_fragment_phone_number(
            "+886123456789"
        ));
    }

    #[test]
    fn test_is_fragment_phone_number_887() {
        assert!(CountryInfoManager::is_fragment_phone_number(
            "+887123456789"
        ));
    }

    #[test]
    fn test_is_fragment_phone_number_normal() {
        assert!(!CountryInfoManager::is_fragment_phone_number(
            "+14155552671"
        ));
    }

    #[test]
    fn test_is_fragment_phone_number_empty() {
        assert!(!CountryInfoManager::is_fragment_phone_number(""));
    }

    #[test]
    fn test_is_fragment_phone_number_formatted() {
        assert!(CountryInfoManager::is_fragment_phone_number(
            "+888 (123) 456-789"
        ));
    }

    // ========== Add/Get Country List Tests ==========

    #[test]
    fn test_add_country_list() {
        let manager = CountryInfoManager::new();
        manager.add_country_list(
            "en".to_string(),
            vec![CountryInfo::new(
                "US".to_string(),
                "United States".to_string(),
                "United States".to_string(),
                vec![CallingCodeInfo::new(
                    "+1".to_string(),
                    vec![],
                    vec!["XXX XXX XXXX".to_string()],
                )],
                false,
            )],
            0,
        );

        let countries = manager.get_countries("en").unwrap();
        assert_eq!(countries.len(), 1);
        assert_eq!(countries[0].country_code, "US");
    }

    #[test]
    fn test_get_countries_not_loaded() {
        let manager = CountryInfoManager::new();
        match manager.get_countries("en") {
            Err(CountryError::CountryListNotLoaded(_)) => {}
            _ => panic!("Expected CountryListNotLoaded error"),
        }
    }

    #[test]
    fn test_get_phone_number_info_sync_basic() {
        let manager = CountryInfoManager::new();
        manager.add_country_list(
            "en".to_string(),
            vec![CountryInfo::new(
                "US".to_string(),
                "United States".to_string(),
                "United States".to_string(),
                vec![CallingCodeInfo::new(
                    "+1".to_string(),
                    vec![String::new()], // Empty prefix matches any number
                    vec!["XXX XXX XXXX".to_string()],
                )],
                false,
            )],
            0,
        );

        let info = manager.get_phone_number_info_sync("en", "+14155552671");
        assert_eq!(info.country, Some("US".to_string()));
        assert_eq!(info.country_code, Some("+1".to_string()));
    }

    #[test]
    fn test_get_phone_number_info_sync_empty() {
        let manager = CountryInfoManager::new();
        let info = manager.get_phone_number_info_sync("en", "");
        assert_eq!(info.formatted, "");
    }

    #[test]
    fn test_get_phone_number_info_sync_anonymous() {
        let manager = CountryInfoManager::new();
        let info = manager.get_phone_number_info_sync("en", "+888123456789");
        assert!(info.is_anonymous);
    }

    // ========== Apply Pattern Tests ==========

    #[test]
    fn test_apply_pattern_basic() {
        let manager = CountryInfoManager::new();
        let result = manager.apply_pattern("4155552671", "XXX XXX XXXX");
        assert_eq!(result, Some("415 555 2671".to_string()));
    }

    #[test]
    fn test_apply_pattern_with_specific_digit() {
        let manager = CountryInfoManager::new();
        let result = manager.apply_pattern("4155552671", "415 XXX XXXX");
        assert_eq!(result, Some("415 555 2671".to_string()));
    }

    #[test]
    fn test_apply_pattern_mismatch() {
        let manager = CountryInfoManager::new();
        let result = manager.apply_pattern("3155552671", "415 XXX XXXX");
        assert_eq!(result, None);
    }

    #[test]
    fn test_apply_pattern_shorter() {
        let manager = CountryInfoManager::new();
        let result = manager.apply_pattern("415555", "XXX XXX XXXX");
        assert_eq!(result, Some("415 555".to_string()));
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-country-info-manager");
    }
}
