// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Venue objects representing places with geographic locations.
//!
//! This module implements the `Venue` type, which represents a place/venue
//! with an associated geographic location. It corresponds to TDLib's
//! `td::Venue` class from `Venue.h:24-101`.

use crate::error::Result;
use crate::location::Location;
use crate::validation::clean_input_string;

/// A venue (place) with geographic location and metadata.
///
/// Venues represent places like restaurants, museums, parks, etc. that can
/// be attached to messages in Telegram.
///
/// Corresponds to TDLib type `td::Venue` from `Venue.h:24-101`.
///
/// # TL Correspondence
///
/// | Rust | TD API | Telegram API |
/// |------|--------|--------------|
/// | `Venue` | `venue` | `inputMediaVenue` |
///
/// # Supported Providers
///
/// TDLib supports the following venue providers (`Venue.cpp:43`):
/// - `"foursquare"` - Foursquare venue database
/// - `"gplaces"` - Google Places
///
/// # Examples
///
/// ```
/// use rustgram_venue::{Location, Venue};
///
/// let kremlin = Venue::new(
///     Location::from_components(55.7558, 37.6173, 10.0, 0),
///     "Кремль".to_string(),
///     "Moscow, Russia".to_string(),
///     "foursquare".to_string(),
///     "4a9406f0f964a5209f7d1fe3".to_string(),
///     "Monument".to_string(),
/// );
///
/// assert!(!kremlin.empty());
/// assert_eq!(kremlin.title(), "Кремль");
/// assert_eq!(kremlin.provider(), "foursquare");
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Venue {
    location: Location,
    title: String,
    address: String,
    provider: String,
    id: String,
    venue_type: String,
}

impl Venue {
    /// Creates a new venue from all components.
    ///
    /// This constructor does not validate the input strings. For validated
    /// construction, use [`Self::validate_and_create`].
    ///
    /// # Arguments
    ///
    /// * `location` - Geographic location of the venue
    /// * `title` - Name of the venue
    /// * `address` - Address of the venue
    /// * `provider` - Venue provider (e.g., "foursquare", "gplaces")
    /// * `id` - Venue ID in the provider's database
    /// * `venue_type` - Type of venue (e.g., "restaurant", "park")
    ///
    /// TDLib reference: `Venue.cpp:24-30`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::{Location, Venue};
    ///
    /// let venue = Venue::new(
    ///     Location::from_components(55.7558, 37.6173, 10.0, 0),
    ///     "Cafe Pushkin".to_string(),
    ///     "Pushkin Square, Moscow".to_string(),
    ///     "foursquare".to_string(),
    ///     "4b5f1faf8f7766efd8e60cb7".to_string(),
    ///     "Restaurant".to_string(),
    /// );
    /// ```
    pub fn new(
        location: Location,
        title: String,
        address: String,
        provider: String,
        id: String,
        venue_type: String,
    ) -> Self {
        Self {
            location,
            title,
            address,
            provider,
            id,
            venue_type,
        }
    }

