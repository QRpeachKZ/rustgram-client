// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Star Gift
//!
//! Star gift for Telegram.
//!
//! Based on TDLib's `StarGift` from `td/telegram/StarGift.h`.
//!
//! # Overview
//!
//! A `StarGift` represents a gift that can be sent using Telegram Stars.
//!
//! # Example
//!
//! ```no_run
//! use rustgram_star_gift::StarGift;
//!
//! let gift = StarGift::new(123, 100);
//! assert_eq!(gift.id(), 123);
//! assert_eq!(gift.star_count(), 100);
//! ```

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use rustgram_dialog_id::DialogId;
use rustgram_star_gift_attribute::{
    StarGiftAttributeBackdrop, StarGiftAttributeOriginalDetails, StarGiftAttributeSticker,
};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Star gift for Telegram.
///
/// Represents a gift that can be sent using Telegram Stars.
///
/// # TDLib Mapping
///
/// - `StarGift::new()` → TDLib: `StarGift()`
/// - `is_valid()` → TDLib: Checks if `id_ != 0` and `sticker_file_id_.is_valid()`
///
/// # Example
///
/// ```rust
/// use rustgram_star_gift::StarGift;
///
/// let gift = StarGift::new(123, 100);
/// assert_eq!(gift.id(), 123);
/// assert_eq!(gift.star_count(), 100);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StarGift {
    /// Gift ID
    id: i64,
    /// Dialog ID that released this gift
    released_by_dialog_id: DialogId,
    /// Whether this is a premium gift
    is_premium: bool,
    /// Sticker file ID
    sticker_file_id: i64,
    /// Star count
    star_count: i64,
    /// Default sell star count
    default_sell_star_count: i64,
    /// Upgrade star count
    upgrade_star_count: i64,
    /// Number of upgrade variants
    upgrade_variants: i32,
    /// Availability remains
    availability_remains: i32,
    /// Availability total
    availability_total: i32,
    /// First sale date
    first_sale_date: i32,
    /// Last sale date
    last_sale_date: i32,
    /// Per user remains
    per_user_remains: i32,
    /// Per user total
    per_user_total: i32,
    /// Locked until date
    locked_until_date: i32,
    /// Whether this is for birthday
    is_for_birthday: bool,
    /// Whether this is an auction gift
    is_auction: bool,
    /// Whether this is a unique gift
    is_unique: bool,
    /// Whether resale is TON only
    resale_ton_only: bool,
    /// Whether theme is available
    is_theme_available: bool,
    /// Model attribute
    model: Option<StarGiftAttributeSticker>,
    /// Pattern attribute
    pattern: Option<StarGiftAttributeSticker>,
    /// Backdrop attribute
    backdrop: Option<StarGiftAttributeBackdrop>,
    /// Original details
    original_details: Option<StarGiftAttributeOriginalDetails>,
    /// Title
    title: Option<String>,
    /// Slug
    slug: Option<String>,
}

impl StarGift {
    /// Creates a new star gift.
    ///
    /// # Arguments
    ///
    /// * `id` - Gift ID
    /// * `star_count` - Star count
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_star_gift::StarGift;
    ///
    /// let gift = StarGift::new(123, 100);
    /// ```
    #[must_use]
    pub fn new(id: i64, star_count: i64) -> Self {
        Self {
            id,
            released_by_dialog_id: DialogId::new(0),
            is_premium: false,
            sticker_file_id: 0,
            star_count,
            default_sell_star_count: 0,
            upgrade_star_count: 0,
            upgrade_variants: 0,
            availability_remains: 0,
            availability_total: 0,
            first_sale_date: 0,
            last_sale_date: 0,
            per_user_remains: 0,
            per_user_total: 0,
            locked_until_date: 0,
            is_for_birthday: false,
            is_auction: false,
            is_unique: false,
            resale_ton_only: false,
            is_theme_available: false,
            model: None,
            pattern: None,
            backdrop: None,
            original_details: None,
            title: None,
            slug: None,
        }
    }

    /// Sets the sticker file ID.
    ///
    /// # Arguments
    ///
    /// * `file_id` - Sticker file ID
    pub fn set_sticker_file_id(&mut self, file_id: i64) {
        self.sticker_file_id = file_id;
    }

