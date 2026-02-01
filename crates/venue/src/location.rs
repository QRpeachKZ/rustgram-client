// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Geographic location with coordinates and accuracy.
//!
//! This module implements the `Location` type, which represents a geographic
//! location with optional horizontal accuracy. It corresponds to TDLib's
//! `td::Location` class from `Location.h:24-120`.

/// Maximum valid latitude for map display.
///
/// This is the Mercator projection limit, approximately 85.05 degrees.
/// Locations with higher latitude are valid coordinates but cannot be
/// displayed on standard web maps.
///
/// TDLib reference: `Location.cpp:79`
pub const MAX_VALID_MAP_LATITUDE: f64 = 85.05112877;

/// Maximum horizontal accuracy in meters.
///
/// Server-side limit for location accuracy. Values greater than this
/// will be clamped to this maximum.
///
/// TDLib reference: `Location.cpp:17-25`
pub const MAX_HORIZONTAL_ACCURACY: f64 = 1500.0;

/// Geographic location with coordinates and optional accuracy.
///
/// Corresponds to TDLib type `td::Location` from `Location.h:24-120`.
///
/// # TL Correspondence
///
/// | Rust | Telegram API | TD API |
/// |------|--------------|--------|
/// | `Location::empty()` | `geoPointEmpty` | - |
/// | `Location` | `geoPoint` | `location` |
///
/// # Validation
///
/// - Latitude: `[-90, 90]`
/// - Longitude: `[-180, 180]`
/// - Horizontal accuracy: `[0, 1500]` meters
///
/// # Examples
///
/// ```
/// use rustgram_venue::Location;
///
/// // Create location from coordinates
/// let moscow = Location::from_components(55.7558, 37.6173, 10.0, 0);
/// assert!(!moscow.is_empty());
/// assert_eq!(moscow.latitude(), 55.7558);
/// assert_eq!(moscow.longitude(), 37.6173);
///
/// // Invalid coordinates return empty location
/// let invalid = Location::from_components(91.0, 0.0, 0.0, 0);
/// assert!(invalid.is_empty());
///
/// // Accuracy is clamped to [0, 1500]
/// let loc = Location::from_components(0.0, 0.0, 2000.0, 0);
/// assert_eq!(loc.horizontal_accuracy(), 1500.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    is_empty: bool,
    latitude: f64,
    longitude: f64,
    horizontal_accuracy: f64,
    access_hash: i64,
}

impl Location {
    /// Creates an empty location.
    ///
    /// Empty locations represent the absence of location data.
    /// They are used as a default state and when invalid coordinates
    /// are provided to [`Self::from_components`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::Location;
    ///
    /// let loc = Location::empty();
    /// assert!(loc.is_empty());
    /// ```
    pub const fn empty() -> Self {
        Self {
            is_empty: true,
            latitude: 0.0,
            longitude: 0.0,
            horizontal_accuracy: 0.0,
            access_hash: 0,
        }
    }

    /// Creates a location from coordinate components.
    ///
    /// This method validates the input coordinates and returns an empty
    /// location if validation fails. Accuracy is automatically clamped
    /// to the range `[0, 1500]` meters.
    ///
    /// # Arguments
    ///
    /// * `latitude` - Latitude in degrees, must be in [-90, 90]
    /// * `longitude` - Longitude in degrees, must be in [-180, 180]
    /// * `horizontal_accuracy` - Horizontal accuracy in meters, will be clamped to [0, 1500]
    /// * `access_hash` - MTProto access hash for the location
    ///
    /// # Returns
    ///
    /// Returns a valid location if coordinates are within range,
    /// otherwise returns an empty location.
    ///
    /// TDLib reference: `Location.cpp:27-38`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::Location;
    ///
    /// // Valid coordinates
    /// let loc = Location::from_components(55.7558, 37.6173, 10.0, 12345);
    /// assert!(!loc.is_empty());
    /// assert_eq!(loc.latitude(), 55.7558);
    /// assert_eq!(loc.access_hash(), 12345);
    ///
    /// // Invalid latitude
    /// let invalid = Location::from_components(91.0, 0.0, 0.0, 0);
    /// assert!(invalid.is_empty());
    /// ```
    pub fn from_components(
        latitude: f64,
        longitude: f64,
        horizontal_accuracy: f64,
        access_hash: i64,
    ) -> Self {
        // TDLib Location.cpp:28
        if !latitude.is_finite() || !longitude.is_finite() {
            return Self::empty();
        }

        // TDLib Location.cpp:28
        if latitude.abs() > 90.0 || longitude.abs() > 180.0 {
            return Self::empty();
        }

        Self {
            is_empty: false,
            latitude,
            longitude,
            horizontal_accuracy: Self::fix_accuracy(horizontal_accuracy),
            access_hash,
        }
    }

