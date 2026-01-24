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

//! # Paid Reaction Type
//!
//! Paid reaction type representation for Telegram MTProto client.
//!
//! ## Overview
//!
//! Telegram supports different privacy settings for paid reactions:
//!
//! - **Regular**: Public paid reaction (visible to all)
//! - **Anonymous**: Anonymous paid reaction (sender hidden)
//! - **Dialog**: Dialog-specific paid reaction (visible to specific dialog)
//!
//! ## TDLib Alignment
//!
//! This implementation aligns with TDLib's `PaidReactionType` from `td/telegram/PaidReactionType.h`:
//! - `Type::Regular` maps to regular public paid reactions
//! - `Type::Anonymous` maps to anonymous paid reactions
//! - `Type::Dialog` maps to dialog-specific paid reactions with a DialogId
//!
//! ## Examples
//!
//! Creating paid reaction types:
//!
//! ```rust
//! use rustgram_paid_reaction_type::PaidReactionType;
//!
//! // Regular (public) paid reaction
//! let regular = PaidReactionType::regular();
//! assert!(regular.is_regular());
//! assert!(regular.is_valid());
//!
//! // Anonymous paid reaction
//! let anonymous = PaidReactionType::anonymous();
//! assert!(anonymous.is_anonymous());
//! assert!(anonymous.is_valid());
//! ```
//!
//! Creating dialog-specific paid reactions:
//!
//! ```rust
//! use rustgram_paid_reaction_type::PaidReactionType;
//! use rustgram_dialog_id::DialogId;
//!
//! let dialog_id = DialogId::new(123456);
//! let dialog = PaidReactionType::dialog(dialog_id);
//! assert!(dialog.is_dialog());
//! assert!(dialog.is_valid());
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::len_without_is_empty)]

use rustgram_dialog_id::DialogId;
use std::fmt;

/// Paid reaction type.
///
/// Represents the privacy setting for a paid reaction in Telegram.
/// Paid reactions can be public, anonymous, or restricted to a specific dialog.
///
/// # TDLib Correspondence
///
/// Corresponds to TDLib `PaidReactionType` with three variants:
/// - `Type::Regular` - Public paid reactions
/// - `Type::Anonymous` - Anonymous paid reactions
/// - `Type::Dialog` - Dialog-specific paid reactions
///
/// # Examples
///
/// ```
/// use rustgram_paid_reaction_type::PaidReactionType;
///
/// let regular = PaidReactionType::regular();
/// assert!(regular.is_regular());
///
/// let anonymous = PaidReactionType::anonymous();
/// assert!(anonymous.is_anonymous());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaidReactionType {
    /// Regular (public) paid reaction.
    ///
    /// The sender of the paid reaction is visible to all users.
    Regular,
    /// Anonymous paid reaction.
    ///
    /// The sender of the paid reaction is hidden.
    Anonymous,
    /// Dialog-specific paid reaction.
    ///
    /// The paid reaction is only visible to the specified dialog.
    Dialog(DialogId),
}

impl PaidReactionType {
    /// Creates a regular (public) paid reaction type.
    ///
    /// Regular paid reactions show the sender to all users.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_paid_reaction_type::PaidReactionType;
    ///
    /// let reaction_type = PaidReactionType::regular();
    /// assert!(reaction_type.is_regular());
    /// ```
    #[must_use]
    pub fn regular() -> Self {
        Self::Regular
    }

    /// Creates an anonymous paid reaction type.
    ///
    /// Anonymous paid reactions hide the sender from all users.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_paid_reaction_type::PaidReactionType;
    ///
    /// let reaction_type = PaidReactionType::anonymous();
    /// assert!(reaction_type.is_anonymous());
    /// ```
    #[must_use]
    pub fn anonymous() -> Self {
        Self::Anonymous
    }

    /// Creates a dialog-specific paid reaction type.
    ///
    /// Dialog-specific paid reactions are only visible to the specified dialog.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - The dialog that can view this paid reaction
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_paid_reaction_type::PaidReactionType;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog_id = DialogId::new(123456);
    /// let reaction_type = PaidReactionType::dialog(dialog_id);
    /// assert!(reaction_type.is_dialog());
    /// ```
    #[must_use]
    pub const fn dialog(dialog_id: DialogId) -> Self {
        Self::Dialog(dialog_id)
    }

    /// Creates a legacy anonymous paid reaction type.
    ///
    /// This is for compatibility with older versions of the API.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_paid_reaction_type::PaidReactionType;
    ///
    /// let reaction_type = PaidReactionType::legacy(true);
    /// assert!(reaction_type.is_anonymous());
    ///
    /// let reaction_type = PaidReactionType::legacy(false);
    /// assert!(reaction_type.is_regular());
    /// ```
    #[must_use]
    pub fn legacy(is_anonymous: bool) -> Self {
        if is_anonymous {
            Self::Anonymous
        } else {
            Self::Regular
        }
    }

