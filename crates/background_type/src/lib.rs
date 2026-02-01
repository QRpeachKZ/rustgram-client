//! # Rustgram BackgroundType
//!
//! Background type handling for Telegram MTProto client.
//!
//! This crate provides types for managing chat backgrounds including:
//! - Wallpaper backgrounds (with blur/motion effects)
//! - Pattern backgrounds (with intensity and fill)
//! - Solid/gradient/freeform fills
//! - Chat theme backgrounds
//!
//! ## Overview
//!
//! - [`BackgroundType`] - Main background type enumeration
//! - [`BackgroundFill`] - Background fill (solid, gradient, freeform)
//! - [`FillType`] - Type of fill (Solid, Gradient, FreeformGradient)
//!
//! ## Examples
//!
//! Create a wallpaper background:
//!
//! ```
//! use rustgram_background_type::BackgroundType;
//!
//! let wallpaper = BackgroundType::wallpaper(true, false, 50);
//! assert!(wallpaper.has_file());
//! assert_eq!(wallpaper.mime_type(), "image/jpeg");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use std::fmt;

/// Background fill type.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FillType {
    /// Solid color fill
    Solid,
    /// Gradient fill with two colors
    Gradient,
    /// Freeform gradient with 3-4 colors
    FreeformGradient,
}

/// Background fill configuration.
///
/// Defines the colors and pattern for background fills.
///
/// # Examples
///
/// ```
/// use rustgram_background_type::BackgroundFill;
///
/// let solid = BackgroundFill::solid(0xFF5733);
/// assert_eq!(solid.fill_type(), rustgram_background_type::FillType::Solid);
///
/// let gradient = BackgroundFill::gradient(0xFF5733, 0x33FF57, 45);
/// assert_eq!(gradient.fill_type(), rustgram_background_type::FillType::Gradient);
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BackgroundFill {
    top_color: i32,
    bottom_color: i32,
    rotation_angle: i32,
    third_color: i32,
    fourth_color: i32,
}

impl Default for BackgroundFill {
    fn default() -> Self {
        Self::solid(0)
    }
}

impl BackgroundFill {
    /// Creates a solid color fill.
    ///
    /// # Arguments
    ///
    /// * `color` - RGB color value (0x000000 to 0xFFFFFF)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_type::BackgroundFill;
    ///
    /// let fill = BackgroundFill::solid(0xFF5733);
    /// ```
    #[inline]
    #[must_use]
    pub const fn solid(color: i32) -> Self {
        Self {
            top_color: color,
            bottom_color: color,
            rotation_angle: 0,
            third_color: -1,
            fourth_color: -1,
        }
    }

    /// Creates a gradient fill with two colors.
    ///
    /// # Arguments
    ///
    /// * `top_color` - Top RGB color value
    /// * `bottom_color` - Bottom RGB color value
    /// * `rotation_angle` - Rotation angle in degrees (0-315, multiples of 45)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_type::BackgroundFill;
    ///
    /// let fill = BackgroundFill::gradient(0xFF5733, 0x33FF57, 45);
    /// ```
    #[inline]
    #[must_use]
    pub const fn gradient(top_color: i32, bottom_color: i32, rotation_angle: i32) -> Self {
        Self {
            top_color,
            bottom_color,
            rotation_angle,
            third_color: -1,
            fourth_color: -1,
        }
    }

    /// Creates a freeform gradient with 3-4 colors.
    ///
    /// # Arguments
    ///
    /// * `colors` - Array of 3 or 4 RGB color values
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_type::BackgroundFill;
    ///
    /// let fill = BackgroundFill::freeform(&[0xFF5733, 0x33FF57, 0x3357FF, 0xFFFF00]);
    /// ```
    #[must_use]
    pub const fn freeform(colors: &[i32; 4]) -> Self {
        Self {
            top_color: colors[0],
            bottom_color: colors[1],
            rotation_angle: 0,
            third_color: colors[2],
            fourth_color: colors[3],
        }
    }

    /// Returns the fill type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_type::{BackgroundFill, FillType};
    ///
    /// let fill = BackgroundFill::solid(0xFF5733);
    /// assert_eq!(fill.fill_type(), FillType::Solid);
    /// ```
    #[inline]
    #[must_use]
    pub fn fill_type(&self) -> FillType {
        if self.third_color != -1 {
            FillType::FreeformGradient
        } else if self.top_color == self.bottom_color {
            FillType::Solid
        } else {
            FillType::Gradient
        }
    }

