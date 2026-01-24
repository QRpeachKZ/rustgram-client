//! # DialogDate
//!
//! Dialog date with ordering support.
//!
//! This module implements TDLib's DialogDate pattern for sorting dialogs
//! by date/message. The ordering is reversed (newer first) with dialog_id
//! as a tiebreaker.
//!
//! ## Overview
//!
//! - [`DialogDate`] - Date with message ordering for dialogs
//!
//! ## Usage
//!
//! ```
//! use rustgram_dialog_date::DialogDate;
//! use rustgram_dialog_id::DialogId;
//!
//! let date = DialogDate::new(123456789, DialogId::from(42));
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use rustgram_dialog_id::DialogId;
use rustgram_server_message_id::ServerMessageId;
use serde::{Deserialize, Serialize};

/// Dialog date with ordering support.
///
/// Combines a timestamp order with a dialog_id for unique identification
/// and sorting. The comparison operators use reverse ordering (higher
/// order = "less than" in comparisons) to sort newer items first.
///
/// The order is composed of:
/// - High 32 bits: message date (timestamp)
/// - Low 32 bits: message_id
///
/// # Examples
///
/// ```
/// use rustgram_dialog_date::DialogDate;
/// use rustgram_dialog_id::DialogId;
///
/// let date1 = DialogDate::new(200, DialogId::from(1));
/// let date2 = DialogDate::new(100, DialogId::from(2));
///
/// // Higher order comes first (reverse ordering)
/// assert!(date1 < date2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DialogDate {
    /// Combined order value (date + message_id)
    order: i64,

    /// Associated dialog ID
    dialog_id: DialogId,
}

// ========== Constants ==========

/// Minimum dialog date (used as sentinel).
pub const MIN_DIALOG_DATE: DialogDate = DialogDate {
    order: i64::MAX,
    dialog_id: DialogId::new(0),
};

/// Maximum dialog date (used as sentinel).
pub const MAX_DIALOG_DATE: DialogDate = DialogDate {
    order: 0,
    dialog_id: DialogId::new(0),
};

/// Default order value for dialogs without messages.
pub const DEFAULT_ORDER: i64 = -1;

// ========== Constructors ==========

impl DialogDate {
    /// Creates a new dialog date with the given order and dialog_id.
    ///
    /// # Arguments
    ///
    /// * `order` - The order value (combined date and message_id)
    /// * `dialog_id` - The associated dialog ID
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_date::DialogDate;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let date = DialogDate::new(123456789, DialogId::from(42));
    /// assert_eq!(date.order(), 123456789);
    /// assert_eq!(date.dialog_id(), DialogId::from(42));
    /// ```
    #[must_use]
    pub const fn new(order: i64, dialog_id: DialogId) -> Self {
        Self { order, dialog_id }
    }

    /// Creates a dialog date from a message ID and message date.
    ///
    /// This combines the date and message_id into a single order value
    /// suitable for sorting.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The server message ID
    /// * `message_date` - The Unix timestamp of the message
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_date::DialogDate;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_server_message_id::ServerMessageId;
    ///
    /// let msg_id = ServerMessageId::from(100);
    /// let date = DialogDate::with_message(msg_id, 1704067200);
    /// ```
    #[must_use]
    pub fn with_message(message_id: ServerMessageId, message_date: i32) -> Self {
        let order = Self::get_dialog_order(message_id, message_date);
        Self {
            order,
            dialog_id: DialogId::new(0),
        }
    }

    /// Returns the order value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_date::DialogDate;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let date = DialogDate::new(12345, DialogId::from(1));
    /// assert_eq!(date.order(), 12345);
    /// ```
    #[must_use]
    pub const fn order(&self) -> i64 {
        self.order
    }

    /// Returns the dialog ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_date::DialogDate;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let date = DialogDate::new(0, DialogId::from(42));
    /// assert_eq!(date.dialog_id(), DialogId::from(42));
    /// ```
    #[must_use]
    pub const fn dialog_id(&self) -> DialogId {
        self.dialog_id
    }

