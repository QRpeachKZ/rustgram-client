// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Client actor for Telegram MTProto client.
//!
//! This module implements TDLib's ClientActor from `td/telegram/ClientActor.h`.
//!
//! # Overview
//!
//! The ClientActor is the central coordinator for all Telegram client operations.
//! It provides:
//! - Network query dispatching through NetQueryDispatcher
//! - Request/response correlation
//! - Update callbacks for real-time events
//! - Manager coordination

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicI64, AtomicI32, AtomicU64, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{mpsc, oneshot, RwLock};

use rustgram_net::{
    AuthFlag, DcId, GzipFlag, NetQuery, NetQueryCallback, NetQueryDispatcher, NetQueryId,
    NetQueryType, QueryError,
};
use rustgram_types::TlHelper;
use bytes::BytesMut;

/// Error type for ClientActor operations.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ClientActorError {
    /// Request execution failed.
    #[error("request execution failed: {0}")]
    ExecutionFailed(String),

    /// Invalid request data.
    #[error("invalid request data")]
    InvalidRequest,

    /// Client not initialized.
    #[error("client not initialized")]
    NotInitialized,

    /// Request timed out.
    #[error("request timed out")]
    Timeout,

    /// Network error.
    #[error("network error: {0}")]
    NetworkError(String),

    /// Serialization error.
    #[error("serialization error: {0}")]
    SerializationError(String),

    /// Deserialization error.
    #[error("deserialization error: {0}")]
    DeserializationError(String),
}

/// Result type for ClientActor operations.
pub type Result<T> = std::result::Result<T, ClientActorError>;

/// Request ID type.
pub type RequestId = i64;

/// TL constructor for a request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TlConstructor(pub i32);

impl TlConstructor {
    /// Creates a new TL constructor.
    #[inline]
    pub const fn new(value: i32) -> Self {
        Self(value)
    }

    /// Returns the constructor value.
    #[inline]
    pub const fn get(&self) -> i32 {
        self.0
    }
}

/// Update type for real-time events.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum Update {
    /// New message received.
    #[serde(rename = "updateNewMessage")]
    NewMessage(serde_json::Value),

    /// Message edit received.
    #[serde(rename = "updateEditMessage")]
    EditMessage(serde_json::Value),

    /// Messages deleted.
    #[serde(rename = "updateDeleteMessages")]
    DeleteMessages(serde_json::Value),

    /// Authorization state changed.
    #[serde(rename = "updateAuthorizationState")]
    AuthorizationState(serde_json::Value),

    /// User status changed.
    #[serde(rename = "updateUserStatus")]
    UserStatus(serde_json::Value),

    /// New chat added.
    #[serde(rename = "updateNewChat")]
    NewChat(serde_json::Value),

    /// Generic update.
    #[serde(rename = "update")]
    Generic(serde_json::Value),
}

/// Callback type for client updates.
pub type UpdateCallback = Box<dyn Fn(Update) + Send + Sync>;

/// Options for creating a ClientActor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientActorOptions {
    /// Whether to enable net query statistics.
    net_query_stats: bool,

    /// Default DC ID.
    default_dc_id: i32,

    /// Query timeout.
    query_timeout: Duration,
}

impl ClientActorOptions {
    /// Creates new ClientActorOptions with default values.
    pub fn new() -> Self {
        Self {
            net_query_stats: false,
            default_dc_id: 2,
            query_timeout: Duration::from_secs(60),
        }
    }

    /// Sets whether to enable net query statistics.
    pub fn with_net_query_stats(mut self, enabled: bool) -> Self {
        self.net_query_stats = enabled;
        self
    }

    /// Sets the default DC ID.
    pub fn with_default_dc_id(mut self, dc_id: i32) -> Self {
        self.default_dc_id = dc_id;
        self
    }

    /// Sets the query timeout.
    pub fn with_query_timeout(mut self, timeout: Duration) -> Self {
        self.query_timeout = timeout;
        self
    }

    /// Returns whether net query stats are enabled.
    pub fn net_query_stats(&self) -> bool {
        self.net_query_stats
    }

    /// Returns the default DC ID.
    pub fn default_dc_id(&self) -> i32 {
        self.default_dc_id
    }

    /// Returns the query timeout.
    pub fn query_timeout(&self) -> Duration {
        self.query_timeout
    }
}

impl Default for ClientActorOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from a network query.
#[derive(Debug, Clone)]
pub enum QueryResponse {
    /// Successful response with data.
    Ok(Bytes),

    /// Error response.
    Err(QueryError),

    /// Query is still pending.
    Pending,
}

/// Pending request context.
struct PendingRequest {
    /// Response channel sender.
    sender: oneshot::Sender<QueryResponse>,

    /// Query ID.
    query_id: NetQueryId,