    /// Returns the top color.
    #[inline]
    #[must_use]
    pub const fn top_color(&self) -> i32 {
        self.top_color
    }

    /// Returns the bottom color.
    #[inline]
    #[must_use]
    pub const fn bottom_color(&self) -> i32 {
        self.bottom_color
    }

    /// Returns the rotation angle (for gradient fills).
    #[inline]
    #[must_use]
    pub const fn rotation_angle(&self) -> i32 {
        self.rotation_angle
    }

    /// Returns the third color (for freeform gradients).
    #[inline]
    #[must_use]
    pub const fn third_color(&self) -> i32 {
        self.third_color
    }

    /// Returns the fourth color (for freeform gradients).
    #[inline]
    #[must_use]
    pub const fn fourth_color(&self) -> i32 {
        self.fourth_color
    }

    /// Checks if this is a dark background.
    ///
    /// # Returns
    ///
    /// `true` if the background colors are dark
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_type::BackgroundFill;
    ///
    /// let dark = BackgroundFill::solid(0x333333);
    /// assert!(dark.is_dark());
    ///
    /// let light = BackgroundFill::solid(0xFFFFFF);
    /// assert!(!light.is_dark());
    /// ```
    #[must_use]
    pub fn is_dark(&self) -> bool {
        match self.fill_type() {
            FillType::Solid => (self.top_color & 0x808080) == 0,
            FillType::Gradient => {
                (self.top_color & 0x808080) == 0 && (self.bottom_color & 0x808080) == 0
            }
            FillType::FreeformGradient => {
                (self.top_color & 0x808080) == 0
                    && (self.bottom_color & 0x808080) == 0
                    && (self.third_color & 0x808080) == 0
                    && (self.fourth_color == -1 || (self.fourth_color & 0x808080) == 0)
            }
        }
    }
}

/// Background type enumeration.
///
/// Represents the type of chat background.
///
/// # Examples
///
/// ```
/// use rustgram_background_type::BackgroundType;
///
/// let wallpaper = BackgroundType::wallpaper(false, false, 0);
/// assert!(wallpaper.has_file());
///
/// let fill = BackgroundType::fill(rustgram_background_type::BackgroundFill::solid(0xFF0000), 0);
/// assert!(!fill.has_file());
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BackgroundType {
    /// Wallpaper with optional blur and motion effects
    Wallpaper {
        /// Whether the wallpaper is blurred
        is_blurred: bool,
        /// Whether the wallpaper is animated (moving)
        is_moving: bool,
        /// Dark theme dimming (0-100)
        dark_theme_dimming: i32,
    },
    /// Pattern with fill and intensity
    Pattern {
        /// Whether the pattern is animated
        is_moving: bool,
        /// Background fill
        fill: BackgroundFill,
        /// Intensity (-100 to 100, negative for inverted)
        intensity: i32,
    },
    /// Solid/gradient/freeform fill only
    Fill {
        /// Background fill
        fill: BackgroundFill,
        /// Dark theme dimming (0-100)
        dark_theme_dimming: i32,
    },
    /// Chat theme (emoticon-based)
    ChatTheme {
        /// Theme name/emoticon
        theme_name: String,
    },
}

impl Default for BackgroundType {
    fn default() -> Self {
        Self::Fill {
            fill: BackgroundFill::default(),
            dark_theme_dimming: 0,
        }
    }
}

impl BackgroundType {
    /// Creates a wallpaper background type.
    ///
    /// # Arguments
    ///
    /// * `is_blurred` - Whether the wallpaper is blurred
    /// * `is_moving` - Whether the wallpaper is animated
    /// * `dark_theme_dimming` - Dark theme dimming value (0-100)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_type::BackgroundType;
    ///
    /// let wallpaper = BackgroundType::wallpaper(true, false, 50);
    /// assert!(wallpaper.has_file());
    /// ```
    #[inline]
    #[must_use]
    pub fn wallpaper(is_blurred: bool, is_moving: bool, dark_theme_dimming: i32) -> Self {
        Self::Wallpaper {
            is_blurred,
            is_moving,
            dark_theme_dimming,
        }
    }

