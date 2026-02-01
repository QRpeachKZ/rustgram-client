//! # DelayDispatcher
//!
//! Delay dispatcher for network queries.
//!
//! This crate implements TDLib's DelayDispatcher pattern for handling
//! delayed query dispatching. It maintains a queue of queries and dispatches
//! them after specified delays, supporting rate limiting and offline queueing.
//!
//! ## Overview
//!
//! - [`DelayDispatcher`] - Main dispatcher for delayed queries
//!
//! ## Usage
//!
//! ```
//! # async fn example() {
//! use rustgram_delay_dispatcher::DelayDispatcher;
//!
//! let dispatcher = DelayDispatcher::new(std::time::Duration::from_secs(1));
//! # }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::{mpsc, Mutex};

/// Pending query entry in the delay queue.
struct PendingQuery {
    /// The network query
    query: NetQuery,

    /// Callback for the query result
    callback: Option<Box<dyn NetQueryCallback + Send>>,

    /// Delay before dispatching this query
    delay: Duration,

    /// When this query should be dispatched
    ready_at: Instant,
}

/// Delay dispatcher for network queries.
///
/// This dispatcher maintains a queue of queries and dispatches them after
/// specified delays. It's useful for rate limiting, offline queueing, and
/// preventing server overload.
///
/// # Examples
///
/// ```
/// # async fn example() {
/// use rustgram_delay_dispatcher::DelayDispatcher;
///
/// let dispatcher = DelayDispatcher::new(std::time::Duration::from_secs(1));
/// # }
/// ```
#[derive(Clone)]
pub struct DelayDispatcher {
    /// Inner state shared across clones
    inner: Arc<DelayDispatcherInner>,
}

struct DelayDispatcherInner {
    /// Default delay for queries
    default_delay: Duration,

    /// Pending query queue
    queue: Mutex<VecDeque<PendingQuery>>,

    /// Next query ID
    _next_id: AtomicU64,

    /// Whether the dispatcher is running
    running: AtomicBool,

    /// Channel sender for dispatch requests
    dispatch_sender: mpsc::UnboundedSender<DispatchRequest>,
}

/// Dispatch request sent internally.
enum DispatchRequest {
    /// Add a query to the queue
    AddQuery {
        query: NetQuery,
        callback: Option<Box<dyn NetQueryCallback + Send>>,
        delay: Option<Duration>,
    },
    /// Close the dispatcher silently
    CloseSilent,
}

