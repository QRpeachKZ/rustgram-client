// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Network actor base trait.
//!
//! This module implements TDLib's NetActor from `td/telegram/net/NetActor.h`.
//!
//! Provides a base trait for actors that handle network queries with callbacks.

use std::sync::Arc;

use thiserror::Error;

use crate::query::{NetQuery, QueryError};

/// Error types for actor operations.
#[derive(Debug, Error)]
pub enum ActorError {
    /// Actor is not initialized
    #[error("Actor not initialized")]
    NotInitialized,

    /// Actor is stopped
    #[error("Actor stopped")]
    Stopped,

    /// Query execution failed
    #[error("Query failed: {0}")]
    QueryFailed(#[from] QueryError),

    /// Parent actor not set
    #[error("Parent actor not set")]
    NoParent,
}

/// Result type for actor operations.
pub type ActorResult<T> = Result<T, ActorError>;

/// Callback trait for network query results in actor context.
///
/// This is a simplified version of NetQueryCallback for use with NetActor.
/// Based on TDLib's NetQueryCallback from `td/telegram/net/NetQuery.h`.
pub trait ActorQueryCallback: Send + Sync {
    /// Called when a query completes successfully.
    ///
    /// # Arguments
    ///
    /// * `query` - The completed query
    fn on_result(&self, query: NetQuery);

    /// Called when a query fails.
    ///
    /// # Arguments
    ///
    /// * `error` - The error that occurred
    fn on_error(&self, error: QueryError);
}

/// Simple callback adapter for closures.
impl<F: Fn(NetQuery) + Send + Sync> ActorQueryCallback for F {
    fn on_result(&self, query: NetQuery) {
        self(query);
    }

    fn on_error(&self, _error: QueryError) {
        // Default: ignore errors
    }
}

/// Actor trait for handling network queries.
///
/// Based on TDLib's NetActor from `td/telegram/net/NetActor.h`.
///
/// # Example
///
/// ```rust
/// use std::sync::Arc;
/// use rustgram_net::net_actor::{NetActor, ActorError, ActorResult, ActorQueryCallback};
/// use rustgram_net::query::NetQuery;
///
/// struct MyActor {
///     parent: Option<Arc<dyn ActorQueryCallback>>,
/// }
///
/// impl NetActor for MyActor {
///     fn parent(&self) -> Option<&Arc<dyn ActorQueryCallback>> {
///         self.parent.as_ref()
///     }
///
///     fn set_parent(&mut self, parent: Arc<dyn ActorQueryCallback>) {
///         self.parent = Some(parent);
///     }
///
///     fn on_result(&self, query: NetQuery) -> ActorResult<()> {
///         // Handle successful query result
///         Ok(())
///     }
///
///     fn on_error(&self, error: rustgram_net::query::QueryError) -> ActorResult<()> {
///         // Handle error
///         Ok(())
///     }
/// }
/// ```
pub trait NetActor {
    /// Gets the parent actor callback.
    fn parent(&self) -> Option<&Arc<dyn ActorQueryCallback>>;

    /// Sets the parent actor callback.
    ///
    /// # Arguments
    ///
    /// * `parent` - The parent actor to receive callbacks
    fn set_parent(&mut self, parent: Arc<dyn ActorQueryCallback>);

    /// Called when a query completes successfully.
    ///
    /// # Arguments
    ///
    /// * `query` - The completed query with result data
    ///
    /// # Returns
    ///
    /// Ok(()) if successful, Err with ActorError if handling failed.
    fn on_result(&self, query: NetQuery) -> ActorResult<()>;

    /// Called when a query fails.
    ///
    /// # Arguments
    ///
    /// * `error` - The error that occurred
    ///
    /// # Returns
    ///
    /// Ok(()) if error was handled, Err with ActorError if handling failed.
    fn on_error(&self, error: QueryError) -> ActorResult<()>;

    /// Called after result processing is complete.
    ///
    /// This is called after `on_result` for cleanup or additional processing.
    ///
    /// # Returns
    ///
    /// Ok(()) if successful, Err with ActorError if handling failed.
    fn on_result_finish(&self) -> ActorResult<()> {
        // Default implementation: do nothing
        Ok(())
    }

    /// Sends a query to be executed.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to send
    ///
    /// # Returns
    ///
    /// Ok(()) if the query was sent, Err with ActorError if sending failed.
    fn send_query(&self, query: NetQuery) -> ActorResult<()> {
        // Default implementation: forward to parent if available
        if let Some(parent) = self.parent() {
            parent.on_result(query);
            Ok(())
        } else {
            Err(ActorError::NoParent)
        }
    }

