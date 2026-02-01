// Copyright 2024 rustgram-client contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Media Area Coordinates
//!
//! Coordinates for media areas in Telegram stories.
//!
//! ## Overview
//!
//! `MediaAreaCoordinates` represents the position and size of an interactive area
//! within a media story, with validation and normalization logic ported from TDLib.
//!
//! ## TDLib Correspondence
//!
//! | Rust Type | TDLib Type | File |
//! |-----------|-----------|------|
//! | [`MediaAreaCoordinates`] | `mediaAreaCoordinates` | `MediaAreaCoordinates.h` |
//!
//! The validation logic is ported from `MediaAreaCoordinates.cpp:15-32`.
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_media_area_coordinates::MediaAreaCoordinates;
//!
//! // Create coordinates with automatic clamping
//! let coords = MediaAreaCoordinates::new(50.0, 50.0, 25.0, 25.0, 0.0, 0.0);
//! assert!(coords.is_valid());
//! assert_eq!(coords.x(), 50.0);
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Media area coordinates with validation.
///
/// Represents position and size of an interactive area within a media story.
/// All values are clamped to valid ranges during construction:
/// - x, y, width, height, radius: 0.0 to 100.0
/// - rotation_angle: normalized to 0.0 to 360.0
///
/// # Examples
///
/// ```
/// use rustgram_media_area_coordinates::MediaAreaCoordinates;
///
/// // Create valid coordinates
/// let coords = MediaAreaCoordinates::new(50.0, 50.0, 25.0, 25.0, 45.0, 5.0);
/// assert!(coords.is_valid());
///
/// // Out-of-range values are automatically clamped
/// let coords = MediaAreaCoordinates::new(150.0, 50.0, 25.0, 25.0, 0.0, 0.0);
/// assert_eq!(coords.x(), 100.0); // Clamped to 100
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAreaCoordinates {
    /// X coordinate (percentage, 0-100)
    x: f64,
    /// Y coordinate (percentage, 0-100)
    y: f64,
    /// Width (percentage, 0-100)
    width: f64,
    /// Height (percentage, 0-100)
    height: f64,
    /// Rotation angle (degrees, 0-360, normalized)
    rotation_angle: f64,
    /// Corner radius (percentage, 0-100)
    radius: f64,
}

impl Default for MediaAreaCoordinates {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            rotation_angle: 0.0,
            radius: 0.0,
        }
    }
}

impl MediaAreaCoordinates {
    /// Minimum clamping value for most fields (0.0).
    pub const MIN_VALUE: f64 = 0.0;

    /// Maximum clamping value for most fields (100.0).
    pub const MAX_VALUE: f64 = 100.0;

    /// Minimum value for rotation angle (-360.0).
    pub const MIN_ROTATION: f64 = -360.0;

    /// Maximum value for rotation angle (360.0).
    pub const MAX_ROTATION: f64 = 360.0;

    /// Tolerance for floating point comparisons.
    pub const EPSILON: f64 = 1e-6;

    /// Creates new media area coordinates with validation.
    ///
    /// All values are automatically clamped to valid ranges:
    /// - x, y, width, height, radius: 0.0 to 100.0
    /// - rotation_angle: normalized to 0.0 to 360.0 (negative values wrap around)
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate (percentage)
    /// * `y` - Y coordinate (percentage)
    /// * `width` - Width (percentage)
    /// * `height` - Height (percentage)
    /// * `rotation_angle` - Rotation angle in degrees
    /// * `radius` - Corner radius (percentage)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let coords = MediaAreaCoordinates::new(50.0, 50.0, 25.0, 25.0, 45.0, 5.0);
    /// assert_eq!(coords.x(), 50.0);
    /// assert_eq!(coords.rotation_angle(), 45.0);
    /// ```
    pub fn new(x: f64, y: f64, width: f64, height: f64, rotation_angle: f64, radius: f64) -> Self {
        Self {
            x: Self::fix_double(x),
            y: Self::fix_double(y),
            width: Self::fix_double(width),
            height: Self::fix_double(height),
            rotation_angle: Self::fix_and_normalize_angle(rotation_angle),
            radius: Self::fix_double(radius),
        }
    }

