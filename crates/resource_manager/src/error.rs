//! Error types for resource management.

use std::fmt;

/// Errors that can occur during resource management.
///
/// # Examples
///
/// ```
/// use rustgram_resource_manager::Error;
///
/// let error = Error::LockPoisoned;
/// assert_eq!(error.to_string(), "lock poisoned");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The internal lock was poisoned
    LockPoisoned,
    /// Invalid request parameters
    InvalidRequest(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::LockPoisoned => f.write_str("lock poisoned"),
            Error::InvalidRequest(msg) => write!(f, "invalid request: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Result type for resource management operations.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(Error::LockPoisoned.to_string(), "lock poisoned");
        assert_eq!(
            Error::InvalidRequest("test".to_string()).to_string(),
            "invalid request: test"
        );
    }

    #[test]
    fn test_error_debug() {
        let error = Error::LockPoisoned;
        assert_eq!(format!("{:?}", error), "LockPoisoned");
    }

    #[test]
    fn test_error_eq() {
        assert_eq!(Error::LockPoisoned, Error::LockPoisoned);
        assert_eq!(
            Error::InvalidRequest("test".to_string()),
            Error::InvalidRequest("test".to_string())
        );
    }

    #[test]
    fn test_error_clone() {
        let error1 = Error::LockPoisoned;
        let error2 = error1.clone();
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_result_type() {
        let ok: Result<()> = Ok(());
        let err: Result<()> = Err(Error::LockPoisoned);
        assert!(ok.is_ok());
        assert!(err.is_err());
    }
}
