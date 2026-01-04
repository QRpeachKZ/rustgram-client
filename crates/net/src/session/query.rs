// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Query lifecycle management for MTProto sessions.
//!
//! This module manages the lifecycle of queries within a session.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use bytes::Bytes;
use parking_lot::Mutex;
use tokio::sync::oneshot;

/// Query state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryState {
    /// Query is pending
    Pending,

    /// Query is in flight
    InFlight,

    /// Query completed successfully
    Completed,

    /// Query failed
    Failed,

    /// Query was canceled
    Canceled,
}

/// Query entry in the lifecycle manager.
struct QueryEntry {
    /// Query ID
    query_id: u64,

    /// Message ID
    message_id: Option<u64>,

    /// Current state
    state: QueryState,

    /// Creation time
    created_at: Instant,

    /// Last activity time
    last_activity: Instant,

    /// Timeout duration
    timeout: Duration,

    /// Response sender
    response_sender: Option<oneshot::Sender<Result<Bytes, String>>>,

    /// Retry count
    retry_count: u32,

    /// Maximum retries
    max_retries: u32,
}

/// Query lifecycle manager.
///
/// Manages the lifecycle of queries from creation to completion.
pub struct QueryLifecycle {
    /// Next query ID
    next_query_id: AtomicU64,

    /// Active queries (query_id -> QueryEntry)
    queries: Mutex<HashMap<u64, QueryEntry>>,

    /// Queries by message ID (message_id -> query_id)
    by_message_id: Mutex<HashMap<u64, u64>>,

    /// Statistics
    statistics: Mutex<QueryLifecycleStats>,
}

/// Query lifecycle statistics.
#[derive(Debug, Clone, Default)]
pub struct QueryLifecycleStats {
    /// Total queries created
    pub total_queries: u64,

    /// Queries completed successfully
    pub successful_queries: u64,

    /// Queries failed
    pub failed_queries: u64,

    /// Queries timed out
    pub timed_out_queries: u64,

    /// Queries retried
    pub retried_queries: u64,

    /// Average query duration in milliseconds
    pub avg_duration_ms: f64,
}

impl QueryLifecycle {
    /// Creates a new query lifecycle manager.
    pub fn new() -> Self {
        Self {
            next_query_id: AtomicU64::new(1),
            queries: Mutex::new(HashMap::new()),
            by_message_id: Mutex::new(HashMap::new()),
            statistics: Mutex::new(QueryLifecycleStats::default()),
        }
    }

    /// Creates a new query.
    pub fn create_query(
        &self,
        timeout: Duration,
        max_retries: u32,
    ) -> (u64, oneshot::Receiver<Result<Bytes, String>>) {
        let query_id = self.next_query_id.fetch_add(1, Ordering::Relaxed);

        let (sender, receiver) = oneshot::channel();

        let entry = QueryEntry {
            query_id,
            message_id: None,
            state: QueryState::Pending,
            created_at: Instant::now(),
            last_activity: Instant::now(),
            timeout,
            response_sender: Some(sender),
            retry_count: 0,
            max_retries,
        };

        self.queries.lock().insert(query_id, entry);
        self.statistics.lock().total_queries += 1;

        (query_id, receiver)
    }

    /// Associates a message ID with a query (sent).
    pub fn mark_sent(&self, query_id: u64, message_id: u64) -> Result<(), String> {
        let mut queries = self.queries.lock();

        let entry = queries
            .get_mut(&query_id)
            .ok_or_else(|| "Query not found".to_string())?;

        entry.message_id = Some(message_id);
        entry.state = QueryState::InFlight;
        entry.last_activity = Instant::now();

        self.by_message_id.lock().insert(message_id, query_id);

        Ok(())
    }

    /// Marks a query as completed with a response.
    pub fn mark_completed(&self, message_id: u64, response: Bytes) -> Result<(), String> {
        let query_id = self
            .by_message_id
            .lock()
            .remove(&message_id)
            .ok_or_else(|| "No query for message ID".to_string())?;

        let mut queries = self.queries.lock();

        let entry = queries
            .get_mut(&query_id)
            .ok_or_else(|| "Query not found".to_string())?;

        entry.state = QueryState::Completed;
        entry.last_activity = Instant::now();

        let duration = entry.created_at.elapsed().as_millis() as f64;

        // Update statistics
        let mut stats = self.statistics.lock();
        stats.successful_queries += 1;
        stats.avg_duration_ms = (stats.avg_duration_ms * (stats.successful_queries - 1) as f64
            + duration)
            / stats.successful_queries as f64;

        // Send response
        if let Some(sender) = entry.response_sender.take() {
            let _ = sender.send(Ok(response));
        }

        // Remove from active queries
        queries.remove(&query_id);

        Ok(())
    }

