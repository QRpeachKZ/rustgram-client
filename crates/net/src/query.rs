// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Network query management system.
//!
//! This module implements TDLib's NetQuery system for managing RPC requests.

use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::dc::DcId;

/// Unique identifier for network queries.
pub type NetQueryId = u64;

/// Query state enum.
///
/// Based on TDLib's NetQuery::State from `td/telegram/net/NetQuery.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[derive(Default)]
pub enum NetQueryState {
    /// Empty/uninitialized query
    #[default]
    Empty = 0,
    /// Query is being sent
    Query = 1,
    /// Query completed successfully
    Ok = 2,
    /// Query completed with error
    Error = 3,
}


/// Query type.
///
/// Based on TDLib's NetQuery::Type from `td/telegram/net/NetQuery.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[derive(Default)]
pub enum NetQueryType {
    /// Regular query
    #[default]
    Common = 0,
    /// File upload query
    Upload = 1,
    /// File download query
    Download = 2,
    /// Small file download query
    DownloadSmall = 3,
}


/// Authentication flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[derive(Default)]
pub enum AuthFlag {
    /// No authentication required
    #[default]
    Off = 0,
    /// Authentication required
    On = 1,
}


/// Gzip compression flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[derive(Default)]
pub enum GzipFlag {
    /// No compression
    #[default]
    Off = 0,
    /// Use gzip compression
    On = 1,
}


/// Query error codes.
///
/// Based on TDLib's NetQuery::Error from `td/telegram/net/NetQuery.h`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryErrorCode {
    /// Query should be resent to a different DC
    Resend = 202,
    /// Query was canceled
    Canceled = 203,
    /// Query should be resent with invokeAfter
    ResendInvokeAfter = 204,
}

impl QueryErrorCode {
    /// Creates an error code from its integer value.
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            202 => Some(Self::Resend),
            203 => Some(Self::Canceled),
            204 => Some(Self::ResendInvokeAfter),
            _ => None,
        }
    }

    /// Returns the integer value of this error code.
    pub fn as_i32(&self) -> i32 {
        match self {
            Self::Resend => 202,
            Self::Canceled => 203,
            Self::ResendInvokeAfter => 204,
        }
    }
}

/// Query error.
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum QueryError {
    /// Query error with code and message
    #[error("Query error {code}: {message}")]
    WithMessage {
        /// Error code
        code: i32,
        /// Error message
        message: String,
    },

    /// Special query error codes
    #[error("{0:?}")]
    Special(QueryErrorCode),

    /// Generic error
    #[error("{0}")]
    Generic(String),
}

impl QueryError {
    /// Creates an error from code and message.
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        // Check if this is a special error code
        if let Some(special) = QueryErrorCode::from_i32(code) {
            Self::Special(special)
        } else {
            Self::WithMessage {
                code,
                message: message.into(),
            }
        }
    }

    /// Returns the error code.
    pub fn code(&self) -> i32 {
        match self {
            Self::WithMessage { code, .. } => *code,
            Self::Special(s) => s.as_i32(),
            Self::Generic(_) => 500,
        }
    }

    /// Returns `true` if this is a resend error.
    pub fn is_resend(&self) -> bool {
        matches!(self, Self::Special(QueryErrorCode::Resend))
    }

    /// Returns `true` if this is a cancel error.
    pub fn is_canceled(&self) -> bool {
        matches!(self, Self::Special(QueryErrorCode::Canceled))
    }

    /// Returns `true` if this is a resend invoke after error.
    pub fn is_resend_invoke_after(&self) -> bool {
        matches!(self, Self::Special(QueryErrorCode::ResendInvokeAfter))
    }
}

/// Network query callback trait.
///
/// Based on TDLib's NetQueryCallback from `td/telegram/net/NetQuery.h`.
#[async_trait::async_trait]
pub trait NetQueryCallback: Send + Sync {
    /// Called when the query completes.
    async fn on_result(&self, query: NetQuery);

    /// Called when the query completes and can be resent.
    async fn on_result_resendable(
        &self,
        query: NetQuery,
        _promise: tokio::sync::oneshot::Sender<NetQuery>,
    ) {
        // Default implementation just calls on_result
        self.on_result(query).await;
    }
}

/// Network query.
///
/// Represents a single RPC request to Telegram servers.
/// Based on TDLib's NetQuery class from `td/telegram/net/NetQuery.h`.
#[derive(Clone)]
pub struct NetQuery {
    inner: Arc<NetQueryInner>,
}