    /// Extracts the message date from the order.
    ///
    /// Returns the high 32 bits of the order as the Unix timestamp.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_date::DialogDate;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// // Order: (date << 32) | message_id
    /// let date = DialogDate::new((1704067200i64 << 32) | 100, DialogId::from(1));
    /// assert_eq!(date.date(), 1704067200);
    /// ```
    #[must_use]
    pub fn date(&self) -> i32 {
        ((self.order >> 32) & 0x7FFFFFFF) as i32
    }

    /// Extracts the message ID from the order.
    ///
    /// Returns the low 32 bits of the order as the server message ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_date::DialogDate;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_server_message_id::ServerMessageId;
    ///
    /// // Order: (date << 32) | message_id
    /// let date = DialogDate::new((100i64 << 32) | 42, DialogId::from(1));
    /// assert_eq!(date.message_id(), ServerMessageId::from(42));
    /// ```
    #[must_use]
    pub fn message_id(&self) -> ServerMessageId {
        ServerMessageId::from((self.order & 0x7FFFFFFF) as i32)
    }

    /// Calculates the dialog order from a message ID and date.
    ///
    /// This combines the date and message_id into a single order value:
    /// - High 32 bits: message_date
    /// - Low 32 bits: message_id
    ///
    /// # Arguments
    ///
    /// * `message_id` - The server message ID
    /// * `message_date` - The Unix timestamp of the message
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_date::DialogDate;
    /// use rustgram_server_message_id::ServerMessageId;
    ///
    /// let msg_id = ServerMessageId::from(100);
    /// let order = DialogDate::get_dialog_order(msg_id, 1704067200);
    /// assert_eq!(order, (1704067200i64 << 32) | 100);
    /// ```
    #[must_use]
    pub const fn get_dialog_order(message_id: ServerMessageId, message_date: i32) -> i64 {
        (message_date as i64) << 32 | (message_id.get() as i64)
    }

    /// Sets the dialog ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_date::DialogDate;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let mut date = DialogDate::new(100, DialogId::from(1));
    /// date.set_dialog_id(DialogId::from(99));
    /// assert_eq!(date.dialog_id(), DialogId::from(99));
    /// ```
    pub fn set_dialog_id(&mut self, dialog_id: DialogId) {
        self.dialog_id = dialog_id;
    }

    /// Sets the order value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_date::DialogDate;
    /// use rustgram_dialog_id::DialogId;
    ///
    /// let mut date = DialogDate::new(100, DialogId::from(1));
    /// date.set_order(200);
    /// assert_eq!(date.order(), 200);
    /// ```
    pub fn set_order(&mut self, order: i64) {
        self.order = order;
    }
}

// ========== Comparison Operators ==========

/// Partial ordering for DialogDate.
///
/// Uses reverse ordering: higher order values are "less than" lower ones.
/// This sorts newer dialogs first. Dialog_id is used as a tiebreaker.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_date::DialogDate;
/// use rustgram_dialog_id::DialogId;
///
/// let date1 = DialogDate::new(200, DialogId::from(1));
/// let date2 = DialogDate::new(100, DialogId::from(2));
///
/// // date1 has higher order, so it's "less" (comes first)
/// assert!(date1 < date2);
/// ```
impl PartialOrd for DialogDate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Total ordering for DialogDate.
///
/// Uses reverse ordering: higher order values are "less than" lower ones.
/// When orders are equal, higher dialog_id comes first.
impl Ord for DialogDate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.order.cmp(&self.order) {
            std::cmp::Ordering::Equal => {
                // When orders are equal, compare by dialog_id (higher first)
                other.dialog_id.get().cmp(&self.dialog_id.get())
            }
            other => other,
        }
    }
}

// ========== Default ==========

impl Default for DialogDate {
    fn default() -> Self {
        Self {
            order: DEFAULT_ORDER,
            dialog_id: DialogId::new(0),
        }
    }
}

// ========== Display ==========

impl std::fmt::Display for DialogDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.order, self.dialog_id.get())
    }
}

