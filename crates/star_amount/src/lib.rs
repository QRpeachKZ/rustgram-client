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

//! # Star Amount
//!
//! Represents Telegram Stars amounts with decimal precision.
//!
//! Stars are stored as two components:
//! - `star_count`: Whole stars
//! - `nanostar_count`: Fractional part (0-999,999,999, representing 0-0.999999999 stars)

use serde::{Deserialize, Serialize};

/// Maximum value for nanostar count (1 star = 1,000,000,000 nanostars).
pub const NANOSTAR_MAX: i32 = 999_999_999;

/// Represents a Telegram Stars amount with decimal precision.
///
/// # Example
///
/// ```rust
/// use rustgram_star_amount::StarAmount;
///
/// // Create from parts
/// let amount = StarAmount::from_parts(100, 500_000_000).unwrap();
/// assert_eq!(amount.star_count(), 100);
/// assert_eq!(amount.nanostar_count(), 500_000_000);
///
/// // Parse from string
/// let parsed = StarAmount::from_string("100.5").unwrap();
/// assert_eq!(parsed.star_count(), 100);
/// assert_eq!(parsed.nanostar_count(), 500_000_000);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct StarAmount {
    star_count: i64,
    nanostar_count: i32,
}

impl StarAmount {
    /// Creates a new `StarAmount` from star and nanostar parts.
    ///
    /// # Arguments
    ///
    /// * `star_count` - Whole stars
    /// * `nanostar_count` - Fractional part (0-999,999,999)
    ///
    /// # Returns
    ///
    /// Returns `None` if nanostar_count is out of range.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_amount::StarAmount;
    ///
    /// let amount = StarAmount::from_parts(100, 500_000_000).unwrap();
    /// assert_eq!(amount.star_count(), 100);
    /// ```
    pub fn from_parts(star_count: i64, nanostar_count: i32) -> Option<Self> {
        if nanostar_count < 0 {
            return None;
        }

        // Auto-carry overflow (allow values >= 1_000_000_000)
        let raw_nanos = nanostar_count as i64;
        let carry = raw_nanos / 1_000_000_000;
        let remainder = (raw_nanos % 1_000_000_000) as i32;

        let stars = star_count + carry;
        let nanos = remainder;

        // Validate final nanos are in range
        if !(0..=NANOSTAR_MAX).contains(&nanos) {
            return None;
        }

        Some(Self {
            star_count: stars,
            nanostar_count: nanos,
        })
    }

    /// Creates a new `StarAmount` from a string representation.
    ///
    /// # Arguments
    ///
    /// * `s` - String in format "stars" or "stars.nanos" (e.g., "100", "100.5")
    ///
    /// # Returns
    ///
    /// Returns `None` if parsing fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_amount::StarAmount;
    ///
    /// let amount = StarAmount::from_string("100.5").unwrap();
    /// assert_eq!(amount.star_count(), 100);
    /// assert_eq!(amount.nanostar_count(), 500_000_000);
    /// ```
    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();

        match parts.len() {
            1 => {
                // Whole number only
                let star_count: i64 = parts[0].parse().ok()?;
                Some(Self {
                    star_count,
                    nanostar_count: 0,
                })
            }
            2 => {
                // Decimal number
                let star_count: i64 = parts[0].parse().ok()?;
                let decimal_part = parts[1];

                // Pad or truncate to 9 digits
                let padded = format!("{:<9}", decimal_part);
                let truncated = &padded[..9.min(decimal_part.len())];
                let nanostar_count: i32 = format!("{:0<9}", truncated).parse().ok()?;

                Self::from_parts(star_count, nanostar_count)
            }
            _ => None,
        }
    }

    /// Returns the whole star count.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_amount::StarAmount;
    ///
    /// let amount = StarAmount::from_parts(100, 500_000_000).unwrap();
    /// assert_eq!(amount.star_count(), 100);
    /// ```
    pub fn star_count(&self) -> i64 {
        self.star_count
    }

    /// Returns the nanostar count (0-999,999,999).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_amount::StarAmount;
    ///
    /// let amount = StarAmount::from_parts(100, 500_000_000).unwrap();
    /// assert_eq!(amount.nanostar_count(), 500_000_000);
    /// ```
    pub fn nanostar_count(&self) -> i32 {
        self.nanostar_count
    }

    /// Returns `true` if the amount is positive (> 0 stars or > 0 nanostars).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_amount::StarAmount;
    ///
    /// let amount = StarAmount::from_parts(100, 0).unwrap();
    /// assert!(amount.is_positive());
    ///
    /// let zero = StarAmount::from_parts(0, 0).unwrap();
    /// assert!(!zero.is_positive());
    /// ```
    pub fn is_positive(&self) -> bool {
        self.star_count > 0 || self.nanostar_count > 0
    }

    /// Returns `true` if the amount is zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_amount::StarAmount;
    ///
    /// let amount = StarAmount::from_parts(0, 0).unwrap();
    /// assert!(amount.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.star_count == 0 && self.nanostar_count == 0
    }

    /// Converts the amount to a string representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_amount::StarAmount;
    ///
    /// let amount = StarAmount::from_parts(100, 500_000_000).unwrap();
    /// assert_eq!(amount.as_string(), "100.5");
    /// ```
    pub fn as_string(&self) -> String {
        if self.nanostar_count == 0 {
            return self.star_count.to_string();
        }

        // Convert nanostars to decimal string
        let nano_str = format!("{:09}", self.nanostar_count);
        // Remove trailing zeros
        let trimmed = nano_str.trim_end_matches('0');
        format!("{}.{}", self.star_count, trimmed)
    }
}

