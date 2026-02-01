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

//! # Star Gift Background
//!
//! Модуль для работы с фонами звездных подарков (Star Gifts) в Telegram.
//!
//! Фон звездного подарка определяется тремя цветами в формате RGB:
//! - `center_color`: Цвет центральной части градиента
//! - `edge_color`: Цвет краев градиента
//! - `text_color`: Цвет текста, отображаемого на фоне
//!
//! ## Пример использования
//!
//! ```rust
//! use star_gift_background::{StarGiftBackground, tl::StarGiftBackgroundTl};
//!
//! // Создание из TL типа
//! let tl = StarGiftBackgroundTl {
//!     center_color: 0xFF6B9D,
//!     edge_color: 0xC850C0,
//!     text_color: 0xFFFFFF,
//! };
//!
//! let background = StarGiftBackground::from(tl);
//! assert_eq!(background.center_color(), 0xFF6B9D);
//!
//! // Создание напрямую
//! let background = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
//! ```

pub mod tl;

pub use tl::*;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Фон звездного подарка (Star Gift Background).
///
/// Определяет градиентный фон для звездных подарков с тремя цветами:
/// - центральный цвет градиента
/// - цвет краев градиента
/// - цвет текста
///
/// Цвета представлены как 32-битные целые числа в RGB формате.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StarGiftBackground {
    /// Цвет центральной части градиента (RGB)
    center_color: i32,

    /// Цвет краев градиента (RGB)
    edge_color: i32,

    /// Цвет текста на фоне (RGB)
    text_color: i32,
}

impl StarGiftBackground {
    /// Создает новый фон звездного подарка.
    ///
    /// # Аргументы
    ///
    /// * `center_color` - Цвет центральной части градиента (RGB)
    /// * `edge_color` - Цвет краев градиента (RGB)
    /// * `text_color` - Цвет текста (RGB)
    ///
    /// # Пример
    ///
    /// ```rust
    /// use star_gift_background::StarGiftBackground;
    ///
    /// let background = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
    /// assert_eq!(background.center_color(), 0xFF6B9D);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(center_color: i32, edge_color: i32, text_color: i32) -> Self {
        Self {
            center_color,
            edge_color,
            text_color,
        }
    }

    /// Создает фон с черным цветом по умолчанию.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use star_gift_background::StarGiftBackground;
    ///
    /// let background = StarGiftBackground::default();
    /// assert_eq!(background.center_color(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn default() -> Self {
        Self {
            center_color: 0,
            edge_color: 0,
            text_color: 0,
        }
    }

    /// Возвращает цвет центральной части градиента.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use star_gift_background::StarGiftBackground;
    ///
    /// let background = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
    /// assert_eq!(background.center_color(), 0xFF6B9D);
    /// ```
    #[inline]
    #[must_use]
    pub const fn center_color(&self) -> i32 {
        self.center_color
    }

    /// Возвращает цвет краев градиента.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use star_gift_background::StarGiftBackground;
    ///
    /// let background = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
    /// assert_eq!(background.edge_color(), 0xC850C0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn edge_color(&self) -> i32 {
        self.edge_color
    }

    /// Возвращает цвет текста на фоне.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use star_gift_background::StarGiftBackground;
    ///
    /// let background = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
    /// assert_eq!(background.text_color(), 0xFFFFFF);
    /// ```
    #[inline]
    #[must_use]
    pub const fn text_color(&self) -> i32 {
        self.text_color
    }

    /// Проверяет, являются ли все цвета нулевыми (черными).
    ///
    /// # Пример
    ///
    /// ```rust
    /// use star_gift_background::StarGiftBackground;
    ///
    /// let black = StarGiftBackground::default();
    /// assert!(black.is_empty());
    ///
    /// let colored = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
    /// assert!(!colored.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.center_color == 0 && self.edge_color == 0 && self.text_color == 0
    }

    /// Устанавливает цвет центральной части градиента.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use star_gift_background::StarGiftBackground;
    ///
    /// let mut background = StarGiftBackground::default();
    /// background.set_center_color(0xFF6B9D);
    /// assert_eq!(background.center_color(), 0xFF6B9D);
    /// ```
    #[inline]
    pub fn set_center_color(&mut self, color: i32) {
        self.center_color = color;
    }

    /// Устанавливает цвет краев градиента.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use star_gift_background::StarGiftBackground;
    ///
    /// let mut background = StarGiftBackground::default();
    /// background.set_edge_color(0xC850C0);
    /// assert_eq!(background.edge_color(), 0xC850C0);
    /// ```
    #[inline]
    pub fn set_edge_color(&mut self, color: i32) {
        self.edge_color = color;
    }

    /// Устанавливает цвет текста.
    ///
    /// # Пример
    ///
    /// ```rust
    /// use star_gift_background::StarGiftBackground;
    ///
    /// let mut background = StarGiftBackground::default();
    /// background.set_text_color(0xFFFFFF);
    /// assert_eq!(background.text_color(), 0xFFFFFF);
    /// ```
    #[inline]
    pub fn set_text_color(&mut self, color: i32) {
        self.text_color = color;
    }
}

