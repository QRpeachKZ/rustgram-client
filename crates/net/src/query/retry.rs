// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Retry logic for network queries.
//!
//! This module implements exponential backoff retry logic for
//! handling transient network failures.
//!
//! # References
//!
//! - TDLib: `td/telegram/net/NetQuery.cpp`

use parking_lot::Mutex;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use thiserror::Error;

use crate::query::{NetQuery, NetQueryId, QueryError, QueryErrorCode};

/// Error types for retry operations.
#[derive(Debug, Error)]
pub enum RetryError {
    /// Query not found
    #[error("Query not found: {0}")]
    NotFound(NetQueryId),

    /// Maximum retry attempts exceeded
    #[error("Maximum retry attempts exceeded for query {0}")]
    MaxAttemptsExceeded(NetQueryId),

    /// Invalid retry policy
    #[error("Invalid retry policy: {0}")]
    InvalidPolicy(String),
}

/// Result type for retry operations.
pub type RetryResult<T> = Result<T, RetryError>;

/// Retry policy configuration.
///
/// Defines how queries should be retried on failure.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts.
    /// Default: 3
    pub max_attempts: u32,

    /// Base delay before first retry.
    /// Default: 1 second
    pub base_delay: Duration,

    /// Maximum delay between retries.
    /// Default: 60 seconds
    pub max_delay: Duration,

    /// Backoff multiplier for exponential backoff.
    /// Default: 2.0 (doubles each time)
    pub backoff_multiplier: f64,

    /// Whether to add jitter to delay (random variation).
    /// Default: true
    pub jitter: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
}

impl RetryPolicy {
    /// Creates a new retry policy with the specified max attempts.
    pub fn new(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            ..Default::default()
        }
    }

    /// Calculates the delay for a given retry attempt.
    ///
    /// Uses exponential backoff with optional jitter.
    ///
    /// # Arguments
    ///
    /// * `attempt` - The attempt number (0-based)
    ///
    /// # Returns
    ///
    /// The delay duration for this attempt
    pub fn retry_delay(&self, attempt: u32) -> Duration {
        let base_delay_ms = self.base_delay.as_millis() as f64;
        let multiplier = self.backoff_multiplier.powi(attempt as i32);
        let delay_ms = base_delay_ms * multiplier;

        // Cap at max delay
        let delay_ms = delay_ms.min(self.max_delay.as_millis() as f64);

        // Add jitter if enabled
        let delay_ms = if self.jitter {
            // Add ±25% random variation
            let jitter_factor = 0.75 + (rand::random::<f64>() * 0.5);
            delay_ms * jitter_factor
        } else {
            delay_ms
        };

        Duration::from_millis(delay_ms as u64)
    }

    /// Validates the retry policy.
    ///
    /// # Returns
    ///
    /// Ok(()) if valid, Err otherwise
    pub fn validate(&self) -> RetryResult<()> {
        if self.max_attempts == 0 {
            return Err(RetryError::InvalidPolicy(
                "max_attempts must be > 0".into(),
            ));
        }
        if self.base_delay.as_millis() == 0 {
            return Err(RetryError::InvalidPolicy(
                "base_delay must be > 0".into(),
            ));
        }
        if self.backoff_multiplier < 1.0 {
            return Err(RetryError::InvalidPolicy(
                "backoff_multiplier must be >= 1.0".into(),
            ));
        }
        Ok(())
    }
}

/// Entry tracking retry attempts for a query.
#[derive(Debug, Clone)]
struct RetryEntry {
    /// The query being retried
    query: NetQuery,

    /// Current attempt number (0-based)
    attempt: u32,

    /// When the next retry should be attempted
    next_retry_at: Option<Instant>,

    /// The last error that caused the retry
    last_error: Option<QueryError>,
}

impl RetryEntry {
    /// Creates a new retry entry.
    fn new(query: NetQuery) -> Self {
        Self {
            query,
            attempt: 0,
            next_retry_at: None,
            last_error: None,
        }
    }

