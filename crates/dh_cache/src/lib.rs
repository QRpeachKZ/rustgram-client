// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Diffie-Hellman prime cache for Telegram MTProto client.
//!
//! This module implements TDLib's DhCache from `td/telegram/DhCache.h` and
//! `td/telegram/DhCache.cpp`.
//!
//! # Overview
//!
//! The DhCache is used to cache the results of prime number checks for the
//! Diffie-Hellman key exchange. This improves performance by avoiding repeated
//! expensive primality tests on the same primes.

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::collections::HashSet;
use std::sync::RwLock;

/// Result of checking if a prime is good (valid for DH).
///
/// # Example
///
/// ```
/// use rustgram_dh_cache::PrimeCheckResult;
///
/// let good = PrimeCheckResult::Good;
/// let bad = PrimeCheckResult::Bad;
/// let unknown = PrimeCheckResult::Unknown;
///
/// assert!(good.is_good());
/// assert!(bad.is_bad());
/// assert!(unknown.is_unknown());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimeCheckResult {
    /// The prime is known to be good (valid for DH).
    Good,
    /// The prime is known to be bad (invalid for DH).
    Bad,
    /// The prime has not been checked yet.
    Unknown,
}

impl PrimeCheckResult {
    /// Returns true if the prime is good.
    #[inline]
    pub const fn is_good(self) -> bool {
        matches!(self, Self::Good)
    }

    /// Returns true if the prime is bad.
    #[inline]
    pub const fn is_bad(self) -> bool {
        matches!(self, Self::Bad)
    }

    /// Returns true if the prime is unknown (not yet checked).
    #[inline]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Cache for Diffie-Hellman prime validation results.
///
/// This singleton cache stores which primes have been validated as good or bad
/// for use in DH key exchange, improving performance by avoiding repeated checks.
///
/// # Example
///
/// ```
/// use rustgram_dh_cache::DhCache;
///
/// let cache = DhCache::instance();
///
/// // Check a prime (returns Unknown if not in cache)
/// let result = cache.is_good_prime("c71caeb9...");
/// assert!(result.is_unknown());
///
/// // Add it to the good primes
/// cache.add_good_prime("c71caeb9...");
///
/// // Now it returns Good
/// let result = cache.is_good_prime("c71caeb9...");
/// assert!(result.is_good());
///
/// // The built-in prime always returns Good
/// let result = cache.is_good_prime(DhCache::built_in_prime());
/// assert!(result.is_good());
/// ```
#[derive(Debug)]
pub struct DhCache {
    /// Set of primes that have been validated as good.
    good_primes: RwLock<HashSet<String>>,
    /// Set of primes that have been validated as bad.
    bad_primes: RwLock<HashSet<String>>,
    /// The built-in good prime from TDLib.
    built_in_prime: &'static str,
}

impl DhCache {
    /// The built-in good prime from TDLib (2048-bit).
    ///
    /// This is the prime used by Telegram for DH key exchange.
    pub const BUILT_IN_PRIME: &str = "c71caeb9c6b1c9048e6c522f70f13f73980d40238e3e21c14934d037563d930f48198a0aa7c14058229493d22530f4dbfa336f6e0ac925139543aed44cce7c3720fd51f69458705ac68cd4fe6b6b13abdc9746512969328454f18faf8c595f642477fe96bb2a941d5bcd1d4ac8cc49880708fa9b378e3c4f3a9060bee67cf9a4a4a695811051907e162753b56b0f6b410dba74d8a84b2a14b3144e0ef1284754fd17ed950d5965b4b9dd46582db1178d169c6bc465b0d6ff9ca3928fef5b9ae4e418fc15e83ebea0f87fa9ff5eed70050ded2849f47bf959d956850ce929851f0d8115f635b105ee2e4e15d04b2454bf6f4fadf034b10403119cd8e3b92fcc5b";

    /// Creates a new DhCache instance.
    ///
    /// This is private because DhCache is a singleton. Use `instance()` instead.
    fn new() -> Self {
        Self {
            good_primes: RwLock::new(HashSet::new()),
            bad_primes: RwLock::new(HashSet::new()),
            built_in_prime: Self::BUILT_IN_PRIME,
        }
    }

    /// Returns the singleton instance of the DhCache.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dh_cache::DhCache;
    ///
    /// let cache1 = DhCache::instance();
    /// let cache2 = DhCache::instance();
    ///
    /// // Both references point to the same instance
    /// assert!(std::ptr::eq(cache1, cache2));
    /// ```
    pub fn instance() -> &'static Self {
        use std::sync::OnceLock;
        static INSTANCE: OnceLock<DhCache> = OnceLock::new();
        INSTANCE.get_or_init(DhCache::new)
    }

