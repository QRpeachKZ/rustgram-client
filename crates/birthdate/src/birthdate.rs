//! Core Birthdate type with bit-packed storage.
//!
//! This module implements the `Birthdate` type which stores date information
//! in a compressed i32 format using bit packing:
//! - Bits 0-4: day (5 bits, values 1-31)
//! - Bits 5-8: month (4 bits, values 1-12)
//! - Bits 9-31: year (23 bits, values 0-3000, where 0 means unknown)

use crate::error::{BirthdateError, Result};
use crate::tl::TelegramApiBirthday;
use std::fmt;

/// Magic constant for Telegram API birthday constructor.
///
/// From TL schema: `birthday#6c8e1e06 flags:# day:int month:int year:flags.0?int = Birthday`
pub const BIRTHDAY_MAGIC: u32 = 0x6c8e1e06;

/// Bit mask for the year flag in Telegram API.
///
/// The year is present only when flag bit 0 is set.
pub const YEAR_FLAG_MASK: u32 = 0x1;

/// Bit mask for extracting day from packed storage (5 bits, values 0-31).
const DAY_MASK: i32 = 0x1f;

/// Bit shift for extracting month from packed storage (4 bits after 5 bit day).
const MONTH_SHIFT: i32 = 5;

/// Bit mask for extracting month (4 bits, values 0-15).
const MONTH_MASK: i32 = 0xf;

/// Bit shift for extracting year from packed storage (after day and month).
const YEAR_SHIFT: i32 = 9;

/// Minimum valid year.
const MIN_YEAR: i32 = 1800;

/// Maximum valid year.
const MAX_YEAR: i32 = 3000;

/// Days in each month (index 0 is unused for 1-based months).
const DAYS_IN_MONTH: [u8; 13] = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Birthdate type with compressed bit-packed storage.
///
/// This type stores a birthdate in a single i32 value using bit packing
/// to match TDLib's storage format. The layout is:
/// - Bits 0-4: day (1-31)
/// - Bits 5-8: month (1-12)
/// - Bits 9-31: year (1800-3000, or 0 for unknown)
///
/// # Examples
///
/// ```
/// use rustgram_birthdate::Birthdate;
///
/// // Create a birthdate with year
/// let bd = Birthdate::new(15, 6, 1990).unwrap();
/// assert_eq!(bd.day(), 15);
/// assert_eq!(bd.month(), 6);
/// assert_eq!(bd.year(), Some(1990));
///
/// // Create a birthdate without year (year is unknown)
/// let bd_no_year = Birthdate::new(15, 6, 0).unwrap();
/// assert_eq!(bd_no_year.day(), 15);
/// assert_eq!(bd_no_year.month(), 6);
/// assert_eq!(bd_no_year.year(), None);
///
/// // Empty birthdate
/// assert!(Birthdate::new(0, 0, 0).unwrap().is_empty());
/// ```
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Birthdate {
    /// Packed birthdate storage.
    /// Format: day | (month << 5) | (year << 9)
    birthdate: i32,
}

impl Birthdate {
    /// Creates a new birthdate from day, month, and year components.
    ///
    /// # Arguments
    ///
    /// * `day` - Day of month (1-31, or 0 for empty)
    /// * `month` - Month (1-12, or 0 for empty)
    /// * `year` - Year (1800-3000, or 0 for unknown)
    ///
    /// # Returns
    ///
    /// Returns `Ok(Birthdate)` if the date is valid, `Err(BirthdateError)` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_birthdate::Birthdate;
    ///
    /// let bd = Birthdate::new(15, 6, 1990).unwrap();
    /// assert_eq!(bd.day(), 15);
    /// assert_eq!(bd.month(), 6);
    /// assert_eq!(bd.year(), Some(1990));
    /// ```
    pub fn new(day: i32, month: i32, year: i32) -> Result<Self> {
        // All zeros means empty birthdate
        if day == 0 && month == 0 && year == 0 {
            return Ok(Self { birthdate: 0 });
        }

        // Validate day: allow 0 only if month is also 0 (partial empty)
        if !(0..=31).contains(&day) {
            return Err(BirthdateError::InvalidDay(day));
        }

        // Validate month: allow 0 only if day is also 0 (partial empty)
        if !(0..=12).contains(&month) {
            return Err(BirthdateError::InvalidMonth(month));
        }

        // Disallow partial empty states (day=0 XOR month=0)
        if (day == 0) != (month == 0) {
            // day is 0 but month is not, or vice versa
            if day == 0 {
                return Err(BirthdateError::InvalidDay(day));
            } else {
                return Err(BirthdateError::InvalidMonth(month));
            }
        }

        // Validate year
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) && year != 0 {
            return Err(BirthdateError::InvalidYear(year));
        }

