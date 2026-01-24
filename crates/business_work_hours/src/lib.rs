//! # Business Work Hours
//!
//! Represents business opening hours in Telegram.
//!
//! ## Overview
//!
//! This module defines the `BusinessWorkHours` struct, which specifies
//! when a business is available for contact, with support for time zones
//! and weekly recurring intervals.
//!
//! ## TDLib Correspondence
//!
//! TDLib class: `BusinessWorkHours`
//!
//! ## Examples
//!
//! ```
//! use rustgram_business_work_hours::{BusinessWorkHours, WorkHoursInterval};
//!
//! // Create work hours with a single interval
//! let interval = WorkHoursInterval::new(600, 1140); // 10:00 AM - 7:00 PM
//! let work_hours = BusinessWorkHours::with_intervals(vec![interval])
//!     .with_time_zone("America/New_York");
//!
//! // Check if open
//! assert!(!work_hours.is_empty());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use core::fmt;

/// A work hours interval within a week.
///
/// Intervals are specified in minutes from the start of the week.
/// Week starts at Monday 00:00 (minute 0) and ends at Sunday 24:00 (minute 10080).
///
/// Valid ranges:
/// - Start: 0 to 10079
/// - End: 1 to 10080
/// - End must be greater than start
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorkHoursInterval {
    /// Start minute of the interval (0-10079).
    start_minute: i32,
    /// End minute of the interval (1-10080).
    end_minute: i32,
}

impl Default for WorkHoursInterval {
    fn default() -> Self {
        Self {
            start_minute: 0,
            end_minute: 0,
        }
    }
}

impl WorkHoursInterval {
    /// Maximum minute value in a week (7 days * 24 hours * 60 minutes).
    pub const MAX_WEEK_MINUTE: i32 = 7 * 24 * 60;

    /// Creates a new WorkHoursInterval.
    ///
    /// # Arguments
    ///
    /// * `start_minute` - Start minute from week start (0-10079)
    /// * `end_minute` - End minute from week start (1-10080)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::WorkHoursInterval;
    ///
    /// // Monday 10:00 AM to 7:00 PM
    /// let interval = WorkHoursInterval::new(600, 1140);
    /// assert_eq!(interval.start_minute(), 600);
    /// assert_eq!(interval.end_minute(), 1140);
    /// ```
    #[must_use]
    pub const fn new(start_minute: i32, end_minute: i32) -> Self {
        Self {
            start_minute,
            end_minute,
        }
    }

    /// Returns the start minute of the interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::WorkHoursInterval;
    ///
    /// let interval = WorkHoursInterval::new(600, 1140);
    /// assert_eq!(interval.start_minute(), 600);
    /// ```
    #[must_use]
    pub const fn start_minute(self) -> i32 {
        self.start_minute
    }

    /// Returns the end minute of the interval.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::WorkHoursInterval;
    ///
    /// let interval = WorkHoursInterval::new(600, 1140);
    /// assert_eq!(interval.end_minute(), 1140);
    /// ```
    #[must_use]
    pub const fn end_minute(self) -> i32 {
        self.end_minute
    }

    /// Checks if this interval is valid.
    ///
    /// A valid interval has:
    /// - Start >= 0
    /// - End <= MAX_WEEK_MINUTE
    /// - Start < End
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::WorkHoursInterval;
    ///
    /// let interval = WorkHoursInterval::new(600, 1140);
    /// assert!(interval.is_valid());
    ///
    /// let invalid = WorkHoursInterval::new(1140, 600);
    /// assert!(!invalid.is_valid());
    /// ```
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.start_minute >= 0
            && self.end_minute <= Self::MAX_WEEK_MINUTE
            && self.start_minute < self.end_minute
    }

    /// Returns the duration of the interval in minutes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::WorkHoursInterval;
    ///
    /// let interval = WorkHoursInterval::new(600, 1140);
    /// assert_eq!(interval.duration(), 540); // 9 hours
    /// ```
    #[must_use]
    pub const fn duration(self) -> i32 {
        self.end_minute - self.start_minute
    }

    /// Creates a daily interval (same time every day).
    ///
    /// # Arguments
    ///
    /// * `start_hour` - Start hour (0-23)
    /// * `start_minute` - Start minute (0-59)
    /// * `end_hour` - End hour (0-23)
    /// * `end_minute` - End minute (0-59)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::WorkHoursInterval;
    ///
    /// // 9:00 AM to 5:00 PM every day
    /// let intervals = WorkHoursInterval::daily(9, 0, 17, 0);
    /// assert_eq!(intervals.len(), 7);
    /// ```
    #[must_use]
    pub fn daily(start_hour: i32, start_minute: i32, end_hour: i32, end_minute: i32) -> Vec<Self> {
        let day_minutes = 24 * 60;
        let start = start_hour * 60 + start_minute;
        let end = end_hour * 60 + end_minute;

        (0..7)
            .map(|day| Self {
                start_minute: day * day_minutes + start,
                end_minute: day * day_minutes + end,
            })
            .collect()
    }
}

