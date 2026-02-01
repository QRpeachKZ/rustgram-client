// Copyright (c) 2025 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for poll manager.

use thiserror::Error;

use crate::PollId;

/// Result type for poll manager operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during poll operations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    /// Poll was not found.
    #[error("Poll not found: {poll_id:?}")]
    PollNotFound {
        /// The poll ID that was not found.
        poll_id: PollId,
    },

    /// Poll is closed and cannot accept votes.
    #[error("Poll is closed")]
    PollClosed,

    /// Invalid option ID.
    #[error("Invalid option ID: {0}")]
    InvalidOptionId(i32),

    /// Network error occurred.
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Timeout occurred.
    #[error("Operation timed out")]
    Timeout,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(format!("{}", Error::PollClosed), "Poll is closed");
        assert_eq!(
            format!("{}", Error::InvalidOptionId(999)),
            "Invalid option ID: 999"
        );
        assert_eq!(format!("{}", Error::Timeout), "Operation timed out");
    }

    #[test]
    fn test_error_poll_not_found() {
        let poll_id = PollId::new(123);
        let error = Error::PollNotFound { poll_id };

        assert!(format!("{}", error).contains("123"));
    }
}