    /// Returns `true` if this entry can be retried.
    fn can_retry(&self, max_attempts: u32) -> bool {
        self.attempt < max_attempts
    }

    /// Records an attempt.
    fn record_attempt(&mut self, error: QueryError, delay: Duration) {
        self.attempt += 1;
        self.last_error = Some(error);
        self.next_retry_at = Some(Instant::now() + delay);
    }

    /// Returns `true` if it's time to retry.
    fn ready_to_retry(&self) -> bool {
        if let Some(next_at) = self.next_retry_at {
            Instant::now() >= next_at
        } else {
            true
        }
    }

    /// Returns the time until next retry.
    fn time_until_retry(&self) -> Option<Duration> {
        self.next_retry_at.map(|next_at| {
            let now = Instant::now();
            if now >= next_at {
                Duration::ZERO
            } else {
                next_at - now
            }
        })
    }
}

/// Manages retry logic for network queries.
///
/// Tracks retry attempts, calculates backoff delays, and determines
/// when queries should be retried.
pub struct RetryManager {
    /// Active retry entries (query_id -> entry)
    retries: Mutex<HashMap<NetQueryId, RetryEntry>>,

    /// Retry policy
    policy: RetryPolicy,
}

impl RetryManager {
    /// Creates a new retry manager with default policy.
    pub fn new() -> Self {
        Self::with_policy(RetryPolicy::default())
    }

    /// Creates a new retry manager with the given policy.
    #[allow(clippy::unwrap_used)]
    pub fn with_policy(policy: RetryPolicy) -> Self {
        policy.validate().unwrap();
        Self {
            retries: Mutex::new(HashMap::new()),
            policy,
        }
    }

    /// Returns the retry policy.
    pub fn policy(&self) -> &RetryPolicy {
        &self.policy
    }

    /// Records a retry attempt for a query.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The query ID
    /// * `error` - The error that caused the retry
    ///
    /// # Returns
    ///
    /// The delay before the next retry, or an error if max attempts exceeded
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rustgram_net::query::retry::RetryManager;
    ///
    /// let manager = RetryManager::new();
    /// let delay = manager.record_attempt(query_id, error)?;
    /// ```
    pub fn record_attempt(&self, query_id: NetQueryId, error: QueryError) -> RetryResult<Duration> {
        let mut retries = self.retries.lock();

        let entry = retries
            .get_mut(&query_id)
            .ok_or(RetryError::NotFound(query_id))?;

        // Check if we can retry
        if !entry.can_retry(self.policy.max_attempts) {
            return Err(RetryError::MaxAttemptsExceeded(query_id));
        }

        // Calculate delay
        let delay = self.policy.retry_delay(entry.attempt);

        // Record the attempt
        entry.record_attempt(error, delay);

        Ok(delay)
    }

    /// Determines whether an error should trigger a retry.
    ///
    /// # Arguments
    ///
    /// * `error` - The error to check
    ///
    /// # Returns
    ///
    /// `true` if the query should be retried
    pub fn should_retry(&self, error: &QueryError) -> bool {
        match error {
            // Network errors should be retried
            QueryError::WithMessage { code, message } => {
                // Retry on 5xx errors and timeout
                *code >= 500 || message.contains("timeout") || message.contains("network")
            }
            QueryError::Special(s) => matches!(s, QueryErrorCode::Resend | QueryErrorCode::ResendInvokeAfter),

            // Generic errors with certain keywords should be retried
            QueryError::Generic(msg) => {
                msg.contains("timeout") || msg.contains("network") || msg.contains("connection")
            }
        }
    }

    /// Registers a query for retry tracking.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The query ID
    /// * `query` - The query to track
    pub fn register(&self, query_id: NetQueryId, query: NetQuery) {
        let mut retries = self.retries.lock();
        retries.insert(query_id, RetryEntry::new(query));
    }

