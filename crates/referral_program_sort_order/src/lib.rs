// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Referral Program Sort Order
//!
//! Sort order types for referral programs in Telegram.
//!
//! ## Overview
//!
//! This module provides the [`ReferralProgramSortOrder`] enum, which represents
//! the different sort orders for referral/affiliate programs in Telegram. It mirrors
//! TDLib's `ReferralProgramSortOrder` enum, providing type-safe sorting options.
//!
//! ## Sort Orders
//!
//! - [`Profitability`] - Sort by profitability (default)
//! - [`Date`] - Sort by creation date
//! - [`Revenue`] - Sort by revenue
//!
//! ## Example
//!
//! ```rust
//! use rustgram_referral_program_sort_order::ReferralProgramSortOrder;
//!
//! // Default is profitability
//! let default = ReferralProgramSortOrder::default();
//! assert_eq!(default, ReferralProgramSortOrder::Profitability);
//!
//! // Create different sort orders
//! let by_date = ReferralProgramSortOrder::Date;
//! let by_revenue = ReferralProgramSortOrder::Revenue;
//!
//! // All orders are Copy and Clone
//! let copied = by_date;
//! assert_eq!(by_date, copied);
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Sort order for referral/affiliate programs.
///
/// This enum defines the different ways to sort referral programs in Telegram.
/// It corresponds to TDLib's `ReferralProgramSortOrder` enum and the TL types
/// `AffiliateProgramSortOrder`.
///
/// # Variants
///
/// - [`Profitability`](Self::Profitability) - Sort by profitability (highest first)
/// - [`Date`](Self::Date) - Sort by creation date (newest first)
/// - [`Revenue`](Self::Revenue) - Sort by revenue (highest first)
///
/// # Example
///
/// ```rust
/// use rustgram_referral_program_sort_order::ReferralProgramSortOrder;
///
/// // Default sort order is profitability
/// let default = ReferralProgramSortOrder::default();
/// assert_eq!(default, ReferralProgramSortOrder::Profitability);
///
/// // Create specific sort orders
/// let by_date = ReferralProgramSortOrder::Date;
/// let by_revenue = ReferralProgramSortOrder::Revenue;
///
/// // Display format
/// assert_eq!(format!("{}", ReferralProgramSortOrder::Profitability), "Profitability");
/// assert_eq!(format!("{}", ReferralProgramSortOrder::Date), "Date");
/// assert_eq!(format!("{}", ReferralProgramSortOrder::Revenue), "Revenue");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ReferralProgramSortOrder {
    /// Sort by profitability.
    ///
    /// This is the default sort order, showing the most profitable programs first.
    ///
    /// Corresponds to `td_api::affiliateProgramSortOrderProfitability`.
    Profitability,

    /// Sort by creation date.
    ///
    /// Shows the most recently created programs first.
    ///
    /// Corresponds to `td_api::affiliateProgramSortOrderCreationDate`.
    Date,

    /// Sort by revenue.
    ///
    /// Shows the programs with the highest revenue first.
    ///
    /// Corresponds to `td_api::affiliateProgramSortOrderRevenue`.
    Revenue,
}

impl Default for ReferralProgramSortOrder {
    /// Returns the default sort order (Profitability).
    ///
    /// This matches TDLib's behavior where `nullptr` sort order defaults to Profitability.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_referral_program_sort_order::ReferralProgramSortOrder;
    ///
    /// let default = ReferralProgramSortOrder::default();
    /// assert_eq!(default, ReferralProgramSortOrder::Profitability);
    /// ```
    fn default() -> Self {
        Self::Profitability
    }
}

impl fmt::Display for ReferralProgramSortOrder {
    /// Formats the sort order for display.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_referral_program_sort_order::ReferralProgramSortOrder;
    ///
    /// assert_eq!(format!("{}", ReferralProgramSortOrder::Profitability), "Profitability");
    /// assert_eq!(format!("{}", ReferralProgramSortOrder::Date), "Date");
    /// assert_eq!(format!("{}", ReferralProgramSortOrder::Revenue), "Revenue");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Profitability => write!(f, "Profitability"),
            Self::Date => write!(f, "Date"),
            Self::Revenue => write!(f, "Revenue"),
        }
    }
}

