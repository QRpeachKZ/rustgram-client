//! # Rustgram Message TTL
//!
//! Message time-to-live (auto-delete) management for Telegram MTProto client.
//!
//! This crate provides a simple value type that wraps the auto-delete period
//! for messages in Telegram. It includes validation, TL serialization support,
//! and conversion methods for different API representations.
//!
//! ## Overview
//!
//! - [`MessageTtl`] - Time-to-live period wrapper with validation
//!
//! ## Examples
//!
//! Basic usage:
//!
//! ```rust
//! use rustgram_message_ttl::MessageTtl;
//!
//! // Create a TTL with a positive period
//! let ttl = MessageTtl::new(60);
//! assert_eq!(ttl.period(), 60);
//! assert!(!ttl.is_empty());
//!
//! // Negative values are clamped to 0
//! let ttl_zero = MessageTtl::new(-10);
//! assert_eq!(ttl_zero.period(), 0);
//! assert!(ttl_zero.is_empty());
//!
//! // Zero TTL means no auto-delete
//! let ttl_no_delete = MessageTtl::new(0);
//! assert!(ttl_no_delete.is_empty());
//! assert_eq!(ttl_no_delete.get_message_auto_delete_time_object(), 0);
//! ```
//!
//! Display formatting:
//!
//! ```rust
//! use rustgram_message_ttl::MessageTtl;
//!
//! let ttl = MessageTtl::new(30);
//! assert_eq!(format!("{}", ttl), "MessageTtl[30]");
//! assert_eq!(format!("{:?}", ttl), "MessageTtl[30]");
//! ```
//!
//! TL serialization:
//!
//! ```rust
//! use rustgram_message_ttl::MessageTtl;
//!
//! let ttl = MessageTtl::new(120);
//!
//! // Serialize to TL format
//! let mut buf = bytes::BytesMut::new();
//! ttl.store(&mut buf).unwrap();
//!
//! // Deserialize from TL format
//! let mut parser = rustgram_types::tl::Bytes::new(buf.freeze());
//! let ttl2 = MessageTtl::parse(&mut parser).unwrap();
//! assert_eq!(ttl, ttl2);
//! ```
//!
//! ## TDLib Alignment
//!
//! This implementation aligns with TDLib's `MessageTtl` class:
//! - Source: `references/td/td/telegram/MessageTtl.h`
//! - Negative values are clamped to 0 (with error logging in TDLib)
//! - Period is stored in seconds as i32
//! - Default value is 0 (no auto-delete)
//!
//! ## Thread Safety
//!
//! `MessageTtl` is `Copy`, `Clone`, `Send`, and `Sync`, making it safe to use
//! across threads without any synchronization primitives.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::let_and_return)]

use bytes::BytesMut;
use rustgram_types::error::TypeResult;
use rustgram_types::tl::{Bytes, TlHelper};
use std::fmt;

/// Message time-to-live period in seconds.
///
/// This is a simple value type that wraps the auto-delete period for messages.
/// Negative values are automatically clamped to 0 during construction.
///
/// # Examples
///
/// ```rust
/// use rustgram_message_ttl::MessageTtl;
///
/// // Create with positive period
/// let ttl = MessageTtl::new(60);
/// assert_eq!(ttl.period(), 60);
///
/// // Negative values are clamped to 0
/// let ttl_zero = MessageTtl::new(-10);
/// assert_eq!(ttl_zero.period(), 0);
///
/// // Zero TTL means no auto-delete
/// assert!(ttl_zero.is_empty());
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct MessageTtl(pub i32);