impl fmt::Display for WorkHoursInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {})", self.start_minute, self.end_minute)
    }
}

/// Represents business opening hours in Telegram.
///
/// BusinessWorkHours specifies when a business is available for contact,
/// with support for time zones and weekly recurring intervals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BusinessWorkHours {
    /// List of work hour intervals.
    work_hours: Vec<WorkHoursInterval>,
    /// Time zone identifier (IANA time zone database).
    time_zone_id: String,
}

impl Default for BusinessWorkHours {
    fn default() -> Self {
        Self::new()
    }
}

impl BusinessWorkHours {
    /// Creates an empty BusinessWorkHours.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::BusinessWorkHours;
    ///
    /// let work_hours = BusinessWorkHours::new();
    /// assert!(work_hours.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            work_hours: Vec::new(),
            time_zone_id: String::new(),
        }
    }

    /// Creates BusinessWorkHours with a list of intervals.
    ///
    /// # Arguments
    ///
    /// * `work_hours` - List of work hour intervals
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::{BusinessWorkHours, WorkHoursInterval};
    ///
    /// let interval = WorkHoursInterval::new(600, 1140);
    /// let work_hours = BusinessWorkHours::with_intervals(vec![interval]);
    /// assert!(!work_hours.is_empty());
    /// ```
    #[must_use]
    pub fn with_intervals(work_hours: Vec<WorkHoursInterval>) -> Self {
        let mut result = Self {
            work_hours,
            time_zone_id: String::new(),
        };
        result.sanitize();
        result
    }

    /// Sets the list of work hour intervals.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::{BusinessWorkHours, WorkHoursInterval};
    ///
    /// let intervals = vec![WorkHoursInterval::new(600, 1140)];
    /// let work_hours = BusinessWorkHours::new().set_intervals(intervals);
    /// ```
    #[must_use]
    pub fn set_intervals(mut self, mut work_hours: Vec<WorkHoursInterval>) -> Self {
        self.work_hours = work_hours.clone();
        self.sanitize();
        self
    }

    /// Sets the time zone ID.
    ///
    /// # Arguments
    ///
    /// * `time_zone_id` - IANA time zone identifier (e.g., "America/New_York")
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::BusinessWorkHours;
    ///
    /// let work_hours = BusinessWorkHours::new()
    ///     .with_time_zone("America/New_York");
    /// assert_eq!(work_hours.time_zone_id(), "America/New_York");
    /// ```
    #[must_use]
    pub fn with_time_zone(mut self, time_zone_id: &str) -> Self {
        self.time_zone_id = time_zone_id.to_string();
        self
    }

    /// Returns the list of work hour intervals.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::{BusinessWorkHours, WorkHoursInterval};
    ///
    /// let intervals = vec![WorkHoursInterval::new(600, 1140)];
    /// let work_hours = BusinessWorkHours::with_intervals(intervals.clone());
    /// assert_eq!(work_hours.intervals(), &intervals);
    /// ```
    #[must_use]
    pub const fn intervals(&self) -> &Vec<WorkHoursInterval> {
        &self.work_hours
    }

    /// Returns the time zone ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::BusinessWorkHours;
    ///
    /// let work_hours = BusinessWorkHours::new()
    ///     .with_time_zone("UTC");
    /// assert_eq!(work_hours.time_zone_id(), "UTC");
    /// ```
    #[must_use]
    pub fn time_zone_id(&self) -> &str {
        &self.time_zone_id
    }

    /// Checks if this BusinessWorkHours is empty (no intervals).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::BusinessWorkHours;
    ///
    /// let work_hours = BusinessWorkHours::new();
    /// assert!(work_hours.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.work_hours.is_empty()
    }

    /// Returns the number of work hour intervals.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::{BusinessWorkHours, WorkHoursInterval};
    ///
    /// let work_hours = BusinessWorkHours::with_intervals(vec![
    ///     WorkHoursInterval::new(600, 1140),
    ///     WorkHoursInterval::new(1500, 1800),
    /// ]);
    /// assert_eq!(work_hours.interval_count(), 2);
    /// ```
    #[must_use]
    pub fn interval_count(&self) -> usize {
        self.work_hours.len()
    }

    /// Sanitizes the work hours by removing invalid intervals.
    ///
    /// This is called automatically when setting intervals.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_business_work_hours::{BusinessWorkHours, WorkHoursInterval};
    ///
    /// let mut work_hours = BusinessWorkHours::new();
    /// work_hours.sanitize();
    /// assert!(work_hours.is_empty());
    /// ```
    pub fn sanitize(&mut self) {
        self.work_hours.retain(|interval| interval.is_valid());
    }
}

