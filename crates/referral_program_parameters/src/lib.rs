// rustgram_referral_program_parameters
// Copyright (C) 2025 rustgram-client contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! # Referral Program Parameters
//!
//! Represents parameters for a referral program including commission and duration.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_referral_program_parameters::ReferralProgramParameters;
//!
//! let params = ReferralProgramParameters::with_params(100, 12);
//! assert!(params.is_valid());
//! assert_eq!(params.commission(), 100);
//! assert_eq!(params.month_count(), 12);
//! ```

use std::fmt;

/// Valid commission range (in permille, 1/1000)
const COMMISSION_MIN: i32 = 1;
const COMMISSION_MAX: i32 = 999;

/// Valid month count range
const MONTH_COUNT_MIN: i32 = 0;
const MONTH_COUNT_MAX: i32 = 36;

/// Marker value for invalid parameters
const INVALID_COMMISSION: i32 = -1;

/// Parameters for a referral or affiliate program.
///
/// Contains commission rate (in permille, i.e., 1/1000 or 0.1%) and duration in months.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferralProgramParameters {
    /// Commission in permille (0.1%), valid range: 1-999
    commission: i32,
    /// Duration in months, valid range: 0-36
    month_count: i32,
}

impl Default for ReferralProgramParameters {
    fn default() -> Self {
        Self::new()
    }
}

impl ReferralProgramParameters {
    /// Creates a new referral program parameters with default (zero) values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_referral_program_parameters::ReferralProgramParameters;
    ///
    /// let params = ReferralProgramParameters::new();
    /// assert_eq!(params.commission(), 0);
    /// assert_eq!(params.month_count(), 0);
    /// assert!(!params.is_valid()); // Zero commission is invalid
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            commission: 0,
            month_count: 0,
        }
    }

    /// Creates a new referral program parameters with the specified values.
    ///
    /// # Arguments
    ///
    /// * `commission_permille` - Commission rate in permille (1/1000 or 0.1%)
    /// * `duration_months` - Program duration in months
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_referral_program_parameters::ReferralProgramParameters;
    ///
    /// let params = ReferralProgramParameters::with_params(150, 12);
    /// assert!(params.is_valid());
    /// assert_eq!(params.commission(), 150);
    /// ```
    #[must_use]
    pub fn with_params(commission_permille: i32, duration_months: i32) -> Self {
        Self {
            commission: commission_permille,
            month_count: duration_months,
        }
    }

    /// Creates referral program parameters from TDLib API object.
    ///
    /// If the parsed parameters are invalid, sets commission to -1 as a marker.
    ///
    /// # Arguments
    ///
    /// * `commission_permille` - Commission from TDLib API
    /// * `month_count` - Month count from TDLib API
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_referral_program_parameters::ReferralProgramParameters;
    ///
    /// let params = ReferralProgramParameters::from_td_api(100, 12);
    /// assert!(params.is_valid());
    /// ```
    #[must_use]
    pub fn from_td_api(commission_permille: i32, month_count: i32) -> Self {
        let params = Self {
            commission: commission_permille,
            month_count,
        };

        if params.is_valid() {
            params
        } else {
            Self {
                commission: INVALID_COMMISSION,
                month_count: 0,
            }
        }
    }

    /// Returns the commission value in permille (1/1000 or 0.1%).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_referral_program_parameters::ReferralProgramParameters;
    ///
    /// let params = ReferralProgramParameters::with_params(250, 6);
    /// assert_eq!(params.commission(), 250);
    /// ```
    #[must_use]
    pub const fn commission(&self) -> i32 {
        self.commission
    }

    /// Returns the month count (program duration).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_referral_program_parameters::ReferralProgramParameters;
    ///
    /// let params = ReferralProgramParameters::with_params(100, 24);
    /// assert_eq!(params.month_count(), 24);
    /// ```
    #[must_use]
    pub const fn month_count(&self) -> i32 {
        self.month_count
    }

    /// Checks if the parameters are valid.
    ///
    /// Valid parameters have:
    /// - Commission in range [1, 999]
    /// - Month count in range [0, 36]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_referral_program_parameters::ReferralProgramParameters;
    ///
    /// let valid = ReferralProgramParameters::with_params(100, 12);
    /// assert!(valid.is_valid());
    ///
    /// let invalid_commission = ReferralProgramParameters::with_params(0, 12);
    /// assert!(!invalid_commission.is_valid());
    ///
    /// let invalid_months = ReferralProgramParameters::with_params(100, 40);
    /// assert!(!invalid_months.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        (COMMISSION_MIN..=COMMISSION_MAX).contains(&self.commission)
            && (MONTH_COUNT_MIN..=MONTH_COUNT_MAX).contains(&self.month_count)
    }
}