    /// Creates a location from TD API format.
    ///
    /// TD API locations don't include access_hash, so it's set to 0.
    ///
    /// TDLib reference: `Location.cpp:66-72`
    pub fn from_td_location(latitude: f64, longitude: f64, horizontal_accuracy: f64) -> Self {
        Self::from_components(latitude, longitude, horizontal_accuracy, 0)
    }

    /// Fixes accuracy to be within valid range.
    ///
    /// - Non-finite or non-positive values become 0.0
    /// - Values >= 1500.0 become 1500.0
    /// - Other values are unchanged
    ///
    /// TDLib reference: `Location.cpp:17-25`
    fn fix_accuracy(accuracy: f64) -> f64 {
        if !accuracy.is_finite() || accuracy <= 0.0 {
            0.0
        } else if accuracy >= MAX_HORIZONTAL_ACCURACY {
            MAX_HORIZONTAL_ACCURACY
        } else {
            accuracy
        }
    }

    /// Returns true if this is an empty location.
    ///
    /// Empty locations represent missing or invalid location data.
    ///
    /// TDLib reference: `Location.cpp:74-76`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::Location;
    ///
    /// assert!(Location::empty().is_empty());
    /// assert!(!Location::from_components(0.0, 0.0, 0.0, 0).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    /// Returns true if this location is valid for map display.
    ///
    /// A location is valid for map display if:
    /// - It is not empty
    /// - Its absolute latitude is <= MAX_VALID_MAP_LATITUDE
    ///
    /// The latitude limit is due to Mercator projection used by web maps.
    ///
    /// TDLib reference: `Location.cpp:78-81`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::Location;
    ///
    /// let moscow = Location::from_components(55.7558, 37.6173, 0.0, 0);
    /// assert!(moscow.is_valid_map_point());
    ///
    /// // Near the pole (beyond Mercator limit)
    /// let pole = Location::from_components(86.0, 0.0, 0.0, 0);
    /// assert!(!pole.is_valid_map_point());
    /// ```
    pub fn is_valid_map_point(&self) -> bool {
        !self.is_empty && self.latitude.abs() <= MAX_VALID_MAP_LATITUDE
    }

    /// Returns the latitude in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::Location;
    ///
    /// let loc = Location::from_components(55.7558, 37.6173, 0.0, 0);
    /// assert_eq!(loc.latitude(), 55.7558);
    /// ```
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    /// Returns the longitude in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::Location;
    ///
    /// let loc = Location::from_components(55.7558, 37.6173, 0.0, 0);
    /// assert_eq!(loc.longitude(), 37.6173);
    /// ```
    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    /// Returns the horizontal accuracy in meters.
    ///
    /// A value of 0.0 indicates accuracy is not specified.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::Location;
    ///
    /// let loc = Location::from_components(0.0, 0.0, 10.5, 0);
    /// assert_eq!(loc.horizontal_accuracy(), 10.5);
    /// ```
    pub fn horizontal_accuracy(&self) -> f64 {
        self.horizontal_accuracy
    }

    /// Returns the access hash.
    ///
    /// The access hash is used in MTProto for location validation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::Location;
    ///
    /// let loc = Location::from_components(0.0, 0.0, 0.0, 12345);
    /// assert_eq!(loc.access_hash(), 12345);
    /// ```
    pub fn access_hash(&self) -> i64 {
        self.access_hash
    }

