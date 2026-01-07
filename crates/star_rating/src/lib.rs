// rustgram_star_rating
// Copyright (C) 2025 rustgram-client contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! # Star Rating
//!
//! Represents a star rating with level, star counts, and maximum level status.
//!
//! ## Example
//!
//! ```rust
//! use rustgram_star_rating::StarRating;
//!
//! let rating = StarRating::with_values(5, 1000, 500, 1500);
//! assert_eq!(rating.level(), 5);
//! assert_eq!(rating.star_count(), 1000);
//! ```

use std::fmt;

/// Star rating representation with level and progression information.
///
/// Contains the current level, total stars earned, stars at the current level,
/// and stars needed for the next level.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarRating {
    /// Current level
    level: i32,
    /// Total stars earned
    star_count: i64,
    /// Stars at current level
    current_level_star_count: i64,
    /// Stars needed for next level
    next_level_star_count: i64,
    /// Maximum level reached flag
    is_maximum_level_reached: bool,
}

impl Default for StarRating {
    fn default() -> Self {
        Self::new()
    }
}

impl StarRating {
    /// Creates a new star rating with default values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_rating::StarRating;
    ///
    /// let rating = StarRating::new();
    /// assert_eq!(rating.level(), 0);
    /// assert_eq!(rating.star_count(), 0);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            level: 0,
            star_count: 0,
            current_level_star_count: 0,
            next_level_star_count: 0,
            is_maximum_level_reached: false,
        }
    }

    /// Creates a new star rating from Telegram API values.
    ///
    /// This is a simplified version that accepts the raw star values.
    /// In production, this would use `StarManager::get_star_count` to convert
    /// the API values to internal representation.
    ///
    /// # Arguments
    ///
    /// * `level` - Current level
    /// * `stars` - Total stars from API
    /// * `current_level_stars` - Stars at current level from API
    /// * `next_level_stars` - Stars needed for next level from API
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_rating::StarRating;
    ///
    /// let rating = StarRating::from_telegram_api(5, 1000, 500, 1500);
    /// assert_eq!(rating.level(), 5);
    /// assert_eq!(rating.star_count(), 1000);
    /// assert!(!rating.is_maximum_level_reached());
    ///
    /// let max_rating = StarRating::from_telegram_api(10, 5000, 5000, 0);
    /// assert!(max_rating.is_maximum_level_reached());
    /// ```
    #[must_use]
    pub fn from_telegram_api(
        level: i32,
        stars: i64,
        current_level_stars: i64,
        next_level_stars: i64,
    ) -> Self {
        let is_maximum_level_reached = next_level_stars == 0 && level > 0;

        Self {
            level,
            star_count: stars,
            current_level_star_count: current_level_stars,
            next_level_star_count: next_level_stars,
            is_maximum_level_reached,
        }
    }

    /// Creates a new star rating with explicit values.
    ///
    /// # Arguments
    ///
    /// * `level` - Current level
    /// * `star_count` - Total stars earned
    /// * `current_level_star_count` - Stars at current level
    /// * `next_level_star_count` - Stars needed for next level
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_rating::StarRating;
    ///
    /// let rating = StarRating::with_values(3, 750, 250, 1000);
    /// assert_eq!(rating.level(), 3);
    /// assert_eq!(rating.star_count(), 750);
    /// ```
    #[must_use]
    pub fn with_values(
        level: i32,
        star_count: i64,
        current_level_star_count: i64,
        next_level_star_count: i64,
    ) -> Self {
        let is_maximum_level_reached = next_level_star_count == 0 && level > 0;

        Self {
            level,
            star_count,
            current_level_star_count,
            next_level_star_count,
            is_maximum_level_reached,
        }
    }

    /// Returns the current level.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_rating::StarRating;
    ///
    /// let rating = StarRating::with_values(5, 1000, 500, 1500);
    /// assert_eq!(rating.level(), 5);
    /// ```
    #[must_use]
    pub const fn level(&self) -> i32 {
        self.level
    }

    /// Returns the total star count.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_rating::StarRating;
    ///
    /// let rating = StarRating::with_values(3, 1000, 500, 1500);
    /// assert_eq!(rating.star_count(), 1000);
    /// ```
    #[must_use]
    pub const fn star_count(&self) -> i64 {
        self.star_count
    }

    /// Returns the star count at the current level.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_rating::StarRating;
    ///
    /// let rating = StarRating::with_values(3, 1000, 500, 1500);
    /// assert_eq!(rating.current_level_star_count(), 500);
    /// ```
    #[must_use]
    pub const fn current_level_star_count(&self) -> i64 {
        self.current_level_star_count
    }

    /// Returns the star count needed for the next level.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_rating::StarRating;
    ///
    /// let rating = StarRating::with_values(3, 1000, 500, 1500);
    /// assert_eq!(rating.next_level_star_count(), 1500);
    /// ```
    #[must_use]
    pub const fn next_level_star_count(&self) -> i64 {
        self.next_level_star_count
    }

    /// Returns whether the maximum level has been reached.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_star_rating::StarRating;
    ///
    /// let normal = StarRating::with_values(5, 1000, 500, 1500);
    /// assert!(!normal.is_maximum_level_reached());
    ///
    /// let max_level = StarRating::with_values(10, 5000, 5000, 0);
    /// assert!(max_level.is_maximum_level_reached());
    /// ```
    #[must_use]
    pub const fn is_maximum_level_reached(&self) -> bool {
        self.is_maximum_level_reached
    }
}

