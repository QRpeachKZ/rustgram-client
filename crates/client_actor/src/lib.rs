// Copyright (c) 2024 rustgram-client contributors
//
// Licensed under MIT OR Apache-2.0

//! Client actor for Telegram MTProto client.
//!
//! This module implements a simplified version of TDLib's ClientActor
//! from `td/telegram/ClientActor.h`.
//!
//! # Overview
//!
//! The ClientActor provides a low-level interface for interacting with the
//! Telegram client. It handles sending requests and receiving responses
//! through callbacks.

#![warn(missing_docs, clippy::all)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

use std::fmt;
use thiserror::Error;

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
}

/// Result type for ClientActor operations.
pub type Result<T> = std::result::Result<T, ClientActorError>;

/// Options for creating a ClientActor.
///
/// # Example
///
/// ```
/// use rustgram_client_actor::ClientActorOptions;
///
/// let options = ClientActorOptions::new();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientActorOptions {
    /// Whether to enable net query statistics.
    net_query_stats: bool,
}

impl ClientActorOptions {
    /// Creates new ClientActorOptions with default values.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_client_actor::ClientActorOptions;
    ///
    /// let options = ClientActorOptions::new();
    /// ```
    pub fn new() -> Self {
        Self {
            net_query_stats: false,
        }
    }

    /// Sets whether to enable net query statistics.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable statistics
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_client_actor::ClientActorOptions;
    ///
    /// let options = ClientActorOptions::new()
    ///     .with_net_query_stats(true);
    /// ```
    pub fn with_net_query_stats(mut self, enabled: bool) -> Self {
        self.net_query_stats = enabled;
        self
    }

    /// Returns whether net query stats are enabled.
    pub fn net_query_stats(&self) -> bool {
        self.net_query_stats
    }
}

impl Default for ClientActorOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// A client request to be executed.
///
/// # Example
///
/// ```
/// use rustgram_client_actor::ClientRequest;
///
/// let request = ClientRequest::new(123, vec![0x01, 0x02, 0x03]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientRequest {
    /// The request ID.
    id: i64,
    /// The request data (serialized bytes).
    data: Vec<u8>,
}

impl ClientRequest {
    /// Creates a new ClientRequest.
    ///
    /// # Arguments
    ///
    /// * `id` - The request identifier
    /// * `data` - The serialized request data
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_client_actor::ClientRequest;
    ///
    /// let request = ClientRequest::new(1, vec![0x01, 0x02, 0x03]);
    /// assert_eq!(request.id(), 1);
    /// assert_eq!(request.data(), &[0x01, 0x02, 0x03]);
    /// ```
    pub fn new(id: i64, data: Vec<u8>) -> Self {
        Self { id, data }
    }

    /// Returns the request ID.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_client_actor::ClientRequest;
    ///
    /// let request = ClientRequest::new(999, vec![]);
    /// assert_eq!(request.id(), 999);
    /// ```
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Returns the request data.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_client_actor::ClientRequest;
    ///
    /// let data = vec![0x01, 0x02, 0x03];
    /// let request = ClientRequest::new(1, data.clone());
    /// assert_eq!(request.data(), &data);
    /// ```
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Returns the length of the request data.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_client_actor::ClientRequest;
    ///
    /// let request = ClientRequest::new(1, vec![0x01, 0x02, 0x03, 0x04]);
    /// assert_eq!(request.data_len(), 4);
    /// ```
    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the request data is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_client_actor::ClientRequest;
    ///
    /// let empty = ClientRequest::new(1, vec![]);
    /// assert!(empty.is_empty());
    ///
    /// let non_empty = ClientRequest::new(1, vec![0x01]);
    /// assert!(!non_empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Sets a new request ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The new request ID
    pub fn with_id(mut self, id: i64) -> Self {
        self.id = id;
        self
    }

    /// Sets new request data.
    ///
    /// # Arguments
    ///
    /// * `data` - The new request data
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }
}

impl fmt::Display for ClientRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ClientRequest(id={}, data_len={})",
            self.id,
            self.data.len()
        )
    }
}

/// Callback type for client notifications.
///
/// This callback is invoked when the client receives a response or update.
pub type ClientCallback = Box<dyn Fn(ClientRequest) + Send + Sync>;

