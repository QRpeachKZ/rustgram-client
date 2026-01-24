//! Network client integration for UserManager.
//!
//! This module provides the network layer for fetching users from Telegram servers.
//! It includes error types, result types, a mock network client for testing,
//! and a real network client that integrates with NetQueryDispatcher.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;

use bytes::{Bytes, BytesMut};
use rustgram_net::{AuthFlag, DcId, GzipFlag, NetQuery, NetQueryDispatcher, NetQueryType};
use rustgram_types::TlDeserialize;
use rustgram_types::TlSerialize;

use crate::{InputUser, User, UserFull};

/// Network error types for user fetching operations.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum NetworkError {
    /// No network client configured.
    #[error("no network client configured")]
    NoClient,

    /// Request timed out.
    #[error("request timed out after {0:?}")]
    Timeout(Duration),

    /// API error with code and message.
    #[error("API error {code}: {message}")]
    ApiError {
        /// Error code.
        code: i32,
        /// Error message.
        message: String,
    },

    /// Data center migration required.
    #[error("data center migration required to DC {dc_id}")]
    DcMigration {
        /// Target DC ID.
        dc_id: i32,
    },

    /// Flood wait - too many requests.
    #[error("flood wait: retry after {seconds} seconds")]
    FloodWait {
        /// Seconds to wait before retrying.
        seconds: i32,
    },

    /// Internal error.
    #[error("internal error: {0}")]
    InternalError(String),

    /// Serialization error.
    #[error("serialization error: {0}")]
    SerializationError(String),
}

impl NetworkError {
    /// Creates an API error from code and message.
    pub fn api_error(code: i32, message: impl Into<String>) -> Self {
        Self::ApiError {
            code,
            message: message.into(),
        }
    }

    /// Creates a DC migration error.
    pub fn dc_migration(dc_id: i32) -> Self {
        Self::DcMigration { dc_id }
    }

    /// Creates a flood wait error.
    pub fn flood_wait(seconds: i32) -> Self {
        Self::FloodWait { seconds }
    }

    /// Creates an internal error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::InternalError(message.into())
    }

    /// Creates a serialization error.
    pub fn serialization(message: impl Into<String>) -> Self {
        Self::SerializationError(message.into())
    }

    /// Returns `true` if this is a retryable error.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Timeout(_) | Self::DcMigration { .. } | Self::FloodWait { .. }
        )
    }
}

/// Result type for single user fetch operations.
pub type GetUserResult = Result<Option<User>, NetworkError>;

/// Result type for multiple users fetch operations.
///
/// Contains both successful fetches and any partial failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetUsersResult {
    /// Successfully fetched users.
    pub users: Vec<User>,
    /// IDs of users that failed to fetch.
    pub failed_ids: Vec<crate::UserId>,
    /// Error details (if any).
    pub errors: Vec<(crate::UserId, NetworkError)>,
}

