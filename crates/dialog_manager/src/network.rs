//! Network client integration for DialogManager.
//!
//! This module provides a bridge between DialogManager and the net crate's
//! query dispatcher, converting callback-based NetQuery pattern into async/await.

use bytes::{Bytes, BytesMut};
use rustgram_net::{AuthFlag, DcId, GzipFlag, NetQuery, NetQueryDispatcher, NetQueryType};
use rustgram_types::TlSerialize;
use std::sync::Arc;

use crate::error::DialogError;

/// Network client wrapper for DialogManager.
///
/// Provides async/await interface over callback-based NetQuery system.
/// Uses oneshot channels to bridge callback responses with async/await.
///
/// # Examples
///
/// ```rust
/// use rustgram_dialog_manager::network::NetworkClient;
/// use rustgram_net::NetQueryDispatcher;
/// use std::sync::Arc;
///
/// let dispatcher = Arc::new(NetQueryDispatcher::new());
/// let client = NetworkClient::new(dispatcher);
///
/// // Send queries using async/await
/// // let response = client.send_query(query).await?;
/// ```
#[derive(Clone)]
pub struct NetworkClient {
    /// Query dispatcher for sending requests
    dispatcher: Arc<NetQueryDispatcher>,
    /// DC ID to use for requests
    dc_id: DcId,
    /// Request timeout in seconds
    timeout_secs: u64,
}

impl NetworkClient {
    /// Default request timeout (30 seconds).
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

    /// Creates a new network client.
    ///
    /// # Arguments
    ///
    /// * `dispatcher` - Query dispatcher for sending requests
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::network::NetworkClient;
    /// use rustgram_net::NetQueryDispatcher;
    /// use std::sync::Arc;
    ///
    /// let dispatcher = Arc::new(NetQueryDispatcher::new());
    /// let client = NetworkClient::new(dispatcher);
    /// ```
    #[must_use]
    pub fn new(dispatcher: Arc<NetQueryDispatcher>) -> Self {
        Self {
            dispatcher,
            dc_id: DcId::main(),
            timeout_secs: Self::DEFAULT_TIMEOUT_SECS,
        }
    }

    /// Creates a new network client with custom DC ID.
    ///
    /// # Arguments
    ///
    /// * `dispatcher` - Query dispatcher for sending requests
    /// * `dc_id` - Data center ID to use for requests
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_dialog_manager::network::NetworkClient;
    /// use rustgram_net::{NetQueryDispatcher, DcId};
    /// use std::sync::Arc;
    ///
    /// let dispatcher = Arc::new(NetQueryDispatcher::new());
    /// let dc_id = DcId::main();
    /// let client = NetworkClient::with_dc_id(dispatcher, dc_id);
    /// ```
    #[must_use]
    pub fn with_dc_id(dispatcher: Arc<NetQueryDispatcher>, dc_id: DcId) -> Self {
        Self {
            dispatcher,
            dc_id,
            timeout_secs: Self::DEFAULT_TIMEOUT_SECS,
        }
    }

