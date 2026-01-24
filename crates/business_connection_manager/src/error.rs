// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for business connection manager.

use thiserror::Error;

/// Error type for business connection operations.
#[derive(Debug, Clone, Error)]
pub enum Error {
    /// Connection not found
    #[error("Connection not found: {0}")]
    ConnectionNotFound(String),

    /// Connection was closed or deleted
    #[error("Connection closed: {0}")]
    ConnectionClosed(String),

    /// Invalid dialog ID
    #[error("Invalid dialog ID")]
    InvalidDialogId,

    /// Empty message content
    #[error("Message content cannot be empty")]
    EmptyContent,

    /// Empty message list
    #[error("Message list cannot be empty")]
    EmptyMessageList,

    /// Invalid album size
    #[error("Invalid album size: {0} (must be 2-10)")]
    InvalidAlbumSize(usize),

    /// Name too long
    #[error("Name too long for '{field}': {actual} > {max}")]
    NameTooLong {
        field: &'static str,
        max: usize,
        actual: usize,
    },

    /// Network error occurred
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Internal error occurred
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Result type for business connection operations.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", Error::ConnectionNotFound("test".to_string())),
            "Connection not found: test"
        );
        assert_eq!(
            format!("{}", Error::ConnectionClosed("test".to_string())),
            "Connection closed: test"
        );
        assert_eq!(format!("{}", Error::InvalidDialogId), "Invalid dialog ID");
        assert_eq!(
            format!("{}", Error::EmptyContent),
            "Message content cannot be empty"
        );
        assert_eq!(
            format!("{}", Error::EmptyMessageList),
            "Message list cannot be empty"
        );
        assert_eq!(
            format!("{}", Error::InvalidAlbumSize(5)),
            "Invalid album size: 5 (must be 2-10)"
        );
        assert_eq!(
            format!(
                "{}",
                Error::NameTooLong {
                    field: "first_name",
                    max: 64,
                    actual: 100
                }
            ),
            "Name too long for 'first_name': 100 > 64"
        );
        assert_eq!(
            format!("{}", Error::NetworkError("timeout".to_string())),
            "Network error: timeout"
        );
        assert_eq!(
            format!("{}", Error::InternalError("bug".to_string())),
            "Internal error: bug"
        );
    }

    #[test]
    fn test_error_clone() {
        let error1 = Error::ConnectionNotFound("test".to_string());
        let error2 = error1.clone();
        assert_eq!(error1.to_string(), error2.to_string());
    }

    #[test]
    fn test_error_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Error>();
    }
}
