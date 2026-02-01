// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Error types for the venue module.

use thiserror::Error;

/// Errors that can occur when working with venues and locations.
///
/// These errors correspond to the TDLib error handling in
/// `Venue.cpp:100-130` and `Location.cpp`.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum VenueError {
    /// Venue location is empty or invalid.
    ///
    /// Corresponds to TDLib error: `"Wrong venue location specified"`
    #[error("Invalid venue location: {0}")]
    InvalidLocation(String),

    /// Venue title is not valid UTF-8 or too long.
    ///
    /// Corresponds to TDLib error: `"Venue title must be encoded in UTF-8"`
    #[error("Invalid venue title: must be valid UTF-8")]
    InvalidTitle,

    /// Venue address is not valid UTF-8 or too long.
    ///
    /// Corresponds to TDLib error: `"Venue address must be encoded in UTF-8"`
    #[error("Invalid venue address: must be valid UTF-8")]
    InvalidAddress,

    /// Venue provider is not valid UTF-8 or too long.
    ///
    /// Corresponds to TDLib error: `"Venue provider must be encoded in UTF-8"`
    #[error("Invalid venue provider: must be valid UTF-8")]
    InvalidProvider,

    /// Venue ID is not valid UTF-8 or too long.
    ///
    /// Corresponds to TDLib error: `"Venue identifier must be encoded in UTF-8"`
    #[error("Invalid venue identifier: must be valid UTF-8")]
    InvalidId,

    /// Venue type is not valid UTF-8 or too long.
    ///
    /// Corresponds to TDLib error: `"Venue type must be encoded in UTF-8"`
    #[error("Invalid venue type: must be valid UTF-8")]
    InvalidType,

    /// Venue must be non-empty.
    ///
    /// Corresponds to TDLib error: `"Venue must be non-empty"`
    #[error("Venue must be non-empty")]
    EmptyVenue,

    /// Latitude out of range.
    ///
    /// Valid range is [-90, 90].
    #[error("Latitude must be between -90 and 90, got: {0}")]
    LatitudeOutOfRange(f64),

    /// Longitude out of range.
    ///
    /// Valid range is [-180, 180].
    #[error("Longitude must be between -180 and 180, got: {0}")]
    LongitudeOutOfRange(f64),

    /// Horizontal accuracy out of range.
    ///
    /// Valid range is [0, 1500] meters.
    #[error("Horizontal accuracy must be between 0 and 1500, got: {0}")]
    AccuracyOutOfRange(f64),

    /// String exceeds maximum length.
    #[error("String exceeds maximum length of {max} characters")]
    StringTooLong {
        /// Maximum allowed length.
        max: usize,
    },

    /// Invalid UTF-8 encoding.
    #[error("Invalid UTF-8 encoding")]
    InvalidUtf8,
}

/// Result type for venue operations.
pub type Result<T> = std::result::Result<T, VenueError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        assert_eq!(
            VenueError::InvalidLocation("test".to_string()).to_string(),
            "Invalid venue location: test"
        );
        assert_eq!(
            VenueError::InvalidTitle.to_string(),
            "Invalid venue title: must be valid UTF-8"
        );
        assert_eq!(
            VenueError::LatitudeOutOfRange(91.0).to_string(),
            "Latitude must be between -90 and 90, got: 91"
        );
    }

    #[test]
    fn test_error_equality() {
        assert_eq!(VenueError::InvalidTitle, VenueError::InvalidTitle);
        assert_ne!(
            VenueError::LatitudeOutOfRange(90.0),
            VenueError::LatitudeOutOfRange(91.0)
        );
    }
}
