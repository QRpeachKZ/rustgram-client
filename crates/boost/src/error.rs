// Copyright 2025 rustgram-client
//
// Licensed under MIT License

//! Error types for boost module.

use thiserror::Error;

/// Errors that can occur in the boost module.
#[derive(Error, Debug)]
pub enum BoostError {
    /// The dialog is invalid or not accessible.
    #[error("Invalid dialog: {0}")]
    InvalidDialog(String),

    /// The user ID is invalid.
    #[error("Invalid user ID: {0}")]
    InvalidUserId(String),

    /// The slot ID is invalid.
    #[error("Invalid slot ID: {0}")]
    InvalidSlotId(String),

    /// Access to the dialog was denied.
    #[error("Access denied to dialog: {0}")]
    AccessDenied(String),

    /// Network error occurred.
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Failed to parse response.
    #[error("Parse error: {0}")]
    ParseError(String),

    /// Invalid boost link URL.
    #[error("Invalid boost link: {0}")]
    InvalidBoostLink(String),

    /// The limit parameter must be positive.
    #[error("Limit must be positive, got {0}")]
    InvalidLimit(i32),

    /// Cannot boost the specified chat type.
    #[error("Cannot boost this type of chat")]
    CannotBoostChat,

    /// Internal error.
    #[error("Internal error: {0}")]
    Internal(String),
}

impl BoostError {
    /// Create an invalid dialog error.
    pub fn invalid_dialog(msg: impl Into<String>) -> Self {
        Self::InvalidDialog(msg.into())
    }

    /// Create an invalid user ID error.
    pub fn invalid_user_id(msg: impl Into<String>) -> Self {
        Self::InvalidUserId(msg.into())
    }

    /// Create an access denied error.
    pub fn access_denied(msg: impl Into<String>) -> Self {
        Self::AccessDenied(msg.into())
    }

    /// Create a network error.
    pub fn network(msg: impl Into<String>) -> Self {
        Self::NetworkError(msg.into())
    }

    /// Create a parse error.
    pub fn parse(msg: impl Into<String>) -> Self {
        Self::ParseError(msg.into())
    }
}

/// Result type for boost operations.
pub type Result<T> = std::result::Result<T, BoostError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        let err = BoostError::InvalidDialog("test error".to_string());
        assert_eq!(err.to_string(), "Invalid dialog: test error");

        let err = BoostError::InvalidLimit(0);
        assert_eq!(err.to_string(), "Limit must be positive, got 0");
    }

    #[test]
    fn test_error_constructors() {
        let err = BoostError::invalid_dialog("dialog not found");
        assert!(matches!(err, BoostError::InvalidDialog(_)));

        let err = BoostError::access_denied("no permission");
        assert!(matches!(err, BoostError::AccessDenied(_)));
    }
}
