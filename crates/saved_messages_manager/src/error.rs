//! Error types for saved messages manager.

use thiserror::Error;

/// Errors that can occur in the saved messages manager.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum SavedMessagesError {
    /// Topic not found.
    #[error("topic not found: {0}")]
    TopicNotFound(i64),

    /// Invalid topic ID.
    #[error("invalid topic ID: {0}")]
    InvalidTopicId(i64),

    /// Dialog not found.
    #[error("dialog not found")]
    DialogNotFound,

    /// Invalid dialog ID.
    #[error("invalid dialog ID")]
    InvalidDialogId,

    /// Message not found.
    #[error("message not found: {0}")]
    MessageNotFound(i64),

    /// Invalid message ID.
    #[error("invalid message ID: {0}")]
    InvalidMessageId(i64),

    /// Operation not supported.
    #[error("operation not supported: {0}")]
    NotSupported(String),

    /// Invalid state for operation.
    #[error("invalid state: {0}")]
    InvalidState(String),

    /// Maximum pinned topics exceeded.
    #[error("maximum pinned topics exceeded: {0} > {1}")]
    MaxPinnedExceeded(usize, usize),

    /// Topic list not loaded.
    #[error("topic list not loaded")]
    TopicListNotLoaded,

    /// History loading in progress.
    #[error("history loading in progress")]
    HistoryLoadingInProgress,

    /// Invalid date range.
    #[error("invalid date range: {0} > {1}")]
    InvalidDateRange(i32, i32),
}

impl SavedMessagesError {
    /// Creates a topic not found error.
    #[inline]
    pub const fn topic_not_found(topic_id: i64) -> Self {
        Self::TopicNotFound(topic_id)
    }

    /// Creates an invalid topic ID error.
    #[inline]
    pub const fn invalid_topic_id(topic_id: i64) -> Self {
        Self::InvalidTopicId(topic_id)
    }

    /// Creates a message not found error.
    #[inline]
    pub const fn message_not_found(message_id: i64) -> Self {
        Self::MessageNotFound(message_id)
    }

    /// Creates an invalid message ID error.
    #[inline]
    pub const fn invalid_message_id(message_id: i64) -> Self {
        Self::InvalidMessageId(message_id)
    }

    /// Creates a not supported error.
    #[inline]
    pub fn not_supported(msg: String) -> Self {
        Self::NotSupported(msg)
    }

    /// Creates an invalid state error.
    #[inline]
    pub fn invalid_state(msg: String) -> Self {
        Self::InvalidState(msg)
    }

    /// Creates a max pinned exceeded error.
    #[inline]
    pub const fn max_pinned_exceeded(actual: usize, max: usize) -> Self {
        Self::MaxPinnedExceeded(actual, max)
    }
}

/// Result type for saved messages operations.
pub type Result<T> = std::result::Result<T, SavedMessagesError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = SavedMessagesError::topic_not_found(123);
        assert_eq!(err.to_string(), "topic not found: 123");

        let err = SavedMessagesError::invalid_topic_id(0);
        assert_eq!(err.to_string(), "invalid topic ID: 0");

        let err = SavedMessagesError::DialogNotFound;
        assert_eq!(err.to_string(), "dialog not found");

        let err = SavedMessagesError::message_not_found(456);
        assert_eq!(err.to_string(), "message not found: 456");

        let err = SavedMessagesError::not_supported("test operation".to_string());
        assert_eq!(err.to_string(), "operation not supported: test operation");

        let err = SavedMessagesError::max_pinned_exceeded(6, 5);
        assert_eq!(err.to_string(), "maximum pinned topics exceeded: 6 > 5");

        let err = SavedMessagesError::InvalidDateRange(100, 50);
        assert_eq!(err.to_string(), "invalid date range: 100 > 50");
    }

    #[test]
    fn test_error_equality() {
        let err1 = SavedMessagesError::topic_not_found(123);
        let err2 = SavedMessagesError::topic_not_found(123);
        assert_eq!(err1, err2);

        let err3 = SavedMessagesError::topic_not_found(456);
        assert_ne!(err1, err3);
    }
}
