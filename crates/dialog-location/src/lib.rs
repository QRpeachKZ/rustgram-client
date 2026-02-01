//! Dialog location information.
//!
//! This module provides the `DialogLocation` type, which represents a location
//! attached to a dialog (e.g., for groups, channels, or businesses).
//!
//! # TDLib Alignment
//!
//! - TDLib type: `DialogLocation` (td/telegram/DialogLocation.h)
//! - Contains geographic location and optional address
//!
//! # Example
//!
//! ```rust
//! use rustgram_dialog_location::DialogLocation;
//!
//! // Create a location with latitude, longitude, and address
//! let location = DialogLocation::new(37.7749, -122.4194, Some("San Francisco, CA"));
//! assert!(!location.is_empty());
//! assert_eq!(location.address(), Some("San Francisco, CA"));
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Dialog location information.
///
/// Represents a geographic location that can be attached to a dialog,
/// such as a group chat location or business location.
///
/// # Example
///
/// ```rust
/// use rustgram_dialog_location::DialogLocation;
///
/// let location = DialogLocation::new(37.7749, -122.4194, Some("San Francisco"));
/// assert_eq!(location.latitude(), 37.7749);
/// assert_eq!(location.longitude(), -122.4194);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DialogLocation {
    /// Latitude of the location.
    latitude: f64,
    /// Longitude of the location.
    longitude: f64,
    /// Optional address string.
    address: Option<String>,
}

impl DialogLocation {
    /// Creates a new dialog location.
    ///
    /// # Arguments
    ///
    /// * `latitude` - Latitude in degrees
    /// * `longitude` - Longitude in degrees
    /// * `address` - Optional address string
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_location::DialogLocation;
    ///
    /// let location = DialogLocation::new(37.7749, -122.4194, Some("SF"));
    /// assert_eq!(location.latitude(), 37.7749);
    /// ```
    pub fn new(latitude: f64, longitude: f64, address: Option<String>) -> Self {
        Self {
            latitude,
            longitude,
            address,
        }
    }

    /// Creates an empty dialog location.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_location::DialogLocation;
    ///
    /// let location = DialogLocation::empty();
    /// assert!(location.is_empty());
    /// ```
    pub fn empty() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            address: None,
        }
    }

    /// Returns the latitude.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_location::DialogLocation;
    ///
    /// let location = DialogLocation::new(37.7749, -122.4194, None);
    /// assert_eq!(location.latitude(), 37.7749);
    /// ```
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    /// Returns the longitude.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_location::DialogLocation;
    ///
    /// let location = DialogLocation::new(37.7749, -122.4194, None);
    /// assert_eq!(location.longitude(), -122.4194);
    /// ```
    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    /// Returns the address if available.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_location::DialogLocation;
    ///
    /// let location = DialogLocation::new(37.7749, -122.4194, Some("San Francisco"));
    /// assert_eq!(location.address(), Some("San Francisco"));
    /// ```
    pub fn address(&self) -> Option<&str> {
        self.address.as_deref()
    }

    /// Checks if this location is empty.
    ///
    /// A location is considered empty if both latitude and longitude are 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_location::DialogLocation;
    ///
    /// let empty = DialogLocation::empty();
    /// assert!(empty.is_empty());
    ///
    /// let location = DialogLocation::new(37.7749, -122.4194, None);
    /// assert!(!location.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.latitude == 0.0 && self.longitude == 0.0
    }

    /// Sets the address.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dialog_location::DialogLocation;
    ///
    /// let mut location = DialogLocation::new(37.7749, -122.4194, None);
    /// location.set_address(Some("New Address"));
    /// assert_eq!(location.address(), Some("New Address"));
    /// ```
    pub fn set_address(&mut self, address: Option<String>) {
        self.address = address;
    }
}

impl Default for DialogLocation {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for DialogLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            write!(f, "no location")
        } else if let Some(addr) = &self.address {
            write!(f, "{}, {} ({})", self.latitude, self.longitude, addr)
        } else {
            write!(f, "{}, {}", self.latitude, self.longitude)
        }
    }
}

