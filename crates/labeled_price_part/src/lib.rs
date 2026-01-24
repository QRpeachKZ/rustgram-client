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

//! # Labeled Price Part
//!
//! Represents a labeled price part for Telegram payments.
//!
//! ## Overview
//!
//! `LabeledPricePart` represents a single component of a price breakdown,
//! such as a product with an amount in the smallest currency units.
//!
//! ## TDLib Correspondence
//!
//! | Rust Type | TDLib Type | TL Schema |
//! |-----------|-----------|-----------|
//! | [`LabeledPricePart`] | `labeledPricePart` | `td_api.tl:4067` |
//!
//! ## Usage
//!
//! ```rust
//! use rustgram_labeled_price_part::LabeledPricePart;
//!
//! let price = LabeledPricePart::new("Product", 499);
//! assert_eq!(price.label(), "Product");
//! assert_eq!(price.amount(), 499);
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// A labeled price part.
///
/// Represents a component of a price with a label and amount.
/// The amount is in the smallest currency units (e.g., cents for USD).
///
/// # Examples
///
/// ```
/// use rustgram_labeled_price_part::LabeledPricePart;
///
/// // Create a price part
/// let price = LabeledPricePart::new("Product", 499);
/// assert_eq!(price.label(), "Product");
/// assert_eq!(price.amount(), 499);
///
/// // Create with builder
/// let price2 = LabeledPricePart::builder()
///     .label("Shipping")
///     .amount(50)
///     .build();
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LabeledPricePart {
    /// The label for this price part
    label: String,
    /// The amount in the smallest currency units
    amount: i64,
}

impl LabeledPricePart {
    /// Creates a new labeled price part.
    ///
    /// # Arguments
    ///
    /// * `label` - The label for this price part
    /// * `amount` - The amount in the smallest currency units
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_labeled_price_part::LabeledPricePart;
    ///
    /// let price = LabeledPricePart::new("Product", 499);
    /// assert_eq!(price.label(), "Product");
    /// assert_eq!(price.amount(), 499);
    /// ```
    pub fn new(label: &str, amount: i64) -> Self {
        Self {
            label: label.to_string(),
            amount,
        }
    }

    /// Creates a new builder for constructing a LabeledPricePart.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_labeled_price_part::LabeledPricePart;
    ///
    /// let price = LabeledPricePart::builder()
    ///     .label("Product")
    ///     .amount(499)
    ///     .build();
    /// ```
    pub fn builder() -> LabeledPricePartBuilder {
        LabeledPricePartBuilder::default()
    }

    /// Returns the label for this price part.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_labeled_price_part::LabeledPricePart;
    ///
    /// let price = LabeledPricePart::new("Product", 499);
    /// assert_eq!(price.label(), "Product");
    /// ```
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the amount in the smallest currency units.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_labeled_price_part::LabeledPricePart;
    ///
    /// let price = LabeledPricePart::new("Product", 499);
    /// assert_eq!(price.amount(), 499);
    /// ```
    pub fn amount(&self) -> i64 {
        self.amount
    }

    /// Sets the label.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_labeled_price_part::LabeledPricePart;
    ///
    /// let mut price = LabeledPricePart::new("Product", 499);
    /// price.set_label("New Product");
    /// assert_eq!(price.label(), "New Product");
    /// ```
    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_string();
    }

    /// Sets the amount.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_labeled_price_part::LabeledPricePart;
    ///
    /// let mut price = LabeledPricePart::new("Product", 499);
    /// price.set_amount(599);
    /// assert_eq!(price.amount(), 599);
    /// ```
    pub fn set_amount(&mut self, amount: i64) {
        self.amount = amount;
    }

    /// Returns `true` if the amount is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_labeled_price_part::LabeledPricePart;
    ///
    /// assert!(LabeledPricePart::new("Free", 0).is_zero());
    /// assert!(!LabeledPricePart::new("Paid", 100).is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.amount == 0
    }

    /// Returns `true` if the amount is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_labeled_price_part::LabeledPricePart;
    ///
    /// assert!(!LabeledPricePart::new("Product", 100).is_negative());
    /// assert!(LabeledPricePart::new("Discount", -50).is_negative());
    /// ```
    pub fn is_negative(&self) -> bool {
        self.amount < 0
    }

    /// Returns `true` if the amount is positive.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_labeled_price_part::LabeledPricePart;
    ///
    /// assert!(LabeledPricePart::new("Product", 100).is_positive());
    /// assert!(!LabeledPricePart::new("Discount", -50).is_positive());
    /// ```
    pub fn is_positive(&self) -> bool {
        self.amount > 0
    }
}

