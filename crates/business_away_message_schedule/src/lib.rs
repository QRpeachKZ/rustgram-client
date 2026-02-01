//! # Rustgram BusinessAwayMessageSchedule
//!
//! Business away message schedule handling for Telegram MTProto client.
//!
//! This crate provides types for defining when automatic away messages
//! should be sent for business accounts.
//!
//! ## Overview
//!
//! - [`BusinessAwayMessageSchedule`] - Schedule for away messages
//!
//! ## Examples
//!
//! ```
//! use rustgram_business_away_message_schedule::BusinessAwayMessageSchedule;
//!
//! let always = BusinessAwayMessageSchedule::Always;
//! assert!(always.is_active());
//!
//! let custom = BusinessAwayMessageSchedule::Custom { start_date: 1000, end_date: 2000 };
//! // Check if currently active based on timestamps
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

/// Away message schedule type.
///
/// Defines when automatic away messages should be sent.
///
/// # Examples
///
/// ```
/// use rustgram_business_away_message_schedule::BusinessAwayMessageSchedule;
///
/// let always = BusinessAwayMessageSchedule::Always;
/// assert!(matches!(always, BusinessAwayMessageSchedule::Always));
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BusinessAwayMessageSchedule {
    /// Away message is always active
    Always,
    /// Away message active outside work hours
    OutsideWorkHours,
    /// Away message active for a custom time period
    Custom {
        /// Start timestamp (Unix time)
        start_date: i32,
        /// End timestamp (Unix time)
        end_date: i32,
    },
}

impl Default for BusinessAwayMessageSchedule {
    fn default() -> Self {
        Self::Always
    }
}

impl BusinessAwayMessageSchedule {
    /// Creates an "always" schedule.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_away_message_schedule::BusinessAwayMessageSchedule;
    ///
    /// let schedule = BusinessAwayMessageSchedule::always();
    /// assert!(matches!(schedule, BusinessAwayMessageSchedule::Always));
    /// ```
    #[inline]
    #[must_use]
    pub fn always() -> Self {
        Self::Always
    }

    /// Creates an "outside work hours" schedule.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_away_message_schedule::BusinessAwayMessageSchedule;
    ///
    /// let schedule = BusinessAwayMessageSchedule::outside_work_hours();
    /// assert!(matches!(schedule, BusinessAwayMessageSchedule::OutsideWorkHours));
    /// ```
    #[inline]
    #[must_use]
    pub fn outside_work_hours() -> Self {
        Self::OutsideWorkHours
    }

    /// Creates a custom time period schedule.
    ///
    /// # Arguments
    ///
    /// * `start_date` - Start timestamp (Unix time)
    /// * `end_date` - End timestamp (Unix time)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_away_message_schedule::BusinessAwayMessageSchedule;
    ///
    /// let schedule = BusinessAwayMessageSchedule::custom(1000, 2000);
    /// assert!(matches!(schedule, BusinessAwayMessageSchedule::Custom { .. }));
    /// ```
    #[inline]
    #[must_use]
    pub const fn custom(start_date: i32, end_date: i32) -> Self {
        Self::Custom {
            start_date,
            end_date,
        }
    }

    /// Checks if the schedule is currently active.
    ///
    /// # Returns
    ///
    /// - `Always`: always returns `true`
    /// - `OutsideWorkHours`: returns `true` if outside typical business hours
    /// - `Custom`: returns `true` if current time is within the range
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_away_message_schedule::BusinessAwayMessageSchedule;
    ///
    /// assert!(BusinessAwayMessageSchedule::Always.is_active());
    /// ```
    #[must_use]
    pub fn is_active(&self) -> bool {
        match self {
            Self::Always => true,
            Self::OutsideWorkHours => Self::is_outside_work_hours(),
            Self::Custom {
                start_date,
                end_date,
            } => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs() as i32)
                    .unwrap_or(0);
                *start_date <= now && now <= *end_date
            }
        }
    }

    /// Returns the start date for Custom schedules.
    ///
    /// # Returns
    ///
    /// `Some(start_date)` for Custom schedules, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_away_message_schedule::BusinessAwayMessageSchedule;
    ///
    /// let schedule = BusinessAwayMessageSchedule::custom(1000, 2000);
    /// assert_eq!(schedule.start_date(), Some(1000));
    ///
    /// let always = BusinessAwayMessageSchedule::Always;
    /// assert_eq!(always.start_date(), None);
    /// ```
    #[inline]
    #[must_use]
    pub const fn start_date(&self) -> Option<i32> {
        match self {
            Self::Custom { start_date, .. } => Some(*start_date),
            _ => None,
        }
    }

    /// Returns the end date for Custom schedules.
    ///
    /// # Returns
    ///
    /// `Some(end_date)` for Custom schedules, `None` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_away_message_schedule::BusinessAwayMessageSchedule;
    ///
    /// let schedule = BusinessAwayMessageSchedule::custom(1000, 2000);
    /// assert_eq!(schedule.end_date(), Some(2000));
    /// ```
    #[inline]
    #[must_use]
    pub const fn end_date(&self) -> Option<i32> {
        match self {
            Self::Custom { end_date, .. } => Some(*end_date),
            _ => None,
        }
    }

    /// Checks if currently outside work hours.
    ///
    /// This is a simplified check that considers:
    /// - Weekends (Saturday/Sunday) as outside work hours
    /// - Hours outside 9 AM - 5 PM UTC on weekdays
    #[must_use]
    fn is_outside_work_hours() -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // Simple check: outside 9-17 UTC on weekdays
        let seconds_in_day = now % 86400;
        let day_of_week = (now / 86400) % 7;

        // Weekend (Saturday=5, Sunday=6)
        if day_of_week >= 5 {
            return true;
        }

        // Outside working hours: before 9 AM (32400 seconds) or after 5 PM (61200 seconds)
        !(32400..=61200).contains(&seconds_in_day)
    }
}