impl std::fmt::Display for StarAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.nanostar_count == 0 {
            write!(f, "{} Telegram Stars", self.star_count)
        } else {
            write!(f, "{} Telegram Stars", self.as_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_parts_whole() {
        let amount = StarAmount::from_parts(100, 0).unwrap();
        assert_eq!(amount.star_count(), 100);
        assert_eq!(amount.nanostar_count(), 0);
        assert!(amount.is_positive());
    }

    #[test]
    fn test_from_parts_decimal() {
        let amount = StarAmount::from_parts(100, 500_000_000).unwrap();
        assert_eq!(amount.star_count(), 100);
        assert_eq!(amount.nanostar_count(), 500_000_000);
    }

    #[test]
    fn test_from_parts_invalid_nano() {
        assert!(StarAmount::from_parts(100, -1).is_none());
        // 1_000_000_000 now triggers auto-carry, so it's valid
        let amount = StarAmount::from_parts(100, 1_000_000_000).unwrap();
        assert_eq!(amount.star_count(), 101);
        assert_eq!(amount.nanostar_count(), 0);
    }

    #[test]
    fn test_from_parts_auto_carry() {
        // 1.5 stars should carry to 2 stars
        let amount = StarAmount::from_parts(1, 1_500_000_000).unwrap();
        assert_eq!(amount.star_count(), 2);
        assert_eq!(amount.nanostar_count(), 500_000_000);
    }

    #[test]
    fn test_from_parts_multiple_carry() {
        // 3.5 stars with carry should be 4.5
        let amount = StarAmount::from_parts(3, 2_000_000_000).unwrap();
        assert_eq!(amount.star_count(), 5);
        assert_eq!(amount.nanostar_count(), 0);
    }

    #[test]
    fn test_from_string_whole() {
        let amount = StarAmount::from_string("100").unwrap();
        assert_eq!(amount.star_count(), 100);
        assert_eq!(amount.nanostar_count(), 0);
    }

    #[test]
    fn test_from_string_decimal() {
        let amount = StarAmount::from_string("100.5").unwrap();
        assert_eq!(amount.star_count(), 100);
        assert_eq!(amount.nanostar_count(), 500_000_000);
    }

    #[test]
    fn test_from_string_decimal_full_precision() {
        let amount = StarAmount::from_string("100.123456789").unwrap();
        assert_eq!(amount.star_count(), 100);
        assert_eq!(amount.nanostar_count(), 123_456_789);
    }

    #[test]
    fn test_from_string_invalid() {
        assert!(StarAmount::from_string("").is_none());
        assert!(StarAmount::from_string("abc").is_none());
        assert!(StarAmount::from_string("100.5.3").is_none());
    }

    #[test]
    fn test_from_string_zero() {
        let amount = StarAmount::from_string("0").unwrap();
        assert_eq!(amount.star_count(), 0);
        assert!(amount.is_zero());
    }

    #[test]
    fn test_is_positive() {
        let positive_stars = StarAmount::from_parts(100, 0).unwrap();
        assert!(positive_stars.is_positive());

        let positive_nanos = StarAmount::from_parts(0, 1).unwrap();
        assert!(positive_nanos.is_positive());

        let zero = StarAmount::from_parts(0, 0).unwrap();
        assert!(!zero.is_positive());
    }

    #[test]
    fn test_is_zero() {
        let zero = StarAmount::from_parts(0, 0).unwrap();
        assert!(zero.is_zero());

        let non_zero = StarAmount::from_parts(0, 1).unwrap();
        assert!(!non_zero.is_zero());
    }

    #[test]
    fn test_as_string_whole() {
        let amount = StarAmount::from_parts(100, 0).unwrap();
        assert_eq!(amount.as_string(), "100");
    }

    #[test]
    fn test_as_string_decimal() {
        let amount = StarAmount::from_parts(100, 500_000_000).unwrap();
        assert_eq!(amount.as_string(), "100.5");
    }

    #[test]
    fn test_as_string_full_precision() {
        let amount = StarAmount::from_parts(100, 123_456_789).unwrap();
        assert_eq!(amount.as_string(), "100.123456789");
    }

    #[test]
    fn test_as_string_trailing_zeros() {
        let amount = StarAmount::from_parts(100, 500_000_100).unwrap();
        assert_eq!(amount.as_string(), "100.5000001");
    }

    #[test]
    fn test_equality() {
        let amount1 = StarAmount::from_parts(100, 500_000_000).unwrap();
        let amount2 = StarAmount::from_parts(100, 500_000_000).unwrap();
        assert_eq!(amount1, amount2);

        let amount3 = StarAmount::from_parts(100, 0).unwrap();
        assert_ne!(amount1, amount3);
    }

    #[test]
    fn test_default() {
        let amount = StarAmount::default();
        assert_eq!(amount.star_count(), 0);
        assert_eq!(amount.nanostar_count(), 0);
        assert!(amount.is_zero());
    }

    #[test]
    fn test_clone() {
        let amount1 = StarAmount::from_parts(100, 500_000_000).unwrap();
        let amount2 = amount1.clone();
        assert_eq!(amount1, amount2);
    }

    #[test]
    fn test_display() {
        let amount = StarAmount::from_parts(100, 500_000_000).unwrap();
        let display_str = format!("{}", amount);
        assert!(display_str.contains("100"));
        assert!(display_str.contains("Telegram Stars"));
    }

    #[test]
    fn test_serialization() {
        let amount = StarAmount::from_parts(100, 500_000_000).unwrap();
        let json = serde_json::to_string(&amount).unwrap();
        let parsed: StarAmount = serde_json::from_str(&json).unwrap();
        assert_eq!(amount, parsed);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let amount1 = StarAmount::from_parts(100, 500_000_000).unwrap();
        let amount2 = StarAmount::from_parts(100, 500_000_000).unwrap();

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        amount1.hash(&mut hasher1);
        amount2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_from_string_short_decimal() {
        let amount = StarAmount::from_string("100.1").unwrap();
        assert_eq!(amount.star_count(), 100);
        assert_eq!(amount.nanostar_count(), 100_000_000);
    }

    #[test]
    fn test_from_string_medium_decimal() {
        let amount = StarAmount::from_string("100.123").unwrap();
        assert_eq!(amount.star_count(), 100);
        assert_eq!(amount.nanostar_count(), 123_000_000);
    }

    #[test]
    fn test_edge_case_max_nanos() {
        let amount = StarAmount::from_parts(0, NANOSTAR_MAX).unwrap();
        assert_eq!(amount.nanostar_count(), NANOSTAR_MAX);
        assert!(amount.is_positive());
    }

    #[test]
    fn test_edge_case_min_nanos() {
        let amount = StarAmount::from_parts(0, 0).unwrap();
        assert_eq!(amount.nanostar_count(), 0);
    }

    #[test]
    fn test_large_star_count() {
        let amount = StarAmount::from_parts(1_000_000_000, 0).unwrap();
        assert_eq!(amount.star_count(), 1_000_000_000);
    }

    #[test]
    fn test_negative_star_count() {
        // Allow negative star count for future extensibility
        let amount = StarAmount::from_parts(-100, 0).unwrap();
        assert_eq!(amount.star_count(), -100);
        // But negative star count with positive nanos is not positive
        assert!(!amount.is_positive());
    }

    #[test]
    fn test_zero_nanos_display() {
        let amount = StarAmount::from_parts(100, 0).unwrap();
        assert_eq!(amount.as_string(), "100");
    }
}
