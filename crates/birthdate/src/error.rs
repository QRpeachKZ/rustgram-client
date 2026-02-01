//! Error types for the birthdate module.

use thiserror::Error;

/// Result type for birthdate operations.
pub type Result<T> = std::result::Result<T, BirthdateError>;

/// Errors that can occur when working with birthdates.
///
/// These errors typically arise from validation failures when constructing
/// or converting birthdate values.
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BirthdateError {
    /// The day value is invalid (must be 1-31).
    #[error("invalid day: {0}, must be between 1 and 31")]
    InvalidDay(i32),

    /// The month value is invalid (must be 1-12).
    #[error("invalid month: {0}, must be between 1 and 12")]
    InvalidMonth(i32),

    /// The year value is invalid (must be 1800-3000 or 0 for unknown).
    #[error("invalid year: {0}, must be between 1800 and 3000, or 0 for unknown")]
    InvalidYear(i32),

    /// The date combination is invalid (e.g., February 30).
    #[error("invalid date: {day}/{month}/{year}")]
    InvalidDate {
        /// The invalid day value.
        day: i32,
        /// The invalid month value.
        month: i32,
        /// The invalid year value.
        year: i32,
    },

    /// The year flag bit pattern is invalid in Telegram API data.
    #[error("invalid year flag in Telegram API data")]
    InvalidYearFlag,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            BirthdateError::InvalidDay(32).to_string(),
            "invalid day: 32, must be between 1 and 31"
        );
        assert_eq!(
            BirthdateError::InvalidMonth(13).to_string(),
            "invalid month: 13, must be between 1 and 12"
        );
        assert_eq!(
            BirthdateError::InvalidYear(1799).to_string(),
            "invalid year: 1799, must be between 1800 and 3000, or 0 for unknown"
        );
        assert_eq!(
            BirthdateError::InvalidDate {
                day: 31,
                month: 2,
                year: 2023
            }
            .to_string(),
            "invalid date: 31/2/2023"
        );
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(
            BirthdateError::InvalidDay(32),
            BirthdateError::InvalidDay(32)
        );
        assert_eq!(
            BirthdateError::InvalidMonth(0),
            BirthdateError::InvalidMonth(0)
        );
        assert_ne!(BirthdateError::InvalidDay(1), BirthdateError::InvalidDay(2));
    }

    #[test]
    fn test_error_clone_copy() {
        let err1 = BirthdateError::InvalidYear(1799);
        let err2 = err1;
        let err3 = err1;
        assert_eq!(err2, err3);
    }
}