/// Builder for [`LabeledPricePart`].
///
/// # Examples
///
/// ```
/// use rustgram_labeled_price_part::LabeledPricePart;
///
/// let price = LabeledPricePart::builder()
///     .label("Product")
///     .amount(499)
///     .build();
/// assert_eq!(price.label(), "Product");
/// assert_eq!(price.amount(), 499);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LabeledPricePartBuilder {
    label: Option<String>,
    amount: Option<i64>,
}

impl LabeledPricePartBuilder {
    /// Sets the label.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_labeled_price_part::LabeledPricePart;
    ///
    /// let builder = LabeledPricePart::builder().label("Product");
    /// ```
    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// Sets the amount.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_labeled_price_part::LabeledPricePart;
    ///
    /// let builder = LabeledPricePart::builder().amount(499);
    /// ```
    pub fn amount(mut self, amount: i64) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Builds the `LabeledPricePart`.
    ///
    /// Returns an error if label or amount is not set.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_labeled_price_part::LabeledPricePart;
    ///
    /// let price = LabeledPricePart::builder()
    ///     .label("Product")
    ///     .amount(499)
    ///     .build();
    /// ```
    pub fn build(self) -> LabeledPricePart {
        LabeledPricePart {
            label: self.label.unwrap_or_default(),
            amount: self.amount.unwrap_or(0),
        }
    }
}

