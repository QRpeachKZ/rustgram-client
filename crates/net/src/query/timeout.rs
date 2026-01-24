// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Query timeout management for MTProto network queries.
//!
//! This module provides timeout tracking for network queries,
//! preventing queries from hanging indefinitely.
//!
//! # References
//!
//! - TDLib: `td/telegram/net/NetQuery.h`

use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;

use crate::query::{NetQuery, NetQueryId};

/// Error types for timeout management.
#[derive(Debug, Error)]
pub enum TimeoutError {
    /// Query not found
    #[error("Query not found: {0}")]
    NotFound(NetQueryId),

    /// Query already timed out
    #[error("Query already timed out: {0}")]
    AlreadyTimedOut(NetQueryId),

    /// Invalid timeout duration
    #[error("Invalid timeout duration: {0:?}")]
    InvalidTimeout(Duration),
}

/// Result type for timeout operations.
pub type TimeoutResult<T> = Result<T, TimeoutError>;

/// Entry for tracking a query timeout.
#[derive(Debug, Clone)]
struct TimeoutEntry {
    /// The query
    query: NetQuery,

    /// When the query was registered
    registered_at: Instant,

    /// When the query times out
    timeout_at: Instant,

    /// The timeout duration
    timeout: Duration,

    /// Whether this query has timed out
    timed_out: bool,
}

impl TimeoutEntry {
    /// Creates a new timeout entry.
    fn new(query: NetQuery, timeout: Duration) -> Self {
        let now = Instant::now();
        Self {
            query,
            registered_at: now,
            timeout_at: now + timeout,
            timeout,
            timed_out: false,
        }
    }

    /// Returns `true` if this entry has timed out.
    fn is_timed_out(&self, now: Instant) -> bool {
        now >= self.timeout_at
    }

    /// Returns the remaining time until timeout.
    fn remaining(&self) -> Option<Duration> {
        let now = Instant::now();
        if now < self.timeout_at {
            Some(self.timeout_at - now)
        } else {
            None
        }
    }
}

/// Configuration for timeout management.
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    /// Default timeout for queries.
    /// Default: 30 seconds
    pub default_timeout: Duration,

    /// Minimum timeout allowed.
    /// Default: 1 second
    pub min_timeout: Duration,

    /// Maximum timeout allowed.
    /// Default: 5 minutes
    pub max_timeout: Duration,

    /// Interval for checking timeouts.
    /// Default: 1 second
    pub check_interval: Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            default_timeout: Duration::from_secs(30),
            min_timeout: Duration::from_secs(1),
            max_timeout: Duration::from_secs(300),
            check_interval: Duration::from_secs(1),
        }
    }
}

/// Manages timeout tracking for network queries.
///
/// The TimeoutManager tracks query lifetimes and notifies when
/// queries exceed their allocated time.
pub struct QueryTimeoutManager {
    /// Active timeouts (query_id -> entry)
    timeouts: Mutex<HashMap<NetQueryId, TimeoutEntry>>,

    /// Configuration
    config: TimeoutConfig,

    /// Next query ID (for generating unique IDs)
    next_id: Arc<std::sync::atomic::AtomicU64>,
}

impl QueryTimeoutManager {
    /// Creates a new timeout manager with default configuration.
    pub fn new() -> Self {
        Self::with_config(TimeoutConfig::default())
    }

    /// Creates a new timeout manager with the given configuration.
    pub fn with_config(config: TimeoutConfig) -> Self {
        Self {
            timeouts: Mutex::new(HashMap::new()),
            config,
            next_id: Arc::new(std::sync::atomic::AtomicU64::new(1)),
        }
    }

    /// Returns the configuration.
    pub fn config(&self) -> &TimeoutConfig {
        &self.config
    }

    /// Registers a query for timeout tracking.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to track
    /// * `timeout` - The timeout duration (uses default if None)
    ///
    /// # Returns
    ///
    /// The query ID
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rustgram_net::query::timeout::QueryTimeoutManager;
    ///
    /// let manager = QueryTimeoutManager::new();
    /// let query = NetQuery::new();
    /// let id = manager.register(query, None)?;
    /// ```
    pub fn register(&self, query: NetQuery, timeout: Option<Duration>) -> TimeoutResult<NetQueryId> {
        let timeout = timeout.unwrap_or(self.config.default_timeout);

        // Validate timeout
        if timeout < self.config.min_timeout || timeout > self.config.max_timeout {
            return Err(TimeoutError::InvalidTimeout(timeout));
        }

        // Generate unique ID
        let id = self.next_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let entry = TimeoutEntry::new(query, timeout);

        let mut timeouts = self.timeouts.lock();
        timeouts.insert(id, entry);

        Ok(id)
    }

    /// Cancels timeout tracking for a query.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The query ID to cancel
    pub fn cancel(&self, query_id: NetQueryId) -> TimeoutResult<NetQuery> {
        let mut timeouts = self.timeouts.lock();

        timeouts
            .remove(&query_id)
            .map(|entry| entry.query)
            .ok_or(TimeoutError::NotFound(query_id))
    }

