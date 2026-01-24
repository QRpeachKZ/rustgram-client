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

//! # Referral Program Info
//!
//! Information about a referral or affiliate program.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_referral_program_info::ReferralProgramInfo;
//! use rustgram_referral_program_parameters::ReferralProgramParameters;
//! use rustgram_star_amount::StarAmount;
//!
//! let params = ReferralProgramParameters::with_params(100, 12);
//! let daily_amount = StarAmount::from_parts(50, 0).expect("valid star amount");
//! let info = ReferralProgramInfo::with_params(params, 0, daily_amount);
//! assert!(info.is_valid());
//! assert!(info.is_active());
//! ```

use rustgram_referral_program_parameters::ReferralProgramParameters;
use rustgram_star_amount::StarAmount;
use std::fmt;

/// Information about a referral or affiliate program.
///
/// Contains the program parameters, end date, and daily star amount.
///
/// # Example
///
/// ```rust
/// use rustgram_referral_program_info::ReferralProgramInfo;
/// use rustgram_referral_program_parameters::ReferralProgramParameters;
/// use rustgram_star_amount::StarAmount;
///
/// let params = ReferralProgramParameters::with_params(150, 6);
/// let daily_amount = StarAmount::from_parts(100, 0).expect("valid star amount");
/// let info = ReferralProgramInfo::with_params(params, 1234567890, daily_amount);
/// assert!(info.is_valid());
/// assert!(!info.is_active());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ReferralProgramInfo {
    /// Program parameters (commission, duration)
    parameters: ReferralProgramParameters,
    /// End date as Unix timestamp (0 = active/ongoing)
    end_date: i32,
    /// Daily star amount for the program
    daily_star_amount: StarAmount,
}

impl ReferralProgramInfo {
    /// Creates a new referral program info.
    ///
    /// # Arguments
    ///
    /// * `parameters` - Program parameters (commission, duration)
    /// * `end_date` - End date as Unix timestamp (0 = active/ongoing)
    /// * `daily_star_amount` - Daily star amount
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_referral_program_info::ReferralProgramInfo;
    /// use rustgram_referral_program_parameters::ReferralProgramParameters;
    /// use rustgram_star_amount::StarAmount;
    ///
    /// let params = ReferralProgramParameters::with_params(100, 12);
    /// let daily = StarAmount::from_parts(50, 0).expect("valid star amount");
    /// let info = ReferralProgramInfo::with_params(params, 0, daily);
    /// ```
    pub fn with_params(
        parameters: ReferralProgramParameters,
        end_date: i32,
        daily_star_amount: StarAmount,
    ) -> Self {
        Self {
            parameters,
            end_date,
            daily_star_amount,
        }
    }

    /// Creates referral program info from a mock telegram_api::starRefProgram object.
    ///
    /// This is a simplified version for testing. The real implementation would
    /// parse the actual MTProto object.
    ///
    /// # Arguments
    ///
    /// * `commission_permille` - Commission rate in permille (1/1000)
    /// * `month_count` - Program duration in months
    /// * `end_date` - End date as Unix timestamp
    /// * `daily_stars` - Daily star amount
    /// * `daily_nanos` - Daily nanostar amount
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_referral_program_info::ReferralProgramInfo;
    ///
    /// let info = ReferralProgramInfo::from_telegram_api(100, 12, 0, 50, 0);
    /// assert!(info.is_valid());
    /// ```
    pub fn from_telegram_api(
        commission_permille: i32,
        month_count: i32,
        end_date: i32,
        daily_stars: i64,
        daily_nanos: i32,
    ) -> Self {
        let parameters = ReferralProgramParameters::from_td_api(commission_permille, month_count);
        let daily_star_amount =
            StarAmount::from_parts(daily_stars, daily_nanos).unwrap_or_default();

        Self {
            parameters,
            end_date,
            daily_star_amount,
        }
    }