impl MessageTtl {
    /// Creates a new `MessageTtl` with the given period in seconds.
    ///
    /// If the period is negative, it will be clamped to 0. This matches
    /// TDLib's behavior where negative values are logged as errors and
    /// then set to 0.
    ///
    /// # Arguments
    ///
    /// * `period` - Time-to-live period in seconds (clamped to >= 0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_message_ttl::MessageTtl;
    ///
    /// let ttl = MessageTtl::new(60);
    /// assert_eq!(ttl.period(), 60);
    ///
    /// let ttl_zero = MessageTtl::new(-10);
    /// assert_eq!(ttl_zero.period(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(period: i32) -> Self {
        if period < 0 {
            Self(0)
        } else {
            Self(period)
        }
    }

    /// Returns `true` if the period is 0 (no auto-delete).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_message_ttl::MessageTtl;
    ///
    /// assert!(MessageTtl::new(0).is_empty());
    /// assert!(!MessageTtl::new(60).is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns the raw period value in seconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_message_ttl::MessageTtl;
    ///
    /// let ttl = MessageTtl::new(120);
    /// assert_eq!(ttl.period(), 120);
    /// ```
    #[inline]
    #[must_use]
    pub const fn period(self) -> i32 {
        self.0
    }

    /// Returns the period for TDLib API object (max(period, 0)).
    ///
    /// This is used when converting to TDLib's `messageAutoDeleteTime` object.
    /// Since we already clamp negative values to 0 in the constructor, this
    /// simply returns the period value, but the method name matches TDLib's API.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_message_ttl::MessageTtl;
    ///
    /// let ttl = MessageTtl::new(60);
    /// assert_eq!(ttl.get_message_auto_delete_time_object(), 60);
    ///
    /// let ttl_zero = MessageTtl::new(0);
    /// assert_eq!(ttl_zero.get_message_auto_delete_time_object(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get_message_auto_delete_time_object(self) -> i32 {
        if self.0 < 0 {
            0
        } else {
            self.0
        }
    }

    /// Serializes this `MessageTtl` to TL format.
    ///
    /// Writes the period as a little-endian i32 to the buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - Buffer to write the serialized data to
    ///
    /// # Errors
    ///
    /// This function currently never returns an error, but it returns a
    /// `TypeResult` for consistency with the TL serialization API.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_message_ttl::MessageTtl;
    ///
    /// let ttl = MessageTtl::new(60);
    /// let mut buf = bytes::BytesMut::new();
    /// ttl.store(&mut buf).unwrap();
    /// assert_eq!(buf.len(), 4); // i32 = 4 bytes
    /// ```
    pub fn store(&self, buf: &mut BytesMut) -> TypeResult<()> {
        TlHelper::write_i32(buf, self.0);
        Ok(())
    }

    /// Deserializes a `MessageTtl` from TL format.
    ///
    /// Reads a little-endian i32 from the buffer and creates a `MessageTtl`.
    /// Negative values will be clamped to 0 during construction.
    ///
    /// # Arguments
    ///
    /// * `parser` - Buffer to read the serialized data from
    ///
    /// # Errors
    ///
    /// Returns an error if there are not enough bytes in the buffer to read
    /// an i32 value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_message_ttl::MessageTtl;
    /// use rustgram_types::tl::Bytes;
    ///
    /// let mut buf = bytes::BytesMut::new();
    /// rustgram_types::tl::TlHelper::write_i32(&mut buf, 60);
    ///
    /// let mut parser = Bytes::new(buf.freeze());
    /// let ttl = MessageTtl::parse(&mut parser).unwrap();
    /// assert_eq!(ttl.period(), 60);
    /// ```
    pub fn parse(parser: &mut Bytes) -> TypeResult<Self> {
        let period = TlHelper::read_i32(parser)?;
        Ok(Self::new(period))
    }
}

impl fmt::Display for MessageTtl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MessageTtl[{}]", self.0)
    }
}

impl fmt::Debug for MessageTtl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MessageTtl[{}]", self.0)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for MessageTtl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for MessageTtl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let period = i32::deserialize(deserializer)?;
        Ok(MessageTtl::new(period))
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-message_ttl";

#[cfg(test)]
mod tests {
    use super::*;

    // Test: new with positive value
    #[test]
    fn test_new_positive() {
        let ttl = MessageTtl::new(60);
        assert_eq!(ttl.period(), 60);
        assert!(!ttl.is_empty());
    }

    // Test: new with negative value (should clamp to 0)
    #[test]
    fn test_new_negative_clamped() {
        let ttl = MessageTtl::new(-10);
        assert_eq!(ttl.period(), 0);
        assert!(ttl.is_empty());
    }

