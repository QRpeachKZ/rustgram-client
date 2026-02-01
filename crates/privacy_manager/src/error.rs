// Copyright (c) 2025 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for privacy manager.

use thiserror::Error;

use crate::PrivacyKey;

/// Result type for privacy manager operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during privacy operations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    /// A set query is already in progress.
    #[error("Set query already in progress for {:?}", key)]
    SetQueryInProgress {
        /// The privacy setting key.
        key: PrivacyKey,
    },

    /// Network error occurred.
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Timeout occurred.
    #[error("Operation timed out")]
    Timeout,

    /// Invalid user ID.
    #[error("Invalid user ID")]
    InvalidUserId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", Error::Timeout),
            "Operation timed out"
        );
        assert_eq!(
            format!("{}", Error::InvalidUserId),
            "Invalid user ID"
        );
    }

    #[test]
    fn test_error_set_query_in_progress() {
        let error = Error::SetQueryInProgress {
            key: PrivacyKey::Status,
        };

        assert!(format!("{}", error).contains("Status"));
    }
}