    /// Sets the access hash.
    ///
    /// TDLib reference: `Location.h:74-76`
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_venue::Location;
    ///
    /// let mut loc = Location::from_components(0.0, 0.0, 0.0, 0);
    /// loc.set_access_hash(67890);
    /// assert_eq!(loc.access_hash(), 67890);
    /// ```
    pub fn set_access_hash(&mut self, hash: i64) {
        self.access_hash = hash;
    }

    /// Converts this location to TD API format.
    ///
    /// TDLib reference: `Location.cpp:83-88`
    pub fn to_td_location(&self) -> Option<TdLocation> {
        if self.is_empty {
            return None;
        }
        Some(TdLocation {
            latitude: self.latitude,
            longitude: self.longitude,
            horizontal_accuracy: self.horizontal_accuracy,
        })
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::empty()
    }
}

/// TD API location format.
///
/// Corresponds to `td_api.tl:537`:
/// ```text
/// location latitude:double longitude:double horizontal_accuracy:double = Location;
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TdLocation {
    /// Latitude in degrees.
    pub latitude: f64,
    /// Longitude in degrees.
    pub longitude: f64,
    /// Horizontal accuracy in meters.
    pub horizontal_accuracy: f64,
}

/// Input GeoPoint for Telegram API.
///
/// Corresponds to `telegram_api.tl:81-82`:
/// ```text
/// inputGeoPointEmpty#e4c123d6 = InputGeoPoint;
/// inputGeoPoint#48222faf flags:# lat:double long:double
///     accuracy_radius:flags.0?int = InputGeoPoint;
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputGeoPoint {
    /// Empty geo point.
    Empty,
    /// Geo point with data.
    Data {
        /// TL flags.
        flags: i32,
        /// Latitude in degrees.
        lat: f64,
        /// Longitude in degrees.
        long: f64,
        /// Optional accuracy radius in meters.
        accuracy_radius: Option<i32>,
    },
}