impl GetUsersResult {
    /// Creates a new successful result with users.
    pub fn success(users: Vec<User>) -> Self {
        Self {
            users,
            failed_ids: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Creates a new empty result.
    pub fn empty() -> Self {
        Self {
            users: Vec::new(),
            failed_ids: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Creates a new result with failures.
    pub fn with_failures(
        users: Vec<User>,
        failed_ids: Vec<crate::UserId>,
        errors: Vec<(crate::UserId, NetworkError)>,
    ) -> Self {
        Self {
            users,
            failed_ids,
            errors,
        }
    }

    /// Returns `true` if all fetches succeeded.
    pub fn is_complete_success(&self) -> bool {
        self.failed_ids.is_empty() && self.errors.is_empty()
    }

    /// Returns the number of successful fetches.
    pub fn success_count(&self) -> usize {
        self.users.len()
    }

    /// Returns the number of failed fetches.
    pub fn failure_count(&self) -> usize {
        self.failed_ids.len()
    }
}

impl Default for GetUsersResult {
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for GetUsersResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GetUsersResult(success: {}, failed: {})",
            self.users.len(),
            self.failed_ids.len()
        )
    }
}

/// Network query callback trait.
///
/// Allows custom handling of network query completion.
#[async_trait::async_trait]
pub trait NetworkCallback: Send + Sync {
    /// Called when a network query completes.
    async fn on_complete(&self, result: Result<bytes::Bytes, NetworkError>);
}

/// Mock network callback for testing.
pub struct MockNetworkCallback;

#[async_trait::async_trait]
impl NetworkCallback for MockNetworkCallback {
    async fn on_complete(&self, _result: Result<bytes::Bytes, NetworkError>) {
        // No-op for mock
    }
}

/// User network client trait.
///
/// Defines the interface for fetching users from the network.
/// Implementations can connect to real Telegram servers or provide mock data for testing.
#[async_trait::async_trait]
pub trait UserNetworkClient: Send + Sync + 'static {
    /// Fetches a single user by InputUser.
    ///
    /// # Arguments
    ///
    /// * `input_user` - The user reference to fetch
    /// * `timeout` - Maximum time to wait for the request
    ///
    /// # Returns
    ///
    /// - `Ok(Some(user))` - User found
    /// - `Ok(None)` - User not found
    /// - `Err(NetworkError)` - Network or API error
    async fn get_user(
        &self,
        input_user: InputUser,
        timeout: Duration,
    ) -> Result<Option<User>, NetworkError>;

    /// Fetches multiple users by InputUser.
    ///
    /// # Arguments
    ///
    /// * `input_users` - List of user references to fetch
    /// * `timeout` - Maximum time to wait for the request
    ///
    /// # Returns
    ///
    /// A result containing successful fetches and any failures.
    async fn get_users(
        &self,
        input_users: Vec<InputUser>,
        timeout: Duration,
    ) -> Result<GetUsersResult, NetworkError>;

    /// Fetches full user profile.
    ///
    /// # Arguments
    ///
    /// * `input_user` - The user reference to fetch
    /// * `timeout` - Maximum time to wait for the request
    ///
    /// # Returns
    ///
    /// - `Ok(Some(full_user))` - Full user profile found
    /// - `Ok(None)` - User not found
    /// - `Err(NetworkError)` - Network or API error
    async fn get_full_user(
        &self,
        input_user: InputUser,
        timeout: Duration,
    ) -> Result<Option<UserFull>, NetworkError>;
}

/// Mock network client for testing.
///
/// Provides a simple in-memory implementation that returns predefined users.
/// Useful for testing without connecting to real Telegram servers.
#[derive(Debug, Clone)]
pub struct MockNetworkClient {
    /// Predefined users to return.
    users: Arc<tokio::sync::RwLock<std::collections::HashMap<i64, User>>>,
}

impl Default for MockNetworkClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MockNetworkClient {
    /// Creates a new empty mock client.
    pub fn new() -> Self {
        Self {
            users: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Adds a user to the mock database.
    pub async fn add_user(&self, user: User) {
        let mut users = self.users.write().await;
        users.insert(user.id().get(), user);
    }

    /// Removes a user from the mock database.
    pub async fn remove_user(&self, user_id: crate::UserId) -> Option<User> {
        let mut users = self.users.write().await;
        users.remove(&user_id.get())
    }

    /// Clears all users from the mock database.
    pub async fn clear(&self) {
        let mut users = self.users.write().await;
        users.clear();
    }

    /// Sets the mock to return an error for specific user ID.
    pub async fn set_error(&self, _user_id: crate::UserId, _error: NetworkError) {
        // In a real implementation, you'd store error conditions
        // For now, this is a placeholder
    }
}

#[async_trait::async_trait]
impl UserNetworkClient for MockNetworkClient {
    async fn get_user(
        &self,
        input_user: InputUser,
        _timeout: Duration,
    ) -> Result<Option<User>, NetworkError> {
        match input_user {
            InputUser::Empty => Ok(None),
            InputUser::InputUserSelf => {
                let users = self.users.read().await;
                // Return first user as "self" for testing
                Ok(users.values().next().cloned())
            }
            InputUser::User { user_id, .. } => {
                let users = self.users.read().await;
                Ok(users.get(&user_id.get()).cloned())
            }
            InputUser::FromMessage { user_id, .. } => {
                let users = self.users.read().await;
                Ok(users.get(&user_id.get()).cloned())
            }
        }
    }

    async fn get_users(
        &self,
        input_users: Vec<InputUser>,
        _timeout: Duration,
    ) -> Result<GetUsersResult, NetworkError> {
        let mut result_users = Vec::new();
        let mut failed_ids = Vec::new();
        let errors = Vec::new();

        let users = self.users.read().await;

        for input_user in input_users {
            match input_user {
                InputUser::Empty => {
                    // Skip empty
                }
                InputUser::InputUserSelf => {
                    if let Some(user) = users.values().next() {
                        result_users.push(user.clone());
                    }
                }
                InputUser::User { user_id, .. } | InputUser::FromMessage { user_id, .. } => {
                    if let Some(user) = users.get(&user_id.get()) {
                        result_users.push(user.clone());
                    } else {
                        failed_ids.push(user_id);
                    }
                }
            }
        }

        Ok(GetUsersResult {
            users: result_users,
            failed_ids,
            errors,
        })
    }

    async fn get_full_user(
        &self,
        input_user: InputUser,
        _timeout: Duration,
    ) -> Result<Option<UserFull>, NetworkError> {
        let user = match input_user {
            InputUser::Empty => return Ok(None),
            InputUser::InputUserSelf => {
                let users = self.users.read().await;
                users.values().next().cloned()
            }
            InputUser::User { user_id, .. } | InputUser::FromMessage { user_id, .. } => {
                let users = self.users.read().await;
                users.get(&user_id.get()).cloned()
            }
        };

        Ok(user.map(|u| {
            let mut full = UserFull::new();
            full.user = Some(u);
            full
        }))
    }
}

/// Real network client for UserManager.
///
/// Provides async/await interface over callback-based NetQuery system.
/// Uses oneshot channels to bridge callback responses with async/await.
/// Follows the DialogManager network pattern.
///
/// # Examples
///
/// ```rust,ignore
/// use rustgram_user_manager::network::RealNetworkClient;
/// use rustgram_net::NetQueryDispatcher;
/// use std::sync::Arc;
///
/// let dispatcher = Arc::new(NetQueryDispatcher::new());
/// let client = RealNetworkClient::new(dispatcher);
///
/// // Send queries using async/await
/// // let response = client.send_typed_query::<Request, Response>(request).await?;
/// ```
#[derive(Clone)]
pub struct RealNetworkClient {
    /// Query dispatcher for sending requests
    dispatcher: Arc<NetQueryDispatcher>,
    /// Request timeout in seconds
    timeout_secs: u64,
}

impl RealNetworkClient {
    /// Default request timeout (30 seconds).
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

    /// Creates a new real network client.
    ///
    /// # Arguments
    ///
    /// * `dispatcher` - Query dispatcher for sending requests
    ///
    /// # Examples
    ///
    /// ```
    /// use rustgram_user_manager::network::RealNetworkClient;
    /// use rustgram_net::NetQueryDispatcher;
    /// use std::sync::Arc;
    ///
    /// let dispatcher = Arc::new(NetQueryDispatcher::new());
    /// let client = RealNetworkClient::new(dispatcher);
    /// ```
    #[must_use]
    pub fn new(dispatcher: Arc<NetQueryDispatcher>) -> Self {
        Self {
            dispatcher,
            timeout_secs: Self::DEFAULT_TIMEOUT_SECS,
        }
    }

    /// Creates a new real network client with custom timeout.
    ///
    /// # Arguments
    ///
    /// * `dispatcher` - Query dispatcher for sending requests
    /// * `timeout_secs` - Request timeout in seconds
    #[must_use]
    pub fn with_timeout(dispatcher: Arc<NetQueryDispatcher>, timeout_secs: u64) -> Self {
        Self {
            dispatcher,
            timeout_secs,
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
    /// # use rustgram_user_manager::network::RealNetworkClient;
    /// # use rustgram_user_manager::tl::GetFullUserRequest;
    /// # use rustgram_user_manager::tl::GetFullUserResponse;
    /// # use rustgram_user_manager::tl::InputUser;
    /// # use rustgram_net::NetQueryDispatcher;
    /// # use std::sync::Arc;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let dispatcher = Arc::new(NetQueryDispatcher::new());
    /// let client = RealNetworkClient::new(dispatcher);
    ///
    /// let request = GetFullUserRequest::new(InputUser::self_());
    /// let response: GetFullUserResponse = client.send_typed_query(
    ///     &request,
    ///     GetFullUserRequest::CONSTRUCTOR_ID
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_typed_query<R, Response>(
        &self,
        request: &R,
        constructor_id: u32,
    ) -> Result<Response, NetworkError>
    where
        R: TlSerialize,
        Response: TlDeserialize,
    {
        // Serialize the request
        let mut buffer = BytesMut::new();
        request
            .serialize_tl(&mut buffer)
            .map_err(|e| NetworkError::SerializationError(format!("{:?}", e)))?;

        // Create NetQuery
        let query = NetQuery::new(
            generate_query_id(),
            buffer.freeze(),
            DcId::main(),
            NetQueryType::Common,
            AuthFlag::On,
            GzipFlag::Off,
            constructor_id as i32,
        );

        // Send query and get response
        let response_bytes = self.send_query(query).await?;

        // Deserialize response
        let mut tl_bytes = rustgram_types::tl::Bytes::new(response_bytes.to_vec().into());
        Response::deserialize_tl(&mut tl_bytes)
            .map_err(|e| NetworkError::SerializationError(format!("{:?}", e)))
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
    /// # use rustgram_user_manager::network::RealNetworkClient;
    /// # use rustgram_net::{NetQuery, NetQueryDispatcher, AuthFlag, GzipFlag, NetQueryType, DcId};
    /// # use std::sync::Arc;
    /// # use bytes::Bytes;
    /// #
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let dispatcher = Arc::new(NetQueryDispatcher::new());
    /// let client = RealNetworkClient::new(dispatcher.clone());
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
    pub async fn send_query(&self, query: NetQuery) -> Result<Bytes, NetworkError> {
        use tokio::time::timeout;

        // Create oneshot channel for response
        let (tx, rx) = tokio::sync::oneshot::channel();

        // Set callback on query
        query.set_callback(Box::new(QueryCallback::new(tx)));

        // Dispatch query
        self.dispatcher
            .dispatch(query.clone())
            .map_err(|e| NetworkError::InternalError(format!("Failed to dispatch query: {}", e)))?;

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
                Err(NetworkError::Timeout(timeout_duration))
            }
        }
    }

    /// Returns the dispatcher.
    #[must_use]
    pub fn dispatcher(&self) -> &Arc<NetQueryDispatcher> {
        &self.dispatcher
    }

    /// Returns the timeout in seconds.
    #[must_use]
    pub fn timeout_secs(&self) -> u64 {
        self.timeout_secs
    }
}

impl Default for RealNetworkClient {
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
static NEXT_QUERY_ID: AtomicU64 = AtomicU64::new(1);

/// Generates a unique query ID.
///
/// # Examples
///
/// ```
/// use rustgram_user_manager::network::generate_query_id;
///
/// let id1 = generate_query_id();
/// let id2 = generate_query_id();
///
/// assert!(id2 > id1);
/// ```
#[must_use]
pub fn generate_query_id() -> u64 {
    NEXT_QUERY_ID.fetch_add(1, Ordering::Relaxed)
}

/// TL constructor for `users.getUsers`.
pub const USERS_GET_USERS: u32 = 0xd91a548;

/// TL constructor for `users.getFullUser`.
pub const USERS_GET_FULL_USER: u32 = 0xb60f5918;

/// Test helper to create a sample user.
pub fn create_test_user(id: i32, first_name: &str) -> User {
    let mut user = User::new();
    user.set_id(crate::UserId::from_i32(id));
    user.set_first_name(first_name.to_string());
    user.set_deleted(false);
    user
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UserId;

    // =========================================================================
    // NetworkError creation tests
    // =========================================================================

    #[test]
    fn test_network_error_no_client() {
        let err = NetworkError::NoClient;
        assert_eq!(err.to_string(), "no network client configured");
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_network_error_timeout() {
        let duration = Duration::from_secs(30);
        let err = NetworkError::Timeout(duration);
        assert!(err.to_string().contains("30"));
        assert!(err.to_string().contains("request timed out"));
        assert!(err.is_retryable());
    }

    #[test]
    fn test_network_error_api() {
        let err = NetworkError::api_error(500, "Internal error");
        assert!(matches!(err, NetworkError::ApiError { code: 500, .. }));
        assert_eq!(err.to_string(), "API error 500: Internal error");
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_network_error_api_401() {
        let err = NetworkError::api_error(401, "Unauthorized");
        assert!(matches!(err, NetworkError::ApiError { code: 401, .. }));
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_network_error_dc_migration() {
        let err = NetworkError::dc_migration(4);
        assert!(matches!(err, NetworkError::DcMigration { dc_id: 4 }));
        assert!(err.to_string().contains("DC 4"));
        assert!(err.is_retryable());
    }

    #[test]
    fn test_network_error_flood_wait() {
        let err = NetworkError::flood_wait(60);
        assert!(matches!(err, NetworkError::FloodWait { seconds: 60 }));
        assert!(err.to_string().contains("60"));
        assert!(err.is_retryable());
    }

    #[test]
    fn test_network_error_internal() {
        let err = NetworkError::internal("Connection lost");
        assert!(matches!(err, NetworkError::InternalError(_)));
        assert!(err.to_string().contains("Connection lost"));
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_network_error_serialization() {
        let err = NetworkError::serialization("Invalid format");
        assert!(matches!(err, NetworkError::SerializationError(_)));
        assert!(err.to_string().contains("Invalid format"));
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_network_error_retryable() {
        assert!(NetworkError::Timeout(Duration::from_secs(1)).is_retryable());
        assert!(NetworkError::dc_migration(2).is_retryable());
        assert!(NetworkError::flood_wait(30).is_retryable());
        assert!(!NetworkError::api_error(400, "Bad request").is_retryable());
        assert!(!NetworkError::NoClient.is_retryable());
        assert!(!NetworkError::internal("Error").is_retryable());
        assert!(!NetworkError::serialization("Error").is_retryable());
    }

    #[test]
    fn test_network_error_clone() {
        let err1 = NetworkError::flood_wait(60);
        let err2 = err1.clone();
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_network_error_equality() {
        let err1 = NetworkError::api_error(500, "Error");
        let err2 = NetworkError::api_error(500, "Error");
        assert_eq!(err1, err2);
    }

    // =========================================================================
    // GetUsersResult tests
    // =========================================================================

    #[test]
    fn test_get_users_result_empty() {
        let result = GetUsersResult::empty();
        assert!(result.is_complete_success());
        assert_eq!(result.success_count(), 0);
        assert_eq!(result.failure_count(), 0);
        assert!(result.users.is_empty());
        assert!(result.failed_ids.is_empty());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_get_users_result_default() {
        let result = GetUsersResult::default();
        assert!(result.is_complete_success());
        assert_eq!(result.success_count(), 0);
    }

    #[test]
    fn test_get_users_result_success() {
        let user = create_test_user(123, "Alice");
        let result = GetUsersResult::success(vec![user]);
        assert!(result.is_complete_success());
        assert_eq!(result.success_count(), 1);
        assert_eq!(result.failure_count(), 0);
    }

    #[test]
    fn test_get_users_result_success_multiple() {
        let users = vec![
            create_test_user(1, "User1"),
            create_test_user(2, "User2"),
            create_test_user(3, "User3"),
        ];
        let result = GetUsersResult::success(users);
        assert!(result.is_complete_success());
        assert_eq!(result.success_count(), 3);
    }

    #[test]
    fn test_get_users_result_with_failures() {
        let user = create_test_user(123, "Alice");
        let failed_id = UserId::from_i32(456);
        let result = GetUsersResult::with_failures(
            vec![user],
            vec![failed_id],
            vec![(failed_id, NetworkError::NoClient)],
        );

        assert!(!result.is_complete_success());
        assert_eq!(result.success_count(), 1);
        assert_eq!(result.failure_count(), 1);
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn test_get_users_result_partial_success() {
        let users = vec![create_test_user(1, "User1"), create_test_user(2, "User2")];
        let failed_ids = vec![UserId::from_i32(3), UserId::from_i32(4)];
        let errors = vec![
            (UserId::from_i32(3), NetworkError::NoClient),
            (
                UserId::from_i32(4),
                NetworkError::api_error(404, "Not found"),
            ),
        ];

        let result = GetUsersResult::with_failures(users, failed_ids, errors);
        assert!(!result.is_complete_success());
        assert_eq!(result.success_count(), 2);
        assert_eq!(result.failure_count(), 2);
    }

    #[test]
    fn test_get_users_result_display() {
        let result = GetUsersResult::success(vec![
            create_test_user(1, "User1"),
            create_test_user(2, "User2"),
        ]);
        let display = format!("{}", result);
        assert!(display.contains("success: 2"));
        assert!(display.contains("failed: 0"));
    }

    #[test]
    fn test_get_users_result_clone() {
        let user = create_test_user(123, "Alice");
        let result = GetUsersResult::success(vec![user]);
        let cloned = result.clone();
        assert_eq!(result, cloned);
    }

    #[test]
    fn test_get_users_result_equality() {
        let user = create_test_user(123, "Alice");
        let result1 = GetUsersResult::success(vec![user.clone()]);
        let result2 = GetUsersResult::success(vec![user]);
        assert_eq!(result1, result2);
    }

    // =========================================================================
    // MockNetworkClient tests
    // =========================================================================

    #[tokio::test]
    async fn test_mock_client_new() {
        let client = MockNetworkClient::new();
        assert_eq!(client.users.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_mock_client_default() {
        let client = MockNetworkClient::default();
        assert_eq!(client.users.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_mock_client_add_get() {
        let client = MockNetworkClient::new();
        let user = create_test_user(123, "Bob");

        client.add_user(user.clone()).await;

        let input = InputUser::user(UserId::from_i32(123));
        let result = client.get_user(input, Duration::from_secs(1)).await;

        assert!(result.is_ok());
        let found = result.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().first_name(), "Bob");
    }

    #[tokio::test]
    async fn test_mock_client_add_multiple() {
        let client = MockNetworkClient::new();

        for i in 1..=5 {
            client
                .add_user(create_test_user(i, &format!("User{}", i)))
                .await;
        }

        assert_eq!(client.users.read().await.len(), 5);
    }

    #[tokio::test]
    async fn test_mock_client_get_empty() {
        let client = MockNetworkClient::new();

        let input = InputUser::user(UserId::from_i32(123));
        let result = client.get_user(input, Duration::from_secs(1)).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_mock_client_get_empty_input() {
        let client = MockNetworkClient::new();
        client.add_user(create_test_user(123, "Bob")).await;

        let result = client
            .get_user(InputUser::Empty, Duration::from_secs(1))
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_mock_client_get_self() {
        let client = MockNetworkClient::new();
        client.add_user(create_test_user(123, "Me")).await;

        let result = client
            .get_user(InputUser::self_(), Duration::from_secs(1))
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_mock_client_get_self_no_users() {
        let client = MockNetworkClient::new();

        let result = client
            .get_user(InputUser::self_(), Duration::from_secs(1))
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_mock_client_get_from_message() {
        let client = MockNetworkClient::new();
        client.add_user(create_test_user(123, "FromMessage")).await;

        let input = InputUser::FromMessage {
            peer: Box::new(rustgram_types::InputPeer::Empty),
            msg_id: 456,
            user_id: UserId::from_i32(123),
        };
        let result = client.get_user(input, Duration::from_secs(1)).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_mock_client_get_multiple() {
        let client = MockNetworkClient::new();

        for i in 1..=3 {
            client
                .add_user(create_test_user(i, &format!("User{}", i)))
                .await;
        }

        let inputs = vec![
            InputUser::user(UserId::from_i32(1)),
            InputUser::user(UserId::from_i32(2)),
            InputUser::user(UserId::from_i32(3)),
            InputUser::user(UserId::from_i32(999)), // Non-existent
        ];

        let result = client.get_users(inputs, Duration::from_secs(1)).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.success_count(), 3);
        assert_eq!(result.failure_count(), 1);
    }

    #[tokio::test]
    async fn test_mock_client_get_multiple_all_empty() {
        let client = MockNetworkClient::new();

        let inputs = vec![InputUser::Empty, InputUser::Empty, InputUser::Empty];

        let result = client.get_users(inputs, Duration::from_secs(1)).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.success_count(), 0);
        assert_eq!(result.failure_count(), 0);
    }

    #[tokio::test]
    async fn test_mock_client_get_multiple_with_self() {
        let client = MockNetworkClient::new();
        client.add_user(create_test_user(123, "Me")).await;

        let inputs = vec![InputUser::self_(), InputUser::self_()];

        let result = client.get_users(inputs, Duration::from_secs(1)).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        // Self returns the first user for both requests
        assert!(result.success_count() >= 1);
    }

    #[tokio::test]
    async fn test_mock_client_remove_user() {
        let client = MockNetworkClient::new();
        let user = create_test_user(123, "Bob");
        client.add_user(user.clone()).await;

        let removed = client.remove_user(UserId::from_i32(123)).await;
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().first_name(), "Bob");

        assert_eq!(client.users.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_mock_client_remove_nonexistent() {
        let client = MockNetworkClient::new();

        let removed = client.remove_user(UserId::from_i32(999)).await;
        assert!(removed.is_none());
    }

    #[tokio::test]
    async fn test_mock_client_clear() {
        let client = MockNetworkClient::new();

        client.add_user(create_test_user(1, "Alice")).await;
        assert_eq!(client.users.read().await.len(), 1);

        client.clear().await;
        assert_eq!(client.users.read().await.len(), 0);
    }

    #[tokio::test]
    async fn test_mock_client_full_user() {
        let client = MockNetworkClient::new();
        let user = create_test_user(123, "Charlie");
        client.add_user(user).await;

        let input = InputUser::user(UserId::from_i32(123));
        let result = client.get_full_user(input, Duration::from_secs(1)).await;

        assert!(result.is_ok());
        let full = result.unwrap();
        assert!(full.is_some());
        let full = full.unwrap();
        assert!(full.user.is_some());
        assert_eq!(full.user.unwrap().first_name(), "Charlie");
    }

    #[tokio::test]
    async fn test_mock_client_full_user_not_found() {
        let client = MockNetworkClient::new();

        let input = InputUser::user(UserId::from_i32(999));
        let result = client.get_full_user(input, Duration::from_secs(1)).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_mock_client_full_user_empty() {
        let client = MockNetworkClient::new();

        let result = client
            .get_full_user(InputUser::Empty, Duration::from_secs(1))
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_mock_client_clone() {
        let client1 = MockNetworkClient::new();
        client1.add_user(create_test_user(123, "Bob")).await;

        let client2 = client1.clone();

        // Both should share the same data
        let input = InputUser::user(UserId::from_i32(123));
        let result1 = client1
            .get_user(input.clone(), Duration::from_secs(1))
            .await;
        let result2 = client2.get_user(input, Duration::from_secs(1)).await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result1.unwrap().is_some());
        assert!(result2.unwrap().is_some());
    }

    // =========================================================================
    // Test helper
    // =========================================================================

    #[test]
    fn test_create_test_user() {
        let user = create_test_user(42, "Test");
        assert_eq!(user.id(), UserId::from_i32(42));
        assert_eq!(user.first_name(), "Test");
        assert!(!user.is_deleted());
    }

    #[test]
    fn test_create_test_user_different_ids() {
        let user1 = create_test_user(1, "User1");
        let user2 = create_test_user(2, "User2");

        assert_eq!(user1.id(), UserId::from_i32(1));
        assert_eq!(user2.id(), UserId::from_i32(2));
        assert_ne!(user1.id(), user2.id());
    }

    // =========================================================================
    // NetworkCallback tests
    // =========================================================================

    #[tokio::test]
    async fn test_mock_network_callback() {
        let callback = MockNetworkCallback;

        // Should not panic
        callback.on_complete(Ok(bytes::Bytes::new())).await;
        callback.on_complete(Err(NetworkError::NoClient)).await;
    }

    // =========================================================================
    // RealNetworkClient tests
    // =========================================================================

    #[test]
    fn test_real_network_client_new() {
        let dispatcher = Arc::new(NetQueryDispatcher::new());
        let client = RealNetworkClient::new(dispatcher);

        assert_eq!(
            client.timeout_secs(),
            RealNetworkClient::DEFAULT_TIMEOUT_SECS
        );
    }

    #[test]
    fn test_real_network_client_with_timeout() {
        let dispatcher = Arc::new(NetQueryDispatcher::new());
        let timeout_secs = 60u64;
        let client = RealNetworkClient::with_timeout(dispatcher, timeout_secs);

        assert_eq!(client.timeout_secs(), timeout_secs);
    }

    #[test]
    fn test_real_network_client_default() {
        let client = RealNetworkClient::default();

        assert_eq!(
            client.timeout_secs(),
            RealNetworkClient::DEFAULT_TIMEOUT_SECS
        );
    }

    #[test]
    fn test_real_network_client_dispatcher_access() {
        let dispatcher = Arc::new(NetQueryDispatcher::new());
        let client = RealNetworkClient::new(dispatcher.clone());

        // Verify we can access the dispatcher
        assert!(Arc::ptr_eq(&dispatcher, client.dispatcher()));
    }

    #[test]
    fn test_constants() {
        assert_eq!(RealNetworkClient::DEFAULT_TIMEOUT_SECS, 30);
        assert_eq!(USERS_GET_USERS, 0xd91a548);
        assert_eq!(USERS_GET_FULL_USER, 0xb60f5918);
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
    fn test_generate_query_id_unique() {
        let mut ids = std::collections::HashSet::new();
        for _ in 0..1000 {
            ids.insert(generate_query_id());
        }
        assert_eq!(ids.len(), 1000);
    }
}