    /// Marks a query as failed.
    pub fn mark_failed(&self, query_id: u64, error: String) -> Result<(), String> {
        let mut queries = self.queries.lock();

        let entry = queries
            .get_mut(&query_id)
            .ok_or_else(|| "Query not found".to_string())?;

        entry.state = QueryState::Failed;
        entry.last_activity = Instant::now();

        self.statistics.lock().failed_queries += 1;

        // Check if we should retry
        if entry.retry_count + 1 < entry.max_retries {
            entry.retry_count += 1;
            entry.state = QueryState::Pending;
            entry.message_id = None;
            self.statistics.lock().retried_queries += 1;

            return Ok(()); // Don't send error yet, will retry
        }

        // Send error
        if let Some(sender) = entry.response_sender.take() {
            let _ = sender.send(Err(error));
        }

        // Remove from active queries
        queries.remove(&query_id);

        Ok(())
    }

    /// Marks a query as canceled.
    pub fn mark_canceled(&self, query_id: u64) -> Result<(), String> {
        let mut queries = self.queries.lock();

        let entry = queries
            .get_mut(&query_id)
            .ok_or_else(|| "Query not found".to_string())?;

        entry.state = QueryState::Canceled;
        entry.last_activity = Instant::now();

        // Send cancellation
        if let Some(sender) = entry.response_sender.take() {
            let _ = sender.send(Err("Query canceled".to_string()));
        }

        // Remove from active queries
        queries.remove(&query_id);

        Ok(())
    }

    /// Finds a query by message ID.
    pub fn find_by_message_id(&self, message_id: u64) -> Option<u64> {
        self.by_message_id.lock().get(&message_id).copied()
    }

    /// Gets the query state.
    pub fn get_state(&self, query_id: u64) -> Option<QueryState> {
        self.queries.lock().get(&query_id).map(|e| e.state)
    }

    /// Cleans up timed out queries.
    pub fn cleanup_timeouts(&self) -> usize {
        let mut queries = self.queries.lock();
        let mut to_remove = Vec::new();
        let now = Instant::now();

        for (&query_id, entry) in queries.iter() {
            if entry.state == QueryState::InFlight
                && now.duration_since(entry.last_activity) > entry.timeout
            {
                to_remove.push(query_id);
            }
        }

        let count = to_remove.len();

        for query_id in &to_remove {
            if let Some(entry) = queries.get_mut(query_id) {
                // Check if we should retry
                if entry.retry_count + 1 < entry.max_retries {
                    entry.retry_count += 1;
                    entry.state = QueryState::Pending;
                    entry.message_id = None;
                    self.statistics.lock().retried_queries += 1;
                    self.statistics.lock().timed_out_queries += 1;
                } else {
                    // Mark as failed
                    entry.state = QueryState::Failed;

                    if let Some(sender) = entry.response_sender.take() {
                        let _ = sender.send(Err("Query timeout".to_string()));
                    }

                    if let Some(msg_id) = entry.message_id {
                        self.by_message_id.lock().remove(&msg_id);
                    }
                }
            }
        }

        // Remove permanently failed queries
        let permanently_failed: Vec<_> = queries
            .iter()
            .filter(|(_, e)| e.state == QueryState::Failed)
            .map(|(&id, _)| id)
            .collect();

        for id in permanently_failed {
            queries.remove(&id);
        }

        count
    }

    /// Returns the number of active queries.
    pub fn active_count(&self) -> usize {
        let queries = self.queries.lock();
        queries.len()
    }

    /// Returns the statistics.
    pub fn statistics(&self) -> QueryLifecycleStats {
        self.statistics.lock().clone()
    }

    /// Clears all queries.
    pub fn clear(&self) {
        self.queries.lock().clear();
        self.by_message_id.lock().clear();
    }
}

