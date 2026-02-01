// Copyright (c) 2025 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for phone number manager.

use thiserror::Error;

/// Result type for phone number manager operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during phone number operations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    /// An operation is already in progress.
    #[error("Operation already in progress: {operation:?}")]
    OperationInProgress {
        /// The currently active operation.
        operation: crate::Type,
    },

    /// No operation is currently in progress.
    #[error("No operation is currently in progress")]
    NoOperationInProgress,

    /// The authentication code is invalid.
    #[error("Invalid authentication code")]
    InvalidCode,

    /// The state is invalid for this operation.
    #[error("Invalid state: expected {expected:?}, got {got:?}")]
    InvalidState {
        /// The expected state.
        expected: crate::State,
        /// The actual state.
        got: crate::State,
    },

    /// A network error occurred.
    #[error("Network error: {0}")]
    NetworkError(String),

    /// A timeout occurred.
    #[error("Operation timed out")]
    Timeout,

    /// The phone number format is invalid.
    #[error("Invalid phone number format: {0}")]
    InvalidPhoneNumber(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", Error::NoOperationInProgress),
            "No operation is currently in progress"
        );
        assert_eq!(
            format!("{}", Error::InvalidCode),
            "Invalid authentication code"
        );
        assert_eq!(format!("{}", Error::Timeout), "Operation timed out");
    }

    #[test]
    fn test_error_operation_in_progress() {
        let error = Error::OperationInProgress {
            operation: crate::Type::ChangePhone,
        };
        assert!(format!("{:?}", error).contains("ChangePhone"));
    }

    #[test]
    fn test_error_invalid_state() {
        let error = Error::InvalidState {
            expected: crate::State::WaitCode,
            got: crate::State::Ok,
        };
        assert!(format!("{}", error).contains("WaitCode"));
        assert!(format!("{}", error).contains("Ok"));
    }
}
