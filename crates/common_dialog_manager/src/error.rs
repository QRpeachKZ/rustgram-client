//! Error types for common dialog manager.

use thiserror::Error;

/// Errors that can occur in the common dialog manager.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CommonDialogError {
    /// Failed to acquire lock on internal state.
    #[error("failed to acquire lock on common dialog manager state")]
    LockError,

    /// User not found in cache.
    #[error("user not found in cache: {0}")]
    UserNotFound(i64),

    /// Limit exceeds maximum allowed.
    #[error("limit {requested} exceeds maximum {max}")]
    LimitExceeded {
        /// The requested limit.
        requested: i32,
        /// The maximum allowed limit.
        max: i32,
    },

    /// Invalid limit value.
    #[error("invalid limit: {0} (must be > 0)")]
    InvalidLimit(i32),
}

/// Result type for common dialog manager operations.
pub type Result<T> = std::result::Result<T, CommonDialogError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", CommonDialogError::LockError),
            "failed to acquire lock on common dialog manager state"
        );
        assert_eq!(
            format!("{}", CommonDialogError::UserNotFound(123)),
            "user not found in cache: 123"
        );
        assert_eq!(
            format!(
                "{}",
                CommonDialogError::LimitExceeded {
                    requested: 150,
                    max: 100
                }
            ),
            "limit 150 exceeds maximum 100"
        );
        assert_eq!(
            format!("{}", CommonDialogError::InvalidLimit(0)),
            "invalid limit: 0 (must be > 0)"
        );
        assert_eq!(
            format!("{}", CommonDialogError::InvalidLimit(-5)),
            "invalid limit: -5 (must be > 0)"
        );
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(CommonDialogError::LockError, CommonDialogError::LockError);
        assert_eq!(
            CommonDialogError::UserNotFound(123),
            CommonDialogError::UserNotFound(123)
        );
        assert_ne!(
            CommonDialogError::UserNotFound(123),
            CommonDialogError::UserNotFound(456)
        );
        assert_eq!(
            CommonDialogError::LimitExceeded {
                requested: 150,
                max: 100
            },
            CommonDialogError::LimitExceeded {
                requested: 150,
                max: 100
            }
        );
        assert_ne!(
            CommonDialogError::LimitExceeded {
                requested: 150,
                max: 100
            },
            CommonDialogError::LimitExceeded {
                requested: 200,
                max: 100
            }
        );
        assert_eq!(
            CommonDialogError::InvalidLimit(0),
            CommonDialogError::InvalidLimit(0)
        );
        assert_ne!(
            CommonDialogError::InvalidLimit(0),
            CommonDialogError::InvalidLimit(-1)
        );
    }
}