    /// Unregisters a query from retry tracking.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The query ID to unregister
    ///
    /// # Returns
    ///
    /// The query entry if it existed
    pub fn unregister(&self, query_id: NetQueryId) -> Option<NetQuery> {
        let mut retries = self.retries.lock();
        retries.remove(&query_id).map(|entry| entry.query)
    }

    /// Returns queries that are ready to be retried.
    ///
    /// # Returns
    ///
    /// A list of (query_id, query) tuples ready for retry
    pub fn ready_queries(&self) -> Vec<(NetQueryId, NetQuery)> {
        let retries = self.retries.lock();

        retries
            .iter()
            .filter(|(_, entry)| entry.ready_to_retry())
            .map(|(id, entry)| (*id, entry.query.clone()))
            .collect()
    }

    /// Returns the number of active tracked queries.
    pub fn active_count(&self) -> usize {
        self.retries.lock().len()
    }

    /// Returns the attempt count for a query.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The query ID to check
    ///
    /// # Returns
    ///
    /// The number of attempts made, or None if query not tracked
    pub fn attempt_count(&self, query_id: NetQueryId) -> Option<u32> {
        let retries = self.retries.lock();
        retries.get(&query_id).map(|entry| entry.attempt)
    }

    /// Returns the time until next retry for a query.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The query ID to check
    ///
    /// # Returns
    ///
    /// The duration until next retry, or None if not applicable
    pub fn time_until_retry(&self, query_id: NetQueryId) -> Option<Duration> {
        let retries = self.retries.lock();
        retries.get(&query_id).and_then(|entry| entry.time_until_retry())
    }

    /// Clears all tracked queries.
    pub fn clear(&self) {
        self.retries.lock().clear();
    }
}

