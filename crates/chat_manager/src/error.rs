//! Error types for chat manager.

use thiserror::Error;

/// Errors that can occur in the chat manager.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ChatError {
    /// Failed to acquire lock on internal state.
    #[error("failed to acquire lock on chat manager state")]
    LockError,

    /// Chat not found.
    #[error("chat not found: {0}")]
    ChatNotFound(i64),

    /// Channel not found.
    #[error("channel not found: {0}")]
    ChannelNotFound(i64),

    /// Invalid access rights for operation.
    #[error("insufficient access rights: {0:?}")]
    InsufficientAccess(super::AccessRights),
}

/// Result type for chat manager operations.
pub type Result<T> = std::result::Result<T, ChatError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", ChatError::LockError),
            "failed to acquire lock on chat manager state"
        );
        assert_eq!(
            format!("{}", ChatError::ChatNotFound(123)),
            "chat not found: 123"
        );
        assert_eq!(
            format!("{}", ChatError::ChannelNotFound(456)),
            "channel not found: 456"
        );
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(ChatError::LockError, ChatError::LockError);
        assert_eq!(ChatError::ChatNotFound(123), ChatError::ChatNotFound(123));
        assert_ne!(ChatError::ChatNotFound(123), ChatError::ChatNotFound(456));
    }
}