    /// Returns the program parameters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_referral_program_info::ReferralProgramInfo;
    /// use rustgram_referral_program_parameters::ReferralProgramParameters;
    /// use rustgram_star_amount::StarAmount;
    ///
    /// let params = ReferralProgramParameters::with_params(100, 12);
    /// let info = ReferralProgramInfo::with_params(params, 0, StarAmount::default());
    /// assert_eq!(info.parameters().commission(), 100);
    /// ```
    pub fn parameters(&self) -> &ReferralProgramParameters {
        &self.parameters
    }

    /// Returns the end date as Unix timestamp.
    ///
    /// Returns 0 if the program is active/ongoing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_referral_program_info::ReferralProgramInfo;
    ///
    /// let info = ReferralProgramInfo::from_telegram_api(100, 12, 1234567890, 50, 0);
    /// assert_eq!(info.end_date(), 1234567890);
    /// ```
    pub fn end_date(&self) -> i32 {
        self.end_date
    }

    /// Returns the daily star amount.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_referral_program_info::ReferralProgramInfo;
    /// use rustgram_referral_program_parameters::ReferralProgramParameters;
    /// use rustgram_star_amount::StarAmount;
    ///
    /// let daily = StarAmount::from_parts(100, 500_000_000).expect("valid star amount");
    /// let info = ReferralProgramInfo::with_params(
    ///     ReferralProgramParameters::default(),
    ///     0,
    ///     daily.clone()
    /// );
    /// assert_eq!(info.daily_star_amount().star_count(), 100);
    /// ```
    pub fn daily_star_amount(&self) -> &StarAmount {
        &self.daily_star_amount
    }

    /// Checks if the referral program info is valid.
    ///
    /// Valid info has:
    /// - Valid parameters (commission in range, duration in range)
    /// - Non-negative end date
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_referral_program_info::ReferralProgramInfo;
    /// use rustgram_referral_program_parameters::ReferralProgramParameters;
    /// use rustgram_star_amount::StarAmount;
    ///
    /// let params = ReferralProgramParameters::with_params(100, 12);
    /// let info = ReferralProgramInfo::with_params(params, 0, StarAmount::default());
    /// assert!(info.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.parameters.is_valid() && self.end_date >= 0
    }

    /// Checks if the referral program is currently active.
    ///
    /// A program is active if end_date is 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_referral_program_info::ReferralProgramInfo;
    ///
    /// // Active program
    /// let active = ReferralProgramInfo::from_telegram_api(100, 12, 0, 50, 0);
    /// assert!(active.is_active());
    ///
    /// // Ended program
    /// let ended = ReferralProgramInfo::from_telegram_api(100, 12, 1234567890, 50, 0);
    /// assert!(!ended.is_active());
    /// ```
    pub fn is_active(&self) -> bool {
        self.end_date == 0
    }

    /// Checks if the program has ended.
    ///
    /// A program has ended if end_date is not 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_referral_program_info::ReferralProgramInfo;
    ///
    /// let active = ReferralProgramInfo::from_telegram_api(100, 12, 0, 50, 0);
    /// assert!(!active.has_ended());
    ///
    /// let ended = ReferralProgramInfo::from_telegram_api(100, 12, 1234567890, 50, 0);
    /// assert!(ended.has_ended());
    /// ```
    pub fn has_ended(&self) -> bool {
        self.end_date != 0
    }
}

