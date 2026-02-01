//! # Affiliate Type
//!
//! Represents an affiliate reference in the Telegram system.
//!
//! ## Overview
//!
//! This module defines the `AffiliateType` struct, which represents
//! a reference to a dialog (chat/channel/user) that is an affiliate.
//!
//! ## TDLib Correspondence
//!
//! TDLib class: `AffiliateType`
//! - `AffiliateType` → TDLib `AffiliateType` (wrapper around DialogId)
//!
//! ## Examples
//!
//! ```
//! use rustgram_affiliate_type::AffiliateType;
//! use rustgram_dialog_id::DialogId;
//!
//! // Create affiliate type from dialog ID
//! let dialog_id = DialogId::new(123456);
//! let affiliate = AffiliateType::new(dialog_id);
//!
//! // Get the dialog ID
//! assert_eq!(affiliate.dialog_id(), dialog_id);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

use core::fmt;
use rustgram_dialog_id::DialogId;

/// Represents an affiliate reference in the Telegram system.
///
/// An affiliate type is essentially a wrapper around a `DialogId` that
/// identifies which dialog is the affiliate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AffiliateType {
    /// The dialog ID of the affiliate.
    dialog_id: DialogId,
}

impl Default for AffiliateType {
    fn default() -> Self {
        Self::new(DialogId::new(0))
    }
}

impl AffiliateType {
    /// Creates a new AffiliateType from a DialogId.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_affiliate_type::AffiliateType;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog_id = DialogId::new(123456);
    /// let affiliate = AffiliateType::new(dialog_id);
    /// ```
    #[must_use]
    pub const fn new(dialog_id: DialogId) -> Self {
        Self { dialog_id }
    }

    /// Returns the dialog ID of this affiliate.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_affiliate_type::AffiliateType;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog_id = DialogId::new(123456);
    /// let affiliate = AffiliateType::new(dialog_id);
    /// assert_eq!(affiliate.dialog_id(), dialog_id);
    /// ```
    #[must_use]
    pub const fn dialog_id(self) -> DialogId {
        self.dialog_id
    }

    /// Checks if this affiliate type is valid (has a non-zero dialog ID).
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_affiliate_type::AffiliateType;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let valid = AffiliateType::new(DialogId::new(123456));
    /// assert!(valid.is_valid());
    ///
    /// let invalid = AffiliateType::new(DialogId::new(0));
    /// assert!(!invalid.is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.dialog_id.is_valid()
    }

    /// Creates an AffiliateType from an i64 value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_affiliate_type::AffiliateType;
    ///
    /// let affiliate = AffiliateType::from_i64(123456);
    /// assert_eq!(affiliate.dialog_id().get(), 123456);
    /// ```
    #[must_use]
    pub const fn from_i64(id: i64) -> Self {
        Self {
            dialog_id: DialogId::new(id),
        }
    }
}

impl From<DialogId> for AffiliateType {
    fn from(dialog_id: DialogId) -> Self {
        Self::new(dialog_id)
    }
}

impl From<AffiliateType> for DialogId {
    fn from(affiliate: AffiliateType) -> Self {
        affiliate.dialog_id
    }
}

impl From<i64> for AffiliateType {
    fn from(id: i64) -> Self {
        Self::from_i64(id)
    }
}

