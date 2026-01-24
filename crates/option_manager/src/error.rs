// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for option manager.

use thiserror::Error;

/// Result type for option manager operations.
pub type Result<T> = std::result::Result<T, OptionManagerError>;

/// Errors that can occur in the option manager.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum OptionManagerError {
    /// Option not found.
    #[error("Option not found: {0}")]
    OptionNotFound(String),

    /// Invalid option value.
    #[error("Invalid option value for option `{name}`: expected {expected}, got {got}")]
    InvalidValue {
        /// Option name
        name: String,
        /// Expected type
        expected: String,
        /// Actual type
        got: String,
    },

    /// Maximum number of options exceeded.
    #[error("Maximum number of options exceeded: {0}")]
    MaxOptionsExceeded(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_not_found() {
        let err = OptionManagerError::OptionNotFound("test_option".to_string());
        assert_eq!(err.to_string(), "Option not found: test_option");
    }

    #[test]
    fn test_invalid_value() {
        let err = OptionManagerError::InvalidValue {
            name: "test".to_string(),
            expected: "integer".to_string(),
            got: "string".to_string(),
        };
        assert!(err.to_string().contains("test"));
        assert!(err.to_string().contains("integer"));
        assert!(err.to_string().contains("string"));
    }

    #[test]
    fn test_max_options_exceeded() {
        let err = OptionManagerError::MaxOptionsExceeded(10000);
        assert_eq!(err.to_string(), "Maximum number of options exceeded: 10000");
    }
}