/// Client actor for Telegram MTProto.
///
/// This is a simplified implementation of TDLib's ClientActor.
/// It provides a low-level interface for sending requests and receiving responses.
///
/// # Example
///
/// ```
/// use rustgram_client_actor::{ClientActor, ClientActorOptions, ClientRequest};
///
/// let callback: rustgram_client_actor::ClientCallback = Box::new(|request| {
///     println!("Received response: {}", request.id());
/// });
///
/// let options = ClientActorOptions::new();
/// let actor = ClientActor::new(callback, options);
///
/// let request = ClientRequest::new(1, vec![0x01, 0x02, 0x03]);
/// actor.request(1, request);
/// ```
#[derive(Debug, Clone)]
pub struct ClientActor {
    /// Actor options.
    options: ClientActorOptions,
}

impl ClientActor {
    /// Creates a new ClientActor.
    ///
    /// # Arguments
    ///
    /// * `callback` - Callback for handling responses
    /// * `options` - Configuration options
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_client_actor::{ClientActor, ClientActorOptions};
    ///
    /// let callback = Box::new(|_| {});
    /// let options = ClientActorOptions::new();
    /// let actor = ClientActor::new(callback, options);
    /// ```
    pub fn new(callback: ClientCallback, options: ClientActorOptions) -> Self {
        // In a full implementation, we would store the callback
        // and set up the actor infrastructure
        let _ = callback; // Suppress unused warning in this simplified version

        Self { options }
    }

    /// Creates a new ClientActor with default options.
    ///
    /// # Arguments
    ///
    /// * `callback` - Callback for handling responses
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_client_actor::ClientActor;
    ///
    /// let callback = Box::new(|_| {});
    /// let actor = ClientActor::new_with_defaults(callback);
    /// ```
    pub fn new_with_defaults(callback: ClientCallback) -> Self {
        Self::new(callback, ClientActorOptions::default())
    }

    /// Sends a request to the client.
    ///
    /// # Arguments
    ///
    /// * `id` - The request identifier (must be positive)
    /// * `request` - The request to send
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_client_actor::{ClientActor, ClientActorOptions, ClientRequest};
    ///
    /// let callback = Box::new(|_| {});
    /// let actor = ClientActor::new(callback, ClientActorOptions::new());
    ///
    /// let request = ClientRequest::new(1, vec![0x01, 0x02, 0x03]);
    /// actor.request(1, request);
    /// ```
    pub fn request(&self, id: i64, request: ClientRequest) {
        // In a full implementation, this would send the request
        // through the actor system
        let _ = (id, request);
    }

    /// Synchronously executes a request.
    ///
    /// Only a few requests can be executed synchronously.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to execute
    ///
    /// # Returns
    ///
    /// The response data
    ///
    /// # Errors
    ///
    /// Returns `ClientActorError::ExecutionFailed` if the request cannot be executed.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_client_actor::{ClientActor, ClientActorOptions, ClientRequest};
    ///
    /// let callback = Box::new(|_| {});
    /// let actor = ClientActor::new(callback, ClientActorOptions::new());
    ///
    /// let request = ClientRequest::new(1, vec![0x01, 0x02, 0x03]);
    /// match actor.execute(request) {
    ///     Ok(response) => println!("Got response: {:?}", response),
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub fn execute(&self, _request: ClientRequest) -> Result<Vec<u8>> {
        // In a full implementation, this would execute the request
        // synchronously and return the response
        // For now, return a placeholder error
        Err(ClientActorError::ExecutionFailed(
            "Synchronous execution not yet implemented".to_string(),
        ))
    }

    /// Returns the actor options.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_client_actor::{ClientActor, ClientActorOptions};
    ///
    /// let callback = Box::new(|_| {});
    /// let options = ClientActorOptions::new().with_net_query_stats(true);
    /// let actor = ClientActor::new(callback, options.clone());
    ///
    /// assert!(actor.options().net_query_stats());
    /// ```
    pub fn options(&self) -> &ClientActorOptions {
        &self.options
    }

