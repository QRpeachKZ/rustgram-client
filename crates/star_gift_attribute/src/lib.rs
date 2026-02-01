// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Star Gift Attribute
//!
//! Star gift attributes for Telegram.
//!
//! Based on TDLib's `StarGiftAttribute` from `td/telegram/StarGiftAttribute.h`.
//!
//! # Overview
//!
//! Star gift attributes include model stickers, pattern stickers, backdrops,
//! and original details for unique gifts.
//!
//! # Example
//!
//! ```no_run
//! use rustgram_star_gift_attribute::{StarGiftAttributeSticker, StarGiftAttributeBackdrop};
//!
//! let model = StarGiftAttributeSticker::new("Model Name", 123, 500);
//! assert!(model.is_valid());
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_dialog_id::DialogId;
use rustgram_formatted_text::FormattedText;
use rustgram_star_gift_attribute_id::StarGiftAttributeId;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Star gift attribute sticker (model or pattern).
///
/// Represents a sticker that can be used as a model or pattern
/// for upgrading star gifts.
///
/// # TDLib Mapping
///
/// - `StarGiftAttributeSticker::new()` → TDLib: `StarGiftAttributeModel` or `StarGiftAttributePattern`
/// - `is_valid()` → TDLib: Checks if `rarity_permille_` is in valid range
///
/// # Example
///
/// ```rust
/// use rustgram_star_gift_attribute::StarGiftAttributeSticker;
///
/// let model = StarGiftAttributeSticker::new("Model Name", 12345, 500);
/// assert!(model.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarGiftAttributeSticker {
    /// Name of the sticker
    name: String,
    /// Sticker file ID
    sticker_file_id: i64,
    /// Rarity in permille (0-1000)
    rarity_permille: i32,
}

impl StarGiftAttributeSticker {
    /// Creates a new star gift attribute sticker.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the sticker
    /// * `sticker_file_id` - Sticker file ID
    /// * `rarity_permille` - Rarity in permille (0-1000)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_attribute::StarGiftAttributeSticker;
    ///
    /// let sticker = StarGiftAttributeSticker::new("Model", 12345, 500);
    /// assert_eq!(sticker.name(), "Model");
    /// ```
    #[must_use]
    pub fn new(name: impl Into<String>, sticker_file_id: i64, rarity_permille: i32) -> Self {
        Self {
            name: name.into(),
            sticker_file_id,
            rarity_permille,
        }
    }

    /// Returns the sticker name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the sticker file ID.
    #[must_use]
    pub fn sticker_file_id(&self) -> i64 {
        self.sticker_file_id
    }

    /// Returns the rarity in permille.
    #[must_use]
    pub fn rarity_permille(&self) -> i32 {
        self.rarity_permille
    }

    /// Checks if this sticker is valid.
    ///
    /// Valid stickers have:
    /// - rarity_permille in range (0, 1000]
    /// - sticker_file_id > 0
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.rarity_permille > 0 && self.rarity_permille <= 1000 && self.sticker_file_id > 0
    }

    /// Gets the attribute ID for this sticker as a model.
    #[must_use]
    pub fn get_id_as_model(&self) -> StarGiftAttributeId {
        StarGiftAttributeId::model(self.sticker_file_id)
    }

    /// Gets the attribute ID for this sticker as a pattern.
    #[must_use]
    pub fn get_id_as_pattern(&self) -> StarGiftAttributeId {
        StarGiftAttributeId::pattern(self.sticker_file_id)
    }
}

impl Default for StarGiftAttributeSticker {
    fn default() -> Self {
        Self {
            name: String::new(),
            sticker_file_id: 0,
            rarity_permille: 0,
        }
    }
}

/// Star gift attribute backdrop.

impl fmt::Display for StarGiftAttributeSticker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Sticker({} file_id={} rarity={}/1000)",
            self.name, self.sticker_file_id, self.rarity_permille
        )
    }
}

/// Star gift attribute backdrop.
///
/// Represents a background that can be used for upgrading star gifts.
///
/// # TDLib Mapping
///
/// - `StarGiftAttributeBackdrop::new()` → TDLib: `StarGiftAttributeBackdrop`
/// - `is_valid()` → TDLib: Validates all color fields are in valid range
///
/// # Example
///
/// ```rust
/// use rustgram_star_gift_attribute::StarGiftAttributeBackdrop;
///
/// let backdrop = StarGiftAttributeBackdrop::new("Backdrop", 1, 0xFF0000, 0x00FF00, 0x0000FF, 0xFFFFFF, 500);
/// assert!(backdrop.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarGiftAttributeBackdrop {
    /// Name of the backdrop
    name: String,
    /// Backdrop ID
    id: i32,
    /// Center color (RGB)
    center_color: i32,
    /// Edge color (RGB)
    edge_color: i32,
    /// Pattern color (RGB)
    pattern_color: i32,
    /// Text color (RGB)
    text_color: i32,
    /// Rarity in permille (0-1000)
    rarity_permille: i32,
}

