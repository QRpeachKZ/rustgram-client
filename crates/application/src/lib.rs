//! # Application Utilities
//!
//! Utility functions for Telegram application-level operations.
//!
//! ## Overview
//!
//! This module provides utility functions for common application-level
//! operations like generating invite text and saving application logs.
//!
//! ## TDLib Correspondence
//!
//! TDLib functions in `Application.cpp`:
//! - `get_invite_text` → `invite_text`
//! - `save_app_log` → `save_app_log`
//! - `on_save_app_log_binlog_event` → (handled internally)
//!
//! ## Examples
//!
//! ```
//! use rustgram_application::invite_text;
//!
//! // Get invite text (returns placeholder)
//! let text = invite_text();
//! assert!(!text.is_empty());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use core::fmt;

/// Result type for application operations.
pub type Result<T> = core::result::Result<T, Error>;

/// Error type for application operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Invalid input parameter
    InvalidInput(String),
    /// Operation not supported
    NotSupported(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::NotSupported(msg) => write!(f, "Not supported: {}", msg),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Generates invite text for Telegram.
///
/// This function returns the default invite text that can be used
/// when inviting users to Telegram.
///
/// # Returns
///
/// A string containing the invite text.
///
/// # Examples
///
/// ```
/// use rustgram_application::invite_text;
///
/// let text = invite_text();
/// assert!(!text.is_empty());
/// ```
#[must_use]
pub fn invite_text() -> String {
    // Default invite text from TDLib
    // In a full implementation, this would be localized
    String::from("Join me on Telegram!")
}

/// Saves application log data.
///
/// This function saves application log data to the Telegram server.
/// This is a stub implementation that returns success.
///
/// # Arguments
///
/// * `log_type` - Type of log entry
/// * `data` - Log data to save
///
/// # Returns
///
/// Returns `Ok(())` if successful, `Err(Error)` otherwise.
///
/// # Examples
///
/// ```
/// use rustgram_application::save_app_log;
///
/// let result = save_app_log("test", "sample log data");
/// assert!(result.is_ok());
/// ```
pub fn save_app_log(log_type: &str, data: &str) -> Result<()> {
    if log_type.is_empty() {
        return Err(Error::InvalidInput("log_type cannot be empty".to_string()));
    }
    if data.is_empty() {
        return Err(Error::InvalidInput("data cannot be empty".to_string()));
    }

    // Stub: In a full implementation, this would send the log to the server
    // For now, just validate and return success
    Ok(())
}

/// Validates log data for saving.
///
/// This function checks if log data is valid before saving.
///
/// # Arguments
///
/// * `data` - Log data to validate
///
/// # Returns
///
/// Returns `Ok(())` if valid, `Err(Error)` otherwise.
///
/// # Examples
///
/// ```
/// use rustgram_application::validate_log_data;
///
/// assert!(validate_log_data("valid log data").is_ok());
/// assert!(validate_log_data("invalid \x00 data").is_err());
/// ```
pub fn validate_log_data(data: &str) -> Result<()> {
    // Check for null bytes which are invalid in strings
    if data.contains('\x00') {
        return Err(Error::InvalidInput(
            "log data cannot contain null bytes".to_string(),
        ));
    }

    // Check length limits
    if data.len() > 1_000_000 {
        return Err(Error::InvalidInput(
            "log data too large (max 1MB)".to_string(),
        ));
    }

    Ok(())
}

/// Gets the maximum log data size in bytes.
///
/// # Returns
///
/// The maximum size in bytes (1MB).
///
/// # Examples
///
/// ```
/// use rustgram_application::max_log_size;
///
/// assert_eq!(max_log_size(), 1_000_000);
/// ```
#[must_use]
pub const fn max_log_size() -> usize {
    1_000_000 // 1MB
}

#[cfg(test)]
mod tests {
    use super::*;

    // invite_text tests (3)
    #[test]
    fn test_invite_text_not_empty() {
        let text = invite_text();
        assert!(!text.is_empty());
    }

    #[test]
    fn test_invite_text_contains_telegram() {
        let text = invite_text();
        assert!(text.to_lowercase().contains("telegram"));
    }

    #[test]
    fn test_invite_text_multiple_calls() {
        let text1 = invite_text();
        let text2 = invite_text();
        assert_eq!(text1, text2);
    }

    // save_app_log tests (4)
    #[test]
    fn test_save_app_log_success() {
        let result = save_app_log("test", "sample log data");
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_app_log_empty_type() {
        let result = save_app_log("", "sample log data");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            Error::InvalidInput("log_type cannot be empty".to_string())
        );
    }

    #[test]
    fn test_save_app_log_empty_data() {
        let result = save_app_log("test", "");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            Error::InvalidInput("data cannot be empty".to_string())
        );
    }

    #[test]
    fn test_save_app_log_both_empty() {
        let result = save_app_log("", "");
        assert!(result.is_err());
    }

    // validate_log_data tests (4)
    #[test]
    fn test_validate_log_data_valid() {
        assert!(validate_log_data("valid log data").is_ok());
    }

    #[test]
    fn test_validate_log_data_empty() {
        assert!(validate_log_data("").is_ok());
    }

    #[test]
    fn test_validate_log_data_null_byte() {
        let result = validate_log_data("invalid \x00 data");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_log_data_too_large() {
        let large_data = "x".repeat(1_000_001);
        let result = validate_log_data(&large_data);
        assert!(result.is_err());
    }

    // max_log_size tests (2)
    #[test]
    fn test_max_log_size() {
        assert_eq!(max_log_size(), 1_000_000);
    }

    #[test]
    fn test_max_log_size_const() {
        const SIZE: usize = max_log_size();
        assert_eq!(SIZE, 1_000_000);
    }

    // Error tests (3)
    #[test]
    fn test_error_display_invalid_input() {
        let error = Error::InvalidInput("test error".to_string());
        assert_eq!(format!("{}", error), "Invalid input: test error");
    }

    #[test]
    fn test_error_display_not_supported() {
        let error = Error::NotSupported("test feature".to_string());
        assert_eq!(format!("{}", error), "Not supported: test feature");
    }

    #[test]
    fn test_error_partial_eq() {
        let e1 = Error::InvalidInput("test".to_string());
        let e2 = Error::InvalidInput("test".to_string());
        assert_eq!(e1, e2);

        let e3 = Error::InvalidInput("other".to_string());
        assert_ne!(e1, e3);
    }

    // Edge case tests (2)
    #[test]
    fn test_save_app_log_with_special_chars() {
        let result = save_app_log("test", "data with \n newlines and \t tabs");
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_app_log_long_type() {
        let long_type = "a".repeat(1000);
        let result = save_app_log(&long_type, "data");
        assert!(result.is_ok());
    }
}