    /// TL constructor (reserved for future use).
    _constructor: TlConstructor,

    /// Timestamp when request was created.
    created_at: std::time::Instant,
}

/// Client actor for Telegram MTProto.
///
/// Central coordinator for all Telegram client operations.
pub struct ClientActor {
    /// Actor options.
    options: ClientActorOptions,

    /// Network query dispatcher.
    dispatcher: Arc<NetQueryDispatcher>,

    /// Next request ID.
    next_request_id: Arc<AtomicI64>,

    /// Next query ID.
    next_query_id: Arc<AtomicU64>,

    /// Pending requests.
    pending_requests: Arc<Mutex<HashMap<RequestId, PendingRequest>>>,
    pending_requests_by_query_id: Arc<Mutex<HashMap<NetQueryId, RequestId>>>,

    /// Update callbacks.
    update_callbacks: Arc<RwLock<Vec<UpdateCallback>>>,

    /// Update channel sender.
    update_sender: mpsc::UnboundedSender<Update>,

    /// Main DC ID.
    main_dc_id: Arc<AtomicI32>,

    /// Whether the actor is stopped.
    stopped: Arc<AtomicU32>,
}

impl ClientActor {
    /// Creates a new ClientActor.
    pub fn new(options: ClientActorOptions) -> Self {
        let default_dc_id = options.default_dc_id;
        let dispatcher = Arc::new(NetQueryDispatcher::new());
        dispatcher.set_main_dc_id(default_dc_id);

        let (update_sender, _) = mpsc::unbounded_channel();

        Self {
            options,
            dispatcher,
            next_request_id: Arc::new(AtomicI64::new(1)),
            next_query_id: Arc::new(AtomicU64::new(1)),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            pending_requests_by_query_id: Arc::new(Mutex::new(HashMap::new())),
            update_callbacks: Arc::new(RwLock::new(Vec::new())),
            update_sender,
            main_dc_id: Arc::new(AtomicI32::new(default_dc_id)),
            stopped: Arc::new(AtomicU32::new(0)),
        }
    }

    /// Creates a new ClientActor with default options.
    pub fn new_with_defaults() -> Self {
        Self::new(ClientActorOptions::default())
    }

    /// Returns the actor options.
    pub fn options(&self) -> &ClientActorOptions {
        &self.options
    }

    /// Returns the main DC ID.
    pub fn main_dc_id(&self) -> DcId {
        DcId::internal(self.main_dc_id.load(Ordering::Relaxed))
    }

    /// Sets the main DC ID.
    pub fn set_main_dc_id(&self, dc_id: DcId) {
        self.main_dc_id.store(dc_id.get_raw_id(), Ordering::Relaxed);
        self.dispatcher.set_main_dc_id(dc_id.get_raw_id());
    }

    /// Checks if net query stats are enabled.
    pub fn is_net_query_stats_enabled(&self) -> bool {
        self.options.net_query_stats()
    }

    /// Sends a TL request and returns the response.
    ///
    /// # Arguments
    ///
    /// * `constructor` - TL constructor ID
    /// * `data` - Serialized TL request data
    /// * `dc_id` - Target DC ID (uses main DC if None)
    ///
    /// # Returns
    ///
    /// Response data on success
    pub async fn send_request(
        &self,
        constructor: TlConstructor,
        data: Bytes,
        dc_id: Option<DcId>,
    ) -> Result<Bytes> {
        if self.is_stopped() {
            return Err(ClientActorError::NotInitialized);
        }

        let request_id = self.next_request_id.fetch_add(1, Ordering::Relaxed);
        let query_id = self.next_query_id.fetch_add(1, Ordering::Relaxed);

        let target_dc = dc_id.unwrap_or_else(|| self.main_dc_id());

        // Create NetQuery
        let query = NetQuery::new(
            query_id,
            data,
            target_dc,
            NetQueryType::Common,
            AuthFlag::On,
            GzipFlag::Off,
            constructor.get(),
        );

        // Create response channel
        let (tx, rx) = oneshot::channel();

        // Store pending request
        {
            let mut pending = self.pending_requests.lock();
            let mut by_query = self.pending_requests_by_query_id.lock();

            pending.insert(
                request_id,
                PendingRequest {
                    sender: tx,
                    query_id,
                    _constructor: constructor,
                    created_at: std::time::Instant::now(),
                },
            );
            by_query.insert(query_id, request_id);
        }

        // Set callback for query completion
        let pending_requests = self.pending_requests.clone();
        let pending_by_query = self.pending_requests_by_query_id.clone();
        let update_sender = self.update_sender.clone();
        let callback = QueryCallbackImpl {
            _query_id: query_id,
            pending_requests,
            pending_by_query,
            update_sender,
        };
        query.set_callback(Box::new(callback));

        // Dispatch query
        self.dispatcher
            .dispatch(query)
            .map_err(|e| ClientActorError::NetworkError(e.to_string()))?;

        // Wait for response
        tokio::time::timeout(self.options.query_timeout, rx)
            .await
            .map_err(|_| ClientActorError::Timeout)?
            .map_err(|_| ClientActorError::ExecutionFailed("Response channel closed".into()))?
            .into_result()
    }

