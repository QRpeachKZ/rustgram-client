// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram TdLog - Logging interface types for Telegram MTProto.
//!
//! This module provides type definitions for the TDLib logging interface.
//! The actual FFI calls are only available when the TDLib C library is linked.
//!
//! ## Overview
//!
//! The logging interface provides type definitions for managing TDLib's internal logging:
//!
//! - [`TdLogError`] - Error type for logging operations
//! - [`TdLogFatalErrorCallback`] - Fatal error callback function type
//!
//! ## TDLib Correspondence
//!
//! | Rust type | C type | File |
//! |-----------|-------|------|
//! | [`TdLogError`] | N/A | `td_log.h` |
//! | [`TdLogFatalErrorCallback`] | `td_log_fatal_error_callback_ptr` | `td_log.h:66` |
//!
//! ## Examples
//!
//! ```no_run
//! use rustgram_td_log::TdLogError;
//!
//! // Error types for logging operations
//! let error = TdLogError::FileOpenFailed;
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

/// Errors that can occur when working with the TDLib logging interface.
#[derive(thiserror::Error, Debug)]
pub enum TdLogError {
    /// Failed to open log file
    #[error("Failed to open log file")]
    FileOpenFailed,

    /// Invalid UTF-8 string
    #[error("Invalid UTF-8 string")]
    InvalidUtf8,

    /// Null pointer received
    #[error("Null pointer encountered")]
    NullPointer,
}

/// Result type for logging operations.
pub type Result<T> = std::result::Result<T, TdLogError>;

/// Fatal error callback function type.
///
/// This callback is invoked when a fatal error occurs in TDLib.
/// The application will crash as soon as the callback returns.
///
/// # TDLib Correspondence
///
/// TDLib reference: `td_log.h:66`
///
/// # Arguments
///
/// * `error_message` - Null-terminated C string describing the fatal error
///
/// # Safety
///
/// None of the TDLib methods can be called from this callback.
pub type TdLogFatalErrorCallback = unsafe extern "C" fn(error_message: *const i8);

/// Maximum verbosity level.
pub const MAX_VERBOSITY_LEVEL: i32 = 1024;

/// Minimum verbosity level.
pub const MIN_VERBOSITY_LEVEL: i32 = 0;

/// Default verbosity level (TDLib default).
pub const DEFAULT_VERBOSITY_LEVEL: i32 = 5;

/// Default maximum log file size (10 MB).
pub const DEFAULT_MAX_FILE_SIZE: i64 = 10 * 1024 * 1024;

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-td-log";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-td-log");
    }

    #[test]
    fn test_constants() {
        assert_eq!(MIN_VERBOSITY_LEVEL, 0);
        assert_eq!(MAX_VERBOSITY_LEVEL, 1024);
        assert_eq!(DEFAULT_VERBOSITY_LEVEL, 5);
        assert_eq!(DEFAULT_MAX_FILE_SIZE, 10 * 1024 * 1024);
    }

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", TdLogError::FileOpenFailed),
            "Failed to open log file"
        );
        assert_eq!(
            format!("{}", TdLogError::InvalidUtf8),
            "Invalid UTF-8 string"
        );
        assert_eq!(
            format!("{}", TdLogError::NullPointer),
            "Null pointer encountered"
        );
    }
}