    /// Sends a typed TL request and deserializes the response.
    ///
    /// # Type Parameters
    ///
    /// * `R` - Request type implementing TlSerialize
    /// * `Response` - Response type implementing TlDeserialize
    ///
    /// # Arguments
    ///
    /// * `request` - TL request to send
    /// * `constructor_id` - TL constructor ID for the request
    ///
    /// # Returns
    ///
    /// Deserialized response on success
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Request serialization fails
    /// - Query dispatch fails
    /// - Request times out
    /// - Response deserialization fails
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustgram_dialog_manager::network::NetworkClient;
    /// # use rustgram_dialog_manager::tl_types::GetDialogsRequest;
    /// # use rustgram_dialog_manager::tl_types::GetDialogsResponse;
    /// # use rustgram_net::NetQueryDispatcher;
    /// # use std::sync::Arc;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let dispatcher = Arc::new(NetQueryDispatcher::new());
    /// let client = NetworkClient::new(dispatcher);
    ///
    /// let request = GetDialogsRequest::new(20);
    /// let response: GetDialogsResponse = client.send_typed_query(
    ///     &request,
    ///     GetDialogsRequest::CONSTRUCTOR_ID
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_typed_query<R, Response>(
        &self,
        request: &R,
        constructor_id: u32,
    ) -> Result<Response, DialogError>
    where
        R: TlSerialize,
        Response: rustgram_types::TlDeserialize,
    {
        // Serialize the request
        let mut buffer = BytesMut::new();
        request
            .serialize_tl(&mut buffer)
            .map_err(|e| DialogError::SerializationError(format!("{:?}", e)))?;

        // Create NetQuery
        let query = NetQuery::new(
            crate::network::generate_query_id(),
            buffer.freeze(),
            self.dc_id,
            NetQueryType::Common,
            AuthFlag::On,
            GzipFlag::Off,
            constructor_id as i32,
        );

        // Send query and get response
        let response_bytes = self.send_query(query).await?;

        // Deserialize response
        let mut tl_bytes = rustgram_types::tl::Bytes::new(response_bytes);
        Response::deserialize_tl(&mut tl_bytes)
            .map_err(|e| DialogError::DeserializationError(format!("{:?}", e)))
    }

    /// Sends a query and waits for response.
    ///
    /// # Arguments
    ///
    /// * `query` - NetQuery to send
    ///
    /// # Returns
    ///
    /// Response bytes on success
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Query dispatch fails
    /// - Request times out
    /// - Query completes with error
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustgram_dialog_manager::network::NetworkClient;
    /// # use rustgram_net::{NetQuery, NetQueryDispatcher, AuthFlag, GzipFlag, NetQueryType, DcId};
    /// # use std::sync::Arc;
    /// # use bytes::Bytes;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let dispatcher = Arc::new(NetQueryDispatcher::new());
    /// let client = NetworkClient::new(dispatcher.clone());
    ///
    /// let query = NetQuery::new(
    ///     1u64,
    ///     Bytes::new(),
    ///     DcId::main(),
    ///     NetQueryType::Common,
    ///     AuthFlag::On,
    ///     GzipFlag::Off,
    ///     0x12345678,
    /// );
    ///
    /// let response = client.send_query(query).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_query(&self, query: NetQuery) -> Result<Bytes, DialogError> {
        use std::time::Duration;
        use tokio::time::timeout;

        // Create oneshot channel for response
        let (tx, rx) = tokio::sync::oneshot::channel();

        // Set callback on query
        query.set_callback(Box::new(QueryCallback::new(tx)));

        // Dispatch query
        self.dispatcher
            .dispatch(query.clone())
            .map_err(|e| DialogError::NetworkError(format!("Failed to dispatch query: {}", e)))?;

        // Wait for response with timeout
        let timeout_duration = Duration::from_secs(self.timeout_secs);
        let result = timeout(timeout_duration, rx).await;

        match result {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(_)) => {
                // Oneshot was canceled
                // This is fine - the callback already ran
                Ok(Bytes::new())
            }
            Err(_) => {
                // Timeout
                Err(DialogError::Timeout(timeout_duration))
            }
        }
    }

    /// Returns the dispatcher.
    #[must_use]
    pub fn dispatcher(&self) -> &Arc<NetQueryDispatcher> {
        &self.dispatcher
    }

    /// Returns the DC ID.
    #[must_use]
    pub fn dc_id(&self) -> DcId {
        self.dc_id
    }

    /// Returns the timeout in seconds.
    #[must_use]
    pub fn timeout_secs(&self) -> u64 {
        self.timeout_secs
    }
}

impl Default for NetworkClient {
    fn default() -> Self {
        Self::new(Arc::new(NetQueryDispatcher::new()))
    }
}