impl fmt::Display for AffiliateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AffiliateType({})", self.dialog_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic trait tests (6)
    #[test]
    fn test_default() {
        let affiliate = AffiliateType::default();
        assert_eq!(affiliate.dialog_id().get(), 0);
    }

    #[test]
    fn test_copy() {
        let affiliate = AffiliateType::new(DialogId::new(123456));
        let copy = affiliate;
        assert_eq!(affiliate.dialog_id().get(), 123456);
        assert_eq!(copy.dialog_id().get(), 123456);
    }

    #[test]
    fn test_clone() {
        let affiliate = AffiliateType::new(DialogId::new(123456));
        let cloned = affiliate.clone();
        assert_eq!(affiliate, cloned);
    }

    #[test]
    fn test_partial_eq() {
        let id = DialogId::new(123456);
        assert_eq!(AffiliateType::new(id), AffiliateType::new(id));
        assert_ne!(
            AffiliateType::new(DialogId::new(123456)),
            AffiliateType::new(DialogId::new(789012))
        );
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let id = DialogId::new(123456);
        set.insert(AffiliateType::new(id));
        set.insert(AffiliateType::new(id));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<AffiliateType>();
        assert_sync::<AffiliateType>();
    }

    // new() tests (2)
    #[test]
    fn test_new() {
        let dialog_id = DialogId::new(123456);
        let affiliate = AffiliateType::new(dialog_id);
        assert_eq!(affiliate.dialog_id(), dialog_id);
    }

    #[test]
    fn test_new_zero() {
        let dialog_id = DialogId::new(0);
        let affiliate = AffiliateType::new(dialog_id);
        assert_eq!(affiliate.dialog_id().get(), 0);
    }

    // dialog_id() tests (2)
    #[test]
    fn test_dialog_id() {
        let dialog_id = DialogId::new(123456);
        let affiliate = AffiliateType::new(dialog_id);
        assert_eq!(affiliate.dialog_id(), dialog_id);
    }

    #[test]
    fn test_dialog_id_get() {
        let affiliate = AffiliateType::new(DialogId::new(123456));
        assert_eq!(affiliate.dialog_id().get(), 123456);
    }

    // is_valid() tests (2)
    #[test]
    fn test_is_valid_true() {
        let affiliate = AffiliateType::new(DialogId::new(123456));
        assert!(affiliate.is_valid());
    }

    #[test]
    fn test_is_valid_false() {
        let affiliate = AffiliateType::new(DialogId::new(0));
        assert!(!affiliate.is_valid());
    }

    // from_i64() tests (2)
    #[test]
    fn test_from_i64() {
        let affiliate = AffiliateType::from_i64(123456);
        assert_eq!(affiliate.dialog_id().get(), 123456);
    }

    #[test]
    fn test_from_i64_zero() {
        let affiliate = AffiliateType::from_i64(0);
        assert_eq!(affiliate.dialog_id().get(), 0);
    }

    // From trait tests (3)
    #[test]
    fn test_from_dialog_id() {
        let dialog_id = DialogId::new(123456);
        let affiliate: AffiliateType = dialog_id.into();
        assert_eq!(affiliate.dialog_id(), dialog_id);
    }

    #[test]
    fn test_from_affiliate_type_to_dialog_id() {
        let affiliate = AffiliateType::new(DialogId::new(123456));
        let dialog_id: DialogId = affiliate.into();
        assert_eq!(dialog_id.get(), 123456);
    }

    #[test]
    fn test_from_i64_trait() {
        let affiliate: AffiliateType = 123456.into();
        assert_eq!(affiliate.dialog_id().get(), 123456);
    }

    // Display tests (2)
    #[test]
    fn test_display() {
        let affiliate = AffiliateType::new(DialogId::new(123456));
        let display = format!("{}", affiliate);
        assert!(display.contains("AffiliateType"));
        assert!(display.contains("123456"));
    }

    #[test]
    fn test_display_zero() {
        let affiliate = AffiliateType::new(DialogId::new(0));
        let display = format!("{}", affiliate);
        assert!(display.contains("AffiliateType"));
    }

    // Edge case tests (3)
    #[test]
    fn test_negative_id() {
        let affiliate = AffiliateType::new(DialogId::new(-123456));
        assert_eq!(affiliate.dialog_id().get(), -123456);
        assert!(affiliate.is_valid());
    }

    #[test]
    fn test_max_id() {
        let affiliate = AffiliateType::new(DialogId::new(i64::MAX));
        assert_eq!(affiliate.dialog_id().get(), i64::MAX);
        assert!(affiliate.is_valid());
    }

    #[test]
    fn test_min_id() {
        let affiliate = AffiliateType::new(DialogId::new(i64::MIN));
        assert_eq!(affiliate.dialog_id().get(), i64::MIN);
        assert!(affiliate.is_valid());
    }
}