    /// Checks if this is a regular (public) paid reaction type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_paid_reaction_type::PaidReactionType;
    ///
    /// assert!(PaidReactionType::regular().is_regular());
    /// assert!(!PaidReactionType::anonymous().is_regular());
    /// ```
    #[must_use]
    pub const fn is_regular(&self) -> bool {
        matches!(self, Self::Regular)
    }

    /// Checks if this is an anonymous paid reaction type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_paid_reaction_type::PaidReactionType;
    ///
    /// assert!(!PaidReactionType::regular().is_anonymous());
    /// assert!(PaidReactionType::anonymous().is_anonymous());
    /// ```
    #[must_use]
    pub const fn is_anonymous(&self) -> bool {
        matches!(self, Self::Anonymous)
    }

    /// Checks if this is a dialog-specific paid reaction type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_paid_reaction_type::PaidReactionType;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog_id = DialogId::new(123456);
    /// let reaction_type = PaidReactionType::dialog(dialog_id);
    /// assert!(reaction_type.is_dialog());
    /// ```
    #[must_use]
    pub const fn is_dialog(&self) -> bool {
        matches!(self, Self::Dialog(_))
    }

    /// Checks if this is a valid paid reaction type.
    ///
    /// Dialog-specific reactions must have a valid dialog ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_paid_reaction_type::PaidReactionType;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// assert!(PaidReactionType::regular().is_valid());
    /// assert!(PaidReactionType::anonymous().is_valid());
    ///
    /// let valid_dialog = DialogId::new(123456);
    /// assert!(PaidReactionType::dialog(valid_dialog).is_valid());
    ///
    /// let invalid_dialog = DialogId::new(0);
    /// assert!(!PaidReactionType::dialog(invalid_dialog).is_valid());
    /// ```
    #[must_use]
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Regular | Self::Anonymous => true,
            Self::Dialog(dialog_id) => dialog_id.is_valid(),
        }
    }

    /// Gets the dialog ID if this is a dialog-specific paid reaction.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_paid_reaction_type::PaidReactionType;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let dialog_id = DialogId::new(123456);
    /// let reaction_type = PaidReactionType::dialog(dialog_id);
    /// assert_eq!(reaction_type.dialog_id(), Some(dialog_id));
    ///
    /// assert_eq!(PaidReactionType::regular().dialog_id(), None);
    /// ```
    #[must_use]
    pub const fn dialog_id(&self) -> Option<DialogId> {
        match self {
            Self::Dialog(dialog_id) => Some(*dialog_id),
            _ => None,
        }
    }

    /// Gets the dialog ID for a paid reaction, using a default if not dialog-specific.
    ///
    /// If this is a regular or anonymous reaction, returns the provided default dialog ID.
    ///
    /// # Arguments
    ///
    /// * `my_dialog_id` - The default dialog ID to return for non-dialog reactions
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_paid_reaction_type::PaidReactionType;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let my_dialog_id = DialogId::new(999999);
    ///
    /// assert_eq!(PaidReactionType::regular().get_dialog_id(my_dialog_id), my_dialog_id);
    /// assert_eq!(PaidReactionType::anonymous().get_dialog_id(my_dialog_id), my_dialog_id);
    ///
    /// let dialog_id = DialogId::new(123456);
    /// let reaction_type = PaidReactionType::dialog(dialog_id);
    /// assert_eq!(reaction_type.get_dialog_id(my_dialog_id), dialog_id);
    /// ```
    #[must_use]
    pub const fn get_dialog_id(&self, my_dialog_id: DialogId) -> DialogId {
        match self {
            Self::Dialog(dialog_id) => *dialog_id,
            _ => my_dialog_id,
        }
    }
}

impl Default for PaidReactionType {
    /// Returns a regular paid reaction type as the default.
    fn default() -> Self {
        Self::Regular
    }
}