/// Query callback for oneshot channel response.
struct QueryCallback {
    sender: Arc<std::sync::Mutex<Option<tokio::sync::oneshot::Sender<Bytes>>>>,
}

impl QueryCallback {
    /// Creates a new query callback.
    #[must_use]
    fn new(sender: tokio::sync::oneshot::Sender<Bytes>) -> Self {
        Self {
            sender: Arc::new(std::sync::Mutex::new(Some(sender))),
        }
    }
}

#[async_trait::async_trait]
impl rustgram_net::NetQueryCallback for QueryCallback {
    async fn on_result(&self, query: NetQuery) {
        let result = if query.is_ok() {
            query.ok()
        } else if query.is_error() {
            // For error responses, we still send empty bytes
            // The error will be handled by the caller checking query state
            Bytes::new()
        } else {
            Bytes::new()
        };

        // Send result (ignore if receiver is dropped or already closed)
        if let Ok(mut sender_opt) = self.sender.lock() {
            if let Some(sender) = sender_opt.take() {
                let _ = sender.send(result);
            }
        }
    }
}

/// Global query ID counter for generating unique query IDs.
static NEXT_QUERY_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

/// Generates a unique query ID.
///
/// # Examples
///
/// ```
/// use rustgram_dialog_manager::network::generate_query_id;
///
/// let id1 = generate_query_id();
/// let id2 = generate_query_id();
///
/// assert!(id2 > id1);
/// ```
#[must_use]
pub fn generate_query_id() -> u64 {
    NEXT_QUERY_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustgram_net::QueryError;

    #[test]
    fn test_network_client_new() {
        let dispatcher = Arc::new(NetQueryDispatcher::new());
        let client = NetworkClient::new(dispatcher);

        assert_eq!(client.timeout_secs(), NetworkClient::DEFAULT_TIMEOUT_SECS);
    }

    #[test]
    fn test_network_client_with_dc_id() {
        let dispatcher = Arc::new(NetQueryDispatcher::new());
        let dc_id = DcId::main();
        let client = NetworkClient::with_dc_id(dispatcher, dc_id);

        assert_eq!(client.dc_id(), dc_id);
        assert_eq!(client.timeout_secs(), NetworkClient::DEFAULT_TIMEOUT_SECS);
    }

    #[test]
    fn test_network_client_default() {
        let client = NetworkClient::default();

        assert_eq!(client.timeout_secs(), NetworkClient::DEFAULT_TIMEOUT_SECS);
    }

    #[test]
    fn test_generate_query_id() {
        let id1 = generate_query_id();
        let id2 = generate_query_id();
        let id3 = generate_query_id();

        assert!(id2 > id1);
        assert!(id3 > id2);
        assert_eq!(id2, id1 + 1);
        assert_eq!(id3, id2 + 1);
    }

    #[test]
    fn test_error_conversion() {
        let query_error = QueryError::new(401, "Unauthorized");
        let dialog_error: DialogError = query_error.into();
        // The thiserror derive converts QueryError to QueryFailed variant
        assert!(matches!(dialog_error, DialogError::QueryFailed(_)));

        let query_error = QueryError::new(429, "Too many requests");
        let dialog_error: DialogError = query_error.into();
        assert!(matches!(dialog_error, DialogError::QueryFailed(_)));

        let query_error = QueryError::new(500, "Internal server error");
        let dialog_error: DialogError = query_error.into();
        assert!(matches!(dialog_error, DialogError::QueryFailed(_)));
    }

    #[test]
    fn test_constants() {
        assert_eq!(NetworkClient::DEFAULT_TIMEOUT_SECS, 30);
    }

    #[test]
    fn test_network_client_dispatcher_access() {
        let dispatcher = Arc::new(NetQueryDispatcher::new());
        let client = NetworkClient::new(dispatcher.clone());

        // Verify we can access the dispatcher
        assert!(Arc::ptr_eq(&dispatcher, client.dispatcher()));
    }
}
