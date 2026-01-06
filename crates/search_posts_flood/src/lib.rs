//! # Rustgram Search Posts Flood Control
//!
//! Flood control for public post search in Telegram MTProto client.
//!
//! This crate provides a simple data structure that tracks rate limiting
//! information for searching posts in public channels. It includes daily
//! quota tracking, star-based paid queries, and wait time information.
//!
//! ## Overview
//!
//! - [`SearchPostsFlood`] - Flood control information for post search
//!
//! ## Examples
//!
//! Basic usage:
//!
//! ```rust
//! use rustgram_search_posts_flood::SearchPostsFlood;
//!
//! // Create flood control info with remaining quota
//! let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
//! assert_eq!(flood.total_daily(), 100);
//! assert_eq!(flood.remains(), 50);
//! assert!(flood.is_free());
//!
//! // Display shows remaining queries
//! assert_eq!(
//!     format!("{}", flood),
//!     "SearchPostsFlood[50 free queries left]"
//! );
//!
//! // Create flood control info with exhausted quota
//! let flood_exhausted = SearchPostsFlood::new(100, 0, 500, 1735200000, false);
//! assert_eq!(flood_exhausted.remains(), 0);
//! assert!(!flood_exhausted.is_free());
//! assert_eq!(
//!     format!("{}", flood_exhausted),
//!     "SearchPostsFlood[quota exhausted, wait until 1735200000]"
//! );
//! ```
//!
//! ## TDLib Alignment
//!
//! This implementation aligns with TDLib's flood control mechanism for
//! public post search:
//! - Tracks daily query quota for free searches
//! - Supports paid queries using Telegram Stars
//! - Provides wait timestamp for quota reset
//! - Indicates whether current query is free or paid
//!
//! ## Thread Safety
//!
//! `SearchPostsFlood` is `Clone`, `Send`, and `Sync`, making it safe to use
//! across threads without any synchronization primitives.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::let_and_return)]

use std::fmt;

/// Flood control information for public post search.
///
/// This structure tracks rate limiting information for searching posts in
/// public channels, including daily quota, remaining free queries, star
/// count for paid queries, and wait time for quota reset.
///
/// # Fields
///
/// - `total_daily` - Total daily quota of free search queries
/// - `remains` - Number of free queries remaining today
/// - `star_count` - Number of Telegram Stars required for paid queries
/// - `wait_till` - Unix timestamp when the daily quota resets
/// - `is_free` - Whether the current query is free (true) or paid (false)
///
/// # Examples
///
/// ```rust
/// use rustgram_search_posts_flood::SearchPostsFlood;
///
/// // Create with remaining quota
/// let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
/// assert_eq!(flood.remains(), 50);
/// assert!(flood.is_free());
///
/// // Create with exhausted quota
/// let flood = SearchPostsFlood::new(100, 0, 500, 1735200000, false);
/// assert_eq!(flood.remains(), 0);
/// assert!(!flood.is_free());
/// assert_eq!(flood.star_count(), 500);
/// ```
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SearchPostsFlood {
    total_daily: i32,
    remains: i32,
    star_count: i64,
    wait_till: i32,
    is_free: bool,
}

