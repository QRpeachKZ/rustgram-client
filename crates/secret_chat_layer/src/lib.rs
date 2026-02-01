// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Secret chat layer enumeration for Telegram MTProto client.
//!
//! This module implements TDLib's SecretChatLayer.
//!
//! # Example
//!
//! ```rust
//! use rustgram_secret_chat_layer::SecretChatLayer;
//!
//! let layer = SecretChatLayer::Layer143;
//! assert!(layer.is_layer143());
//! assert_eq!(layer.value(), 143);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::fmt::{self, Display, Formatter};

/// Secret chat layer.
///
/// Based on TDLib's `SecretChatLayer` enum.
///
/// Represents the MTProto layer version for secret chats.
///
/// # Example
///
/// ```rust
/// use rustgram_secret_chat_layer::SecretChatLayer;
///
/// let layer = SecretChatLayer::Layer143;
/// assert!(layer.is_layer143());
/// assert_eq!(layer.value(), 143);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum SecretChatLayer {
    /// Layer 73 - Early secret chat implementation
    Layer73 = 73,

    /// Layer 101 - Improved secret chat
    Layer101 = 101,

    /// Layer 123 - Enhanced encryption
    Layer123 = 123,

    /// Layer 143 - Current standard
    Layer143 = 143,

    /// Layer 144 - Latest version
    Layer144 = 144,

    /// Unknown layer
    #[default]
    Unknown = 0,
}

