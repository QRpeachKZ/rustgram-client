// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

//! # Media Area
//!
//! Interactive areas on story media for Telegram.
//!
//! ## Overview
//!
//! This module provides the [`MediaArea`] struct, which represents
//! an interactive area on story media (e.g., location, venue, reaction).
//!
//! ## Example
//!
//! ```rust
//! use rustgram_media_area::MediaArea;
//!
//! // Create a location area
//! let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
//! assert_eq!(area.area_type(), "location");
//! assert_eq!(area.coordinates(), (10.0, 20.0, 30.0, 40.0));
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Interactive area on story media.
///
/// Represents an interactive area (hotspot) on a story that users can interact with.
///
/// # Fields
///
/// - `area_type` - Type of area ("location", "venue", "reaction", etc.)
/// - `coordinates` - (x, y, width, height) as percentages (0-100)
///
/// # Example
///
/// ```rust
/// use rustgram_media_area::MediaArea;
///
/// let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
/// assert_eq!(area.area_type(), "location");
/// assert_eq!(area.coordinates(), (10.0, 20.0, 30.0, 40.0));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MediaArea {
    /// Type of area ("location", "venue", "reaction", etc.)
    area_type: String,

    /// Coordinates as (x, y, width, height) in percentages
    coordinates: (f32, f32, f32, f32),
}

impl MediaArea {
    /// Creates a new media area.
    ///
    /// # Arguments
    ///
    /// * `area_type` - Type of area (e.g., "location", "venue", "reaction")
    /// * `coordinates` - (x, y, width, height) as percentages (0-100)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_media_area::MediaArea;
    ///
    /// let area = MediaArea::new("location".to_string(), (10.0, 20.0, 30.0, 40.0));
    /// assert_eq!(area.area_type(), "location");
    /// ```
    #[must_use]
    pub fn new(area_type: String, coordinates: (f32, f32, f32, f32)) -> Self {
        Self {
            area_type,
            coordinates,
        }
    }

    /// Creates a location area.
    ///
    /// # Arguments
    ///
    /// * `x` - X position in percent (0-100)
    /// * `y` - Y position in percent (0-100)
    /// * `width` - Width in percent (0-100)
    /// * `height` - Height in percent (0-100)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_media_area::MediaArea;
    ///
    /// let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
    /// assert_eq!(area.area_type(), "location");
    /// ```
    #[must_use]
    pub fn location(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            area_type: String::from("location"),
            coordinates: (x, y, width, height),
        }
    }

    /// Creates a venue area.
    ///
    /// # Arguments
    ///
    /// * `x` - X position in percent (0-100)
    /// * `y` - Y position in percent (0-100)
    /// * `width` - Width in percent (0-100)
    /// * `height` - Height in percent (0-100)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_media_area::MediaArea;
    ///
    /// let area = MediaArea::venue(10.0, 20.0, 30.0, 40.0);
    /// assert_eq!(area.area_type(), "venue");
    /// ```
    #[must_use]
    pub fn venue(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            area_type: String::from("venue"),
            coordinates: (x, y, width, height),
        }
    }

    /// Creates a reaction area.
    ///
    /// # Arguments
    ///
    /// * `x` - X position in percent (0-100)
    /// * `y` - Y position in percent (0-100)
    /// * `width` - Width in percent (0-100)
    /// * `height` - Height in percent (0-100)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_media_area::MediaArea;
    ///
    /// let area = MediaArea::reaction(10.0, 20.0, 30.0, 40.0);
    /// assert_eq!(area.area_type(), "reaction");
    /// ```
    #[must_use]
    pub fn reaction(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            area_type: String::from("reaction"),
            coordinates: (x, y, width, height),
        }
    }

    /// Returns the area type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_media_area::MediaArea;
    ///
    /// let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
    /// assert_eq!(area.area_type(), "location");
    /// ```
    #[must_use]
    pub fn area_type(&self) -> &str {
        &self.area_type
    }

    /// Returns the coordinates.
    ///
    /// Returns (x, y, width, height) as percentages.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_media_area::MediaArea;
    ///
    /// let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
    /// assert_eq!(area.coordinates(), (10.0, 20.0, 30.0, 40.0));
    /// ```
    #[must_use]
    pub const fn coordinates(&self) -> (f32, f32, f32, f32) {
        self.coordinates
    }

    /// Returns the X position.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_media_area::MediaArea;
    ///
    /// let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
    /// assert_eq!(area.x(), 10.0);
    /// ```
    #[must_use]
    pub const fn x(&self) -> f32 {
        self.coordinates.0
    }

    /// Returns the Y position.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_media_area::MediaArea;
    ///
    /// let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
    /// assert_eq!(area.y(), 20.0);
    /// ```
    #[must_use]
    pub const fn y(&self) -> f32 {
        self.coordinates.1
    }

    /// Returns the width.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_media_area::MediaArea;
    ///
    /// let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
    /// assert_eq!(area.width(), 30.0);
    /// ```
    #[must_use]
    pub const fn width(&self) -> f32 {
        self.coordinates.2
    }

    /// Returns the height.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_media_area::MediaArea;
    ///
    /// let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
    /// assert_eq!(area.height(), 40.0);
    /// ```
    #[must_use]
    pub const fn height(&self) -> f32 {
        self.coordinates.3
    }

    /// Checks if this is a location area.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_media_area::MediaArea;
    ///
    /// assert!(MediaArea::location(0.0, 0.0, 10.0, 10.0).is_location());
    /// assert!(!MediaArea::venue(0.0, 0.0, 10.0, 10.0).is_location());
    /// ```
    #[must_use]
    pub fn is_location(&self) -> bool {
        self.area_type == "location"
    }

    /// Checks if this is a venue area.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_media_area::MediaArea;
    ///
    /// assert!(MediaArea::venue(0.0, 0.0, 10.0, 10.0).is_venue());
    /// assert!(!MediaArea::location(0.0, 0.0, 10.0, 10.0).is_venue());
    /// ```
    #[must_use]
    pub fn is_venue(&self) -> bool {
        self.area_type == "venue"
    }

    /// Checks if this is a reaction area.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_media_area::MediaArea;
    ///
    /// assert!(MediaArea::reaction(0.0, 0.0, 10.0, 10.0).is_reaction());
    /// assert!(!MediaArea::location(0.0, 0.0, 10.0, 10.0).is_reaction());
    /// ```
    #[must_use]
    pub fn is_reaction(&self) -> bool {
        self.area_type == "reaction"
    }

    /// Checks if the coordinates are valid (all values in range 0-100).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_media_area::MediaArea;
    ///
    /// let valid = MediaArea::location(10.0, 20.0, 30.0, 40.0);
    /// assert!(valid.has_valid_coordinates());
    ///
    /// let invalid = MediaArea::location(-1.0, 20.0, 30.0, 40.0);
    /// assert!(!invalid.has_valid_coordinates());
    /// ```
    #[must_use]
    pub fn has_valid_coordinates(&self) -> bool {
        let (x, y, w, h) = self.coordinates;
        (0.0..=100.0).contains(&x)
            && (0.0..=100.0).contains(&y)
            && (0.0..=100.0).contains(&w)
            && (0.0..=100.0).contains(&h)
    }
}

