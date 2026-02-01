// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

use std::fmt;

/// Errors that can occur when working with passkeys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasskeyError {
    /// Invalid passkey ID (empty string).
    InvalidId,
    /// Invalid passkey name (empty string).
    InvalidName,
    /// Invalid custom emoji ID (zero).
    InvalidCustomEmojiId(i64),
}

impl fmt::Display for PasskeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidId => write!(f, "passkey ID cannot be empty"),
            Self::InvalidName => write!(f, "passkey name cannot be empty"),
            Self::InvalidCustomEmojiId(id) => {
                write!(f, "custom emoji ID must be non-zero, got {}", id)
            }
        }
    }
}

impl std::error::Error for PasskeyError {}

/// Result type for passkey operations.
pub type Result<T> = std::result::Result<T, PasskeyError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_invalid_id() {
        let err = PasskeyError::InvalidId;
        assert_eq!(format!("{}", err), "passkey ID cannot be empty");
    }

    #[test]
    fn test_display_invalid_name() {
        let err = PasskeyError::InvalidName;
        assert_eq!(format!("{}", err), "passkey name cannot be empty");
    }

    #[test]
    fn test_display_invalid_custom_emoji() {
        let err = PasskeyError::InvalidCustomEmojiId(0);
        assert_eq!(
            format!("{}", err),
            "custom emoji ID must be non-zero, got 0"
        );
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(PasskeyError::InvalidId, PasskeyError::InvalidId);
        assert_eq!(PasskeyError::InvalidName, PasskeyError::InvalidName);
        assert_eq!(
            PasskeyError::InvalidCustomEmojiId(123),
            PasskeyError::InvalidCustomEmojiId(123)
        );
    }
}