    /// Clamps a double value to the valid range [0.0, 100.0].
    ///
    /// Non-finite values (NaN, infinity) return 0.0.
    /// Ported from TDLib MediaAreaCoordinates.cpp:15-19.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// assert_eq!(MediaAreaCoordinates::fix_double(50.0), 50.0);
    /// assert_eq!(MediaAreaCoordinates::fix_double(-10.0), 0.0);
    /// assert_eq!(MediaAreaCoordinates::fix_double(150.0), 100.0);
    /// assert_eq!(MediaAreaCoordinates::fix_double(f64::NAN), 0.0);
    /// ```
    pub fn fix_double(value: f64) -> f64 {
        if !value.is_finite() {
            return 0.0;
        }
        value.clamp(Self::MIN_VALUE, Self::MAX_VALUE)
    }

    /// Clamps and normalizes an angle to [0.0, 360.0].
    ///
    /// Ported from TDLib MediaAreaCoordinates.cpp:27-30.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// assert_eq!(MediaAreaCoordinates::fix_and_normalize_angle(45.0), 45.0);
    /// assert_eq!(MediaAreaCoordinates::fix_and_normalize_angle(-45.0), 315.0);
    /// assert_eq!(MediaAreaCoordinates::fix_and_normalize_angle(450.0), 360.0);
    /// ```
    pub fn fix_and_normalize_angle(angle: f64) -> f64 {
        let fixed = Self::fix_double_with_range(angle, Self::MIN_ROTATION, Self::MAX_ROTATION);
        if fixed < 0.0 {
            fixed + 360.0
        } else {
            fixed
        }
    }

    /// Clamps a double value to a custom range.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// assert_eq!(MediaAreaCoordinates::fix_double_with_range(50.0, 0.0, 100.0), 50.0);
    /// assert_eq!(MediaAreaCoordinates::fix_double_with_range(-10.0, 0.0, 100.0), 0.0);
    /// ```
    pub fn fix_double_with_range(value: f64, min_value: f64, max_value: f64) -> f64 {
        if !value.is_finite() {
            return 0.0;
        }
        value.clamp(min_value, max_value)
    }

    /// Returns the X coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let coords = MediaAreaCoordinates::new(50.0, 50.0, 25.0, 25.0, 0.0, 0.0);
    /// assert_eq!(coords.x(), 50.0);
    /// ```
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Returns the Y coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let coords = MediaAreaCoordinates::new(50.0, 50.0, 25.0, 25.0, 0.0, 0.0);
    /// assert_eq!(coords.y(), 50.0);
    /// ```
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Returns the width.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let coords = MediaAreaCoordinates::new(50.0, 50.0, 25.0, 25.0, 0.0, 0.0);
    /// assert_eq!(coords.width(), 25.0);
    /// ```
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Returns the height.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let coords = MediaAreaCoordinates::new(50.0, 50.0, 25.0, 25.0, 0.0, 0.0);
    /// assert_eq!(coords.height(), 25.0);
    /// ```
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Returns the rotation angle in degrees (0-360).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let coords = MediaAreaCoordinates::new(0.0, 0.0, 100.0, 100.0, 45.0, 0.0);
    /// assert_eq!(coords.rotation_angle(), 45.0);
    /// ```
    pub fn rotation_angle(&self) -> f64 {
        self.rotation_angle
    }

    /// Returns the corner radius.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let coords = MediaAreaCoordinates::new(0.0, 0.0, 100.0, 100.0, 0.0, 5.0);
    /// assert_eq!(coords.radius(), 5.0);
    /// ```
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Sets the X coordinate (will be clamped).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let mut coords = MediaAreaCoordinates::default();
    /// coords.set_x(50.0);
    /// assert_eq!(coords.x(), 50.0);
    /// ```
    pub fn set_x(&mut self, x: f64) {
        self.x = Self::fix_double(x);
    }

