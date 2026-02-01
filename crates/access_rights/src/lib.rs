//! # Access Rights
//!
//! Represents access permission levels in the Telegram system.
//!
//! ## Overview
//!
//! This module defines the `AccessRights` enum, which represents different
//! permission levels for accessing resources in Telegram.
//!
//! ## TDLib Correspondence
//!
//! TDLib enum: `AccessRights`
//! - `AccessRights::Know` → TDLib `AccessRights::Know`
//! - `AccessRights::Read` → TDLib `AccessRights::Read`
//! - `AccessRights::Edit` → TDLib `AccessRights::Edit`
//! - `AccessRights::Write` → TDLib `AccessRights::Write`
//!
//! ## Examples
//!
//! ```
//! use rustgram_access_rights::AccessRights;
//!
//! // Create access rights
//! let rights = AccessRights::Read;
//!
//! // Check if user can edit
//! if rights.can_edit() {
//!     println!("User has edit permissions");
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use core::fmt;

/// Represents access permission levels in the Telegram system.
///
/// This enum defines hierarchical permission levels, where higher levels
/// include all permissions from lower levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum AccessRights {
    /// Know - Basic awareness of a resource
    Know = 0,
    /// Read - Can view the resource
    Read = 1,
    /// Edit - Can modify the resource
    Edit = 2,
    /// Write - Full control including creation
    Write = 3,
}

impl Default for AccessRights {
    fn default() -> Self {
        Self::Know
    }
}

impl AccessRights {
    /// Creates AccessRights from an i32 value.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not in the range 0..=3.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_access_rights::AccessRights;
    ///
    /// assert_eq!(AccessRights::from_i32(0), Ok(AccessRights::Know));
    /// assert_eq!(AccessRights::from_i32(3), Ok(AccessRights::Write));
    /// assert!(AccessRights::from_i32(99).is_err());
    /// ```
    pub fn from_i32(value: i32) -> Result<Self, Error> {
        match value {
            0 => Ok(Self::Know),
            1 => Ok(Self::Read),
            2 => Ok(Self::Edit),
            3 => Ok(Self::Write),
            _ => Err(Error::InvalidValue(value)),
        }
    }

    /// Returns the i32 representation of this AccessRights.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_access_rights::AccessRights;
    ///
    /// assert_eq!(AccessRights::Know.as_i32(), 0);
    /// assert_eq!(AccessRights::Read.as_i32(), 1);
    /// ```
    #[must_use]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }

    /// Checks if this access level includes read permissions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_access_rights::AccessRights;
    ///
    /// assert!(!AccessRights::Know.can_read());
    /// assert!(AccessRights::Read.can_read());
    /// assert!(AccessRights::Edit.can_read());
    /// ```
    #[must_use]
    pub const fn can_read(self) -> bool {
        (self as i32) >= (Self::Read as i32)
    }

    /// Checks if this access level includes edit permissions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_access_rights::AccessRights;
    ///
    /// assert!(!AccessRights::Read.can_edit());
    /// assert!(AccessRights::Edit.can_edit());
    /// assert!(AccessRights::Write.can_edit());
    /// ```
    #[must_use]
    pub const fn can_edit(self) -> bool {
        (self as i32) >= (Self::Edit as i32)
    }

    /// Checks if this access level includes write permissions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_access_rights::AccessRights;
    ///
    /// assert!(!AccessRights::Edit.can_write());
    /// assert!(AccessRights::Write.can_write());
    /// ```
    #[must_use]
    pub const fn can_write(self) -> bool {
        (self as i32) >= (Self::Write as i32)
    }

    /// Returns all access rights variants.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_access_rights::AccessRights;
    ///
    /// let all = AccessRights::all();
    /// assert_eq!(all.len(), 4);
    /// ```
    #[must_use]
    pub fn all() -> &'static [Self] {
        &[Self::Know, Self::Read, Self::Edit, Self::Write]
    }
}

impl fmt::Display for AccessRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Know => write!(f, "Know"),
            Self::Read => write!(f, "Read"),
            Self::Edit => write!(f, "Edit"),
            Self::Write => write!(f, "Write"),
        }
    }
}