    /// Handles a query result from the network layer.
    ///
    /// This is a convenience method that calls the appropriate callbacks.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to handle
    ///
    /// # Returns
    ///
    /// Ok(()) if successful, Err with ActorError if handling failed.
    fn handle_query(&self, query: NetQuery) -> ActorResult<()> {
        // Check if query has an error
        if query.is_error() {
            let error = query.error();
            self.on_error(error.clone())?;
            Err(ActorError::QueryFailed(error.clone()))
        } else {
            self.on_result(query)?;
            self.on_result_finish()?;
            Ok(())
        }
    }
}

/// Simple actor implementation for testing.
#[derive(Default)]
pub struct TestActor {
    parent: Option<Arc<dyn ActorQueryCallback>>,
    result_count: Arc<std::sync::atomic::AtomicUsize>,
    error_count: Arc<std::sync::atomic::AtomicUsize>,
}

impl TestActor {
    /// Creates a new test actor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the number of successful results handled.
    pub fn result_count(&self) -> usize {
        self.result_count.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Gets the number of errors handled.
    pub fn error_count(&self) -> usize {
        self.error_count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl NetActor for TestActor {
    fn parent(&self) -> Option<&Arc<dyn ActorQueryCallback>> {
        self.parent.as_ref()
    }

    fn set_parent(&mut self, parent: Arc<dyn ActorQueryCallback>) {
        self.parent = Some(parent);
    }

    fn on_result(&self, _query: NetQuery) -> ActorResult<()> {
        self.result_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    fn on_error(&self, _error: QueryError) -> ActorResult<()> {
        self.error_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}

/// Callback that counts results and errors.
#[derive(Debug)]
pub struct CountingCallback {
    result_count: Arc<std::sync::atomic::AtomicUsize>,
    error_count: Arc<std::sync::atomic::AtomicUsize>,
}

impl CountingCallback {
    /// Creates a new counting callback.
    pub fn new() -> Self {
        Self {
            result_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            error_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    /// Gets the number of results received.
    pub fn result_count(&self) -> usize {
        self.result_count.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Gets the number of errors received.
    pub fn error_count(&self) -> usize {
        self.error_count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Default for CountingCallback {
    fn default() -> Self {
        Self::new()
    }
}

impl ActorQueryCallback for CountingCallback {
    fn on_result(&self, _query: NetQuery) {
        self.result_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn on_error(&self, _error: QueryError) {
        self.error_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dc::DcId;
    use crate::query::{AuthFlag, GzipFlag, NetQueryType};

    fn create_test_query(id: u64) -> NetQuery {
        NetQuery::new(
            id,
            bytes::Bytes::from(vec![1, 2, 3]),
            DcId::main(),
            NetQueryType::Common,
            AuthFlag::On,
            GzipFlag::Off,
            0, // tl_constructor
        )
    }

    #[test]
    fn test_test_actor_handles_result() {
        let actor = TestActor::new();
        let query = create_test_query(1);

        let result = actor.on_result(query);

        assert!(result.is_ok());
        assert_eq!(actor.result_count(), 1);
        assert_eq!(actor.error_count(), 0);
    }

    #[test]
    fn test_test_actor_handles_error() {
        let actor = TestActor::new();
        let error = QueryError::new(500, "Internal error");

        let result = actor.on_error(error);

        assert!(result.is_ok());
        assert_eq!(actor.result_count(), 0);
        assert_eq!(actor.error_count(), 1);
    }

    #[test]
    fn test_test_actor_set_parent() {
        let mut actor = TestActor::new();
        let callback = Arc::new(CountingCallback::new());

        assert!(actor.parent().is_none());

        actor.set_parent(callback);

        assert!(actor.parent().is_some());
    }

    #[test]
    fn test_counting_callback() {
        let callback = CountingCallback::new();
        let query = create_test_query(1);
        let error = QueryError::new(500, "Error");

        callback.on_result(query);
        callback.on_error(error);

        assert_eq!(callback.result_count(), 1);
        assert_eq!(callback.error_count(), 1);
    }

    #[test]
    fn test_send_query_with_parent() {
        let actor = TestActor::new();
        let callback = Arc::new(CountingCallback::new());

        let mut actor_owned = actor;
        actor_owned.set_parent(callback.clone());

        let query = create_test_query(1);
        let result = actor_owned.send_query(query);

        assert!(result.is_ok());
        assert_eq!(callback.result_count(), 1);
    }

    #[test]
    fn test_send_query_without_parent() {
        let actor = TestActor::new();
        let query = create_test_query(1);

        let result = actor.send_query(query);

        assert!(matches!(result, Err(ActorError::NoParent)));
    }

    #[test]
    fn test_on_result_finish_default() {
        let actor = TestActor::new();

        // Default implementation should return Ok
        let result = actor.on_result_finish();

        assert!(result.is_ok());
    }

    #[test]
    fn test_closure_callback() {
        let count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let count_clone = count.clone();

        let callback: Arc<dyn ActorQueryCallback> = Arc::new(move |_query: NetQuery| {
            count_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        });

        let query = create_test_query(1);
        callback.on_result(query);

        assert_eq!(count.load(std::sync::atomic::Ordering::Relaxed), 1);
    }
}
