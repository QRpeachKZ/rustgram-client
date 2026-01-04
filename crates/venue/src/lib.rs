// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Rustgram Venue - Geographic locations and venue objects for Telegram MTProto client.
//!
//! This module provides types for working with geographic locations and venue objects
//! (places) in the Telegram MTProto protocol.
//!
//! ## Overview
//!
//! The venue module provides two main types:
//!
//! - [`Location`] - Geographic coordinates with optional horizontal accuracy
//! - [`Venue`] - A place (restaurant, museum, etc.) with an associated location
//!
//! ## TDLib Correspondence
//!
//! | Rust type | TDLib type | File |
//! |-----------|------------|------|
//! | [`Location`] | `td::Location` | `Location.h/cpp` |
//! | [`Venue`] | `td::Venue` | `Venue.h/cpp` |
//!
//! ## Examples
//!
//! ### Creating a Location
//!
//! ```
//! use rustgram_venue::Location;
//!
//! // Moscow coordinates
//! let moscow = Location::from_components(55.7558, 37.6173, 10.0, 0);
//! assert!(!moscow.is_empty());
//! assert_eq!(moscow.latitude(), 55.7558);
//! assert_eq!(moscow.longitude(), 37.6173);
//! ```
//!
//! ### Creating a Venue
//!
//! ```
//! use rustgram_venue::{Location, Venue};
//!
//! let kremlin = Venue::new(
//!     Location::from_components(55.7558, 37.6173, 10.0, 0),
//!     "Кремль".to_string(),
//!     "Moscow, Russia".to_string(),
//!     "foursquare".to_string(),
//!     "4a9406f0f964a5209f7d1fe3".to_string(),
//!     "Monument".to_string(),
//! );
//!
//! assert!(!kremlin.empty());
//! assert_eq!(kremlin.title(), "Кремль");
//! assert_eq!(kremlin.provider(), "foursquare");
//! ```
//!
//! ### Validated Venue Creation
//!
//! ```
//! use rustgram_venue::{Location, Venue};
//!
//! let loc = Location::from_components(55.7558, 37.6173, 10.0, 0);
//!
//! // Validates and cleans all string fields
//! let venue = Venue::validate_and_create(
//!     loc,
//!     "Cafe Pushkin",
//!     "Pushkin Square, Moscow",
//!     "foursquare",
//!     "4b5f1faf8f7766efd8e60cb7",
//!     "Restaurant",
//! );
//!
//! assert!(venue.is_ok());
//! ```
//!
//! ## Validation
//!
//! ### Location Validation
//!
//! - **Latitude**: [-90, 90] degrees
//! - **Longitude**: [-180, 180] degrees
//! - **Horizontal accuracy**: [0, 1500] meters (automatically clamped)
//!
//! Invalid coordinates result in an empty location (via [`Location::empty()`]).
//!
//! ### Venue Validation
//!
//! Venue string fields are validated using [`validate_and_create`]:
//! - UTF-8 encoding is verified
//! - Control characters are removed/replaced
//! - Strings are trimmed
//! - Maximum length is 35000 characters
//!
//! ## TL Correspondence
//!
//! ### Telegram API
//!
//! ```text
//! geoPointEmpty#1117dd5f = GeoPoint;
//! geoPoint#b2a2f663 flags:# long:double lat:double access_hash:long
//!     accuracy_radius:flags.0?int = GeoPoint;
//!
//! inputMediaVenue#c13d1c11 geo_point:InputGeoPoint title:string address:string
//!     provider:string venue_id:string venue_type:string = InputMedia;
//! ```
//!
//! ### TD API
//!
//! ```text
//! location latitude:double longitude:double horizontal_accuracy:double = Location;
//!
//! venue location:location title:string address:string provider:string
//!     id:string type:string = Venue;
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::let_and_return)]

pub mod error;
pub mod location;
pub mod validation;
pub mod venue;

// Re-export public API
pub use error::{Result, VenueError};
pub use location::{
    InputGeoPoint, Location, TdLocation, MAX_HORIZONTAL_ACCURACY, MAX_VALID_MAP_LATITUDE,
};
pub use validation::{clean_input_string, MAX_STRING_LENGTH};
pub use venue::{InputMediaVenue, TdVenue, Venue};

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-venue";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-venue");
    }

    #[test]
    fn test_constants() {
        assert_eq!(MAX_VALID_MAP_LATITUDE, 85.05112877);
        assert_eq!(MAX_HORIZONTAL_ACCURACY, 1500.0);
        assert_eq!(MAX_STRING_LENGTH, 35000);
    }

    #[test]
    fn test_location_empty() {
        let loc = Location::empty();
        assert!(loc.is_empty());
    }

    #[test]
    fn test_venue_empty() {
        let venue = Venue::default();
        assert!(venue.empty());
    }

    #[test]
    fn test_location_valid() {
        let loc = Location::from_components(55.7558, 37.6173, 10.0, 0);
        assert!(!loc.is_empty());
        assert_eq!(loc.latitude(), 55.7558);
        assert_eq!(loc.longitude(), 37.6173);
    }

    #[test]
    fn test_venue_valid() {
        let loc = Location::from_components(55.7558, 37.6173, 10.0, 0);
        let venue = Venue::new(
            loc,
            "Title".to_string(),
            "Address".to_string(),
            "foursquare".to_string(),
            "id".to_string(),
            "type".to_string(),
        );
        assert!(!venue.empty());
        assert_eq!(venue.title(), "Title");
    }

    #[test]
    fn test_clean_string() {
        match clean_input_string("Hello, world!") {
            Ok(result) => assert_eq!(result, "Hello, world!"),
            Err(e) => panic!("clean_input_string failed: {:?}", e),
        }
    }
}