// ========== Tests ==========

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Constructor Tests ==========

    #[test]
    fn test_new_creates_dialog_date() {
        let dialog_id = DialogId::from(42);
        let date = DialogDate::new(12345, dialog_id);

        assert_eq!(date.order(), 12345);
        assert_eq!(date.dialog_id(), dialog_id);
    }

    #[test]
    fn test_with_message_creates_dialog_date() {
        let msg_id = ServerMessageId::from(100);
        let date = DialogDate::with_message(msg_id, 1704067200);

        assert_eq!(date.date(), 1704067200);
        assert_eq!(date.message_id(), msg_id);
    }

    #[test]
    fn test_default_creates_dialog_date() {
        let date = DialogDate::default();

        assert_eq!(date.order(), DEFAULT_ORDER);
        assert_eq!(date.dialog_id(), DialogId::new(0));
    }

    // ========== Getter Tests ==========

    #[test]
    fn test_order_returns_correct_value() {
        let date = DialogDate::new(99999, DialogId::from(1));
        assert_eq!(date.order(), 99999);
    }

    #[test]
    fn test_dialog_id_returns_correct_value() {
        let dialog_id = DialogId::from(123);
        let date = DialogDate::new(0, dialog_id);
        assert_eq!(date.dialog_id(), dialog_id);
    }

    #[test]
    fn test_date_extracts_high_bits() {
        let msg_date = 1704067200i32;
        let msg_id = 42i32;
        let order = ((msg_date as i64) << 32) | (msg_id as i64);

        let date = DialogDate::new(order, DialogId::from(1));
        assert_eq!(date.date(), msg_date);
    }

    #[test]
    fn test_message_id_extracts_low_bits() {
        let msg_date = 100i32;
        let msg_id = 9999i32;
        let order = ((msg_date as i64) << 32) | (msg_id as i64);

        let date = DialogDate::new(order, DialogId::from(1));
        assert_eq!(date.message_id(), ServerMessageId::from(msg_id));
    }

    // ========== get_dialog_order Tests ==========

    #[test]
    fn test_get_dialog_order_combines_values() {
        let msg_id = ServerMessageId::from(100);
        let msg_date = 1704067200i32;

        let order = DialogDate::get_dialog_order(msg_id, msg_date);
        assert_eq!(order, (msg_date as i64) << 32 | 100);
    }

    #[test]
    fn test_get_dialog_order_with_zero_values() {
        let msg_id = ServerMessageId::from(0);
        let msg_date = 0i32;

        let order = DialogDate::get_dialog_order(msg_id, msg_date);
        assert_eq!(order, 0);
    }

    #[test]
    fn test_get_dialog_order_preserves_both_values() {
        let msg_id = ServerMessageId::from(0x12345678);
        let msg_date = 0x12345678i32;

        let order = DialogDate::get_dialog_order(msg_id, msg_date);

        let date = DialogDate::new(order, DialogId::new(0));
        assert_eq!(date.date(), msg_date);
        assert_eq!(date.message_id(), msg_id);
    }

    // ========== Setter Tests ==========

    #[test]
    fn test_set_dialog_id_updates_value() {
        let mut date = DialogDate::new(100, DialogId::from(1));
        date.set_dialog_id(DialogId::from(99));
        assert_eq!(date.dialog_id(), DialogId::from(99));
    }

    #[test]
    fn test_set_order_updates_value() {
        let mut date = DialogDate::new(100, DialogId::from(1));
        date.set_order(999);
        assert_eq!(date.order(), 999);
    }

    // ========== Comparison Tests ==========

    #[test]
    fn test_ord_reverse_ordering() {
        let date1 = DialogDate::new(200, DialogId::from(1));
        let date2 = DialogDate::new(100, DialogId::from(1));

        // Higher order is "less" (comes first)
        assert!(date1 < date2);
        assert!(date2 > date1);
    }

    #[test]
    fn test_ord_with_dialog_id_tiebreaker() {
        let date1 = DialogDate::new(100, DialogId::from(50));
        let date2 = DialogDate::new(100, DialogId::from(10));

        // Same order, higher dialog_id comes first
        assert!(date1 < date2);
        assert!(date2 > date1);
    }

    #[test]
    fn test_ord_with_both_different() {
        let date1 = DialogDate::new(200, DialogId::from(1));
        let date2 = DialogDate::new(100, DialogId::from(50));

        // Order takes precedence
        assert!(date1 < date2);
    }

    #[test]
    fn test_ord_equal() {
        let date1 = DialogDate::new(100, DialogId::from(42));
        let date2 = DialogDate::new(100, DialogId::from(42));

        assert_eq!(date1, date2);
        assert!(date1 <= date2);
        assert!(date1 >= date2);
    }

    #[test]
    fn test_partial_ord_some() {
        let date1 = DialogDate::new(100, DialogId::from(1));
        let date2 = DialogDate::new(200, DialogId::from(2));

        assert!(date1.partial_cmp(&date2).is_some());
    }

    // ========== PartialEq Tests ==========

    #[test]
    fn test_eq_same_values() {
        let date1 = DialogDate::new(123, DialogId::from(456));
        let date2 = DialogDate::new(123, DialogId::from(456));
        assert_eq!(date1, date2);
    }

    #[test]
    fn test_eq_different_order() {
        let date1 = DialogDate::new(123, DialogId::from(456));
        let date2 = DialogDate::new(456, DialogId::from(456));
        assert_ne!(date1, date2);
    }

    #[test]
    fn test_eq_different_dialog_id() {
        let date1 = DialogDate::new(123, DialogId::from(1));
        let date2 = DialogDate::new(123, DialogId::from(2));
        assert_ne!(date1, date2);
    }

    // ========== Constant Tests ==========

    #[test]
    fn test_min_dialog_date_constants() {
        assert_eq!(MIN_DIALOG_DATE.order, i64::MAX);
        assert_eq!(MIN_DIALOG_DATE.dialog_id, DialogId::new(0));
    }

    #[test]
    fn test_max_dialog_date_constants() {
        assert_eq!(MAX_DIALOG_DATE.order, 0);
        assert_eq!(MAX_DIALOG_DATE.dialog_id, DialogId::new(0));
    }

    #[test]
    fn test_default_order_constant() {
        assert_eq!(DEFAULT_ORDER, -1);
    }

    #[test]
    fn test_min_is_less_than_max() {
        assert!(MIN_DIALOG_DATE < MAX_DIALOG_DATE);
    }

    #[test]
    fn test_constants_are_valid_dialog_dates() {
        // Constants should be usable as DialogDate values
        let date = MIN_DIALOG_DATE;
        assert_eq!(date.order(), i64::MAX);

        let date = MAX_DIALOG_DATE;
        assert_eq!(date.order(), 0);
    }

    // ========== Display Tests ==========

    #[test]
    fn test_display_format() {
        let date = DialogDate::new(12345, DialogId::from(67));
        let s = format!("{}", date);
        assert!(s.contains("12345"));
        assert!(s.contains("67"));
    }

    // ========== Clone Tests ==========

    #[test]
    fn test_clone_creates_copy() {
        let date1 = DialogDate::new(999, DialogId::from(42));
        let date2 = date1;

        assert_eq!(date1, date2);
        assert_eq!(date1.order(), date2.order());
        assert_eq!(date1.dialog_id(), date2.dialog_id());
    }

    // ========== Hash Tests ==========

    #[test]
    fn test_hash_same_for_same_values() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let date1 = DialogDate::new(123, DialogId::from(456));
        let date2 = DialogDate::new(123, DialogId::from(456));

        let mut hasher1 = DefaultHasher::new();
        date1.hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        date2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_hash_different_for_different_values() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let date1 = DialogDate::new(123, DialogId::from(456));
        let date2 = DialogDate::new(456, DialogId::from(123));

        let mut hasher1 = DefaultHasher::new();
        date1.hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        date2.hash(&mut hasher2);

        assert_ne!(hasher1.finish(), hasher2.finish());
    }

    // ========== Serde Tests ==========

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let date = DialogDate::new(123456789, DialogId::from(42));

        let json = serde_json::to_string(&date).unwrap();
        let deserialized: DialogDate = serde_json::from_str(&json).unwrap();

        assert_eq!(date, deserialized);
    }

    #[test]
    fn test_serialize_contains_expected_fields() {
        let date = DialogDate::new(999, DialogId::from(42));

        let json = serde_json::to_string(&date).unwrap();
        assert!(json.contains("\"order\":999") || json.contains("\"order\": 999"));
    }
}