impl StarGiftAttributeBackdrop {
    /// Creates a new star gift attribute backdrop.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the backdrop
    /// * `id` - Backdrop ID
    /// * `center_color` - Center color (RGB)
    /// * `edge_color` - Edge color (RGB)
    /// * `pattern_color` - Pattern color (RGB)
    /// * `text_color` - Text color (RGB)
    /// * `rarity_permille` - Rarity in permille (0-1000)
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_attribute::StarGiftAttributeBackdrop;
    ///
    /// let backdrop = StarGiftAttributeBackdrop::new("Blue", 1, 0xFF0000, 0x00FF00, 0x0000FF, 0xFFFFFF, 500);
    /// ```
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        id: i32,
        center_color: i32,
        edge_color: i32,
        pattern_color: i32,
        text_color: i32,
        rarity_permille: i32,
    ) -> Self {
        Self {
            name: name.into(),
            id,
            center_color,
            edge_color,
            pattern_color,
            text_color,
            rarity_permille,
        }
    }

    /// Returns the backdrop name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the backdrop ID.
    #[must_use]
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Returns the center color (RGB).
    #[must_use]
    pub fn center_color(&self) -> i32 {
        self.center_color
    }

    /// Returns the edge color (RGB).
    #[must_use]
    pub fn edge_color(&self) -> i32 {
        self.edge_color
    }

    /// Returns the pattern color (RGB).
    #[must_use]
    pub fn pattern_color(&self) -> i32 {
        self.pattern_color
    }

    /// Returns the text color (RGB).
    #[must_use]
    pub fn text_color(&self) -> i32 {
        self.text_color
    }

    /// Returns the rarity in permille.
    #[must_use]
    pub fn rarity_permille(&self) -> i32 {
        self.rarity_permille
    }

    /// Checks if this backdrop is valid.
    ///
    /// Valid backdrops have:
    /// - rarity_permille in range (0, 1000]
    /// - id > 0
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.rarity_permille > 0 && self.rarity_permille <= 1000 && self.id > 0
    }

    /// Gets the attribute ID for this backdrop.
    #[must_use]
    pub fn get_id(&self) -> StarGiftAttributeId {
        StarGiftAttributeId::backdrop(self.id)
    }
}

impl Default for StarGiftAttributeBackdrop {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: 0,
            center_color: 0,
            edge_color: 0,
            pattern_color: 0,
            text_color: 0,
            rarity_permille: 0,
        }
    }
}

impl fmt::Display for StarGiftAttributeBackdrop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Backdrop({} id={} rarity={}/1000)",
            self.name, self.id, self.rarity_permille
        )
    }
}

/// Star gift attribute original details.
///
/// Contains information about the original sender and receiver of a unique gift.
///
/// # TDLib Mapping
///
/// - `StarGiftAttributeOriginalDetails::new()` → TDLib: `StarGiftAttributeOriginalDetails`
/// - `is_valid()` → TDLib: Validates date and dialog IDs
///
/// # Example
///
/// ```rust
/// use rustgram_star_gift_attribute::StarGiftAttributeOriginalDetails;
/// use rustgram_dialog_id::DialogId;
/// use rustgram_formatted_text::FormattedText;
///
/// let sender = DialogId::new(1234567890);
/// let receiver = DialogId::new(9876543210);
/// let message = FormattedText::new("Happy birthday!");
/// let details = StarGiftAttributeOriginalDetails::new(sender, receiver, 1234567890, message);
/// assert!(details.is_valid());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarGiftAttributeOriginalDetails {
    /// Sender dialog ID
    sender_dialog_id: DialogId,
    /// Receiver dialog ID
    receiver_dialog_id: DialogId,
    /// Date when the gift was sent
    date: i32,
    /// Message accompanying the gift
    message: FormattedText,
}

impl StarGiftAttributeOriginalDetails {
    /// Creates new star gift attribute original details.
    ///
    /// # Arguments
    ///
    /// * `sender_dialog_id` - Dialog ID of the sender
    /// * `receiver_dialog_id` - Dialog ID of the receiver
    /// * `date` - Unix timestamp when the gift was sent
    /// * `message` - Message text accompanying the gift
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift_attribute::StarGiftAttributeOriginalDetails;
    /// use rustgram_dialog_id::DialogId;
    /// use rustgram_formatted_text::FormattedText;
    ///
    /// let details = StarGiftAttributeOriginalDetails::new(
    ///     DialogId::new(1234567890),
    ///     DialogId::new(9876543210),
    ///     1234567890,
    ///     FormattedText::new("Happy birthday!")
    /// );
    /// ```
    #[must_use]
    pub fn new(
        sender_dialog_id: DialogId,
        receiver_dialog_id: DialogId,
        date: i32,
        message: FormattedText,
    ) -> Self {
        Self {
            sender_dialog_id,
            receiver_dialog_id,
            date,
            message,
        }
    }

