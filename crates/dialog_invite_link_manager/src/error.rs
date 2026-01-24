//! Error types for dialog invite link manager.

use rustgram_types::DialogId;
use thiserror::Error;

/// Result type for invite link operations.
pub type Result<T> = std::result::Result<T, InviteLinkError>;

/// Errors that can occur in invite link operations.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InviteLinkError {
    /// Lock error - failed to acquire RwLock.
    #[error("failed to acquire lock")]
    LockError,

    /// Dialog not found.
    #[error("dialog {0:?} not found")]
    DialogNotFound(DialogId),

    /// Invite link not found.
    #[error("invite link '{0}' not found")]
    LinkNotFound(String),

    /// Invalid invite link URL.
    #[error("invalid invite link URL: {0}")]
    InvalidLinkUrl(String),

    /// Invalid expire date.
    #[error("invalid expire date: {0}")]
    InvalidExpireDate(i32),

    /// Invalid usage limit.
    #[error("invalid usage limit: {0}")]
    InvalidUsageLimit(i32),

    /// Insufficient permissions.
    #[error("insufficient permissions to manage invite links")]
    InsufficientPermissions,

    /// Link title too long.
    #[error("link title too long: max {max} chars, got {len}")]
    TitleTooLong {
        /// The maximum length.
        max: usize,
        /// The actual length.
        len: usize,
    },

    /// No permanent link exists.
    #[error("no permanent invite link exists for this dialog")]
    NoPermanentLink,

    /// Link already revoked.
    #[error("invite link is already revoked")]
    AlreadyRevoked,
}
