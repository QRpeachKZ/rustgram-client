//! # Public Dialog Type
//!
//! Type of public dialog in Telegram.
//!
//! ## TDLib Reference
//!
//! - TDLib header: `td/telegram/PublicDialogType.h`
//! - TDLib enum: `PublicDialogType`
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_public_dialog_type::PublicDialogType;
//!
//! let dialog_type = PublicDialogType::HasUsername;
//! ```

use core::fmt;

/// Type of public dialog.
///
/// TDLib: `enum class PublicDialogType : int32`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
#[non_exhaustive]
pub enum PublicDialogType {
    /// Dialog has a public username
    HasUsername = 0,
    /// Dialog is location-based
    IsLocationBased = 1,
    /// Dialog is for personal use
    #[default]
    ForPersonalDialog = 2,
}

impl PublicDialogType {
    /// Get the i32 representation of this type.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_public_dialog_type::PublicDialogType;
    ///
    /// assert_eq!(PublicDialogType::HasUsername.as_i32(), 0);
    /// assert_eq!(PublicDialogType::IsLocationBased.as_i32(), 1);
    /// ```
    #[inline]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Create a PublicDialogType from an i32 value.
    ///
    /// Returns `None` if the value is not a valid type.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_public_dialog_type::PublicDialogType;
    ///
    /// assert_eq!(PublicDialogType::from_i32(0), Some(PublicDialogType::HasUsername));
    /// assert_eq!(PublicDialogType::from_i32(99), None);
    /// ```
    #[inline]
    pub const fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(PublicDialogType::HasUsername),
            1 => Some(PublicDialogType::IsLocationBased),
            2 => Some(PublicDialogType::ForPersonalDialog),
            _ => None,
        }
    }

    /// Check if this type has a username.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_public_dialog_type::PublicDialogType;
    ///
    /// assert!(PublicDialogType::HasUsername.has_username());
    /// assert!(!PublicDialogType::IsLocationBased.has_username());
    /// ```
    #[inline]
    pub const fn has_username(self) -> bool {
        matches!(self, PublicDialogType::HasUsername)
    }

    /// Check if this type is location-based.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_public_dialog_type::PublicDialogType;
    ///
    /// assert!(PublicDialogType::IsLocationBased.is_location_based());
    /// assert!(!PublicDialogType::HasUsername.is_location_based());
    /// ```
    #[inline]
    pub const fn is_location_based(self) -> bool {
        matches!(self, PublicDialogType::IsLocationBased)
    }

    /// Check if this type is for personal dialog.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_public_dialog_type::PublicDialogType;
    ///
    /// assert!(PublicDialogType::ForPersonalDialog.is_for_personal_dialog());
    /// assert!(!PublicDialogType::HasUsername.is_for_personal_dialog());
    /// ```
    #[inline]
    pub const fn is_for_personal_dialog(self) -> bool {
        matches!(self, PublicDialogType::ForPersonalDialog)
    }
}

impl fmt::Display for PublicDialogType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PublicDialogType::HasUsername => write!(f, "HasUsername"),
            PublicDialogType::IsLocationBased => write!(f, "IsLocationBased"),
            PublicDialogType::ForPersonalDialog => write!(f, "ForPersonalDialog"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (10 tests)
    #[test]
    fn test_clone() {
        let a = PublicDialogType::HasUsername;
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_copy() {
        let a = PublicDialogType::HasUsername;
        let b = a;
        assert_eq!(a, PublicDialogType::HasUsername);
        assert_eq!(b, PublicDialogType::HasUsername);
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(PublicDialogType::HasUsername, PublicDialogType::HasUsername);
        assert_ne!(
            PublicDialogType::HasUsername,
            PublicDialogType::IsLocationBased
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(
            PublicDialogType::default(),
            PublicDialogType::ForPersonalDialog
        );
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        PublicDialogType::HasUsername.hash(&mut hasher);
        let h1 = hasher.finish();

        hasher = DefaultHasher::new();
        PublicDialogType::HasUsername.hash(&mut hasher);
        let h2 = hasher.finish();

        assert_eq!(h1, h2);
    }

    // Constructor tests (2 tests)
    #[test]
    fn test_from_i32_valid() {
        assert_eq!(
            PublicDialogType::from_i32(0),
            Some(PublicDialogType::HasUsername)
        );
        assert_eq!(
            PublicDialogType::from_i32(1),
            Some(PublicDialogType::IsLocationBased)
        );
        assert_eq!(
            PublicDialogType::from_i32(2),
            Some(PublicDialogType::ForPersonalDialog)
        );
    }

    #[test]
    fn test_from_i32_invalid() {
        assert_eq!(PublicDialogType::from_i32(-1), None);
        assert_eq!(PublicDialogType::from_i32(3), None);
        assert_eq!(PublicDialogType::from_i32(99), None);
    }

    // Method tests (12 tests)
    #[test]
    fn test_as_i32() {
        assert_eq!(PublicDialogType::HasUsername.as_i32(), 0);
        assert_eq!(PublicDialogType::IsLocationBased.as_i32(), 1);
        assert_eq!(PublicDialogType::ForPersonalDialog.as_i32(), 2);
    }

    #[test]
    fn test_has_username() {
        assert!(PublicDialogType::HasUsername.has_username());
        assert!(!PublicDialogType::IsLocationBased.has_username());
        assert!(!PublicDialogType::ForPersonalDialog.has_username());
    }

    #[test]
    fn test_is_location_based() {
        assert!(PublicDialogType::IsLocationBased.is_location_based());
        assert!(!PublicDialogType::HasUsername.is_location_based());
        assert!(!PublicDialogType::ForPersonalDialog.is_location_based());
    }

    #[test]
    fn test_is_for_personal_dialog() {
        assert!(PublicDialogType::ForPersonalDialog.is_for_personal_dialog());
        assert!(!PublicDialogType::HasUsername.is_for_personal_dialog());
        assert!(!PublicDialogType::IsLocationBased.is_for_personal_dialog());
    }

    // Display tests (3 tests)
    #[test]
    fn test_display() {
        assert_eq!(format!("{}", PublicDialogType::HasUsername), "HasUsername");
        assert_eq!(
            format!("{}", PublicDialogType::IsLocationBased),
            "IsLocationBased"
        );
        assert_eq!(
            format!("{}", PublicDialogType::ForPersonalDialog),
            "ForPersonalDialog"
        );
    }

    // Debug tests (3 tests)
    #[test]
    fn test_debug() {
        assert_eq!(
            format!("{:?}", PublicDialogType::HasUsername),
            "HasUsername"
        );
        assert_eq!(
            format!("{:?}", PublicDialogType::IsLocationBased),
            "IsLocationBased"
        );
        assert_eq!(
            format!("{:?}", PublicDialogType::ForPersonalDialog),
            "ForPersonalDialog"
        );
    }

    // Round-trip tests (3 tests)
    #[test]
    fn test_round_trip() {
        for dialog_type in [
            PublicDialogType::HasUsername,
            PublicDialogType::IsLocationBased,
            PublicDialogType::ForPersonalDialog,
        ] {
            assert_eq!(
                PublicDialogType::from_i32(dialog_type.as_i32()),
                Some(dialog_type)
            );
        }
    }
}