impl Default for QueryLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_state_values() {
        assert_eq!(QueryState::Pending as i32, 0);
        assert_eq!(QueryState::InFlight as i32, 1);
        assert_eq!(QueryState::Completed as i32, 2);
        assert_eq!(QueryState::Failed as i32, 3);
        assert_eq!(QueryState::Canceled as i32, 4);
    }

    #[test]
    fn test_query_lifecycle_new() {
        let lifecycle = QueryLifecycle::new();
        assert_eq!(lifecycle.active_count(), 0);
    }

    #[test]
    fn test_query_lifecycle_create() {
        let lifecycle = QueryLifecycle::new();

        let (query_id, _receiver) = lifecycle.create_query(Duration::from_secs(10), 3);

        assert_eq!(query_id, 1); // First query
        assert_eq!(lifecycle.active_count(), 1);
        assert_eq!(lifecycle.get_state(query_id), Some(QueryState::Pending));
    }

    #[test]
    fn test_query_lifecycle_mark_sent() {
        let lifecycle = QueryLifecycle::new();

        let (query_id, _receiver) = lifecycle.create_query(Duration::from_secs(10), 3);

        let message_id = 12345;
        assert!(lifecycle.mark_sent(query_id, message_id).is_ok());

        assert_eq!(lifecycle.get_state(query_id), Some(QueryState::InFlight));
        assert_eq!(lifecycle.find_by_message_id(message_id), Some(query_id));
    }

    #[test]
    fn test_query_lifecycle_mark_completed() {
        let lifecycle = QueryLifecycle::new();

        let (query_id, mut receiver) = lifecycle.create_query(Duration::from_secs(10), 3);
        assert!(lifecycle.mark_sent(query_id, 12345).is_ok());

        let response = Bytes::from_static(b"test response");
        assert!(lifecycle.mark_completed(12345, response.clone()).is_ok());

        assert_eq!(lifecycle.active_count(), 0);

        // Check we received the response
        match receiver.try_recv() {
            Ok(Ok(result)) => assert_eq!(result, response),
            _ => panic!("Expected Ok response"),
        }

        let stats = lifecycle.statistics();
        assert_eq!(stats.successful_queries, 1);
    }

    #[test]
    fn test_query_lifecycle_mark_failed_with_retry() {
        let lifecycle = QueryLifecycle::new();

        let (query_id, mut receiver) = lifecycle.create_query(Duration::from_secs(10), 3); // Allow 2 retries (max 3 failures)
        assert!(lifecycle.mark_sent(query_id, 12345).is_ok());

        // First failure - should retry (retry_count becomes 1, 1+1 < 3 = true)
        assert!(lifecycle.mark_failed(query_id, "Error".to_string()).is_ok());

        // Should be in Pending state, not removed
        assert_eq!(lifecycle.get_state(query_id), Some(QueryState::Pending));
        assert_eq!(lifecycle.active_count(), 1);

        // Second failure - should retry again (retry_count becomes 2, 2+1 < 3 = true)
        assert!(lifecycle.mark_failed(query_id, "Error".to_string()).is_ok());

        // Should still be in Pending state
        assert_eq!(lifecycle.get_state(query_id), Some(QueryState::Pending));
        assert_eq!(lifecycle.active_count(), 1);

        // Third failure - should not retry (retry_count becomes 3, 3+1 < 3 = false)
        assert!(lifecycle.mark_failed(query_id, "Error".to_string()).is_ok());

        // Should be removed now since we've exceeded the retry limit
        assert_eq!(lifecycle.get_state(query_id), None);
        assert_eq!(lifecycle.active_count(), 0);

        match receiver.try_recv() {
            Ok(result) => assert!(result.is_err()),
            Err(_) => panic!("Expected response"),
        }
    }

    #[test]
    fn test_query_lifecycle_mark_canceled() {
        let lifecycle = QueryLifecycle::new();

        let (query_id, mut receiver) = lifecycle.create_query(Duration::from_secs(10), 3);

        assert!(lifecycle.mark_canceled(query_id).is_ok());

        assert_eq!(lifecycle.active_count(), 0);

        match receiver.try_recv() {
            Ok(result) => assert!(result.is_err()),
            Err(_) => panic!("Expected response"),
        }
    }

    #[test]
    fn test_query_lifecycle_statistics() {
        let lifecycle = QueryLifecycle::new();

        let stats = lifecycle.statistics();
        assert_eq!(stats.total_queries, 0);

        let (query_id, _receiver) = lifecycle.create_query(Duration::from_secs(10), 3);
        assert!(lifecycle.mark_sent(query_id, 12345).is_ok());
        assert!(lifecycle.mark_completed(12345, Bytes::new()).is_ok());

        let stats = lifecycle.statistics();
        assert_eq!(stats.total_queries, 1);
        assert_eq!(stats.successful_queries, 1);
    }

    #[test]
    fn test_query_lifecycle_clear() {
        let lifecycle = QueryLifecycle::new();

        let (query_id, _receiver) = lifecycle.create_query(Duration::from_secs(10), 3);
        assert!(lifecycle.mark_sent(query_id, 12345).is_ok());

        assert_eq!(lifecycle.active_count(), 1);

        lifecycle.clear();

        assert_eq!(lifecycle.active_count(), 0);
    }
}