impl fmt::Display for MediaArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} @ ({}, {}, {}x{})",
            self.area_type,
            self.coordinates.0,
            self.coordinates.1,
            self.coordinates.2,
            self.coordinates.3
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let area = MediaArea::new("location".to_string(), (10.0, 20.0, 30.0, 40.0));
        assert_eq!(area.area_type(), "location");
        assert_eq!(area.coordinates(), (10.0, 20.0, 30.0, 40.0));
    }

    #[test]
    fn test_location() {
        let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
        assert_eq!(area.area_type(), "location");
        assert!(area.is_location());
        assert!(!area.is_venue());
        assert!(!area.is_reaction());
    }

    #[test]
    fn test_venue() {
        let area = MediaArea::venue(10.0, 20.0, 30.0, 40.0);
        assert_eq!(area.area_type(), "venue");
        assert!(!area.is_location());
        assert!(area.is_venue());
        assert!(!area.is_reaction());
    }

    #[test]
    fn test_reaction() {
        let area = MediaArea::reaction(10.0, 20.0, 30.0, 40.0);
        assert_eq!(area.area_type(), "reaction");
        assert!(!area.is_location());
        assert!(!area.is_venue());
        assert!(area.is_reaction());
    }

    #[test]
    fn test_area_type() {
        let area = MediaArea::new("custom".to_string(), (0.0, 0.0, 10.0, 10.0));
        assert_eq!(area.area_type(), "custom");
    }

    #[test]
    fn test_coordinates() {
        let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
        assert_eq!(area.coordinates(), (10.0, 20.0, 30.0, 40.0));
    }

    #[test]
    fn test_x() {
        let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
        assert_eq!(area.x(), 10.0);
    }

    #[test]
    fn test_y() {
        let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
        assert_eq!(area.y(), 20.0);
    }

    #[test]
    fn test_width() {
        let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
        assert_eq!(area.width(), 30.0);
    }

    #[test]
    fn test_height() {
        let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
        assert_eq!(area.height(), 40.0);
    }

    #[test]
    fn test_is_location() {
        assert!(MediaArea::location(0.0, 0.0, 10.0, 10.0).is_location());
        assert!(!MediaArea::venue(0.0, 0.0, 10.0, 10.0).is_location());
        assert!(!MediaArea::reaction(0.0, 0.0, 10.0, 10.0).is_location());
    }

    #[test]
    fn test_is_venue() {
        assert!(MediaArea::venue(0.0, 0.0, 10.0, 10.0).is_venue());
        assert!(!MediaArea::location(0.0, 0.0, 10.0, 10.0).is_venue());
        assert!(!MediaArea::reaction(0.0, 0.0, 10.0, 10.0).is_venue());
    }

    #[test]
    fn test_is_reaction() {
        assert!(MediaArea::reaction(0.0, 0.0, 10.0, 10.0).is_reaction());
        assert!(!MediaArea::location(0.0, 0.0, 10.0, 10.0).is_reaction());
        assert!(!MediaArea::venue(0.0, 0.0, 10.0, 10.0).is_reaction());
    }

    #[test]
    fn test_has_valid_coordinates_true() {
        assert!(MediaArea::location(0.0, 0.0, 10.0, 10.0).has_valid_coordinates());
        assert!(MediaArea::location(50.0, 50.0, 50.0, 50.0).has_valid_coordinates());
        assert!(MediaArea::location(100.0, 100.0, 100.0, 100.0).has_valid_coordinates());
    }

    #[test]
    fn test_has_valid_coordinates_false() {
        assert!(!MediaArea::location(-1.0, 0.0, 10.0, 10.0).has_valid_coordinates());
        assert!(!MediaArea::location(0.0, -1.0, 10.0, 10.0).has_valid_coordinates());
        assert!(!MediaArea::location(0.0, 0.0, -1.0, 10.0).has_valid_coordinates());
        assert!(!MediaArea::location(0.0, 0.0, 10.0, -1.0).has_valid_coordinates());
        assert!(!MediaArea::location(101.0, 0.0, 10.0, 10.0).has_valid_coordinates());
        assert!(!MediaArea::location(0.0, 101.0, 10.0, 10.0).has_valid_coordinates());
        assert!(!MediaArea::location(0.0, 0.0, 101.0, 10.0).has_valid_coordinates());
        assert!(!MediaArea::location(0.0, 0.0, 10.0, 101.0).has_valid_coordinates());
    }

    #[test]
    fn test_equality() {
        let area1 = MediaArea::location(10.0, 20.0, 30.0, 40.0);
        let area2 = MediaArea::location(10.0, 20.0, 30.0, 40.0);
        assert_eq!(area1, area2);

        let area3 = MediaArea::location(11.0, 20.0, 30.0, 40.0);
        assert_ne!(area1, area3);

        let area4 = MediaArea::venue(10.0, 20.0, 30.0, 40.0);
        assert_ne!(area1, area4);
    }

    #[test]
    fn test_clone() {
        let area1 = MediaArea::location(10.0, 20.0, 30.0, 40.0);
        let area2 = area1.clone();
        assert_eq!(area1, area2);
    }

    #[test]
    fn test_display() {
        let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
        let display = format!("{}", area);
        assert!(display.contains("location"));
        assert!(display.contains("10"));
        assert!(display.contains("20"));
        assert!(display.contains("30"));
        assert!(display.contains("40"));
    }

    #[test]
    fn test_debug_format() {
        let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
        let debug_str = format!("{:?}", area);
        assert!(debug_str.contains("MediaArea"));
        assert!(debug_str.contains("location"));
    }

    #[test]
    fn test_custom_area_type() {
        let custom = MediaArea::new("custom_type".to_string(), (0.0, 0.0, 10.0, 10.0));
        assert_eq!(custom.area_type(), "custom_type");
        assert!(!custom.is_location());
        assert!(!custom.is_venue());
        assert!(!custom.is_reaction());
    }

    #[test]
    fn test_zero_coordinates() {
        let area = MediaArea::location(0.0, 0.0, 0.0, 0.0);
        assert!(area.has_valid_coordinates());
    }

    #[test]
    fn test_boundary_coordinates() {
        let area = MediaArea::location(0.0, 0.0, 100.0, 100.0);
        assert!(area.has_valid_coordinates());
    }

    #[test]
    fn test_fractional_coordinates() {
        let area = MediaArea::location(12.5, 37.8, 45.3, 99.9);
        assert!(area.has_valid_coordinates());
    }

    // ========== Serialization Tests ==========

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_deserialize() {
        let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
        let json = serde_json::to_string(&area).unwrap();
        let deserialized: MediaArea = serde_json::from_str(&json).unwrap();
        assert_eq!(area, deserialized);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serialize_format() {
        let area = MediaArea::location(10.0, 20.0, 30.0, 40.0);
        let json = serde_json::to_string(&area).unwrap();
        assert!(json.contains("location"));
    }
}