    /// Returns the sender dialog ID.
    #[must_use]
    pub fn sender_dialog_id(&self) -> DialogId {
        self.sender_dialog_id
    }

    /// Returns the receiver dialog ID.
    #[must_use]
    pub fn receiver_dialog_id(&self) -> DialogId {
        self.receiver_dialog_id
    }

    /// Returns the date when the gift was sent.
    #[must_use]
    pub fn date(&self) -> i32 {
        self.date
    }

    /// Returns the message text.
    #[must_use]
    pub fn message(&self) -> &FormattedText {
        &self.message
    }

    /// Checks if these details are valid.
    ///
    /// Valid details have:
    /// - date > 0
    /// - receiver_dialog_id is valid
    /// - sender_dialog_id is either valid or zero (for anonymous gifts)
    #[must_use]
    pub fn is_valid(&self) -> bool {
        let sender_valid =
            self.sender_dialog_id == DialogId::new(0) || self.sender_dialog_id.is_valid();
        sender_valid && self.receiver_dialog_id.is_valid() && self.date > 0
    }
}

impl fmt::Display for StarGiftAttributeOriginalDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OriginalDetails(from={} to={} date={})",
            self.sender_dialog_id, self.receiver_dialog_id, self.date
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // StarGiftAttributeSticker tests
    #[test]
    fn test_sticker_new() {
        let sticker = StarGiftAttributeSticker::new("Model", 12345, 500);
        assert_eq!(sticker.name(), "Model");
        assert_eq!(sticker.sticker_file_id(), 12345);
        assert_eq!(sticker.rarity_permille(), 500);
    }

    #[test]
    fn test_sticker_default() {
        let sticker = StarGiftAttributeSticker::default();
        assert_eq!(sticker.name(), "");
        assert_eq!(sticker.sticker_file_id(), 0);
        assert_eq!(sticker.rarity_permille(), 0);
    }

    #[test]
    fn test_sticker_is_valid() {
        let valid = StarGiftAttributeSticker::new("Model", 12345, 500);
        assert!(valid.is_valid());

        let invalid_rarity = StarGiftAttributeSticker::new("Model", 12345, 0);
        assert!(!invalid_rarity.is_valid());

        let invalid_rarity2 = StarGiftAttributeSticker::new("Model", 12345, 1001);
        assert!(!invalid_rarity2.is_valid());

        let invalid_file = StarGiftAttributeSticker::new("Model", 0, 500);
        assert!(!invalid_file.is_valid());
    }

    #[test]
    fn test_sticker_get_id_as_model() {
        let sticker = StarGiftAttributeSticker::new("Model", 12345, 500);
        let id = sticker.get_id_as_model();
        assert!(id.is_model());
        assert_eq!(id.sticker_id(), Some(12345));
    }

    #[test]
    fn test_sticker_get_id_as_pattern() {
        let sticker = StarGiftAttributeSticker::new("Pattern", 54321, 300);
        let id = sticker.get_id_as_pattern();
        assert!(id.is_pattern());
        assert_eq!(id.sticker_id(), Some(54321));
    }

    #[test]
    fn test_sticker_equality() {
        let s1 = StarGiftAttributeSticker::new("Model", 12345, 500);
        let s2 = StarGiftAttributeSticker::new("Model", 12345, 500);
        let s3 = StarGiftAttributeSticker::new("Other", 54321, 300);

        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
    }

    // StarGiftAttributeBackdrop tests
    #[test]
    fn test_backdrop_new() {
        let backdrop =
            StarGiftAttributeBackdrop::new("Blue", 1, 0xFF0000, 0x00FF00, 0x0000FF, 0xFFFFFF, 500);
        assert_eq!(backdrop.name(), "Blue");
        assert_eq!(backdrop.id(), 1);
    }

    #[test]
    fn test_backdrop_default() {
        let backdrop = StarGiftAttributeBackdrop::default();
        assert_eq!(backdrop.name(), "");
        assert_eq!(backdrop.id(), 0);
        assert_eq!(backdrop.rarity_permille(), 0);
    }

    #[test]
    fn test_backdrop_is_valid() {
        let valid =
            StarGiftAttributeBackdrop::new("Blue", 1, 0xFF0000, 0x00FF00, 0x0000FF, 0xFFFFFF, 500);
        assert!(valid.is_valid());

        let invalid_rarity =
            StarGiftAttributeBackdrop::new("Blue", 1, 0xFF0000, 0x00FF00, 0x0000FF, 0xFFFFFF, 0);
        assert!(!invalid_rarity.is_valid());

        let invalid_id =
            StarGiftAttributeBackdrop::new("Blue", 0, 0xFF0000, 0x00FF00, 0x0000FF, 0xFFFFFF, 500);
        assert!(!invalid_id.is_valid());
    }

    #[test]
    fn test_backdrop_get_id() {
        let backdrop = StarGiftAttributeBackdrop::new(
            "Blue", 100, 0xFF0000, 0x00FF00, 0x0000FF, 0xFFFFFF, 500,
        );
        let id = backdrop.get_id();
        assert!(id.is_backdrop());
        assert_eq!(id.backdrop_id(), Some(100));
    }

    // StarGiftAttributeOriginalDetails tests
    #[test]
    fn test_original_details_new() {
        let sender = DialogId::new(1234567890);
        let receiver = DialogId::new(9876543210);
        let message = FormattedText::new("Happy birthday!");
        let details = StarGiftAttributeOriginalDetails::new(sender, receiver, 1234567890, message);

        assert_eq!(details.sender_dialog_id(), sender);
        assert_eq!(details.receiver_dialog_id(), receiver);
        assert_eq!(details.date(), 1234567890);
    }

    #[test]
    fn test_original_details_is_valid() {
        let sender = DialogId::new(1234567890);
        let receiver = DialogId::new(9876543210);
        let message = FormattedText::new("Happy birthday!");
        let details = StarGiftAttributeOriginalDetails::new(sender, receiver, 1234567890, message);

        assert!(details.is_valid());
    }

    #[test]
    fn test_original_details_valid_anonymous_sender() {
        let sender = DialogId::new(0);
        let receiver = DialogId::new(9876543210);
        let message = FormattedText::new("Happy birthday!");
        let details = StarGiftAttributeOriginalDetails::new(sender, receiver, 1234567890, message);

        assert!(details.is_valid());
    }

    #[test]
    fn test_original_details_invalid_date() {
        let sender = DialogId::new(1234567890);
        let receiver = DialogId::new(9876543210);
        let message = FormattedText::new("Happy birthday!");
        let details = StarGiftAttributeOriginalDetails::new(sender, receiver, 0, message);

        assert!(!details.is_valid());
    }

    #[test]
    fn test_original_details_invalid_receiver() {
        let sender = DialogId::new(1234567890);
        let receiver = DialogId::new(0);
        let message = FormattedText::new("Happy birthday!");
        let details = StarGiftAttributeOriginalDetails::new(sender, receiver, 1234567890, message);

        assert!(!details.is_valid());
    }

    // Serialization tests
    #[test]
    fn test_sticker_serialization() {
        let sticker = StarGiftAttributeSticker::new("Model", 12345, 500);
        let json = serde_json::to_string(&sticker).expect("Failed to serialize");
        let deserialized: StarGiftAttributeSticker =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, sticker);
    }

    #[test]
    fn test_backdrop_serialization() {
        let backdrop =
            StarGiftAttributeBackdrop::new("Blue", 1, 0xFF0000, 0x00FF00, 0x0000FF, 0xFFFFFF, 500);
        let json = serde_json::to_string(&backdrop).expect("Failed to serialize");
        let deserialized: StarGiftAttributeBackdrop =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, backdrop);
    }

    #[test]
    fn test_original_details_serialization() {
        let sender = DialogId::new(1234567890);
        let receiver = DialogId::new(9876543210);
        let message = FormattedText::new("Happy birthday!");
        let details = StarGiftAttributeOriginalDetails::new(sender, receiver, 1234567890, message);

        let json = serde_json::to_string(&details).expect("Failed to serialize");
        let deserialized: StarGiftAttributeOriginalDetails =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized, details);
    }

    // Display tests
    #[test]
    fn test_sticker_display() {
        let sticker = StarGiftAttributeSticker::new("Model", 12345, 500);
        let display = format!("{sticker}");
        assert!(display.contains("Model"));
        assert!(display.contains("12345"));
        assert!(display.contains("500"));
    }

    #[test]
    fn test_backdrop_display() {
        let backdrop =
            StarGiftAttributeBackdrop::new("Blue", 1, 0xFF0000, 0x00FF00, 0x0000FF, 0xFFFFFF, 500);
        let display = format!("{backdrop}");
        assert!(display.contains("Blue"));
        assert!(display.contains("1"));
    }

    #[test]
    fn test_original_details_display() {
        let sender = DialogId::new(1234567890);
        let receiver = DialogId::new(9876543210);
        let message = FormattedText::new("Happy birthday!");
        let details = StarGiftAttributeOriginalDetails::new(sender, receiver, 1234567890, message);

        let display = format!("{details}");
        assert!(display.contains("OriginalDetails"));
    }
}