impl SearchPostsFlood {
    /// Creates a new `SearchPostsFlood` with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `total_daily` - Total daily quota of free search queries
    /// * `remains` - Number of free queries remaining today
    /// * `star_count` - Number of Telegram Stars required for paid queries
    /// * `wait_till` - Unix timestamp when the daily quota resets
    /// * `is_free` - Whether the current query is free (true) or paid (false)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_search_posts_flood::SearchPostsFlood;
    ///
    /// let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
    /// assert_eq!(flood.total_daily(), 100);
    /// assert_eq!(flood.remains(), 50);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(
        total_daily: i32,
        remains: i32,
        star_count: i64,
        wait_till: i32,
        is_free: bool,
    ) -> Self {
        Self {
            total_daily,
            remains,
            star_count,
            wait_till,
            is_free,
        }
    }

    /// Returns the total daily quota of free search queries.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_search_posts_flood::SearchPostsFlood;
    ///
    /// let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
    /// assert_eq!(flood.total_daily(), 100);
    /// ```
    #[inline]
    #[must_use]
    pub const fn total_daily(&self) -> i32 {
        self.total_daily
    }

    /// Returns the number of free queries remaining today.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_search_posts_flood::SearchPostsFlood;
    ///
    /// let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
    /// assert_eq!(flood.remains(), 50);
    /// ```
    #[inline]
    #[must_use]
    pub const fn remains(&self) -> i32 {
        self.remains
    }

    /// Returns the number of Telegram Stars required for paid queries.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_search_posts_flood::SearchPostsFlood;
    ///
    /// let flood = SearchPostsFlood::new(100, 0, 500, 1735200000, false);
    /// assert_eq!(flood.star_count(), 500);
    /// ```
    #[inline]
    #[must_use]
    pub const fn star_count(&self) -> i64 {
        self.star_count
    }

    /// Returns the Unix timestamp when the daily quota resets.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_search_posts_flood::SearchPostsFlood;
    ///
    /// let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
    /// assert_eq!(flood.wait_till(), 1735200000);
    /// ```
    #[inline]
    #[must_use]
    pub const fn wait_till(&self) -> i32 {
        self.wait_till
    }

    /// Returns `true` if the current query is free.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_search_posts_flood::SearchPostsFlood;
    ///
    /// let flood_free = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
    /// assert!(flood_free.is_free());
    ///
    /// let flood_paid = SearchPostsFlood::new(100, 0, 500, 1735200000, false);
    /// assert!(!flood_paid.is_free());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_free(&self) -> bool {
        self.is_free
    }

    /// Returns `true` if the daily quota is exhausted (no free queries remaining).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_search_posts_flood::SearchPostsFlood;
    ///
    /// let flood_ok = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
    /// assert!(!flood_ok.is_exhausted());
    ///
    /// let flood_exhausted = SearchPostsFlood::new(100, 0, 500, 1735200000, false);
    /// assert!(flood_exhausted.is_exhausted());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_exhausted(&self) -> bool {
        self.remains == 0
    }

    /// Returns `true` if this is a paid query (requires stars).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rustgram_search_posts_flood::SearchPostsFlood;
    ///
    /// let flood_free = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
    /// assert!(!flood_free.is_paid());
    ///
    /// let flood_paid = SearchPostsFlood::new(100, 0, 500, 1735200000, false);
    /// assert!(flood_paid.is_paid());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_paid(&self) -> bool {
        !self.is_free
    }
}

impl fmt::Display for SearchPostsFlood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.remains > 0 {
            write!(f, "SearchPostsFlood[{} free queries left]", self.remains)
        } else {
            write!(
                f,
                "SearchPostsFlood[quota exhausted, wait until {}]",
                self.wait_till
            )
        }
    }
}

