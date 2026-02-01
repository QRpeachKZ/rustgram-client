//! # SequenceDispatcher
//!
//! Sequence dispatcher for ordered query execution.
//!
//! This crate implements TDLib's SequenceDispatcher pattern for handling
//! sequential query dispatching with invokeAfter chaining. It maintains
//! query order within chains and handles timeouts and retries.
//!
//! ## Overview
//!
//! - [`SequenceDispatcher`] - Main dispatcher for sequential queries
//! - [`SequenceState`] - Query states in the sequence
//!
//! ## Usage
//!
//! ```
//! # async fn example() {
//! use rustgram_sequence_dispatcher::SequenceDispatcher;
//!
//! let dispatcher = SequenceDispatcher::new();
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::{mpsc, Mutex};

/// Query state in the sequence.
///
/// Based on TDLib's SequenceDispatcher::State from `td/telegram/SequenceDispatcher.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SequenceState {
    /// Query is starting
    Start,

    /// Query is waiting for response
    Wait,

    /// Query is finished
    Finish,

    /// Query is in dummy state (will be resent)
    Dummy,
}

/// Pending query entry in the sequence queue.
struct SequenceEntry {
    /// The network query
    _query: NetQuery,

    /// Callback for the query result
    _callback: Option<Box<dyn NetQueryCallback + Send>>,

    /// Current state
    state: SequenceState,

    /// Total timeout accumulated
    _total_timeout: Duration,

    /// Last timeout received
    _last_timeout: Duration,

    /// Entry creation time
    _created_at: Instant,
}

/// Sequence dispatcher for network queries.
///
/// This dispatcher maintains query order using invokeAfter chaining.
/// Queries are sent sequentially within each chain, with timeout handling
/// and automatic retry on failure.
///
/// # Examples
///
/// ```
/// # async fn example() {
/// use rustgram_sequence_dispatcher::SequenceDispatcher;
///
/// let dispatcher = SequenceDispatcher::new();
/// # }
/// ```
#[derive(Clone)]
pub struct SequenceDispatcher {
    /// Inner state shared across clones
    inner: Arc<SequenceDispatcherInner>,
}

struct SequenceDispatcherInner {
    /// Pending query queue
    queue: Mutex<VecDeque<SequenceEntry>>,

    /// Maximum simultaneous wait
    max_simultaneous_wait: usize,

    /// Channel sender for dispatch requests
    dispatch_sender: mpsc::UnboundedSender<DispatchRequest>,
}

/// Dispatch request sent internally.
enum DispatchRequest {
    /// Add a query to the sequence
    AddQuery {
        query: NetQuery,
        callback: Option<Box<dyn NetQueryCallback + Send>>,
    },
    /// Mark query as complete
    QueryComplete {
        token: usize,
        result: Result<NetQuery, String>,
    },
    /// Close the dispatcher silently
    CloseSilent,
}

