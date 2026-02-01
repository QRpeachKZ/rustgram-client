// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for MessageQueryManager.

use std::fmt;

/// Errors that can occur in message query operations.
///
/// # Example
///
/// ```rust
/// use rustgram_message_query_manager::Error;
///
/// let err = Error::InvalidDialog;
/// assert!(matches!(err, Error::InvalidDialog));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Invalid dialog ID provided.
    InvalidDialog,

    /// Invalid message ID provided.
    InvalidMessage,

    /// Message upload operation failed.
    UploadFailed,

    /// Message deletion operation failed.
    DeleteFailed,

    /// Message search operation failed.
    SearchFailed,

    /// Invalid state for the requested operation.
    InvalidState,

    /// Network or I/O error.
    IoError(String),

    /// Generic error with message.
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDialog => write!(f, "Invalid dialog ID"),
            Self::InvalidMessage => write!(f, "Invalid message ID"),
            Self::UploadFailed => write!(f, "Message upload failed"),
            Self::DeleteFailed => write!(f, "Message deletion failed"),
            Self::SearchFailed => write!(f, "Message search failed"),
            Self::InvalidState => write!(f, "Invalid state for operation"),
            Self::IoError(msg) => write!(f, "I/O error: {msg}"),
            Self::Other(msg) => write!(f, "Error: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

/// Result type for MessageQueryManager operations.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_invalid_dialog() {
        let err = Error::InvalidDialog;
        assert!(matches!(err, Error::InvalidDialog));
        assert_eq!(format!("{err}"), "Invalid dialog ID");
    }

    #[test]
    fn test_error_invalid_message() {
        let err = Error::InvalidMessage;
        assert!(matches!(err, Error::InvalidMessage));
        assert_eq!(format!("{err}"), "Invalid message ID");
    }

    #[test]
    fn test_error_upload_failed() {
        let err = Error::UploadFailed;
        assert!(matches!(err, Error::UploadFailed));
        assert_eq!(format!("{err}"), "Message upload failed");
    }

    #[test]
    fn test_error_delete_failed() {
        let err = Error::DeleteFailed;
        assert!(matches!(err, Error::DeleteFailed));
        assert_eq!(format!("{err}"), "Message deletion failed");
    }

    #[test]
    fn test_error_search_failed() {
        let err = Error::SearchFailed;
        assert!(matches!(err, Error::SearchFailed));
        assert_eq!(format!("{err}"), "Message search failed");
    }

    #[test]
    fn test_error_invalid_state() {
        let err = Error::InvalidState;
        assert!(matches!(err, Error::InvalidState));
        assert_eq!(format!("{err}"), "Invalid state for operation");
    }

    #[test]
    fn test_error_io_error() {
        let err = Error::IoError("connection failed".to_string());
        assert!(matches!(err, Error::IoError(_)));
        assert!(format!("{err}").contains("connection failed"));
    }

    #[test]
    fn test_error_other() {
        let err = Error::Other("unknown error".to_string());
        assert!(matches!(err, Error::Other(_)));
        assert!(format!("{err}").contains("unknown error"));
    }

    #[test]
    fn test_error_clone() {
        let err1 = Error::InvalidDialog;
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_error_equality() {
        let err1 = Error::UploadFailed;
        let err2 = Error::UploadFailed;
        let err3 = Error::DeleteFailed;

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_error_debug() {
        let err = Error::SearchFailed;
        let debug_str = format!("{err:?}");
        assert!(debug_str.contains("SearchFailed"));
    }
}
