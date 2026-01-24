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

//! # Base Theme
//!
//! Base theme types for Telegram client.
//!
//! Based on TDLib's BaseTheme implementation.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::let_and_return)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Base theme for Telegram.
///
/// Represents the base theme used in Telegram's theming system.
///
/// # Example
///
/// ```rust
/// use rustgram_base_theme::BaseTheme;
///
/// let theme = BaseTheme::Classic;
/// assert_eq!(theme, BaseTheme::Classic);
/// assert!(!theme.is_dark());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[repr(i32)]
pub enum BaseTheme {
    /// Classic light theme
    #[default]
    Classic = 0,

    /// Day theme
    Day = 1,

    /// Night theme (dark)
    Night = 2,

    /// Tinted theme
    Tinted = 3,

    /// Arctic theme
    Arctic = 4,
}

impl BaseTheme {
    /// Creates a BaseTheme from an i32 value.
    ///
    /// # Arguments
    ///
    /// * `value` - The integer value to convert
    ///
    /// # Returns
    ///
    /// Returns `Some(BaseTheme)` if the value is valid, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_base_theme::BaseTheme;
    ///
    /// assert_eq!(BaseTheme::from_i32(0), Some(BaseTheme::Classic));
    /// assert_eq!(BaseTheme::from_i32(1), Some(BaseTheme::Day));
    /// assert_eq!(BaseTheme::from_i32(99), None);
    /// ```
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Classic),
            1 => Some(Self::Day),
            2 => Some(Self::Night),
            3 => Some(Self::Tinted),
            4 => Some(Self::Arctic),
            _ => None,
        }
    }

    /// Returns the i32 representation of this base theme.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_base_theme::BaseTheme;
    ///
    /// assert_eq!(BaseTheme::Classic.to_i32(), 0);
    /// assert_eq!(BaseTheme::Day.to_i32(), 1);
    /// assert_eq!(BaseTheme::Night.to_i32(), 2);
    /// ```
    pub fn to_i32(self) -> i32 {
        self as i32
    }

    /// Returns `true` if this is a dark theme.
    ///
    /// According to TDLib, Night and Tinted are considered dark themes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_base_theme::BaseTheme;
    ///
    /// assert!(!BaseTheme::Classic.is_dark());
    /// assert!(!BaseTheme::Day.is_dark());
    /// assert!(BaseTheme::Night.is_dark());
    /// assert!(BaseTheme::Tinted.is_dark());
    /// assert!(!BaseTheme::Arctic.is_dark());
    /// ```
    pub fn is_dark(&self) -> bool {
        matches!(self, Self::Night | Self::Tinted)
    }

    /// Returns `true` if this is a light theme.
    ///
    /// According to TDLib, Classic, Day, and Arctic are considered light themes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_base_theme::BaseTheme;
    ///
    /// assert!(BaseTheme::Classic.is_light());
    /// assert!(BaseTheme::Day.is_light());
    /// assert!(!BaseTheme::Night.is_light());
    /// assert!(!BaseTheme::Tinted.is_light());
    /// assert!(BaseTheme::Arctic.is_light());
    /// ```
    pub fn is_light(&self) -> bool {
        matches!(self, Self::Classic | Self::Day | Self::Arctic)
    }

    /// Returns the name of this theme.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_base_theme::BaseTheme;
    ///
    /// assert_eq!(BaseTheme::Classic.name(), "Classic");
    /// assert_eq!(BaseTheme::Night.name(), "Night");
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Self::Classic => "Classic",
            Self::Day => "Day",
            Self::Night => "Night",
            Self::Tinted => "Tinted",
            Self::Arctic => "Arctic",
        }
    }

    /// Returns all available base themes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_base_theme::BaseTheme;
    ///
    /// let themes = BaseTheme::all();
    /// assert_eq!(themes.len(), 5);
    /// ```
    pub fn all() -> &'static [BaseTheme] {
        &[
            BaseTheme::Classic,
            BaseTheme::Day,
            BaseTheme::Night,
            BaseTheme::Tinted,
            BaseTheme::Arctic,
        ]
    }
}