    // Test: new with zero
    #[test]
    fn test_new_zero() {
        let ttl = MessageTtl::new(0);
        assert_eq!(ttl.period(), 0);
        assert!(ttl.is_empty());
    }

    // Test: is_empty returns true when period == 0
    #[test]
    fn test_is_empty_true() {
        let ttl = MessageTtl::new(0);
        assert!(ttl.is_empty());
    }

    // Test: is_empty returns false when period > 0
    #[test]
    fn test_is_empty_false() {
        let ttl = MessageTtl::new(60);
        assert!(!ttl.is_empty());
    }

    // Test: period returns the original value
    #[test]
    fn test_period() {
        let ttl = MessageTtl::new(120);
        assert_eq!(ttl.period(), 120);
    }

    // Test: get_message_auto_delete_time_object
    #[test]
    fn test_get_message_auto_delete_time_object() {
        let ttl = MessageTtl::new(60);
        assert_eq!(ttl.get_message_auto_delete_time_object(), 60);

        let ttl_zero = MessageTtl::new(0);
        assert_eq!(ttl_zero.get_message_auto_delete_time_object(), 0);
    }

    // Test: Display format
    #[test]
    fn test_display_format() {
        let ttl = MessageTtl::new(60);
        assert_eq!(format!("{}", ttl), "MessageTtl[60]");

        let ttl_zero = MessageTtl::new(0);
        assert_eq!(format!("{}", ttl_zero), "MessageTtl[0]");
    }

    // Test: Debug format
    #[test]
    fn test_debug_format() {
        let ttl = MessageTtl::new(60);
        assert_eq!(format!("{:?}", ttl), "MessageTtl[60]");
    }

    // Test: Equality - equal values
    #[test]
    fn test_equality_equal() {
        let ttl1 = MessageTtl::new(60);
        let ttl2 = MessageTtl::new(60);
        assert_eq!(ttl1, ttl2);
    }

    // Test: Equality - different values
    #[test]
    fn test_equality_different() {
        let ttl1 = MessageTtl::new(60);
        let ttl2 = MessageTtl::new(120);
        assert_ne!(ttl1, ttl2);
    }

    // Test: Hash consistency
    #[test]
    fn test_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let ttl1 = MessageTtl::new(60);
        let ttl2 = MessageTtl::new(60);

        let mut hasher1 = DefaultHasher::new();
        ttl1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        ttl2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    // Test: Copy semantics
    #[test]
    fn test_copy_semantics() {
        let ttl1 = MessageTtl::new(60);
        let ttl2 = ttl1; // Copy, not move
        assert_eq!(ttl1, ttl2);
        assert_eq!(ttl1.period(), 60);
        assert_eq!(ttl2.period(), 60);
    }

    // Test: Clone semantics
    #[test]
    fn test_clone_semantics() {
        let ttl1 = MessageTtl::new(60);
        let ttl2 = ttl1.clone();
        assert_eq!(ttl1, ttl2);
    }

    // Test: Default trait
    #[test]
    fn test_default() {
        let ttl = MessageTtl::default();
        assert_eq!(ttl.period(), 0);
        assert!(ttl.is_empty());
    }

    // Test: TL store
    #[test]
    fn test_tl_store() {
        let ttl = MessageTtl::new(60);
        let mut buf = BytesMut::new();
        ttl.store(&mut buf).unwrap();
        assert_eq!(buf.len(), 4); // i32 = 4 bytes
    }

    // Test: TL parse
    #[test]
    fn test_tl_parse() {
        let mut buf = BytesMut::new();
        TlHelper::write_i32(&mut buf, 60);

        let mut parser = Bytes::new(buf.freeze());
        let ttl = MessageTtl::parse(&mut parser).unwrap();
        assert_eq!(ttl.period(), 60);
    }

    // Test: TL parse negative value (should clamp to 0)
    #[test]
    fn test_tl_parse_negative_clamped() {
        let mut buf = BytesMut::new();
        TlHelper::write_i32(&mut buf, -10);

        let mut parser = Bytes::new(buf.freeze());
        let ttl = MessageTtl::parse(&mut parser).unwrap();
        assert_eq!(ttl.period(), 0);
    }

