// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Query Combiner
//!
//! Query combiner state for Telegram.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use serde::{Deserialize, Serialize};

/// Query state for the combiner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QueryState {
    /// Query is pending.
    Pending,
    /// Query was sent.
    Sent,
    /// Query completed successfully.
    Succeeded,
    /// Query failed.
    Failed,
}

impl Default for QueryState {
    fn default() -> Self {
        Self::Pending
    }
}

/// Query combiner info.
///
/// Represents query combination state.
/// Based on TDLib's `QueryCombiner` class.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryCombiner {
    /// Query count.
    query_count: i32,
    /// Minimum delay between queries in seconds.
    min_delay: f64,
}

impl Default for QueryCombiner {
    fn default() -> Self {
        Self {
            query_count: 0,
            min_delay: 0.0,
        }
    }
}

impl QueryCombiner {
    /// Creates a new query combiner.
    #[must_use]
    pub const fn new(min_delay: f64) -> Self {
        Self {
            query_count: 0,
            min_delay,
        }
    }

    /// Returns the query count.
    #[must_use]
    pub const fn query_count(&self) -> i32 {
        self.query_count
    }

    /// Returns the minimum delay.
    #[must_use]
    pub const fn min_delay(&self) -> f64 {
        self.min_delay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let combiner = QueryCombiner::default();
        assert_eq!(combiner.query_count(), 0);
    }

    #[test]
    fn test_new() {
        let combiner = QueryCombiner::new(1.5);
        assert_eq!(combiner.min_delay(), 1.5);
    }
}
