//! Error types for dialog manager.

use rustgram_net::QueryError;
use rustgram_types::DialogId;
use std::time::Duration;
use thiserror::Error;

/// Result type for dialog operations.
pub type Result<T> = std::result::Result<T, DialogError>;

/// Errors that can occur in dialog operations.
#[derive(Debug, Clone, Error)]
pub enum DialogError {
    /// Lock error - failed to acquire RwLock.
    #[error("failed to acquire lock")]
    LockError,

    /// Dialog not found.
    #[error("dialog {0:?} not found")]
    DialogNotFound(DialogId),

    /// Dialog access denied.
    #[error("access denied to dialog {0:?}")]
    AccessDenied(DialogId),

    /// Invalid input peer.
    #[error("invalid input peer for dialog {0:?}")]
    InvalidInputPeer(DialogId),

    /// Insufficient access rights.
    #[error("insufficient access rights: need {required}, have {have}")]
    InsufficientAccessRights {
        /// Required access level.
        required: String,
        /// Current access level.
        have: String,
    },

    /// Dialog title too long.
    #[error("dialog title too long: max {max} chars, got {len}")]
    TitleTooLong {
        /// The maximum length.
        max: usize,
        /// The actual length.
        len: usize,
    },

    /// Empty dialog title.
    #[error("dialog title cannot be empty")]
    EmptyTitle,

    /// Dialog already exists.
    #[error("dialog {0:?} already exists")]
    AlreadyExists(DialogId),

    /// Username not found.
    #[error("username '{0}' not found")]
    UsernameNotFound(String),

    /// Invalid username format.
    #[error("invalid username format: {0}")]
    InvalidUsername(String),

    /// Network error.
    #[error("network error: {0}")]
    NetworkError(String),

    /// Query execution failed.
    #[error("query failed: {0}")]
    QueryFailed(#[from] QueryError),

    /// Request timeout.
    #[error("request timeout after {0:?}")]
    Timeout(Duration),

    /// Authentication required.
    #[error("authentication required for this operation")]
    AuthRequired,

    /// Rate limited.
    #[error("rate limited: retry after {0}s")]
    RateLimited(u32),

    /// Invalid TL data.
    #[error("invalid TL data: {0}")]
    InvalidTlData(String),

    /// Serialization error.
    #[error("serialization error: {0}")]
    SerializationError(String),

    /// Deserialization error.
    #[error("deserialization error: {0}")]
    DeserializationError(String),

    /// Cache error.
    #[error("cache error: {0}")]
    CacheError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_types::{ChatId, DialogId};

    #[test]
    fn test_error_display() {
        let chat_id = ChatId::new(123).unwrap();
        let dialog_id = DialogId::from_chat(chat_id);

        assert_eq!(
            format!("{}", DialogError::DialogNotFound(dialog_id)),
            format!("dialog {:?} not found", dialog_id)
        );

        assert_eq!(
            format!("{}", DialogError::EmptyTitle),
            "dialog title cannot be empty"
        );

        assert_eq!(
            format!("{}", DialogError::AuthRequired),
            "authentication required for this operation"
        );

        assert_eq!(
            format!("{}", DialogError::RateLimited(60)),
            "rate limited: retry after 60s"
        );
    }

    #[test]
    fn test_network_error_variants() {
        let err = DialogError::NetworkError("Connection lost".to_string());
        assert_eq!(err.to_string(), "network error: Connection lost");

        let timeout = Duration::from_secs(30);
        let err = DialogError::Timeout(timeout);
        assert_eq!(err.to_string(), "request timeout after 30s");

        let err = DialogError::InvalidTlData("Invalid constructor".to_string());
        assert_eq!(err.to_string(), "invalid TL data: Invalid constructor");
    }
}

impl PartialEq for DialogError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (DialogError::LockError, DialogError::LockError) => true,
            (DialogError::DialogNotFound(a), DialogError::DialogNotFound(b)) => a == b,
            (DialogError::AccessDenied(a), DialogError::AccessDenied(b)) => a == b,
            (DialogError::InvalidInputPeer(a), DialogError::InvalidInputPeer(b)) => a == b,
            (
                DialogError::InsufficientAccessRights {
                    required: r1,
                    have: h1,
                },
                DialogError::InsufficientAccessRights {
                    required: r2,
                    have: h2,
                },
            ) => r1 == r2 && h1 == h2,
            (
                DialogError::TitleTooLong { max: m1, len: l1 },
                DialogError::TitleTooLong { max: m2, len: l2 },
            ) => m1 == m2 && l1 == l2,
            (DialogError::EmptyTitle, DialogError::EmptyTitle) => true,
            (DialogError::AlreadyExists(a), DialogError::AlreadyExists(b)) => a == b,
            (DialogError::UsernameNotFound(a), DialogError::UsernameNotFound(b)) => a == b,
            (DialogError::InvalidUsername(a), DialogError::InvalidUsername(b)) => a == b,
            (DialogError::NetworkError(a), DialogError::NetworkError(b)) => a == b,
            (DialogError::QueryFailed(a), DialogError::QueryFailed(b)) => {
                // QueryError doesn't implement PartialEq, so we compare the error messages
                format!("{:?}", a) == format!("{:?}", b)
            }
            (DialogError::Timeout(_), DialogError::Timeout(_)) => true,
            (DialogError::AuthRequired, DialogError::AuthRequired) => true,
            (DialogError::RateLimited(a), DialogError::RateLimited(b)) => a == b,
            (DialogError::InvalidTlData(a), DialogError::InvalidTlData(b)) => a == b,
            (DialogError::SerializationError(a), DialogError::SerializationError(b)) => a == b,
            (DialogError::DeserializationError(a), DialogError::DeserializationError(b)) => a == b,
            (DialogError::CacheError(a), DialogError::CacheError(b)) => a == b,
            _ => false,
        }
    }
}
