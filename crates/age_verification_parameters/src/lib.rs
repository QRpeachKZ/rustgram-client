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

//! # Age Verification Parameters
//!
//! Parameters for age verification in Telegram.
//!
//! Age verification is required in certain jurisdictions for accessing restricted content.

use serde::{Deserialize, Serialize};

/// Represents age verification parameters.
///
/// # Example
///
/// ```rust
/// use rustgram_age_verification_parameters::AgeVerificationParameters;
///
/// // Create age verification parameters
/// let params = AgeVerificationParameters::with_params(
///     true,
///     "@verifier_bot",
///     "US",
///     18
/// ).unwrap();
///
/// assert!(params.need_verification());
/// assert_eq!(params.bot_username(), Some("@verifier_bot"));
/// assert_eq!(params.country(), Some("US"));
/// assert_eq!(params.min_age(), Some(18));
/// assert!(params.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct AgeVerificationParameters {
    need_verification: bool,
    bot_username: Option<String>,
    country: Option<String>,
    min_age: Option<i32>,
}

impl AgeVerificationParameters {
    /// Creates age verification parameters.
    ///
    /// # Arguments
    ///
    /// * `need_verification` - Whether age verification is needed
    /// * `bot_username` - Username of the verification bot (with @ prefix)
    /// * `country` - ISO 3166-1 alpha-2 country code (e.g., "US")
    /// * `min_age` - Minimum age required (0-120)
    ///
    /// # Returns
    ///
    /// Returns `None` if parameters are invalid.
    ///
    /// # Validation Rules
    ///
    /// - If `need_verification` is true:
    ///   - `bot_username` must be non-empty
    ///   - `country` must be a 2-letter ISO code
    ///   - `min_age` must be > 0
    /// - If `need_verification` is false:
    ///   - All other fields must be None/empty
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_age_verification_parameters::AgeVerificationParameters;
    ///
    /// let params = AgeVerificationParameters::with_params(
    ///     true,
    ///     "@verifier_bot",
    ///     "US",
    ///     18
    /// ).unwrap();
    /// ```
    pub fn with_params(
        need_verification: bool,
        bot_username: &str,
        country: &str,
        min_age: i32,
    ) -> Option<Self> {
        if need_verification {
            // Validate required fields
            if bot_username.is_empty() || country.is_empty() || min_age <= 0 {
                return None;
            }

            // Validate country code (2 letters)
            if country.len() != 2 || !country.chars().all(|c| c.is_ascii_alphabetic()) {
                return None;
            }

            // Validate min_age range (0-120)
            if min_age > 120 {
                return None;
            }

            Some(Self {
                need_verification: true,
                bot_username: Some(bot_username.to_string()),
                country: Some(country.to_string().to_uppercase()),
                min_age: Some(min_age),
            })
        } else {
            // If verification not needed, all fields should be empty
            if !bot_username.is_empty() || !country.is_empty() || min_age != 0 {
                return None;
            }

            Some(Self {
                need_verification: false,
                bot_username: None,
                country: None,
                min_age: None,
            })
        }
    }

    /// Returns `true` if age verification is needed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_age_verification_parameters::AgeVerificationParameters;
    ///
    /// let params = AgeVerificationParameters::with_params(
    ///     true,
    ///     "@bot",
    ///     "US",
    ///     18
    /// ).unwrap();
    /// assert!(params.need_verification());
    /// ```
    pub fn need_verification(&self) -> bool {
        self.need_verification
    }

    /// Returns the verification bot username (with @ prefix).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_age_verification_parameters::AgeVerificationParameters;
    ///
    /// let params = AgeVerificationParameters::with_params(
    ///     true,
    ///     "@verifier_bot",
    ///     "US",
    ///     18
    /// ).unwrap();
    /// assert_eq!(params.bot_username(), Some("@verifier_bot"));
    /// ```
    pub fn bot_username(&self) -> Option<&str> {
        self.bot_username.as_deref()
    }

    /// Returns the ISO 3166-1 alpha-2 country code.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_age_verification_parameters::AgeVerificationParameters;
    ///
    /// let params = AgeVerificationParameters::with_params(
    ///     true,
    ///     "@bot",
    ///     "US",
    ///     18
    /// ).unwrap();
    /// assert_eq!(params.country(), Some("US"));
    /// ```
    pub fn country(&self) -> Option<&str> {
        self.country.as_deref()
    }