        // Validate date combination
        if day > 0 && month > 0 {
            let max_day = if month == 2 {
                // February: 29 for leap years or unknown year, 28 otherwise
                if year == 0 || Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            } else {
                i32::from(DAYS_IN_MONTH[month as usize])
            };

            if day > max_day {
                return Err(BirthdateError::InvalidDate { day, month, year });
            }
        }

        // Pack the birthdate
        let packed = day | (month << MONTH_SHIFT) | (year << YEAR_SHIFT);
        Ok(Self { birthdate: packed })
    }

    /// Returns `true` if this birthdate is empty (all components are zero).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_birthdate::Birthdate;
    ///
    /// assert!(Birthdate::default().is_empty());
    /// assert!(Birthdate::new(0, 0, 0).unwrap().is_empty());
    /// assert!(!Birthdate::new(15, 6, 1990).unwrap().is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.birthdate == 0
    }

    /// Returns the day component (1-31).
    ///
    /// Returns 0 if the birthdate is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_birthdate::Birthdate;
    ///
    /// let bd = Birthdate::new(15, 6, 1990).unwrap();
    /// assert_eq!(bd.day(), 15);
    /// ```
    #[inline]
    pub fn day(&self) -> i32 {
        self.birthdate & DAY_MASK
    }

    /// Returns the month component (1-12).
    ///
    /// Returns 0 if the birthdate is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_birthdate::Birthdate;
    ///
    /// let bd = Birthdate::new(15, 6, 1990).unwrap();
    /// assert_eq!(bd.month(), 6);
    /// ```
    #[inline]
    pub fn month(&self) -> i32 {
        (self.birthdate >> MONTH_SHIFT) & MONTH_MASK
    }

    /// Returns the year component if present (1800-3000).
    ///
    /// Returns `None` if the birthdate is empty or the year is unknown (0).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_birthdate::Birthdate;
    ///
    /// let bd = Birthdate::new(15, 6, 1990).unwrap();
    /// assert_eq!(bd.year(), Some(1990));
    ///
    /// let bd_no_year = Birthdate::new(15, 6, 0).unwrap();
    /// assert_eq!(bd_no_year.year(), None);
    /// ```
    #[inline]
    pub fn year(&self) -> Option<i32> {
        let year = self.birthdate >> YEAR_SHIFT;
        if year == 0 {
            None
        } else {
            Some(year)
        }
    }

    /// Creates a birthdate from a Telegram API birthday object.
    ///
    /// # Arguments
    ///
    /// * `birthday` - Telegram API birthday object
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_birthdate::Birthdate;
    /// # use rustgram_birthdate::tl::TelegramApiBirthday;
    ///
    /// # let telegram_bd = TelegramApiBirthday {
    /// #     flags: 0,
    /// #     day: 15,
    /// #     month: 6,
    /// #     year: None,
    /// # };
    /// let bd = Birthdate::from_telegram_api(&telegram_bd).unwrap();
    /// assert_eq!(bd.day(), 15);
    /// assert_eq!(bd.month(), 6);
    /// ```
    pub fn from_telegram_api(birthday: &TelegramApiBirthday) -> Result<Self> {
        // Extract year from flags
        let has_year = (birthday.flags & YEAR_FLAG_MASK) != 0;
        let year = if has_year {
            birthday.year.ok_or(BirthdateError::InvalidYearFlag)?
        } else {
            0
        };

        Self::new(birthday.day, birthday.month, year)
    }

    /// Creates a birthdate from TD API birthdate object.
    ///
    /// # Arguments
    ///
    /// * `day` - Day from TD API (1-31)
    /// * `month` - Month from TD API (1-12)
    /// * `year` - Year from TD API (0-3000, 0 means unknown)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_birthdate::Birthdate;
    ///
    /// let bd = Birthdate::from_td_api(15, 6, 1990).unwrap();
    /// assert_eq!(bd.day(), 15);
    /// assert_eq!(bd.month(), 6);
    /// assert_eq!(bd.year(), Some(1990));
    /// ```
    pub fn from_td_api(day: i32, month: i32, year: i32) -> Result<Self> {
        Self::new(day, month, year)
    }

    /// Converts this birthdate to a TD API birthdate representation.
    ///
    /// Returns (day, month, year) tuple where year is 0 if unknown.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_birthdate::Birthdate;
    ///
    /// let bd = Birthdate::new(15, 6, 1990).unwrap();
    /// assert_eq!(bd.to_td_api(), (15, 6, 1990));
    ///
    /// let bd_no_year = Birthdate::new(15, 6, 0).unwrap();
    /// assert_eq!(bd_no_year.to_td_api(), (15, 6, 0));
    /// ```
    pub fn to_td_api(&self) -> (i32, i32, i32) {
        (self.day(), self.month(), self.year().unwrap_or(0))
    }

    /// Converts this birthdate to a Telegram API birthday object.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_birthdate::Birthdate;
    ///
    /// let bd = Birthdate::new(15, 6, 1990).unwrap();
    /// let telegram_bd = bd.to_telegram_api();
    /// assert_eq!(telegram_bd.day, 15);
    /// assert_eq!(telegram_bd.month, 6);
    /// assert_eq!(telegram_bd.year, Some(1990));
    /// ```
    pub fn to_telegram_api(&self) -> TelegramApiBirthday {
        let year = self.year();
        let flags = if year.is_some() { YEAR_FLAG_MASK } else { 0 };

        TelegramApiBirthday {
            flags,
            day: self.day(),
            month: self.month(),
            year,
        }
    }

    /// Checks if a year is a leap year.
    ///
    /// Leap years are divisible by 4, except for years divisible by 100
    /// unless they are also divisible by 400.
    #[inline]
    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}