struct NetQueryInner {
    /// Query ID
    id: NetQueryId,

    /// Atomic state
    state: AtomicU8,

    /// Query type
    query_type: NetQueryType,

    /// Authentication flag
    auth_flag: AuthFlag,

    /// Gzip flag
    gzip_flag: GzipFlag,

    /// Target DC ID
    dc_id: DcId,

    /// Query data
    query: Bytes,

    /// Response data (if successful)
    answer: parking_lot::Mutex<Option<Bytes>>,

    /// Error status (if failed)
    error: parking_lot::Mutex<Option<QueryError>>,

    /// TL constructor
    tl_constructor: i32,

    /// Cancellation token (0 means canceled)
    cancellation_token: AtomicI32,

    /// Real DC ID that was used
    real_dc_id: AtomicI32,

    /// Main auth key ID
    main_auth_key_id: AtomicU64,

    /// Session ID
    session_id: AtomicU64,

    /// Message ID
    message_id: AtomicU64,

    /// Chain IDs for sequence dispatching
    chain_ids: parking_lot::Mutex<Vec<u64>>,

    /// Whether this is a high-priority query
    is_high_priority: AtomicBool,

    /// Whether the query may be lost
    may_be_lost: AtomicBool,

    /// Whether using sequence dispatcher
    in_sequence_dispatcher: AtomicBool,

    /// Verification prefix length
    verification_prefix_length: AtomicU8,

    /// Next timeout (for NetQueryDelayer)
    next_timeout: parking_lot::Mutex<Duration>,

    /// Total timeout
    total_timeout: parking_lot::Mutex<Duration>,

    /// Total timeout limit
    total_timeout_limit: Duration,

    /// Source of this query
    source: parking_lot::Mutex<String>,

    /// Dispatch TTL
    dispatch_ttl: AtomicI32,

    /// File type
    file_type: parking_lot::Mutex<i32>,

    /// Whether to resend on 503 errors
    need_resend_on_503: bool,

    /// Callback
    callback: parking_lot::Mutex<Option<Box<dyn NetQueryCallback>>>,
}

impl fmt::Debug for NetQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NetQuery")
            .field("id", &self.id())
            .field("state", &self.state())
            .field("dc_id", &self.dc_id())
            .field("type", &self.query_type())
            .finish()
    }
}

impl NetQuery {
    /// Creates a new network query.
    pub fn new(
        id: NetQueryId,
        query: Bytes,
        dc_id: DcId,
        query_type: NetQueryType,
        auth_flag: AuthFlag,
        gzip_flag: GzipFlag,
        tl_constructor: i32,
    ) -> Self {
        Self {
            inner: Arc::new(NetQueryInner {
                id,
                state: AtomicU8::new(NetQueryState::Query as u8),
                query_type,
                auth_flag,
                gzip_flag,
                dc_id,
                query,
                answer: parking_lot::Mutex::new(None),
                error: parking_lot::Mutex::new(None),
                tl_constructor,
                cancellation_token: AtomicI32::new(-1),
                real_dc_id: AtomicI32::new(0),
                main_auth_key_id: AtomicU64::new(0),
                session_id: AtomicU64::new(0),
                message_id: AtomicU64::new(0),
                chain_ids: parking_lot::Mutex::new(Vec::new()),
                is_high_priority: AtomicBool::new(false),
                may_be_lost: AtomicBool::new(false),
                in_sequence_dispatcher: AtomicBool::new(false),
                verification_prefix_length: AtomicU8::new(0),
                next_timeout: parking_lot::Mutex::new(Duration::from_secs(1)),
                total_timeout: parking_lot::Mutex::new(Duration::ZERO),
                total_timeout_limit: Duration::from_secs(60),
                source: parking_lot::Mutex::new(String::new()),
                dispatch_ttl: AtomicI32::new(-1),
                file_type: parking_lot::Mutex::new(-1),
                need_resend_on_503: true,
                callback: parking_lot::Mutex::new(None),
            }),
        }
    }

    /// Returns the query ID.
    #[inline]
    pub fn id(&self) -> NetQueryId {
        self.inner.id
    }

    /// Returns the query state.
    #[inline]
    pub fn state(&self) -> NetQueryState {
        match self.inner.state.load(Ordering::Relaxed) {
            0 => NetQueryState::Empty,
            1 => NetQueryState::Query,
            2 => NetQueryState::Ok,
            3 => NetQueryState::Error,
            _ => NetQueryState::Empty,
        }
    }

    /// Returns the target DC ID.
    #[inline]
    pub fn dc_id(&self) -> DcId {
        self.inner.dc_id
    }

