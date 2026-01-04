// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Sequence dispatcher for maintaining message order.
//!
//! This module implements sequence-based query dispatching to maintain message ordering.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use parking_lot::Mutex;
use tokio::sync::{mpsc, oneshot};

use crate::dc::DcId;
use crate::query::{NetQuery, NetQueryId};

/// Sequence configuration.
#[derive(Debug, Clone, Copy)]
pub struct SequenceConfig {
    /// Maximum queue size per chain
    pub max_queue_size: usize,

    /// Timeout for waiting in sequence
    pub queue_timeout: std::time::Duration,

    /// Whether to drop old queries when queue is full
    pub drop_on_full: bool,
}

impl Default for SequenceConfig {
    fn default() -> Self {
        Self {
            max_queue_size: 100,
            queue_timeout: std::time::Duration::from_secs(30),
            drop_on_full: false,
        }
    }
}

/// Queued query with response channel.
struct QueuedQuery {
    /// Query
    query: NetQuery,

    /// Response sender
    response_sender: oneshot::Sender<Result<Vec<u8>, String>>,

    /// Enqueue time
    enqueued_at: std::time::Instant,
}

/// Query chain for sequence dispatching.
struct QueryChain {
    /// Chain ID
    chain_id: u64,

    /// Pending queries
    pending: VecDeque<QueuedQuery>,

    /// Currently executing query ID
    current_query: Option<NetQueryId>,

    /// Next sequence number
    next_seq: u32,
}

impl QueryChain {
    fn new(chain_id: u64) -> Self {
        Self {
            chain_id,
            pending: VecDeque::new(),
            current_query: None,
            next_seq: 0,
        }
    }
}

/// Sequence dispatcher.
///
/// Maintains query ordering by sequence numbers within chains.
pub struct SequenceDispatcher {
    /// Configuration
    config: SequenceConfig,

    /// Query chains (chain_id -> QueryChain)
    chains: Mutex<std::collections::HashMap<u64, Arc<Mutex<QueryChain>>>>,

    /// Next chain ID
    next_chain_id: AtomicU64,

    /// Worker sender
    worker_sender: mpsc::UnboundedSender<(u64, NetQuery, oneshot::Sender<Result<Vec<u8>, String>>)>,
}

impl SequenceDispatcher {
    /// Creates a new sequence dispatcher.
    pub fn new(config: SequenceConfig) -> Self {
        let (worker_sender, worker_receiver) = mpsc::unbounded_channel();

        let dispatcher = Self {
            config,
            chains: Mutex::new(std::collections::HashMap::new()),
            next_chain_id: AtomicU64::new(1),
            worker_sender,
        };

        // Start worker task
        dispatcher.start_worker(worker_receiver);

        dispatcher
    }

    /// Adds a query to a sequence chain.
    pub fn add_to_chain(
        &self,
        chain_id: u64,
        query: NetQuery,
    ) -> oneshot::Receiver<Result<Vec<u8>, String>> {
        let (response_sender, response_receiver) = oneshot::channel();

        let queued = QueuedQuery {
            query,
            response_sender,
            enqueued_at: std::time::Instant::now(),
        };

        let chains = self.chains.lock();
        if let Some(chain) = chains.get(&chain_id) {
            let mut chain_guard = chain.lock();

            if chain_guard.pending.len() >= self.config.max_queue_size && self.config.drop_on_full {
                // Drop oldest query
                if let Some(dropped) = chain_guard.pending.pop_front() {
                    let _ = dropped
                        .response_sender
                        .send(Err("Queue full, query dropped".to_string()));
                }
            }

            chain_guard.pending.push_back(queued);

            // Try to process next query
            let chain_for_processing = chain.clone();
            drop(chain_guard);
            self.process_chain(chain_for_processing);
        } else {
            // Chain doesn't exist, create it
            drop(chains);
            self.create_chain(chain_id, queued);
        }

        response_receiver
    }

    /// Creates a new chain and processes the first query.
    fn create_chain(&self, chain_id: u64, queued: QueuedQuery) {
        let chain = Arc::new(Mutex::new(QueryChain::new(chain_id)));
        self.chains.lock().insert(chain_id, chain.clone());

        {
            let mut chain_guard = chain.lock();
            chain_guard.pending.push_back(queued);
        }
        self.process_chain(chain);
    }