impl Default for StarGiftBackground {
    #[inline]
    fn default() -> Self {
        Self::default()
    }
}

impl fmt::Display for StarGiftBackground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GiftBackground[{:#x}/{:#x}/{:#x}]",
            self.center_color, self.edge_color, self.text_color
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let background = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
        assert_eq!(background.center_color(), 0xFF6B9D);
        assert_eq!(background.edge_color(), 0xC850C0);
        assert_eq!(background.text_color(), 0xFFFFFF);
    }

    #[test]
    fn test_default() {
        let background = StarGiftBackground::default();
        assert_eq!(background.center_color(), 0);
        assert_eq!(background.edge_color(), 0);
        assert_eq!(background.text_color(), 0);
    }

    #[test]
    fn test_is_empty() {
        let black = StarGiftBackground::default();
        assert!(black.is_empty());

        let colored = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
        assert!(!colored.is_empty());

        let partial = StarGiftBackground::new(0xFF6B9D, 0, 0);
        assert!(!partial.is_empty());
    }

    #[test]
    fn test_setters() {
        let mut background = StarGiftBackground::default();
        background.set_center_color(0xFF6B9D);
        background.set_edge_color(0xC850C0);
        background.set_text_color(0xFFFFFF);

        assert_eq!(background.center_color(), 0xFF6B9D);
        assert_eq!(background.edge_color(), 0xC850C0);
        assert_eq!(background.text_color(), 0xFFFFFF);
    }

    #[test]
    fn test_equality() {
        let bg1 = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
        let bg2 = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
        let bg3 = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0x000000);

        assert_eq!(bg1, bg2);
        assert_ne!(bg1, bg3);
    }

    #[test]
    fn test_clone() {
        let bg1 = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
        let bg2 = bg1;

        assert_eq!(bg1, bg2);
    }

    #[test]
    fn test_display() {
        let background = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
        let s = format!("{}", background);
        assert!(s.contains("ff6b9d"));
        assert!(s.contains("c850c0"));
        assert!(s.contains("ffffff"));
        assert!(s.contains("GiftBackground"));
    }

    #[test]
    fn test_from_tl() {
        let tl = StarGiftBackgroundTl {
            center_color: 0xFF6B9D,
            edge_color: 0xC850C0,
            text_color: 0xFFFFFF,
        };

        let background = StarGiftBackground::from(tl);
        assert_eq!(background.center_color(), 0xFF6B9D);
        assert_eq!(background.edge_color(), 0xC850C0);
        assert_eq!(background.text_color(), 0xFFFFFF);
    }

    #[test]
    fn test_to_tl() {
        let background = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
        let tl = background.to_tl();

        assert_eq!(tl.center_color, 0xFF6B9D);
        assert_eq!(tl.edge_color, 0xC850C0);
        assert_eq!(tl.text_color, 0xFFFFFF);
    }

    #[test]
    fn test_serialization() {
        let background = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);

        let json = serde_json::to_string(&background).unwrap();
        let deserialized: StarGiftBackground = serde_json::from_str(&json).unwrap();

        assert_eq!(background, deserialized);
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let bg1 = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
        let bg2 = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
        let bg3 = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0x000000);

        let mut hasher = DefaultHasher::new();
        bg1.hash(&mut hasher);
        let hash1 = hasher.finish();

        hasher = DefaultHasher::new();
        bg2.hash(&mut hasher);
        let hash2 = hasher.finish();

        hasher = DefaultHasher::new();
        bg3.hash(&mut hasher);
        let hash3 = hasher.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_premium_gradient() {
        // Реальный пример премиум-градиента Telegram
        let premium = StarGiftBackground::new(0x6B9DFF, 0x9D6BFF, 0xFFFFFF);
        assert!(!premium.is_empty());
        assert_eq!(premium.center_color(), 0x6B9DFF);
    }

    #[test]
    fn test_valentine_gradient() {
        // Валентинский градиент
        let valentine = StarGiftBackground::new(0xFF6B9D, 0xC850C0, 0xFFFFFF);
        assert!(!valentine.is_empty());
    }
}