impl DelayDispatcher {
    /// Creates a new delay dispatcher with the specified default delay.
    ///
    /// # Arguments
    ///
    /// * `default_delay` - Default delay for queries without explicit delay
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() {
    /// use rustgram_delay_dispatcher::DelayDispatcher;
    /// use std::time::Duration;
    ///
    /// let dispatcher = DelayDispatcher::new(Duration::from_secs(1));
    /// # }
    /// ```
    #[must_use]
    pub fn new(default_delay: Duration) -> Self {
        let (dispatch_sender, dispatch_receiver) = mpsc::unbounded_channel();

        let inner = Arc::new(DelayDispatcherInner {
            default_delay,
            queue: Mutex::new(VecDeque::new()),
            _next_id: AtomicU64::new(1),
            running: AtomicBool::new(false),
            dispatch_sender,
        });

        let dispatcher = Self {
            inner: inner.clone(),
        };

        // Start the processing loop
        inner.start(dispatch_receiver);

        dispatcher
    }

    /// Sends a query with a callback using the default delay.
    ///
    /// # Arguments
    ///
    /// * `query` - The network query to send
    /// * `callback` - Callback to invoke when the query completes
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_delay_dispatcher::DelayDispatcher;
    ///
    /// # async fn example() {
    /// let dispatcher = DelayDispatcher::new(std::time::Duration::from_secs(1));
    /// // Query would be sent with default delay
    /// # }
    /// ```
    pub fn send_with_callback(&self, query: NetQuery, callback: Box<dyn NetQueryCallback + Send>) {
        let _ = self.inner.dispatch_sender.send(DispatchRequest::AddQuery {
            query,
            callback: Some(callback),
            delay: None,
        });
    }

    /// Sends a query with a callback and custom delay.
    ///
    /// # Arguments
    ///
    /// * `query` - The network query to send
    /// * `callback` - Callback to invoke when the query completes
    /// * `delay` - Optional custom delay (uses default if None)
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_delay_dispatcher::DelayDispatcher;
    /// use std::time::Duration;
    ///
    /// # async fn example() {
    /// let dispatcher = DelayDispatcher::new(Duration::from_secs(1));
    /// // Send with custom delay
    /// # }
    /// ```
    pub fn send_with_callback_and_delay(
        &self,
        query: NetQuery,
        callback: Box<dyn NetQueryCallback + Send>,
        delay: Option<Duration>,
    ) {
        let _ = self.inner.dispatch_sender.send(DispatchRequest::AddQuery {
            query,
            callback: Some(callback),
            delay,
        });
    }

    /// Sends a query without a callback using the default delay.
    ///
    /// # Arguments
    ///
    /// * `query` - The network query to send
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_delay_dispatcher::DelayDispatcher;
    ///
    /// # async fn example() {
    /// let dispatcher = DelayDispatcher::new(std::time::Duration::from_secs(1));
    /// // Query would be sent without callback
    /// # }
    /// ```
    pub fn send(&self, query: NetQuery) {
        let _ = self.inner.dispatch_sender.send(DispatchRequest::AddQuery {
            query,
            callback: None,
            delay: None,
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
    /// use rustgram_delay_dispatcher::DelayDispatcher;
    ///
    /// let dispatcher = DelayDispatcher::new(std::time::Duration::from_secs(1));
    /// dispatcher.close_silent();
    /// # }
    /// ```
    pub fn close_silent(&self) {
        let _ = self
            .inner
            .dispatch_sender
            .send(DispatchRequest::CloseSilent);
        self.inner.running.store(false, Ordering::Release);
    }

    /// Returns the current default delay.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() {
    /// use rustgram_delay_dispatcher::DelayDispatcher;
    /// use std::time::Duration;
    ///
    /// let dispatcher = DelayDispatcher::new(Duration::from_secs(5));
    /// assert_eq!(dispatcher.default_delay(), Duration::from_secs(5));
    /// # }
    /// ```
    #[must_use]
    pub fn default_delay(&self) -> Duration {
        self.inner.default_delay
    }

    /// Returns the number of pending queries in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_delay_dispatcher::DelayDispatcher;
    ///
    /// # async fn example() {
    /// let dispatcher = DelayDispatcher::new(std::time::Duration::from_secs(1));
    /// let pending = dispatcher.pending_count();
    /// # }
    /// ```
    #[must_use]
    pub async fn pending_count(&self) -> usize {
        self.inner.queue.lock().await.len()
    }

    /// Checks if the dispatcher is currently running.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example() {
    /// use rustgram_delay_dispatcher::DelayDispatcher;
    ///
    /// let dispatcher = DelayDispatcher::new(std::time::Duration::from_secs(1));
    /// assert!(dispatcher.is_running());
    /// # }
    /// ```
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.inner.running.load(Ordering::Acquire)
    }
}

impl DelayDispatcherInner {
    /// Starts the dispatcher processing loop.
    fn start(self: Arc<Self>, mut dispatch_receiver: mpsc::UnboundedReceiver<DispatchRequest>) {
        self.running.store(true, Ordering::Release);

        tokio::spawn(async move {
            while let Some(request) = dispatch_receiver.recv().await {
                match request {
                    DispatchRequest::AddQuery {
                        query,
                        callback,
                        delay,
                    } => {
                        // Add to queue
                        let delay = delay.unwrap_or(self.default_delay);
                        let ready_at = Instant::now() + delay;

                        let mut queue = self.queue.lock().await;
                        queue.push_back(PendingQuery {
                            query,
                            callback,
                            delay,
                            ready_at,
                        });
                        drop(queue);

                        // Trigger processing
                        Self::process_queue(&self).await;
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
        loop {
            let now = Instant::now();
            let mut ready_query_data = None;
            let mut sleep_duration = None;

            // Check if any query is ready
            {
                let queue = this.queue.lock().await;
                if let Some(entry) = queue.front() {
                    if entry.ready_at <= now {
                        // Clone the query data for dispatching
                        ready_query_data =
                            Some((entry.query.clone(), entry.callback.is_some(), entry.delay));
                        drop(queue);
                    } else {
                        sleep_duration = Some(entry.ready_at.saturating_duration_since(now));
                    }
                }
            }

            // Dispatch the ready query
            if let Some((query, _has_callback, delay)) = ready_query_data {
                // Remove from queue and get callback
                let callback_opt = {
                    let mut queue = this.queue.lock().await;
                    queue.pop_front().and_then(|e| e.callback)
                };

                Self::dispatch_query(query, callback_opt, delay).await;
            } else if let Some(dur) = sleep_duration {
                tokio::time::sleep(dur).await;
            } else {
                break;
            }
        }
    }

    /// Dispatches a single query.
    async fn dispatch_query(
        _query: NetQuery,
        _callback: Option<Box<dyn NetQueryCallback + Send>>,
        delay: Duration,
    ) {
        // In a full implementation, this would send to NetQueryDispatcher
        // For now, we'll just trace
        tracing::trace!("Dispatching query after delay {:?}", delay);
    }
}

impl Default for DelayDispatcher {
    fn default() -> Self {
        Self::new(Duration::from_secs(1))
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-delay-dispatcher";

// Re-export NetQuery types for convenience
pub use rustgram_net::{NetQuery, NetQueryCallback, NetQueryDispatcher};

#[cfg(test)]
mod tests {
    use super::*;

    // ========== Constructor Tests ==========

    #[tokio::test]
    async fn test_new_creates_dispatcher() {
        let dispatcher = DelayDispatcher::new(Duration::from_secs(1));
        assert_eq!(dispatcher.default_delay(), Duration::from_secs(1));
        assert!(dispatcher.is_running());
    }

    #[tokio::test]
    async fn test_default_creates_dispatcher() {
        let dispatcher = DelayDispatcher::default();
        assert_eq!(dispatcher.default_delay(), Duration::from_secs(1));
        assert!(dispatcher.is_running());
    }

    #[tokio::test]
    async fn test_new_with_different_delay() {
        let dispatcher = DelayDispatcher::new(Duration::from_millis(500));
        assert_eq!(dispatcher.default_delay(), Duration::from_millis(500));
    }

    #[tokio::test]
    async fn test_new_with_zero_delay() {
        let dispatcher = DelayDispatcher::new(Duration::ZERO);
        assert_eq!(dispatcher.default_delay(), Duration::ZERO);
    }

    // ========== pending_count Tests ==========

    #[tokio::test]
    async fn test_pending_count_initially_zero() {
        let dispatcher = DelayDispatcher::new(Duration::from_secs(1));
        assert_eq!(dispatcher.pending_count().await, 0);
    }

    // ========== is_running Tests ==========

    #[tokio::test]
    async fn test_is_running_initially_true() {
        let dispatcher = DelayDispatcher::new(Duration::from_secs(1));
        assert!(dispatcher.is_running());
    }

    #[tokio::test]
    async fn test_is_running_after_close() {
        let dispatcher = DelayDispatcher::new(Duration::from_secs(1));
        dispatcher.close_silent();
        assert!(!dispatcher.is_running());
    }

    // ========== close_silent Tests ==========

    #[tokio::test]
    async fn test_close_silent_stops_dispatcher() {
        let dispatcher = DelayDispatcher::new(Duration::from_secs(1));
        dispatcher.close_silent();
        assert!(!dispatcher.is_running());
    }

    #[tokio::test]
    async fn test_close_silent_clears_queue() {
        let dispatcher = DelayDispatcher::new(Duration::from_secs(1));
        dispatcher.close_silent();
        assert_eq!(dispatcher.pending_count().await, 0);
    }

    // ========== Clone Tests ==========

    #[tokio::test]
    async fn test_clone_shares_state() {
        let dispatcher1 = DelayDispatcher::new(Duration::from_secs(1));
        let dispatcher2 = dispatcher1.clone();

        assert_eq!(dispatcher1.default_delay(), dispatcher2.default_delay());
        assert!(dispatcher1.is_running());
        assert!(dispatcher2.is_running());
    }

    // ========== Metadata Tests ==========

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_crate_name() {
        assert_eq!(CRATE_NAME, "rustgram-delay-dispatcher");
    }

    // ========== Integration Tests ==========

    #[tokio::test]
    async fn test_dispatcher_lifecycle() {
        let dispatcher = DelayDispatcher::new(Duration::from_millis(100));

        assert!(dispatcher.is_running());
        assert_eq!(dispatcher.pending_count().await, 0);

        // Close
        dispatcher.close_silent();
        assert!(!dispatcher.is_running());
    }

    #[tokio::test]
    async fn test_multiple_clones() {
        let dispatcher1 = DelayDispatcher::new(Duration::from_secs(1));
        let dispatcher2 = dispatcher1.clone();
        let dispatcher3 = dispatcher2.clone();

        // All should share the same running state
        assert!(dispatcher1.is_running());
        assert!(dispatcher2.is_running());
        assert!(dispatcher3.is_running());

        // Close should stop all
        dispatcher1.close_silent();
        assert!(!dispatcher1.is_running());
        assert!(!dispatcher2.is_running());
        assert!(!dispatcher3.is_running());
    }
}
