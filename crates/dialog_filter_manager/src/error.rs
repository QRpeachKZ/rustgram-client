//! Error types for dialog filter manager.

use rustgram_types::ChatId;
use thiserror::Error;

/// Result type for dialog filter operations.
pub type Result<T> = std::result::Result<T, DialogFilterError>;

/// Errors that can occur in dialog filter operations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DialogFilterError {
    /// Lock error - failed to acquire RwLock.
    #[error("failed to acquire lock")]
    LockError,

    /// Dialog filter not found.
    #[error("dialog filter with id {0} not found")]
    FilterNotFound(i32),

    /// Invalid dialog filter ID.
    #[error("invalid dialog filter id: {0}")]
    InvalidFilterId(i32),

    /// Dialog not in filter.
    #[error("dialog {0:?} is not in the filter")]
    DialogNotInFilter(ChatId),

    /// Filter limit exceeded.
    #[error("filter limit exceeded: max {max}, requested {requested}")]
    LimitExceeded {
        /// The maximum allowed.
        max: usize,
        /// The requested amount.
        requested: usize,
    },

    /// Empty filter name.
    #[error("filter name cannot be empty")]
    EmptyFilterName,

    /// Filter name too long.
    #[error("filter name too long: max {max} chars, got {len}")]
    FilterNameTooLong {
        /// The maximum length.
        max: usize,
        /// The actual length.
        len: usize,
    },
}