    /// Creates a venue after validating all string fields.
    ///
    /// This method validates all input strings using [`clean_input_string`]
    /// and ensures the location is not empty.
    ///
    /// # Arguments
    ///
    /// * `location` - Geographic location (must not be empty)
    /// * `title` - Name of the venue (will be validated)
    /// * `address` - Address of the venue (will be validated)
    /// * `provider` - Venue provider (will be validated)
    /// * `id` - Venue ID (will be validated)
    /// * `venue_type` - Type of venue (will be validated)
    ///
    /// # Returns
    ///
    /// Returns `Ok(Venue)` if all validations pass, or `Err(VenueError)` if:
    /// - Location is empty
    /// - Any string field is invalid UTF-8
    ///
    /// TDLib reference: `Venue.cpp:100-130`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::{Location, Venue};
    ///
    /// let loc = Location::from_components(55.7558, 37.6173, 10.0, 0);
    ///
    /// // Valid venue
    /// let venue = Venue::validate_and_create(
    ///     loc,
    ///     "Cafe Pushkin",
    ///     "Pushkin Square",
    ///     "foursquare",
    ///     "abc123",
    ///     "restaurant",
    /// );
    /// assert!(venue.is_ok());
    ///
    /// // Empty location
    /// let venue = Venue::validate_and_create(
    ///     Location::empty(),
    ///     "Title",
    ///     "Address",
    ///     "foursquare",
    ///     "id",
    ///     "type",
    /// );
    /// assert!(venue.is_err());
    /// ```
    pub fn validate_and_create(
        location: Location,
        title: &str,
        address: &str,
        provider: &str,
        id: &str,
        venue_type: &str,
    ) -> Result<Self> {
        // Venue.cpp:104-106
        if location.is_empty() {
            return Err(crate::error::VenueError::InvalidLocation(
                "Venue must be non-empty".to_string(),
            ));
        }

        // Venue.cpp:108-122 - Validate all string fields
        let title =
            clean_input_string(title).map_err(|_| crate::error::VenueError::InvalidTitle)?;
        let address =
            clean_input_string(address).map_err(|_| crate::error::VenueError::InvalidAddress)?;
        let provider =
            clean_input_string(provider).map_err(|_| crate::error::VenueError::InvalidProvider)?;
        let venue_id = clean_input_string(id).map_err(|_| crate::error::VenueError::InvalidId)?;
        let venue_type =
            clean_input_string(venue_type).map_err(|_| crate::error::VenueError::InvalidType)?;

        // Venue.cpp:124-127
        if location.is_empty() {
            return Err(crate::error::VenueError::InvalidLocation(
                "Wrong venue location specified".to_string(),
            ));
        }

        Ok(Self {
            location,
            title,
            address,
            provider,
            id: venue_id,
            venue_type,
        })
    }

    /// Returns true if this venue is empty.
    ///
    /// A venue is empty if its location is empty.
    ///
    /// TDLib reference: `Venue.cpp:42-44`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::{Location, Venue};
    ///
    /// let venue = Venue::new(
    ///     Location::from_components(55.0, 37.0, 0.0, 0),
    ///     "Title".to_string(),
    ///     "Address".to_string(),
    ///     "foursquare".to_string(),
    ///     "id".to_string(),
    ///     "type".to_string(),
    /// );
    /// assert!(!venue.empty());
    ///
    /// let empty_venue = Venue::new(
    ///     Location::empty(),
    ///     "Title".to_string(),
    ///     "Address".to_string(),
    ///     "foursquare".to_string(),
    ///     "id".to_string(),
    ///     "type".to_string(),
    /// );
    /// assert!(empty_venue.empty());
    /// ```
    pub fn empty(&self) -> bool {
        self.location.is_empty()
    }

    /// Checks if this venue has the same provider and ID.
    ///
    /// This is useful for comparing venues from the same provider database.
    ///
    /// TDLib reference: `Venue.h:48-50`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::{Location, Venue};
    ///
    /// let venue = Venue::new(
    ///     Location::from_components(55.0, 37.0, 0.0, 0),
    ///     "Title".to_string(),
    ///     "Address".to_string(),
    ///     "foursquare".to_string(),
    ///     "abc123".to_string(),
    ///     "type".to_string(),
    /// );
    ///
    /// assert!(venue.is_same_provider_id("foursquare", "abc123"));
    /// assert!(!venue.is_same_provider_id("gplaces", "abc123"));
    /// assert!(!venue.is_same_provider_id("foursquare", "xyz789"));
    /// ```
    pub fn is_same_provider_id(&self, provider: &str, id: &str) -> bool {
        self.provider == provider && self.id == id
    }

    /// Returns a reference to the venue's location.
    ///
    /// TDLib reference: `Venue.h:52-54`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::{Location, Venue};
    ///
    /// let loc = Location::from_components(55.7558, 37.6173, 10.0, 0);
    /// let venue = Venue::new(
    ///     loc.clone(),
    ///     "Title".to_string(),
    ///     "Address".to_string(),
    ///     "foursquare".to_string(),
    ///     "id".to_string(),
    ///     "type".to_string(),
    /// );
    ///
    /// assert_eq!(venue.location().latitude(), 55.7558);
    /// ```
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// Returns a mutable reference to the venue's location.
    pub fn location_mut(&mut self) -> &mut Location {
        &mut self.location
    }