impl fmt::Debug for SearchPostsFlood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SearchPostsFlood")
            .field("total_daily", &self.total_daily)
            .field("remains", &self.remains)
            .field("star_count", &self.star_count)
            .field("wait_till", &self.wait_till)
            .field("is_free", &self.is_free)
            .finish()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for SearchPostsFlood {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("SearchPostsFlood", 5)?;
        state.serialize_field("total_daily", &self.total_daily)?;
        state.serialize_field("remains", &self.remains)?;
        state.serialize_field("star_count", &self.star_count)?;
        state.serialize_field("wait_till", &self.wait_till)?;
        state.serialize_field("is_free", &self.is_free)?;
        state.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SearchPostsFlood {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Visitor};

        struct SearchPostsFloodVisitor;

        impl<'de> Visitor<'de> for SearchPostsFloodVisitor {
            type Value = SearchPostsFlood;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("struct SearchPostsFlood")
            }

            fn visit_map<V>(self, mut map: V) -> Result<SearchPostsFlood, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut total_daily = None;
                let mut remains = None;
                let mut star_count = None;
                let mut wait_till = None;
                let mut is_free = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "total_daily" => {
                            if total_daily.is_some() {
                                return Err(de::Error::duplicate_field("total_daily"));
                            }
                            total_daily = Some(map.next_value()?);
                        }
                        "remains" => {
                            if remains.is_some() {
                                return Err(de::Error::duplicate_field("remains"));
                            }
                            remains = Some(map.next_value()?);
                        }
                        "star_count" => {
                            if star_count.is_some() {
                                return Err(de::Error::duplicate_field("star_count"));
                            }
                            star_count = Some(map.next_value()?);
                        }
                        "wait_till" => {
                            if wait_till.is_some() {
                                return Err(de::Error::duplicate_field("wait_till"));
                            }
                            wait_till = Some(map.next_value()?);
                        }
                        "is_free" => {
                            if is_free.is_some() {
                                return Err(de::Error::duplicate_field("is_free"));
                            }
                            is_free = Some(map.next_value()?);
                        }
                        _ => {
                            map.next_value::<de::IgnoredAny>()?;
                        }
                    }
                }

                let total_daily =
                    total_daily.ok_or_else(|| de::Error::missing_field("total_daily"))?;
                let remains = remains.ok_or_else(|| de::Error::missing_field("remains"))?;
                let star_count =
                    star_count.ok_or_else(|| de::Error::missing_field("star_count"))?;
                let wait_till = wait_till.ok_or_else(|| de::Error::missing_field("wait_till"))?;
                let is_free = is_free.ok_or_else(|| de::Error::missing_field("is_free"))?;

                Ok(SearchPostsFlood::new(
                    total_daily,
                    remains,
                    star_count,
                    wait_till,
                    is_free,
                ))
            }
        }

        deserializer.deserialize_struct(
            "SearchPostsFlood",
            &[
                "total_daily",
                "remains",
                "star_count",
                "wait_till",
                "is_free",
            ],
            SearchPostsFloodVisitor,
        )
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-search_posts_flood";

#[cfg(test)]
mod tests {
    use super::*;

    // Test: new with all fields
    #[test]
    fn test_new_with_all_fields() {
        let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        assert_eq!(flood.total_daily(), 100);
        assert_eq!(flood.remains(), 50);
        assert_eq!(flood.star_count(), 0);
        assert_eq!(flood.wait_till(), 1735200000);
        assert!(flood.is_free());
    }

    // Test: new with zero values
    #[test]
    fn test_new_with_zero_values() {
        let flood = SearchPostsFlood::new(0, 0, 0, 0, false);
        assert_eq!(flood.total_daily(), 0);
        assert_eq!(flood.remains(), 0);
        assert_eq!(flood.star_count(), 0);
        assert_eq!(flood.wait_till(), 0);
        assert!(!flood.is_free());
    }

    // Test: total_daily accessor
    #[test]
    fn test_total_daily_accessor() {
        let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        assert_eq!(flood.total_daily(), 100);

        let flood2 = SearchPostsFlood::new(200, 100, 0, 1735200000, true);
        assert_eq!(flood2.total_daily(), 200);
    }

    // Test: remains accessor
    #[test]
    fn test_remains_accessor() {
        let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        assert_eq!(flood.remains(), 50);

        let flood2 = SearchPostsFlood::new(100, 0, 500, 1735200000, false);
        assert_eq!(flood2.remains(), 0);
    }

    // Test: star_count accessor
    #[test]
    fn test_star_count_accessor() {
        let flood = SearchPostsFlood::new(100, 0, 500, 1735200000, false);
        assert_eq!(flood.star_count(), 500);

        let flood2 = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        assert_eq!(flood2.star_count(), 0);
    }

