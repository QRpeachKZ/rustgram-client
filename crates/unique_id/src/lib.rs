// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! # Unique ID
//!
//! Unique identifier generator for Telegram entities.
//!
//! ## Overview
//!
//! Thread-safe unique ID generation using atomic operations.
//!
//! ## Example
//!
//! ```no_run
//! use rustgram_unique_id::UniqueIdGenerator;
//!
//! let gen = UniqueIdGenerator::new();
//! let id1 = gen.next();
//! let id2 = gen.next();
//! assert!(id2 > id1);
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

/// Unique identifier generator
///
/// Thread-safe generator for unique IDs using atomic operations.
#[derive(Debug)]
pub struct UniqueIdGenerator {
    next_id: Arc<AtomicI64>,
}

impl Default for UniqueIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl UniqueIdGenerator {
    /// Creates a new ID generator starting from 1
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_id: Arc::new(AtomicI64::new(1)),
        }
    }

    /// Creates a new ID generator starting from the given value
    #[must_use]
    pub fn with_start(start: i64) -> Self {
        Self {
            next_id: Arc::new(AtomicI64::new(start)),
        }
    }

    /// Returns the next unique ID
    pub fn next(&self) -> i64 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Peeks at the next ID without consuming it
    #[must_use]
    pub fn peek(&self) -> i64 {
        self.next_id.load(Ordering::SeqCst)
    }

    /// Resets the generator to a new starting value
    pub fn reset(&self, new_start: i64) {
        self.next_id.store(new_start, Ordering::SeqCst);
    }
}

/// Unique identifier wrapper
///
/// Type-safe wrapper around i64 for unique IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UniqueId(pub i64);

impl UniqueId {
    /// Creates a new unique ID
    #[must_use]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }

    /// Returns the inner i64 value
    #[must_use]
    pub const fn get(self) -> i64 {
        self.0
    }

    /// Returns whether this ID is valid (non-zero)
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.0 != 0
    }
}

impl Default for UniqueId {
    fn default() -> Self {
        Self(0)
    }
}

impl fmt::Display for UniqueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for UniqueId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<UniqueId> for i64 {
    fn from(id: UniqueId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_new() {
        let gen = UniqueIdGenerator::new();
        assert_eq!(gen.peek(), 1);
    }

    #[test]
    fn test_generator_with_start() {
        let gen = UniqueIdGenerator::with_start(100);
        assert_eq!(gen.peek(), 100);
        assert_eq!(gen.next(), 100);
        assert_eq!(gen.peek(), 101);
    }

    #[test]
    fn test_generator_next() {
        let gen = UniqueIdGenerator::new();
        assert_eq!(gen.next(), 1);
        assert_eq!(gen.next(), 2);
        assert_eq!(gen.next(), 3);
    }

    #[test]
    fn test_generator_reset() {
        let gen = UniqueIdGenerator::new();
        gen.next();
        gen.next();
        gen.reset(50);
        assert_eq!(gen.next(), 50);
    }

    #[test]
    fn test_unique_id_new() {
        let id = UniqueId::new(123);
        assert_eq!(id.get(), 123);
    }

    #[test]
    fn test_unique_id_is_valid() {
        assert!(UniqueId::new(1).is_valid());
        assert!(UniqueId::new(-1).is_valid());
        assert!(!UniqueId::new(0).is_valid());
        assert!(!UniqueId::default().is_valid());
    }

    #[test]
    fn test_unique_id_from_i64() {
        let id: UniqueId = 456.into();
        assert_eq!(id.get(), 456);
    }

    #[test]
    fn test_unique_id_to_i64() {
        let id = UniqueId::new(789);
        let val: i64 = id.into();
        assert_eq!(val, 789);
    }

    #[test]
    fn test_unique_id_display() {
        assert_eq!(format!("{}", UniqueId::new(123)), "123");
    }

    #[test]
    fn test_unique_id_ordering() {
        assert!(UniqueId::new(1) < UniqueId::new(2));
        assert!(UniqueId::new(2) > UniqueId::new(1));
    }
}