    /// Returns the venue's title.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::{Location, Venue};
    ///
    /// let venue = Venue::new(
    ///     Location::from_components(55.0, 37.0, 0.0, 0),
    ///     "Cafe Pushkin".to_string(),
    ///     "Address".to_string(),
    ///     "foursquare".to_string(),
    ///     "id".to_string(),
    ///     "type".to_string(),
    /// );
    ///
    /// assert_eq!(venue.title(), "Cafe Pushkin");
    /// ```
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the venue's address.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::{Location, Venue};
    ///
    /// let venue = Venue::new(
    ///     Location::from_components(55.0, 37.0, 0.0, 0),
    ///     "Title".to_string(),
    ///     "Pushkin Square, Moscow".to_string(),
    ///     "foursquare".to_string(),
    ///     "id".to_string(),
    ///     "type".to_string(),
    /// );
    ///
    /// assert_eq!(venue.address(), "Pushkin Square, Moscow");
    /// ```
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Returns the venue's provider.
    ///
    /// Common values: "foursquare", "gplaces"
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::{Location, Venue};
    ///
    /// let venue = Venue::new(
    ///     Location::from_components(55.0, 37.0, 0.0, 0),
    ///     "Title".to_string(),
    ///     "Address".to_string(),
    ///     "foursquare".to_string(),
    ///     "id".to_string(),
    ///     "type".to_string(),
    /// );
    ///
    /// assert_eq!(venue.provider(), "foursquare");
    /// ```
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Returns the venue's ID.
    ///
    /// The ID is unique within the provider's database.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::{Location, Venue};
    ///
    /// let venue = Venue::new(
    ///     Location::from_components(55.0, 37.0, 0.0, 0),
    ///     "Title".to_string(),
    ///     "Address".to_string(),
    ///     "foursquare".to_string(),
    ///     "4b5f1faf8f7766efd8e60cb7".to_string(),
    ///     "type".to_string(),
    /// );
    ///
    /// assert_eq!(venue.id(), "4b5f1faf8f7766efd8e60cb7");
    /// ```
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the venue's type.
    ///
    /// Examples: "restaurant", "park", "museum", etc.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::{Location, Venue};
    ///
    /// let venue = Venue::new(
    ///     Location::from_components(55.0, 37.0, 0.0, 0),
    ///     "Title".to_string(),
    ///     "Address".to_string(),
    ///     "foursquare".to_string(),
    ///     "id".to_string(),
    ///     "Restaurant".to_string(),
    /// );
    ///
    /// assert_eq!(venue.venue_type(), "Restaurant");
    /// ```
    pub fn venue_type(&self) -> &str {
        &self.venue_type
    }

    /// Converts this venue to TD API format.
    ///
    /// TDLib reference: `Venue.cpp:54-56`
    pub fn to_td_venue(&self) -> Option<TdVenue> {
        if self.empty() {
            return None;
        }

        self.location.to_td_location().map(|loc| TdVenue {
            location: loc,
            title: self.title.clone(),
            address: self.address.clone(),
            provider: self.provider.clone(),
            id: self.id.clone(),
            type_: self.venue_type.clone(),
        })
    }
}

/// TD API venue format.
///
/// Corresponds to `td_api.tl:546`:
/// ```text
/// venue location:location title:string address:string provider:string
///     id:string type:string = Venue;
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TdVenue {
    /// Geographic location.
    pub location: crate::location::TdLocation,
    /// Venue name.
    pub title: String,
    /// Venue address.
    pub address: String,
    /// Venue provider (e.g., "foursquare").
    pub provider: String,
    /// Venue ID in provider's database.
    pub id: String,
    /// Venue type.
    pub type_: String,
}

/// InputMediaVenue for Telegram API.
///
/// Corresponds to `telegram_api.tl`:
/// ```text
/// inputMediaVenue#c13d1c11 geo_point:InputGeoPoint title:string address:string
///     provider:string venue_id:string venue_type:string = InputMedia;
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InputMediaVenue {
    /// Geographic coordinates.
    pub geo_point: crate::location::InputGeoPoint,
    /// Venue name.
    pub title: String,
    /// Venue address.
    pub address: String,
    /// Venue provider.
    pub provider: String,
    /// Venue ID in provider's database.
    pub venue_id: String,
    /// Venue type.
    pub venue_type: String,
}

