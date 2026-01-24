//! # Input Dialog ID
//!
//! Wrapper around `DialogId` with access hash for TL operations.
//!
//! ## Overview
//!
//! `InputDialogId` combines a `DialogId` with an access hash, which is needed
//! for Telegram API operations that reference dialogs.
//!
//! ## Usage
//!
//! ```
//! use rustgram_dialog_id::DialogId;
//! use rustgram_input_dialog_id::InputDialogId;
//!
//! let dialog_id = DialogId::new(123);
//! let input = InputDialogId::new(dialog_id, 456);
//! assert!(input.is_valid());
//! ```

use rustgram_dialog_id::DialogId;
use core::fmt;

/// A dialog identifier with access hash for API operations.
///
/// This type wraps a `DialogId` with an access hash that is required
/// for certain Telegram API operations.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_id::DialogId;
/// use rustgram_input_dialog_id::InputDialogId;
///
/// let dialog_id = DialogId::new(123);
/// let input = InputDialogId::new(dialog_id, 456);
/// assert_eq!(input.dialog_id(), dialog_id);
/// assert_eq!(input.access_hash(), 456);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputDialogId {
    dialog_id: DialogId,
    access_hash: i64,
}

impl InputDialogId {
    /// Creates a new `InputDialogId` with the given dialog ID and access hash.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog identifier
    /// * `access_hash` - The access hash for API operations
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_input_dialog_id::InputDialogId;
    ///
    /// let dialog_id = DialogId::new(123);
    /// let input = InputDialogId::new(dialog_id, 456);
    /// ```
    #[inline]
    pub const fn new(dialog_id: DialogId, access_hash: i64) -> Self {
        Self {
            dialog_id,
            access_hash,
        }
    }

    /// Returns the dialog ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_input_dialog_id::InputDialogId;
    ///
    /// let dialog_id = DialogId::new(123);
    /// let input = InputDialogId::new(dialog_id, 456);
    /// assert_eq!(input.dialog_id(), dialog_id);
    /// ```
    #[inline]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Returns the access hash.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_input_dialog_id::InputDialogId;
    ///
    /// let dialog_id = DialogId::new(123);
    /// let input = InputDialogId::new(dialog_id, 456);
    /// assert_eq!(input.access_hash(), 456);
    /// ```
    #[inline]
    pub const fn access_hash(&self) -> i64 {
        self.access_hash
    }

    /// Returns `true` if this input dialog ID is valid.
    ///
    /// An input dialog ID is valid if its underlying dialog ID is valid.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_input_dialog_id::InputDialogId;
    ///
    /// let dialog_id = DialogId::new(123);
    /// let input = InputDialogId::new(dialog_id, 456);
    /// assert!(input.is_valid());
    /// ```
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.dialog_id.is_valid()
    }
}

impl Default for InputDialogId {
    fn default() -> Self {
        Self {
            dialog_id: DialogId::default(),
            access_hash: 0,
        }
    }
}

impl fmt::Display for InputDialogId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "input {}", self.dialog_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_new() {
        let dialog_id = DialogId::new(123);
        let input = InputDialogId::new(dialog_id, 456);
        assert_eq!(input.dialog_id(), dialog_id);
        assert_eq!(input.access_hash(), 456);
    }

    #[test]
    fn test_dialog_id() {
        let dialog_id = DialogId::new(789);
        let input = InputDialogId::new(dialog_id, 0);
        assert_eq!(input.dialog_id(), dialog_id);
    }

    #[test]
    fn test_access_hash() {
        let dialog_id = DialogId::new(123);
        let input = InputDialogId::new(dialog_id, 456);
        assert_eq!(input.access_hash(), 456);
    }

    #[rstest]
    #[case(1, 0, true)]
    #[case(100, 500, true)]
    #[case(0, 100, false)]
    #[case(-1, 100, false)]
    fn test_is_valid(#[case] dialog_id_value: i64, #[case] access_hash: i64, #[case] expected: bool) {
        let dialog_id = DialogId::new(dialog_id_value);
        let input = InputDialogId::new(dialog_id, access_hash);
        assert_eq!(input.is_valid(), expected);
    }

    #[test]
    fn test_default() {
        let input = InputDialogId::default();
        assert!(!input.is_valid());
        assert_eq!(input.access_hash(), 0);
    }

    #[test]
    fn test_equality() {
        let dialog_id = DialogId::new(123);
        let input1 = InputDialogId::new(dialog_id, 456);
        let input2 = InputDialogId::new(dialog_id, 456);
        let input3 = InputDialogId::new(dialog_id, 789);

        assert_eq!(input1, input2);
        assert_ne!(input1, input3);
    }

    #[test]
    fn test_equality_different_dialog_id() {
        let dialog_id1 = DialogId::new(123);
        let dialog_id2 = DialogId::new(456);
        let input1 = InputDialogId::new(dialog_id1, 789);
        let input2 = InputDialogId::new(dialog_id2, 789);

        assert_ne!(input1, input2);
    }

    #[test]
    fn test_copy() {
        let dialog_id = DialogId::new(123);
        let input1 = InputDialogId::new(dialog_id, 456);
        let input2 = input1;
        assert_eq!(input1, input2);
    }

    #[test]
    fn test_clone() {
        let dialog_id = DialogId::new(123);
        let input1 = InputDialogId::new(dialog_id, 456);
        let input2 = input1.clone();
        assert_eq!(input1, input2);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let dialog_id = DialogId::new(123);
        let mut set = HashSet::new();
        set.insert(InputDialogId::new(dialog_id, 456));
        set.insert(InputDialogId::new(dialog_id, 789));
        set.insert(InputDialogId::new(dialog_id, 456)); // Duplicate
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_display() {
        let dialog_id = DialogId::new(123);
        let input = InputDialogId::new(dialog_id, 456);
        assert_eq!(format!("{}", input), "input dialog 123");
    }

    #[test]
    fn test_debug() {
        let dialog_id = DialogId::new(123);
        let input = InputDialogId::new(dialog_id, 456);
        let debug = format!("{:?}", input);
        assert!(debug.contains("InputDialogId"));
    }

    #[rstest]
    #[case(0)]
    #[case(100)]
    #[case(-100)]
    #[case(i64::MAX)]
    #[case(i64::MIN)]
    fn test_access_hash_values(#[case] access_hash: i64) {
        let dialog_id = DialogId::new(123);
        let input = InputDialogId::new(dialog_id, access_hash);
        assert_eq!(input.access_hash(), access_hash);
    }
}