impl From<&Location> for InputGeoPoint {
    fn from(loc: &Location) -> Self {
        if loc.is_empty() {
            return Self::Empty;
        }

        let mut flags = 0;
        let accuracy_radius = if loc.horizontal_accuracy > 0.0 {
            flags |= 1; // ACCURACY_RADIUS_MASK
            Some(loc.horizontal_accuracy.ceil() as i32)
        } else {
            None
        };

        Self::Data {
            flags,
            lat: loc.latitude,
            long: loc.longitude,
            accuracy_radius,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_location() {
        let loc = Location::empty();
        assert!(loc.is_empty());
        assert_eq!(loc.latitude(), 0.0);
        assert_eq!(loc.longitude(), 0.0);
        assert_eq!(loc.horizontal_accuracy(), 0.0);
        assert_eq!(loc.access_hash(), 0);
    }

    #[test]
    fn test_valid_coordinates() {
        let loc = Location::from_components(55.7558, 37.6173, 10.0, 12345);
        assert!(!loc.is_empty());
        assert_eq!(loc.latitude(), 55.7558);
        assert_eq!(loc.longitude(), 37.6173);
        assert_eq!(loc.horizontal_accuracy(), 10.0);
        assert_eq!(loc.access_hash(), 12345);
    }

    #[test]
    fn test_latitude_out_of_range() {
        // Latitude > 90
        let loc = Location::from_components(91.0, 0.0, 0.0, 0);
        assert!(loc.is_empty());

        // Latitude < -90
        let loc = Location::from_components(-91.0, 0.0, 0.0, 0);
        assert!(loc.is_empty());

        // Exactly 90 is valid
        let loc = Location::from_components(90.0, 0.0, 0.0, 0);
        assert!(!loc.is_empty());

        // Exactly -90 is valid
        let loc = Location::from_components(-90.0, 0.0, 0.0, 0);
        assert!(!loc.is_empty());
    }

    #[test]
    fn test_longitude_out_of_range() {
        // Longitude > 180
        let loc = Location::from_components(0.0, 181.0, 0.0, 0);
        assert!(loc.is_empty());

        // Longitude < -180
        let loc = Location::from_components(0.0, -181.0, 0.0, 0);
        assert!(loc.is_empty());

        // Exactly 180 is valid
        let loc = Location::from_components(0.0, 180.0, 0.0, 0);
        assert!(!loc.is_empty());

        // Exactly -180 is valid
        let loc = Location::from_components(0.0, -180.0, 0.0, 0);
        assert!(!loc.is_empty());
    }

    #[test]
    fn test_nan_coordinates() {
        let loc = Location::from_components(f64::NAN, 0.0, 0.0, 0);
        assert!(loc.is_empty());

        let loc = Location::from_components(0.0, f64::INFINITY, 0.0, 0);
        assert!(loc.is_empty());

        let loc = Location::from_components(0.0, 0.0, f64::NEG_INFINITY, 0);
        assert!(!loc.is_empty()); // Only coordinates matter for empty
    }

    #[test]
    fn test_accuracy_clamping() {
        // Accuracy > 1500 is clamped
        let loc = Location::from_components(0.0, 0.0, 2000.0, 0);
        assert_eq!(loc.horizontal_accuracy(), 1500.0);

        // Exactly 1500 is unchanged
        let loc = Location::from_components(0.0, 0.0, 1500.0, 0);
        assert_eq!(loc.horizontal_accuracy(), 1500.0);

        // Negative accuracy becomes 0
        let loc = Location::from_components(0.0, 0.0, -10.0, 0);
        assert_eq!(loc.horizontal_accuracy(), 0.0);

        // NaN accuracy becomes 0
        let loc = Location::from_components(0.0, 0.0, f64::NAN, 0);
        assert_eq!(loc.horizontal_accuracy(), 0.0);

        // Valid accuracy is unchanged
        let loc = Location::from_components(0.0, 0.0, 10.5, 0);
        assert_eq!(loc.horizontal_accuracy(), 10.5);
    }

    #[test]
    fn test_is_valid_map_point() {
        // Valid point
        let loc = Location::from_components(55.0, 37.0, 0.0, 0);
        assert!(loc.is_valid_map_point());

        // Empty location
        assert!(!Location::empty().is_valid_map_point());

        // High latitude (> 85.05)
        let loc = Location::from_components(86.0, 0.0, 0.0, 0);
        assert!(!loc.is_valid_map_point());

        // Exactly at the limit
        let loc = Location::from_components(MAX_VALID_MAP_LATITUDE, 0.0, 0.0, 0);
        assert!(loc.is_valid_map_point());

        // Negative high latitude
        let loc = Location::from_components(-86.0, 0.0, 0.0, 0);
        assert!(!loc.is_valid_map_point());
    }

    #[test]
    fn test_access_hash() {
        let mut loc = Location::from_components(0.0, 0.0, 0.0, 12345);
        assert_eq!(loc.access_hash(), 12345);

        loc.set_access_hash(67890);
        assert_eq!(loc.access_hash(), 67890);
    }

    #[test]
    fn test_location_equality() {
        let loc1 = Location::from_components(55.0, 37.0, 10.0, 0);
        let loc2 = Location::from_components(55.0, 37.0, 10.0, 0);
        assert_eq!(loc1, loc2);

        // Different accuracy
        let loc3 = Location::from_components(55.0, 37.0, 20.0, 0);
        assert_ne!(loc1, loc3);

        // Different access_hash makes locations different
        let loc4 = Location::from_components(55.0, 37.0, 10.0, 123);
        assert_ne!(loc1, loc4);
    }

    #[test]
    fn test_default() {
        let loc = Location::default();
        assert!(loc.is_empty());
    }

    #[test]
    fn test_from_td_location() {
        let loc = Location::from_td_location(55.7558, 37.6173, 10.0);
        assert!(!loc.is_empty());
        assert_eq!(loc.latitude(), 55.7558);
        assert_eq!(loc.longitude(), 37.6173);
        assert_eq!(loc.horizontal_accuracy(), 10.0);
        assert_eq!(loc.access_hash(), 0); // TD API doesn't have access_hash
    }

    #[test]
    fn test_to_td_location() {
        let loc = Location::from_components(55.7558, 37.6173, 10.0, 12345);
        match loc.to_td_location() {
            Some(td_loc) => {
                assert_eq!(td_loc.latitude, 55.7558);
                assert_eq!(td_loc.longitude, 37.6173);
                assert_eq!(td_loc.horizontal_accuracy, 10.0);
            }
            None => panic!("to_td_location returned None for valid location"),
        }

        // Empty location returns None
        assert!(Location::empty().to_td_location().is_none());
    }

    #[test]
    fn test_to_input_geo_point() {
        // With accuracy
        let loc = Location::from_components(55.7558, 37.6173, 10.5, 0);
        let input_geo: InputGeoPoint = (&loc).into();
        match input_geo {
            InputGeoPoint::Data {
                flags,
                lat,
                long,
                accuracy_radius,
            } => {
                assert_eq!(flags, 1); // ACCURACY_RADIUS_MASK
                assert_eq!(lat, 55.7558);
                assert_eq!(long, 37.6173);
                assert_eq!(accuracy_radius, Some(11)); // ceil(10.5)
            }
            _ => panic!("Expected Data"),
        }

        // Without accuracy
        let loc = Location::from_components(55.7558, 37.6173, 0.0, 0);
        let input_geo: InputGeoPoint = (&loc).into();
        match input_geo {
            InputGeoPoint::Data {
                flags,
                accuracy_radius,
                ..
            } => {
                assert_eq!(flags, 0);
                assert_eq!(accuracy_radius, None);
            }
            _ => panic!("Expected Data"),
        }

        // Empty location
        let input_geo: InputGeoPoint = (&Location::empty()).into();
        assert_eq!(input_geo, InputGeoPoint::Empty);
    }
}

#[cfg(all(test, feature = "proptest"))]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_valid_coordinates(lat in -90.0..90.0, long in -180.0..180.0) {
            let loc = Location::from_components(lat, long, 0.0, 0);
            prop_assert!(!loc.is_empty());
            prop_assert_eq!(loc.latitude(), lat);
            prop_assert_eq!(loc.longitude(), long);
        }