    /// Sends a serialized TL request.
    ///
    /// Convenience method that serializes the data before sending.
    pub async fn send_tl_request<T>(
        &self,
        constructor: TlConstructor,
        value: &T,
        dc_id: Option<DcId>,
    ) -> Result<Bytes>
    where
        T: rustgram_types::TlSerialize,
    {
        let mut buffer = BytesMut::new();
        TlHelper::write_i32(&mut buffer, constructor.get());

        value
            .serialize_tl(&mut buffer)
            .map_err(|e| ClientActorError::SerializationError(e.to_string()))?;

        self.send_request(constructor, buffer.freeze(), dc_id).await
    }

    /// Registers an update callback.
    pub fn on_update(&self, callback: UpdateCallback) {
        let rt = tokio::runtime::Handle::try_current();
        if let Ok(handle) = rt {
            let callbacks = self.update_callbacks.clone();
            handle.spawn(async move {
                let mut cbs = callbacks.write().await;
                cbs.push(callback);
            });
        } else {
            // If no runtime, store directly using blocking
            // This is a fallback for tests
            let mut callbacks = self.update_callbacks.blocking_write();
            callbacks.push(callback);
        }
    }

    /// Subscribes to updates via a channel.
    pub fn subscribe_updates(&self) -> mpsc::UnboundedReceiver<Update> {
        let (tx, rx) = mpsc::unbounded_channel();
        let rt = tokio::runtime::Handle::try_current();
        let callbacks = self.update_callbacks.clone();

        let tx_clone = tx.clone();
        let add_callback = async move {
            let mut cbs = callbacks.write().await;
            cbs.push(Box::new(move |update| {
                let _ = tx.send(update);
            }));
        };

        if let Ok(handle) = rt {
            handle.spawn(add_callback);
        } else {
            // Fallback for tests
            let mut cbs = self.update_callbacks.blocking_write();
            cbs.push(Box::new(move |update| {
                let _ = tx_clone.send(update);
            }));
        }

        rx
    }

    /// Checks if the actor is stopped.
    pub fn is_stopped(&self) -> bool {
        self.stopped.load(Ordering::Relaxed) == 1
    }

    /// Stops the actor.
    pub fn stop(&self) {
        self.stopped.store(1, Ordering::Relaxed);
        self.dispatcher.stop();
    }

    /// Cancels a pending request.
    ///
    /// # Arguments
    ///
    /// * `request_id` - The request ID to cancel
    ///
    /// # Returns
    ///
    /// `true` if the request was found and marked for cancellation
    ///
    /// # Note
    ///
    /// This currently only removes the request from pending tracking.
    /// Full cancellation support requires access to the underlying NetQuery.
    pub fn cancel_request(&self, request_id: RequestId) -> bool {
        let query_id = {
            let pending = self.pending_requests.lock();
            pending.get(&request_id).map(|req| req.query_id)
        };

        if let Some(query_id) = query_id {
            let mut pending = self.pending_requests.lock();
            pending.remove(&request_id);

            let mut by_query = self.pending_requests_by_query_id.lock();
            by_query.remove(&query_id);
            return true;
        }
        false
    }

    /// Gets the number of pending requests.
    ///
    /// # Returns
    ///
    /// The count of requests currently awaiting responses
    pub fn pending_request_count(&self) -> usize {
        self.pending_requests.lock().len()
    }

    /// Cleanup timed out requests.
    pub fn cleanup_timeouts(&self) {
        let now = std::time::Instant::now();
        let timeout = self.options.query_timeout();

        let mut pending = self.pending_requests.lock();
        let mut by_query = self.pending_requests_by_query_id.lock();

        let to_remove: Vec<RequestId> = pending
            .iter()
            .filter(|(_, req)| now.duration_since(req.created_at) > timeout)
            .map(|(id, _)| *id)
            .collect();

        for id in to_remove {
            if let Some(req) = pending.remove(&id) {
                by_query.remove(&req.query_id);
                // Send timeout error
                let _ = req.sender.send(QueryResponse::Err(QueryError::Generic(
                    "Request timeout".into(),
                )));
            }
        }
    }
}

impl QueryResponse {
    /// Converts to Result.
    pub fn into_result(self) -> Result<Bytes> {
        match self {
            Self::Ok(data) => Ok(data),
            Self::Err(err) => Err(ClientActorError::ExecutionFailed(err.to_string())),
            Self::Pending => Err(ClientActorError::Timeout),
        }
    }
}

