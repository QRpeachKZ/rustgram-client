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

//! # Theme Module
//!
//! This module provides theme-related types for Telegram clients.
//!
//! ## Overview
//!
//! The theme module defines [`BaseTheme`] enum representing built-in themes
//! available in Telegram, along with conversion functions for Telegram API types.
//!
//! ## Examples
//!
//! ```rust
//! use theme::BaseTheme;
//!
//! // Create a theme
//! let theme = BaseTheme::Night;
//!
//! // Check if it's dark
//! assert!(theme.is_dark());
//!
//! // Get all built-in themes
//! let themes = [
//!     BaseTheme::Classic,
//!     BaseTheme::Day,
//!     BaseTheme::Night,
//!     BaseTheme::Tinted,
//!     BaseTheme::Arctic,
//! ];
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

pub mod error;
pub mod tl;
pub mod types;

pub use error::ThemeError;
pub use types::BaseTheme;

// Re-exports for convenience
pub use tl::BaseThemeBoxed;

/// Check if a base theme is dark.
///
/// This is a convenience function equivalent to [`BaseTheme::is_dark()`].
///
/// # Examples
///
/// ```rust
/// use theme::{is_dark_theme, BaseTheme};
///
/// assert!(!is_dark_theme(BaseTheme::Classic));
/// assert!(is_dark_theme(BaseTheme::Night));
/// ```
#[must_use]
pub const fn is_dark_theme(theme: BaseTheme) -> bool {
    theme.is_dark()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_themes_defined() {
        let themes = [
            BaseTheme::Classic,
            BaseTheme::Day,
            BaseTheme::Night,
            BaseTheme::Tinted,
            BaseTheme::Arctic,
        ];

        for theme in themes {
            // Ensure we can check darkness for all themes
            let _dark = theme.is_dark();
            let _light = theme.is_light();
        }
    }

    #[test]
    fn test_is_dark_theme_function() {
        assert!(!is_dark_theme(BaseTheme::Classic));
        assert!(!is_dark_theme(BaseTheme::Day));
        assert!(is_dark_theme(BaseTheme::Night));
        assert!(is_dark_theme(BaseTheme::Tinted));
        assert!(!is_dark_theme(BaseTheme::Arctic));
    }

    #[test]
    fn test_theme_equality() {
        assert_eq!(BaseTheme::Classic, BaseTheme::Classic);
        assert_ne!(BaseTheme::Day, BaseTheme::Night);
    }

    #[test]
    fn test_default_theme() {
        assert_eq!(BaseTheme::default(), BaseTheme::Classic);
    }
}
