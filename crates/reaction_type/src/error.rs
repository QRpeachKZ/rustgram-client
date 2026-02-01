// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! Error types for reaction type module.

use thiserror::Error;

/// Errors that can occur when working with reaction types.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ReactionError {
    /// Invalid reaction format.
    #[error("invalid reaction format: {0}")]
    InvalidFormat(String),

    /// Invalid base64 encoding in custom emoji.
    #[error("invalid base64 encoding: {0}")]
    InvalidBase64(String),

    /// Empty reaction string.
    #[error("reaction string cannot be empty")]
    EmptyReaction,

    /// Unknown reaction type.
    #[error("unknown reaction type")]
    UnknownType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", ReactionError::EmptyReaction),
            "reaction string cannot be empty"
        );
        assert_eq!(
            format!("{}", ReactionError::UnknownType),
            "unknown reaction type"
        );
    }

    #[test]
    fn test_error_equality() {
        let e1 = ReactionError::EmptyReaction;
        let e2 = ReactionError::EmptyReaction;
        let e3 = ReactionError::UnknownType;

        assert_eq!(e1, e2);
        assert_ne!(e1, e3);
    }
}