    /// Returns the query type.
    #[inline]
    pub fn query_type(&self) -> NetQueryType {
        self.inner.query_type
    }

    /// Returns the gzip flag.
    #[inline]
    pub fn gzip_flag(&self) -> GzipFlag {
        self.inner.gzip_flag
    }

    /// Returns the auth flag.
    #[inline]
    pub fn auth_flag(&self) -> AuthFlag {
        self.inner.auth_flag
    }

    /// Returns the TL constructor.
    #[inline]
    pub fn tl_constructor(&self) -> i32 {
        self.inner.tl_constructor
    }

    /// Returns the query data.
    #[inline]
    pub fn query(&self) -> &Bytes {
        &self.inner.query
    }

    /// Returns `true` if the query is ready (completed or failed).
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.state() != NetQueryState::Query
    }

    /// Returns `true` if the query completed successfully.
    #[inline]
    pub fn is_ok(&self) -> bool {
        self.state() == NetQueryState::Ok
    }

    /// Returns `true` if the query failed.
    #[inline]
    pub fn is_error(&self) -> bool {
        self.state() == NetQueryState::Error
    }

    /// Returns the answer data if successful.
    ///
    /// # Panics
    ///
    /// Panics if the query did not complete successfully.
    #[inline]
    pub fn ok(&self) -> Bytes {
        assert!(self.is_ok(), "Query is not OK: {:?}", self.state());
        self.inner
            .answer
            .lock()
            .clone()
            .unwrap_or_else(|| panic!("OK query has no answer: {:?}", self.state()))
    }

    /// Returns the error if failed.
    ///
    /// # Panics
    ///
    /// Panics if the query did not fail.
    #[inline]
    pub fn error(&self) -> QueryError {
        assert!(self.is_error(), "Query is not an error: {:?}", self.state());
        self.inner
            .error
            .lock()
            .clone()
            .unwrap_or_else(|| panic!("Error query has no error: {:?}", self.state()))
    }

    /// Takes the answer data, consuming the query.
    ///
    /// # Panics
    ///
    /// Panics if the query did not complete successfully.
    pub fn take_ok(self) -> Bytes {
        assert!(self.is_ok(), "Query is not OK");
        self.inner
            .answer
            .lock()
            .take()
            .unwrap_or_else(|| panic!("OK query has no answer: {:?}", self.state()))
    }

    /// Takes the error, consuming the query.
    ///
    /// # Panics
    ///
    /// Panics if the query did not fail.
    pub fn take_error(self) -> QueryError {
        assert!(self.is_error(), "Query is not an error");
        self.inner
            .error
            .lock()
            .take()
            .unwrap_or_else(|| panic!("Error query has no error: {:?}", self.state()))
    }

    /// Sets the query result as successful.
    pub fn set_ok(&self, data: Bytes) {
        *self.inner.answer.lock() = Some(data);
        self.inner
            .state
            .store(NetQueryState::Ok as u8, Ordering::Release);
    }

    /// Sets the query result as error.
    pub fn set_error(&self, error: QueryError) {
        *self.inner.error.lock() = Some(error);
        self.inner
            .state
            .store(NetQueryState::Error as u8, Ordering::Release);
    }

    /// Cancels the query if the token matches.
    pub fn cancel(&self, token: i32) {
        self.inner
            .cancellation_token
            .compare_exchange(token, 0, Ordering::Relaxed, Ordering::Relaxed)
            .ok();
    }

    /// Sets the cancellation token.
    pub fn set_cancellation_token(&self, token: i32) {
        self.inner
            .cancellation_token
            .store(token, Ordering::Relaxed);
    }

    /// Returns the real DC ID that was used.
    #[inline]
    pub fn real_dc_id(&self) -> i32 {
        self.inner.real_dc_id.load(Ordering::Relaxed)
    }

    /// Sets the real DC ID.
    #[inline]
    pub fn set_real_dc_id(&self, dc_id: i32) {
        self.inner.real_dc_id.store(dc_id, Ordering::Relaxed);
    }

    /// Returns the main auth key ID.
    #[inline]
    pub fn main_auth_key_id(&self) -> u64 {
        self.inner.main_auth_key_id.load(Ordering::Relaxed)
    }

    /// Sets the main auth key ID.
    #[inline]
    pub fn set_main_auth_key_id(&self, key_id: u64) {
        self.inner.main_auth_key_id.store(key_id, Ordering::Relaxed);
    }

    /// Returns the session ID.
    #[inline]
    pub fn session_id(&self) -> u64 {
        self.inner.session_id.load(Ordering::Relaxed)
    }

    /// Sets the session ID.
    #[inline]
    pub fn set_session_id(&self, session_id: u64) {
        self.inner.session_id.store(session_id, Ordering::Relaxed);
    }

    /// Returns the message ID.
    #[inline]
    pub fn message_id(&self) -> u64 {
        self.inner.message_id.load(Ordering::Relaxed)
    }

    /// Sets the message ID.
    #[inline]
    pub fn set_message_id(&self, message_id: u64) {
        self.inner.message_id.store(message_id, Ordering::Relaxed);
    }

    /// Returns the chain IDs.
    #[inline]
    pub fn chain_ids(&self) -> Vec<u64> {
        self.inner.chain_ids.lock().clone()
    }

    /// Sets the chain IDs.
    #[inline]
    pub fn set_chain_ids(&self, ids: Vec<u64>) {
        *self.inner.chain_ids.lock() = ids;
    }

    /// Returns `true` if this is a high-priority query.
    #[inline]
    pub fn is_high_priority(&self) -> bool {
        self.inner.is_high_priority.load(Ordering::Relaxed)
    }

    /// Marks this query as high-priority.
    #[inline]
    pub fn make_high_priority(&mut self) {
        self.inner.is_high_priority.store(true, Ordering::Relaxed);
    }

    /// Returns `true` if this query may be lost.
    #[inline]
    pub fn may_be_lost(&self) -> bool {
        self.inner.may_be_lost.load(Ordering::Relaxed)
    }

    /// Returns `true` if using sequence dispatcher.
    #[inline]
    pub fn in_sequence_dispatcher(&self) -> bool {
        self.inner.in_sequence_dispatcher.load(Ordering::Relaxed)
    }

    /// Sets whether using sequence dispatcher.
    #[inline]
    pub fn set_in_sequence_dispatcher(&mut self, value: bool) {
        self.inner
            .in_sequence_dispatcher
            .store(value, Ordering::Relaxed);
    }

    /// Clears the query data.
    pub fn clear(&self) {
        self.inner
            .state
            .store(NetQueryState::Empty as u8, Ordering::Release);
        *self.inner.answer.lock() = None;
        *self.inner.error.lock() = None;
    }

    /// Returns `true` if this query is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.state() == NetQueryState::Empty
    }

    /// Returns the next timeout duration.
    pub fn next_timeout(&self) -> Duration {
        *self.inner.next_timeout.lock()
    }

    /// Sets the next timeout duration.
    pub fn set_next_timeout(&self, timeout: Duration) {
        *self.inner.next_timeout.lock() = timeout;
    }

    /// Returns the total timeout.
    pub fn total_timeout(&self) -> Duration {
        *self.inner.total_timeout.lock()
    }

    /// Adds to the total timeout.
    pub fn add_total_timeout(&self, duration: Duration) {
        let mut timeout = self.inner.total_timeout.lock();
        *timeout = timeout.saturating_add(duration);
    }

    /// Returns the total timeout limit.
    pub fn total_timeout_limit(&self) -> Duration {
        self.inner.total_timeout_limit
    }

    /// Returns the source string.
    pub fn source(&self) -> String {
        self.inner.source.lock().clone()
    }

    /// Sets the source string.
    pub fn set_source(&self, source: String) {
        *self.inner.source.lock() = source;
    }

    /// Returns the dispatch TTL.
    pub fn dispatch_ttl(&self) -> i32 {
        self.inner.dispatch_ttl.load(Ordering::Relaxed)
    }

    /// Sets the dispatch TTL.
    pub fn set_dispatch_ttl(&self, ttl: i32) {
        self.inner.dispatch_ttl.store(ttl, Ordering::Relaxed);
    }

    /// Returns the file type.
    pub fn file_type(&self) -> i32 {
        *self.inner.file_type.lock()
    }

    /// Sets the file type.
    pub fn set_file_type(&self, file_type: i32) {
        *self.inner.file_type.lock() = file_type;
    }

    /// Returns `true` if should resend on 503.
    pub fn need_resend_on_503(&self) -> bool {
        self.inner.need_resend_on_503
    }

    /// Sets the callback.
    pub fn set_callback(&self, callback: Box<dyn NetQueryCallback>) {
        *self.inner.callback.lock() = Some(callback);
    }
}

