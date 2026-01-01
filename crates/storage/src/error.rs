//! Error types for storage operations.

use thiserror::Error;

/// Result type alias for storage operations.
pub type StorageResult<T> = Result<T, StorageError>;

/// Errors that can occur during storage operations.
#[derive(Error, Debug)]
pub enum StorageError {
    /// Database connection error.
    #[error("Database connection error: {0}")]
    ConnectionError(#[from] rusqlite::Error),

    /// I/O error during file operations.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Database schema error (missing table, invalid schema, etc.).
    #[error("Schema error: {0}")]
    SchemaError(String),

    /// Migration failed.
    #[error("Migration error: {0}")]
    MigrationError(String),

    /// Transaction error.
    #[error("Transaction error: {0}")]
    TransactionError(String),

    /// Query execution error.
    #[error("Query error: {0}")]
    QueryError(String),

    /// Record not found.
    #[error("Record not found: {0}")]
    NotFound(String),

    /// Invalid parameter value.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Encryption/decryption error (secure storage).
    #[error("Crypto error: {0}")]
    CryptoError(String),

    /// Database is locked.
    #[error("Database is locked")]
    DatabaseLocked,

    /// Database is corrupted.
    #[error("Database is corrupted")]
    DatabaseCorrupted,

    /// Connection pool exhausted.
    #[error("Connection pool exhausted")]
    PoolExhausted,
}

impl StorageError {
    /// Returns `true` if the error is transient (retryable).
    pub fn is_transient(&self) -> bool {
        matches!(
            self,
            Self::DatabaseLocked | Self::ConnectionError(_) | Self::PoolExhausted
        )
    }

    /// Returns `true` if the error indicates a missing record.
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_is_transient() {
        let err = StorageError::DatabaseLocked;
        assert!(err.is_transient());

        let err = StorageError::NotFound("test".to_string());
        assert!(!err.is_transient());
    }

    #[test]
    fn test_error_is_not_found() {
        let err = StorageError::NotFound("test".to_string());
        assert!(err.is_not_found());

        let err = StorageError::DatabaseLocked;
        assert!(!err.is_not_found());
    }

    #[test]
    fn test_storage_result_alias() {
        let ok_result: StorageResult<i32> = Ok(42);
        assert!(ok_result.is_ok());

        let err_result: StorageResult<i32> = Err(StorageError::NotFound("test".to_string()));
        assert!(err_result.is_err());
    }
}