    /// Checks for timed out queries.
    ///
    /// # Arguments
    ///
    /// * `now` - The current time (uses Instant::now() if None)
    ///
    /// # Returns
    ///
    /// A list of timed out queries
    pub fn check_timeouts(&self, now: Option<Instant>) -> Vec<NetQuery> {
        let now = now.unwrap_or_else(Instant::now);
        let mut timeouts = self.timeouts.lock();
        let mut timed_out = Vec::new();

        // Collect timed out queries
        let mut to_remove = Vec::new();
        for (id, entry) in timeouts.iter() {
            if entry.is_timed_out(now) && !entry.timed_out {
                to_remove.push(*id);
            }
        }

        // Remove timed out queries and collect them
        for id in to_remove {
            if let Some(mut entry) = timeouts.remove(&id) {
                entry.timed_out = true;
                timed_out.push(entry.query);
            }
        }

        timed_out
    }

    /// Returns the number of active tracked queries.
    pub fn active_count(&self) -> usize {
        self.timeouts.lock().len()
    }

    /// Returns `true` if a query is currently being tracked.
    pub fn is_tracking(&self, query_id: NetQueryId) -> bool {
        self.timeouts.lock().contains_key(&query_id)
    }

    /// Returns the remaining time for a query, if it exists.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The query ID to check
    ///
    /// # Returns
    ///
    /// None if query doesn't exist or has timed out, Some(Duration) otherwise
    pub fn remaining_time(&self, query_id: NetQueryId) -> Option<Duration> {
        let timeouts = self.timeouts.lock();
        timeouts.get(&query_id).and_then(|entry| entry.remaining())
    }

    /// Clears all tracked queries.
    pub fn clear(&self) {
        self.timeouts.lock().clear();
    }
}

impl Default for QueryTimeoutManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dc::DcId;
    use crate::query::{AuthFlag, GzipFlag, NetQueryType};
    use bytes::Bytes;

    #[test]
    fn test_timeout_config_default() {
        let config = TimeoutConfig::default();
        assert_eq!(config.default_timeout, Duration::from_secs(30));
        assert_eq!(config.min_timeout, Duration::from_secs(1));
        assert_eq!(config.max_timeout, Duration::from_secs(300));
        assert_eq!(config.check_interval, Duration::from_secs(1));
    }

    #[test]
    fn test_timeout_manager_new() {
        let manager = QueryTimeoutManager::new();
        assert_eq!(manager.config().default_timeout, Duration::from_secs(30));
        assert_eq!(manager.active_count(), 0);
    }

    #[test]
    fn test_timeout_entry_remaining() {
        use std::thread;
        use std::time::Duration;

        let query = NetQuery::new(
            1,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        );
        let entry = TimeoutEntry::new(query, Duration::from_millis(100));

        // Should have some remaining time
        let remaining = entry.remaining();
        assert!(remaining.is_some());
        assert!(remaining.unwrap() <= Duration::from_millis(100));

        // Wait for timeout
        thread::sleep(Duration::from_millis(150));

        // No remaining time
        let remaining = entry.remaining();
        assert!(remaining.is_none());
    }

    #[test]
    fn test_timeout_error_display() {
        let id = 12345u64;
        let err = TimeoutError::NotFound(id);
        assert!(err.to_string().contains("Query not found"));
        assert!(err.to_string().contains("12345"));

        let err = TimeoutError::AlreadyTimedOut(id);
        assert!(err.to_string().contains("already timed out"));

        let duration = Duration::from_secs(0);
        let err = TimeoutError::InvalidTimeout(duration);
        assert!(err.to_string().contains("Invalid timeout"));
    }

    #[test]
    fn test_invalid_timeout() {
        let manager = QueryTimeoutManager::new();
        let query = NetQuery::new(
            1,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        );

        // Too short
        let result = manager.register(query, Some(Duration::from_millis(100)));
        assert!(result.is_err());

        // Too long
        let query = NetQuery::new(
            2,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        );
        let result = manager.register(query, Some(Duration::from_secs(1000)));
        assert!(result.is_err());
    }

    #[test]
    fn test_timeout_manager_register_and_cancel() {
        let manager = QueryTimeoutManager::new();
        let query = NetQuery::new(
            1,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        );

        let id = manager.register(query, None).unwrap();
        assert_eq!(manager.active_count(), 1);
        assert!(manager.is_tracking(id));

        let _ = manager.cancel(id).unwrap();
        assert_eq!(manager.active_count(), 0);
        assert!(!manager.is_tracking(id));
    }

    #[test]
    fn test_timeout_manager_cancel_nonexistent() {
        let manager = QueryTimeoutManager::new();
        let result = manager.cancel(99999);
        assert!(result.is_err());
    }

    #[test]
    fn test_timeout_manager_clear() {
        let manager = QueryTimeoutManager::new();

        for i in 0..5 {
            let query = NetQuery::new(
                i,
                Bytes::new(),
                DcId::internal(2),
                NetQueryType::Common,
                AuthFlag::Off,
                GzipFlag::Off,
                0,
            );
            manager.register(query, None).unwrap();
        }

        assert_eq!(manager.active_count(), 5);
        manager.clear();
        assert_eq!(manager.active_count(), 0);
    }

    #[test]
    fn test_timeout_manager_remaining_time() {
        let manager = QueryTimeoutManager::new();
        let query = NetQuery::new(
            1,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        );

        let id = manager.register(query, Some(Duration::from_secs(2))).unwrap();

        let remaining = manager.remaining_time(id);
        assert!(remaining.is_some());
        assert!(remaining.unwrap() <= Duration::from_secs(2));

        // Cancel query
        manager.cancel(id).unwrap();

        // No remaining time after cancel
        let remaining = manager.remaining_time(id);
        assert!(remaining.is_none());
    }
}
