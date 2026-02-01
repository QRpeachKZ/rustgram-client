//! Error types for message import operations.

use thiserror::Error;

/// Errors that can occur during message import operations.
///
/// # Example
///
/// ```rust
/// use rustgram_message_import_manager::Error;
///
/// let error = Error::InvalidDialog {
///     context: "Dialog not found".to_string(),
/// };
/// assert!(error.to_string().contains("Dialog"));
/// ```
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The specified dialog is invalid for message import.
    ///
    /// This can occur when:
    /// - The dialog ID is not valid
    /// - The dialog type doesn't support message imports
    /// - The dialog doesn't exist
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::Error;
    ///
    /// let error = Error::InvalidDialog {
    ///     context: "Dialog ID is invalid".to_string(),
    /// };
    /// assert!(matches!(error, Error::InvalidDialog { .. }));
    /// ```
    #[error("Invalid dialog: {context}")]
    InvalidDialog {
        /// Context about why the dialog is invalid
        context: String,
    },

    /// The message file is invalid or cannot be processed.
    ///
    /// This can occur when:
    /// - The file format is not recognized
    /// - The file is corrupted
    /// - The file contains invalid data
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::Error;
    ///
    /// let error = Error::InvalidFile {
    ///     reason: "File format not recognized".to_string(),
    /// };
    /// assert!(matches!(error, Error::InvalidFile { .. }));
    /// ```
    #[error("Invalid message file: {reason}")]
    InvalidFile {
        /// Reason why the file is invalid
        reason: String,
    },

    /// File upload operation failed.
    ///
    /// This can occur when:
    /// - Network error during upload
    /// - File size exceeds limits
    /// - Upload was cancelled
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::Error;
    ///
    /// let error = Error::UploadFailed {
    ///     reason: "Network timeout".to_string(),
    /// };
    /// assert!(matches!(error, Error::UploadFailed { .. }));
    /// ```
    #[error("File upload failed: {reason}")]
    UploadFailed {
        /// Reason why the upload failed
        reason: String,
    },

    /// Message import operation failed.
    ///
    /// This can occur when:
    /// - Import was rejected by server
    /// - Import data is invalid
    /// - Import state is invalid
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_message_import_manager::Error;
    ///
    /// let error = Error::ImportFailed {
    ///     reason: "Import already in progress".to_string(),
    /// };
    /// assert!(matches!(error, Error::ImportFailed { .. }));
    /// ```
    #[error("Message import failed: {reason}")]
    ImportFailed {
        /// Reason why the import failed
        reason: String,
    },
}

/// Result type for message import operations.
///
/// # Example
///
/// ```rust
/// use rustgram_message_import_manager::{Error, Result};
///
/// fn validate_dialog(id: u64) -> Result<()> {
///     if id == 0 {
///         return Err(Error::InvalidDialog {
///             context: "Invalid dialog ID".to_string(),
///         });
///     }
///     Ok(())
/// }
/// ```
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::InvalidDialog {
            context: "test context".to_string(),
        };
        assert_eq!(err.to_string(), "Invalid dialog: test context");

        let err = Error::InvalidFile {
            reason: "test reason".to_string(),
        };
        assert_eq!(err.to_string(), "Invalid message file: test reason");

        let err = Error::UploadFailed {
            reason: "network error".to_string(),
        };
        assert_eq!(err.to_string(), "File upload failed: network error");

        let err = Error::ImportFailed {
            reason: "import error".to_string(),
        };
        assert_eq!(err.to_string(), "Message import failed: import error");
    }

    #[test]
    fn test_error_clone() {
        let err1 = Error::InvalidDialog {
            context: "test".to_string(),
        };
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_error_equality() {
        let err1 = Error::UploadFailed {
            reason: "test".to_string(),
        };
        let err2 = Error::UploadFailed {
            reason: "test".to_string(),
        };
        assert_eq!(err1, err2);

        let err3 = Error::UploadFailed {
            reason: "other".to_string(),
        };
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_error_debug() {
        let err = Error::ImportFailed {
            reason: "test".to_string(),
        };
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("ImportFailed"));
        assert!(debug_str.contains("test"));
    }

    #[test]
    fn test_result_type() {
        fn returns_ok() -> Result<()> {
            Ok(())
        }

        fn returns_err() -> Result<()> {
            Err(Error::InvalidFile {
                reason: "test".to_string(),
            })
        }

        assert!(returns_ok().is_ok());
        assert!(returns_err().is_err());
    }

    #[test]
    fn test_error_variants() {
        let errors = vec![
            Error::InvalidDialog {
                context: "c1".to_string(),
            },
            Error::InvalidFile {
                reason: "r1".to_string(),
            },
            Error::UploadFailed {
                reason: "r2".to_string(),
            },
            Error::ImportFailed {
                reason: "r3".to_string(),
            },
        ];

        for error in errors {
            // All errors should be Display-able
            let _ = format!("{}", error);

            // All errors should be Debug-able
            let _ = format!("{:?}", error);

            // All errors should be cloneable
            let _ = error.clone();
        }
    }
}