impl ReferralProgramSortOrder {
    /// Returns the integer representation of this sort order.
    ///
    /// This matches TDLib's internal representation where the enum values
    /// are represented as integers (0 = Profitability, 1 = Date, 2 = Revenue).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_referral_program_sort_order::ReferralProgramSortOrder;
    ///
    /// assert_eq!(ReferralProgramSortOrder::Profitability.as_i32(), 0);
    /// assert_eq!(ReferralProgramSortOrder::Date.as_i32(), 1);
    /// assert_eq!(ReferralProgramSortOrder::Revenue.as_i32(), 2);
    /// ```
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        match self {
            Self::Profitability => 0,
            Self::Date => 1,
            Self::Revenue => 2,
        }
    }

    /// Creates a sort order from its integer representation.
    ///
    /// Returns `None` if the value is not a valid sort order (must be 0, 1, or 2).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_referral_program_sort_order::ReferralProgramSortOrder;
    ///
    /// assert_eq!(ReferralProgramSortOrder::from_i32(0), Some(ReferralProgramSortOrder::Profitability));
    /// assert_eq!(ReferralProgramSortOrder::from_i32(1), Some(ReferralProgramSortOrder::Date));
    /// assert_eq!(ReferralProgramSortOrder::from_i32(2), Some(ReferralProgramSortOrder::Revenue));
    /// assert_eq!(ReferralProgramSortOrder::from_i32(99), None);
    /// ```
    #[must_use]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Profitability),
            1 => Some(Self::Date),
            2 => Some(Self::Revenue),
            _ => None,
        }
    }

    /// Returns the name of this sort order as a string slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_referral_program_sort_order::ReferralProgramSortOrder;
    ///
    /// assert_eq!(ReferralProgramSortOrder::Profitability.as_str(), "Profitability");
    /// assert_eq!(ReferralProgramSortOrder::Date.as_str(), "Date");
    /// assert_eq!(ReferralProgramSortOrder::Revenue.as_str(), "Revenue");
    /// ```
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Profitability => "Profitability",
            Self::Date => "Date",
            Self::Revenue => "Revenue",
        }
    }

    /// Returns all possible sort orders.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_referral_program_sort_order::ReferralProgramSortOrder;
    ///
    /// let all = ReferralProgramSortOrder::all();
    /// assert_eq!(all.len(), 3);
    /// assert!(all.contains(&ReferralProgramSortOrder::Profitability));
    /// assert!(all.contains(&ReferralProgramSortOrder::Date));
    /// assert!(all.contains(&ReferralProgramSortOrder::Revenue));
    /// ```
    #[must_use]
    pub fn all() -> &'static [Self] {
        &[Self::Profitability, Self::Date, Self::Revenue]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_profitability() {
        let default = ReferralProgramSortOrder::default();
        assert_eq!(default, ReferralProgramSortOrder::Profitability);
    }

    #[test]
    fn test_equality() {
        assert_eq!(
            ReferralProgramSortOrder::Profitability,
            ReferralProgramSortOrder::Profitability
        );
        assert_eq!(
            ReferralProgramSortOrder::Date,
            ReferralProgramSortOrder::Date
        );
        assert_eq!(
            ReferralProgramSortOrder::Revenue,
            ReferralProgramSortOrder::Revenue
        );
    }

    #[test]
    fn test_inequality() {
        assert_ne!(
            ReferralProgramSortOrder::Profitability,
            ReferralProgramSortOrder::Date
        );
        assert_ne!(
            ReferralProgramSortOrder::Date,
            ReferralProgramSortOrder::Revenue
        );
        assert_ne!(
            ReferralProgramSortOrder::Revenue,
            ReferralProgramSortOrder::Profitability
        );
    }

    #[test]
    fn test_copy_semantics() {
        let original = ReferralProgramSortOrder::Date;
        let copied = original;
        assert_eq!(original, copied);
        assert_eq!(original, ReferralProgramSortOrder::Date);
    }

    #[test]
    fn test_clone_semantics() {
        let original = ReferralProgramSortOrder::Revenue;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_display_format() {
        assert_eq!(
            format!("{}", ReferralProgramSortOrder::Profitability),
            "Profitability"
        );
        assert_eq!(format!("{}", ReferralProgramSortOrder::Date), "Date");
        assert_eq!(format!("{}", ReferralProgramSortOrder::Revenue), "Revenue");
    }

    #[test]
    fn test_debug_format() {
        let debug_str = format!("{:?}", ReferralProgramSortOrder::Date);
        assert!(debug_str.contains("Date"));
    }

    #[test]
    fn test_as_i32() {
        assert_eq!(ReferralProgramSortOrder::Profitability.as_i32(), 0);
        assert_eq!(ReferralProgramSortOrder::Date.as_i32(), 1);
        assert_eq!(ReferralProgramSortOrder::Revenue.as_i32(), 2);
    }

    #[test]
    fn test_from_i32_valid() {
        assert_eq!(
            ReferralProgramSortOrder::from_i32(0),
            Some(ReferralProgramSortOrder::Profitability)
        );
        assert_eq!(
            ReferralProgramSortOrder::from_i32(1),
            Some(ReferralProgramSortOrder::Date)
        );
        assert_eq!(
            ReferralProgramSortOrder::from_i32(2),
            Some(ReferralProgramSortOrder::Revenue)
        );
    }

    #[test]
    fn test_from_i32_invalid() {
        assert_eq!(ReferralProgramSortOrder::from_i32(-1), None);
        assert_eq!(ReferralProgramSortOrder::from_i32(3), None);
        assert_eq!(ReferralProgramSortOrder::from_i32(99), None);
        assert_eq!(ReferralProgramSortOrder::from_i32(i32::MAX), None);
    }

    #[test]
    fn test_from_i32_roundtrip() {
        for order in ReferralProgramSortOrder::all() {
            let i32_value = order.as_i32();
            let restored = ReferralProgramSortOrder::from_i32(i32_value);
            assert_eq!(Some(*order), restored);
        }
    }

    #[test]
    fn test_as_str() {
        assert_eq!(
            ReferralProgramSortOrder::Profitability.as_str(),
            "Profitability"
        );
        assert_eq!(ReferralProgramSortOrder::Date.as_str(), "Date");
        assert_eq!(ReferralProgramSortOrder::Revenue.as_str(), "Revenue");
    }

    #[test]
    fn test_all() {
        let all = ReferralProgramSortOrder::all();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&ReferralProgramSortOrder::Profitability));
        assert!(all.contains(&ReferralProgramSortOrder::Date));
        assert!(all.contains(&ReferralProgramSortOrder::Revenue));
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ReferralProgramSortOrder::Profitability);
        set.insert(ReferralProgramSortOrder::Date);
        set.insert(ReferralProgramSortOrder::Revenue);

        assert_eq!(set.len(), 3);

        // Duplicate insertions don't increase size
        set.insert(ReferralProgramSortOrder::Date);
        assert_eq!(set.len(), 3);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let orders = [
            ReferralProgramSortOrder::Profitability,
            ReferralProgramSortOrder::Date,
            ReferralProgramSortOrder::Revenue,
        ];

        for order in orders {
            // JSON serialization
            let json = serde_json::to_string(&order).unwrap();
            let deserialized: ReferralProgramSortOrder = serde_json::from_str(&json).unwrap();
            assert_eq!(order, deserialized);

            // Binary serialization
            let encoded = bincode::serialize(&order).unwrap();
            let decoded: ReferralProgramSortOrder = bincode::deserialize(&encoded).unwrap();
            assert_eq!(order, decoded);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_json_representation() {
        // JSON representation uses external tag for enums
        let json = serde_json::to_string(&ReferralProgramSortOrder::Profitability).unwrap();
        assert_eq!(json, "\"Profitability\"");

        let json = serde_json::to_string(&ReferralProgramSortOrder::Date).unwrap();
        assert_eq!(json, "\"Date\"");

        let json = serde_json::to_string(&ReferralProgramSortOrder::Revenue).unwrap();
        assert_eq!(json, "\"Revenue\"");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize_invalid_json() {
        // Invalid JSON values should fail
        let result: Result<ReferralProgramSortOrder, _> = serde_json::from_str("\"Invalid\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_array_iteration() {
        let all = ReferralProgramSortOrder::all();
        let mut count = 0;

        for order in all {
            match order {
                ReferralProgramSortOrder::Profitability => count += 1,
                ReferralProgramSortOrder::Date => count += 1,
                ReferralProgramSortOrder::Revenue => count += 1,
            }
        }

        assert_eq!(count, 3);
    }

    #[test]
    fn test_match_exhaustiveness() {
        fn check_exhaustive(order: ReferralProgramSortOrder) -> bool {
            matches!(
                order,
                ReferralProgramSortOrder::Profitability
                    | ReferralProgramSortOrder::Date
                    | ReferralProgramSortOrder::Revenue
            )
        }

        assert!(check_exhaustive(ReferralProgramSortOrder::Profitability));
        assert!(check_exhaustive(ReferralProgramSortOrder::Date));
        assert!(check_exhaustive(ReferralProgramSortOrder::Revenue));
    }
}
