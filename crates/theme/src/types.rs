// Copyright 2024 rustgram-client Authors
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

//! Base theme types for Telegram UI.
//!
//! This module provides the [`BaseTheme`] enum representing built-in themes
//! available in Telegram clients.
//!
//! # Example
//!
//! ```rust
//! use theme::BaseTheme;
//!
//! let theme = BaseTheme::Night;
//! assert!(theme.is_dark());
//! ```

use serde::{Deserialize, Serialize};

/// Built-in base themes available in Telegram.
///
/// These themes are the foundational themes that can be customized
/// with accent colors and other settings.
///
/// # TL Mapping
///
/// | Variant | Telegram API | TD API |
/// |---------|--------------|--------|
/// | [`Classic`](BaseTheme::Classic) | `baseThemeClassic` | `builtInThemeClassic` |
/// | [`Day`](BaseTheme::Day) | `baseThemeDay` | `builtInThemeDay` |
/// | [`Night`](BaseTheme::Night) | `baseThemeNight` | `builtInThemeNight` |
/// | [`Tinted`](BaseTheme::Tinted) | `baseThemeTinted` | `builtInThemeTinted` |
/// | [`Arctic`](BaseTheme::Arctic) | `baseThemeArctic` | `builtInThemeArctic` |
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum BaseTheme {
    /// Classic theme (light)
    Classic = 0,

    /// Regular light theme
    Day = 1,

    /// Regular dark theme
    Night = 2,

    /// Tinted dark theme
    Tinted = 3,

    /// Arctic light theme
    Arctic = 4,
}

impl BaseTheme {
    /// Returns `true` if this is a dark theme.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use theme::BaseTheme;
    ///
    /// assert!(!BaseTheme::Classic.is_dark());
    /// assert!(!BaseTheme::Day.is_dark());
    /// assert!(BaseTheme::Night.is_dark());
    /// assert!(BaseTheme::Tinted.is_dark());
    /// assert!(!BaseTheme::Arctic.is_dark());
    /// ```
    #[must_use]
    pub const fn is_dark(self) -> bool {
        matches!(self, Self::Night | Self::Tinted)
    }

    /// Returns `true` if this is a light theme.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use theme::BaseTheme;
    ///
    /// assert!(BaseTheme::Classic.is_light());
    /// assert!(BaseTheme::Day.is_light());
    /// assert!(!BaseTheme::Night.is_light());
    /// ```
    #[must_use]
    pub const fn is_light(self) -> bool {
        !self.is_dark()
    }
}

impl Default for BaseTheme {
    /// Returns the default theme (Classic).
    fn default() -> Self {
        Self::Classic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dark() {
        assert!(!BaseTheme::Classic.is_dark());
        assert!(!BaseTheme::Day.is_dark());
        assert!(BaseTheme::Night.is_dark());
        assert!(BaseTheme::Tinted.is_dark());
        assert!(!BaseTheme::Arctic.is_dark());
    }

    #[test]
    fn test_is_light() {
        assert!(BaseTheme::Classic.is_light());
        assert!(BaseTheme::Day.is_light());
        assert!(!BaseTheme::Night.is_light());
        assert!(!BaseTheme::Tinted.is_light());
        assert!(BaseTheme::Arctic.is_light());
    }

    #[test]
    fn test_default() {
        assert_eq!(BaseTheme::default(), BaseTheme::Classic);
    }
}