    /// Returns the built-in good prime constant.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dh_cache::DhCache;
    ///
    /// let prime = DhCache::built_in_prime();
    /// assert!(!prime.is_empty());
    /// assert!(prime.len() > 100); // It's a long hex string
    /// ```
    #[inline]
    pub const fn built_in_prime() -> &'static str {
        Self::BUILT_IN_PRIME
    }

    /// Checks if a prime is good (valid for DH).
    ///
    /// Returns:
    /// - `PrimeCheckResult::Good` if the prime is the built-in prime or in the good primes cache
    /// - `PrimeCheckResult::Bad` if the prime is in the bad primes cache
    /// - `PrimeCheckResult::Unknown` if the prime hasn't been checked
    ///
    /// # Arguments
    ///
    /// * `prime_str` - The prime to check (as a hex string)
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dh_cache::{DhCache, PrimeCheckResult};
    ///
    /// let cache = DhCache::instance();
    ///
    /// // Built-in prime is always good
    /// let result = cache.is_good_prime(DhCache::built_in_prime());
    /// assert_eq!(result, PrimeCheckResult::Good);
    ///
    /// // Unknown prime
    /// let result = cache.is_good_prime("unknownprime");
    /// assert_eq!(result, PrimeCheckResult::Unknown);
    /// ```
    pub fn is_good_prime(&self, prime_str: &str) -> PrimeCheckResult {
        // Check if it's the built-in prime first
        if prime_str == self.built_in_prime {
            return PrimeCheckResult::Good;
        }

        // Check bad primes cache
        if let Ok(bad) = self.bad_primes.read() {
            if bad.contains(prime_str) {
                return PrimeCheckResult::Bad;
            }
        }

        // Check good primes cache
        if let Ok(good) = self.good_primes.read() {
            if good.contains(prime_str) {
                return PrimeCheckResult::Good;
            }
        }

        PrimeCheckResult::Unknown
    }

    /// Adds a prime to the good primes cache.
    ///
    /// # Arguments
    ///
    /// * `prime_str` - The prime to add (as a hex string)
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dh_cache::{DhCache, PrimeCheckResult};
    ///
    /// let cache = DhCache::instance();
    ///
    /// let prime = "mycustomprime123";
    /// assert_eq!(cache.is_good_prime(prime), PrimeCheckResult::Unknown);
    ///
    /// cache.add_good_prime(prime);
    /// assert_eq!(cache.is_good_prime(prime), PrimeCheckResult::Good);
    /// ```
    pub fn add_good_prime(&self, prime_str: &str) {
        if let Ok(mut good) = self.good_primes.write() {
            good.insert(prime_str.to_string());
        }

        // Remove from bad primes if present
        if let Ok(mut bad) = self.bad_primes.write() {
            bad.remove(prime_str);
        }
    }

    /// Adds a prime to the bad primes cache.
    ///
    /// # Arguments
    ///
    /// * `prime_str` - The prime to add (as a hex string)
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dh_cache::{DhCache, PrimeCheckResult};
    ///
    /// let cache = DhCache::instance();
    ///
    /// let prime = "badprime456";
    /// assert_eq!(cache.is_good_prime(prime), PrimeCheckResult::Unknown);
    ///
    /// cache.add_bad_prime(prime);
    /// assert_eq!(cache.is_good_prime(prime), PrimeCheckResult::Bad);
    /// ```
    pub fn add_bad_prime(&self, prime_str: &str) {
        if let Ok(mut bad) = self.bad_primes.write() {
            bad.insert(prime_str.to_string());
        }

        // Remove from good primes if present
        if let Ok(mut good) = self.good_primes.write() {
            good.remove(prime_str);
        }
    }

    /// Clears all cached primes.
    ///
    /// This removes both good and bad primes from the cache, but the built-in
    /// prime will still be recognized as good.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dh_cache::{DhCache, PrimeCheckResult};
    ///
    /// let cache = DhCache::instance();
    ///
    /// cache.add_good_prime("prime1");
    /// cache.add_bad_prime("prime2");
    ///
    /// assert_eq!(cache.is_good_prime("prime1"), PrimeCheckResult::Good);
    ///
    /// cache.clear();
    ///
    /// assert_eq!(cache.is_good_prime("prime1"), PrimeCheckResult::Unknown);
    /// assert_eq!(cache.is_good_prime("prime2"), PrimeCheckResult::Unknown);
    ///
    /// // Built-in prime is still good
    /// assert_eq!(cache.is_good_prime(DhCache::built_in_prime()), PrimeCheckResult::Good);
    /// ```
    pub fn clear(&self) {
        if let Ok(mut good) = self.good_primes.write() {
            good.clear();
        }
        if let Ok(mut bad) = self.bad_primes.write() {
            bad.clear();
        }
    }

    /// Returns the number of good primes in the cache.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dh_cache::DhCache;
    ///
    /// let cache = DhCache::instance();
    ///
    /// cache.add_good_prime("prime1");
    /// cache.add_good_prime("prime2");
    ///
    /// assert_eq!(cache.good_prime_count(), 2);
    /// ```
    pub fn good_prime_count(&self) -> usize {
        self.good_primes.read().map(|g| g.len()).unwrap_or(0)
    }

    /// Returns the number of bad primes in the cache.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dh_cache::DhCache;
    ///
    /// let cache = DhCache::instance();
    ///
    /// cache.add_bad_prime("prime1");
    /// cache.add_bad_prime("prime2");
    /// cache.add_bad_prime("prime3");
    ///
    /// assert_eq!(cache.bad_prime_count(), 3);
    /// ```
    pub fn bad_prime_count(&self) -> usize {
        self.bad_primes.read().map(|b| b.len()).unwrap_or(0)
    }

    /// Checks if a prime is in the good primes cache (excluding built-in).
    ///
    /// # Arguments
    ///
    /// * `prime_str` - The prime to check
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dh_cache::DhCache;
    ///
    /// let cache = DhCache::instance();
    ///
    /// assert!(!cache.contains_good_prime("test"));
    ///
    /// cache.add_good_prime("test");
    /// assert!(cache.contains_good_prime("test"));
    /// ```
    pub fn contains_good_prime(&self, prime_str: &str) -> bool {
        self.good_primes
            .read()
            .map(|g| g.contains(prime_str))
            .unwrap_or(false)
    }

    /// Checks if a prime is in the bad primes cache.
    ///
    /// # Arguments
    ///
    /// * `prime_str` - The prime to check
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_dh_cache::DhCache;
    ///
    /// let cache = DhCache::instance();
    ///
    /// assert!(!cache.contains_bad_prime("test"));
    ///
    /// cache.add_bad_prime("test");
    /// assert!(cache.contains_bad_prime("test"));
    /// ```
    pub fn contains_bad_prime(&self, prime_str: &str) -> bool {
        self.bad_primes
            .read()
            .map(|b| b.contains(prime_str))
            .unwrap_or(false)
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-dh-cache";

#[cfg(test)]
mod tests {
    use super::*;

    // Test synchronization lock to prevent race conditions with singleton
    static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn test_version() {
        let _lock = TEST_LOCK.lock().unwrap();
        assert_eq!(CRATE_NAME, "rustgram-dh-cache");
    }

    #[test]
    fn test_prime_check_result_is_good() {
        let _lock = TEST_LOCK.lock().unwrap();
        assert!(PrimeCheckResult::Good.is_good());
        assert!(!PrimeCheckResult::Bad.is_good());
        assert!(!PrimeCheckResult::Unknown.is_good());
    }

    #[test]
    fn test_prime_check_result_is_bad() {
        let _lock = TEST_LOCK.lock().unwrap();
        assert!(!PrimeCheckResult::Good.is_bad());
        assert!(PrimeCheckResult::Bad.is_bad());
        assert!(!PrimeCheckResult::Unknown.is_bad());
    }

    #[test]
    fn test_prime_check_result_is_unknown() {
        let _lock = TEST_LOCK.lock().unwrap();
        assert!(!PrimeCheckResult::Good.is_unknown());
        assert!(!PrimeCheckResult::Bad.is_unknown());
        assert!(PrimeCheckResult::Unknown.is_unknown());
    }

    #[test]
    fn test_dh_cache_singleton() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache1 = DhCache::instance();
        let cache2 = DhCache::instance();
        assert!(std::ptr::eq(cache1, cache2));
    }

    #[test]
    fn test_is_good_prime_builtin() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache = DhCache::instance();
        assert_eq!(
            cache.is_good_prime(DhCache::built_in_prime()),
            PrimeCheckResult::Good
        );
    }

    #[test]
    fn test_is_good_prime_unknown() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache = DhCache::instance();
        cache.clear();
        assert_eq!(
            cache.is_good_prime("unknown_prime_xyz"),
            PrimeCheckResult::Unknown
        );
    }

    #[test]
    fn test_add_good_prime() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache = DhCache::instance();
        cache.clear();

        let prime = "test_add_good_abc";
        assert_eq!(cache.is_good_prime(prime), PrimeCheckResult::Unknown);

        cache.add_good_prime(prime);
        assert_eq!(cache.is_good_prime(prime), PrimeCheckResult::Good);
        assert!(cache.contains_good_prime(prime));

        cache.clear();
    }

    #[test]
    fn test_add_bad_prime() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache = DhCache::instance();
        cache.clear();

        let prime = "test_add_bad_xyz";
        assert_eq!(cache.is_good_prime(prime), PrimeCheckResult::Unknown);

        cache.add_bad_prime(prime);
        assert_eq!(cache.is_good_prime(prime), PrimeCheckResult::Bad);
        assert!(cache.contains_bad_prime(prime));

        cache.clear();
    }

    #[test]
    fn test_demote_good_to_bad() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache = DhCache::instance();
        cache.clear();

        let prime = "test_demote_abc";
        cache.add_good_prime(prime);
        assert_eq!(cache.is_good_prime(prime), PrimeCheckResult::Good);

        cache.add_bad_prime(prime);
        assert_eq!(cache.is_good_prime(prime), PrimeCheckResult::Bad);

        cache.clear();
    }

    #[test]
    fn test_promote_bad_to_good() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache = DhCache::instance();
        cache.clear();

        let prime = "test_promote_xyz";
        cache.add_bad_prime(prime);
        assert_eq!(cache.is_good_prime(prime), PrimeCheckResult::Bad);

        cache.add_good_prime(prime);
        assert_eq!(cache.is_good_prime(prime), PrimeCheckResult::Good);

        cache.clear();
    }

    #[test]
    fn test_builtin_prime_not_in_cache() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache = DhCache::instance();
        cache.clear();
        assert!(!cache.contains_good_prime(DhCache::built_in_prime()));
        assert_eq!(
            cache.is_good_prime(DhCache::built_in_prime()),
            PrimeCheckResult::Good
        );
    }

    #[test]
    fn test_good_prime_count() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache = DhCache::instance();
        cache.clear();

        assert_eq!(cache.good_prime_count(), 0);

        cache.add_good_prime("prime1");
        cache.add_good_prime("prime2");
        assert_eq!(cache.good_prime_count(), 2);

        cache.clear();
    }

    #[test]
    fn test_bad_prime_count() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache = DhCache::instance();
        cache.clear();

        assert_eq!(cache.bad_prime_count(), 0);

        cache.add_bad_prime("bad1");
        cache.add_bad_prime("bad2");
        cache.add_bad_prime("bad3");
        assert_eq!(cache.bad_prime_count(), 3);

        cache.clear();
    }

    #[test]
    fn test_multiple_primes() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache = DhCache::instance();
        cache.clear();

        cache.add_good_prime("good1");
        cache.add_good_prime("good2");
        cache.add_bad_prime("bad1");

        assert_eq!(cache.is_good_prime("good1"), PrimeCheckResult::Good);
        assert_eq!(cache.is_good_prime("good2"), PrimeCheckResult::Good);
        assert_eq!(cache.is_good_prime("bad1"), PrimeCheckResult::Bad);

        cache.clear();
    }

    #[test]
    fn test_empty_prime_string() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache = DhCache::instance();
        cache.clear();
        assert_eq!(cache.is_good_prime(""), PrimeCheckResult::Unknown);
    }

    #[test]
    fn test_contains_good_prime() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache = DhCache::instance();
        cache.clear();

        assert!(!cache.contains_good_prime("test_contains_good"));
        cache.add_good_prime("test_contains_good");
        assert!(cache.contains_good_prime("test_contains_good"));

        cache.clear();
    }

    #[test]
    fn test_contains_bad_prime() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache = DhCache::instance();
        cache.clear();

        assert!(!cache.contains_bad_prime("test_contains_bad"));
        cache.add_bad_prime("test_contains_bad");
        assert!(cache.contains_bad_prime("test_contains_bad"));

        cache.clear();
    }

    #[test]
    fn test_clear() {
        let _lock = TEST_LOCK.lock().unwrap();
        let cache = DhCache::instance();

        cache.add_good_prime("clear_test_good");
        cache.add_bad_prime("clear_test_bad");

        assert!(cache.contains_good_prime("clear_test_good"));
        assert!(cache.contains_bad_prime("clear_test_bad"));

        cache.clear();

        assert!(!cache.contains_good_prime("clear_test_good"));
        assert!(!cache.contains_bad_prime("clear_test_bad"));
    }
}