    /// Creates a pattern background type.
    ///
    /// # Arguments
    ///
    /// * `is_moving` - Whether the pattern is animated
    /// * `fill` - Background fill
    /// * `intensity` - Pattern intensity (-100 to 100)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_type::{BackgroundType, BackgroundFill};
    ///
    /// let pattern = BackgroundType::pattern(true, BackgroundFill::solid(0xFF0000), 50);
    /// assert!(pattern.has_file());
    /// ```
    #[inline]
    #[must_use]
    pub fn pattern(is_moving: bool, fill: BackgroundFill, intensity: i32) -> Self {
        Self::Pattern {
            is_moving,
            fill,
            intensity,
        }
    }

    /// Creates a fill background type.
    ///
    /// # Arguments
    ///
    /// * `fill` - Background fill
    /// * `dark_theme_dimming` - Dark theme dimming (0-100)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_type::{BackgroundType, BackgroundFill};
    ///
    /// let fill = BackgroundType::fill(BackgroundFill::solid(0xFF0000), 0);
    /// assert!(!fill.has_file());
    /// ```
    #[inline]
    #[must_use]
    pub fn fill(fill: BackgroundFill, dark_theme_dimming: i32) -> Self {
        Self::Fill {
            fill,
            dark_theme_dimming,
        }
    }

    /// Creates a chat theme background type.
    ///
    /// # Arguments
    ///
    /// * `theme_name` - Theme name/emoticon
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_type::BackgroundType;
    ///
    /// let theme = BackgroundType::chat_theme("blue".to_string());
    /// assert!(!theme.has_file());
    /// ```
    #[inline]
    #[must_use]
    pub fn chat_theme(theme_name: String) -> Self {
        Self::ChatTheme { theme_name }
    }

    /// Checks if this background type has an associated file.
    ///
    /// # Returns
    ///
    /// `true` for Wallpaper and Pattern types
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_type::{BackgroundType, BackgroundFill};
    ///
    /// assert!(BackgroundType::wallpaper(false, false, 0).has_file());
    /// assert!(BackgroundType::pattern(false, BackgroundFill::solid(0), 0).has_file());
    /// assert!(!BackgroundType::fill(BackgroundFill::solid(0), 0).has_file());
    /// ```
    #[must_use]
    pub fn has_file(&self) -> bool {
        matches!(self, Self::Wallpaper { .. } | Self::Pattern { .. })
    }

    /// Checks if this is a gradient fill background.
    ///
    /// # Returns
    ///
    /// `true` if Fill type with non-solid fill
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_type::{BackgroundType, BackgroundFill};
    ///
    /// let solid = BackgroundType::fill(BackgroundFill::solid(0xFF0000), 0);
    /// assert!(!solid.has_gradient_fill());
    ///
    /// let gradient = BackgroundType::fill(BackgroundFill::gradient(0xFF0000, 0x00FF00, 45), 0);
    /// assert!(gradient.has_gradient_fill());
    /// ```
    #[must_use]
    pub fn has_gradient_fill(&self) -> bool {
        if let Self::Fill { fill, .. } = self {
            fill.fill_type() != FillType::Solid
        } else {
            false
        }
    }

    /// Returns the MIME type for file-based backgrounds.
    ///
    /// # Returns
    ///
    /// - `"image/jpeg"` for Wallpaper
    /// - `"image/png"` for Pattern
    ///
    /// # Panics
    ///
    /// Panics if called on non-file backgrounds
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_type::BackgroundType;
    ///
    /// let wallpaper = BackgroundType::wallpaper(false, false, 0);
    /// assert_eq!(wallpaper.mime_type(), "image/jpeg");
    /// ```
    #[must_use]
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Pattern { .. } => "image/png",
            Self::Wallpaper { .. } => "image/jpeg",
            _ => panic!("mime_type() called on non-file background"),
        }
    }

    /// Returns the dark theme dimming value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_type::{BackgroundType, BackgroundFill};
    ///
    /// let fill = BackgroundType::fill(BackgroundFill::solid(0), 75);
    /// assert_eq!(fill.dark_theme_dimming(), 75);
    /// ```
    #[must_use]
    pub fn dark_theme_dimming(&self) -> i32 {
        match self {
            Self::Wallpaper {
                dark_theme_dimming, ..
            } => *dark_theme_dimming,
            Self::Fill {
                dark_theme_dimming, ..
            } => *dark_theme_dimming,
            _ => 0,
        }
    }

    /// Checks if the background is dark.
    ///
    /// # Returns
    ///
    /// `true` if the fill is dark
    ///
    /// # Panics
    ///
    /// Panics if called on non-Fill backgrounds
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_background_type::{BackgroundType, BackgroundFill};
    ///
    /// let dark = BackgroundType::fill(BackgroundFill::solid(0x333333), 0);
    /// assert!(dark.is_dark());
    /// ```
    #[must_use]
    pub fn is_dark(&self) -> bool {
        match self {
            Self::Fill { fill, .. } => fill.is_dark(),
            _ => false,
        }
    }
}