impl Serialize for DialogLocation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.latitude, self.longitude, &self.address).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DialogLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (latitude, longitude, address) =
            <(f64, f64, Option<String>)>::deserialize(deserializer)?;
        Ok(Self {
            latitude,
            longitude,
            address,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (10)
    #[test]
    fn test_debug() {
        let location = DialogLocation::new(37.7749, -122.4194, None);
        assert!(format!("{:?}", location).contains("DialogLocation"));
    }

    #[test]
    fn test_clone() {
        let location = DialogLocation::new(37.7749, -122.4194, Some("SF".to_string()));
        let cloned = location.clone();
        assert_eq!(location, cloned);
    }

    #[test]
    fn test_partial_eq() {
        let loc1 = DialogLocation::new(37.7749, -122.4194, None);
        let loc2 = DialogLocation::new(37.7749, -122.4194, None);
        let loc3 = DialogLocation::new(40.7128, -74.0060, None);
        assert_eq!(loc1, loc2);
        assert_ne!(loc1, loc3);
    }

    #[test]
    fn test_default() {
        let location = DialogLocation::default();
        assert!(location.is_empty());
    }

    #[test]
    fn test_display_with_address() {
        let location = DialogLocation::new(37.7749, -122.4194, Some("San Francisco"));
        assert!(format!("{}", location).contains("San Francisco"));
    }

    #[test]
    fn test_display_without_address() {
        let location = DialogLocation::new(37.7749, -122.4194, None);
        assert!(format!("{}", location).contains("37.7749"));
    }

    #[test]
    fn test_display_empty() {
        let location = DialogLocation::empty();
        assert_eq!(format!("{}", location), "no location");
    }

    // Constructor tests (3 * 3 = 9)
    #[test]
    fn test_new_with_address() {
        let location = DialogLocation::new(37.7749, -122.4194, Some("SF".to_string()));
        assert_eq!(location.latitude(), 37.7749);
        assert_eq!(location.longitude(), -122.4194);
        assert_eq!(location.address(), Some("SF"));
    }

    #[test]
    fn test_new_without_address() {
        let location = DialogLocation::new(37.7749, -122.4194, None);
        assert_eq!(location.latitude(), 37.7749);
        assert_eq!(location.longitude(), -122.4194);
        assert_eq!(location.address(), None);
    }

    #[test]
    fn test_empty() {
        let location = DialogLocation::empty();
        assert_eq!(location.latitude(), 0.0);
        assert_eq!(location.longitude(), 0.0);
        assert_eq!(location.address(), None);
        assert!(location.is_empty());
    }

    // Getter tests (3 * 2 = 6)
    #[test]
    fn test_latitude() {
        let location = DialogLocation::new(37.7749, -122.4194, None);
        assert_eq!(location.latitude(), 37.7749);
    }

    #[test]
    fn test_longitude() {
        let location = DialogLocation::new(37.7749, -122.4194, None);
        assert_eq!(location.longitude(), -122.4194);
    }

    #[test]
    fn test_address_some() {
        let location = DialogLocation::new(37.7749, -122.4194, Some("SF"));
        assert_eq!(location.address(), Some("SF"));
    }

    #[test]
    fn test_address_none() {
        let location = DialogLocation::new(37.7749, -122.4194, None);
        assert_eq!(location.address(), None);
    }

    // Method tests (3 * 3 = 9)
    #[test]
    fn test_is_empty_true() {
        let location = DialogLocation::empty();
        assert!(location.is_empty());
    }

    #[test]
    fn test_is_empty_false_with_coords() {
        let location = DialogLocation::new(37.7749, -122.4194, None);
        assert!(!location.is_empty());
    }

    #[test]
    fn test_set_address() {
        let mut location = DialogLocation::new(37.7749, -122.4194, None);
        location.set_address(Some("New Address".to_string()));
        assert_eq!(location.address(), Some("New Address"));
    }

    #[test]
    fn test_set_address_none() {
        let mut location = DialogLocation::new(37.7749, -122.4194, Some("Old"));
        location.set_address(None);
        assert_eq!(location.address(), None);
    }

    // Serialization tests (2)
    #[test]
    fn test_serialize_deserialize() {
        let location = DialogLocation::new(37.7749, -122.4194, Some("SF"));
        let json = serde_json::to_string(&location).unwrap();
        let deserialized: DialogLocation = serde_json::from_str(&json).unwrap();
        assert_eq!(location, deserialized);
    }

    #[test]
    fn test_serialize_deserialize_empty() {
        let location = DialogLocation::empty();
        let json = serde_json::to_string(&location).unwrap();
        let deserialized: DialogLocation = serde_json::from_str(&json).unwrap();
        assert_eq!(location, deserialized);
    }

    // Edge cases (2)
    #[test]
    fn test_negative_coordinates() {
        let location = DialogLocation::new(-33.8688, 151.2093, None);
        assert_eq!(location.latitude(), -33.8688);
        assert_eq!(location.longitude(), 151.2093);
    }

    #[test]
    fn test_zero_coords_non_empty_address() {
        // Zero coords with address is technically empty by our definition
        let location = DialogLocation::new(0.0, 0.0, Some("Null Island"));
        assert!(location.is_empty());
        assert_eq!(location.address(), Some("Null Island"));
    }

    // Doc test example (1)
    #[test]
    fn test_doc_example() {
        let location = DialogLocation::new(37.7749, -122.4194, Some("San Francisco, CA"));
        assert!(!location.is_empty());
        assert_eq!(location.address(), Some("San Francisco, CA"));
    }
}