    // Test: wait_till accessor
    #[test]
    fn test_wait_till_accessor() {
        let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        assert_eq!(flood.wait_till(), 1735200000);

        let flood2 = SearchPostsFlood::new(100, 0, 500, 1740000000, false);
        assert_eq!(flood2.wait_till(), 1740000000);
    }

    // Test: is_free returns true
    #[test]
    fn test_is_free_returns_true() {
        let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        assert!(flood.is_free());
    }

    // Test: is_free returns false
    #[test]
    fn test_is_free_returns_false() {
        let flood = SearchPostsFlood::new(100, 0, 500, 1735200000, false);
        assert!(!flood.is_free());
    }

    // Test: is_exhausted returns true when remains == 0
    #[test]
    fn test_is_exhausted_true() {
        let flood = SearchPostsFlood::new(100, 0, 500, 1735200000, false);
        assert!(flood.is_exhausted());
    }

    // Test: is_exhausted returns false when remains > 0
    #[test]
    fn test_is_exhausted_false() {
        let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        assert!(!flood.is_exhausted());
    }

    // Test: is_paid returns true when not free
    #[test]
    fn test_is_paid_true() {
        let flood = SearchPostsFlood::new(100, 0, 500, 1735200000, false);
        assert!(flood.is_paid());
    }

    // Test: is_paid returns false when free
    #[test]
    fn test_is_paid_false() {
        let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        assert!(!flood.is_paid());
    }

    // Test: Display format with remaining queries
    #[test]
    fn test_display_remaining_queries() {
        let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        assert_eq!(
            format!("{}", flood),
            "SearchPostsFlood[50 free queries left]"
        );
    }

    // Test: Display format with exhausted quota
    #[test]
    fn test_display_exhausted_quota() {
        let flood = SearchPostsFlood::new(100, 0, 500, 1735200000, false);
        assert_eq!(
            format!("{}", flood),
            "SearchPostsFlood[quota exhausted, wait until 1735200000]"
        );
    }

    // Test: Display format with one remaining query
    #[test]
    fn test_display_one_remaining() {
        let flood = SearchPostsFlood::new(100, 1, 0, 1735200000, true);
        assert_eq!(
            format!("{}", flood),
            "SearchPostsFlood[1 free queries left]"
        );
    }

    // Test: Debug format
    #[test]
    fn test_debug_format() {
        let flood = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        let debug_str = format!("{:?}", flood);
        assert!(debug_str.contains("SearchPostsFlood"));
        assert!(debug_str.contains("total_daily"));
        assert!(debug_str.contains("100"));
        assert!(debug_str.contains("remains"));
        assert!(debug_str.contains("50"));
        assert!(debug_str.contains("star_count"));
        assert!(debug_str.contains("0"));
        assert!(debug_str.contains("wait_till"));
        assert!(debug_str.contains("1735200000"));
        assert!(debug_str.contains("is_free"));
        assert!(debug_str.contains("true"));
    }

    // Test: Equality - equal values
    #[test]
    fn test_equality_equal_values() {
        let flood1 = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        let flood2 = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        assert_eq!(flood1, flood2);
    }

    // Test: Equality - different total_daily
    #[test]
    fn test_equality_different_total_daily() {
        let flood1 = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        let flood2 = SearchPostsFlood::new(200, 50, 0, 1735200000, true);
        assert_ne!(flood1, flood2);
    }

    // Test: Equality - different remains
    #[test]
    fn test_equality_different_remains() {
        let flood1 = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        let flood2 = SearchPostsFlood::new(100, 25, 0, 1735200000, true);
        assert_ne!(flood1, flood2);
    }

    // Test: Equality - different star_count
    #[test]
    fn test_equality_different_star_count() {
        let flood1 = SearchPostsFlood::new(100, 0, 500, 1735200000, false);
        let flood2 = SearchPostsFlood::new(100, 0, 1000, 1735200000, false);
        assert_ne!(flood1, flood2);
    }

    // Test: Equality - different wait_till
    #[test]
    fn test_equality_different_wait_till() {
        let flood1 = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        let flood2 = SearchPostsFlood::new(100, 50, 0, 1740000000, true);
        assert_ne!(flood1, flood2);
    }