impl Default for RetryManager {
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
    fn test_retry_policy_default() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_attempts, 3);
        assert_eq!(policy.base_delay, Duration::from_secs(1));
        assert_eq!(policy.max_delay, Duration::from_secs(60));
        assert_eq!(policy.backoff_multiplier, 2.0);
        assert!(policy.jitter);
    }

    #[test]
    fn test_retry_policy_new() {
        let policy = RetryPolicy::new(5);
        assert_eq!(policy.max_attempts, 5);
        assert_eq!(policy.base_delay, Duration::from_secs(1));
    }

    #[test]
    fn test_retry_policy_validate() {
        let policy = RetryPolicy::default();
        assert!(policy.validate().is_ok());

        let invalid_policy = RetryPolicy {
            max_attempts: 0,
            ..Default::default()
        };
        assert!(invalid_policy.validate().is_err());

        let invalid_policy = RetryPolicy {
            base_delay: Duration::ZERO,
            ..Default::default()
        };
        assert!(invalid_policy.validate().is_err());
    }

    #[test]
    fn test_retry_policy_retry_delay() {
        let policy = RetryPolicy {
            base_delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
            jitter: false,
            ..Default::default()
        };

        // Attempt 0: 100ms
        assert_eq!(policy.retry_delay(0), Duration::from_millis(100));

        // Attempt 1: 200ms
        assert_eq!(policy.retry_delay(1), Duration::from_millis(200));

        // Attempt 2: 400ms
        assert_eq!(policy.retry_delay(2), Duration::from_millis(400));
    }

    #[test]
    fn test_retry_policy_retry_delay_with_cap() {
        let policy = RetryPolicy {
            base_delay: Duration::from_secs(10),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 4.0,
            jitter: false,
            ..Default::default()
        };

        // Should cap at max_delay
        let delay = policy.retry_delay(2); // 10 * 4^2 = 160s, capped at 30s
        assert_eq!(delay, Duration::from_secs(30));
    }

    #[test]
    fn test_retry_manager_new() {
        let manager = RetryManager::new();
        assert_eq!(manager.policy().max_attempts, 3);
        assert_eq!(manager.active_count(), 0);
    }

    #[test]
    fn test_retry_manager_register_and_unregister() {
        let manager = RetryManager::new();
        let query_id = 12345;
        let query = NetQuery::new(
            query_id,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        );

        manager.register(query_id, query.clone());
        assert_eq!(manager.active_count(), 1);

        let retrieved = manager.unregister(query_id);
        assert!(retrieved.is_some());
        assert_eq!(manager.active_count(), 0);
    }

    #[test]
    fn test_retry_manager_attempt_count() {
        let manager = RetryManager::new();
        let query_id = 12345;
        let query = NetQuery::new(
            query_id,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        );

        manager.register(query_id, query);

        assert_eq!(manager.attempt_count(query_id), Some(0));
    }

    #[test]
    fn test_retry_manager_record_attempt() {
        let manager = RetryManager::new();
        let query_id = 12345;
        let query = NetQuery::new(
            query_id,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        );

        manager.register(query_id, query);

        let error = QueryError::Generic("test error".into());
        let delay = manager.record_attempt(query_id, error);

        assert!(delay.is_ok());
        assert_eq!(manager.attempt_count(query_id), Some(1));
    }

    #[test]
    fn test_retry_manager_max_attempts() {
        let manager = RetryManager::new();
        let query_id = 12345;
        let query = NetQuery::new(
            query_id,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        );

        manager.register(query_id, query);

        let error = QueryError::Generic("test error".into());

        // First attempt
        assert!(manager.record_attempt(query_id, error.clone()).is_ok());
        assert_eq!(manager.attempt_count(query_id), Some(1));

        // Second attempt
        assert!(manager.record_attempt(query_id, error.clone()).is_ok());
        assert_eq!(manager.attempt_count(query_id), Some(2));

        // Third attempt
        assert!(manager.record_attempt(query_id, error.clone()).is_ok());
        assert_eq!(manager.attempt_count(query_id), Some(3));

        // Fourth attempt should fail (max is 3)
        assert!(manager.record_attempt(query_id, error).is_err());
    }

    #[test]
    fn test_retry_manager_should_retry() {
        let manager = RetryManager::new();

        // Network and timeout errors should retry
        assert!(manager.should_retry(&QueryError::Generic("timeout".into())));
        assert!(manager.should_retry(&QueryError::Generic("network error".into())));
        assert!(manager.should_retry(&QueryError::Generic("connection failed".into())));

        // 500+ errors should retry
        assert!(manager.should_retry(&QueryError::WithMessage {
            code: 500,
            message: "Internal Server Error".into(),
        }));
        assert!(manager.should_retry(&QueryError::WithMessage {
            code: 503,
            message: "Service Unavailable".into(),
        }));

        // Other errors should not retry
        assert!(!manager.should_retry(&QueryError::WithMessage {
            code: 400,
            message: "Bad Request".into(),
        }));
        assert!(!manager.should_retry(&QueryError::WithMessage {
            code: 404,
            message: "Not Found".into(),
        }));
        assert!(!manager.should_retry(&QueryError::WithMessage {
            code: 429,
            message: "Rate Limited".into(),
        }));
        assert!(!manager.should_retry(&QueryError::Generic("not found".into())));
    }

    #[test]
    fn test_retry_manager_clear() {
        let manager = RetryManager::new();

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
            manager.register(i, query);
        }

        assert_eq!(manager.active_count(), 5);
        manager.clear();
        assert_eq!(manager.active_count(), 0);
    }

    #[test]
    fn test_retry_error_display() {
        let id = 12345u64;
        let err = RetryError::NotFound(id);
        assert!(err.to_string().contains("Query not found"));
        assert!(err.to_string().contains("12345"));

        let err = RetryError::MaxAttemptsExceeded(id);
        assert!(err.to_string().contains("Maximum retry attempts exceeded"));
        assert!(err.to_string().contains("12345"));

        let err = RetryError::InvalidPolicy("test".into());
        assert!(err.to_string().contains("Invalid retry policy"));
        assert!(err.to_string().contains("test"));
    }
}