impl SecretChatLayer {
    /// Creates a new SecretChatLayer from an i32 value.
    ///
    /// # Arguments
    ///
    /// * `value` - The integer value to convert
    ///
    /// # Returns
    ///
    /// Returns `Some(SecretChatLayer)` if the value is valid, `None` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_layer::SecretChatLayer;
    ///
    /// assert_eq!(SecretChatLayer::from_i32(73), Some(SecretChatLayer::Layer73));
    /// assert_eq!(SecretChatLayer::from_i32(143), Some(SecretChatLayer::Layer143));
    /// assert_eq!(SecretChatLayer::from_i32(99), Some(SecretChatLayer::Unknown));
    /// ```
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            73 => Some(Self::Layer73),
            101 => Some(Self::Layer101),
            123 => Some(Self::Layer123),
            143 => Some(Self::Layer143),
            144 => Some(Self::Layer144),
            _ => Some(Self::Unknown),
        }
    }

    /// Returns the i32 representation of this secret chat layer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_layer::SecretChatLayer;
    ///
    /// assert_eq!(SecretChatLayer::Layer73.value(), 73);
    /// assert_eq!(SecretChatLayer::Layer143.value(), 143);
    /// ```
    pub fn value(self) -> i32 {
        self as i32
    }

    /// Returns the name of this secret chat layer.
    ///
    /// # Returns
    ///
    /// Returns a string representation of the layer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_layer::SecretChatLayer;
    ///
    /// assert_eq!(SecretChatLayer::Layer73.name(), "layer73");
    /// assert_eq!(SecretChatLayer::Layer143.name(), "layer143");
    /// ```
    pub fn name(self) -> &'static str {
        match self {
            Self::Layer73 => "layer73",
            Self::Layer101 => "layer101",
            Self::Layer123 => "layer123",
            Self::Layer143 => "layer143",
            Self::Layer144 => "layer144",
            Self::Unknown => "unknown",
        }
    }

    /// Checks if this is layer 73.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is layer 73, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_layer::SecretChatLayer;
    ///
    /// assert!(SecretChatLayer::Layer73.is_layer73());
    /// assert!(!SecretChatLayer::Layer143.is_layer73());
    /// ```
    pub fn is_layer73(self) -> bool {
        matches!(self, Self::Layer73)
    }

    /// Checks if this is layer 101.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is layer 101, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_layer::SecretChatLayer;
    ///
    /// assert!(SecretChatLayer::Layer101.is_layer101());
    /// assert!(!SecretChatLayer::Layer73.is_layer101());
    /// ```
    pub fn is_layer101(self) -> bool {
        matches!(self, Self::Layer101)
    }

    /// Checks if this is layer 123.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is layer 123, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_layer::SecretChatLayer;
    ///
    /// assert!(SecretChatLayer::Layer123.is_layer123());
    /// assert!(!SecretChatLayer::Layer73.is_layer123());
    /// ```
    pub fn is_layer123(self) -> bool {
        matches!(self, Self::Layer123)
    }

    /// Checks if this is layer 143.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is layer 143, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_layer::SecretChatLayer;
    ///
    /// assert!(SecretChatLayer::Layer143.is_layer143());
    /// assert!(!SecretChatLayer::Layer73.is_layer143());
    /// ```
    pub fn is_layer143(self) -> bool {
        matches!(self, Self::Layer143)
    }

    /// Checks if this is layer 144.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is layer 144, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_layer::SecretChatLayer;
    ///
    /// assert!(SecretChatLayer::Layer144.is_layer144());
    /// assert!(!SecretChatLayer::Layer73.is_layer144());
    /// ```
    pub fn is_layer144(self) -> bool {
        matches!(self, Self::Layer144)
    }

    /// Checks if this layer is unknown.
    ///
    /// # Returns
    ///
    /// Returns `true` if this is an unknown layer, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_layer::SecretChatLayer;
    ///
    /// assert!(SecretChatLayer::Unknown.is_unknown());
    /// assert!(!SecretChatLayer::Layer143.is_unknown());
    /// ```
    pub fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Checks if this layer is supported.
    ///
    /// # Returns
    ///
    /// Returns `true` if the layer is supported, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_layer::SecretChatLayer;
    ///
    /// assert!(SecretChatLayer::Layer143.is_supported());
    /// assert!(!SecretChatLayer::Unknown.is_supported());
    /// ```
    pub fn is_supported(self) -> bool {
        !matches!(self, Self::Unknown)
    }

    /// Checks if this is the latest layer (144).
    ///
    /// # Returns
    ///
    /// Returns `true` if this is the latest layer, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_layer::SecretChatLayer;
    ///
    /// assert!(SecretChatLayer::Layer144.is_latest());
    /// assert!(!SecretChatLayer::Layer143.is_latest());
    /// ```
    pub fn is_latest(self) -> bool {
        matches!(self, Self::Layer144)
    }

    /// Checks if this is a legacy layer (older than 143).
    ///
    /// # Returns
    ///
    /// Returns `true` if this is a legacy layer, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_layer::SecretChatLayer;
    ///
    /// assert!(SecretChatLayer::Layer73.is_legacy());
    /// assert!(!SecretChatLayer::Layer143.is_legacy());
    /// ```
    pub fn is_legacy(self) -> bool {
        matches!(self, Self::Layer73 | Self::Layer101 | Self::Layer123)
    }

    /// Checks if this layer supports the current features.
    ///
    /// Layers 143 and 144 support current features.
    ///
    /// # Returns
    ///
    /// Returns `true` if the layer supports current features, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_layer::SecretChatLayer;
    ///
    /// assert!(SecretChatLayer::Layer143.supports_current_features());
    /// assert!(SecretChatLayer::Layer144.supports_current_features());
    /// assert!(!SecretChatLayer::Layer73.supports_current_features());
    /// ```
    pub fn supports_current_features(self) -> bool {
        matches!(self, Self::Layer143 | Self::Layer144)
    }

    /// Returns the recommended layer for new secret chats.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rustgram_secret_chat_layer::SecretChatLayer;
    ///
    /// assert_eq!(SecretChatLayer::recommended(), SecretChatLayer::Layer144);
    /// ```
    pub fn recommended() -> Self {
        Self::Layer144
    }
}