    // Test: TL round-trip
    #[test]
    fn test_tl_round_trip() {
        let original = MessageTtl::new(120);

        let mut buf = BytesMut::new();
        original.store(&mut buf).unwrap();

        let mut parser = Bytes::new(buf.freeze());
        let parsed = MessageTtl::parse(&mut parser).unwrap();

        assert_eq!(original, parsed);
    }

    // Test: Large period value
    #[test]
    fn test_large_period() {
        let ttl = MessageTtl::new(i32::MAX);
        assert_eq!(ttl.period(), i32::MAX);
        assert!(!ttl.is_empty());
    }

    // Test: Maximum practical TTL (1 week in seconds)
    #[test]
    fn test_max_practical_ttl() {
        let one_week_seconds = 7 * 24 * 60 * 60;
        let ttl = MessageTtl::new(one_week_seconds);
        assert_eq!(ttl.period(), one_week_seconds);
        assert!(!ttl.is_empty());
    }

    // Test: Version constants
    #[test]
    fn test_version_constants() {
        assert!(!VERSION.is_empty());
        assert_eq!(CRATE_NAME, "rustgram-message_ttl");
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    // Property: new always returns period >= 0
    proptest! {
        #[test]
        fn prop_new_non_negative(period in any::<i32>()) {
            let ttl = MessageTtl::new(period);
            assert!(ttl.period() >= 0);
        }
    }

    // Property: is_empty true iff period == 0
    proptest! {
        #[test]
        fn prop_is_empty_iff_zero(period in any::<i32>()) {
            let ttl = MessageTtl::new(period);
            assert_eq!(ttl.is_empty(), ttl.period() == 0);
        }
    }

    // Property: period matches input if non-negative
    proptest! {
        #[test]
        fn prop_period_matches_non_negative_input(period in 0i32..) {
            let ttl = MessageTtl::new(period);
            assert_eq!(ttl.period(), period);
        }
    }

    // Property: TL round-trip preserves value
    proptest! {
        #[test]
        fn prop_tl_round_trip(period in any::<i32>()) {
            let original = MessageTtl::new(period);

            let mut buf = BytesMut::new();
            original.store(&mut buf).unwrap();

            let mut parser = Bytes::new(buf.freeze());
            let parsed = MessageTtl::parse(&mut parser).unwrap();

            assert_eq!(original, parsed);
            assert_eq!(parsed.period(), original.period());
        }
    }

    // Property: get_message_auto_delete_time_object never negative
    proptest! {
        #[test]
        fn prop_get_message_auto_delete_time_object_non_negative(period in any::<i32>()) {
            let ttl = MessageTtl::new(period);
            assert!(ttl.get_message_auto_delete_time_object() >= 0);
        }
    }

    // Property: Copy and Clone produce equal values
    proptest! {
        #[test]
        fn prop_copy_clone_equality(period in any::<i32>()) {
            let ttl = MessageTtl::new(period);
            let ttl_copy = ttl;
            let ttl_clone = ttl.clone();
            assert_eq!(ttl, ttl_copy);
            assert_eq!(ttl, ttl_clone);
            assert_eq!(ttl_copy, ttl_clone);
        }
    }

    // Property: Hash consistency for equal values
    proptest! {
        #[test]
        fn prop_hash_consistency(period in any::<i32>()) {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};

            let ttl1 = MessageTtl::new(period);
            let ttl2 = MessageTtl::new(period);

            let mut hasher1 = DefaultHasher::new();
            ttl1.hash(&mut hasher1);
            let hash1 = hasher1.finish();

            let mut hasher2 = DefaultHasher::new();
            ttl2.hash(&mut hasher2);
            let hash2 = hasher2.finish();

            assert_eq!(hash1, hash2);
        }
    }

    // Property: Display format contains period
    proptest! {
        #[test]
        fn prop_display_format_contains_period(period in 0i32..) {
            let ttl = MessageTtl::new(period);
            let s = format!("{}", ttl);
            assert!(s.contains(&period.to_string()));
        }
    }
}