impl fmt::Debug for Birthdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "Birthdate(empty)")
        } else {
            match self.year() {
                Some(year) => write!(
                    f,
                    "Birthdate({:04}-{:02}-{:02})",
                    year,
                    self.month(),
                    self.day()
                ),
                None => write!(f, "Birthdate(----{:02}-{:02})", self.month(), self.day()),
            }
        }
    }
}

impl fmt::Display for Birthdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "")
        } else {
            match self.year() {
                Some(year) => write!(f, "{:04}-{:02}-{:02}", year, self.month(), self.day()),
                None => write!(f, "----{:02}-{:02}", self.month(), self.day()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_birthdate() {
        let bd = Birthdate::default();
        assert!(bd.is_empty());
        assert_eq!(bd.day(), 0);
        assert_eq!(bd.month(), 0);
        assert_eq!(bd.year(), None);

        let bd2 = Birthdate::new(0, 0, 0).unwrap();
        assert!(bd2.is_empty());
        assert_eq!(bd, bd2);
    }

    #[test]
    fn test_valid_date_construction() {
        let bd = Birthdate::new(15, 6, 1990).unwrap();
        assert!(!bd.is_empty());
        assert_eq!(bd.day(), 15);
        assert_eq!(bd.month(), 6);
        assert_eq!(bd.year(), Some(1990));
    }

    #[test]
    fn test_date_without_year() {
        let bd = Birthdate::new(15, 6, 0).unwrap();
        assert!(!bd.is_empty());
        assert_eq!(bd.day(), 15);
        assert_eq!(bd.month(), 6);
        assert_eq!(bd.year(), None);
    }

    #[test]
    fn test_leap_year() {
        // 2000 is a leap year (divisible by 400)
        assert!(Birthdate::is_leap_year(2000));
        assert!(Birthdate::new(29, 2, 2000).is_ok());

        // 1900 is NOT a leap year (divisible by 100 but not 400)
        assert!(!Birthdate::is_leap_year(1900));
        assert!(Birthdate::new(29, 2, 1900).is_err());

        // 2024 is a leap year (divisible by 4, not by 100)
        assert!(Birthdate::is_leap_year(2024));
        assert!(Birthdate::new(29, 2, 2024).is_ok());

        // 2023 is NOT a leap year
        assert!(!Birthdate::is_leap_year(2023));
        assert!(Birthdate::new(29, 2, 2023).is_err());
    }

    #[test]
    fn test_year_validation() {
        // Valid years
        assert!(Birthdate::new(1, 1, 1800).is_ok());
        assert!(Birthdate::new(1, 1, 2000).is_ok());
        assert!(Birthdate::new(1, 1, 3000).is_ok());

        // Invalid years
        assert!(matches!(
            Birthdate::new(1, 1, 1799),
            Err(BirthdateError::InvalidYear(1799))
        ));
        assert!(matches!(
            Birthdate::new(1, 1, 3001),
            Err(BirthdateError::InvalidYear(3001))
        ));
        assert!(matches!(
            Birthdate::new(1, 1, -1),
            Err(BirthdateError::InvalidYear(-1))
        ));
    }

    #[test]
    fn test_invalid_day_rejection() {
        assert!(matches!(
            Birthdate::new(0, 1, 2000),
            Err(BirthdateError::InvalidDay(0))
        ));
        assert!(matches!(
            Birthdate::new(32, 1, 2000),
            Err(BirthdateError::InvalidDay(32))
        ));
        assert!(matches!(
            Birthdate::new(-1, 1, 2000),
            Err(BirthdateError::InvalidDay(-1))
        ));
    }

    #[test]
    fn test_invalid_month_rejection() {
        assert!(matches!(
            Birthdate::new(1, 0, 2000),
            Err(BirthdateError::InvalidMonth(0))
        ));
        assert!(matches!(
            Birthdate::new(1, 13, 2000),
            Err(BirthdateError::InvalidMonth(13))
        ));
        assert!(matches!(
            Birthdate::new(1, -1, 2000),
            Err(BirthdateError::InvalidMonth(-1))
        ));
    }

    #[test]
    fn test_invalid_date_rejection() {
        // February 30 is invalid
        assert!(matches!(
            Birthdate::new(30, 2, 2023),
            Err(BirthdateError::InvalidDate {
                day: 30,
                month: 2,
                year: 2023
            })
        ));

        // April 31 is invalid
        assert!(matches!(
            Birthdate::new(31, 4, 2023),
            Err(BirthdateError::InvalidDate {
                day: 31,
                month: 4,
                year: 2023
            })
        ));

        // June 31 is invalid
        assert!(matches!(
            Birthdate::new(31, 6, 2023),
            Err(BirthdateError::InvalidDate {
                day: 31,
                month: 6,
                year: 2023
            })
        ));

        // September 31 is invalid
        assert!(matches!(
            Birthdate::new(31, 9, 2023),
            Err(BirthdateError::InvalidDate {
                day: 31,
                month: 9,
                year: 2023
            })
        ));

        // November 31 is invalid
        assert!(matches!(
            Birthdate::new(31, 11, 2023),
            Err(BirthdateError::InvalidDate {
                day: 31,
                month: 11,
                year: 2023
            })
        ));
    }

    #[test]
    fn test_from_td_api() {
        let bd = Birthdate::from_td_api(15, 6, 1990).unwrap();
        assert_eq!(bd.day(), 15);
        assert_eq!(bd.month(), 6);
        assert_eq!(bd.year(), Some(1990));

        // Without year
        let bd2 = Birthdate::from_td_api(15, 6, 0).unwrap();
        assert_eq!(bd2.day(), 15);
        assert_eq!(bd2.month(), 6);
        assert_eq!(bd2.year(), None);

        // Empty
        let bd3 = Birthdate::from_td_api(0, 0, 0).unwrap();
        assert!(bd3.is_empty());
    }

    #[test]
    fn test_to_td_api() {
        let bd = Birthdate::new(15, 6, 1990).unwrap();
        assert_eq!(bd.to_td_api(), (15, 6, 1990));

        let bd2 = Birthdate::new(15, 6, 0).unwrap();
        assert_eq!(bd2.to_td_api(), (15, 6, 0));

        let bd3 = Birthdate::default();
        assert_eq!(bd3.to_td_api(), (0, 0, 0));
    }

    #[test]
    fn test_from_telegram_api() {
        // With year
        let telegram_bd = TelegramApiBirthday {
            flags: YEAR_FLAG_MASK,
            day: 15,
            month: 6,
            year: Some(1990),
        };
        let bd = Birthdate::from_telegram_api(&telegram_bd).unwrap();
        assert_eq!(bd.day(), 15);
        assert_eq!(bd.month(), 6);
        assert_eq!(bd.year(), Some(1990));

        // Without year
        let telegram_bd2 = TelegramApiBirthday {
            flags: 0,
            day: 20,
            month: 12,
            year: None,
        };
        let bd2 = Birthdate::from_telegram_api(&telegram_bd2).unwrap();
        assert_eq!(bd2.day(), 20);
        assert_eq!(bd2.month(), 12);
        assert_eq!(bd2.year(), None);

        // Invalid: flag set but no year
        let telegram_bd3 = TelegramApiBirthday {
            flags: YEAR_FLAG_MASK,
            day: 1,
            month: 1,
            year: None,
        };
        assert!(matches!(
            Birthdate::from_telegram_api(&telegram_bd3),
            Err(BirthdateError::InvalidYearFlag)
        ));
    }

    #[test]
    fn test_to_telegram_api() {
        let bd = Birthdate::new(15, 6, 1990).unwrap();
        let telegram_bd = bd.to_telegram_api();
        assert_eq!(telegram_bd.flags, YEAR_FLAG_MASK);
        assert_eq!(telegram_bd.day, 15);
        assert_eq!(telegram_bd.month, 6);
        assert_eq!(telegram_bd.year, Some(1990));

        let bd2 = Birthdate::new(20, 12, 0).unwrap();
        let telegram_bd2 = bd2.to_telegram_api();
        assert_eq!(telegram_bd2.flags, 0);
        assert_eq!(telegram_bd2.day, 20);
        assert_eq!(telegram_bd2.month, 12);
        assert_eq!(telegram_bd2.year, None);

        let bd3 = Birthdate::default();
        let telegram_bd3 = bd3.to_telegram_api();
        assert_eq!(telegram_bd3.flags, 0);
        assert_eq!(telegram_bd3.day, 0);
        assert_eq!(telegram_bd3.month, 0);
        assert_eq!(telegram_bd3.year, None);
    }

    #[test]
    fn test_display_format() {
        let bd = Birthdate::new(15, 6, 1990).unwrap();
        assert_eq!(bd.to_string(), "1990-06-15");

        let bd2 = Birthdate::new(5, 12, 0).unwrap();
        assert_eq!(bd2.to_string(), "----12-05");

        let bd3 = Birthdate::default();
        assert_eq!(bd3.to_string(), "");
    }

    #[test]
    fn test_debug_format() {
        let bd = Birthdate::new(15, 6, 1990).unwrap();
        assert_eq!(format!("{:?}", bd), "Birthdate(1990-06-15)");

        let bd2 = Birthdate::new(5, 12, 0).unwrap();
        assert_eq!(format!("{:?}", bd2), "Birthdate(----12-05)");

        let bd3 = Birthdate::default();
        assert_eq!(format!("{:?}", bd3), "Birthdate(empty)");
    }

    #[test]
    fn test_partial_eq() {
        let bd1 = Birthdate::new(15, 6, 1990).unwrap();
        let bd2 = Birthdate::new(15, 6, 1990).unwrap();
        let bd3 = Birthdate::new(15, 6, 1991).unwrap();
        let bd4 = Birthdate::default();

        assert_eq!(bd1, bd2);
        assert_ne!(bd1, bd3);
        assert_ne!(bd1, bd4);
    }

    #[test]
    fn test_copy_and_clone() {
        let bd1 = Birthdate::new(15, 6, 1990).unwrap();
        let bd2 = bd1; // Copy
        let bd3 = bd1.clone(); // Clone

        assert_eq!(bd1, bd2);
        assert_eq!(bd1, bd3);
        assert_eq!(bd2, bd3);
    }

    #[test]
    fn test_boundary_values() {
        // Minimum year
        let bd1 = Birthdate::new(1, 1, 1800).unwrap();
        assert_eq!(bd1.year(), Some(1800));

        // Maximum year
        let bd2 = Birthdate::new(31, 12, 3000).unwrap();
        assert_eq!(bd2.year(), Some(3000));

        // Maximum valid day for each month
        for month in 1..=12 {
            let max_day = i32::from(DAYS_IN_MONTH[month as usize]);
            let bd = Birthdate::new(max_day, month, 2000).unwrap();
            assert_eq!(bd.day(), max_day);
            assert_eq!(bd.month(), month);
        }
    }

    #[test]
    fn test_february_validation_without_year() {
        // When year is unknown (0), we should allow Feb 29
        let bd = Birthdate::new(29, 2, 0).unwrap();
        assert_eq!(bd.day(), 29);
        assert_eq!(bd.month(), 2);
        assert_eq!(bd.year(), None);
    }

    #[test]
    fn test_february_validation_with_leap_year() {
        // 2024 is a leap year
        let bd = Birthdate::new(29, 2, 2024).unwrap();
        assert_eq!(bd.day(), 29);
        assert_eq!(bd.month(), 2);
        assert_eq!(bd.year(), Some(2024));
    }

    #[test]
    fn test_february_validation_with_non_leap_year() {
        // 2023 is not a leap year
        let result = Birthdate::new(29, 2, 2023);
        assert!(matches!(
            result,
            Err(BirthdateError::InvalidDate {
                day: 29,
                month: 2,
                year: 2023
            })
        ));
    }
}