    /// Returns the minimum age required.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_age_verification_parameters::AgeVerificationParameters;
    ///
    /// let params = AgeVerificationParameters::with_params(
    ///     true,
    ///     "@bot",
    ///     "US",
    ///     18
    /// ).unwrap();
    /// assert_eq!(params.min_age(), Some(18));
    /// ```
    pub fn min_age(&self) -> Option<i32> {
        self.min_age
    }

    /// Returns `true` if the parameters are valid.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_age_verification_parameters::AgeVerificationParameters;
    ///
    /// let params = AgeVerificationParameters::with_params(
    ///     true,
    ///     "@bot",
    ///     "US",
    ///     18
    /// ).unwrap();
    /// assert!(params.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        if self.need_verification {
            self.bot_username.is_some()
                && self.country.is_some()
                && self.min_age.map(|age| age > 0).unwrap_or(false)
        } else {
            self.bot_username.is_none() && self.country.is_none() && self.min_age.is_none()
        }
    }
}

impl std::fmt::Display for AgeVerificationParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.need_verification {
            write!(f, "[no age verification]")
        } else {
            write!(
                f,
                "verify age of {} years for country {} via bot {}",
                self.min_age.unwrap_or(0),
                self.country.as_deref().unwrap_or(""),
                self.bot_username.as_deref().unwrap_or("")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_params_valid() {
        let params = AgeVerificationParameters::with_params(true, "@bot", "US", 18).unwrap();
        assert!(params.need_verification());
        assert_eq!(params.bot_username(), Some("@bot"));
        assert_eq!(params.country(), Some("US"));
        assert_eq!(params.min_age(), Some(18));
        assert!(params.is_valid());
    }

    #[test]
    fn test_with_params_no_verification() {
        let params = AgeVerificationParameters::with_params(false, "", "", 0).unwrap();
        assert!(!params.need_verification());
        assert_eq!(params.bot_username(), None);
        assert_eq!(params.country(), None);
        assert_eq!(params.min_age(), None);
        assert!(params.is_valid());
    }

    #[test]
    fn test_with_params_empty_bot() {
        let params = AgeVerificationParameters::with_params(true, "", "US", 18);
        assert!(params.is_none());
    }

    #[test]
    fn test_with_params_empty_country() {
        let params = AgeVerificationParameters::with_params(true, "@bot", "", 18);
        assert!(params.is_none());
    }

    #[test]
    fn test_with_params_invalid_min_age() {
        let params = AgeVerificationParameters::with_params(true, "@bot", "US", 0);
        assert!(params.is_none());

        let params2 = AgeVerificationParameters::with_params(true, "@bot", "US", -1);
        assert!(params2.is_none());
    }

    #[test]
    fn test_with_params_country_too_long() {
        let params = AgeVerificationParameters::with_params(true, "@bot", "USA", 18);
        assert!(params.is_none());
    }

    #[test]
    fn test_with_params_country_too_short() {
        let params = AgeVerificationParameters::with_params(true, "@bot", "U", 18);
        assert!(params.is_none());
    }

    #[test]
    fn test_with_params_country_uppercase() {
        let params = AgeVerificationParameters::with_params(true, "@bot", "us", 18).unwrap();
        assert_eq!(params.country(), Some("US"));
    }

    #[test]
    fn test_with_params_min_age_too_high() {
        let params = AgeVerificationParameters::with_params(true, "@bot", "US", 121);
        assert!(params.is_none());
    }

    #[test]
    fn test_with_params_no_verification_with_fields() {
        let params = AgeVerificationParameters::with_params(false, "@bot", "US", 18);
        assert!(params.is_none());
    }

    #[test]
    fn test_is_valid_with_verification() {
        let params = AgeVerificationParameters::with_params(true, "@bot", "US", 18).unwrap();
        assert!(params.is_valid());
    }

    #[test]
    fn test_is_valid_without_verification() {
        let params = AgeVerificationParameters::default();
        assert!(params.is_valid());
    }

    #[test]
    fn test_common_countries() {
        let countries = ["US", "GB", "CA", "DE", "FR", "JP", "AU", "BR", "IN", "CN"];

        for country in countries {
            let params = AgeVerificationParameters::with_params(true, "@bot", country, 18);
            assert!(params.is_some(), "Country {} should be valid", country);
        }
    }

    #[test]
    fn test_min_age_range() {
        // Valid ages
        for age in [1, 13, 16, 18, 21, 100, 120] {
            let params = AgeVerificationParameters::with_params(true, "@bot", "US", age);
            assert!(params.is_some(), "Age {} should be valid", age);
        }
    }

    #[test]
    fn test_min_age_boundary() {
        // Maximum valid age
        let params = AgeVerificationParameters::with_params(true, "@bot", "US", 120);
        assert!(params.is_some());

        // Minimum valid age
        let params2 = AgeVerificationParameters::with_params(true, "@bot", "US", 1);
        assert!(params2.is_some());
    }

    #[test]
    fn test_equality() {
        let params1 = AgeVerificationParameters::with_params(true, "@bot", "US", 18).unwrap();
        let params2 = AgeVerificationParameters::with_params(true, "@bot", "US", 18).unwrap();
        assert_eq!(params1, params2);

        let params3 = AgeVerificationParameters::with_params(true, "@bot", "GB", 18).unwrap();
        assert_ne!(params1, params3);
    }

    #[test]
    fn test_default() {
        let params = AgeVerificationParameters::default();
        assert!(!params.need_verification());
        assert!(params.is_valid());
    }

    #[test]
    fn test_clone() {
        let params1 = AgeVerificationParameters::with_params(true, "@bot", "US", 18).unwrap();
        let params2 = params1.clone();
        assert_eq!(params1, params2);
    }

    #[test]
    fn test_display_with_verification() {
        let params = AgeVerificationParameters::with_params(true, "@bot", "US", 18).unwrap();
        let display_str = format!("{}", params);
        assert!(display_str.contains("18"));
        assert!(display_str.contains("US"));
        assert!(display_str.contains("@bot"));
    }

    #[test]
    fn test_display_without_verification() {
        let params = AgeVerificationParameters::default();
        let display_str = format!("{}", params);
        assert!(display_str.contains("no age verification"));
    }

    #[test]
    fn test_serialization() {
        let params = AgeVerificationParameters::with_params(true, "@bot", "US", 18).unwrap();
        let json = serde_json::to_string(&params).unwrap();
        let parsed: AgeVerificationParameters = serde_json::from_str(&json).unwrap();
        assert_eq!(params, parsed);
    }

    #[test]
    fn test_serialization_no_verification() {
        let params = AgeVerificationParameters::default();
        let json = serde_json::to_string(&params).unwrap();
        let parsed: AgeVerificationParameters = serde_json::from_str(&json).unwrap();
        assert_eq!(params, parsed);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let params1 = AgeVerificationParameters::with_params(true, "@bot", "US", 18).unwrap();
        let params2 = AgeVerificationParameters::with_params(true, "@bot", "US", 18).unwrap();

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        params1.hash(&mut hasher1);
        params2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_invalid_country_non_alpha() {
        let params = AgeVerificationParameters::with_params(true, "@bot", "U1", 18);
        assert!(params.is_none());

        let params2 = AgeVerificationParameters::with_params(true, "@bot", "1U", 18);
        assert!(params2.is_none());
    }

    #[test]
    fn test_bot_username_formats() {
        let valid_usernames = ["@bot", "@verifier_bot", "@my_bot123"];

        for username in valid_usernames {
            let params = AgeVerificationParameters::with_params(true, username, "US", 18);
            assert!(params.is_some(), "Username {} should be valid", username);
        }
    }

    #[test]
    fn test_common_min_ages() {
        let ages = [
            (13, "PG-13"),
            (16, "Teen"),
            (18, "Adult"),
            (21, "US Drinking Age"),
        ];

        for (age, _label) in ages {
            let params = AgeVerificationParameters::with_params(true, "@bot", "US", age);
            assert!(params.is_some(), "Age {} should be valid", age);
        }
    }

    #[test]
    fn test_debug_format() {
        let params = AgeVerificationParameters::with_params(true, "@bot", "US", 18).unwrap();
        let debug_str = format!("{:?}", params);
        assert!(debug_str.contains("AgeVerificationParameters"));
    }
}