impl fmt::Display for BaseTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn test_from_i32() {
        assert_eq!(BaseTheme::from_i32(0), Some(BaseTheme::Classic));
        assert_eq!(BaseTheme::from_i32(1), Some(BaseTheme::Day));
        assert_eq!(BaseTheme::from_i32(2), Some(BaseTheme::Night));
        assert_eq!(BaseTheme::from_i32(3), Some(BaseTheme::Tinted));
        assert_eq!(BaseTheme::from_i32(4), Some(BaseTheme::Arctic));
        assert_eq!(BaseTheme::from_i32(-1), None);
        assert_eq!(BaseTheme::from_i32(5), None);
        assert_eq!(BaseTheme::from_i32(99), None);
    }

    #[test]
    fn test_to_i32() {
        assert_eq!(BaseTheme::Classic.to_i32(), 0);
        assert_eq!(BaseTheme::Day.to_i32(), 1);
        assert_eq!(BaseTheme::Night.to_i32(), 2);
        assert_eq!(BaseTheme::Tinted.to_i32(), 3);
        assert_eq!(BaseTheme::Arctic.to_i32(), 4);
    }

    #[test]
    fn test_roundtrip_conversion() {
        for value in 0..=4 {
            let theme = BaseTheme::from_i32(value);
            assert_eq!(theme.map(|t| t.to_i32()), Some(value));
        }
    }

    #[test]
    fn test_is_dark() {
        assert!(!BaseTheme::Classic.is_dark());
        assert!(!BaseTheme::Day.is_dark());
        assert!(BaseTheme::Night.is_dark());
        assert!(BaseTheme::Tinted.is_dark()); // Tinted is dark in TDLib
        assert!(!BaseTheme::Arctic.is_dark());
    }

    #[test]
    fn test_is_light() {
        assert!(BaseTheme::Classic.is_light());
        assert!(BaseTheme::Day.is_light());
        assert!(!BaseTheme::Night.is_light());
        assert!(!BaseTheme::Tinted.is_light()); // Tinted is NOT light in TDLib
        assert!(BaseTheme::Arctic.is_light());
    }

    #[test]
    fn test_name() {
        assert_eq!(BaseTheme::Classic.name(), "Classic");
        assert_eq!(BaseTheme::Day.name(), "Day");
        assert_eq!(BaseTheme::Night.name(), "Night");
        assert_eq!(BaseTheme::Tinted.name(), "Tinted");
        assert_eq!(BaseTheme::Arctic.name(), "Arctic");
    }

    #[test]
    fn test_all() {
        let themes = BaseTheme::all();
        assert_eq!(themes.len(), 5);
        assert!(themes.contains(&BaseTheme::Classic));
        assert!(themes.contains(&BaseTheme::Day));
        assert!(themes.contains(&BaseTheme::Night));
        assert!(themes.contains(&BaseTheme::Tinted));
        assert!(themes.contains(&BaseTheme::Arctic));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", BaseTheme::Classic), "Classic");
        assert_eq!(format!("{}", BaseTheme::Day), "Day");
        assert_eq!(format!("{}", BaseTheme::Night), "Night");
        assert_eq!(format!("{}", BaseTheme::Tinted), "Tinted");
        assert_eq!(format!("{}", BaseTheme::Arctic), "Arctic");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", BaseTheme::Classic), "Classic");
        assert_eq!(format!("{:?}", BaseTheme::Night), "Night");
    }

    #[test]
    fn test_default() {
        assert_eq!(BaseTheme::default(), BaseTheme::Classic);
    }

    #[test]
    fn test_equality() {
        assert_eq!(BaseTheme::Classic, BaseTheme::Classic);
        assert_eq!(BaseTheme::Night, BaseTheme::Night);
        assert_ne!(BaseTheme::Classic, BaseTheme::Night);
        assert_ne!(BaseTheme::Day, BaseTheme::Arctic);
    }

    #[test]
    fn test_copy() {
        let a = BaseTheme::Night;
        let b = a;
        assert_eq!(a, BaseTheme::Night);
        assert_eq!(b, BaseTheme::Night);
    }

    #[test]
    fn test_clone() {
        let theme = BaseTheme::Night;
        let cloned = theme;
        assert_eq!(theme, cloned);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(BaseTheme::Classic);
        set.insert(BaseTheme::Day);
        set.insert(BaseTheme::Night);
        set.insert(BaseTheme::Tinted);
        set.insert(BaseTheme::Arctic);
        assert_eq!(set.len(), 5);
    }

    #[test]
    fn test_serialization() {
        let theme = BaseTheme::Night;
        let json = serde_json::to_string(&theme).unwrap();
        let parsed: BaseTheme = serde_json::from_str(&json).unwrap();
        assert_eq!(theme, parsed);
    }

    #[test]
    fn test_serialization_all() {
        for theme in BaseTheme::all() {
            let json = serde_json::to_string(theme).unwrap();
            let parsed: BaseTheme = serde_json::from_str(&json).unwrap();
            assert_eq!(theme, &parsed);
        }
    }

    #[test]
    fn test_theme_iteration() {
        let themes = BaseTheme::all();
        let dark_count = themes.iter().filter(|t| t.is_dark()).count();
        let light_count = themes.iter().filter(|t| t.is_light()).count();
        assert_eq!(dark_count, 2); // Night and Tinted are dark
        assert_eq!(light_count, 3); // Classic, Day, Arctic are light
    }

    #[test]
    fn test_classic_theme() {
        let theme = BaseTheme::Classic;
        assert!(!theme.is_dark());
        assert!(theme.is_light());
        assert_eq!(theme.name(), "Classic");
        assert_eq!(theme.to_i32(), 0);
    }

    #[test]
    fn test_day_theme() {
        let theme = BaseTheme::Day;
        assert!(!theme.is_dark());
        assert!(theme.is_light());
        assert_eq!(theme.name(), "Day");
        assert_eq!(theme.to_i32(), 1);
    }

    #[test]
    fn test_night_theme() {
        let theme = BaseTheme::Night;
        assert!(theme.is_dark());
        assert!(!theme.is_light());
        assert_eq!(theme.name(), "Night");
        assert_eq!(theme.to_i32(), 2);
    }

    #[test]
    fn test_tinted_theme() {
        let theme = BaseTheme::Tinted;
        assert!(theme.is_dark()); // Tinted is dark in TDLib
        assert!(!theme.is_light()); // Tinted is NOT light in TDLib
        assert_eq!(theme.name(), "Tinted");
        assert_eq!(theme.to_i32(), 3);
    }

    #[test]
    fn test_arctic_theme() {
        let theme = BaseTheme::Arctic;
        assert!(!theme.is_dark());
        assert!(theme.is_light());
        assert_eq!(theme.name(), "Arctic");
        assert_eq!(theme.to_i32(), 4);
    }
}
