// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for the inline queries manager.

use rustgram_dialog_id::DialogId;
use rustgram_types::UserId;
use thiserror::Error;

/// Result type for inline queries manager operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in the inline queries manager.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum Error {
    /// Invalid user ID provided.
    #[error("invalid user ID: {0}")]
    InvalidUserId(UserId),

    /// Invalid dialog ID provided.
    #[error("invalid dialog ID: {0}")]
    InvalidDialogId(DialogId),

    /// Invalid query ID provided.
    #[error("invalid query ID: {0}")]
    InvalidQueryId(i64),

    /// Query timed out.
    #[error("query timed out")]
    QueryTimeout,

    /// Network error occurred.
    #[error("network error: {0}")]
    NetworkError(String),

    /// Serialization error.
    #[error("serialization error: {0}")]
    SerializationError(String),

    /// Invalid inline message content.
    #[error("invalid inline message content: {0}")]
    InvalidMessageContent(String),

    /// Bot not found or not accessible.
    #[error("bot not found: {0}")]
    BotNotFound(UserId),

    /// Rate limit exceeded for inline queries.
    #[error("rate limit exceeded, try again later")]
    RateLimitExceeded,

    /// Invalid prepared message ID.
    #[error("invalid prepared message ID: {0}")]
    InvalidPreparedMessageId(String),

    /// Weather query failed.
    #[error("weather query failed: {0}")]
    WeatherQueryError(String),

    /// Internal error.
    #[error("internal error: {0}")]
    InternalError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let user_id = UserId::new(123).unwrap_or_else(|_| UserId::default());
        let err = Error::InvalidUserId(user_id);
        assert!(err.to_string().contains("invalid user ID"));
    }

    #[test]
    fn test_error_eq() {
        let user_id = UserId::new(123).unwrap_or_else(|_| UserId::default());
        let err1 = Error::InvalidUserId(user_id);
        let err2 = Error::InvalidUserId(user_id);
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_error_invalid_query_id() {
        let err = Error::InvalidQueryId(-1);
        assert!(err.to_string().contains("invalid query ID"));
        assert!(err.to_string().contains("-1"));
    }

    #[test]
    fn test_error_network_error() {
        let err = Error::NetworkError("connection failed".to_string());
        assert!(err.to_string().contains("network error"));
        assert!(err.to_string().contains("connection failed"));
    }

    #[test]
    fn test_error_rate_limit() {
        let err = Error::RateLimitExceeded;
        assert!(err.to_string().contains("rate limit"));
    }
}