        #[test]
        fn test_invalid_latitude(lat in 90.001..=200.0) {
            let loc = Location::from_components(lat, 0.0, 0.0, 0);
            prop_assert!(loc.is_empty());
        }

        #[test]
        fn test_invalid_latitude_negative(lat in -200.0..=-90.001) {
            let loc = Location::from_components(lat, 0.0, 0.0, 0);
            prop_assert!(loc.is_empty());
        }

        #[test]
        fn test_invalid_longitude(long in 180.001..=360.0) {
            let loc = Location::from_components(0.0, long, 0.0, 0);
            prop_assert!(loc.is_empty());
        }

        #[test]
        fn test_invalid_longitude_negative(long in -360.0..=-180.001) {
            let loc = Location::from_components(0.0, long, 0.0, 0);
            prop_assert!(loc.is_empty());
        }

        #[test]
        fn test_accuracy_clamping(accuracy in 0.0..2000.0) {
            let loc = Location::from_components(0.0, 0.0, accuracy, 0);
            let expected = accuracy.min(1500.0);
            prop_assert_eq!(loc.horizontal_accuracy(), expected);
        }

        #[test]
        fn test_accuracy_negative(accuracy in -2000.0..=0.0) {
            let loc = Location::from_components(0.0, 0.0, accuracy, 0);
            prop_assert_eq!(loc.horizontal_accuracy(), 0.0);
        }

        #[test]
        fn test_map_point_validity(lat in -85.05112877..85.05112877) {
            let loc = Location::from_components(lat, 0.0, 0.0, 0);
            prop_assert!(loc.is_valid_map_point());
        }

        #[test]
        fn test_map_point_invalid_high(lat in 85.05112878..=90.0) {
            let loc = Location::from_components(lat, 0.0, 0.0, 0);
            prop_assert!(!loc.is_valid_map_point());
        }

        #[test]
        fn test_map_point_invalid_low(lat in -90.0..=-85.05112878) {
            let loc = Location::from_components(lat, 0.0, 0.0, 0);
            prop_assert!(!loc.is_valid_map_point());
        }
    }
}
