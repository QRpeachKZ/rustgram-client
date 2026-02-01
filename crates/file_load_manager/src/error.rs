use thiserror::Error;

/// Errors returned by [`FileLoadManager`](crate::FileLoadManager).
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error {
    /// Request with the given file_id was not found.
    #[error("Request not found: {0}")]
    NotFound(u64),
}

/// Result type for [`FileLoadManager`](crate::FileLoadManager) operations.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::NotFound(123);
        assert_eq!(err.to_string(), "Request not found: 123");
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(Error::NotFound(123), Error::NotFound(123));
        assert_ne!(Error::NotFound(123), Error::NotFound(456));
    }

    #[test]
    fn test_result_type() {
        let ok: Result<()> = Ok(());
        assert!(ok.is_ok());

        let err: Result<()> = Err(Error::NotFound(1));
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), Error::NotFound(1));
    }

    #[test]
    fn test_error_clone() {
        let err = Error::NotFound(100);
        let cloned = err.clone();
        assert_eq!(err, cloned);
    }
}