    /// Sets the Y coordinate (will be clamped).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let mut coords = MediaAreaCoordinates::default();
    /// coords.set_y(50.0);
    /// assert_eq!(coords.y(), 50.0);
    /// ```
    pub fn set_y(&mut self, y: f64) {
        self.y = Self::fix_double(y);
    }

    /// Sets the width (will be clamped).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let mut coords = MediaAreaCoordinates::default();
    /// coords.set_width(50.0);
    /// assert_eq!(coords.width(), 50.0);
    /// ```
    pub fn set_width(&mut self, width: f64) {
        self.width = Self::fix_double(width);
    }

    /// Sets the height (will be clamped).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let mut coords = MediaAreaCoordinates::default();
    /// coords.set_height(50.0);
    /// assert_eq!(coords.height(), 50.0);
    /// ```
    pub fn set_height(&mut self, height: f64) {
        self.height = Self::fix_double(height);
    }

    /// Sets the rotation angle (will be clamped and normalized).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let mut coords = MediaAreaCoordinates::default();
    /// coords.set_rotation_angle(45.0);
    /// assert_eq!(coords.rotation_angle(), 45.0);
    /// ```
    pub fn set_rotation_angle(&mut self, rotation_angle: f64) {
        self.rotation_angle = Self::fix_and_normalize_angle(rotation_angle);
    }

    /// Sets the corner radius (will be clamped).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let mut coords = MediaAreaCoordinates::default();
    /// coords.set_radius(5.0);
    /// assert_eq!(coords.radius(), 5.0);
    /// ```
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = Self::fix_double(radius);
    }

    /// Returns `true` if the coordinates are valid.
    ///
    /// Valid coordinates have width > 0 and height > 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let coords = MediaAreaCoordinates::new(0.0, 0.0, 100.0, 100.0, 0.0, 0.0);
    /// assert!(coords.is_valid());
    ///
    /// let invalid = MediaAreaCoordinates::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    /// assert!(!invalid.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.width > 0.0 && self.height > 0.0
    }

    /// Creates a builder for constructing MediaAreaCoordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_media_area_coordinates::MediaAreaCoordinates;
    ///
    /// let coords = MediaAreaCoordinates::builder()
    ///     .x(50.0)
    ///     .y(50.0)
    ///     .width(25.0)
    ///     .height(25.0)
    ///     .build();
    /// ```
    pub fn builder() -> MediaAreaCoordinatesBuilder {
        MediaAreaCoordinatesBuilder::default()
    }
}

impl Eq for MediaAreaCoordinates {}

impl PartialEq for MediaAreaCoordinates {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < Self::EPSILON
            && (self.y - other.y).abs() < Self::EPSILON
            && (self.width - other.width).abs() < Self::EPSILON
            && (self.height - other.height).abs() < Self::EPSILON
            && (self.rotation_angle - other.rotation_angle).abs() < Self::EPSILON
            && (self.radius - other.radius).abs() < Self::EPSILON
    }
}

impl fmt::Display for MediaAreaCoordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MediaAreaCoordinates[x={}, y={}, w={}, h={}, rotation={}, radius={}]",
            self.x, self.y, self.width, self.height, self.rotation_angle, self.radius
        )
    }
}

/// Builder for [`MediaAreaCoordinates`].
///
/// # Examples
///
/// ```
/// use rustgram_media_area_coordinates::MediaAreaCoordinates;
///
/// let coords = MediaAreaCoordinates::builder()
///     .x(50.0)
///     .y(50.0)
///     .width(25.0)
///     .height(25.0)
///     .rotation_angle(45.0)
///     .radius(5.0)
///     .build();
/// ```
#[derive(Debug, Default, Clone)]
pub struct MediaAreaCoordinatesBuilder {
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    rotation_angle: Option<f64>,
    radius: Option<f64>,
}

impl MediaAreaCoordinatesBuilder {
    /// Sets the X coordinate.
    pub fn x(mut self, x: f64) -> Self {
        self.x = Some(x);
        self
    }

