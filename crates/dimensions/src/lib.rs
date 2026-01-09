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

//! # Dimensions
//!
//! Represents image dimensions (width x height) with validation.
//!
//! Dimensions are validated to be within the range [0, 65535] for both width and height.
//! If either dimension is invalid, both dimensions are set to 0.

use serde::{Deserialize, Serialize};

/// Maximum valid dimension value (u16::MAX).
pub const MAX_DIMENSION: u32 = 65535;

/// Represents image dimensions with validation.
///
/// # Example
///
/// ```rust
/// use rustgram_dimensions::Dimensions;
///
/// // Create valid dimensions
/// let dims = Dimensions::from_wh(1920, 1080);
/// assert_eq!(dims.width(), 1920);
/// assert_eq!(dims.height(), 1080);
/// assert!(dims.is_valid());
///
/// // Invalid dimensions are clamped to (0, 0)
/// let invalid = Dimensions::from_wh(-1, 100000);
/// assert_eq!(invalid.width(), 0);
/// assert_eq!(invalid.height(), 0);
/// assert!(!invalid.is_valid());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Dimensions {
    width: u16,
    height: u16,
}

impl Dimensions {
    /// Creates dimensions from width and height values.
    ///
    /// # Arguments
    ///
    /// * `width` - Width in pixels (will be clamped to [0, 65535])
    /// * `height` - Height in pixels (will be clamped to [0, 65535])
    ///
    /// # Returns
    ///
    /// Returns validated dimensions. If either width or height is invalid,
    /// both dimensions are set to 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dimensions::Dimensions;
    ///
    /// let dims = Dimensions::from_wh(1920, 1080);
    /// assert_eq!(dims.width(), 1920);
    /// ```
    pub fn from_wh(width: i32, height: i32) -> Self {
        let valid_width = Self::validate_dimension(width);
        let valid_height = Self::validate_dimension(height);

        // If either dimension is invalid, both are set to 0
        match (valid_width, valid_height) {
            (Some(w), Some(h)) => Self {
                width: w,
                height: h,
            },
            _ => Self {
                width: 0,
                height: 0,
            },
        }
    }

    /// Validates a single dimension value.
    ///
    /// # Arguments
    ///
    /// * `size` - Dimension size to validate
    ///
    /// # Returns
    ///
    /// Returns `Some(u16)` if valid, `None` if out of range.
    fn validate_dimension(size: i32) -> Option<u16> {
        if size < 0 || size > MAX_DIMENSION as i32 {
            None
        } else {
            Some(size as u16)
        }
    }

    /// Returns the width in pixels.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dimensions::Dimensions;
    ///
    /// let dims = Dimensions::from_wh(1920, 1080);
    /// assert_eq!(dims.width(), 1920);
    /// ```
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Returns the height in pixels.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dimensions::Dimensions;
    ///
    /// let dims = Dimensions::from_wh(1920, 1080);
    /// assert_eq!(dims.height(), 1080);
    /// ```
    pub fn height(&self) -> u16 {
        self.height
    }

    /// Returns the total pixel count (width * height).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dimensions::Dimensions;
    ///
    /// let dims = Dimensions::from_wh(1920, 1080);
    /// assert_eq!(dims.pixel_count(), 1920 * 1080);
    /// ```
    pub fn pixel_count(&self) -> u32 {
        (self.width as u32) * (self.height as u32)
    }

    /// Returns the aspect ratio (width / height) as a float.
    ///
    /// Returns `None` if height is 0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dimensions::Dimensions;
    ///
    /// let dims = Dimensions::from_wh(1920, 1080);
    /// assert!((dims.aspect_ratio().unwrap() - 16.0 / 9.0).abs() < 0.01);
    /// ```
    pub fn aspect_ratio(&self) -> Option<f64> {
        if self.height == 0 {
            None
        } else {
            Some(self.width as f64 / self.height as f64)
        }
    }

    /// Returns `true` if both dimensions are non-zero.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dimensions::Dimensions;
    ///
    /// let dims = Dimensions::from_wh(1920, 1080);
    /// assert!(dims.is_valid());
    ///
    /// let invalid = Dimensions::from_wh(0, 0);
    /// assert!(!invalid.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }

    /// Returns `true` if dimensions are square (width == height).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dimensions::Dimensions;
    ///
    /// let square = Dimensions::from_wh(1080, 1080);
    /// assert!(square.is_square());
    ///
    /// let rect = Dimensions::from_wh(1920, 1080);
    /// assert!(!rect.is_square());
    /// ```
    pub fn is_square(&self) -> bool {
        self.width == self.height && self.width > 0
    }

    /// Returns `true` if width is greater than height (landscape).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dimensions::Dimensions;
    ///
    /// let landscape = Dimensions::from_wh(1920, 1080);
    /// assert!(landscape.is_landscape());
    /// ```
    pub fn is_landscape(&self) -> bool {
        self.width > self.height
    }

    /// Returns `true` if height is greater than width (portrait).
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_dimensions::Dimensions;
    ///
    /// let portrait = Dimensions::from_wh(1080, 1920);
    /// assert!(portrait.is_portrait());
    /// ```
    pub fn is_portrait(&self) -> bool {
        self.height > self.width
    }
}

impl std::fmt::Display for Dimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_wh_valid() {
        let dims = Dimensions::from_wh(1920, 1080);
        assert_eq!(dims.width(), 1920);
        assert_eq!(dims.height(), 1080);
        assert!(dims.is_valid());
    }

    #[test]
    fn test_from_wh_zero() {
        let dims = Dimensions::from_wh(0, 0);
        assert_eq!(dims.width(), 0);
        assert_eq!(dims.height(), 0);
        assert!(!dims.is_valid());
    }

    #[test]
    fn test_from_wh_negative() {
        let dims = Dimensions::from_wh(-1, 1080);
        assert_eq!(dims.width(), 0);
        assert_eq!(dims.height(), 0);
        assert!(!dims.is_valid());
    }

    #[test]
    fn test_from_wh_too_large() {
        let dims = Dimensions::from_wh(1920, 100000);
        assert_eq!(dims.width(), 0);
        assert_eq!(dims.height(), 0);
        assert!(!dims.is_valid());
    }

    #[test]
    fn test_from_wh_max_valid() {
        let dims = Dimensions::from_wh(65535, 65535);
        assert_eq!(dims.width(), 65535);
        assert_eq!(dims.height(), 65535);
        assert!(dims.is_valid());
    }

    #[test]
    fn test_pixel_count() {
        let dims = Dimensions::from_wh(1920, 1080);
        assert_eq!(dims.pixel_count(), 1920 * 1080);
    }

    #[test]
    fn test_pixel_count_zero() {
        let dims = Dimensions::from_wh(0, 0);
        assert_eq!(dims.pixel_count(), 0);
    }

    #[test]
    fn test_aspect_ratio() {
        let dims = Dimensions::from_wh(1920, 1080);
        let ratio = dims.aspect_ratio().unwrap();
        assert!((ratio - 16.0 / 9.0).abs() < 0.01);
    }

    #[test]
    fn test_aspect_ratio_zero_height() {
        let dims = Dimensions::from_wh(1920, 0);
        assert!(dims.aspect_ratio().is_none());
    }

    #[test]
    fn test_is_valid() {
        let valid = Dimensions::from_wh(100, 100);
        assert!(valid.is_valid());

        let zero_width = Dimensions::from_wh(0, 100);
        assert!(!zero_width.is_valid());

        let zero_height = Dimensions::from_wh(100, 0);
        assert!(!zero_height.is_valid());
    }

    #[test]
    fn test_is_square() {
        let square = Dimensions::from_wh(1080, 1080);
        assert!(square.is_square());

        let rect = Dimensions::from_wh(1920, 1080);
        assert!(!rect.is_square());

        let zero = Dimensions::from_wh(0, 0);
        assert!(!zero.is_square());
    }

    #[test]
    fn test_is_landscape() {
        let landscape = Dimensions::from_wh(1920, 1080);
        assert!(landscape.is_landscape());
        assert!(!landscape.is_portrait());
    }

    #[test]
    fn test_is_portrait() {
        let portrait = Dimensions::from_wh(1080, 1920);
        assert!(portrait.is_portrait());
        assert!(!portrait.is_landscape());
    }

    #[test]
    fn test_neither_landscape_nor_portrait() {
        let square = Dimensions::from_wh(1080, 1080);
        assert!(!square.is_landscape());
        assert!(!square.is_portrait());
    }

    #[test]
    fn test_equality() {
        let dims1 = Dimensions::from_wh(1920, 1080);
        let dims2 = Dimensions::from_wh(1920, 1080);
        assert_eq!(dims1, dims2);

        let dims3 = Dimensions::from_wh(1080, 1920);
        assert_ne!(dims1, dims3);
    }

    #[test]
    fn test_default() {
        let dims = Dimensions::default();
        assert_eq!(dims.width(), 0);
        assert_eq!(dims.height(), 0);
        assert!(!dims.is_valid());
    }

    #[test]
    fn test_copy() {
        let dims1 = Dimensions::from_wh(1920, 1080);
        let dims2 = dims1;
        assert_eq!(dims1.width(), dims2.width());
    }

    #[test]
    fn test_clone() {
        let dims1 = Dimensions::from_wh(1920, 1080);
        let dims2 = dims1.clone();
        assert_eq!(dims1, dims2);
    }

    #[test]
    fn test_display() {
        let dims = Dimensions::from_wh(1920, 1080);
        assert_eq!(format!("{}", dims), "(1920, 1080)");
    }

    #[test]
    fn test_serialization() {
        let dims = Dimensions::from_wh(1920, 1080);
        let json = serde_json::to_string(&dims).unwrap();
        let parsed: Dimensions = serde_json::from_str(&json).unwrap();
        assert_eq!(dims, parsed);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let dims1 = Dimensions::from_wh(1920, 1080);
        let dims2 = Dimensions::from_wh(1920, 1080);

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        dims1.hash(&mut hasher1);
        dims2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_common_resolutions() {
        let test_cases = [
            (1920, 1080), // Full HD
            (1280, 720),  // HD
            (3840, 2160), // 4K
            (2560, 1440), // 2K
            (640, 480),   // VGA
            (320, 240),   // QVGA
        ];

        for (width, height) in test_cases {
            let dims = Dimensions::from_wh(width, height);
            assert_eq!(dims.width(), width as u16);
            assert_eq!(dims.height(), height as u16);
            assert!(dims.is_valid());
        }
    }

    #[test]
    fn test_small_dimensions() {
        let dims = Dimensions::from_wh(1, 1);
        assert_eq!(dims.width(), 1);
        assert_eq!(dims.height(), 1);
        assert!(dims.is_valid());
        assert!(dims.is_square());
    }

    #[test]
    fn test_boundary_values() {
        // Test maximum valid value
        let max = Dimensions::from_wh(65535, 65535);
        assert_eq!(max.width(), 65535);
        assert_eq!(max.height(), 65535);
        assert!(max.is_valid());

        // Test just above maximum
        let overflow = Dimensions::from_wh(65536, 65535);
        assert!(!overflow.is_valid());
    }

    #[test]
    fn test_aspect_ratio_common() {
        let test_cases = [
            (1920, 1080, 16.0 / 9.0),
            (1280, 720, 16.0 / 9.0),
            (3840, 2160, 16.0 / 9.0),
            (1080, 1920, 9.0 / 16.0),
            (1080, 1080, 1.0),
            (800, 600, 4.0 / 3.0),
        ];

        for (width, height, expected) in test_cases {
            let dims = Dimensions::from_wh(width, height);
            let ratio = dims.aspect_ratio().unwrap();
            assert!((ratio - expected).abs() < 0.001);
        }
    }

    #[test]
    fn test_debug_format() {
        let dims = Dimensions::from_wh(1920, 1080);
        let debug_str = format!("{:?}", dims);
        assert!(debug_str.contains("Dimensions"));
        assert!(debug_str.contains("1920"));
        assert!(debug_str.contains("1080"));
    }
}