impl fmt::Display for BusinessWorkHours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BusinessWorkHours[")?;
        for (i, interval) in self.work_hours.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", interval)?;
        }
        if !self.time_zone_id.is_empty() {
            write!(f, " in {}]", self.time_zone_id)?;
        } else {
            write!(f, "]")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // WorkHoursInterval tests
    mod interval_tests {
        use super::*;

        #[test]
        fn test_interval_new() {
            let interval = WorkHoursInterval::new(600, 1140);
            assert_eq!(interval.start_minute(), 600);
            assert_eq!(interval.end_minute(), 1140);
        }

        #[test]
        fn test_interval_is_valid() {
            let valid = WorkHoursInterval::new(600, 1140);
            assert!(valid.is_valid());

            let invalid = WorkHoursInterval::new(1140, 600);
            assert!(!invalid.is_valid());

            let out_of_range = WorkHoursInterval::new(-1, 100);
            assert!(!out_of_range.is_valid());
        }

        #[test]
        fn test_interval_duration() {
            let interval = WorkHoursInterval::new(600, 1140);
            assert_eq!(interval.duration(), 540);
        }

        #[test]
        fn test_interval_display() {
            let interval = WorkHoursInterval::new(600, 1140);
            assert_eq!(format!("{}", interval), "[600, 1140)");
        }

        #[test]
        fn test_interval_daily() {
            let intervals = WorkHoursInterval::daily(9, 0, 17, 0);
            assert_eq!(intervals.len(), 7);
            for (i, interval) in intervals.iter().enumerate() {
                let i = i as i32;
                assert_eq!(interval.start_minute(), i * 1440 + 540);
                assert_eq!(interval.end_minute(), i * 1440 + 1020);
            }
        }

        #[test]
        fn test_interval_default() {
            let interval = WorkHoursInterval::default();
            assert_eq!(interval.start_minute(), 0);
            assert_eq!(interval.end_minute(), 0);
        }

        #[test]
        fn test_interval_copy() {
            let interval = WorkHoursInterval::new(600, 1140);
            let copy = interval;
            assert_eq!(interval, copy);
        }

        #[test]
        fn test_interval_clone() {
            let interval = WorkHoursInterval::new(600, 1140);
            let cloned = interval.clone();
            assert_eq!(interval, cloned);
        }

        #[test]
        fn test_interval_partial_eq() {
            let i1 = WorkHoursInterval::new(600, 1140);
            let i2 = WorkHoursInterval::new(600, 1140);
            assert_eq!(i1, i2);

            let i3 = WorkHoursInterval::new(700, 1200);
            assert_ne!(i1, i3);
        }

        #[test]
        fn test_interval_partial_ord() {
            let i1 = WorkHoursInterval::new(600, 1140);
            let i2 = WorkHoursInterval::new(700, 1200);
            assert!(i1 < i2);
        }

        #[test]
        fn test_interval_hash() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            set.insert(WorkHoursInterval::new(600, 1140));
            set.insert(WorkHoursInterval::new(600, 1140));
            assert_eq!(set.len(), 1);
        }

        #[test]
        fn test_interval_max_week_minute() {
            assert_eq!(WorkHoursInterval::MAX_WEEK_MINUTE, 10080);
        }
    }

    // BusinessWorkHours tests
    mod work_hours_tests {
        use super::*;

        #[test]
        fn test_new() {
            let work_hours = BusinessWorkHours::new();
            assert!(work_hours.is_empty());
            assert_eq!(work_hours.time_zone_id(), "");
        }

        #[test]
        fn test_with_intervals() {
            let intervals = vec![WorkHoursInterval::new(600, 1140)];
            let work_hours = BusinessWorkHours::with_intervals(intervals.clone());
            assert_eq!(work_hours.intervals(), &intervals);
        }

        #[test]
        fn test_with_time_zone() {
            let work_hours = BusinessWorkHours::new().with_time_zone("UTC");
            assert_eq!(work_hours.time_zone_id(), "UTC");
        }

        #[test]
        fn test_set_intervals() {
            let intervals = vec![WorkHoursInterval::new(600, 1140)];
            let work_hours = BusinessWorkHours::new().set_intervals(intervals.clone());
            assert_eq!(work_hours.intervals(), &intervals);
        }

        #[test]
        fn test_is_empty() {
            let work_hours = BusinessWorkHours::new();
            assert!(work_hours.is_empty());

            let work_hours =
                BusinessWorkHours::with_intervals(vec![WorkHoursInterval::new(600, 1140)]);
            assert!(!work_hours.is_empty());
        }

        #[test]
        fn test_interval_count() {
            let work_hours = BusinessWorkHours::with_intervals(vec![
                WorkHoursInterval::new(600, 1140),
                WorkHoursInterval::new(1500, 1800),
            ]);
            assert_eq!(work_hours.interval_count(), 2);
        }

        #[test]
        fn test_sanitize() {
            let mut work_hours = BusinessWorkHours::new();
            work_hours.work_hours = vec![
                WorkHoursInterval::new(600, 1140),
                WorkHoursInterval::new(1140, 600), // Invalid
            ];
            work_hours.sanitize();
            assert_eq!(work_hours.interval_count(), 1);
        }

        #[test]
        fn test_sanitize_auto() {
            // Sanitize is called automatically in set_intervals
            let work_hours = BusinessWorkHours::new().set_intervals(vec![
                WorkHoursInterval::new(600, 1140),
                WorkHoursInterval::new(1140, 600), // Invalid - should be removed
            ]);
            assert_eq!(work_hours.interval_count(), 1);
        }

        #[test]
        fn test_display() {
            let work_hours = BusinessWorkHours::new()
                .set_intervals(vec![WorkHoursInterval::new(600, 1140)])
                .with_time_zone("UTC");
            let display = format!("{}", work_hours);
            assert!(display.contains("BusinessWorkHours"));
            assert!(display.contains("[600, 1140)"));
            assert!(display.contains("UTC"));
        }

        #[test]
        fn test_display_empty() {
            let work_hours = BusinessWorkHours::new();
            let display = format!("{}", work_hours);
            assert!(display.contains("BusinessWorkHours"));
            assert!(display.contains("[]"));
        }

        #[test]
        fn test_default() {
            let work_hours = BusinessWorkHours::default();
            assert!(work_hours.is_empty());
        }

        #[test]
        fn test_clone() {
            let work_hours = BusinessWorkHours::new().with_time_zone("UTC");
            let cloned = work_hours.clone();
            assert_eq!(work_hours, cloned);
        }

        #[test]
        fn test_partial_eq() {
            let w1 = BusinessWorkHours::new().with_time_zone("UTC");
            let w2 = BusinessWorkHours::new().with_time_zone("UTC");
            assert_eq!(w1, w2);

            let w3 = BusinessWorkHours::new().with_time_zone("EST");
            assert_ne!(w1, w3);
        }

        #[test]
        fn test_debug() {
            let work_hours = BusinessWorkHours::new();
            let debug = format!("{:?}", work_hours);
            assert!(debug.contains("BusinessWorkHours"));
        }

        #[test]
        fn test_send_sync() {
            fn assert_send<T: Send>() {}
            fn assert_sync<T: Sync>() {}
            assert_send::<BusinessWorkHours>();
            assert_sync::<BusinessWorkHours>();
        }

        #[test]
        fn test_multiple_intervals() {
            let work_hours = BusinessWorkHours::with_intervals(vec![
                WorkHoursInterval::new(600, 1140),
                WorkHoursInterval::new(1500, 1800),
                WorkHoursInterval::new(2000, 2100),
            ]);
            assert_eq!(work_hours.interval_count(), 3);
        }

        #[test]
        fn test_with_intervals_removes_invalid() {
            let work_hours = BusinessWorkHours::with_intervals(vec![
                WorkHoursInterval::new(600, 1140),
                WorkHoursInterval::new(1140, 600), // Invalid
                WorkHoursInterval::new(-1, 100),   // Invalid
            ]);
            assert_eq!(work_hours.interval_count(), 1);
        }

        #[test]
        fn test_time_zone_id_empty() {
            let work_hours = BusinessWorkHours::new();
            assert_eq!(work_hours.time_zone_id(), "");
        }
    }
}
