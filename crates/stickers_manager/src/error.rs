// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for StickersManager.

use std::fmt;

/// Result type alias for StickersManager operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur in StickersManager operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Sticker set not found.
    StickerSetNotFound,
    /// Invalid sticker ID.
    InvalidStickerId,
    /// Invalid sticker format.
    InvalidStickerFormat,
    /// Network error.
    NetworkError(String),
    /// Rate limited.
    RateLimited,
    /// Internal error.
    Internal(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StickerSetNotFound => write!(f, "Sticker set not found"),
            Self::InvalidStickerId => write!(f, "Invalid sticker ID"),
            Self::InvalidStickerFormat => write!(f, "Invalid sticker format"),
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::RateLimited => write!(f, "Rate limited"),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Result of checking a sticker set name for validity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckStickerSetNameResult {
    /// Name is valid and available.
    Ok,
    /// Name format is invalid.
    Invalid,
    /// Name is already taken.
    Occupied,
    /// Name is too long.
    TooLong,
    /// Name contains invalid characters.
    InvalidCharacters,
}

impl CheckStickerSetNameResult {
    /// Returns `true` if the name is OK.
    #[must_use]
    pub const fn is_ok(self) -> bool {
        matches!(self, Self::Ok)
    }
}

impl fmt::Display for CheckStickerSetNameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok => write!(f, "OK"),
            Self::Invalid => write!(f, "Invalid format"),
            Self::Occupied => write!(f, "Name already taken"),
            Self::TooLong => write!(f, "Name too long"),
            Self::InvalidCharacters => write!(f, "Contains invalid characters"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", Error::StickerSetNotFound),
            "Sticker set not found"
        );
        assert_eq!(format!("{}", Error::InvalidStickerId), "Invalid sticker ID");
    }

    #[test]
    fn test_check_result_is_ok() {
        assert!(CheckStickerSetNameResult::Ok.is_ok());
        assert!(!CheckStickerSetNameResult::Invalid.is_ok());
    }
}