impl Display for SecretChatLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_i32() {
        assert_eq!(
            SecretChatLayer::from_i32(73),
            Some(SecretChatLayer::Layer73)
        );
        assert_eq!(
            SecretChatLayer::from_i32(101),
            Some(SecretChatLayer::Layer101)
        );
        assert_eq!(
            SecretChatLayer::from_i32(123),
            Some(SecretChatLayer::Layer123)
        );
        assert_eq!(
            SecretChatLayer::from_i32(143),
            Some(SecretChatLayer::Layer143)
        );
        assert_eq!(
            SecretChatLayer::from_i32(144),
            Some(SecretChatLayer::Layer144)
        );
        assert_eq!(
            SecretChatLayer::from_i32(99),
            Some(SecretChatLayer::Unknown)
        );
        assert_eq!(SecretChatLayer::from_i32(0), Some(SecretChatLayer::Unknown));
    }

    #[test]
    fn test_value() {
        assert_eq!(SecretChatLayer::Layer73.value(), 73);
        assert_eq!(SecretChatLayer::Layer101.value(), 101);
        assert_eq!(SecretChatLayer::Layer123.value(), 123);
        assert_eq!(SecretChatLayer::Layer143.value(), 143);
        assert_eq!(SecretChatLayer::Layer144.value(), 144);
        assert_eq!(SecretChatLayer::Unknown.value(), 0);
    }

    #[test]
    fn test_name() {
        assert_eq!(SecretChatLayer::Layer73.name(), "layer73");
        assert_eq!(SecretChatLayer::Layer101.name(), "layer101");
        assert_eq!(SecretChatLayer::Layer123.name(), "layer123");
        assert_eq!(SecretChatLayer::Layer143.name(), "layer143");
        assert_eq!(SecretChatLayer::Layer144.name(), "layer144");
        assert_eq!(SecretChatLayer::Unknown.name(), "unknown");
    }

    #[test]
    fn test_is_layer73() {
        assert!(SecretChatLayer::Layer73.is_layer73());
        assert!(!SecretChatLayer::Layer101.is_layer73());
        assert!(!SecretChatLayer::Layer123.is_layer73());
        assert!(!SecretChatLayer::Layer143.is_layer73());
        assert!(!SecretChatLayer::Layer144.is_layer73());
        assert!(!SecretChatLayer::Unknown.is_layer73());
    }

    #[test]
    fn test_is_layer101() {
        assert!(!SecretChatLayer::Layer73.is_layer101());
        assert!(SecretChatLayer::Layer101.is_layer101());
        assert!(!SecretChatLayer::Layer123.is_layer101());
        assert!(!SecretChatLayer::Layer143.is_layer101());
        assert!(!SecretChatLayer::Layer144.is_layer101());
        assert!(!SecretChatLayer::Unknown.is_layer101());
    }

    #[test]
    fn test_is_layer123() {
        assert!(!SecretChatLayer::Layer73.is_layer123());
        assert!(!SecretChatLayer::Layer101.is_layer123());
        assert!(SecretChatLayer::Layer123.is_layer123());
        assert!(!SecretChatLayer::Layer143.is_layer123());
        assert!(!SecretChatLayer::Layer144.is_layer123());
        assert!(!SecretChatLayer::Unknown.is_layer123());
    }

    #[test]
    fn test_is_layer143() {
        assert!(!SecretChatLayer::Layer73.is_layer143());
        assert!(!SecretChatLayer::Layer101.is_layer143());
        assert!(!SecretChatLayer::Layer123.is_layer143());
        assert!(SecretChatLayer::Layer143.is_layer143());
        assert!(!SecretChatLayer::Layer144.is_layer143());
        assert!(!SecretChatLayer::Unknown.is_layer143());
    }

    #[test]
    fn test_is_layer144() {
        assert!(!SecretChatLayer::Layer73.is_layer144());
        assert!(!SecretChatLayer::Layer101.is_layer144());
        assert!(!SecretChatLayer::Layer123.is_layer144());
        assert!(!SecretChatLayer::Layer143.is_layer144());
        assert!(SecretChatLayer::Layer144.is_layer144());
        assert!(!SecretChatLayer::Unknown.is_layer144());
    }

    #[test]
    fn test_is_unknown() {
        assert!(!SecretChatLayer::Layer73.is_unknown());
        assert!(!SecretChatLayer::Layer101.is_unknown());
        assert!(!SecretChatLayer::Layer123.is_unknown());
        assert!(!SecretChatLayer::Layer143.is_unknown());
        assert!(!SecretChatLayer::Layer144.is_unknown());
        assert!(SecretChatLayer::Unknown.is_unknown());
    }

    #[test]
    fn test_is_supported() {
        assert!(SecretChatLayer::Layer73.is_supported());
        assert!(SecretChatLayer::Layer101.is_supported());
        assert!(SecretChatLayer::Layer123.is_supported());
        assert!(SecretChatLayer::Layer143.is_supported());
        assert!(SecretChatLayer::Layer144.is_supported());
        assert!(!SecretChatLayer::Unknown.is_supported());
    }

    #[test]
    fn test_is_latest() {
        assert!(!SecretChatLayer::Layer73.is_latest());
        assert!(!SecretChatLayer::Layer101.is_latest());
        assert!(!SecretChatLayer::Layer123.is_latest());
        assert!(!SecretChatLayer::Layer143.is_latest());
        assert!(SecretChatLayer::Layer144.is_latest());
        assert!(!SecretChatLayer::Unknown.is_latest());
    }

    #[test]
    fn test_is_legacy() {
        assert!(SecretChatLayer::Layer73.is_legacy());
        assert!(SecretChatLayer::Layer101.is_legacy());
        assert!(SecretChatLayer::Layer123.is_legacy());
        assert!(!SecretChatLayer::Layer143.is_legacy());
        assert!(!SecretChatLayer::Layer144.is_legacy());
        assert!(!SecretChatLayer::Unknown.is_legacy());
    }

    #[test]
    fn test_supports_current_features() {
        assert!(!SecretChatLayer::Layer73.supports_current_features());
        assert!(!SecretChatLayer::Layer101.supports_current_features());
        assert!(!SecretChatLayer::Layer123.supports_current_features());
        assert!(SecretChatLayer::Layer143.supports_current_features());
        assert!(SecretChatLayer::Layer144.supports_current_features());
        assert!(!SecretChatLayer::Unknown.supports_current_features());
    }

    #[test]
    fn test_recommended() {
        assert_eq!(SecretChatLayer::recommended(), SecretChatLayer::Layer144);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", SecretChatLayer::Layer73), "layer73");
        assert_eq!(format!("{}", SecretChatLayer::Layer101), "layer101");
        assert_eq!(format!("{}", SecretChatLayer::Layer123), "layer123");
        assert_eq!(format!("{}", SecretChatLayer::Layer143), "layer143");
        assert_eq!(format!("{}", SecretChatLayer::Layer144), "layer144");
        assert_eq!(format!("{}", SecretChatLayer::Unknown), "unknown");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", SecretChatLayer::Layer73), "Layer73");
        assert_eq!(format!("{:?}", SecretChatLayer::Layer101), "Layer101");
        assert_eq!(format!("{:?}", SecretChatLayer::Layer123), "Layer123");
        assert_eq!(format!("{:?}", SecretChatLayer::Layer143), "Layer143");
        assert_eq!(format!("{:?}", SecretChatLayer::Layer144), "Layer144");
        assert_eq!(format!("{:?}", SecretChatLayer::Unknown), "Unknown");
    }

    #[test]
    fn test_default() {
        assert_eq!(SecretChatLayer::default(), SecretChatLayer::Unknown);
    }

    #[test]
    fn test_equality() {
        assert_eq!(SecretChatLayer::Layer73, SecretChatLayer::Layer73);
        assert_eq!(SecretChatLayer::Layer101, SecretChatLayer::Layer101);
        assert_eq!(SecretChatLayer::Layer123, SecretChatLayer::Layer123);
        assert_eq!(SecretChatLayer::Layer143, SecretChatLayer::Layer143);
        assert_eq!(SecretChatLayer::Layer144, SecretChatLayer::Layer144);
        assert_eq!(SecretChatLayer::Unknown, SecretChatLayer::Unknown);
    }

    #[test]
    fn test_inequality() {
        assert_ne!(SecretChatLayer::Layer73, SecretChatLayer::Layer101);
        assert_ne!(SecretChatLayer::Layer101, SecretChatLayer::Layer123);
        assert_ne!(SecretChatLayer::Layer123, SecretChatLayer::Layer143);
        assert_ne!(SecretChatLayer::Layer143, SecretChatLayer::Layer144);
        assert_ne!(SecretChatLayer::Layer144, SecretChatLayer::Unknown);
    }

    #[test]
    fn test_copy() {
        let a = SecretChatLayer::Layer143;
        let b = a;
        assert_eq!(a, SecretChatLayer::Layer143);
        assert_eq!(b, SecretChatLayer::Layer143);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(SecretChatLayer::Layer73);
        set.insert(SecretChatLayer::Layer101);
        set.insert(SecretChatLayer::Layer123);
        set.insert(SecretChatLayer::Layer143);
        set.insert(SecretChatLayer::Layer144);
        set.insert(SecretChatLayer::Unknown);
        assert_eq!(set.len(), 6);
    }

    #[test]
    fn test_all_layers_distinct() {
        let layers = [
            SecretChatLayer::Layer73,
            SecretChatLayer::Layer101,
            SecretChatLayer::Layer123,
            SecretChatLayer::Layer143,
            SecretChatLayer::Layer144,
            SecretChatLayer::Unknown,
        ];

        for i in 0..layers.len() {
            for j in (i + 1)..layers.len() {
                assert_ne!(layers[i], layers[j]);
            }
        }
    }

    #[test]
    fn test_layer_values() {
        let layers = [
            (SecretChatLayer::Layer73, 73),
            (SecretChatLayer::Layer101, 101),
            (SecretChatLayer::Layer123, 123),
            (SecretChatLayer::Layer143, 143),
            (SecretChatLayer::Layer144, 144),
        ];

        for (layer, expected_value) in layers {
            assert_eq!(layer.value(), expected_value);
        }
    }

    #[test]
    fn test_layer_progression() {
        assert!(SecretChatLayer::Layer73.value() < SecretChatLayer::Layer101.value());
        assert!(SecretChatLayer::Layer101.value() < SecretChatLayer::Layer123.value());
        assert!(SecretChatLayer::Layer123.value() < SecretChatLayer::Layer143.value());
        assert!(SecretChatLayer::Layer143.value() < SecretChatLayer::Layer144.value());
    }

    #[test]
    fn test_legacy_layers() {
        let legacy_layers = [
            SecretChatLayer::Layer73,
            SecretChatLayer::Layer101,
            SecretChatLayer::Layer123,
        ];

        for layer in legacy_layers {
            assert!(layer.is_legacy());
        }

        assert!(!SecretChatLayer::Layer143.is_legacy());
        assert!(!SecretChatLayer::Layer144.is_legacy());
    }

    #[test]
    fn test_current_feature_layers() {
        let current_layers = [SecretChatLayer::Layer143, SecretChatLayer::Layer144];

        for layer in current_layers {
            assert!(layer.supports_current_features());
        }

        assert!(!SecretChatLayer::Layer73.supports_current_features());
        assert!(!SecretChatLayer::Layer101.supports_current_features());
        assert!(!SecretChatLayer::Layer123.supports_current_features());
    }

    #[test]
    fn test_from_various_i32() {
        let test_values = [
            (73, SecretChatLayer::Layer73),
            (101, SecretChatLayer::Layer101),
            (123, SecretChatLayer::Layer123),
            (143, SecretChatLayer::Layer143),
            (144, SecretChatLayer::Layer144),
            (1, SecretChatLayer::Unknown),
            (50, SecretChatLayer::Unknown),
            (200, SecretChatLayer::Unknown),
            (-1, SecretChatLayer::Unknown),
        ];

        for (value, expected) in test_values {
            assert_eq!(SecretChatLayer::from_i32(value), Some(expected));
        }
    }

    #[test]
    fn test_type_count() {
        let layers = [
            SecretChatLayer::Layer73,
            SecretChatLayer::Layer101,
            SecretChatLayer::Layer123,
            SecretChatLayer::Layer143,
            SecretChatLayer::Layer144,
            SecretChatLayer::Unknown,
        ];
        assert_eq!(layers.len(), 6);
    }
}
