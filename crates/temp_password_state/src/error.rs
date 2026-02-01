// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under the MIT OR Apache-2.0 license

//! Error types for temp_password_state module.

use std::fmt;

/// Errors that can occur during temp password operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TempPasswordError {
    /// Temporary password has expired
    Expired,

    /// No temporary password is set
    NotSet,

    /// Invalid timestamp value
    InvalidTimestamp(i32),

    /// Serialization error
    SerializationError(String),
}

impl fmt::Display for TempPasswordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Expired => write!(f, "Temporary password has expired"),
            Self::NotSet => write!(f, "No temporary password is set"),
            Self::InvalidTimestamp(ts) => write!(f, "Invalid timestamp: {}", ts),
            Self::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for TempPasswordError {}

/// Result type for temp password operations.
pub type Result<T> = std::result::Result<T, TempPasswordError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            TempPasswordError::Expired.to_string(),
            "Temporary password has expired"
        );
        assert_eq!(
            TempPasswordError::NotSet.to_string(),
            "No temporary password is set"
        );
        assert_eq!(
            TempPasswordError::InvalidTimestamp(-1).to_string(),
            "Invalid timestamp: -1"
        );
        assert_eq!(
            TempPasswordError::SerializationError("test error".to_string()).to_string(),
            "Serialization error: test error"
        );
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(TempPasswordError::Expired, TempPasswordError::Expired);
        assert_eq!(TempPasswordError::NotSet, TempPasswordError::NotSet);
    }
}
