// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for the config module.

use std::io;
use thiserror::Error;

// Import from rustgram_types for conversion
impl From<rustgram_types::TypeError> for ConfigError {
    fn from(err: rustgram_types::TypeError) -> Self {
        ConfigError::SerializationError(err.to_string())
    }
}

/// Error type for config operations.
///
/// This type encapsulates all possible errors that can occur when
/// working with Telegram configuration.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Invalid configuration value.
    #[error("Invalid config: {0}")]
    InvalidConfig(String),

    /// Network error from query execution.
    #[error("Network error: {0}")]
    NetworkError(String),

    /// I/O error during config storage operations.
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    /// TL serialization/deserialization error.
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Config not found in cache or storage.
    #[error("Config not found: {0}")]
    NotFound(String),

    /// Config has expired.
    #[error("Config expired: {0}")]
    Expired(String),

    /// Generic error with a message.
    #[error("{0}")]
    Other(String),
}

impl ConfigError {
    /// Creates an invalid config error.
    pub fn invalid_config(msg: impl Into<String>) -> Self {
        Self::InvalidConfig(msg.into())
    }

    /// Creates a not found error.
    pub fn not_found(what: impl Into<String>) -> Self {
        Self::NotFound(what.into())
    }

    /// Creates an expired error.
    pub fn expired(what: impl Into<String>) -> Self {
        Self::Expired(what.into())
    }

    /// Creates a generic error.
    pub fn other(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }

    /// Returns `true` if this error indicates the config should be refetched.
    pub fn should_refetch(&self) -> bool {
        matches!(self, Self::NotFound(_) | Self::Expired(_))
    }
}

/// Result type for config operations.
pub type Result<T> = std::result::Result<T, ConfigError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_error_creation() {
        let err = ConfigError::invalid_config("test error");
        assert!(matches!(err, ConfigError::InvalidConfig(_)));
        assert_eq!(err.to_string(), "Invalid config: test error");
    }

    #[test]
    fn test_not_found_error() {
        let err = ConfigError::not_found("app_config");
        assert!(matches!(err, ConfigError::NotFound(_)));
        assert!(err.should_refetch());
    }

    #[test]
    fn test_expired_error() {
        let err = ConfigError::expired("app_config");
        assert!(matches!(err, ConfigError::Expired(_)));
        assert!(err.should_refetch());
    }

    #[test]
    fn test_other_error_not_retryable() {
        let err = ConfigError::Other("test".to_string());
        assert!(!err.should_refetch());
    }
}