impl fmt::Display for PaidReactionType {
    /// Formats the paid reaction type for display.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular => write!(f, "Regular"),
            Self::Anonymous => write!(f, "Anonymous"),
            Self::Dialog(dialog_id) => write!(f, "Dialog({})", dialog_id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Constructor Tests ===

    #[test]
    fn test_regular() {
        let reaction_type = PaidReactionType::regular();
        assert!(reaction_type.is_regular());
        assert!(!reaction_type.is_anonymous());
        assert!(!reaction_type.is_dialog());
        assert!(reaction_type.is_valid());
    }

    #[test]
    fn test_anonymous() {
        let reaction_type = PaidReactionType::anonymous();
        assert!(!reaction_type.is_regular());
        assert!(reaction_type.is_anonymous());
        assert!(!reaction_type.is_dialog());
        assert!(reaction_type.is_valid());
    }

    #[test]
    fn test_dialog_valid() {
        let dialog_id = DialogId::new(123456);
        let reaction_type = PaidReactionType::dialog(dialog_id);
        assert!(!reaction_type.is_regular());
        assert!(!reaction_type.is_anonymous());
        assert!(reaction_type.is_dialog());
        assert!(reaction_type.is_valid());
        assert_eq!(reaction_type.dialog_id(), Some(dialog_id));
    }

    #[test]
    fn test_dialog_invalid() {
        let dialog_id = DialogId::new(0);
        let reaction_type = PaidReactionType::dialog(dialog_id);
        assert!(reaction_type.is_dialog());
        assert!(!reaction_type.is_valid());
        assert_eq!(reaction_type.dialog_id(), Some(dialog_id));
    }

    #[test]
    fn test_legacy_anonymous() {
        let reaction_type = PaidReactionType::legacy(true);
        assert!(reaction_type.is_anonymous());
        assert!(reaction_type.is_valid());
    }

    #[test]
    fn test_legacy_regular() {
        let reaction_type = PaidReactionType::legacy(false);
        assert!(reaction_type.is_regular());
        assert!(reaction_type.is_valid());
    }

    // === Type Query Tests ===

    #[test]
    fn test_is_regular() {
        assert!(PaidReactionType::regular().is_regular());
        assert!(!PaidReactionType::anonymous().is_regular());
        assert!(!PaidReactionType::dialog(DialogId::new(123456)).is_regular());
    }

    #[test]
    fn test_is_anonymous() {
        assert!(!PaidReactionType::regular().is_anonymous());
        assert!(PaidReactionType::anonymous().is_anonymous());
        assert!(!PaidReactionType::dialog(DialogId::new(123456)).is_anonymous());
    }

    #[test]
    fn test_is_dialog() {
        assert!(!PaidReactionType::regular().is_dialog());
        assert!(!PaidReactionType::anonymous().is_dialog());
        assert!(PaidReactionType::dialog(DialogId::new(123456)).is_dialog());
    }

    // === Validation Tests ===

    #[test]
    fn test_is_valid_regular() {
        assert!(PaidReactionType::regular().is_valid());
    }

    #[test]
    fn test_is_valid_anonymous() {
        assert!(PaidReactionType::anonymous().is_valid());
    }

    #[test]
    fn test_is_valid_dialog_with_valid_id() {
        let dialog_id = DialogId::new(123456);
        assert!(PaidReactionType::dialog(dialog_id).is_valid());
    }

    #[test]
    fn test_is_valid_dialog_with_invalid_id() {
        let dialog_id = DialogId::new(0);
        assert!(!PaidReactionType::dialog(dialog_id).is_valid());
    }

    #[test]
    fn test_is_valid_dialog_with_negative_id() {
        let dialog_id = DialogId::new(-123456);
        assert!(PaidReactionType::dialog(dialog_id).is_valid());
    }

    // === dialog_id() Tests ===

    #[test]
    fn test_dialog_id_regular() {
        assert_eq!(PaidReactionType::regular().dialog_id(), None);
    }

    #[test]
    fn test_dialog_id_anonymous() {
        assert_eq!(PaidReactionType::anonymous().dialog_id(), None);
    }

    #[test]
    fn test_dialog_id_dialog() {
        let dialog_id = DialogId::new(123456);
        let reaction_type = PaidReactionType::dialog(dialog_id);
        assert_eq!(reaction_type.dialog_id(), Some(dialog_id));
    }

    // === get_dialog_id() Tests ===

    #[test]
    fn test_get_dialog_id_regular() {
        let my_dialog_id = DialogId::new(999999);
        assert_eq!(
            PaidReactionType::regular().get_dialog_id(my_dialog_id),
            my_dialog_id
        );
    }

    #[test]
    fn test_get_dialog_id_anonymous() {
        let my_dialog_id = DialogId::new(999999);
        assert_eq!(
            PaidReactionType::anonymous().get_dialog_id(my_dialog_id),
            my_dialog_id
        );
    }

    #[test]
    fn test_get_dialog_id_dialog() {
        let my_dialog_id = DialogId::new(999999);
        let dialog_id = DialogId::new(123456);
        let reaction_type = PaidReactionType::dialog(dialog_id);
        assert_eq!(reaction_type.get_dialog_id(my_dialog_id), dialog_id);
    }

    // === Equality Tests ===

    #[test]
    fn test_eq_regular() {
        let r1 = PaidReactionType::regular();
        let r2 = PaidReactionType::regular();
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_eq_anonymous() {
        let r1 = PaidReactionType::anonymous();
        let r2 = PaidReactionType::anonymous();
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_eq_dialog() {
        let dialog_id = DialogId::new(123456);
        let r1 = PaidReactionType::dialog(dialog_id);
        let r2 = PaidReactionType::dialog(dialog_id);
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_ne_different_types() {
        let regular = PaidReactionType::regular();
        let anonymous = PaidReactionType::anonymous();
        assert_ne!(regular, anonymous);

        let dialog_id = DialogId::new(123456);
        let dialog = PaidReactionType::dialog(dialog_id);
        assert_ne!(regular, dialog);
        assert_ne!(anonymous, dialog);
    }

    #[test]
    fn test_ne_different_dialog_ids() {
        let r1 = PaidReactionType::dialog(DialogId::new(123456));
        let r2 = PaidReactionType::dialog(DialogId::new(789012));
        assert_ne!(r1, r2);
    }

    // === Clone Tests ===

    #[test]
    fn test_clone_regular() {
        let r1 = PaidReactionType::regular();
        let r2 = r1.clone();
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_clone_anonymous() {
        let r1 = PaidReactionType::anonymous();
        let r2 = r1.clone();
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_clone_dialog() {
        let r1 = PaidReactionType::dialog(DialogId::new(123456));
        let r2 = r1.clone();
        assert_eq!(r1, r2);
    }

    // === Default Tests ===

    #[test]
    fn test_default() {
        let reaction_type = PaidReactionType::default();
        assert!(reaction_type.is_regular());
        assert!(reaction_type.is_valid());
    }

    // === Display Tests ===

    #[test]
    fn test_display_regular() {
        let reaction_type = PaidReactionType::regular();
        assert_eq!(format!("{}", reaction_type), "Regular");
    }

    #[test]
    fn test_display_anonymous() {
        let reaction_type = PaidReactionType::anonymous();
        assert_eq!(format!("{}", reaction_type), "Anonymous");
    }

    #[test]
    fn test_display_dialog() {
        let reaction_type = PaidReactionType::dialog(DialogId::new(123456));
        let display = format!("{}", reaction_type);
        assert!(display.contains("Dialog"));
        assert!(display.contains("123456"));
    }

    // === Debug Tests ===

    #[test]
    fn test_debug_regular() {
        let reaction_type = PaidReactionType::regular();
        let debug_str = format!("{:?}", reaction_type);
        assert!(debug_str.contains("Regular"));
    }

    #[test]
    fn test_debug_anonymous() {
        let reaction_type = PaidReactionType::anonymous();
        let debug_str = format!("{:?}", reaction_type);
        assert!(debug_str.contains("Anonymous"));
    }

    #[test]
    fn test_debug_dialog() {
        let reaction_type = PaidReactionType::dialog(DialogId::new(123456));
        let debug_str = format!("{:?}", reaction_type);
        assert!(debug_str.contains("Dialog"));
    }

    // === Edge Cases ===

    #[test]
    fn test_dialog_with_max_id() {
        let dialog_id = DialogId::new(i64::MAX);
        let reaction_type = PaidReactionType::dialog(dialog_id);
        assert!(reaction_type.is_valid());
        assert_eq!(reaction_type.dialog_id(), Some(dialog_id));
    }

    #[test]
    fn test_dialog_with_min_id() {
        let dialog_id = DialogId::new(i64::MIN);
        let reaction_type = PaidReactionType::dialog(dialog_id);
        assert!(reaction_type.is_valid());
        assert_eq!(reaction_type.dialog_id(), Some(dialog_id));
    }

    #[test]
    fn test_multiple_dialogs() {
        let dialog_ids = [123456, 789012, 456789];
        let reaction_types: Vec<_> = dialog_ids
            .iter()
            .map(|&id| PaidReactionType::dialog(DialogId::new(id)))
            .collect();

        assert_eq!(reaction_types[0].dialog_id(), Some(DialogId::new(123456)));
        assert_eq!(reaction_types[1].dialog_id(), Some(DialogId::new(789012)));
        assert_eq!(reaction_types[2].dialog_id(), Some(DialogId::new(456789)));
    }

    #[test]
    fn test_all_types_are_valid_except_invalid_dialog() {
        assert!(PaidReactionType::regular().is_valid());
        assert!(PaidReactionType::anonymous().is_valid());
        assert!(PaidReactionType::dialog(DialogId::new(123456)).is_valid());
        assert!(!PaidReactionType::dialog(DialogId::new(0)).is_valid());
    }

    #[test]
    fn test_conversion_from_legacy() {
        // Legacy true = anonymous
        let r1 = PaidReactionType::legacy(true);
        assert!(r1.is_anonymous());

        // Legacy false = regular
        let r2 = PaidReactionType::legacy(false);
        assert!(r2.is_regular());
    }
}