impl fmt::Display for StarRating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "level {} with rating {}", self.level, self.star_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let rating = StarRating::default();
        assert_eq!(rating.level(), 0);
        assert_eq!(rating.star_count(), 0);
        assert_eq!(rating.current_level_star_count(), 0);
        assert_eq!(rating.next_level_star_count(), 0);
        assert!(!rating.is_maximum_level_reached());
    }

    #[test]
    fn test_new() {
        let rating = StarRating::new();
        assert_eq!(rating.level(), 0);
        assert_eq!(rating.star_count(), 0);
    }

    #[test]
    fn test_with_values() {
        let rating = StarRating::with_values(5, 1000, 500, 1500);
        assert_eq!(rating.level(), 5);
        assert_eq!(rating.star_count(), 1000);
        assert_eq!(rating.current_level_star_count(), 500);
        assert_eq!(rating.next_level_star_count(), 1500);
        assert!(!rating.is_maximum_level_reached());
    }

    #[test]
    fn test_from_telegram_api() {
        let rating = StarRating::from_telegram_api(3, 750, 250, 1000);
        assert_eq!(rating.level(), 3);
        assert_eq!(rating.star_count(), 750);
        assert_eq!(rating.current_level_star_count(), 250);
        assert_eq!(rating.next_level_star_count(), 1000);
    }

    #[test]
    fn test_maximum_level_detection() {
        // Maximum level: next_level_star_count == 0 && level > 0
        let max_rating = StarRating::from_telegram_api(10, 5000, 5000, 0);
        assert!(max_rating.is_maximum_level_reached());

        // Not maximum level even with zero level
        let zero_level = StarRating::from_telegram_api(0, 0, 0, 0);
        assert!(!zero_level.is_maximum_level_reached());

        // Not maximum level with next_level > 0
        let normal = StarRating::from_telegram_api(5, 1000, 500, 1500);
        assert!(!normal.is_maximum_level_reached());
    }

    #[test]
    fn test_getters() {
        let rating = StarRating::with_values(7, 3500, 700, 4200);

        assert_eq!(rating.level(), 7);
        assert_eq!(rating.star_count(), 3500);
        assert_eq!(rating.current_level_star_count(), 700);
        assert_eq!(rating.next_level_star_count(), 4200);
    }

    #[test]
    fn test_equality() {
        let a = StarRating::with_values(5, 1000, 500, 1500);
        let b = StarRating::with_values(5, 1000, 500, 1500);
        let c = StarRating::with_values(5, 1001, 500, 1500);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_display() {
        let rating = StarRating::with_values(5, 1000, 500, 1500);
        assert_eq!(format!("{}", rating), "level 5 with rating 1000");
    }

    #[test]
    fn test_display_zero_level() {
        let rating = StarRating::new();
        assert_eq!(format!("{}", rating), "level 0 with rating 0");
    }

    #[test]
    fn test_display_large_values() {
        let rating = StarRating::with_values(100, 999999, 50000, 1000000);
        assert_eq!(format!("{}", rating), "level 100 with rating 999999");
    }

    #[test]
    fn test_cloning() {
        let rating1 = StarRating::with_values(5, 1000, 500, 1500);
        let rating2 = rating1.clone();
        assert_eq!(rating1, rating2);
    }

    #[test]
    fn test_progress_calculation() {
        // Example: level 3 with 750/1000 stars at current level
        let rating = StarRating::with_values(3, 2750, 750, 1000);
        assert_eq!(rating.level(), 3);
        assert_eq!(rating.current_level_star_count(), 750);
        assert_eq!(rating.next_level_star_count(), 1000);
    }

    #[test]
    fn test_edge_case_zero_next_level() {
        // Level > 0 but next_level_star_count == 0 indicates max level
        let rating = StarRating::with_values(50, 100000, 100000, 0);
        assert!(rating.is_maximum_level_reached());
        assert_eq!(rating.next_level_star_count(), 0);
    }

    #[test]
    fn test_multiple_levels_progression() {
        let levels = [
            StarRating::with_values(1, 100, 100, 200),
            StarRating::with_values(2, 300, 100, 300),
            StarRating::with_values(3, 600, 300, 400),
            StarRating::with_values(4, 1000, 400, 500),
        ];

        assert_eq!(levels[0].level(), 1);
        assert_eq!(levels[1].level(), 2);
        assert_eq!(levels[2].level(), 3);
        assert_eq!(levels[3].level(), 4);
    }
}