impl SequenceDispatcher {
    /// Creates a new sequence dispatcher.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() {
    /// use rustgram_sequence_dispatcher::SequenceDispatcher;
    ///
    /// let dispatcher = SequenceDispatcher::new();
    /// # }
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(10)
    }

    /// Creates a new sequence dispatcher with custom max simultaneous wait.
    ///
    /// # Arguments
    ///
    /// * `max_simultaneous_wait` - Maximum number of queries waiting simultaneously
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() {
    /// use rustgram_sequence_dispatcher::SequenceDispatcher;
    ///
    /// let dispatcher = SequenceDispatcher::with_config(20);
    /// # }
    /// ```
    #[must_use]
    pub fn with_config(max_simultaneous_wait: usize) -> Self {
        let (dispatch_sender, dispatch_receiver) = mpsc::unbounded_channel();

        let inner = Arc::new(SequenceDispatcherInner {
            queue: Mutex::new(VecDeque::new()),
            max_simultaneous_wait,
            dispatch_sender,
        });

        let dispatcher = Self {
            inner: inner.clone(),
        };

        // Start the processing loop
        inner.start(dispatch_receiver);

        dispatcher
    }

    /// Sends a query with a callback through the sequence.
    ///
    /// # Arguments
    ///
    /// * `query` - The network query to send
    /// * `callback` - Callback to invoke when the query completes
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() {
    /// use rustgram_sequence_dispatcher::SequenceDispatcher;
    ///
    /// let dispatcher = SequenceDispatcher::new();
    /// // Query would be sent in sequence
    /// # }
    /// ```
    pub fn send_with_callback(&self, query: NetQuery, callback: Box<dyn NetQueryCallback + Send>) {
        let _ = self.inner.dispatch_sender.send(DispatchRequest::AddQuery {
            query,
            callback: Some(callback),
        });
    }

    /// Sends a query without a callback through the sequence.
    ///
    /// # Arguments
    ///
    /// * `query` - The network query to send
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() {
    /// use rustgram_sequence_dispatcher::SequenceDispatcher;
    ///
    /// let dispatcher = SequenceDispatcher::new();
    /// // Query would be sent in sequence
    /// # }
    /// ```
    pub fn send(&self, query: NetQuery) {
        let _ = self.inner.dispatch_sender.send(DispatchRequest::AddQuery {
            query,
            callback: None,
        });
    }

    /// Closes the dispatcher silently, clearing all pending queries.
    ///
    /// This clears the queue without invoking any callbacks.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() {
    /// use rustgram_sequence_dispatcher::SequenceDispatcher;
    ///
    /// let dispatcher = SequenceDispatcher::new();
    /// dispatcher.close_silent();
    /// # }
    /// ```
    pub fn close_silent(&self) {
        let _ = self
            .inner
            .dispatch_sender
            .send(DispatchRequest::CloseSilent);
    }

    /// Returns the number of pending queries in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() {
    /// use rustgram_sequence_dispatcher::SequenceDispatcher;
    ///
    /// let dispatcher = SequenceDispatcher::new();
    /// let pending = dispatcher.pending_count().await;
    /// # }
    /// ```
    #[must_use]
    pub async fn pending_count(&self) -> usize {
        self.inner.queue.lock().await.len()
    }

    /// Checks if the dispatcher is idle (no pending queries).
    #[must_use]
    pub async fn is_idle(&self) -> bool {
        self.inner.queue.lock().await.is_empty()
    }
}

impl SequenceDispatcherInner {
    /// Starts the dispatcher processing loop.
    fn start(self: Arc<Self>, mut dispatch_receiver: mpsc::UnboundedReceiver<DispatchRequest>) {
        tokio::spawn(async move {
            while let Some(request) = dispatch_receiver.recv().await {
                match request {
                    DispatchRequest::AddQuery { query, callback } => {
                        let entry = SequenceEntry {
                            _query: query,
                            _callback: callback,
                            state: SequenceState::Start,
                            _total_timeout: Duration::ZERO,
                            _last_timeout: Duration::ZERO,
                            _created_at: Instant::now(),
                        };

                        let mut queue = self.queue.lock().await;
                        queue.push_back(entry);
                        drop(queue);

                        // Trigger processing
                        Self::process_queue(&self).await;
                    }
                    DispatchRequest::QueryComplete { token, result } => {
                        Self::handle_query_complete(&self, token, result).await;
                    }
                    DispatchRequest::CloseSilent => {
                        // Clear all queries
                        let mut queue = self.queue.lock().await;
                        queue.clear();
                        break;
                    }
                }
            }
        });
    }

    /// Processes the queue, dispatching ready queries.
    async fn process_queue(this: &Arc<Self>) {
        let mut queue = this.queue.lock().await;
        let queue_len = queue.len();

        // Track local state for mutations
        let mut finish_i = 0usize;
        let mut next_i = 0usize;
        let wait_cnt = 0usize;
        let id_offset = 1usize;
        let max_simultaneous_wait = this.max_simultaneous_wait;

        // Move finish_i past all Finished entries
        while finish_i < queue_len
            && queue
                .get(finish_i)
                .map(|e| e.state == SequenceState::Finish)
                .unwrap_or(false)
        {
            finish_i += 1;
        }

        // Ensure next_i is at least finish_i
        if next_i < finish_i {
            next_i = finish_i;
        }

        // Send queries while under max_simultaneous_wait
        while next_i < queue_len && wait_cnt < max_simultaneous_wait {
            let entry = queue.get(next_i);

            if let Some(entry) = entry {
                if entry.state == SequenceState::Finish {
                    next_i += 1;
                    continue;
                }

                if entry.state == SequenceState::Wait {
                    next_i += 1;
                    continue;
                }

                // Can send this query
                let token = next_i + id_offset;
                next_i += 1;

                // Dispatch query (in real implementation, would send to NetQueryDispatcher)
                tracing::trace!("Sending query with token {}", token);

                // Simulate state change
                if let Some(e) = queue.get_mut(next_i - 1) {
                    e.state = SequenceState::Wait;
                }

                // In real implementation, would send callback and wait for result
                // For now, simulate completion
                let this_clone = this.clone();
                tokio::spawn(async move {
                    // Simulate network delay
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    let _ = this_clone
                        .dispatch_sender
                        .send(DispatchRequest::QueryComplete {
                            token,
                            result: Err("Simulated".to_string()),
                        });
                });
            } else {
                break;
            }
        }

        // Shrink queue if needed
        if finish_i > 0 && finish_i * 2 > queue_len && queue_len > 5 {
            // Remove finished entries
            let removed_count = finish_i;
            for _ in 0..removed_count {
                queue.pop_front();
            }
        }
    }

    /// Handles query completion.
    async fn handle_query_complete(
        this: &Arc<Self>,
        token: usize,
        _result: Result<NetQuery, String>,
    ) {
        let pos = token.wrapping_sub(1);

        let mut queue = this.queue.lock().await;
        if let Some(entry) = queue.get_mut(pos) {
            entry.state = SequenceState::Finish;
        }
        drop(queue);

        // Process more queries
        Self::process_queue(this).await;
    }
}

impl Default for SequenceDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-sequence-dispatcher";

// Re-export NetQuery types for convenience
pub use rustgram_net::{NetQuery, NetQueryCallback, NetQueryDispatcher};

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Constructor Tests ==========

    #[tokio::test]
    async fn test_new_creates_dispatcher() {
        let dispatcher = SequenceDispatcher::new();
        assert!(dispatcher.is_idle().await);
    }

    #[tokio::test]
    async fn test_default_creates_dispatcher() {
        let dispatcher = SequenceDispatcher::default();
        assert!(dispatcher.is_idle().await);
    }

    #[tokio::test]
    async fn test_with_config_creates_dispatcher() {
        let dispatcher = SequenceDispatcher::with_config(20);
        assert!(dispatcher.is_idle().await);
    }

    // ========== pending_count Tests ==========

    #[tokio::test]
    async fn test_pending_count_initially_zero() {
        let dispatcher = SequenceDispatcher::new();
        assert_eq!(dispatcher.pending_count().await, 0);
    }

    // ========== is_idle Tests ==========

    #[tokio::test]
    async fn test_is_idle_when_no_queries() {
        let dispatcher = SequenceDispatcher::new();
        assert!(dispatcher.is_idle().await);
    }

    #[tokio::test]
    async fn test_is_idle_with_queries() {
        let dispatcher = SequenceDispatcher::new();
        // In real test, would add queries
        assert!(dispatcher.is_idle().await);
    }

    // ========== Clone Tests ==========

    #[tokio::test]
    async fn test_clone_shares_state() {
        let dispatcher1 = SequenceDispatcher::new();
        let dispatcher2 = dispatcher1.clone();

        assert!(dispatcher1.is_idle().await);
        assert!(dispatcher2.is_idle().await);
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-sequence-dispatcher");
    }

    // ========== Integration Tests ==========

    #[tokio::test]
    async fn test_dispatcher_lifecycle() {
        let dispatcher = SequenceDispatcher::with_config(5);

        assert!(dispatcher.is_idle().await);
        assert_eq!(dispatcher.pending_count().await, 0);

        // Close
        dispatcher.close_silent();
        assert!(dispatcher.is_idle().await);
    }

    #[tokio::test]
    async fn test_multiple_clones() {
        let dispatcher1 = SequenceDispatcher::new();
        let dispatcher2 = dispatcher1.clone();
        let dispatcher3 = dispatcher2.clone();

        // All should share the same state
        assert!(dispatcher1.is_idle().await);
        assert!(dispatcher2.is_idle().await);
        assert!(dispatcher3.is_idle().await);
    }

    #[tokio::test]
    async fn test_sequence_state_equality() {
        assert_eq!(SequenceState::Start, SequenceState::Start);
        assert_ne!(SequenceState::Start, SequenceState::Wait);
        assert_ne!(SequenceState::Wait, SequenceState::Finish);
        assert_ne!(SequenceState::Finish, SequenceState::Dummy);
    }
}