    /// Processes the next query in a chain.
    fn process_chain(&self, chain: Arc<Mutex<QueryChain>>) {
        let mut chain_guard = chain.lock();

        // If there's a current query, wait for it to complete
        if chain_guard.current_query.is_some() {
            return;
        }

        // Get next query
        if let Some(queued) = chain_guard.pending.pop_front() {
            chain_guard.current_query = Some(queued.query.id());

            let query = queued.query;
            let response_sender = queued.response_sender;
            let chain_id = chain_guard.chain_id;

            drop(chain_guard);

            // Send to worker
            let _ = self.worker_sender.send((chain_id, query, response_sender));
        }
    }

    /// Marks a query as completed and processes the next one.
    pub fn on_query_complete(
        &self,
        chain_id: u64,
        query_id: NetQueryId,
        _result: Result<Vec<u8>, String>,
    ) {
        if let Some(chain) = {
            let chains = self.chains.lock();
            chains.get(&chain_id).cloned()
        } {
            let mut chain_guard = chain.lock();

            if chain_guard.current_query == Some(query_id) {
                chain_guard.current_query = None;
                chain_guard.next_seq += 1;

                // Process next query
                drop(chain_guard);
                self.process_chain(chain);
            }
        }
    }

    /// Starts the worker task.
    fn start_worker(
        &self,
        mut receiver: mpsc::UnboundedReceiver<(
            u64,
            NetQuery,
            oneshot::Sender<Result<Vec<u8>, String>>,
        )>,
    ) {
        tokio::spawn(async move {
            while let Some((_chain_id, _query, response_sender)) = receiver.recv().await {
                // In a real implementation, this would send the query
                // through the actual network layer

                // For now, simulate a response
                let _ = tokio::time::sleep(std::time::Duration::from_millis(10)).await;

                // Send result (simulated)
                let _ = response_sender.send(Ok(vec![]));
            }
        });
    }

    /// Creates a new chain ID.
    pub fn create_chain_id(&self) -> u64 {
        self.next_chain_id.fetch_add(1, Ordering::Relaxed)
    }

    /// Returns the number of active chains.
    pub fn active_chain_count(&self) -> usize {
        self.chains.lock().len()
    }

    /// Removes a chain.
    pub fn remove_chain(&self, chain_id: u64) {
        self.chains.lock().remove(&chain_id);
    }

    /// Clears all chains.
    pub fn clear(&self) {
        self.chains.lock().clear();
    }
}

impl Default for SequenceDispatcher {
    fn default() -> Self {
        Self::new(SequenceConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::{AuthFlag, GzipFlag, NetQueryType};
    use bytes::Bytes;

    fn create_test_query(id: u64) -> NetQuery {
        NetQuery::new(
            id,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        )
    }

    #[test]
    fn test_sequence_config_default() {
        let config = SequenceConfig::default();
        assert_eq!(config.max_queue_size, 100);
        assert_eq!(config.queue_timeout, std::time::Duration::from_secs(30));
        assert!(!config.drop_on_full);
    }

    #[tokio::test]
    async fn test_sequence_dispatcher_new() {
        let dispatcher = SequenceDispatcher::new(SequenceConfig::default());

        assert_eq!(dispatcher.active_chain_count(), 0);
    }

    #[tokio::test]
    async fn test_sequence_dispatcher_create_chain_id() {
        let dispatcher = SequenceDispatcher::new(SequenceConfig::default());

        let id1 = dispatcher.create_chain_id();
        let id2 = dispatcher.create_chain_id();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[tokio::test]
    async fn test_sequence_dispatcher_add_to_chain() {
        let dispatcher = SequenceDispatcher::new(SequenceConfig::default());

        let query = create_test_query(1);
        let _receiver = dispatcher.add_to_chain(42, query);

        assert_eq!(dispatcher.active_chain_count(), 1);
    }

    #[tokio::test]
    async fn test_sequence_dispatcher_remove_chain() {
        let dispatcher = SequenceDispatcher::new(SequenceConfig::default());

        let query = create_test_query(1);
        let _receiver = dispatcher.add_to_chain(42, query);

        dispatcher.remove_chain(42);

        assert_eq!(dispatcher.active_chain_count(), 0);
    }

    #[tokio::test]
    async fn test_sequence_dispatcher_clear() {
        let dispatcher = SequenceDispatcher::new(SequenceConfig::default());

        let query1 = create_test_query(1);
        let query2 = create_test_query(2);

        dispatcher.add_to_chain(1, query1);
        dispatcher.add_to_chain(2, query2);

        dispatcher.clear();

        assert_eq!(dispatcher.active_chain_count(), 0);
    }

    #[test]
    fn test_query_chain_new() {
        let chain = QueryChain::new(123);

        assert_eq!(chain.chain_id, 123);
        assert!(chain.current_query.is_none());
        assert!(chain.pending.is_empty());
        assert_eq!(chain.next_seq, 0);
    }
}
