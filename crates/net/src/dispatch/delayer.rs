// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Network query delayer.
//!
//! This module implements query delaying for handling rate limits and server responses.

use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use tokio::sync::mpsc;
use tokio::time::sleep;

use crate::query::{NetQuery, NetQueryId};

/// Delay configuration.
#[derive(Debug, Clone, Copy)]
pub struct DelayConfig {
    /// Base delay between retries
    pub base_delay: Duration,

    /// Maximum delay
    pub max_delay: Duration,

    /// Delay multiplier (exponential backoff)
    pub delay_multiplier: f64,

    /// Maximum retry count
    pub max_retries: u32,
}

impl Default for DelayConfig {
    fn default() -> Self {
        Self {
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
            delay_multiplier: 2.0,
            max_retries: 5,
        }
    }
}

/// Delay entry for a query.
struct DelayEntry {
    /// Query
    query: NetQuery,

    /// Current retry count
    retry_count: u32,

    /// Next retry time
    next_retry: Instant,

    /// Current delay
    current_delay: Duration,
}

/// Network query delayer.
///
/// Handles delayed sending of queries for rate limiting and retries.
pub struct NetQueryDelayer {
    /// Configuration
    config: DelayConfig,

    /// Delayed queries (query_id -> DelayEntry)
    delayed: Mutex<HashMap<NetQueryId, DelayEntry>>,

    /// Next query ID
    next_id: AtomicU64,

    /// Output sender
    output_sender: mpsc::UnboundedSender<NetQuery>,
}

impl NetQueryDelayer {
    /// Creates a new query delayer.
    pub fn new(config: DelayConfig) -> (Self, mpsc::UnboundedReceiver<NetQuery>) {
        let (output_sender, output_receiver) = mpsc::unbounded_channel();

        let delayer = Self {
            config,
            delayed: Mutex::new(HashMap::new()),
            next_id: AtomicU64::new(1),
            output_sender,
        };

        (delayer, output_receiver)
    }

    /// Adds a query to be delayed.
    pub fn add_query(&self, query: NetQuery) {
        let entry = DelayEntry {
            query,
            retry_count: 0,
            next_retry: Instant::now(),
            current_delay: self.config.base_delay,
        };

        let query_id = entry.query.id();
        self.delayed.lock().insert(query_id, entry);

        // Try to send immediately
        self.try_send_query(query_id);
    }

    /// Retries a query after a delay.
    pub fn retry_query(&self, query: NetQuery) {
        let mut delayed = self.delayed.lock();

        if let Some(entry) = delayed.get_mut(&query.id()) {
            entry.retry_count += 1;

            // Calculate new delay with exponential backoff
            let new_delay = Duration::from_secs_f64(
                entry.current_delay.as_secs_f64() * self.config.delay_multiplier,
            );
            entry.current_delay = new_delay.min(self.config.max_delay);
            entry.next_retry = Instant::now() + entry.current_delay;

            // Update query
            entry.query = query.clone();

            // Schedule retry
            let _query_id = query.id();
            let delay = entry.current_delay;
            let sender = self.output_sender.clone();

            drop(delayed);

            tokio::spawn(async move {
                sleep(delay).await;
                let _ = sender.send(query);
            });
        }
    }

    /// Removes a query from the delay queue (completed or failed permanently).
    pub fn remove_query(&self, query_id: NetQueryId) -> Option<NetQuery> {
        self.delayed.lock().remove(&query_id).map(|e| e.query)
    }

    /// Tries to send a query immediately if it's ready.
    fn try_send_query(&self, query_id: NetQueryId) {
        let delayed = self.delayed.lock();

        if let Some(entry) = delayed.get(&query_id) {
            if entry.next_retry <= Instant::now() {
                let query = entry.query.clone();
                drop(delayed);

                let _ = self.output_sender.send(query);
            }
        }
    }

    /// Processes all ready queries.
    pub fn process_ready(&self) {
        let ready_queries: Vec<NetQuery> = self
            .delayed
            .lock()
            .values()
            .filter(|e| e.next_retry <= Instant::now())
            .map(|e| e.query.clone())
            .collect();

        for query in ready_queries {
            let _ = self.output_sender.send(query);
        }
    }

    /// Returns the number of delayed queries.
    pub fn delayed_count(&self) -> usize {
        self.delayed.lock().len()
    }

    /// Clears all delayed queries.
    pub fn clear(&self) {
        self.delayed.lock().clear();
    }
}

impl Default for NetQueryDelayer {
    fn default() -> Self {
        let (delayer, _) = Self::new(DelayConfig::default());
        delayer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dc::DcId;
    use crate::query::{AuthFlag, GzipFlag, NetQueryType};
    use bytes::Bytes;

    #[test]
    fn test_delay_config_default() {
        let config = DelayConfig::default();
        assert_eq!(config.base_delay, Duration::from_secs(1));
        assert_eq!(config.max_delay, Duration::from_secs(60));
        assert_eq!(config.delay_multiplier, 2.0);
        assert_eq!(config.max_retries, 5);
    }

    #[test]
    fn test_net_query_delayer_new() {
        let (delayer, _receiver) = NetQueryDelayer::new(DelayConfig::default());

        assert_eq!(delayer.delayed_count(), 0);
    }

    #[test]
    fn test_net_query_delayer_add_query() {
        let (delayer, mut receiver) = NetQueryDelayer::new(DelayConfig::default());

        let query = NetQuery::new(
            1,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        );

        delayer.add_query(query.clone());

        assert_eq!(delayer.delayed_count(), 1);

        // Query should be sent immediately
        let received = receiver.try_recv().unwrap();
        assert_eq!(received.id(), query.id());
    }

    #[test]
    fn test_net_query_delayer_remove_query() {
        let (delayer, _receiver) = NetQueryDelayer::new(DelayConfig::default());

        let query = NetQuery::new(
            1,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        );

        delayer.add_query(query.clone());

        let removed = delayer.remove_query(query.id());
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id(), query.id());
        assert_eq!(delayer.delayed_count(), 0);
    }

    #[test]
    fn test_net_query_delayer_clear() {
        let (delayer, _receiver) = NetQueryDelayer::new(DelayConfig::default());

        let query = NetQuery::new(
            1,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        );

        delayer.add_query(query);
        delayer.clear();

        assert_eq!(delayer.delayed_count(), 0);
    }
}