impl fmt::Display for ReferralProgramParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ReferralProgram[{}%", self.commission / 10)?;
        if self.month_count != 0 {
            write!(f, " X {} months]", self.month_count)?;
        } else {
            write!(f, "]")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let params = ReferralProgramParameters::default();
        assert_eq!(params.commission(), 0);
        assert_eq!(params.month_count(), 0);
        assert!(!params.is_valid());
    }

    #[test]
    fn test_new() {
        let params = ReferralProgramParameters::new();
        assert_eq!(params.commission(), 0);
        assert_eq!(params.month_count(), 0);
    }

    #[test]
    fn test_with_params() {
        let params = ReferralProgramParameters::with_params(150, 12);
        assert_eq!(params.commission(), 150);
        assert_eq!(params.month_count(), 12);
    }

    #[test]
    fn test_is_valid() {
        // Valid cases
        assert!(ReferralProgramParameters::with_params(1, 0).is_valid());
        assert!(ReferralProgramParameters::with_params(999, 36).is_valid());
        assert!(ReferralProgramParameters::with_params(500, 18).is_valid());

        // Invalid commission
        assert!(!ReferralProgramParameters::with_params(0, 12).is_valid());
        assert!(!ReferralProgramParameters::with_params(1000, 12).is_valid());
        assert!(!ReferralProgramParameters::with_params(-1, 12).is_valid());

        // Invalid month count
        assert!(!ReferralProgramParameters::with_params(100, -1).is_valid());
        assert!(!ReferralProgramParameters::with_params(100, 37).is_valid());

        // Both invalid
        assert!(!ReferralProgramParameters::with_params(0, -1).is_valid());
    }

    #[test]
    fn test_from_td_api_valid() {
        let params = ReferralProgramParameters::from_td_api(100, 12);
        assert!(params.is_valid());
        assert_eq!(params.commission(), 100);
        assert_eq!(params.month_count(), 12);
    }

    #[test]
    fn test_from_td_api_invalid() {
        // Invalid commission
        let params = ReferralProgramParameters::from_td_api(0, 12);
        assert!(!params.is_valid());
        assert_eq!(params.commission(), INVALID_COMMISSION);

        // Invalid month count
        let params2 = ReferralProgramParameters::from_td_api(100, 40);
        assert!(!params2.is_valid());
        assert_eq!(params2.commission(), INVALID_COMMISSION);
    }

    #[test]
    fn test_display_with_months() {
        let params = ReferralProgramParameters::with_params(150, 12);
        assert_eq!(format!("{}", params), "ReferralProgram[15% X 12 months]");
    }

    #[test]
    fn test_display_without_months() {
        let params = ReferralProgramParameters::with_params(200, 0);
        assert_eq!(format!("{}", params), "ReferralProgram[20%]");
    }

    #[test]
    fn test_display_invalid() {
        let params = ReferralProgramParameters::with_params(0, 0);
        assert_eq!(format!("{}", params), "ReferralProgram[0%]");
    }

    #[test]
    fn test_equality() {
        let a = ReferralProgramParameters::with_params(100, 12);
        let b = ReferralProgramParameters::with_params(100, 12);
        let c = ReferralProgramParameters::with_params(100, 6);
        let d = ReferralProgramParameters::with_params(200, 12);

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
    }

    #[test]
    fn test_commission_boundaries() {
        // Minimum valid commission
        assert!(ReferralProgramParameters::with_params(1, 0).is_valid());

        // Maximum valid commission
        assert!(ReferralProgramParameters::with_params(999, 0).is_valid());

        // Just below minimum
        assert!(!ReferralProgramParameters::with_params(0, 0).is_valid());

        // Just above maximum
        assert!(!ReferralProgramParameters::with_params(1000, 0).is_valid());
    }

    #[test]
    fn test_month_boundaries() {
        // Minimum valid month count
        assert!(ReferralProgramParameters::with_params(100, 0).is_valid());

        // Maximum valid month count
        assert!(ReferralProgramParameters::with_params(100, 36).is_valid());

        // Just below minimum
        assert!(!ReferralProgramParameters::with_params(100, -1).is_valid());

        // Just above maximum
        assert!(!ReferralProgramParameters::with_params(100, 37).is_valid());
    }

    #[test]
    fn test_cloning() {
        let params1 = ReferralProgramParameters::with_params(150, 12);
        let params2 = params1.clone();
        assert_eq!(params1, params2);
    }
}