impl fmt::Display for ClientActor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ClientActor(net_query_stats={}, main_dc={})",
            self.options.net_query_stats(),
            self.main_dc_id()
        )
    }
}

/// Query callback implementation.
struct QueryCallbackImpl {
    _query_id: NetQueryId,
    pending_requests: Arc<Mutex<HashMap<RequestId, PendingRequest>>>,
    pending_by_query: Arc<Mutex<HashMap<NetQueryId, RequestId>>>,
    update_sender: mpsc::UnboundedSender<Update>,
}

#[async_trait::async_trait]
impl NetQueryCallback for QueryCallbackImpl {
    async fn on_result(&self, query: NetQuery) {
        // Find the request ID
        let (request_id, sender) = {
            let mut by_query = self.pending_by_query.lock();
            if let Some(req_id) = by_query.remove(&query.id()) {
                let mut pending = self.pending_requests.lock();
                if let Some(req) = pending.remove(&req_id) {
                    (Some(req_id), Some(req.sender))
                } else {
                    // Rollback: put it back to prevent memory leak
                    by_query.insert(query.id(), req_id);
                    (None, None)
                }
            } else {
                (None, None)
            }
        };

        if let (Some(req_id), Some(sender)) = (request_id, sender) {
            // Send response
            let response = if query.is_ok() {
                QueryResponse::Ok(query.ok())
            } else if query.is_error() {
                QueryResponse::Err(query.error())
            } else {
                QueryResponse::Pending
            };

            let _ = sender.send(response);

            tracing::debug!("Query {} completed for request {}", query.id(), req_id);

            // Handle update if this is an update-type response
            if query.is_ok() {
                let data = query.ok();
                if let Ok(update_str) = std::str::from_utf8(&data) {
                    if let Ok(update) = serde_json::from_str::<Update>(update_str) {
                        let _ = self.update_sender.send(update);
                    }
                }
            }
        }
    }
}

/// Version information for the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Crate name.
pub const CRATE_NAME: &str = "rustgram-client-actor";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(CRATE_NAME, "rustgram-client-actor");
    }

    #[test]
    fn test_options_new() {
        let options = ClientActorOptions::new();
        assert!(!options.net_query_stats());
        assert_eq!(options.default_dc_id(), 2);
    }

    #[test]
    fn test_options_with_net_query_stats() {
        let options = ClientActorOptions::new().with_net_query_stats(true);
        assert!(options.net_query_stats());
    }

    #[test]
    fn test_options_with_default_dc() {
        let options = ClientActorOptions::new().with_default_dc_id(4);
        assert_eq!(options.default_dc_id(), 4);
    }

    #[test]
    fn test_tl_constructor() {
        let ctor = TlConstructor::new(0x12345678);
        assert_eq!(ctor.get(), 0x12345678);
    }

    #[test]
    fn test_client_actor_new() {
        let actor = ClientActor::new_with_defaults();
        assert!(!actor.is_net_query_stats_enabled());
        assert!(!actor.is_stopped());
        assert_eq!(actor.pending_request_count(), 0);
    }

    #[test]
    fn test_client_actor_display() {
        let actor = ClientActor::new_with_defaults();
        let display = format!("{}", actor);
        assert!(display.contains("ClientActor"));
    }

    #[test]
    fn test_client_actor_stop() {
        let actor = ClientActor::new_with_defaults();
        assert!(!actor.is_stopped());
        actor.stop();
        assert!(actor.is_stopped());
    }

    #[test]
    fn test_client_actor_set_main_dc() {
        let actor = ClientActor::new_with_defaults();
        actor.set_main_dc_id(DcId::internal(4));
        assert_eq!(actor.main_dc_id(), DcId::internal(4));
    }

    #[test]
    fn test_query_response_into_result() {
        let data = Bytes::from_static(b"test data");
        let response = QueryResponse::Ok(data.clone());
        assert!(response.into_result().is_ok());

        let err = QueryResponse::Err(QueryError::Generic("test error".into()));
        assert!(err.into_result().is_err());

        let pending = QueryResponse::Pending;
        assert!(pending.into_result().is_err());
    }

    // Test error display
    #[test]
    fn test_error_display() {
        assert_eq!(
            format!("{}", ClientActorError::InvalidRequest),
            "invalid request data"
        );
        assert_eq!(
            format!("{}", ClientActorError::NotInitialized),
            "client not initialized"
        );
        assert_eq!(
            format!("{}", ClientActorError::Timeout),
            "request timed out"
        );
        assert!(format!(
            "{}",
            ClientActorError::ExecutionFailed("test".to_string())
        )
        .contains("test"));
    }
}