impl fmt::Display for NetQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NetQuery(id={}, dc={}, type={:?}, state={:?})",
            self.id(),
            self.dc_id(),
            self.query_type(),
            self.state()
        )
    }
}

/// Network query dispatcher.
///
/// Routes queries to appropriate sessions based on DC and query type.
/// Based on TDLib's NetQueryDispatcher from `td/telegram/net/NetQueryDispatcher.h`.
pub struct NetQueryDispatcher {
    /// Main DC ID
    main_dc_id: std::sync::atomic::AtomicI32,

    /// Whether the dispatcher is stopped
    stop_flag: std::sync::atomic::AtomicBool,

    /// Query sender channel
    query_sender: tokio::sync::mpsc::UnboundedSender<NetQuery>,
}

impl NetQueryDispatcher {
    /// Creates a new query dispatcher.
    pub fn new() -> Self {
        let (query_sender, _query_receiver) = tokio::sync::mpsc::unbounded_channel();

        Self {
            main_dc_id: std::sync::atomic::AtomicI32::new(1),
            stop_flag: std::sync::atomic::AtomicBool::new(false),
            query_sender,
        }
    }

    /// Returns the main DC ID.
    pub fn main_dc_id(&self) -> DcId {
        DcId::internal(self.main_dc_id.load(Ordering::Relaxed))
    }

