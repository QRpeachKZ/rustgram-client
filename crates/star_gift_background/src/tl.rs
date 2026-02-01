// Copyright 2025 QRpeach
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

//! TL constructors для Star Gift Background.
//!
//! Соответствует MTProto схеме:
//! ```text
//! starGiftBackground#aff56398 center_color:int edge_color:int text_color:int = StarGiftBackground;
//! ```

use crate::StarGiftBackground;
use serde::{Deserialize, Serialize};

/// TL конструктор для фона звездного подарка.
///
/// Соответствует MTProto типу `starGiftBackground#aff56398`.
///
/// # Поля
///
/// - `center_color`: Цвет центральной части градиента (RGB)
/// - `edge_color`: Цвет краев градиента (RGB)
/// - `text_color`: Цвет текста на фоне (RGB)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StarGiftBackgroundTl {
    /// Цвет центральной части градиента (RGB)
    pub center_color: i32,

    /// Цвет краев градиента (RGB)
    pub edge_color: i32,

    /// Цвет текста на фоне (RGB)
    pub text_color: i32,
}

impl From<StarGiftBackgroundTl> for StarGiftBackground {
    #[inline]
    fn from(tl: StarGiftBackgroundTl) -> Self {
        Self::new(tl.center_color, tl.edge_color, tl.text_color)
    }
}

impl From<StarGiftBackground> for StarGiftBackgroundTl {
    #[inline]
    fn from(background: StarGiftBackground) -> Self {
        Self {
            center_color: background.center_color(),
            edge_color: background.edge_color(),
            text_color: background.text_color(),
        }
    }
}

impl StarGiftBackground {
    /// Преобразует фон в TL формат.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use star_gift_background::StarGiftBackground;
    ///
    /// let background = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
    /// let tl = background.to_tl();
    /// assert_eq!(tl.center_color, 0xFF6B9D);
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_tl(self) -> StarGiftBackgroundTl {
        StarGiftBackgroundTl {
            center_color: self.center_color,
            edge_color: self.edge_color,
            text_color: self.text_color,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tl_conversion_roundtrip() {
        let original = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
        let tl = original.to_tl();
        let converted = StarGiftBackground::from(tl);

        assert_eq!(original, converted);
    }

    #[test]
    fn test_tl_from_background() {
        let background = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
        let tl: StarGiftBackgroundTl = background.into();

        assert_eq!(tl.center_color, 0xFF6B9D);
        assert_eq!(tl.edge_color, 0xC850C0);
        assert_eq!(tl.text_color, 0xFFFFFF);
    }

    #[test]
    fn test_tl_serialization() {
        let tl = StarGiftBackgroundTl {
            center_color: 0xFF6B9D,
            edge_color: 0xC850C0,
            text_color: 0xFFFFFF,
        };

        let json = serde_json::to_string(&tl).unwrap();
        let deserialized: StarGiftBackgroundTl = serde_json::from_str(&json).unwrap();

        assert_eq!(tl, deserialized);
    }
}