    /// Sets the released by dialog ID.
    ///
    /// # Arguments
    ///
    /// * `dialog_id` - Dialog ID
    pub fn set_released_by_dialog_id(&mut self, dialog_id: DialogId) {
        self.released_by_dialog_id = dialog_id;
    }

    /// Sets the premium flag.
    ///
    /// # Arguments
    ///
    /// * `is_premium` - Whether this is a premium gift
    pub fn set_premium(&mut self, is_premium: bool) {
        self.is_premium = is_premium;
    }

    /// Sets the upgrade star count.
    ///
    /// # Arguments
    ///
    /// * `count` - Upgrade star count
    pub fn set_upgrade_star_count(&mut self, count: i64) {
        self.upgrade_star_count = count;
    }

    /// Sets the upgrade variants.
    ///
    /// # Arguments
    ///
    /// * `variants` - Number of upgrade variants
    pub fn set_upgrade_variants(&mut self, variants: i32) {
        self.upgrade_variants = variants;
    }

    /// Sets availability information.
    ///
    /// # Arguments
    ///
    /// * `remains` - Availability remains
    /// * `total` - Availability total
    pub fn set_availability(&mut self, remains: i32, total: i32) {
        self.availability_remains = remains;
        self.availability_total = total;
    }

    /// Sets the model attribute.
    ///
    /// # Arguments
    ///
    /// * `model` - Model attribute
    pub fn set_model(&mut self, model: StarGiftAttributeSticker) {
        self.model = Some(model);
    }

    /// Sets the pattern attribute.
    ///
    /// # Arguments
    ///
    /// * `pattern` - Pattern attribute
    pub fn set_pattern(&mut self, pattern: StarGiftAttributeSticker) {
        self.pattern = Some(pattern);
    }

    /// Sets the backdrop attribute.
    ///
    /// # Arguments
    ///
    /// * `backdrop` - Backdrop attribute
    pub fn set_backdrop(&mut self, backdrop: StarGiftAttributeBackdrop) {
        self.backdrop = Some(backdrop);
    }

    /// Sets the original details.
    ///
    /// # Arguments
    ///
    /// * `details` - Original details
    pub fn set_original_details(&mut self, details: StarGiftAttributeOriginalDetails) {
        self.original_details = Some(details);
    }

