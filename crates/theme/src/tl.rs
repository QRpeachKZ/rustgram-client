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

//! TL (Type Language) constructors for theme types.
//!
//! This module provides Telegram API TL constructors for BaseTheme.

use rustgram_types::TlConstructor;
use serde::{Deserialize, Serialize};

use crate::{BaseTheme, ThemeError};

// TL Constructor IDs from telegram_api.tl:
// baseThemeClassic#c3a12462 = BaseTheme;
// baseThemeDay#fbd81688 = BaseTheme;
// baseThemeNight#b7b31ea8 = BaseTheme;
// baseThemeTinted#6d5f77ee = BaseTheme;
// baseThemeArctic#5b11125a = BaseTheme;

/// TL boxed BaseTheme type.
///
/// This represents the polymorphic BaseTheme type in Telegram API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BaseThemeBoxed {
    /// Classic theme (light)
    #[serde(rename = "baseThemeClassic")]
    Classic,
    /// Regular light theme
    #[serde(rename = "baseThemeDay")]
    Day,
    /// Regular dark theme
    #[serde(rename = "baseThemeNight")]
    Night,
    /// Tinted dark theme
    #[serde(rename = "baseThemeTinted")]
    Tinted,
    /// Arctic light theme
    #[serde(rename = "baseThemeArctic")]
    Arctic,
}

impl BaseThemeBoxed {
    /// Convert to BaseTheme enum.
    #[must_use]
    pub const fn to_base_theme(&self) -> BaseTheme {
        match self {
            Self::Classic => BaseTheme::Classic,
            Self::Day => BaseTheme::Day,
            Self::Night => BaseTheme::Night,
            Self::Tinted => BaseTheme::Tinted,
            Self::Arctic => BaseTheme::Arctic,
        }
    }

    /// Create BaseThemeBoxed from constructor ID.
    ///
    /// # Errors
    ///
    /// Returns [`ThemeError::UnknownBaseThemeConstructor`] if the ID is unknown.
    pub fn from_constructor_id(id: u32) -> Result<Self, ThemeError> {
        match id {
            0xc3a12462 => Ok(Self::Classic), // baseThemeClassic
            0xfbd81688 => Ok(Self::Day),     // baseThemeDay
            0xb7b31ea8 => Ok(Self::Night),   // baseThemeNight
            0x6d5f77ee => Ok(Self::Tinted),  // baseThemeTinted
            0x5b11125a => Ok(Self::Arctic),  // baseThemeArctic
            id => Err(ThemeError::UnknownBaseThemeConstructor(id)),
        }
    }
}

impl TlConstructor for BaseThemeBoxed {
    fn constructor_id(&self) -> u32 {
        match self {
            Self::Classic => 0xc3a12462, // baseThemeClassic
            Self::Day => 0xfbd81688,     // baseThemeDay
            Self::Night => 0xb7b31ea8,   // baseThemeNight
            Self::Tinted => 0x6d5f77ee,  // baseThemeTinted
            Self::Arctic => 0x5b11125a,  // baseThemeArctic
        }
    }
}

impl From<BaseTheme> for BaseThemeBoxed {
    fn from(theme: BaseTheme) -> Self {
        match theme {
            BaseTheme::Classic => Self::Classic,
            BaseTheme::Day => Self::Day,
            BaseTheme::Night => Self::Night,
            BaseTheme::Tinted => Self::Tinted,
            BaseTheme::Arctic => Self::Arctic,
        }
    }
}

impl From<BaseThemeBoxed> for BaseTheme {
    fn from(boxed: BaseThemeBoxed) -> Self {
        boxed.to_base_theme()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor_ids() {
        assert_eq!(BaseThemeBoxed::Classic.constructor_id(), 0xc3a12462);
        assert_eq!(BaseThemeBoxed::Day.constructor_id(), 0xfbd81688);
        assert_eq!(BaseThemeBoxed::Night.constructor_id(), 0xb7b31ea8);
        assert_eq!(BaseThemeBoxed::Tinted.constructor_id(), 0x6d5f77ee);
        assert_eq!(BaseThemeBoxed::Arctic.constructor_id(), 0x5b11125a);
    }

    #[test]
    fn test_from_constructor_id() {
        assert_eq!(
            BaseThemeBoxed::from_constructor_id(0xc3a12462),
            Ok(BaseThemeBoxed::Classic)
        );
        assert_eq!(
            BaseThemeBoxed::from_constructor_id(0xfbd81688),
            Ok(BaseThemeBoxed::Day)
        );
        assert!(BaseThemeBoxed::from_constructor_id(0xdeadbeef).is_err());
    }

    #[test]
    fn test_conversions() {
        let theme = BaseTheme::Night;
        let boxed = BaseThemeBoxed::from(theme);
        assert_eq!(boxed.to_base_theme(), BaseTheme::Night);
    }
}
