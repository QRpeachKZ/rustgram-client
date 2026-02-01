// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for secret chat database operations.

use thiserror::Error;

/// Errors that can occur during secret chat database operations.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum SecretChatDbError {
    /// The requested value was not found in storage.
    #[error("Value not found for key: {0}")]
    NotFound(String),

    /// An error occurred during serialization.
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// An error occurred during deserialization.
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// An error occurred in the underlying storage.
    #[error("Storage error: {0}")]
    StorageError(String),
}

/// Result type for secret chat database operations.
pub type SecretChatDbResult<T> = Result<T, SecretChatDbError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = SecretChatDbError::NotFound("secret123layer".to_string());
        assert_eq!(err.to_string(), "Value not found for key: secret123layer");
    }

    #[test]
    fn test_error_equality() {
        let err1 = SecretChatDbError::NotFound("key".to_string());
        let err2 = SecretChatDbError::NotFound("key".to_string());
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_serialization_error() {
        let err = SecretChatDbError::SerializationError("invalid data".to_string());
        assert!(err.to_string().contains("Serialization error"));
    }

    #[test]
    fn test_deserialization_error() {
        let err = SecretChatDbError::DeserializationError("corrupt data".to_string());
        assert!(err.to_string().contains("Deserialization error"));
    }

    #[test]
    fn test_storage_error() {
        let err = SecretChatDbError::StorageError("disk full".to_string());
        assert!(err.to_string().contains("Storage error"));
    }
}
