// Copyright (c) 2024 rustgram-client contributors
// Licensed under MIT OR Apache-2.0

//! # Query Merger
//!
//! Query merger state for Telegram.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use serde::{Deserialize, Serialize};

/// Query merger info.
///
/// Represents query merging state.
/// Based on TDLib's `QueryMerger` class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryMerger {
    /// Maximum concurrent query count.
    max_concurrent_query_count: usize,
    /// Maximum merged query count.
    max_merged_query_count: usize,
}

impl Default for QueryMerger {
    fn default() -> Self {
        Self {
            max_concurrent_query_count: 1,
            max_merged_query_count: 10,
        }
    }
}

impl QueryMerger {
    /// Creates a new query merger.
    #[must_use]
    pub const fn new(max_concurrent: usize, max_merged: usize) -> Self {
        Self {
            max_concurrent_query_count: max_concurrent,
            max_merged_query_count: max_merged,
        }
    }

    /// Returns the maximum concurrent query count.
    #[must_use]
    pub const fn max_concurrent_query_count(&self) -> usize {
        self.max_concurrent_query_count
    }

    /// Returns the maximum merged query count.
    #[must_use]
    pub const fn max_merged_query_count(&self) -> usize {
        self.max_merged_query_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let merger = QueryMerger::default();
        assert_eq!(merger.max_concurrent_query_count(), 1);
    }

    #[test]
    fn test_new() {
        let merger = QueryMerger::new(5, 20);
        assert_eq!(merger.max_concurrent_query_count(), 5);
        assert_eq!(merger.max_merged_query_count(), 20);
    }
}