    /// Checks if net query stats are enabled.
    ///
    /// # Example
    ///
    /// ```
    /// use rustgram_client_actor::{ClientActor, ClientActorOptions};
    ///
    /// let callback = Box::new(|_| {});
    /// let options = ClientActorOptions::new().with_net_query_stats(true);
    /// let actor = ClientActor::new(callback, options);
    ///
    /// assert!(actor.is_net_query_stats_enabled());
    /// ```
    pub fn is_net_query_stats_enabled(&self) -> bool {
        self.options.net_query_stats()
    }
}

impl fmt::Display for ClientActor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ClientActor(net_query_stats={})",
            self.options.net_query_stats
        )
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

    // ClientActorOptions tests
    #[test]
    fn test_options_new() {
        let options = ClientActorOptions::new();
        assert!(!options.net_query_stats());
    }

    #[test]
    fn test_options_with_net_query_stats() {
        let options = ClientActorOptions::new().with_net_query_stats(true);
        assert!(options.net_query_stats());

        let options = ClientActorOptions::new().with_net_query_stats(false);
        assert!(!options.net_query_stats());
    }

    #[test]
    fn test_options_default() {
        let options = ClientActorOptions::default();
        assert!(!options.net_query_stats());
    }

    #[test]
    fn test_options_clone() {
        let options1 = ClientActorOptions::new().with_net_query_stats(true);
        let options2 = options1.clone();
        assert_eq!(options1, options2);
    }

    #[test]
    fn test_options_eq() {
        let options1 = ClientActorOptions::new().with_net_query_stats(true);
        let options2 = ClientActorOptions::new().with_net_query_stats(true);
        assert_eq!(options1, options2);

        let options3 = ClientActorOptions::new().with_net_query_stats(false);
        assert_ne!(options1, options3);
    }

    // ClientRequest tests
    #[test]
    fn test_request_new() {
        let request = ClientRequest::new(123, vec![0x01, 0x02, 0x03]);
        assert_eq!(request.id(), 123);
        assert_eq!(request.data(), &[0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_request_id() {
        let request = ClientRequest::new(999, vec![]);
        assert_eq!(request.id(), 999);
    }

    #[test]
    fn test_request_data() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let request = ClientRequest::new(1, data.clone());
        assert_eq!(request.data(), &data);
    }

    #[test]
    fn test_request_data_len() {
        let request = ClientRequest::new(1, vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        assert_eq!(request.data_len(), 5);

        let empty = ClientRequest::new(1, vec![]);
        assert_eq!(empty.data_len(), 0);
    }

    #[test]
    fn test_request_is_empty() {
        let empty = ClientRequest::new(1, vec![]);
        assert!(empty.is_empty());

        let non_empty = ClientRequest::new(1, vec![0x01]);
        assert!(!non_empty.is_empty());
    }

    #[test]
    fn test_request_with_id() {
        let request = ClientRequest::new(1, vec![]).with_id(999);
        assert_eq!(request.id(), 999);
    }

    #[test]
    fn test_request_with_data() {
        let new_data = vec![0x04, 0x05, 0x06];
        let request = ClientRequest::new(1, vec![0x01, 0x02, 0x03]).with_data(new_data.clone());
        assert_eq!(request.data(), &new_data);
    }

    #[test]
    fn test_request_display() {
        let request = ClientRequest::new(123, vec![0x01, 0x02, 0x03]);
        let display = format!("{request}");
        assert!(display.contains("123"));
        assert!(display.contains("3"));
    }

    #[test]
    fn test_request_clone() {
        let request1 = ClientRequest::new(1, vec![0x01, 0x02, 0x03]);
        let request2 = request1.clone();
        assert_eq!(request1, request2);
    }

    #[test]
    fn test_request_eq() {
        let request1 = ClientRequest::new(1, vec![0x01, 0x02, 0x03]);
        let request2 = ClientRequest::new(1, vec![0x01, 0x02, 0x03]);
        assert_eq!(request1, request2);

        let request3 = ClientRequest::new(2, vec![0x01, 0x02, 0x03]);
        assert_ne!(request1, request3);

        let request4 = ClientRequest::new(1, vec![0x01, 0x02]);
        assert_ne!(request1, request4);
    }

    #[test]
    fn test_request_builder_chain() {
        let request = ClientRequest::new(1, vec![0x01])
            .with_id(2)
            .with_data(vec![0x02, 0x03]);

        assert_eq!(request.id(), 2);
        assert_eq!(request.data(), &[0x02, 0x03]);
    }

    // ClientActor tests
    #[test]
    fn test_actor_new() {
        let callback = Box::new(|_| {});
        let options = ClientActorOptions::new();
        let actor = ClientActor::new(callback, options);

        assert!(!actor.is_net_query_stats_enabled());
    }

    #[test]
    fn test_actor_new_with_defaults() {
        let callback = Box::new(|_| {});
        let actor = ClientActor::new_with_defaults(callback);

        assert!(!actor.is_net_query_stats_enabled());
    }

    #[test]
    fn test_actor_with_net_query_stats() {
        let callback = Box::new(|_| {});
        let options = ClientActorOptions::new().with_net_query_stats(true);
        let actor = ClientActor::new(callback, options);

        assert!(actor.is_net_query_stats_enabled());
    }

    #[test]
    fn test_actor_options() {
        let callback = Box::new(|_| {});
        let options = ClientActorOptions::new().with_net_query_stats(true);
        let actor = ClientActor::new(callback, options.clone());

        assert_eq!(actor.options().net_query_stats(), options.net_query_stats());
    }

    #[test]
    fn test_actor_request() {
        let callback = Box::new(|_| {});
        let actor = ClientActor::new_with_defaults(callback);

        let request = ClientRequest::new(1, vec![0x01, 0x02, 0x03]);
        actor.request(1, request);
        // Just ensure it doesn't panic
    }

    #[test]
    fn test_actor_execute() {
        let callback = Box::new(|_| {});
        let actor = ClientActor::new_with_defaults(callback);

        let request = ClientRequest::new(1, vec![0x01, 0x02, 0x03]);
        let result = actor.execute(request);

        assert!(result.is_err());
        assert!(matches!(result, Err(ClientActorError::ExecutionFailed(_))));
    }

    #[test]
    fn test_actor_display() {
        let callback = Box::new(|_| {});
        let options = ClientActorOptions::new().with_net_query_stats(true);
        let actor = ClientActor::new(callback, options);

        let display = format!("{actor}");
        assert!(display.contains("net_query_stats=true"));
    }

    #[test]
    fn test_actor_clone() {
        let callback = Box::new(|_| {});
        let options = ClientActorOptions::new();
        let actor1 = ClientActor::new(callback, options.clone());
        let actor2 = actor1.clone();

        assert_eq!(actor1.options(), actor2.options());
    }

    #[test]
    fn test_actor_error_display() {
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
            ClientActorError::ExecutionFailed("test error".to_string())
        )
        .contains("test error"));
    }

    #[test]
    fn test_multiple_actors() {
        let callback1 = Box::new(|_| {});
        let callback2 = Box::new(|_| {});

        let actor1 = ClientActor::new_with_defaults(callback1);
        let actor2 = ClientActor::new_with_defaults(callback2);

        assert!(!actor1.is_net_query_stats_enabled());
        assert!(!actor2.is_net_query_stats_enabled());
    }

    #[test]
    fn test_request_with_large_data() {
        let large_data = vec![0xFF; 10_000];
        let request = ClientRequest::new(1, large_data);

        assert_eq!(request.data_len(), 10_000);
        assert!(!request.is_empty());
    }

    #[test]
    fn test_request_negative_id() {
        let request = ClientRequest::new(-1, vec![]);
        assert_eq!(request.id(), -1);
        // Negative IDs are allowed in this simplified implementation
    }

    #[test]
    fn test_request_zero_id() {
        let request = ClientRequest::new(0, vec![]);
        assert_eq!(request.id(), 0);
    }

    #[test]
    fn test_options_debug() {
        let options = ClientActorOptions::new().with_net_query_stats(true);
        let debug = format!("{options:?}");
        assert!(debug.contains("net_query_stats"));
    }

    #[test]
    fn test_actor_debug() {
        let callback = Box::new(|_| {});
        let options = ClientActorOptions::new();
        let actor = ClientActor::new(callback, options);
        let debug = format!("{actor:?}");
        assert!(debug.contains("ClientActor"));
    }

    #[test]
    fn test_request_debug() {
        let request = ClientRequest::new(123, vec![0x01, 0x02, 0x03]);
        let debug = format!("{request:?}");
        assert!(debug.contains("123"));
    }
}