impl From<&Venue> for InputMediaVenue {
    fn from(venue: &Venue) -> Self {
        Self {
            geo_point: (&venue.location).into(),
            title: venue.title.clone(),
            address: venue.address.clone(),
            provider: venue.provider.clone(),
            venue_id: venue.id.clone(),
            venue_type: venue.venue_type.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_location() -> Location {
        Location::from_components(55.7558, 37.6173, 10.0, 0)
    }

    #[test]
    fn test_valid_venue() {
        let loc = make_test_location();
        let venue = Venue::new(
            loc.clone(),
            "Кремль".to_string(),
            "Moscow, Russia".to_string(),
            "foursquare".to_string(),
            "4a9406f0f964a5209f7d1fe3".to_string(),
            "Monument".to_string(),
        );

        assert!(!venue.empty());
        assert_eq!(venue.title(), "Кремль");
        assert_eq!(venue.address(), "Moscow, Russia");
        assert_eq!(venue.provider(), "foursquare");
        assert_eq!(venue.id(), "4a9406f0f964a5209f7d1fe3");
        assert_eq!(venue.venue_type(), "Monument");
    }

    #[test]
    fn test_empty_venue() {
        let venue = Venue::default();
        assert!(venue.empty());
        assert_eq!(venue.title(), "");
        assert_eq!(venue.address(), "");
    }

    #[test]
    fn test_venue_with_empty_location() {
        let venue = Venue::new(
            Location::empty(),
            "Test".to_string(),
            "Address".to_string(),
            "foursquare".to_string(),
            "id123".to_string(),
            "type".to_string(),
        );

        assert!(venue.empty());
    }

    #[test]
    fn test_is_same_provider_id() {
        let loc = make_test_location();
        let venue = Venue::new(
            loc,
            "Title".to_string(),
            "Address".to_string(),
            "foursquare".to_string(),
            "abc123".to_string(),
            "type".to_string(),
        );

        assert!(venue.is_same_provider_id("foursquare", "abc123"));
        assert!(!venue.is_same_provider_id("gplaces", "abc123"));
        assert!(!venue.is_same_provider_id("foursquare", "xyz789"));
        assert!(!venue.is_same_provider_id("gplaces", "xyz789"));
    }

    #[test]
    fn test_venue_validation() {
        let loc = make_test_location();

        // Valid venue
        let result = Venue::validate_and_create(
            loc.clone(),
            "Valid Title",
            "Valid Address",
            "foursquare",
            "valid_id",
            "cafe",
        );
        assert!(result.is_ok());
        match result {
            Ok(venue) => assert_eq!(venue.title(), "Valid Title"),
            Err(e) => panic!("validate_and_create failed: {:?}", e),
        }

        // Invalid UTF-8 in title - since our function takes &str,
        // we can't pass invalid UTF-8 directly.
        // The simdutf8 check in clean_input_string validates this,
        // but in Rust, &str is guaranteed valid UTF-8.
        // So this test just validates that valid UTF-8 works
        let result = Venue::validate_and_create(
            loc.clone(),
            "Valid Title",
            "Address",
            "foursquare",
            "id",
            "type",
        );
        assert!(result.is_ok());

        // Empty location
        let result = Venue::validate_and_create(
            Location::empty(),
            "Title",
            "Address",
            "foursquare",
            "id",
            "type",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_venue_validation_cleans_strings() {
        let loc = make_test_location();

        // Strings with control characters should be cleaned
        let result = Venue::validate_and_create(
            loc,
            "Title\x00with\x01control",
            "Address\r\nwith\rcarriage",
            "foursquare",
            "id",
            "type",
        );
        assert!(result.is_ok());
        match result {
            Ok(venue) => {
                // Control chars 0-8, 11-31 become spaces, \r is removed
                assert_eq!(venue.title(), "Title with control");
                // \r is removed, \n preserved (falls to default)
                assert_eq!(venue.address(), "Address\nwithcarriage");
            }
            Err(e) => panic!("validate_and_create failed: {:?}", e),
        }
    }

    #[test]
    fn test_venue_equality() {
        let loc = make_test_location();
        let venue1 = Venue::new(
            loc.clone(),
            "Title".to_string(),
            "Address".to_string(),
            "foursquare".to_string(),
            "id".to_string(),
            "type".to_string(),
        );
        let venue2 = Venue::new(
            loc,
            "Title".to_string(),
            "Address".to_string(),
            "foursquare".to_string(),
            "id".to_string(),
            "type".to_string(),
        );
        assert_eq!(venue1, venue2);

        // Different title
        let venue3 = Venue::new(
            make_test_location(),
            "Other".to_string(),
            "Address".to_string(),
            "foursquare".to_string(),
            "id".to_string(),
            "type".to_string(),
        );
        assert_ne!(venue1, venue3);
    }

    #[test]
    fn test_to_td_venue() {
        let loc = make_test_location();
        let venue = Venue::new(
            loc.clone(),
            "Test Venue".to_string(),
            "Test Address".to_string(),
            "foursquare".to_string(),
            "test_id".to_string(),
            "restaurant".to_string(),
        );

        let td_venue = venue.to_td_venue();
        assert!(td_venue.is_some());
        match td_venue {
            Some(td_venue) => {
                assert_eq!(td_venue.title, "Test Venue");
                assert_eq!(td_venue.address, "Test Address");
                assert_eq!(td_venue.provider, "foursquare");
                assert_eq!(td_venue.id, "test_id");
                assert_eq!(td_venue.type_, "restaurant");
                assert_eq!(td_venue.location.latitude, 55.7558);
            }
            None => panic!("to_td_venue returned None for valid venue"),
        }

        // Empty venue returns None
        let empty_venue = Venue::default();
        assert!(empty_venue.to_td_venue().is_none());
    }

    #[test]
    fn test_to_input_media_venue() {
        let loc = make_test_location();
        let venue = Venue::new(
            loc.clone(),
            "Test Venue".to_string(),
            "Test Address".to_string(),
            "foursquare".to_string(),
            "test_id".to_string(),
            "restaurant".to_string(),
        );

        let input_media: InputMediaVenue = (&venue).into();
        assert_eq!(input_media.title, "Test Venue");
        assert_eq!(input_media.address, "Test Address");
        assert_eq!(input_media.provider, "foursquare");
        assert_eq!(input_media.venue_id, "test_id");
        assert_eq!(input_media.venue_type, "restaurant");

        match input_media.geo_point {
            crate::location::InputGeoPoint::Data { lat, long, .. } => {
                assert_eq!(lat, 55.7558);
                assert_eq!(long, 37.6173);
            }
            _ => panic!("Expected Data"),
        }
    }

    #[test]
    fn test_default() {
        let venue = Venue::default();
        assert!(venue.empty());
        assert_eq!(venue.title(), "");
        assert_eq!(venue.address(), "");
        assert_eq!(venue.provider(), "");
        assert_eq!(venue.id(), "");
        assert_eq!(venue.venue_type(), "");
    }
}

#[cfg(all(test, feature = "proptest"))]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    fn make_test_location() -> Location {
        Location::from_components(55.7558, 37.6173, 10.0, 0)
    }

    proptest! {
        #[test]
        fn test_venue_roundtrip(
            title in "[a-zA-Z0-9 ]{1,100}",
            address in "[a-zA-Z0-9 ]{1,100}",
            provider in "foursquare|gplaces",
            id in "[a-z0-9]{1,50}",
            venue_type in "[a-z]{1,30}"
        ) {
            let loc = make_test_location();
            let venue = Venue::new(
                loc.clone(),
                title.clone(),
                address.clone(),
                provider.to_string(),
                id.clone(),
                venue_type.to_string(),
            );

            prop_assert_eq!(venue.title(), title);
            prop_assert_eq!(venue.address(), address);
            prop_assert_eq!(venue.provider(), provider);
            prop_assert_eq!(venue.id(), id);
            prop_assert_eq!(venue.venue_type(), venue_type);
        }

        #[test]
        fn test_venue_with_cleaned_strings(
            title in "[a-zA-Z0-9\x00\x01\x02]{1,100}",
            address in "[a-zA-Z0-9\r\n]{1,100}"
        ) {
            let loc = make_test_location();

            // validate_and_create should clean the strings
            let result = Venue::validate_and_create(
                loc,
                &title,
                &address,
                "foursquare",
                "id",
                "type",
            );

            prop_assert!(result.is_ok());
            match result {
                Ok(venue) => {
                    // Cleaned title should not contain control chars (except they become spaces)
                    // Actually, control chars 0-8, 11-31 become spaces
                    // So the title might have spaces instead
                    prop_assert!(!venue.title().contains('\x00'));
                    prop_assert!(!venue.title().contains('\x01'));
                }
                Err(e) => panic!("validate_and_create failed: {:?}", e),
            }
        }
    }
}