    // Test: Equality - different is_free
    #[test]
    fn test_equality_different_is_free() {
        let flood1 = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        let flood2 = SearchPostsFlood::new(100, 50, 0, 1735200000, false);
        assert_ne!(flood1, flood2);
    }

    // Test: Clone semantics
    #[test]
    fn test_clone_semantics() {
        let flood1 = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        let flood2 = flood1.clone();
        assert_eq!(flood1, flood2);
        assert_eq!(flood2.total_daily(), 100);
        assert_eq!(flood2.remains(), 50);
    }

    // Test: Hash consistency
    #[test]
    fn test_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let flood1 = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        let flood2 = SearchPostsFlood::new(100, 50, 0, 1735200000, true);

        let mut hasher1 = DefaultHasher::new();
        flood1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        flood2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    // Test: Hash differs for different values
    #[test]
    fn test_hash_different_values() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let flood1 = SearchPostsFlood::new(100, 50, 0, 1735200000, true);
        let flood2 = SearchPostsFlood::new(200, 50, 0, 1735200000, true);

        let mut hasher1 = DefaultHasher::new();
        flood1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        flood2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        // Note: While hash collisions are possible, they're extremely unlikely
        // for these structurally different values
        assert_ne!(hash1, hash2);
    }

    // Test: Version constants
    #[test]
    fn test_version_constants() {
        assert_eq!(CRATE_NAME, "rustgram-search_posts_flood");
        // VERSION is set by cargo at compile time
        let _v = VERSION;
    }

    // Test: Large values
    #[test]
    fn test_large_values() {
        let flood = SearchPostsFlood::new(i32::MAX, i32::MAX, i64::MAX, i32::MAX, true);
        assert_eq!(flood.total_daily(), i32::MAX);
        assert_eq!(flood.remains(), i32::MAX);
        assert_eq!(flood.star_count(), i64::MAX);
        assert_eq!(flood.wait_till(), i32::MAX);
    }

    // Test: Negative star_count (should be allowed for internal use)
    #[test]
    fn test_negative_star_count() {
        let flood = SearchPostsFlood::new(100, 50, -100, 1735200000, true);
        assert_eq!(flood.star_count(), -100);
    }
}

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    // Property: new stores values correctly
    proptest! {
        #[test]
        fn prop_new_stores_values(
            total_daily in any::<i32>(),
            remains in any::<i32>(),
            star_count in any::<i64>(),
            wait_till in any::<i32>(),
            is_free in any::<bool>()
        ) {
            let flood = SearchPostsFlood::new(total_daily, remains, star_count, wait_till, is_free);
            assert_eq!(flood.total_daily(), total_daily);
            assert_eq!(flood.remains(), remains);
            assert_eq!(flood.star_count(), star_count);
            assert_eq!(flood.wait_till(), wait_till);
            assert_eq!(flood.is_free(), is_free);
        }
    }

    // Property: is_exhausted iff remains == 0
    proptest! {
        #[test]
        fn prop_is_exhausted_iff_remains_zero(
            total_daily in any::<i32>(),
            remains in any::<i32>(),
            star_count in any::<i64>(),
            wait_till in any::<i32>(),
            is_free in any::<bool>()
        ) {
            let flood = SearchPostsFlood::new(total_daily, remains, star_count, wait_till, is_free);
            assert_eq!(flood.is_exhausted(), remains == 0);
        }
    }

    // Property: is_paid iff not is_free
    proptest! {
        #[test]
        fn prop_is_paid_iff_not_free(
            total_daily in any::<i32>(),
            remains in any::<i32>(),
            star_count in any::<i64>(),
            wait_till in any::<i32>(),
            is_free in any::<bool>()
        ) {
            let flood = SearchPostsFlood::new(total_daily, remains, star_count, wait_till, is_free);
            assert_eq!(flood.is_paid(), !is_free);
        }
    }

    // Property: Clone produces equal values
    proptest! {
        #[test]
        fn prop_clone_equality(
            total_daily in any::<i32>(),
            remains in any::<i32>(),
            star_count in any::<i64>(),
            wait_till in any::<i32>(),
            is_free in any::<bool>()
        ) {
            let flood1 = SearchPostsFlood::new(total_daily, remains, star_count, wait_till, is_free);
            let flood2 = flood1.clone();
            assert_eq!(flood1, flood2);
        }
    }

    // Property: Equality is reflexive
    proptest! {
        #[test]
        fn prop_equality_reflexive(
            total_daily in any::<i32>(),
            remains in any::<i32>(),
            star_count in any::<i64>(),
            wait_till in any::<i32>(),
            is_free in any::<bool>()
        ) {
            let flood = SearchPostsFlood::new(total_daily, remains, star_count, wait_till, is_free);
            assert_eq!(flood, flood);
        }
    }

    // Property: Equality is symmetric
    proptest! {
        #[test]
        fn prop_equality_symmetric(
            total_daily in any::<i32>(),
            remains in any::<i32>(),
            star_count in any::<i64>(),
            wait_till in any::<i32>(),
            is_free in any::<bool>()
        ) {
            let flood1 = SearchPostsFlood::new(total_daily, remains, star_count, wait_till, is_free);
            let flood2 = SearchPostsFlood::new(total_daily, remains, star_count, wait_till, is_free);
            assert_eq!(flood1 == flood2, flood2 == flood1);
        }
    }

    // Property: Hash consistency for equal values
    proptest! {
        #[test]
        fn prop_hash_consistency(
            total_daily in any::<i32>(),
            remains in any::<i32>(),
            star_count in any::<i64>(),
            wait_till in any::<i32>(),
            is_free in any::<bool>()
        ) {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};

            let flood1 = SearchPostsFlood::new(total_daily, remains, star_count, wait_till, is_free);
            let flood2 = SearchPostsFlood::new(total_daily, remains, star_count, wait_till, is_free);

            let mut hasher1 = DefaultHasher::new();
            flood1.hash(&mut hasher1);
            let hash1 = hasher1.finish();

            let mut hasher2 = DefaultHasher::new();
            flood2.hash(&mut hasher2);
            let hash2 = hasher2.finish();

            assert_eq!(hash1, hash2);
        }
    }

    // Property: Display format contains remains when > 0
    proptest! {
        #[test]
        fn prop_display_contains_remains_when_positive(remains in 1i32..) {
            let flood = SearchPostsFlood::new(100, remains, 0, 1735200000, true);
            let s = format!("{}", flood);
            assert!(s.contains(&remains.to_string()));
            assert!(s.contains("free queries left"));
        }
    }

    // Property: Display format contains wait_till when remains == 0
    proptest! {
        #[test]
        fn prop_display_contains_wait_till_when_exhausted(wait_till in any::<i32>()) {
            let flood = SearchPostsFlood::new(100, 0, 500, wait_till, false);
            let s = format!("{}", flood);
            assert!(s.contains(&wait_till.to_string()));
            assert!(s.contains("quota exhausted"));
        }
    }

    // Property: Debug format contains all field values
    proptest! {
        #[test]
        fn prop_debug_contains_all_fields(
            total_daily in any::<i32>(),
            remains in any::<i32>(),
            star_count in any::<i64>(),
            wait_till in any::<i32>(),
            is_free in any::<bool>()
        ) {
            let flood = SearchPostsFlood::new(total_daily, remains, star_count, wait_till, is_free);
            let s = format!("{:?}", flood);
            assert!(s.contains(&total_daily.to_string()));
            assert!(s.contains(&remains.to_string()));
            assert!(s.contains(&star_count.to_string()));
            assert!(s.contains(&wait_till.to_string()));
            assert!(s.contains(&is_free.to_string()));
        }
    }
}
