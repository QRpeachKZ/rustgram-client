// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Sticker Mask Position
//!
//! Sticker mask position for Telegram.
//!
//! Based on TDLib's `StickerMaskPosition` from `td/telegram/StickerMaskPosition.h`.
//!
//! # Overview
//!
//! A `StickerMaskPosition` represents the position of a mask sticker on a face.
//!
//! # Example
//!
//! ```rust
//! use rustgram_sticker_mask_position::StickerMaskPosition;
//!
//! let position = StickerMaskPosition::new();
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Mask point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum MaskPoint {
    /// Forehead
    #[default]
    Forehead = 0,
    /// Eyes
    Eyes = 1,
    /// Mouth
    Mouth = 2,
    /// Chin
    Chin = 3,
}

/// Sticker mask position.
///
/// Represents the position of a mask sticker on a face.
///
/// # TDLib Mapping
///
/// TDLib: `StickerMaskPosition`
///
/// # Example
///
/// ```rust
/// use rustgram_sticker_mask_position::{StickerMaskPosition, MaskPoint};
///
/// let position = StickerMaskPosition::new(0.5, 0.5, 1.0);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct StickerMaskPosition {
    /// Point on face
    point: i32,
    /// X shift
    x_shift: f64,
    /// Y shift
    y_shift: f64,
    /// Scale
    scale: f64,
}

impl StickerMaskPosition {
    /// Creates a new mask position.
    ///
    /// # Arguments
    ///
    /// * `x_shift` - X shift (-1.0 to 1.0)
    /// * `y_shift` - Y shift (-1.0 to 1.0)
    /// * `scale` - Scale (> 0)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_sticker_mask_position::StickerMaskPosition;
    ///
    /// let position = StickerMaskPosition::new(0.5, 0.5, 1.0);
    /// ```
    #[must_use]
    pub fn new(x_shift: f64, y_shift: f64, scale: f64) -> Self {
        Self {
            point: -1,
            x_shift,
            y_shift,
            scale,
        }
    }

    /// Returns the point.
    #[must_use]
    pub const fn point(&self) -> i32 {
        self.point
    }

    /// Returns the x shift.
    #[must_use]
    pub const fn x_shift(&self) -> f64 {
        self.x_shift
    }

    /// Returns the y shift.
    #[must_use]
    pub const fn y_shift(&self) -> f64 {
        self.y_shift
    }

    /// Returns the scale.
    #[must_use]
    pub const fn scale(&self) -> f64 {
        self.scale
    }
}

impl fmt::Display for StickerMaskPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MaskPosition(x={}, y={}, scale={})",
            self.x_shift, self.y_shift, self.scale
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let position = StickerMaskPosition::new(0.5, 0.5, 1.0);
        assert_eq!(position.x_shift(), 0.5);
        assert_eq!(position.y_shift(), 0.5);
        assert_eq!(position.scale(), 1.0);
    }

    #[test]
    fn test_default() {
        let position = StickerMaskPosition::default();
        assert_eq!(position.point(), -1);
    }

    #[test]
    fn test_display() {
        let position = StickerMaskPosition::new(0.5, 0.5, 1.0);
        assert!(format!("{position}").contains("MaskPosition"));
    }

    #[test]
    fn test_equality() {
        let pos1 = StickerMaskPosition::new(0.5, 0.5, 1.0);
        let pos2 = StickerMaskPosition::new(0.5, 0.5, 1.0);
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_clone() {
        let pos1 = StickerMaskPosition::new(0.5, 0.5, 1.0);
        let pos2 = pos1.clone();
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_serialization() {
        let position = StickerMaskPosition::new(0.5, 0.5, 1.0);
        let json = serde_json::to_string(&position).expect("Failed to serialize");
        let deserialized: StickerMaskPosition =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, position);
    }
}