    /// Sets the title.
    ///
    /// # Arguments
    ///
    /// * `title` - Gift title
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = Some(title.into());
    }

    /// Sets the slug.
    ///
    /// # Arguments
    ///
    /// * `slug` - Gift slug
    pub fn set_slug(&mut self, slug: impl Into<String>) {
        self.slug = Some(slug.into());
    }

    /// Sets the unique flag.
    ///
    /// # Arguments
    ///
    /// * `is_unique` - Whether this is a unique gift
    pub fn set_unique(&mut self, is_unique: bool) {
        self.is_unique = is_unique;
    }

    /// Sets the for birthday flag.
    ///
    /// # Arguments
    ///
    /// * `is_for_birthday` - Whether this is for birthday
    pub fn set_for_birthday(&mut self, is_for_birthday: bool) {
        self.is_for_birthday = is_for_birthday;
    }

    /// Sets the auction flag.
    ///
    /// # Arguments
    ///
    /// * `is_auction` - Whether this is an auction gift
    pub fn set_auction(&mut self, is_auction: bool) {
        self.is_auction = is_auction;
    }

    /// Returns the gift ID.
    #[must_use]
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Returns the star count.
    #[must_use]
    pub fn star_count(&self) -> i64 {
        self.star_count
    }

    /// Returns the sticker file ID.
    #[must_use]
    pub fn sticker_file_id(&self) -> i64 {
        self.sticker_file_id
    }

    /// Returns the released by dialog ID.
    #[must_use]
    pub fn released_by_dialog_id(&self) -> DialogId {
        self.released_by_dialog_id
    }

    /// Returns whether this is a premium gift.
    #[must_use]
    pub fn is_premium(&self) -> bool {
        self.is_premium
    }

    /// Returns the upgrade star count.
    #[must_use]
    pub fn upgrade_star_count(&self) -> i64 {
        self.upgrade_star_count
    }

    /// Returns the number of upgrade variants.
    #[must_use]
    pub fn upgrade_variants(&self) -> i32 {
        self.upgrade_variants
    }

    /// Returns the availability remains.
    #[must_use]
    pub fn availability_remains(&self) -> i32 {
        self.availability_remains
    }

    /// Returns the availability total.
    #[must_use]
    pub fn availability_total(&self) -> i32 {
        self.availability_total
    }

    /// Returns the model attribute.
    #[must_use]
    pub fn model(&self) -> Option<&StarGiftAttributeSticker> {
        self.model.as_ref()
    }

    /// Returns the pattern attribute.
    #[must_use]
    pub fn pattern(&self) -> Option<&StarGiftAttributeSticker> {
        self.pattern.as_ref()
    }

    /// Returns the backdrop attribute.
    #[must_use]
    pub fn backdrop(&self) -> Option<&StarGiftAttributeBackdrop> {
        self.backdrop.as_ref()
    }

    /// Returns the original details.
    #[must_use]
    pub fn original_details(&self) -> Option<&StarGiftAttributeOriginalDetails> {
        self.original_details.as_ref()
    }

    /// Returns the title.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Returns the slug.
    #[must_use]
    pub fn slug(&self) -> Option<&str> {
        self.slug.as_deref()
    }

    /// Checks if this is a unique gift.
    #[must_use]
    pub fn is_unique(&self) -> bool {
        self.is_unique
    }

    /// Checks if this is for birthday.
    #[must_use]
    pub fn is_for_birthday(&self) -> bool {
        self.is_for_birthday
    }

    /// Checks if this is an auction gift.
    #[must_use]
    pub fn is_auction(&self) -> bool {
        self.is_auction
    }

    /// Checks if this gift is valid.
    ///
    /// Valid gifts have:
    /// - id != 0
    /// - For non-unique gifts: sticker_file_id > 0
    /// - For unique gifts: valid model, pattern, and backdrop
    #[must_use]
    pub fn is_valid(&self) -> bool {
        if self.id == 0 {
            return false;
        }

        if self.is_unique {
            self.model.as_ref().map_or(false, |m| m.is_valid())
                && self.pattern.as_ref().map_or(false, |p| p.is_valid())
                && self.backdrop.as_ref().map_or(false, |b| b.is_valid())
        } else {
            self.sticker_file_id > 0
        }
    }
}

