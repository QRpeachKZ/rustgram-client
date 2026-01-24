//! # Photo Size Type
//!
//! Photo size type identifier.
//!
//! ## TDLib Reference
//!
//! - TDLib header: `td/telegram/PhotoSizeType.h`
//! - TDLib struct: `PhotoSizeType`
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_photo_size_type::PhotoSizeType;
//!
//! let size_type = PhotoSizeType::new('a');
//! ```

use core::fmt;

/// Photo size type identifier.
///
/// This is typically a single character identifier for different photo sizes:
/// - 's': Small
/// - 'm': Medium
/// - 'x': Large
/// - 'y': Larger
/// - 'w': Largest
/// - 'a': Avatar small
/// - etc.
///
/// TDLib: `struct PhotoSizeType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PhotoSizeType {
    /// The type character (typically 'a'-'z')
    type_value: i32,
}

impl PhotoSizeType {
    /// Create a new PhotoSizeType from a character.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_size_type::PhotoSizeType;
    ///
    /// let size_type = PhotoSizeType::new('s');
    /// ```
    #[inline]
    pub const fn new(c: char) -> Self {
        Self {
            type_value: c as i32,
        }
    }

    /// Create a new PhotoSizeType from an i32 value.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_size_type::PhotoSizeType;
    ///
    /// let size_type = PhotoSizeType::from_i32(115); // 's'
    /// ```
    #[inline]
    pub const fn from_i32(value: i32) -> Self {
        Self { type_value: value }
    }

    /// Get the i32 representation of this type.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_size_type::PhotoSizeType;
    ///
    /// let size_type = PhotoSizeType::new('s');
    /// assert_eq!(size_type.as_i32(), 115);
    /// ```
    #[inline]
    pub const fn as_i32(self) -> i32 {
        self.type_value
    }

    /// Get the character representation of this type.
    ///
    /// Returns `None` if the value is not a valid Unicode character.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_size_type::PhotoSizeType;
    ///
    /// let size_type = PhotoSizeType::new('s');
    /// assert_eq!(size_type.as_char(), Some('s'));
    /// ```
    #[inline]
    pub const fn as_char(self) -> Option<char> {
        if self.type_value >= 0 && self.type_value <= 0x10FFFF {
            // Safety: The value is within valid Unicode range
            Some(unsafe { char::from_u32_unchecked(self.type_value as u32) })
        } else {
            None
        }
    }

    /// Check if this is a small photo size.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_size_type::PhotoSizeType;
    ///
    /// assert!(PhotoSizeType::new('s').is_small());
    /// assert!(!PhotoSizeType::new('x').is_small());
    /// ```
    #[inline]
    pub const fn is_small(self) -> bool {
        self.type_value == ('s' as i32) || self.type_value == ('a' as i32)
    }

    /// Check if this is a medium photo size.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_size_type::PhotoSizeType;
    ///
    /// assert!(PhotoSizeType::new('m').is_medium());
    /// assert!(!PhotoSizeType::new('s').is_medium());
    /// ```
    #[inline]
    pub const fn is_medium(self) -> bool {
        self.type_value == ('m' as i32) || self.type_value == ('b' as i32)
    }

    /// Check if this is a large photo size.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_size_type::PhotoSizeType;
    ///
    /// assert!(PhotoSizeType::new('x').is_large());
    /// assert!(PhotoSizeType::new('y').is_large());
    /// assert!(PhotoSizeType::new('w').is_large());
    /// assert!(!PhotoSizeType::new('s').is_large());
    /// ```
    #[inline]
    pub const fn is_large(self) -> bool {
        self.type_value == ('x' as i32)
            || self.type_value == ('y' as i32)
            || self.type_value == ('w' as i32)
    }

    /// Check if this is an avatar photo size.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_photo_size_type::PhotoSizeType;
    ///
    /// assert!(PhotoSizeType::new('a').is_avatar());
    /// assert!(!PhotoSizeType::new('s').is_avatar());
    /// ```
    #[inline]
    pub const fn is_avatar(self) -> bool {
        self.type_value == ('a' as i32)
    }
}

impl Default for PhotoSizeType {
    fn default() -> Self {
        Self::new('s')
    }
}

impl fmt::Display for PhotoSizeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if ('a'..='z').contains(&self.type_value) {
            write!(f, "{}", self.type_value as u8 as char)
        } else {
            write!(f, "{}", self.type_value)
        }
    }
}

impl PartialEq<char> for PhotoSizeType {
    fn eq(&self, other: &char) -> bool {
        self.type_value == (*other as i32)
    }
}

impl PartialEq<i32> for PhotoSizeType {
    fn eq(&self, other: &i32) -> bool {
        self.type_value == *other
    }
}

impl From<char> for PhotoSizeType {
    fn from(c: char) -> Self {
        Self::new(c)
    }
}

