// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for the people_nearby module.

use std::fmt;

/// Errors that can occur during people nearby operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeopleNearbyError {
    /// Invalid distance value (negative or unreasonably large)
    InvalidDistance(i32),

    /// Invalid chat ID
    InvalidChatId(i64),

    /// Empty results when nearby users were expected
    EmptyResults,

    /// Invalid offset string
    InvalidOffset(String),

    /// Location not provided for search
    LocationRequired,

    /// Update received without previous search
    UpdateWithoutSearch,

    /// Serialization error
    SerializationError(String),
}

impl fmt::Display for PeopleNearbyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDistance(dist) => {
                write!(f, "Invalid distance value: {} (must be non-negative)", dist)
            }
            Self::InvalidChatId(id) => {
                write!(f, "Invalid chat ID: {}", id)
            }
            Self::EmptyResults => {
                write!(f, "No nearby users found")
            }
            Self::InvalidOffset(offset) => {
                write!(f, "Invalid offset string: {}", offset)
            }
            Self::LocationRequired => {
                write!(f, "Location is required for searching nearby users")
            }
            Self::UpdateWithoutSearch => {
                write!(f, "Received nearby update without performing a search")
            }
            Self::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
        }
    }
}

impl std::error::Error for PeopleNearbyError {}

/// Result type for people nearby operations.
pub type Result<T> = std::result::Result<T, PeopleNearbyError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        assert_eq!(
            PeopleNearbyError::InvalidDistance(-1).to_string(),
            "Invalid distance value: -1 (must be non-negative)"
        );
        assert_eq!(
            PeopleNearbyError::InvalidChatId(0).to_string(),
            "Invalid chat ID: 0"
        );
        assert_eq!(
            PeopleNearbyError::EmptyResults.to_string(),
            "No nearby users found"
        );
        assert_eq!(
            PeopleNearbyError::LocationRequired.to_string(),
            "Location is required for searching nearby users"
        );
    }

    #[test]
    fn test_error_debug() {
        let err = PeopleNearbyError::InvalidDistance(100);
        assert_eq!(format!("{:?}", err), "InvalidDistance(100)");
    }
}