impl Default for StarGift {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl fmt::Display for StarGift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StarGift(id={}, stars={}", self.id, self.star_count)?;
        if let Some(title) = &self.title {
            write!(f, " title=\"{}\"", title)?;
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_formatted_text::FormattedText;

    #[test]
    fn test_star_gift_new() {
        let gift = StarGift::new(123, 100);
        assert_eq!(gift.id(), 123);
        assert_eq!(gift.star_count(), 100);
    }

    #[test]
    fn test_star_gift_default() {
        let gift = StarGift::default();
        assert_eq!(gift.id(), 0);
        assert_eq!(gift.star_count(), 0);
    }

    #[test]
    fn test_set_sticker_file_id() {
        let mut gift = StarGift::new(123, 100);
        gift.set_sticker_file_id(456);
        assert_eq!(gift.sticker_file_id(), 456);
    }

    #[test]
    fn test_set_premium() {
        let mut gift = StarGift::new(123, 100);
        gift.set_premium(true);
        assert!(gift.is_premium());
    }

    #[test]
    fn test_set_availability() {
        let mut gift = StarGift::new(123, 100);
        gift.set_availability(50, 100);
        assert_eq!(gift.availability_remains(), 50);
        assert_eq!(gift.availability_total(), 100);
    }

    #[test]
    fn test_set_title() {
        let mut gift = StarGift::new(123, 100);
        gift.set_title("Test Gift");
        assert_eq!(gift.title(), Some("Test Gift"));
    }

    #[test]
    fn test_set_slug() {
        let mut gift = StarGift::new(123, 100);
        gift.set_slug("test-gift");
        assert_eq!(gift.slug(), Some("test-gift"));
    }

    #[test]
    fn test_set_model() {
        let mut gift = StarGift::new(123, 100);
        let model = StarGiftAttributeSticker::new("Model", 1000, 500);
        gift.set_model(model);
        assert!(gift.model().is_some());
        assert_eq!(gift.model().unwrap().name(), "Model");
    }

    #[test]
    fn test_set_pattern() {
        let mut gift = StarGift::new(123, 100);
        let pattern = StarGiftAttributeSticker::new("Pattern", 2000, 300);
        gift.set_pattern(pattern);
        assert!(gift.pattern().is_some());
    }

    #[test]
    fn test_set_backdrop() {
        let mut gift = StarGift::new(123, 100);
        let backdrop =
            StarGiftAttributeBackdrop::new("Blue", 1, 0xFF0000, 0x00FF00, 0x0000FF, 0xFFFFFF, 500);
        gift.set_backdrop(backdrop);
        assert!(gift.backdrop().is_some());
    }

    #[test]
    fn test_set_original_details() {
        let mut gift = StarGift::new(123, 100);
        let sender = DialogId::new(1234567890);
        let receiver = DialogId::new(9876543210);
        let message = FormattedText::new("Happy birthday!");
        let details = StarGiftAttributeOriginalDetails::new(sender, receiver, 1234567890, message);
        gift.set_original_details(details);
        assert!(gift.original_details().is_some());
    }

    #[test]
    fn test_is_valid_non_unique() {
        let mut gift = StarGift::new(123, 100);
        gift.set_sticker_file_id(456);
        assert!(gift.is_valid());
    }

    #[test]
    fn test_is_valid_non_unique_invalid() {
        let gift = StarGift::new(123, 100);
        assert!(!gift.is_valid()); // No sticker file ID
    }

    #[test]
    fn test_is_valid_unique() {
        let mut gift = StarGift::new(123, 100);
        gift.set_unique(true);
        gift.set_model(StarGiftAttributeSticker::new("Model", 1000, 500));
        gift.set_pattern(StarGiftAttributeSticker::new("Pattern", 2000, 300));
        gift.set_backdrop(StarGiftAttributeBackdrop::new(
            "Blue", 1, 0xFF0000, 0x00FF00, 0x0000FF, 0xFFFFFF, 500,
        ));
        assert!(gift.is_valid());
    }

    #[test]
    fn test_is_valid_unique_invalid() {
        let mut gift = StarGift::new(123, 100);
        gift.set_unique(true);
        assert!(!gift.is_valid()); // Missing model, pattern, backdrop
    }

    #[test]
    fn test_equality() {
        let gift1 = StarGift::new(123, 100);
        let gift2 = StarGift::new(123, 100);
        let gift3 = StarGift::new(456, 200);

        assert_eq!(gift1, gift2);
        assert_ne!(gift1, gift3);
    }

    #[test]
    fn test_clone() {
        let mut gift1 = StarGift::new(123, 100);
        gift1.set_title("Test");
        let gift2 = gift1.clone();
        assert_eq!(gift2.title(), Some("Test"));
    }

    #[test]
    fn test_display() {
        let gift = StarGift::new(123, 100);
        let display = format!("{gift}");
        assert!(display.contains("123"));
        assert!(display.contains("100"));
    }

    #[test]
    fn test_display_with_title() {
        let mut gift = StarGift::new(123, 100);
        gift.set_title("Test Gift");
        let display = format!("{gift}");
        assert!(display.contains("Test Gift"));
    }

    #[test]
    fn test_serialization() {
        let mut gift = StarGift::new(123, 100);
        gift.set_title("Test");

        let json = serde_json::to_string(&gift).expect("Failed to serialize");
        let deserialized: StarGift = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(deserialized.id(), 123);
        assert_eq!(deserialized.title(), Some("Test"));
    }

    #[test]
    fn test_for_birthday() {
        let mut gift = StarGift::new(123, 100);
        assert!(!gift.is_for_birthday());
        gift.set_for_birthday(true);
        assert!(gift.is_for_birthday());
    }

    #[test]
    fn test_is_auction() {
        let mut gift = StarGift::new(123, 100);
        assert!(!gift.is_auction());
        gift.set_auction(true);
        assert!(gift.is_auction());
    }

    #[test]
    fn test_upgrade_star_count() {
        let mut gift = StarGift::new(123, 100);
        gift.set_upgrade_star_count(50);
        assert_eq!(gift.upgrade_star_count(), 50);
    }

    #[test]
    fn test_upgrade_variants() {
        let mut gift = StarGift::new(123, 100);
        gift.set_upgrade_variants(3);
        assert_eq!(gift.upgrade_variants(), 3);
    }
}