impl From<i32> for PhotoSizeType {
    fn from(value: i32) -> Self {
        Self::from_i32(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (10 tests)
    #[test]
    fn test_clone() {
        let a = PhotoSizeType::new('s');
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_copy() {
        let a = PhotoSizeType::new('m');
        let b = a;
        assert_eq!(a, PhotoSizeType::new('m'));
        assert_eq!(b, PhotoSizeType::new('m'));
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(PhotoSizeType::new('s'), PhotoSizeType::new('s'));
        assert_ne!(PhotoSizeType::new('s'), PhotoSizeType::new('m'));
    }

    #[test]
    fn test_default() {
        assert_eq!(PhotoSizeType::default(), PhotoSizeType::new('s'));
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        PhotoSizeType::new('s').hash(&mut hasher);
        let h1 = hasher.finish();

        hasher = DefaultHasher::new();
        PhotoSizeType::new('s').hash(&mut hasher);
        let h2 = hasher.finish();

        assert_eq!(h1, h2);
    }

    // Constructor tests (6 tests)
    #[test]
    fn test_new() {
        let size_type = PhotoSizeType::new('s');
        assert_eq!(size_type.as_i32(), 's' as i32);
    }

    #[test]
    fn test_from_i32() {
        let size_type = PhotoSizeType::from_i32(115);
        assert_eq!(size_type.as_i32(), 115);
    }

    #[test]
    fn test_from_char() {
        let size_type = PhotoSizeType::from('s');
        assert_eq!(size_type.as_char(), Some('s'));
    }

    #[test]
    fn test_from_i32_trait() {
        let size_type = PhotoSizeType::from(115);
        assert_eq!(size_type.as_i32(), 115);
    }

    // Method tests (12 tests)
    #[test]
    fn test_as_i32() {
        assert_eq!(PhotoSizeType::new('s').as_i32(), 115);
        assert_eq!(PhotoSizeType::new('m').as_i32(), 109);
        assert_eq!(PhotoSizeType::new('x').as_i32(), 120);
    }

    #[test]
    fn test_as_char() {
        assert_eq!(PhotoSizeType::new('s').as_char(), Some('s'));
        assert_eq!(PhotoSizeType::new('m').as_char(), Some('m'));
        assert_eq!(PhotoSizeType::new('x').as_char(), Some('x'));
        assert_eq!(PhotoSizeType::from_i32(-1).as_char(), None);
    }

    #[test]
    fn test_is_small() {
        assert!(PhotoSizeType::new('s').is_small());
        assert!(PhotoSizeType::new('a').is_small());
        assert!(!PhotoSizeType::new('m').is_small());
        assert!(!PhotoSizeType::new('x').is_small());
    }

    #[test]
    fn test_is_medium() {
        assert!(PhotoSizeType::new('m').is_medium());
        assert!(PhotoSizeType::new('b').is_medium());
        assert!(!PhotoSizeType::new('s').is_medium());
        assert!(!PhotoSizeType::new('x').is_medium());
    }

    #[test]
    fn test_is_large() {
        assert!(PhotoSizeType::new('x').is_large());
        assert!(PhotoSizeType::new('y').is_large());
        assert!(PhotoSizeType::new('w').is_large());
        assert!(!PhotoSizeType::new('s').is_large());
        assert!(!PhotoSizeType::new('m').is_large());
    }

    #[test]
    fn test_is_avatar() {
        assert!(PhotoSizeType::new('a').is_avatar());
        assert!(!PhotoSizeType::new('s').is_avatar());
        assert!(!PhotoSizeType::new('m').is_avatar());
    }

    // PartialEq tests (6 tests)
    #[test]
    fn test_eq_char() {
        assert_eq!(PhotoSizeType::new('s'), 's');
        assert_ne!(PhotoSizeType::new('s'), 'm');
    }

    #[test]
    fn test_eq_i32() {
        assert_eq!(PhotoSizeType::new('s'), 115);
        assert_ne!(PhotoSizeType::new('s'), 109);
    }

    // Display tests (3 tests)
    #[test]
    fn test_display_lowercase() {
        assert_eq!(format!("{}", PhotoSizeType::new('s')), "s");
        assert_eq!(format!("{}", PhotoSizeType::new('m')), "m");
        assert_eq!(format!("{}", PhotoSizeType::new('x')), "x");
    }

    #[test]
    fn test_display_uppercase() {
        assert_eq!(format!("{}", PhotoSizeType::new('S')), "S");
    }

    #[test]
    fn test_display_non_char() {
        assert_eq!(format!("{}", PhotoSizeType::from_i32(-1)), "-1");
    }

    // Debug tests (3 tests)
    #[test]
    fn test_debug() {
        let size_type = PhotoSizeType::new('s');
        assert_eq!(
            format!("{:?}", size_type),
            "PhotoSizeType { type_value: 115 }"
        );
    }

    // Round-trip tests (3 tests)
    #[test]
    fn test_round_trip_char() {
        for c in ['s', 'm', 'x', 'y', 'w', 'a', 'b'] {
            let size_type = PhotoSizeType::new(c);
            assert_eq!(size_type.as_char(), Some(c));
        }
    }

    #[test]
    fn test_round_trip_i32() {
        for value in [115, 109, 120, 121, 119, 97, 98] {
            let size_type = PhotoSizeType::from_i32(value);
            assert_eq!(size_type.as_i32(), value);
        }
    }
}