impl fmt::Display for BackgroundType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Wallpaper {
                is_blurred,
                is_moving,
                ..
            } => write!(f, "Wallpaper(blurred={}, moving={})", is_blurred, is_moving),
            Self::Pattern {
                is_moving,
                intensity,
                ..
            } => write!(f, "Pattern(moving={}, intensity={})", is_moving, intensity),
            Self::Fill { .. } => write!(f, "Fill"),
            Self::ChatTheme { theme_name } => write!(f, "ChatTheme({})", theme_name),
        }
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-background-type";

#[cfg(test)]
mod tests {
    use super::*;

    // ========== BackgroundFill Tests ==========

    #[test]
    fn test_fill_solid() {
        let fill = BackgroundFill::solid(0xFF5733);
        assert_eq!(fill.fill_type(), FillType::Solid);
        assert_eq!(fill.top_color(), 0xFF5733);
        assert_eq!(fill.bottom_color(), 0xFF5733);
    }

    #[test]
    fn test_fill_gradient() {
        let fill = BackgroundFill::gradient(0xFF5733, 0x33FF57, 45);
        assert_eq!(fill.fill_type(), FillType::Gradient);
        assert_eq!(fill.top_color(), 0xFF5733);
        assert_eq!(fill.bottom_color(), 0x33FF57);
        assert_eq!(fill.rotation_angle(), 45);
    }

    #[test]
    fn test_fill_freeform_three_colors() {
        let colors = [0xFF5733, 0x33FF57, 0x3357FF, -1];
        let fill = BackgroundFill::freeform(&colors);
        assert_eq!(fill.fill_type(), FillType::FreeformGradient);
        assert_eq!(fill.top_color(), 0xFF5733);
        assert_eq!(fill.bottom_color(), 0x33FF57);
        assert_eq!(fill.third_color(), 0x3357FF);
    }

    #[test]
    fn test_fill_freeform_four_colors() {
        let colors = [0xFF5733, 0x33FF57, 0x3357FF, 0xFFFF00];
        let fill = BackgroundFill::freeform(&colors);
        assert_eq!(fill.fill_type(), FillType::FreeformGradient);
        assert_eq!(fill.fourth_color(), 0xFFFF00);
    }

    #[test]
    fn test_fill_is_dark() {
        let dark = BackgroundFill::solid(0x333333);
        assert!(dark.is_dark());

        let light = BackgroundFill::solid(0xFFFFFF);
        assert!(!light.is_dark());

        let dark_gradient = BackgroundFill::gradient(0x333333, 0x222222, 0);
        assert!(dark_gradient.is_dark());

        let light_gradient = BackgroundFill::gradient(0xAAAAAA, 0xBBBBBB, 0);
        assert!(!light_gradient.is_dark());
    }

    #[test]
    fn test_fill_default() {
        let fill = BackgroundFill::default();
        assert_eq!(fill.top_color(), 0);
        assert_eq!(fill.fill_type(), FillType::Solid);
    }

    // ========== BackgroundType Constructor Tests ==========

    #[test]
    fn test_wallpaper() {
        let bg = BackgroundType::wallpaper(true, false, 50);
        assert!(bg.has_file());
        assert_eq!(bg.mime_type(), "image/jpeg");
        assert_eq!(bg.dark_theme_dimming(), 50);
    }

    #[test]
    fn test_pattern() {
        let fill = BackgroundFill::solid(0xFF0000);
        let bg = BackgroundType::pattern(true, fill, 75);
        assert!(bg.has_file());
        assert_eq!(bg.mime_type(), "image/png");
    }

    #[test]
    fn test_fill() {
        let fill = BackgroundFill::solid(0xFF0000);
        let bg = BackgroundType::fill(fill, 25);
        assert!(!bg.has_file());
        assert_eq!(bg.dark_theme_dimming(), 25);
    }

    #[test]
    fn test_chat_theme() {
        let bg = BackgroundType::chat_theme("blue".to_string());
        assert!(!bg.has_file());
        assert_eq!(bg.dark_theme_dimming(), 0);
    }

