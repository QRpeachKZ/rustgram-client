// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for the language pack manager.

use thiserror::Error;

/// Result type for language pack manager operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in the language pack manager.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum Error {
    /// Invalid language code provided.
    #[error("invalid language code: {0}")]
    InvalidLanguageCode(String),

    /// Invalid language pack name provided.
    #[error("invalid language pack name: {0}")]
    InvalidLanguagePackName(String),

    /// Invalid custom language code.
    #[error("invalid custom language code (must contain '_'): {0}")]
    InvalidCustomLanguageCode(String),

    /// Language pack not found.
    #[error("language pack not found: {0}")]
    LanguagePackNotFound(String),

    /// Language string not found.
    #[error("language string not found: {0}")]
    StringNotFound(String),

    /// Database error.
    #[error("database error: {0}")]
    DatabaseError(String),

    /// Network error occurred.
    #[error("network error: {0}")]
    NetworkError(String),

    /// Serialization error.
    #[error("serialization error: {0}")]
    SerializationError(String),

    /// Language pack too long.
    #[error("language pack too long: {0}")]
    LanguagePackTooLong(String),

    /// String value too long.
    #[error("string value too long: {0} chars (max {1})")]
    StringValueTooLong(String, usize),

    /// Synchronization failed.
    #[error("synchronization failed: {0}")]
    SynchronizationError(String),

    /// Internal error.
    #[error("internal error: {0}")]
    InternalError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::InvalidLanguageCode("invalid!".to_string());
        assert!(err.to_string().contains("invalid language code"));
        assert!(err.to_string().contains("invalid!"));
    }

    #[test]
    fn test_error_eq() {
        let err1 = Error::InvalidLanguageCode("en".to_string());
        let err2 = Error::InvalidLanguageCode("en".to_string());
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_error_language_pack_not_found() {
        let err = Error::LanguagePackNotFound("missing_pack".to_string());
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn test_error_string_value_too_long() {
        let err = Error::StringValueTooLong("test_key".to_string(), 50000);
        assert!(err.to_string().contains("too long"));
        assert!(err.to_string().contains("50000"));
    }

    #[test]
    fn test_error_custom_language_code() {
        let err = Error::InvalidCustomLanguageCode("en".to_string());
        assert!(err.to_string().contains("must contain"));
    }
}