impl fmt::Display for BusinessAwayMessageSchedule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Always => write!(f, "Always"),
            Self::OutsideWorkHours => write!(f, "OutsideWorkHours"),
            Self::Custom {
                start_date,
                end_date,
            } => write!(f, "Custom({}..{})", start_date, end_date),
        }
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-business-away-message-schedule";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Constructor Tests ==========

    #[test]
    fn test_always() {
        let schedule = BusinessAwayMessageSchedule::always();
        assert!(matches!(schedule, BusinessAwayMessageSchedule::Always));
    }

    #[test]
    fn test_outside_work_hours() {
        let schedule = BusinessAwayMessageSchedule::outside_work_hours();
        assert!(matches!(
            schedule,
            BusinessAwayMessageSchedule::OutsideWorkHours
        ));
    }

    #[test]
    fn test_custom() {
        let schedule = BusinessAwayMessageSchedule::custom(1000, 2000);
        assert!(matches!(
            schedule,
            BusinessAwayMessageSchedule::Custom { .. }
        ));
        assert_eq!(schedule.start_date(), Some(1000));
        assert_eq!(schedule.end_date(), Some(2000));
    }

    #[test]
    fn test_default() {
        let schedule = BusinessAwayMessageSchedule::default();
        assert!(matches!(schedule, BusinessAwayMessageSchedule::Always));
    }

    // ========== is_active Tests ==========

    #[test]
    fn test_is_active_always() {
        assert!(BusinessAwayMessageSchedule::Always.is_active());
    }

    #[test]
    fn test_is_active_outside_work_hours() {
        let result = BusinessAwayMessageSchedule::OutsideWorkHours.is_active();
        // Result depends on current time, just ensure it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_is_active_custom_within_range() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        let schedule = BusinessAwayMessageSchedule::Custom {
            start_date: now - 100,
            end_date: now + 100,
        };
        assert!(schedule.is_active());
    }

    #[test]
    fn test_is_active_custom_before_range() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        let schedule = BusinessAwayMessageSchedule::Custom {
            start_date: now + 100,
            end_date: now + 200,
        };
        assert!(!schedule.is_active());
    }

    #[test]
    fn test_is_active_custom_after_range() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        let schedule = BusinessAwayMessageSchedule::Custom {
            start_date: now - 200,
            end_date: now - 100,
        };
        assert!(!schedule.is_active());
    }

    #[test]
    fn test_is_active_custom_exactly_at_boundaries() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);

        let schedule = BusinessAwayMessageSchedule::Custom {
            start_date: now,
            end_date: now,
        };
        assert!(schedule.is_active());
    }

    // ========== Accessor Tests ==========

    #[test]
    fn test_start_date_always() {
        assert_eq!(BusinessAwayMessageSchedule::Always.start_date(), None);
    }

    #[test]
    fn test_start_date_outside_work_hours() {
        assert_eq!(
            BusinessAwayMessageSchedule::OutsideWorkHours.start_date(),
            None
        );
    }

    #[test]
    fn test_start_date_custom() {
        let schedule = BusinessAwayMessageSchedule::Custom {
            start_date: 12345,
            end_date: 67890,
        };
        assert_eq!(schedule.start_date(), Some(12345));
    }

    #[test]
    fn test_end_date_custom() {
        let schedule = BusinessAwayMessageSchedule::Custom {
            start_date: 12345,
            end_date: 67890,
        };
        assert_eq!(schedule.end_date(), Some(67890));
    }

    // ========== Equality Tests ==========

    #[test]
    fn test_equality_same_type() {
        assert_eq!(
            BusinessAwayMessageSchedule::Always,
            BusinessAwayMessageSchedule::Always
        );
    }

    #[test]
    fn test_equality_different_type() {
        assert_ne!(
            BusinessAwayMessageSchedule::Always,
            BusinessAwayMessageSchedule::OutsideWorkHours
        );
    }

    #[test]
    fn test_equality_custom_same_values() {
        let s1 = BusinessAwayMessageSchedule::custom(100, 200);
        let s2 = BusinessAwayMessageSchedule::custom(100, 200);
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_inequality_custom_different_values() {
        let s1 = BusinessAwayMessageSchedule::custom(100, 200);
        let s2 = BusinessAwayMessageSchedule::custom(100, 300);
        assert_ne!(s1, s2);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_always() {
        assert_eq!(format!("{}", BusinessAwayMessageSchedule::Always), "Always");
    }

    #[test]
    fn test_display_outside_work_hours() {
        assert_eq!(
            format!("{}", BusinessAwayMessageSchedule::OutsideWorkHours),
            "OutsideWorkHours"
        );
    }

    #[test]
    fn test_display_custom() {
        let schedule = BusinessAwayMessageSchedule::custom(1000, 2000);
        let s = format!("{}", schedule);
        assert!(s.contains("1000"));
        assert!(s.contains("2000"));
    }

    // ========== Clone Tests ==========

    #[test]
    fn test_clone() {
        let s1 = BusinessAwayMessageSchedule::custom(100, 200);
        let s2 = s1.clone();
        assert_eq!(s1, s2);
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-business-away-message-schedule");
    }
}