    /// Sets the Y coordinate.
    pub fn y(mut self, y: f64) -> Self {
        self.y = Some(y);
        self
    }

    /// Sets the width.
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the height.
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets the rotation angle.
    pub fn rotation_angle(mut self, rotation_angle: f64) -> Self {
        self.rotation_angle = Some(rotation_angle);
        self
    }

    /// Sets the corner radius.
    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = Some(radius);
        self
    }

    /// Builds the `MediaAreaCoordinates`.
    ///
    /// Unset fields will use their default values (0.0).
    pub fn build(self) -> MediaAreaCoordinates {
        MediaAreaCoordinates::new(
            self.x.unwrap_or(0.0),
            self.y.unwrap_or(0.0),
            self.width.unwrap_or(0.0),
            self.height.unwrap_or(0.0),
            self.rotation_angle.unwrap_or(0.0),
            self.radius.unwrap_or(0.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_coords() -> MediaAreaCoordinates {
        MediaAreaCoordinates::new(50.0, 50.0, 25.0, 25.0, 45.0, 5.0)
    }

    #[test]
    fn test_new() {
        let coords = create_test_coords();
        assert_eq!(coords.x(), 50.0);
        assert_eq!(coords.y(), 50.0);
        assert_eq!(coords.width(), 25.0);
        assert_eq!(coords.height(), 25.0);
        assert_eq!(coords.rotation_angle(), 45.0);
        assert_eq!(coords.radius(), 5.0);
    }

    #[test]
    fn test_default() {
        let coords = MediaAreaCoordinates::default();
        assert_eq!(coords.x(), 0.0);
        assert_eq!(coords.y(), 0.0);
        assert_eq!(coords.width(), 0.0);
        assert_eq!(coords.height(), 0.0);
        assert_eq!(coords.rotation_angle(), 0.0);
        assert_eq!(coords.radius(), 0.0);
    }

    #[test]
    fn test_fix_double_normal() {
        assert_eq!(MediaAreaCoordinates::fix_double(50.0), 50.0);
    }

    #[test]
    fn test_fix_double_negative() {
        assert_eq!(MediaAreaCoordinates::fix_double(-10.0), 0.0);
    }

    #[test]
    fn test_fix_double_over_max() {
        assert_eq!(MediaAreaCoordinates::fix_double(150.0), 100.0);
    }

    #[test]
    fn test_fix_double_nan() {
        assert_eq!(MediaAreaCoordinates::fix_double(f64::NAN), 0.0);
    }

    #[test]
    fn test_fix_double_infinity() {
        // Non-finite values return 0.0 (as per TDLib implementation)
        assert_eq!(MediaAreaCoordinates::fix_double(f64::INFINITY), 0.0);
        assert_eq!(MediaAreaCoordinates::fix_double(f64::NEG_INFINITY), 0.0);
    }

    #[test]
    fn test_fix_double_with_range() {
        assert_eq!(
            MediaAreaCoordinates::fix_double_with_range(50.0, 0.0, 100.0),
            50.0
        );
        assert_eq!(
            MediaAreaCoordinates::fix_double_with_range(-10.0, 0.0, 100.0),
            0.0
        );
        assert_eq!(
            MediaAreaCoordinates::fix_double_with_range(150.0, 0.0, 100.0),
            100.0
        );
    }

    #[test]
    fn test_fix_and_normalize_angle() {
        assert_eq!(MediaAreaCoordinates::fix_and_normalize_angle(45.0), 45.0);
        assert_eq!(MediaAreaCoordinates::fix_and_normalize_angle(0.0), 0.0);
        assert_eq!(MediaAreaCoordinates::fix_and_normalize_angle(360.0), 360.0);
    }

    #[test]
    fn test_fix_and_normalize_negative_angle() {
        assert_eq!(MediaAreaCoordinates::fix_and_normalize_angle(-45.0), 315.0);
        assert_eq!(MediaAreaCoordinates::fix_and_normalize_angle(-180.0), 180.0);
        assert_eq!(MediaAreaCoordinates::fix_and_normalize_angle(-360.0), 0.0);
    }

    #[test]
    fn test_fix_and_normalize_angle_over_max() {
        assert_eq!(MediaAreaCoordinates::fix_and_normalize_angle(450.0), 360.0);
        assert_eq!(MediaAreaCoordinates::fix_and_normalize_angle(720.0), 360.0);
    }

    #[test]
    fn test_is_valid() {
        assert!(create_test_coords().is_valid());
        assert!(MediaAreaCoordinates::new(0.0, 0.0, 0.1, 0.1, 0.0, 0.0).is_valid());
    }

    #[test]
    fn test_is_invalid_zero_width() {
        assert!(!MediaAreaCoordinates::new(0.0, 0.0, 0.0, 10.0, 0.0, 0.0).is_valid());
    }

    #[test]
    fn test_is_invalid_zero_height() {
        assert!(!MediaAreaCoordinates::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0).is_valid());
    }

    #[test]
    fn test_x() {
        assert_eq!(create_test_coords().x(), 50.0);
    }

    #[test]
    fn test_y() {
        assert_eq!(create_test_coords().y(), 50.0);
    }

    #[test]
    fn test_width() {
        assert_eq!(create_test_coords().width(), 25.0);
    }

    #[test]
    fn test_height() {
        assert_eq!(create_test_coords().height(), 25.0);
    }

    #[test]
    fn test_rotation_angle() {
        assert_eq!(create_test_coords().rotation_angle(), 45.0);
    }

    #[test]
    fn test_radius() {
        assert_eq!(create_test_coords().radius(), 5.0);
    }

    #[test]
    fn test_set_x() {
        let mut coords = MediaAreaCoordinates::default();
        coords.set_x(75.0);
        assert_eq!(coords.x(), 75.0);
    }

    #[test]
    fn test_set_y() {
        let mut coords = MediaAreaCoordinates::default();
        coords.set_y(75.0);
        assert_eq!(coords.y(), 75.0);
    }

    #[test]
    fn test_set_width() {
        let mut coords = MediaAreaCoordinates::default();
        coords.set_width(30.0);
        assert_eq!(coords.width(), 30.0);
    }

    #[test]
    fn test_set_height() {
        let mut coords = MediaAreaCoordinates::default();
        coords.set_height(30.0);
        assert_eq!(coords.height(), 30.0);
    }

    #[test]
    fn test_set_rotation_angle() {
        let mut coords = MediaAreaCoordinates::default();
        coords.set_rotation_angle(90.0);
        assert_eq!(coords.rotation_angle(), 90.0);
    }

    #[test]
    fn test_set_radius() {
        let mut coords = MediaAreaCoordinates::default();
        coords.set_radius(10.0);
        assert_eq!(coords.radius(), 10.0);
    }

    #[test]
    fn test_set_clamps_values() {
        let mut coords = MediaAreaCoordinates::default();
        coords.set_x(150.0);
        assert_eq!(coords.x(), 100.0);

        coords.set_x(-10.0);
        assert_eq!(coords.x(), 0.0);
    }

    #[test]
    fn test_set_rotation_angle_normalizes() {
        let mut coords = MediaAreaCoordinates::default();
        coords.set_rotation_angle(-45.0);
        assert_eq!(coords.rotation_angle(), 315.0);
    }

    #[test]
    fn test_equality() {
        let coords1 = create_test_coords();
        let coords2 = create_test_coords();
        assert_eq!(coords1, coords2);

        let coords3 = MediaAreaCoordinates::new(50.0, 50.0, 25.0, 25.0, 45.0, 6.0);
        assert_ne!(coords1, coords3);
    }

    #[test]
    fn test_equality_with_epsilon() {
        let coords1 = MediaAreaCoordinates::new(50.0, 50.0, 25.0, 25.0, 45.0, 5.0);
        let coords2 = MediaAreaCoordinates::new(50.0000001, 50.0, 25.0, 25.0, 45.0, 5.0);
        assert_eq!(coords1, coords2);

        let coords3 = MediaAreaCoordinates::new(50.001, 50.0, 25.0, 25.0, 45.0, 5.0);
        assert_ne!(coords1, coords3);
    }

    #[test]
    fn test_clone() {
        let coords = create_test_coords();
        let cloned = coords.clone();
        assert_eq!(coords, cloned);
    }

    #[test]
    fn test_debug() {
        let coords = create_test_coords();
        let debug = format!("{:?}", coords);
        assert!(debug.contains("MediaAreaCoordinates"));
    }

    #[test]
    fn test_display() {
        let coords = create_test_coords();
        let display = format!("{}", coords);
        assert!(display.contains("x=50"));
        assert!(display.contains("y=50"));
        assert!(display.contains("w=25"));
        assert!(display.contains("h=25"));
        assert!(display.contains("rotation=45"));
        assert!(display.contains("radius=5"));
    }

    #[test]
    fn test_builder() {
        let coords = MediaAreaCoordinates::builder()
            .x(50.0)
            .y(50.0)
            .width(25.0)
            .height(25.0)
            .rotation_angle(45.0)
            .radius(5.0)
            .build();
        assert_eq!(coords.x(), 50.0);
        assert_eq!(coords.y(), 50.0);
        assert_eq!(coords.width(), 25.0);
        assert_eq!(coords.height(), 25.0);
        assert_eq!(coords.rotation_angle(), 45.0);
        assert_eq!(coords.radius(), 5.0);
    }

    #[test]
    fn test_builder_default_values() {
        let coords = MediaAreaCoordinates::builder().build();
        assert_eq!(coords.x(), 0.0);
        assert_eq!(coords.y(), 0.0);
        assert_eq!(coords.width(), 0.0);
        assert_eq!(coords.height(), 0.0);
        assert_eq!(coords.rotation_angle(), 0.0);
        assert_eq!(coords.radius(), 0.0);
    }

    #[test]
    fn test_builder_partial() {
        let coords = MediaAreaCoordinates::builder().x(50.0).width(25.0).build();
        assert_eq!(coords.x(), 50.0);
        assert_eq!(coords.y(), 0.0);
        assert_eq!(coords.width(), 25.0);
        assert_eq!(coords.height(), 0.0);
    }

    #[test]
    fn test_serialization() {
        let coords = create_test_coords();
        let json = serde_json::to_string(&coords).unwrap();
        let parsed: MediaAreaCoordinates = serde_json::from_str(&json).unwrap();
        assert_eq!(coords, parsed);
    }

    #[test]
    fn test_clamping_on_creation() {
        let coords = MediaAreaCoordinates::new(150.0, -10.0, 25.0, 200.0, 0.0, 0.0);
        assert_eq!(coords.x(), 100.0);
        assert_eq!(coords.y(), 0.0);
        assert_eq!(coords.width(), 25.0);
        assert_eq!(coords.height(), 100.0);
    }

    #[test]
    fn test_angle_normalization_various() {
        let test_cases = vec![
            (0.0, 0.0),
            (90.0, 90.0),
            (180.0, 180.0),
            (270.0, 270.0),
            (360.0, 360.0),
            (-90.0, 270.0),
            (-180.0, 180.0),
            (-270.0, 90.0),
            (-360.0, 0.0),
            (450.0, 360.0),
            (-450.0, 0.0), // Clamped to -360, then normalized to 0
        ];

        for (input, expected) in test_cases {
            let coords = MediaAreaCoordinates::new(0.0, 0.0, 100.0, 100.0, input, 0.0);
            assert_eq!(coords.rotation_angle(), expected, "Input: {}", input);
        }
    }

    #[test]
    fn test_angle_clamp_negative_beyond_min() {
        let coords = MediaAreaCoordinates::new(0.0, 0.0, 100.0, 100.0, -400.0, 0.0);
        assert_eq!(coords.rotation_angle(), 0.0); // -400 -> clamped to -360 -> normalized to 0
    }

    #[test]
    fn test_angle_clamp_positive_beyond_max() {
        let coords = MediaAreaCoordinates::new(0.0, 0.0, 100.0, 100.0, 400.0, 0.0);
        assert_eq!(coords.rotation_angle(), 360.0); // 400 > 360, clamped to 360
    }

    #[test]
    fn test_full_coverage_coords() {
        let coords = MediaAreaCoordinates::new(0.0, 0.0, 100.0, 100.0, 0.0, 0.0);
        assert!(coords.is_valid());
        assert_eq!(coords.x(), 0.0);
        assert_eq!(coords.y(), 0.0);
        assert_eq!(coords.width(), 100.0);
        assert_eq!(coords.height(), 100.0);
    }

    #[test]
    fn test_center_coords() {
        let coords = MediaAreaCoordinates::new(50.0, 50.0, 50.0, 50.0, 0.0, 0.0);
        assert!(coords.is_valid());
    }

    #[test]
    fn test_small_valid_dimensions() {
        let coords = MediaAreaCoordinates::new(50.0, 50.0, 0.01, 0.01, 0.0, 0.0);
        assert!(coords.is_valid());
    }

    #[test]
    fn test_max_radius() {
        let coords = MediaAreaCoordinates::new(50.0, 50.0, 50.0, 50.0, 0.0, 100.0);
        assert_eq!(coords.radius(), 100.0);
    }

    #[test]
    fn test_radius_clamping() {
        let coords = MediaAreaCoordinates::new(50.0, 50.0, 50.0, 50.0, 0.0, 150.0);
        assert_eq!(coords.radius(), 100.0);

        let coords2 = MediaAreaCoordinates::new(50.0, 50.0, 50.0, 50.0, 0.0, -10.0);
        assert_eq!(coords2.radius(), 0.0);
    }

    #[test]
    fn test_special_float_values_in_angle() {
        let coords = MediaAreaCoordinates::new(0.0, 0.0, 100.0, 100.0, f64::NAN, 0.0);
        // NaN should be handled (becomes 0.0 in fix_double)
        assert!(coords.rotation_angle().is_finite());
    }

    #[test]
    fn test_constants() {
        assert_eq!(MediaAreaCoordinates::MIN_VALUE, 0.0);
        assert_eq!(MediaAreaCoordinates::MAX_VALUE, 100.0);
        assert_eq!(MediaAreaCoordinates::MIN_ROTATION, -360.0);
        assert_eq!(MediaAreaCoordinates::MAX_ROTATION, 360.0);
        assert_eq!(MediaAreaCoordinates::EPSILON, 1e-6);
    }

    #[test]
    fn test_all_zero_except_dimensions() {
        let coords = MediaAreaCoordinates::new(0.0, 0.0, 50.0, 50.0, 0.0, 0.0);
        assert!(coords.is_valid());
        assert_eq!(coords.x(), 0.0);
        assert_eq!(coords.y(), 0.0);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let original = create_test_coords();
        let json = serde_json::to_string(&original).unwrap();
        let parsed: MediaAreaCoordinates = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_negative_width_clamped() {
        let coords = MediaAreaCoordinates::new(50.0, 50.0, -10.0, 25.0, 0.0, 0.0);
        assert_eq!(coords.width(), 0.0);
        assert!(!coords.is_valid());
    }

    #[test]
    fn test_negative_height_clamped() {
        let coords = MediaAreaCoordinates::new(50.0, 50.0, 25.0, -10.0, 0.0, 0.0);
        assert_eq!(coords.height(), 0.0);
        assert!(!coords.is_valid());
    }

    #[test]
    fn test_builder_chaining() {
        let coords = MediaAreaCoordinates::builder()
            .x(10.0)
            .x(20.0)
            .width(30.0)
            .width(40.0)
            .build();
        assert_eq!(coords.x(), 20.0);
        assert_eq!(coords.width(), 40.0);
    }
}