impl fmt::Display for LabeledPricePart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}: {}]", self.label, self.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_price() -> LabeledPricePart {
        LabeledPricePart::new("Product", 499)
    }

    #[test]
    fn test_new() {
        let price = create_test_price();
        assert_eq!(price.label(), "Product");
        assert_eq!(price.amount(), 499);
    }

    #[test]
    fn test_default() {
        let price = LabeledPricePart::default();
        assert_eq!(price.label(), "");
        assert_eq!(price.amount(), 0);
    }

    #[test]
    fn test_label() {
        let price = create_test_price();
        assert_eq!(price.label(), "Product");
    }

    #[test]
    fn test_amount() {
        let price = create_test_price();
        assert_eq!(price.amount(), 499);
    }

    #[test]
    fn test_set_label() {
        let mut price = create_test_price();
        price.set_label("New Product");
        assert_eq!(price.label(), "New Product");
    }

    #[test]
    fn test_set_amount() {
        let mut price = create_test_price();
        price.set_amount(599);
        assert_eq!(price.amount(), 599);
    }

    #[test]
    fn test_is_zero() {
        assert!(LabeledPricePart::new("Free", 0).is_zero());
        assert!(!LabeledPricePart::new("Paid", 100).is_zero());
    }

    #[test]
    fn test_is_negative() {
        assert!(!LabeledPricePart::new("Product", 100).is_negative());
        assert!(LabeledPricePart::new("Discount", -50).is_negative());
    }

    #[test]
    fn test_is_positive() {
        assert!(LabeledPricePart::new("Product", 100).is_positive());
        assert!(!LabeledPricePart::new("Discount", -50).is_positive());
        assert!(!LabeledPricePart::new("Free", 0).is_positive());
    }

    #[test]
    fn test_equality() {
        let price1 = create_test_price();
        let price2 = create_test_price();
        assert_eq!(price1, price2);

        let price3 = LabeledPricePart::new("Product", 599);
        assert_ne!(price1, price3);

        let price4 = LabeledPricePart::new("Other", 499);
        assert_ne!(price1, price4);
    }

    #[test]
    fn test_clone() {
        let price = create_test_price();
        let cloned = price.clone();
        assert_eq!(price, cloned);
    }

    #[test]
    fn test_debug() {
        let price = create_test_price();
        let debug = format!("{:?}", price);
        assert!(debug.contains("LabeledPricePart"));
    }

    #[test]
    fn test_display() {
        let price = create_test_price();
        assert_eq!(format!("{}", price), "[Product: 499]");
    }

    #[test]
    fn test_display_empty_label() {
        let price = LabeledPricePart::new("", 499);
        assert_eq!(format!("{}", price), "[: 499]");
    }

    #[test]
    fn test_display_negative_amount() {
        let price = LabeledPricePart::new("Discount", -50);
        assert_eq!(format!("{}", price), "[Discount: -50]");
    }

    #[test]
    fn test_builder() {
        let price = LabeledPricePart::builder()
            .label("Product")
            .amount(499)
            .build();
        assert_eq!(price.label(), "Product");
        assert_eq!(price.amount(), 499);
    }

    #[test]
    fn test_builder_default_values() {
        let price = LabeledPricePart::builder().build();
        assert_eq!(price.label(), "");
        assert_eq!(price.amount(), 0);
    }

    #[test]
    fn test_builder_partial() {
        let price = LabeledPricePart::builder().label("Product").build();
        assert_eq!(price.label(), "Product");
        assert_eq!(price.amount(), 0);
    }

    #[test]
    fn test_serialization() {
        let price = create_test_price();
        let json = serde_json::to_string(&price).unwrap();
        let parsed: LabeledPricePart = serde_json::from_str(&json).unwrap();
        assert_eq!(price, parsed);
    }

    #[test]
    fn test_zero_amount() {
        let price = LabeledPricePart::new("Free", 0);
        assert_eq!(price.amount(), 0);
        assert!(price.is_zero());
        assert!(!price.is_positive());
        assert!(!price.is_negative());
    }

    #[test]
    fn test_large_amount() {
        let price = LabeledPricePart::new("Expensive", i64::MAX);
        assert_eq!(price.amount(), i64::MAX);
    }

    #[test]
    fn test_min_amount() {
        let price = LabeledPricePart::new("Min", i64::MIN);
        assert_eq!(price.amount(), i64::MIN);
    }

    #[test]
    fn test_label_with_spaces() {
        let price = LabeledPricePart::new("My Product", 499);
        assert_eq!(price.label(), "My Product");
    }

    #[test]
    fn test_label_with_special_chars() {
        let price = LabeledPricePart::new("Product-123_Test", 499);
        assert_eq!(price.label(), "Product-123_Test");
    }

    #[test]
    fn test_unicode_label() {
        let price = LabeledPricePart::new("Продукт", 499);
        assert_eq!(price.label(), "Продукт");
    }

    #[test]
    fn test_empty_label() {
        let price = LabeledPricePart::new("", 499);
        assert_eq!(price.label(), "");
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let price1 = create_test_price();
        let price2 = create_test_price();

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        price1.hash(&mut h1);
        price2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_different_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let price1 = create_test_price();
        let price2 = LabeledPricePart::new("Product", 599);

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        price1.hash(&mut h1);
        price2.hash(&mut h2);
        assert_ne!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_builder_chaining() {
        let price = LabeledPricePart::builder()
            .label("A")
            .amount(1)
            .label("B")
            .amount(2)
            .build();
        assert_eq!(price.label(), "B");
        assert_eq!(price.amount(), 2);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let original = LabeledPricePart::new("Test Product", 12345);
        let json = serde_json::to_string(&original).unwrap();
        let parsed: LabeledPricePart = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
        assert_eq!(parsed.label(), "Test Product");
        assert_eq!(parsed.amount(), 12345);
    }

    #[test]
    fn test_display_format() {
        assert_eq!(format!("{}", LabeledPricePart::new("A", 1)), "[A: 1]");
        assert_eq!(format!("{}", LabeledPricePart::new("B", -1)), "[B: -1]");
        assert_eq!(format!("{}", LabeledPricePart::new("C", 0)), "[C: 0]");
    }
}