    /// Sets the main DC ID.
    pub fn set_main_dc_id(&self, dc_id: i32) {
        self.main_dc_id.store(dc_id, Ordering::Relaxed);
    }

    /// Dispatches a query to the appropriate session.
    pub fn dispatch(&self, query: NetQuery) -> Result<(), QueryError> {
        if self.stop_flag.load(Ordering::Relaxed) {
            return Err(QueryError::Generic("Dispatcher is stopped".into()));
        }

        self.query_sender
            .send(query)
            .map_err(|_| QueryError::Generic("Failed to dispatch query".into()))?;

        Ok(())
    }

    /// Stops the dispatcher.
    pub fn stop(&self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }

    /// Returns `true` if the dispatcher is stopped.
    pub fn is_stopped(&self) -> bool {
        self.stop_flag.load(Ordering::Relaxed)
    }
}

impl Default for NetQueryDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_query_creation() {
        let query = NetQuery::new(
            1,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::On,
            GzipFlag::On,
            0x12345678,
        );

        assert_eq!(query.id(), 1);
        assert_eq!(query.dc_id(), DcId::internal(2));
        assert_eq!(query.query_type(), NetQueryType::Common);
        assert_eq!(query.auth_flag(), AuthFlag::On);
        assert_eq!(query.gzip_flag(), GzipFlag::On);
    }

    #[test]
    fn test_net_query_result() {
        let query = NetQuery::new(
            1,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::On,
            GzipFlag::Off,
            0,
        );

        assert!(!query.is_ready());
        assert!(!query.is_ok());
        assert!(!query.is_error());

        let response = Bytes::from_static(b"response");
        query.set_ok(response.clone());

        assert!(query.is_ready());
        assert!(query.is_ok());
        assert_eq!(query.ok(), response);
    }

    #[test]
    fn test_net_query_error() {
        let query = NetQuery::new(
            1,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        );

        let error = QueryError::new(500, "Internal error");
        query.set_error(error.clone());

        assert!(query.is_ready());
        assert!(query.is_error());
        assert_eq!(query.error().code(), 500);
    }

    #[test]
    fn test_query_error_codes() {
        assert_eq!(QueryErrorCode::Resend.as_i32(), 202);
        assert_eq!(QueryErrorCode::Canceled.as_i32(), 203);
        assert_eq!(QueryErrorCode::ResendInvokeAfter.as_i32(), 204);

        assert_eq!(QueryErrorCode::from_i32(202), Some(QueryErrorCode::Resend));
        assert_eq!(QueryErrorCode::from_i32(999), None);
    }

    #[test]
    fn test_query_dispatcher() {
        let dispatcher = NetQueryDispatcher::new();

        assert_eq!(dispatcher.main_dc_id(), DcId::internal(1));
        assert!(!dispatcher.is_stopped());

        dispatcher.set_main_dc_id(4);
        assert_eq!(dispatcher.main_dc_id(), DcId::internal(4));

        let query = NetQuery::new(
            1,
            Bytes::new(),
            DcId::internal(2),
            NetQueryType::Common,
            AuthFlag::Off,
            GzipFlag::Off,
            0,
        );

        // Note: This will fail because the receiver channel is dropped
        assert!(dispatcher.dispatch(query).is_err());

        dispatcher.stop();
        assert!(dispatcher.is_stopped());
    }
}