/// Error type for AccessRights operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Invalid i32 value for AccessRights
    InvalidValue(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidValue(v) => write!(f, "Invalid AccessRights value: {}", v),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (6)
    #[test]
    fn test_default() {
        assert_eq!(AccessRights::default(), AccessRights::Know);
    }

    #[test]
    fn test_copy() {
        let rights = AccessRights::Read;
        let copy = rights;
        assert_eq!(rights, AccessRights::Read);
        assert_eq!(copy, AccessRights::Read);
    }

    #[test]
    fn test_clone() {
        let rights = AccessRights::Edit;
        let cloned = rights.clone();
        assert_eq!(rights, cloned);
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(AccessRights::Read, AccessRights::Read);
        assert_ne!(AccessRights::Read, AccessRights::Write);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(AccessRights::Read);
        set.insert(AccessRights::Read);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_ord() {
        assert!(AccessRights::Know < AccessRights::Read);
        assert!(AccessRights::Read < AccessRights::Edit);
        assert!(AccessRights::Edit < AccessRights::Write);
    }

    // from_i32 tests (3)
    #[test]
    fn test_from_i32_valid() {
        assert_eq!(AccessRights::from_i32(0), Ok(AccessRights::Know));
        assert_eq!(AccessRights::from_i32(1), Ok(AccessRights::Read));
        assert_eq!(AccessRights::from_i32(2), Ok(AccessRights::Edit));
        assert_eq!(AccessRights::from_i32(3), Ok(AccessRights::Write));
    }

    #[test]
    fn test_from_i32_invalid() {
        assert_eq!(AccessRights::from_i32(-1), Err(Error::InvalidValue(-1)));
        assert_eq!(AccessRights::from_i32(4), Err(Error::InvalidValue(4)));
        assert_eq!(AccessRights::from_i32(99), Err(Error::InvalidValue(99)));
    }

    #[test]
    fn test_from_i32_roundtrip() {
        for rights in AccessRights::all() {
            assert_eq!(AccessRights::from_i32(rights.as_i32()), Ok(*rights));
        }
    }

    // as_i32 tests (2)
    #[test]
    fn test_as_i32() {
        assert_eq!(AccessRights::Know.as_i32(), 0);
        assert_eq!(AccessRights::Read.as_i32(), 1);
        assert_eq!(AccessRights::Edit.as_i32(), 2);
        assert_eq!(AccessRights::Write.as_i32(), 3);
    }

    // Permission check tests (4)
    #[test]
    fn test_can_read() {
        assert!(!AccessRights::Know.can_read());
        assert!(AccessRights::Read.can_read());
        assert!(AccessRights::Edit.can_read());
        assert!(AccessRights::Write.can_read());
    }

    #[test]
    fn test_can_edit() {
        assert!(!AccessRights::Know.can_edit());
        assert!(!AccessRights::Read.can_edit());
        assert!(AccessRights::Edit.can_edit());
        assert!(AccessRights::Write.can_edit());
    }

    #[test]
    fn test_can_write() {
        assert!(!AccessRights::Know.can_write());
        assert!(!AccessRights::Read.can_write());
        assert!(!AccessRights::Edit.can_write());
        assert!(AccessRights::Write.can_write());
    }

    #[test]
    fn test_permission_hierarchy() {
        let write = AccessRights::Write;
        assert!(write.can_read() && write.can_edit() && write.can_write());
    }

    // all() tests (2)
    #[test]
    fn test_all_count() {
        assert_eq!(AccessRights::all().len(), 4);
    }

    #[test]
    fn test_all_contains_all() {
        let all = AccessRights::all();
        assert!(all.contains(&AccessRights::Know));
        assert!(all.contains(&AccessRights::Read));
        assert!(all.contains(&AccessRights::Edit));
        assert!(all.contains(&AccessRights::Write));
    }

    // Display tests (2)
    #[test]
    fn test_display() {
        assert_eq!(format!("{}", AccessRights::Know), "Know");
        assert_eq!(format!("{}", AccessRights::Read), "Read");
        assert_eq!(format!("{}", AccessRights::Edit), "Edit");
        assert_eq!(format!("{}", AccessRights::Write), "Write");
    }

    // Error tests (2)
    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", Error::InvalidValue(99)),
            "Invalid AccessRights value: 99"
        );
    }

    #[test]
    fn test_error_partial_eq() {
        assert_eq!(Error::InvalidValue(1), Error::InvalidValue(1));
        assert_ne!(Error::InvalidValue(1), Error::InvalidValue(2));
    }

    // Const eval tests (2)
    #[test]
    fn test_const_as_i32() {
        const VALUE: i32 = AccessRights::Read.as_i32();
        assert_eq!(VALUE, 1);
    }

    #[test]
    fn test_const_can_read() {
        const CAN_EDIT: bool = AccessRights::Edit.can_read();
        assert!(CAN_EDIT);
    }
}