    #[test]
    fn test_default() {
        let bg = BackgroundType::default();
        assert!(!bg.has_file());
    }

    // ========== has_file Tests ==========

    #[test]
    fn test_has_file_wallpaper() {
        assert!(BackgroundType::wallpaper(false, false, 0).has_file());
    }

    #[test]
    fn test_has_file_pattern() {
        let fill = BackgroundFill::solid(0);
        assert!(BackgroundType::pattern(false, fill, 0).has_file());
    }

    #[test]
    fn test_has_file_fill() {
        let fill = BackgroundFill::solid(0);
        assert!(!BackgroundType::fill(fill, 0).has_file());
    }

    #[test]
    fn test_has_file_chat_theme() {
        assert!(!BackgroundType::chat_theme("test".to_string()).has_file());
    }

    // ========== has_gradient_fill Tests ==========

    #[test]
    fn test_has_gradient_fill_solid() {
        let fill = BackgroundFill::solid(0xFF0000);
        let bg = BackgroundType::fill(fill, 0);
        assert!(!bg.has_gradient_fill());
    }

    #[test]
    fn test_has_gradient_fill_gradient() {
        let fill = BackgroundFill::gradient(0xFF0000, 0x00FF00, 45);
        let bg = BackgroundType::fill(fill, 0);
        assert!(bg.has_gradient_fill());
    }

    #[test]
    fn test_has_gradient_fill_freeform() {
        let colors = [0xFF0000, 0x00FF00, 0x0000FF, -1];
        let fill = BackgroundFill::freeform(&colors);
        let bg = BackgroundType::fill(fill, 0);
        assert!(bg.has_gradient_fill());
    }

    // ========== is_dark Tests ==========

    #[test]
    fn test_is_dark_fill() {
        let dark_fill = BackgroundFill::solid(0x333333);
        let dark_bg = BackgroundType::fill(dark_fill, 0);
        assert!(dark_bg.is_dark());

        let light_fill = BackgroundFill::solid(0xAAAAAA);
        let light_bg = BackgroundType::fill(light_fill, 0);
        assert!(!light_bg.is_dark());
    }

    // ========== Equality Tests ==========

    #[test]
    fn test_equality_wallpaper() {
        let bg1 = BackgroundType::wallpaper(true, false, 50);
        let bg2 = BackgroundType::wallpaper(true, false, 50);
        assert_eq!(bg1, bg2);
    }

    #[test]
    fn test_equality_pattern() {
        let fill = BackgroundFill::solid(0xFF0000);
        let bg1 = BackgroundType::pattern(true, fill.clone(), 50);
        let bg2 = BackgroundType::pattern(true, fill, 50);
        assert_eq!(bg1, bg2);
    }

    #[test]
    fn test_inequality_different_type() {
        let bg1 = BackgroundType::wallpaper(false, false, 0);
        let bg2 = BackgroundType::fill(BackgroundFill::solid(0), 0);
        assert_ne!(bg1, bg2);
    }

    #[test]
    fn test_fill_equality() {
        let fill1 = BackgroundFill::solid(0xFF5733);
        let fill2 = BackgroundFill::solid(0xFF5733);
        assert_eq!(fill1, fill2);
    }

    #[test]
    fn test_fill_inequality() {
        let fill1 = BackgroundFill::solid(0xFF5733);
        let fill2 = BackgroundFill::solid(0x33FF57);
        assert_ne!(fill1, fill2);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_wallpaper() {
        let bg = BackgroundType::wallpaper(true, false, 50);
        let s = format!("{}", bg);
        assert!(s.contains("Wallpaper"));
        assert!(s.contains("blurred=true"));
    }

    #[test]
    fn test_display_chat_theme() {
        let bg = BackgroundType::chat_theme("blue".to_string());
        let s = format!("{}", bg);
        assert!(s.contains("ChatTheme"));
        assert!(s.contains("blue"));
    }

    // ========== Clone Tests ==========

    #[test]
    fn test_clone_background_type() {
        let fill = BackgroundFill::gradient(0xFF0000, 0x00FF00, 45);
        let bg1 = BackgroundType::pattern(true, fill, 50);
        let bg2 = bg1.clone();
        assert_eq!(bg1, bg2);
    }

    #[test]
    fn test_clone_fill() {
        let fill1 = BackgroundFill::freeform(&[1, 2, 3, 4]);
        let fill2 = fill1.clone();
        assert_eq!(fill1, fill2);
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-background-type");
    }
}