impl fmt::Display for ReferralProgramInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ReferralProgramInfo[")?;
        write!(f, "parameters={}, ", self.parameters)?;
        write!(f, "daily_stars={}, ", self.daily_star_amount.as_string())?;

        if self.end_date == 0 {
            write!(f, "active]")
        } else {
            write!(f, "ends_at={}]", self.end_date)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_params() {
        let params = ReferralProgramParameters::with_params(100, 12);
        let daily = StarAmount::from_parts(50, 0).unwrap();
        let info = ReferralProgramInfo::with_params(params, 0, daily);

        assert_eq!(info.parameters().commission(), 100);
        assert_eq!(info.parameters().month_count(), 12);
        assert_eq!(info.end_date(), 0);
        assert_eq!(info.daily_star_amount().star_count(), 50);
    }

    #[test]
    fn test_from_telegram_api() {
        let info = ReferralProgramInfo::from_telegram_api(150, 6, 1234567890, 100, 0);

        assert_eq!(info.parameters().commission(), 150);
        assert_eq!(info.parameters().month_count(), 6);
        assert_eq!(info.end_date(), 1234567890);
        assert_eq!(info.daily_star_amount().star_count(), 100);
    }

    #[test]
    fn test_from_telegram_api_with_nanos() {
        let info = ReferralProgramInfo::from_telegram_api(100, 12, 0, 50, 500_000_000);

        assert_eq!(info.daily_star_amount().star_count(), 50);
        assert_eq!(info.daily_star_amount().nanostar_count(), 500_000_000);
    }

    #[test]
    fn test_is_valid() {
        // Valid info
        let params = ReferralProgramParameters::with_params(100, 12);
        let info = ReferralProgramInfo::with_params(params, 0, StarAmount::default());
        assert!(info.is_valid());

        // Valid with end date
        let params = ReferralProgramParameters::with_params(100, 12);
        let info = ReferralProgramInfo::with_params(params, 1234567890, StarAmount::default());
        assert!(info.is_valid());
    }

    #[test]
    fn test_is_valid_invalid_parameters() {
        // Invalid parameters
        let params = ReferralProgramParameters::with_params(0, 12);
        let info = ReferralProgramInfo::with_params(params, 0, StarAmount::default());
        assert!(!info.is_valid());
    }

    #[test]
    fn test_is_valid_negative_end_date() {
        let params = ReferralProgramParameters::with_params(100, 12);
        let info = ReferralProgramInfo::with_params(params, -1, StarAmount::default());
        assert!(!info.is_valid());
    }

    #[test]
    fn test_is_active() {
        // Active program
        let info = ReferralProgramInfo::from_telegram_api(100, 12, 0, 50, 0);
        assert!(info.is_active());
        assert!(!info.has_ended());

        // Ended program
        let info = ReferralProgramInfo::from_telegram_api(100, 12, 1234567890, 50, 0);
        assert!(!info.is_active());
        assert!(info.has_ended());
    }

    #[test]
    fn test_default() {
        let info = ReferralProgramInfo::default();
        assert!(!info.is_valid()); // Default params are invalid
        assert!(info.is_active()); // end_date is 0
    }

    #[test]
    fn test_equality() {
        let params = ReferralProgramParameters::with_params(100, 12);
        let daily = StarAmount::from_parts(50, 0).unwrap();

        let info1 = ReferralProgramInfo::with_params(params.clone(), 0, daily.clone());
        let info2 = ReferralProgramInfo::with_params(params, 0, daily);
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_inequality() {
        let params = ReferralProgramParameters::with_params(100, 12);
        let daily = StarAmount::from_parts(50, 0).unwrap();

        let info1 = ReferralProgramInfo::with_params(params.clone(), 0, daily.clone());
        let info2 = ReferralProgramInfo::with_params(params, 1234567890, daily);
        assert_ne!(info1, info2);
    }

    #[test]
    fn test_clone() {
        let params = ReferralProgramParameters::with_params(100, 12);
        let daily = StarAmount::from_parts(50, 0).unwrap();
        let info1 = ReferralProgramInfo::with_params(params, 0, daily);
        let info2 = info1.clone();
        assert_eq!(info1, info2);
    }

    #[test]
    fn test_display_active() {
        let info = ReferralProgramInfo::from_telegram_api(100, 12, 0, 50, 0);
        let display = format!("{}", info);
        assert!(display.contains("active"));
    }

    #[test]
    fn test_display_ended() {
        let info = ReferralProgramInfo::from_telegram_api(100, 12, 1234567890, 50, 0);
        let display = format!("{}", info);
        assert!(display.contains("1234567890"));
    }

    #[test]
    fn test_from_telegram_api_invalid_params() {
        let info = ReferralProgramInfo::from_telegram_api(0, 12, 0, 50, 0);
        assert!(!info.is_valid());
        assert_eq!(info.parameters().commission(), -1); // Marker for invalid
    }

    #[test]
    fn test_from_telegram_api_invalid_nanos() {
        // Invalid nanos should result in default StarAmount
        let info = ReferralProgramInfo::from_telegram_api(100, 12, 0, 50, -1);
        assert_eq!(info.daily_star_amount().star_count(), 0);
        assert_eq!(info.daily_star_amount().nanostar_count(), 0);
    }

    #[test]
    fn test_multiple_programs() {
        let programs = [
            ReferralProgramInfo::from_telegram_api(100, 12, 0, 50, 0),
            ReferralProgramInfo::from_telegram_api(200, 6, 1234567890, 100, 0),
            ReferralProgramInfo::from_telegram_api(150, 24, 987654321, 75, 500_000_000),
        ];

        assert!(programs[0].is_active());
        assert!(!programs[1].is_active());
        assert!(!programs[2].is_active());
    }

    #[test]
    fn test_commission_display() {
        let params = ReferralProgramParameters::with_params(150, 12);
        let info = ReferralProgramInfo::with_params(params, 0, StarAmount::default());
        assert_eq!(info.parameters().commission(), 150);
    }

    #[test]
    fn test_daily_amount_with_fraction() {
        let params = ReferralProgramParameters::with_params(100, 12);
        let daily = StarAmount::from_parts(100, 500_000_000).unwrap();
        let info = ReferralProgramInfo::with_params(params, 0, daily);

        assert_eq!(info.daily_star_amount().star_count(), 100);
        assert_eq!(info.daily_star_amount().nanostar_count(), 500_000_000);
        assert_eq!(info.daily_star_amount().as_string(), "100.5");
    }

    #[test]
    fn test_zero_end_date_means_active() {
        let info = ReferralProgramInfo::from_telegram_api(100, 12, 0, 50, 0);
        assert_eq!(info.end_date(), 0);
        assert!(info.is_active());
    }

    #[test]
    fn test_future_end_date() {
        let future_date = 2147483647; // Max i32
        let info = ReferralProgramInfo::from_telegram_api(100, 12, future_date, 50, 0);
        assert_eq!(info.end_date(), future_date);
        assert!(!info.is_active());
        assert!(info.has_ended());
    }

    #[test]
    fn test_max_commission() {
        // Test maximum valid commission (999 permille = 99.9%)
        let params = ReferralProgramParameters::with_params(999, 12);
        let info = ReferralProgramInfo::with_params(params, 0, StarAmount::default());
        assert!(info.is_valid());
        assert_eq!(info.parameters().commission(), 999);
    }

    #[test]
    fn test_max_month_count() {
        // Test maximum valid month count (36 months = 3 years)
        let params = ReferralProgramParameters::with_params(100, 36);
        let info = ReferralProgramInfo::with_params(params, 0, StarAmount::default());
        assert!(info.is_valid());
        assert_eq!(info.parameters().month_count(), 36);
    }

    #[test]
    fn test_zero_daily_stars() {
        let params = ReferralProgramParameters::with_params(100, 12);
        let daily = StarAmount::default();
        let info = ReferralProgramInfo::with_params(params, 0, daily);
        assert_eq!(info.daily_star_amount().star_count(), 0);
        assert!(info.is_valid());
    }

    #[test]
    fn test_large_daily_stars() {
        let params = ReferralProgramParameters::with_params(100, 12);
        let daily = StarAmount::from_parts(1_000_000_000, 0).expect("valid");
        let info = ReferralProgramInfo::with_params(params, 0, daily);
        assert_eq!(info.daily_star_amount().star_count(), 1_000_000_000);
    }

    #[test]
    fn test_program_duration() {
        let params = ReferralProgramParameters::with_params(100, 6);
        let info = ReferralProgramInfo::with_params(params, 0, StarAmount::default());
        assert_eq!(info.parameters().month_count(), 6);
    }
}
